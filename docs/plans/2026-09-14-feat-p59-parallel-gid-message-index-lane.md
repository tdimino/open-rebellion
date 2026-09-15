---
title: "P59 Parallel Strategic Lane: GID Contract, Owned A0 Captures, and Message Index Input"
description: "A non-tactical interface-parity bundle that runs beside the Codex P58E tactical lane in one working tree, delegating implementation, review, and browser acceptance through codex-orchestrator"
category: plan
type: feat
status: active
created: 2026-09-14
updated: 2026-09-14
date: 2026-09-14
parent: 2026-09-11-feat-batched-interface-parity-plan.md
tags: [interface, parity, gid, message-index, a0, windows-vm, codex, astra, sol, harness]
---

# P59 Parallel Strategic Lane

## Overview

Three Codex CLI sessions own the `UIP-B06` tactical lane and are mid-way through
P58E (task-force and fighter-group presentation). Their edits are uncommitted in
the shared working tree on `main`, and project policy forbids side branches and
worktrees. This plan opens a second lane in the same tree that never touches a
file the tactical lane has dirty, sequences the few shared files behind the P58E
commit, and delegates implementation, independent review, and browser acceptance
to `codex-orchestrator` subagents under the rules in `AGENTS.md` and
`agent_docs/agent-tooling.md`.

The lane closes three blockers that keep every strategic family at zero accepted
cells:

1. **RE-GID-01 is static-partial.** The GID menu interior, typography, expanded
   180x240 legend, non-support thresholds, and special markers are provisional
   because `FUN_00607740`, `FUN_00426d00`, and `FUN_00427010` are zero-byte
   exports in `ghidra/notes/`. An empty export is not negative evidence; the
   saved Ghidra project has never been queried for them.
2. **No A0 capture exists.** All 992 GID comparison slots are `unbaselined`.
   `baselines/accepted-original.json` is empty, so no golden can be created.
   The ledger already carries this as `RE-A0-01`, runtime-needed across every
   family. Producing A0 evidence means running the unmodified `REBEXE.EXE`, and
   the only defensible environment for that is Windows itself. This is a
   decision, not a task; see A0-01 below.
3. **CMD-08 has no input path.** The Message Index rail draws resting art only;
   commands `0x136..0x13e` are never dispatched, and `FUN_0042a240` is a
   zero-byte export.

Everything below is an implementation checkpoint until its cell passes the full
A0, native, browser, faction, viewport, interaction, and diagnostics matrix. No
slice in this plan promotes a cell on its own.

## Problem statement

The batched plan names the complete `CMD-02` family as the next `UIP-B02`
bundle and the acceleration sidecar defines its gate, but two of its inputs are
missing: the recovered menu contract and an accepted original capture. Working
those inputs in the tactical lane's shadow requires an explicit file-ownership
contract, because `run.mjs`, `main.rs`, `web/open-rebellion.wasm`, the shared
`target/` directory, and the fixture site directory are all touched by both
lanes. `scripts/build-interface-test-wasm.sh` calls `scripts/build-wasm.sh`
first, which copies over the tracked production binary the peers have dirty,
with no restore path.

## Lane ownership and shared-tree rules

| Set | Files | Rule |
|---|---|---|
| Codex P58E, dirty now | `crates/rebellion-render/src/tactical_view.rs`, `tactical_resources.rs`, `crates/rebellion-app/src/tactical_test_fixture.rs`, `crates/rebellion-app/src/main.rs` (tactical test hunks), `tools/interface-parity/run.mjs` (tactical probes, `verifyTacticalBitmap` signature), `tools/interface-parity/scenarios/tactical.catalog.json`, `web/open-rebellion.wasm` | Never edit, never stage, never rebuild over. Untracked `EVALUATE.md`, `program.md`, `run-eval.sh`, `edd.config.json` belong to another session and stay untouched. |
| P59 exclusive | `crates/rebellion-render/src/cockpit.rs`, `lib.rs` (GID functions only), `bmp_cache.rs` (constants only), new `crates/rebellion-app/src/message_index_test_fixture.rs`, new `tools/interface-parity/message-index-probes.mjs`, new `scenarios/message-index.catalog.json`, `schemas/scenario-catalog.schema.json`, `validate-catalog.mjs`, `scripts/build-interface-test-wasm.sh`, new `tools/original-capture/`, `ghidra/notes/`, evidence and ledgers under `docs/qa/2026-09-10-interface-parity-audit/` | Edit freely. Before every edit run `git status --short` and confirm the target is not in the peers' dirty set. |
| Shared, sequenced after the P58E commit | `run.mjs` (family-selector hunks at lines 23-26, 99, 117-118, 1550-1556, 1581, 2067-2068, 2090-2095), `main.rs` (six-line namespace-2 dispatch beside line 1007), `verify-production-exclusion.mjs` (new forbidden tokens), `web/open-rebellion.wasm` | Do not touch until `git log` shows the P58E commit and `git status` no longer lists the file. Fallback only if the peers stall for more than one working day: craft a patch limited to P59 hunks and apply it with `git apply --cached`, as done for `agent_docs/skills.md` on 2026-09-14. |

