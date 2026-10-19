---
title: "Agent Tooling"
description: "Project-specific routing for the Claude Code Minoan and Codex skill toolbelt"
category: "agent-docs"
created: 2026-09-08
updated: 2026-10-06
tags: [agents, skills, codex, fable, ghidra, qa, research, gemini, audio, native, cua-driver, task-graph]
---

# Agent Tooling for Open Rebellion

Use the smallest skill set that fits the task. Read the selected skill's
`SKILL.md` before acting, keep generated evidence out of Git unless the audit
explicitly calls for it, and record material conclusions in the audit JSON and
Markdown rather than leaving them only in agent output.

Claude Code Minoan skills link to their canonical definitions in
[`tdimino/claude-code-minoan`](https://github.com/tdimino/claude-code-minoan).

## Required Review Lanes

| Tool | Use it for | Project rule |
|------|------------|--------------|
| [`codex-orchestrator`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/integration-automation/codex-orchestrator) | Risk-based independent review, architecture checks, debugging, and real-browser acceptance | Use Sol high or extra-high when consequential architecture, reverse-engineering ambiguity, persistence/network formats, security, or unusual diff risk warrants it; routine, well-tested slices need no separate Sol review. Use Astra only for live browser/computer-use acceptance: low for routine checks and medium for complex or release-significant journeys. Retain inspected screenshots, console/network logs, and artifact hashes. Run evidence agents from a dedicated `/tmp` workspace so the repository stays read-only. |
| [`fable`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/fable) | Deep cross-cutting parity audits, browser/multiplayer optimization, and synthesis across prior findings | Fable 5.1 is the default; `--naos` selects 5.0. Verify the reported `MODEL:` line and verdict trailer before trusting results, then weave accepted findings into the audit and roadmaps. Keep `.subdaimon-output/` local and untracked. |

## Native GUI Acceptance

Native builds have no fixture bridge, so a native check plays a real
campaign through the window and the screenshots are the evidence.

Drive it first with the game's own developer commands
([dev-commands.md](dev-commands.md)). `scripts/launch-native.sh --live`
opens a command channel, and `scripts/dev-send.sh` sends one command and
prints its JSON result. `Capture` saves the frame itself (no window id, no
Screen Recording, works while covered). `Click`, `Drag` and `Press` hand real
`NSEvent`s to the game's view, so they test the input path without moving
anyone's pointer or changing the front app. `Battle at`, `Post message` and
`Open Message Index` reach states the simulation is slow to produce.

Fall back to [`cua-driver`](https://github.com/trycua/cua/tree/main/libs/cua-driver)
(trycua, MIT) for what the game cannot do itself, such as an OS dialog or a
check of the window from outside.

Why cua-driver and not the earlier harnesses:

- It posts clicks and keys to the game's pid as background CGEvents. The
  hardware pointer never moves and the window never comes to the front, so
  those steps don't disturb the desktop (drags are the exception, below).
- It sends right clicks into the canvas. BackgroundComputerUse reports
  secondary clicks as unsupported, which left every pop-up menu journey
  (Move, Confirmed Move, Create Fleet, the F-019 Mission entry, the speed
  menu) pending in the 2026-10-05 native run.
- It presses function keys. BackgroundComputerUse rejects `f3`, so the
  Finder's F3 path couldn't be checked; cua-driver opens it.
- Its pixel input needs no Accessibility tree. Macroquad exposes none, and
  Peekaboo's background drags reach only Accessibility elements, so they
  can't move game objects. For the same reason cua-driver's own `type_text`
  (an AX call) does nothing here; type with one `press_key` per character.
- Accessibility and Screen Recording belong to the signed `CuaDriver.app`
  (`com.trycua.driver`), not to the terminal.

Proven on 2026-10-06: left and right clicks, letters, digits, space and F3.
`drag` on a macOS window runs only with `delivery_mode:"foreground"`: it
fronts the game for the gesture and moves the physical pointer, then
restores the previous app. Front the game yourself first (`osascript -e 'tell
application "System Events" to set frontmost of process "open-rebellion" to
true'`), or macOS spends the press on activating the window and the game
never sees it. Give focus back afterwards, and warn whoever is at the
machine before running one.

### Install

```bash
/bin/bash -c "$(curl -fsSL https://cua.ai/driver/install.sh)"  # checks SHA256SUMS
cua-driver telemetry disable
cua-driver permissions grant   # approve Accessibility, Screen Recording, direct capture
cua-driver permissions status  # all three granted, Source: driver-daemon
```

The installer puts the app in `/Applications/CuaDriver.app` and the CLI in
`~/.local/bin/cua-driver`. Add `--require-signature` if `cosign` is
installed. Start the daemon with `open -n -g -a CuaDriver --args serve`;
`status` reports `unknown` when it isn't running. `cua-driver update`
checks for a newer release.

### Run a check

1. Launch with `scripts/launch-native.sh --build --evidence
   .artifacts/native-checks/<date>-<slug>`. It builds
   `target/release/open-rebellion` and starts it with `data/base` and
   `OPEN_REBELLION_MUTE=1`, which silences music, effects and cutscenes from
   launch, so no options-screen clicks are needed. It then prints, and writes
   to `binary.txt`, the SHA-256, the source commit, the pid and the window
   id. Pass game arguments after `--`. The variable accepts 1, true, yes or
   on, in any case; set it yourself when launching another way. Without it,
   mute by hand: the main menu speaker mutes music only, so open the options
   screen from the main menu, click the left end of both the music and the
   effects sliders, then leave by Restart, which with no game running goes
   straight back to the shuttle cockpit (the screen takes no Escape).
2. The script prints the window id. Otherwise, find the window with
   `cua-driver call list_windows` (app `open-rebellion`).
3. Pass the same `"session":"<name>"` on every call, or captures come back
   `capture_not_found`.
4. Capture with `get_window_state` (`include_accessibility_tree:false`,
   `screenshot_out_file`). Its `capture_id` is required by pixel clicks.
5. Click at screenshot pixels: `click` with `x`, `y`, `capture_id`, and
   `"button":"right"` for a right click (`right_click` rejects
   `capture_id`). Keys use `press_key` without a `capture_id`.
6. Capture again and inspect it. The driver reports background input as
   `unverifiable`, so only the screenshot shows the result.

Coordinates are in the returned screenshot's pixels (`screenshot_width`,
`screenshot_scale`). The scale changes with the display the window sits on.
Keep evidence under `.artifacts/native-checks/<date>-<slug>/` with the
binary hash, and record each check as an agent check, never a human one.

### Reach a state with commands

The developer commands skip the clicks that lead to the state under test:
the palette (backtick), command scripts and the live channel are described in
[dev-commands.md](dev-commands.md), with the full command list, how to add
one, and the rule that a state command is not a test of the input leading to
it.

### Compare against the original

The original game runs on the capture VM on the Mac Mini
([tools/original-capture/README.md](../tools/original-capture/README.md)),
which also covers driving it over VNC. `tools/original-capture/vm-game.sh`
drives it from this Mac: `status`, `up` (start the VM and the game),
`recover`, `cap FILE`, `click X Y [FILE]`, `key KEY [FILE]`. Start from a save in the library
([saves/INDEX.md](../tools/original-capture/saves/INDEX.md)) rather than
replaying a campaign to reach a state, and add a save when you reach a new
one worth keeping. For a state play cannot reach on demand, edit a copy of a
save ([original-save-editing.md](original-save-editing.md)).

## Reverse Engineering

Open Rebellion's Ghidra workflow is project-native rather than a general skill:

- Read `agent_docs/ghidra-re.md` before reverse-engineering game behavior.
- Use the existing Open Rebellion Ghidra project, Jython scripts, decompilation
  corpus, and `ghidra/notes/` sources described there.
- Record addresses, call chains, field offsets, and confidence, then connect the
  finding to a Rust test or acceptance fixture.
- Take reverse-engineering work as `question` nodes of the task graph
  (`agent_docs/task-graph.md`): one question per agent, its review, then its
  port. A delegated agent answers questions only; review and porting stay
  with the lead session.
- Do not use [`bg3se-macos-ghidra`](https://github.com/tdimino/bg3se-macos/tree/main/tools/skills/bg3se-macos-ghidra); that skill encodes Baldur's Gate 3-specific
  binaries, layouts, launch flows, and terminology.

## Supporting Skills

| Skill | Use when |
|-------|----------|
| [`test-harness-auditor`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/test-harness-auditor) | Auditing build, test, lint, browser, and debug gates before M5 CI restoration. |
| [`cloudflare`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/integration-automation/cloudflare) | Implementing the password-protected Pages/Functions v1.0 deployment, secrets, headers, environments, and rollback. |
| [`architecture-md-builder`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/architecture-md-builder) | Updating the architectural map after an approved cross-crate or multiplayer boundary change. |
| [`claude-md-manager`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/claude-md-manager) | Keeping `CLAUDE.md` concise, accurate, and progressively disclosed. |
| [`agents-md-manager`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/agents-md-manager) | Maintaining the cross-agent `AGENTS.md` contract and validating its limits. |
| [`design-audit`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/design-audit) | Scoring accessibility, responsiveness, rendering quality, and browser performance without silently changing the UI. |
| [`keenable`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/research/keenable) | Default or high-volume web search, point-in-time queries, and prompt-guided extraction from known URLs. |
| [`exa-search`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/research/exa-search) | Semantic discovery, academic research, and finding sources related to a strong seed page. |
| [`firecrawl`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/research/firecrawl) | Extracting clean content from selected or JavaScript-heavy pages and mapping or crawling sites. |
| [`nano-banana-pro`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/nano-banana-pro) | Generating or editing Gemini 3 Pro images, including reference-guided and batch workflows. |
| [`gemini-claude-resonance`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/gemini-claude-resonance) | Iterating through shared-memory Claude–Gemini visual dialogue, analysis, and faithful transformation. |
| [`image-forge`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/image-forge) / [`sprite-forge`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/sprite-forge) | Preparing owned replacement art or sprite assets; never repackage copyrighted source data. |
| `parakeet` (local, `~/.claude/skills/parakeet`) | Identifying original WAVE resources by content (see below); two MLX engines, no NeMo. |

## Test Quality Gates

`cargo-mutants` is a CLI installed on the Mini (`cargo install --locked
cargo-mutants`), not a workspace dependency, and runs there like any Cargo
command. It changes small pieces of code and reports each change the tests
fail to notice. A surviving mutant is either a missing test or code no test
can reach; close it or record why it stays (the commit message carries the
equivalents).

```bash
scripts/mini-cargo.sh -- mutants -p rebellion-core \
  --file crates/rebellion-core/src/tick.rs --re 'advance'
```

`--re` limits a run to the touched functions. The output stays on the Mini in
`~/or-native/open-rebellion/mutants.out/` (`missed.txt`, `timeout.txt`);
delete it and `mutants.out.old` before a rerun.

Scope every run to the files a change touches; whole-workspace runs are slow.
The mutant copy includes the ignored `data/base`, but tests that need original
DATs are `#[ignore]`d and a plain run skips them. For seeding or replay code,
append `-- --lib -- --include-ignored` so they run against each mutant.
Mutants in rendering code often survive because the unit tests draw nothing.
Browser acceptance covers that code, so record those survivors instead of
writing tests that only mirror the drawing calls. The same holds for `main`,
`apply_panel_action` and the other code in `rebellion-app/src/main.rs`, which
no native test reaches, and for the WASM fixture bridges
(`interface_test_fixture::emit_*`). Keep new logic out of `main.rs` where a
module can own it and test it; moving the existing action handling out is
roadmap M2 work.

Tests and browser gates that assert original behavior cover both sides. The
sides often run separate classes with different numbering (the advisors'
reaction slots) and different cockpit geometry.

### Provenance Gate

`tools/provenance-scan` parses the simulation code (`rebellion-core` minus
DAT layout and wire format, plus `integrator.rs` and `simulation.rs`) and
flags every function, method, `const`, and `static` that carries a number
other than 0 or 1, or a roll draw, without a recorded source. Tag each item in
the comment block directly above it or inside it:

- a source: a `FUN_`/`DAT_` address, a GNPRTB parameter, a DAT file or table
  name, or `// src: <reference>`;
- `// port: <reason> (F-0xx)` for design the original does not own, such as
  layout or tooling;
- `// hyp: <reason>; recovered by <function or capture>` for a guess that still
  awaits recovery. A hypothesis is counted but never counts as cited.

```bash
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin \
  cargo run -q -p provenance-scan -- check
```

`check` fails on an uncited item missing from `scripts/provenance-baseline.json`
or grown past its count. After citing or tagging items, run `baseline` to
record the smaller set; it refuses to let the baseline grow. `report --json
PATH` writes every item with its hits for review. The scan is item-level: a
cited item still needs a line-by-line check against its decompile, and a
Ghidra note is never a sufficient source alone; cite the `.c` it rests on.

### UI Reachability

The provenance scan covers simulation code only. Before closing a UI
feature, check that every entry point it adds or keeps has a caller a player
can reach, and tag each stand-in for an unported original control `port:`.
F-019 found two whole egui menus with no caller, and a fleet dispatch flow
that went dead with them (F-007C). Invented visible UI is out of bounds
(`docs/qa/2026-09-10-interface-parity-audit/`): remove it, or record it as a
stand-in with the finding that replaces it.

`proptest` is deferred until economy or combat math has edge cases the replay
goldens do not pin. Adding it needs approval as a dev dependency. Formal
verification (`kani`) is out of scope for now.

### Builds and the Build Cache

Cargo runs on the Mac Mini, not this Mac. `scripts/mini-cargo.sh` syncs the
working tree to `mac-mini-ts:~/or-native/open-rebellion` (git's file list:
tracked files plus untracked ones git does not ignore), runs Cargo there and
streams its output:

```bash
scripts/mini-cargo.sh -- test --workspace --features interface-test-fixtures --exclude provenance-scan
scripts/mini-cargo.sh -- run -q -p provenance-scan -- check
scripts/mini-cargo.sh -- build --release -p rebellion-app      # the binary stays on the Mini
scripts/mini-cargo.sh --run ./scripts/build-wasm.sh --pull web/open-rebellion.wasm
```

The Mini holds rustup (the toolchain this Mac uses), the wasm32 target,
rustfmt, clippy and binaryen. Native acceptance runs the binary where it is
built, so nothing is copied back except what `--pull` names. The copy has no
`.git`; `.build-source` records the commit and dirty file count it was synced
from, and `scripts/launch-native.sh` writes that into `binary.txt`.

rsync keeps each file's modification time, so Cargo rebuilds a source only when
its mtime changes. After a hand-applied break on the Mini copy (a mutant check),
touch the local source file before the next `mini-cargo.sh` run, or the synced
copy keeps the old mtime and the broken build stays fresh.

`mini-cargo.sh` sets `CARGO_TERM_COLOR=always`, so its errors and test
results start with ANSI escapes and a bare `grep '^error'` finds nothing.
Strip them first (`sed 's/\x1b\[[0-9;]*m//g'`) or pass `--color never`
before grepping a build or test log.

A private copy for red runs or mutants (`OPEN_REBELLION_BUILD_DIR=or-native/wt-NAME`)
starts as an APFS clone of the shared copy, never a plain `cp -R` or
rsync: `ssh mac-mini-ts 'cd ~/or-native && cp -c -R open-rebellion wt-NAME'`.
The shared copy is about 13.6 GB (`assets/references` 4.1 GB, the Mini's
`.artifacts` 4.1 GB, `target/` 4.3 GB); a clone shares its blocks, so it
takes about 8 s and 0.1 GB. Cargo and `mini-cargo.sh` then write only the
units and files that change. Delete the copy when its run ends.
In a private clone, run cargo-mutants with `--in-place`. Otherwise it
copies the tree again (as clones, since 26.0) and builds every job's
`target/` from nothing, about 3 GB each; in place it reuses the clone's
build. Never use `--in-place` in the shared copy.

After every build `scripts/prune-target.py` cuts the Mini's `target/` back
to what the last six builds used:

- **units**: the wrapper adds `--message-format=json-render-diagnostics`,
  and every unit those builds named, fresh or rebuilt, is kept
  (`target/.prune-history.json`, `--keep-builds`); any other entry of
  `deps/`, `.fingerprint/`, `build/` and `examples/` in their profiles goes.
  Builds that alternate (workspace tests, a fixture feature, one crate)
  stay fresh instead of deleting and rebuilding each other's units. Past
  the size cap only the last build's units stay. The prune first takes
  Cargo's build lock (`<profile>/.cargo-lock`) on every profile, without
  waiting, and prunes nothing while another agent's build holds one.
- **codegen objects**: Cargo's macOS dev default, split-debuginfo
  "unpacked", keeps every build's `*.rcgu.o` under a new name since rustc
  1.88 and never deletes the old ones (rust-lang/rust#161824, fix PR
  #162561 open). An object stays only while an executable's debug map names
  it, so backtraces keep their lines.
- **incremental**: each crate keeps its two most recently used incremental
  directories (a library and its tests). The deps objects are hard links
  into them, so they hold most of the space.
- past 12 GB (`OPEN_REBELLION_TARGET_MAX_GB`) it runs `cargo clean`.

Access times are not used: APFS does not keep them, and `cargo sweep
--stamp`/`--file` deleted fresh units, forcing a full rebuild every other
run. Each prune appends a JSON line to `~/or-native/build-cache.log` on the
Mini. A launchd agent there (`scripts/install-mini-prune-agent.sh`) runs the
object and incremental steps daily at 04:30 as a backstop.

This Mac keeps no `target/`. A local Claude Code SessionStart hook
(`.claude/settings.json`, untracked) runs `scripts/check-build-cache.sh
--auto`, which prints the Mini's last log line and prunes this Mac unless a
cargo, rustc or cargo-mutants process is running:

- `cargo clean` when a local `target/` exists
  (`OPEN_REBELLION_TARGET_LIMIT_GB`, default 0);
- delete stale cargo-mutants copies, and scratch target directories (a
  worktree's `CARGO_TARGET_DIR`) older than two days;
- delete stale run folders: a timestamped run that no tracked file,
  `progress.json` or A0 record names, older than 14 days, and not one of
  the two newest of its kind;
- move evidence untouched for 14 days to the Mac Mini,
  `mac-mini-ts:open-rebellion-archive/.artifacts/<same path>`, deleting the
  local copy only after rsync succeeds. A cited `.artifacts/…` path that is
  missing here is in the archive. `interface-parity/a0` and `site` stay.

Run it without arguments to report, or with `--prune` to prune now. An
editor's rust-analyzer still checks into a local `target/`; the hook clears
it at the next session. Ghidra headless stays on this Mac.

## Voice Resource Identification

Use this to map an original `WAVE` resource ID (TACTICAL, VOICEFXA/E, COMMON DLLs)
to what it says, or to confirm that it is SFX, before wiring it into an audio path:

1. Extract the DLL's `WAVE` resources to a directory outside the repository. Most
   are 8-bit unsigned PCM at 11,025 Hz, and ffmpeg decodes them directly.
2. Run `~/.claude/skills/parakeet/scripts/batch_transcribe.py <dir> --engine both
   --out .artifacts/voice-id/<dll>.jsonl` without hotwords first.
3. Read `status`. `agreed` is corroborated speech and `sfx_likely` is non-speech.
   `needs_listen` means a person must listen before the evidence cites it. You
   may re-run only those clips with `--context-file` and a small proper-noun
   primer as a third opinion. Dumping every TEXTSTRA string made Qwen3 recite
   the list. Never commit a derived vocabulary file.
4. Evidence cites the resource ID, the WAVE `sha256`, the engine models and
   revisions, and the verified line. Commit IDs and hashes, never the audio.

ASR is evidence, not authority. A transcript alone never closes a parity cell.

Baseline, 2026-09-24: 61 of TACTICAL.DLL's 66 `WAVE` resources (13000–13065)
are `sfx_likely` under both engines. Five are `needs_listen`: 13034, 13046, and
13052, plus 13019 and 13059, where Qwen3 hears a single plausible word ("Yeah.",
"Okay.") and Parakeet hears nothing. All 285 VOICEFXA/VOICEFXE resources are
speech: 270 agreed and 15 need a listen.

## Guardrails

- Skills assist implementation and verification; they do not lower a feature's
  acceptance criteria or turn an inferred result into a pass.
- Keep passwords, API tokens, session secrets, and proprietary game assets out
  of prompts, reports, screenshots, and commits.
- Preserve unrelated work. Stage explicit paths, update the evidence ledger,
  and commit and push each verified feature before advancing.