Operating rules for the whole lane:

- **Builds.** Until the P58E commit lands, every harness run uses `--no-build`
  against the existing `.artifacts/interface-parity/site/`. Slice P59A0 adds an
  `OPEN_REBELLION_SKIP_PRODUCTION_WASM=1` guard to
  `scripts/build-interface-test-wasm.sh` so a fixture build never calls
  `build-wasm.sh`; that guard is used for every P59 inner-loop build. Any
  fixture artifact built from a tree containing peer edits is recorded in
  evidence as `HEAD <sha> + dirty tactical lane` and is never cited for
  acceptance.
- **Fixture site.** Both lanes serve `.artifacts/interface-parity/site/`. Announce
  every fixture build on `claude-peers` before running it and never build while
  a peer matrix is running. P59A adds `OPEN_REBELLION_FIXTURE_SITE` so P59 can
  build into `site-p59/` once `run.mjs` is editable.
- **Cargo.** The shared `target/` serialises on the cargo lock and the
  `interface-test-fixtures` feature toggle thrashes rebuilds. Run focused tests
  with `-p rebellion-render` and `-p rebellion-app` only; if a peer's
  half-edited `tactical_view.rs` breaks compilation, log the failing peer file
  and retry, never report the P59 slice red.
- **Native display.** Only one lane runs native macroquad captures at a time.
- **progress.json.** Four sessions write it. Re-read immediately before every
  write and append only the P59 entries.
- **Commits.** Stage explicit paths. `web/open-rebellion.wasm` is staged only
  when rebuilt from a tree whose `git status` shows no other lane's crate
  changes. No `Co-Authored-By` trailers.
- **Chrome.** Every scenario is a fresh muted process on an ephemeral port
  (`server.listen(0)`), so concurrent matrices do not collide on ports; they do
  raise `stableFrame` flakes, so P59 runs its matrices when the peers are idle.

## Proposed solution: slices

Each slice changes at most three substantive files. Docs, ledgers, and evidence
do not count. Order: P59B, E1, and P59A0 start now; A0-01 starts only after its decision is
taken; P59A, P59C, P59D, and E2/E3 wait for the P58E commit; the gate runs
once.

### P59B. RE-GID-01 recovery (docs only, start now)

Targets, with current export size in `ghidra/notes/`:

| Function | Bytes | What it must yield |
|---|---:|---|
| `FUN_00607740` | 0 | GID menu paint and layout: interior fill (resource or GDI brush), row height, text origin, font creation (`CreateFont` face, height, weight) or bitmap glyphs, submenu placement |
| `FUN_00426d00` | 0 | Non-support marker size thresholds per mode, or the table they read |
| `FUN_00427010` | 0 | GID caption and display-window construction, including the 180x240 expanded legend path and its resources |
| `FUN_00425d00` | 18,867 | Filter and overlay predicates per command ID `0x11..0x80`; special markers 10160, 10166, 10167, 10169, 10170 |
| `FUN_0042b330` | 19,569 | Support thresholds (already used), legend selection |
| `FUN_004511e0` | 14,650 | Menu command tree (already used), keyboard and focus routing |

Method: open the saved project (`ghidra/Open Rebellion Ghidra.gpr`), decompile
through GhidraMCP on `:8080` or `pyghidra-mcp` per `agent_docs/ghidra-re.md`,
and re-export each function with a header recording exporter, decompiler
version, and byte size. Write `ghidra/notes/gid-menu-contract.md` as a claim
table: one row per geometry, resource, predicate, or typography fact, each with
`verdict: proven | inferred | unproven`, the function and offset, and the
runtime capture that would corroborate it. Update RE-GID-01 in both ledger
files.

Delegation: Claude performs the Ghidra work. A Sol `adjudicator` at `xhigh`
weighs every `inferred` row against the attached decompiles from a `/tmp` copy
and returns a per-claim verdict; P59C consumes only rows that end `proven`.

Typography decision point: if `FUN_00607740` draws with a GDI font, the exact
face is Windows-licensed and cannot ship. That finding stops at the ledger and
is raised to Tom as a decision (bitmap-font reproduction, metric-compatible
substitute, or accept the cell as blocked). It is not solved inside this plan.

Files: `ghidra/notes/*.c`, `ghidra/notes/gid-menu-contract.md`,
`docs/qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.{md,json}`.

### A0-01. Owned original captures: a decision, then a protocol

The point of running `REBEXE.EXE` is not to study Windows behaviour. The
acceptance rule the ledgers encode says a cell passes only against a lossless
640x480 frame from the unmodified original executable, so that the project
never certifies itself against its own renderer. Every checkpoint since P46 is
A1 evidence, which is why the harness can be 152 of 152 green while the ledger
reads 0 of 564. Original frames also settle the open RE questions (menu
typography, legend geometry, marker thresholds, tactical camera and lighting)
faster than decompiling empty exports.

Two paths. Tom chooses one before this slice starts; nothing else in the plan
waits on it.

**Path 1, close cells: set up a Windows VM once and script the capture
protocol against it.** UTM is free, Windows 11 ARM runs 32-bit x86 through
Microsoft's own emulation, and the GOG build ships its compatibility database
(`goggame_w8.sdb`) and `d3drm.dll` for modern Windows, so this is the
environment GOG certified. Cost is roughly two to three hours once plus a
Windows ISO, and it serves every family including the tactical lane.

**Path 2, defer acceptance: drop A0-01 from this plan and run P59B, P59A0, and
the Message Index work now.** Checkpoints keep shipping as A1 evidence. The
honest consequence is that the 0 of 564 figure does not move until Windows
access is convenient.

Wine Stable 10.0 is present locally and can boot the GOG build for discovery
(font substitution and an incomplete Retained Mode make it a weak witness), but
a Wine frame is never registered as A0. If used at all, it is classified as
discovery evidence beside the existing video frames.

Protocol for Path 1, validated at the smallest scale first:

1. UTM guest with Windows 11 ARM, 640x480-capable display, the GOG installer
   run as shipped (its script installs DirectPlay, `d3drm.dll`, and the
   compatibility database), English locale, default Windows fonts only.
2. Launch `REBEXE.exe -w` (GOG window mode) so no display-mode switch occurs.
   Capture with a lossless in-guest tool that reads the window client area
   directly (no host-side screenshot through the VM's scaled framebuffer), after
   two consecutive hash-equal frames, the same discipline as `stableFrame`.
3. Verify the capture is exactly 640x480 with no decoration. Fidelity metric:
   100% of pixels in the shell aperture region must be members of the decoded
   palette of STRATEGY 900/901, and one known control (Alliance GID normal
   10012 at 446,406, 27x16) must match the decoded resource pixel-for-pixel. A
   capture that fails either check is recorded, not registered. Record Windows
   build, GOG build id, installed fonts, capture rect, and cursor visibility in
   `capture-metadata.json`.
4. Storage: `docs/qa/2026-09-10-interface-parity-audit/reference-captures/owned-a0/`
   with a local `.gitignore` (`*.png`), a committed `README.md`, `SHA256SUMS`,
   and `capture-metadata.json`. The existing 370-image `SHA256SUMS` and
   `reference-ledger.json` `manifest_sha256` are not modified; the reference
   ledger gains a separate `owned-a0` source block with `authority: A0`. The
   audit directory is force-tracked by `.gitignore`, so the local ignore rule
   is mandatory before the first capture exists.
5. Register each passing capture in `tools/interface-parity/baselines/accepted-original.json`
   keyed by scenario id with path and sha256.

Exit ladder, each level recorded as reached or failed with the exact error:

| Level | Proof |
|---|---|
| L0 | Guest boots, GOG installer completes, `REBEXE.exe` window appears |
| L1 | Main menu visible |
| L2 | Alliance campaign entered, shell captured and verified |
| L3 | Empire shell captured and verified |
| L4 | Popular Support GID (menu closed) captured, both factions |
| L5 | GID root menu open captured, both factions |
| L6 | Message Index window captured, both factions |

Levels reached below L6 narrow which cells can later be baselined; the evidence
file enumerates the unclaimed ones.

Progress (2026-09-15): Path 1 is standing. The Windows 11 Arm64 guest is built
in UTM on the Mac Mini, the GOG build is installed, and the unmodified
`REBEXE.exe` runs to its main menu, reaching L0 and L1. Three findings shaped
the rig and are recorded in `tools/original-capture/README.md`: the answer file
must sit on a CD because Windows Setup searches removable media only and QEMU
presents USB disks as fixed; the 32-bit game reads its data locations from the
`WOW6432Node` registry view; and the VM must carry a sound card, without which
`DirectSoundCreate` leaves an unusable interface and the game faults on startup
(`0xc000041d`). The L0 launch frame is captured and its sha256 recorded. L2-L6
remain.

Files: `tools/original-capture/capture.md` (protocol), `tools/original-capture/verify_capture.py`
(palette membership, dimension and decoration checks, SHA registration),
`tools/original-capture/README.md`. Delegation: Claude runs the captures; a Sol
`reviewer` at `high` checks `verify_capture.py` before any hash is registered.

### P59A0. Fixture build guard (one file, start now)

Add `OPEN_REBELLION_SKIP_PRODUCTION_WASM=1` to
`scripts/build-interface-test-wasm.sh` so the production `build-wasm.sh` step is
skipped and `web/open-rebellion.wasm` is never rewritten by a fixture build. The
script is not in the peers' dirty set. Test: run with the guard, confirm
`git status` shows `web/open-rebellion.wasm` unchanged in content hash.

### P59A. Harness family selector (after P58E commit)

Replace the `battle` boolean with `--family=gid|tactical|message-index`
(`--battle` kept as an alias). Touch only the listed `run.mjs` lines: option
parsing, `scenarioId` prefix map (`gid`, `tactical`, `message-index`, existing
ids unchanged), ready-status map (`ready`, `battle-ready`,
`message-index-ready`), per-family smoke slugs (a message-index catalog under
`--smoke` must not yield zero scenarios), and the catalog assertions.
`validate-catalog.mjs` gains `--catalog <path>` with the GID catalog as the
no-argument default, since `build-interface-test-wasm.sh` calls it bare. The
schema becomes a `oneOf` keyed by `family`: the `CMD-02` branch is byte-for-byte
the current constraints (29..38 scenarios, index max 37, no extra properties);
a `CMD-08` branch allows `fixture_namespace: 2` and `status`. The tactical
catalog keeps its inline asserts; making it schema-validated is new coverage
and is out of scope here. `scripts/validate-interface-parity-ledgers.mjs`
continues to bind the GID catalog to `CMD-02` and must bind the new catalog by
its `family: "CMD-08"` value.

Files: `run.mjs`, `validate-catalog.mjs`, `schemas/scenario-catalog.schema.json`.
Tests: `npm run test:unit`, `npm run check`, `node run.mjs --smoke --no-build`
for `gid` and `tactical` unchanged, `--family=message-index --smoke` fails
cleanly until P59E supplies the catalog.

Delegation: Sol `builder` at `high` implements from this specification in a
`/tmp` clone, returns a patch; Claude applies it after confirming the diff stays
inside the listed hunks; Sol `reviewer` at `high` reviews the applied diff.

### P59C. Apply the proven GID contract (after P59B verdicts)

Consume only `proven` rows. Candidates: `gid_popup_frame` fill, `gid_menu_row`
geometry and text origin, expanded legend window and its resources,
`draw_gid_caption` source, `gid_metric_size` thresholds per mode. Anything
`inferred` or `unproven` stays as it is and is listed under "not claimed".
Keep `verifyGidRootFrame` and `verifyMessageIndexRail` green; where the interior
becomes a bitmap, add an interior pixel check against the decoded resource in
the native 640x480 variant only. Any constant renamed in `bmp_cache.rs` is
grepped against the peers' dirty diff first.

Files: `cockpit.rs`, `lib.rs`, `bmp_cache.rs`. Tests: unit tests for every
geometry table and threshold, `cargo test -p rebellion-render`, native snapshot
of both faction menus, GID matrix `--family=gid --all --no-build` (152 cases).

Delegation: Claude implements (RE-derived geometry is exactly the risk class
that warrants review); Sol `reviewer` at `xhigh` reviews the diff with the
claim table attached.

### P59D. Hover, selection, and special markers (conditional)

Wire a resource only when P59B marks both its predicate and its paint site
`proven`: 10166 selection, 10167 hover, 10160 multi-faction fleet, 10169/10170
special. The strategic controls have no hover art (`FUN_00602d30`), so a hover
pixel delta anywhere outside a proven ring is a synthetic-art violation; the
harness diffs hover against rest and asserts the delta equals the ring pixels.
Precedence between selection and hover comes from RE or the slice withholds
both. Add exclusive right and bottom edge probes at `x = right` and
`y = bottom`. If none of the five resources reaches `proven`, skip the slice
and record it.

Files: `lib.rs`, `cockpit.rs`, `run.mjs` (GID probe section only). Delegation as
P59C.

### P59E. CMD-08 Message Index input (three sub-slices)

**E1, RE-MSG-01 (docs, can start now).** Re-export `FUN_0042a240` from the
saved project; read the populated `FUN_00422ce0` (55 KB) dispatch for commands
`0x136..0x13e` and category values `0x7a..0x82`; determine whether the
`10060..10068` / `10040..10048` resources are held-press art or an unread
predicate; recover the Message Index window constructor and category
mapping. Claim table and Sol `adjudicator` at `xhigh` as in P59B.

**E2, rail input and fixture (after P58E commit and E1 verdicts).** In
`cockpit.rs`: hit-test the nine `MessageIndexControlSpec` rectangles with
exclusive edges, capture on press, dispatch the command on release inside the
rectangle, and log `[interface] command=0x13n destination=message_index
status=pending_original_window` so the index window fails closed as a captured
negative control. Swap to the illuminated resource while held only if E1 proves
it is press art; if E1 proves an unread predicate instead, wire that predicate
and no held swap; if unproven, dispatch commands with no art change. New
`message_index_test_fixture.rs` (compiled under the same `cfg` as the GID
fixture) claims namespace `(code >> 16) == 2`, emits `schema_version: 1`,
`status: "message-index-ready"`, and resets the rail to resting so no lit state
leaks into GID fixtures. `main.rs` gains the six-line dispatch beside the
tactical one.

Files: `cockpit.rs`, `message_index_test_fixture.rs`, `main.rs`. Tests: decode
tests for namespace 2 and rejection of `0x20000`, a bijection test that nine
controls map to nine distinct commands, unit tests for exclusive edges and
capture release outside the rectangle.

**E3, catalog and probes.** `scenarios/message-index.catalog.json` with
`family: "CMD-08"`, `fixture_namespace: 2`, scenarios `resting`, `held-<n>`
for each control, `release-outside`, `index-window-fail-closed`, and
`unread` only if E1 proved it. `message-index-probes.mjs` (imported by
`run.mjs` through the family selector) probes: resting pixels for all nine
controls versus decoded BMPs at 640x480; mouse-down, stable capture, pixel
check, mouse-up, resting again; command event registered before input and
observed exactly once; outside-edge and exclusive-edge negatives; the
fail-closed capture recorded as a negative control. `verify-production-exclusion.mjs`
gains `message_index_test_fixture` and `message-index-ready` tokens.

Files: `message-index.catalog.json`, `message-index-probes.mjs`,
`verify-production-exclusion.mjs`. Delegation: Sol `builder` at `high` writes
the probes from this specification; Sol `reviewer` at `high` reviews; Claude
integrates.

### Gate (once, after all slices)

Follow the sidecar ladder exactly:

1. Focused tests and scoped clippy for touched crates (`CLIPPY_CONF_DIR`
   pointed at a clean directory, stated in evidence).
2. Fixture WASM with the P59A0 guard, `npm run check`, production exclusion.
3. Muted fresh-process matrices: `--family=gid --all --no-build`,
   `--family=message-index --all --no-build`, both factions, 640x480 and
   1280x800. Launch statistics go into the evidence table; assertion failures
   are never retried.
4. Native 640x480 captures of the same states (see "Not claimed" for the
   native fixture gap).
5. Astra `medium` reviews the generated evidence packet (contact sheets,
   lossless PNGs, request, console, hash, and cleanup summaries) plus one full
   GID traversal and one Message Index traversal per faction, and returns a
   severity-ranked verdict with `ready_to_commit`. Final artifact hashes are
   computed after the last rebuild and pasted into the prompt; any rebuild
   invalidates the prompt.
6. Sol `xhigh` review of the RE-derived geometry diff and the ledger changes.
7. Golden creation with `--update-goldens` only for scenarios whose A0 capture is
   registered; every other comparison stays `unbaselined`.
8. `node scripts/validate-interface-parity-ledgers.mjs --write`, then `--check`.
9. Rebuild production WASM from a tree with no other lane's crate changes,
   stage explicit paths, one atomic commit per verified slice, push `main`.

## Delegation matrix (codex-orchestrator)

`~/.claude/skills/codex-orchestrator/scripts/codex-status.sh` passes today
(codex-cli 0.154.0, key set, all 13 profiles). Personas are process-local, so
parallel launches are safe; agents that write overlapping files are serialised.
Every write-capable agent works in a `/tmp` clone and returns a patch; the
repository stays read-only to agents.

| Slice | Implement | Review | Acceptance |
|---|---|---|---|
| P59B, E1 | Claude (Ghidra) | `codex-exec.sh adjudicator "<claims + decompiles>" --reasoning xhigh --no-approve` | none |
| A0-01 (Path 1 only) | Claude | `codex-exec.sh reviewer "<verify_capture.py>" --reasoning high --no-approve` | none |
| P59A0 | Claude | none (one guard line) | `npm run check` |
| P59A | `codex-exec.sh builder "<spec>" --reasoning high` in `/tmp` clone | `codex-exec.sh reviewer "<diff>" --reasoning high --no-approve` | `--smoke` both existing families |
| P59C, P59D | Claude | `codex-exec.sh reviewer "<diff + claim table>" --reasoning xhigh --no-approve` | GID matrix |
| E2 | Claude | `codex-exec.sh reviewer "<diff>" --reasoning high --no-approve` | focused tests |
| E3 | `codex-exec.sh builder "<spec>" --reasoning high` in `/tmp` clone | `codex-exec.sh reviewer "<diff>" --reasoning high --no-approve` | Message Index matrix |
| Gate | Claude | Sol `xhigh` on RE geometry and ledgers | `codex-astra.sh reviewer "<evidence packet + site URL>" --reasoning medium` |

Astra receives no source, reverse-engineering, or documentation review. The
`--no-approve` flag forces the read-only sandbox for every reviewer and
adjudicator run.

## System-wide impact

- **Interaction graph.** A GID leaf selection logs a command, sets
  `CockpitState.gid_mode`, and repaints markers through `gid_marker_for_system`;
  P59C changes thresholds inside that path, so every GID scenario's marker
  sizes can shift. A Message Index press flows through
  `handle_cockpit_egui_input` into the same command log the harness registers
  expected events against; E2 must emit exactly one event per release.
- **Error propagation.** Missing resources fail closed and log once through the
  `BmpCache` negative cache; any new constant that does not exist in the staged
  pack surfaces as a missing-asset diagnostic that fails the four-request
  startup assertion. Fixture decode failures return `None` and the app boots
  without a fixture, which the harness treats as a failed ready record.
- **State lifecycle.** The Message Index fixture resets rail state; the GID
  fixture never sets it. Held state must clear on release outside the rectangle
  and on focus loss. No persistence formats change.
- **API surface parity.** Both factions share every table; every probe runs
  per faction. The native path and the WASM path share `cockpit.rs`, so native
  captures inherit P59C automatically.
- **Integration scenarios.** GID menu open under a foreground system window;
  Message Index press while the GID menu is open (must dismiss or ignore per
  RE); Empire rail at `x = 611` near the canvas edge at 1280x800 letterboxing;
  zoomed map with the expanded legend open; Escape with the menu open and a
  control captured.

## Edge cases and tests

- Shared tree: peer file appears dirty mid-slice (abort the edit, wait);
  cargo lock held by a peer build (retry, do not kill); site swapped under a
  running matrix (announce, wait).
- A0: capture taken through the VM's scaled framebuffer instead of the guest
  client area (reject), palette drift from colour management (reject and
  record), cursor captured over a control (retake), window decoration in rect
  (reject), non-default guest fonts (record and flag), a Wine frame offered as
  A0 (refuse; discovery only).
- P59A: `--battle` alias still selects tactical; `--smoke` with the new family
  never yields zero scenarios; existing scenario ids and golden paths unchanged
  (assert by listing `baselines/implementation/` before and after).
- P59C: threshold changes must be covered by a unit test per mode; interior
  pixel check only at native size; frame border checks unchanged.
- P59D: hover delta equals ring pixels or zero; `x = right`, `y = bottom`
  outside; selection and hover coincident.
- P59E: code `0x20000` (scenario 0) rejected; `0x2010a` (unknown scenario)
  rejected; release outside rectangle dispatches nothing; nine commands
  bijective; index window fail-closed captured; rail resting after fixture
  reset in GID scenarios.

## Acceptance criteria

- [ ] RE-GID-01 and RE-MSG-01 claim tables exist with per-claim verdicts, re-exported
      function bodies with exporter metadata, and updated ledger entries.
- [ ] A0-01 decision recorded (Path 1 or Path 2). Under Path 1 the ladder is
      recorded to the level reached, every registered capture passes palette
      membership and the control pixel check, and only hashes and metadata are
      committed. Under Path 2 the evidence states that no cell can close.
- [ ] `build-interface-test-wasm.sh` with the guard leaves `web/open-rebellion.wasm`
      byte-identical.
- [ ] `run.mjs --family=gid --all --no-build` passes 152/152 and
      `--family=tactical --all --no-build` passes the peers' full count after P59A.
- [ ] P59C ships only `proven` claims; every unshipped claim is listed under "not
      claimed" in its evidence file.
- [ ] P59D either ships proven resources with zero synthetic pixel delta or is
      recorded as skipped with the verdicts that blocked it.
- [ ] Message Index: nine distinct commands dispatched once each, held and resting
      pixels exact at 640x480, fail-closed window captured as a negative control,
      production exclusion passes with the new tokens.
- [ ] Astra `medium` returns `ready_to_commit: true` for the evidence packet; Sol
      `xhigh` findings are resolved, not deferred.
- [ ] Ledger validator `--check` clean; every evidence bundle has its `README.md`
      and is linked from `evidence/README.md`, `index.md`, the audit README, the
      batched plan, and the roadmap.
- [ ] No strict cell is marked passed unless its A0 capture is registered, its
      native and browser captures match, and its interaction matrix passes.

## Not claimed by this plan

- Native macroquad fixture entry does not exist (T1 status). Native
  comparisons in this plan come from a manually entered campaign and are
  recorded as `native-manual`, which does not close a strict cell. A
  feature-gated native CLI fixture flag is a separate slice that touches the
  `main.rs` CLI and needs its own approval.
- The original Message Index window, categories, navigation, clear, and delete
  (CMD-08 cells beyond `indicator` and `unread`).
- Any GID typography that depends on a Windows font.
- Cross-browser evidence and the P58E tactical work.

## Risks

| Risk | Mitigation |
|---|---|
| Peers never commit P58E during this lane | P59B, E1, A0-01, P59A0 need nothing from them; after one working day use the `git apply --cached` fallback for shared hunks and record it |
| Path 2 chosen, or the VM is not set up | Ledger stays at 0 of 564 by design; every P59 slice still ships as an A1 checkpoint; the decision is recorded in the evidence file so nobody reads green runs as acceptance |
| VM captures differ from a real machine | Palette membership and control pixel checks reject drift; Windows build and fonts recorded per capture |
| Ghidra project also lacks bodies for the empty exports | Record `unproven`, keep synthetic interior withheld, raise typography decision |
| Concurrent matrices flake | Run P59 matrices when peers are idle; never add assertion retries |
| Build clobbers the peers' WASM | P59A0 guard first; `--no-build` until then |
| Reviewer flags unverifiable geometry | Every geometry constant cites function and offset; adjudicator verdict attached |

## Documentation plan

- Evidence: `evidence/2026-09-1x-gid-menu-contract.md` (`p59b-gid-menu-contract/`),
  `evidence/2026-09-1x-owned-a0-captures.md` (`a0-01-owned-captures/`),
  `evidence/2026-09-1x-gid-contract-applied.md` (`p59c-gid-contract/`),
  `evidence/2026-09-1x-message-index-rail-input.md` (`p59e-message-index-rail/`),
  each with the bundle acceptance record fields and an unclaimed-cells section.
- Ledgers: `surface-ledger.json` `current_implementation` text for CMD-02 and
  CMD-08, `reverse-engineering-ledger.{md,json}`, `reference-ledger.json`
  `owned-a0` block, `audit-report.{json,md}`, catalogs; regenerate status
  markers.
- Indexes: `evidence/README.md`, `index.md`, audit `README.md`, batched plan
  "Current execution focus", roadmap checkboxes, `docs/plans/INDEX.md`,
  `tools/interface-parity/README.md` (family selector, new catalog),
  `agent_docs/agent-tooling.md` only if the delegation rules change (they do not).
- `progress.json` entries P59B, A0-01, P59A0, P59A, P59C, P59D, P59E.

## Sources

- Batched plan `docs/plans/2026-09-11-feat-batched-interface-parity-plan.md`;
  sidecar `2026-09-11-tooling-interface-parity-acceleration-sidecar.md`
  (ACC-01 to ACC-06, verification ladder); launcher plan
  `2026-09-12-tooling-standalone-space-battle-launcher.md` (T1 native flag gap).
- Evidence: `2026-09-12-gid-browser-harness.md`, `2026-09-12-gid-menu-frame.md`,
  `2026-09-12-gid-hover-and-occlusion.md`, `2026-09-11-popular-support-gid.md`,
  `2026-09-12-message-index-rail.md`, `2026-09-14-cockpit-routing-correction.md`,
  `2026-09-11-strategic-command-controls.md` (input contract).
- Code: `crates/rebellion-render/src/cockpit.rs:88-113, 236-247, 471-493,
  773-794, 842-883, 1041-1347`; `crates/rebellion-render/src/lib.rs:213-218,
  446-461, 506-744`; `crates/rebellion-render/src/bmp_cache.rs:395-655`;
  `crates/rebellion-app/src/interface_test_fixture.rs:153-169, 343-361`;
  `crates/rebellion-app/src/main.rs:983-1015, 3123-3155`;
  `tools/interface-parity/run.mjs:23-36, 76, 98-118, 265-562, 1550-1583,
  2046-2141`; `tools/interface-parity/verify-production-exclusion.mjs:11-18`;
  `scripts/build-interface-test-wasm.sh:8`; `scripts/build-wasm.sh:24`;
  `.gitignore:84-88`.
- Ledgers: `surface-ledger.json` (CMD-02 29 cells, CMD-08 16 cells, 0/564),
  `reverse-engineering-ledger.md:39-40, 135-163` (RE-GID-01, RE-MSG-01,
  RE-A0-01), `screenshot-ledger.md:224-244, 287-290`,
  `audit-report.md:224-230` (A0/A1/A2 classes).
- Tooling: `~/.claude/skills/codex-orchestrator/SKILL.md` and
  `references/subagent-patterns.md`; `AGENTS.md` lines 24-32;
  `agent_docs/agent-tooling.md`; `agent_docs/ghidra-re.md`.
- Local environment: `~/Desktop/Programming/star-wars-rebellion/` (GOG build
  58708607120002072, `REBEXE.exe -w`, `goggame_w8.sdb`,
  `__support/add/d3drm.dll`); `/Applications/Wine Stable.app` (wine-10.0,
  discovery only); no Windows VM, no CrossOver, no XQuartz.
