---
title: "PR #46 Review: Strategic Windowing Surface Migration"
type: tooling
status: active
progress: "30 of 31 beads"
epic: or-ep-pr46-review-du66
created: 2026-11-04
updated: 2026-11-04
tags: [review, windowing, pull-request, contributor]
---

# PR #46 review: strategic windowing surface migration

A plan for reviewing Will Forster's PR #46 ("feat(windowing): complete
strategic surface migration") well enough to merge it, repair it, or send
it back with named findings. PR #45, the same initiative's first half, was
reviewed from eight findings, repaired by us on the fork branch, and
squash-merged as `9533b516`; this plan follows that precedent and scales it
to a PR eight times larger.

## Goal

Decide, on evidence, whether PR #46 can merge into `main`. The review is
done when every finding is verified or dismissed, every test the PR deletes
is accounted for, the gates and live journeys pass on the PR's own head, and
Tom has a recommendation with its findings ledger.

## The PR

Facts from `gh pr view 46` and `git diff origin/main...origin/pr-46` on
2026-11-04:

- Head `44704a7e` on `will-forster/open-rebellion`, branch
  `feat/strategic-windowing-fleet-lifecycle`; maintainers may push to it.
  72 commits from 2026-10-15 to 2026-10-25, two of them merges from `main`.
- Its merge base is `main`'s head, `e8ae0457`, so the diff is exactly the
  PR's content, and GitHub reports it mergeable and clean. `main` has not
  moved since 2026-10-22.
- 79 files, +56,440/−10,630. `crates/rebellion-app/src/main.rs` alone is
  +18,193/−6,932 and grows from 17,598 to 28,859 lines.
  `sector_window.rs` +5,946, `strategic_windowing.rs` +4,683, and about
  9,000 lines of new `tools/interface-parity` gate scripts and their tests
  follow.
- Scope, by the PR's own account: manager-owned lifecycles for the Fleet,
  Troop and Personnel Finders, Encyclopedia, Message Index, Status, Build
  Selection, the System, Fleet, Defenses, Missions and Sector windows, the
  reference rail, command modals, the Tactical Alert and Results scenes,
  saved-interface ownership, and a Linux X11 focus-loss observer.
- No Cargo manifest changes, no save-format change (`SAVE_VERSION`
  untouched), no beads data. `web/open-rebellion.wasm`, which is tracked,
  is replaced by a binary built on the contributor's machine.
- After the final merge from `main` the PR reran unit suites only
  (`--test-threads=1`); by its own account the browser and native journeys
  were not repeated. Its native evidence is Linux, and its evidence record
  names reports kept on the contributor's machine.
- No reviews, no comments, no checks on GitHub.

## What the first pass found

Each item is a lead for a review lane, not yet a finding.

1. **The PR deletes 35 tests outright** (39 removed, four re-added under the
   same name). Deleting tests is acceptable here: tests bloat, and the
   project's view of them has changed during this work. What matters is
   whether the behaviour they guarded still holds. Most are this
   repository's own recent work:
   `window_focus.rs` is deleted whole (13 tests, the `GalaxyFocusState`
   focus model traced from `FUN_00422ce0` and its neighbours, landed in
   `4f280da8`, `ff3099fd` and `072d8289`), with six save/restore window
   tests from `main.rs`, five rail and minimize tests from
   `system_window.rs`, three after-Escape ranking tests, seven
   `strategic_windowing.rs` and `windowing_input.rs` tests (most guard the
   legacy bridge) and the movie test that closes the Status window,
   Encyclopedia and Message Index. The full list is in Appendix A.
2. **The focus model changes owner.** The PR "removes renderer-local focus
   serial authority". The rules the deleted tests held (a new window takes
   no focus; the first GID press focuses the menu; Escape closes the focused
   window and otherwise does nothing, `FUN_00422ce0` having no `VK_ESCAPE`
   case) are also what `agent_docs/dev-commands.md` documents for native
   runs. Whether the manager keeps each of them is the review's central
   question.
3. **Evidence breaks the repository's rules.** The new evidence record
   (`docs/qa/…/evidence/2026-10-25-strategic-windowing-acceptance.md`)
   names an `.artifacts/` candidate path and "ignored/local" report paths
   that exist only on the contributor's machine; docs add 58 spaced em
   dashes; `agent_docs/roadmap.md` gains 428 lines, much of it delivery
   history. Tracked docs name evidence by save, day and values (`CLAUDE.md`,
   "Invariants").
4. **Native evidence is on the wrong platform.** Native acceptance here runs
   on the macOS Mini with the developer commands; the PR's native runs were
   Linux. The macOS focus-loss path PR #45 added must still cancel a drag.
5. **New unsafe code.** 22 added lines use `unsafe`: an X11 observer that
   loads libX11 through miniquad's module loader under
   `cfg(target_os = "linux")`, and the fixture command channel's wasm
   imports. Neither machine here runs Linux.
6. **Single-threaded test runs.** The PR's figures come from
   `--test-threads=1`; `scripts/gates.sh` runs the workspace in parallel.
   A suite that passes only serially hides shared state.
7. **The tactical window changes** (+1,012/−215 in `tactical_view.rs`, the
   scene lifecycle). The strip work landed in `e8ae0457` survives the merge
   (its symbols count the same on both heads), but its behaviour needs the
   battle gate and C6 again.
8. **A contributor-built WASM binary.** A binary cannot be reviewed; the
   tracked `web/open-rebellion.wasm` must come from our own build of the
   merged source.

## Review posture

- **Freeze the head.** Review `44704a7e`. If Will pushes, diff the new
  commits and rerun only the affected lanes.
- **Hold `main.rs` work on `main`** until the decision; any change there
  conflicts with a +18,000-line rewrite.
- **Outcomes.** One of: approve as is; request changes with the verified
  findings; repair on the fork branch ourselves and merge (the PR #45 route);
  or split, merging what passes and returning the rest. Tom chooses after
  Phase 6.
- **Nothing outward without Tom.** No review comment, approval, push to the
  fork or merge until Tom says so. Drafts live in the scratchpad.

## Phases

### Phase 0: Setup

- A local worktree of `origin/pr-46` in the scratchpad (the main checkout
  keeps its untracked files untouched), with the interface-parity
  `node_modules` cloned, not symlinked.
- A separate Mini build directory,
  `OPEN_REBELLION_BUILD_DIR=or-native/open-rebellion-pr46`, so the Mini's
  main checkout and its release binary stay untouched. With the red-first
  clone that is the two-clone limit.
- Record baselines on `main` (`e8ae0457`) for comparison: per-crate test
  counts, the provenance baseline, the battle gate's 184 and C6's verdicts.

### Phase 1: Automated baseline on the PR head

- `scripts/gates.sh --keep-going` in the PR build directory, redirected to
  a file; read every PASS/FAIL line (test, fixtures, replay, coverage,
  provenance, wasm, beads, leak greps). The provenance baseline may only
  shrink.
- The workspace tests run in parallel as well as with `--test-threads=1`; a
  difference is a finding.
- `node --test` over `tools/interface-parity`, and
  `verify-production-exclusion.mjs` on a production package built here.
- `cargo check --target x86_64-unknown-linux-gnu -p rebellion-app` on the
  Mini, so the Linux-only code at least compiles (needs the target added
  with `rustup`; ask before changing the Mini's toolchain).
- Scoped `rustfmt --check` and strict Clippy on the touched files, compared
  with `main`'s audited baseline.

### Phase 2: The behaviour behind the deleted tests

A deleted test is not itself a finding, and no test needs a one-for-one
replacement. The review asks only whether the behaviour Appendix A's tests
guarded still holds on the PR head. For each behaviour, one of:

- it holds, shown by a PR test, a gate journey or the Phase 5 native run;
- the PR changes it on purpose, and the original game agrees (Wine for
  strategic windows), cited; or
- it is lost or wrong: a finding.

Where it is unclear, port the old test's assertions to the PR's API in the
scratchpad worktree and run them once, as a probe; the probe is not added
back to the PR.

### Phase 3: Code review lanes

Up to eight reviewer subagents in parallel, one lane each, each reading its
files whole and returning findings with file:line, a failure scenario and a
confidence. Then Codex Sol at extra-high effort reviews the architecture
independently, as `agent_docs/agent-tooling.md` ("Required Review Lanes")
asks for a change of this size and risk.

| Lane | Files | Questions |
|------|-------|-----------|
| A. Windowing kernel | `rebellion-windowing` `manager.rs`, `snapshot.rs`, its three new test files | Kernel invariants, receipt and rollback semantics, reconciliation removal order; the crate stays renderer-independent and standard-library-only |
| B. App frame and input | `main.rs`: frame boundary, input routing, pointer and key receipts, focus | The focus rules of first-pass item 2; key repeat (`key_downs` and `set_frame_key_downs` still reach the tactical window); the dev-command control registry still names every control |
| C. App lifecycle and FFI | `main.rs`: save and restore, scenes, movies; X11 observer; fixture command channel | Saved window order, places, rail slots and hidden state; every `unsafe` block's invariants; nothing fixture-only compiles into production |
| D. Strategic windows | `sector_window.rs`, `system_window.rs`, `defenses_window.rs`, `missions_window.rs`, `fleet_window.rs` | Atomic Sector replacement and rollback; drag capture and cancellation; each window's layout and paint unchanged where the PR claims only lifecycle work |
| E. Finders and modals | Troop, Personnel and Fleet Finders, Encyclopedia, Message Index, Status, Build Selection, mission dialog, Move and Scrap confirmations, object and agent menus, game menu | Modal boundaries, keyboard ownership, empty-list navigation, transient menus |
| F. Tactical | `tactical_view.rs`, `cockpit.rs`, `targeting.rs`, `game_speed.rs` | The Alert and Results scene lifecycle; no change to the strips, combat input or Withdraw |
| G. Gate scripts | the new and changed `tools/interface-parity` scripts and tests | Can each new gate fail? Do the receipt waits hide a real failure behind a timeout? Does any change loosen an existing check? |
| H. Rules and docs | every changed doc, plan and audit file; the new tests' names and citations | Evidence by save, day and values; no history prose; em dashes attached; behaviour-sentence test names; both factions in every new test; every new original-behaviour rule cited (`FUN_`, `port:`, `hyp:`); plan percentages honest |

Every reported finding then gets one adversarial verification pass: a
second agent tries to disprove it against the code, and only confirmed or
plausible findings enter the ledger.

### Phase 4: Test quality

- **Red-first, sampled.** Hand-break the code under about 30 of the new
  tests, chosen for weight (receipt gating, rollback, focus, save and
  restore, modal boundaries, both factions), in a Mini clone; each must go
  red. The red-first script from the strip epic is the model.
- **Mutants**, scoped by `--re` to the PR's new functions in
  `rebellion-windowing/src/manager.rs`, `strategic_windowing.rs`,
  `sector_window.rs` and the save and restore functions in `main.rs`.
  Each survivor is a missing test or an equivalent mutant, explained.

### Phase 5: Live acceptance

- **Browser**, through the interface-parity gates only, muted, on the
  Mini's fixture site built from the PR head: the windowing contract
  journeys the PR adds (identity and history, input scenes, modal
  boundaries), the per-window gates it changes (Fleet window, Fleet
  Finder, Sector quadrants, Status window, Encyclopedia, mission dialog,
  Build Selection), and `run.mjs` across the whole catalog. Read
  `summary.json` and `result.json`, not the console. Chrome only unless Tom
  asks otherwise.
- **Native, macOS Mini, both factions**, with `launch-native.sh --live` and
  `dev-run.sh` batches: open, focus, minimize, restore and close each window
  family; Escape at each level; a save and load with windows shown, hidden
  and minimized; a drag cancelled by focus loss; a movie closing the Status
  window, Encyclopedia and Message Index.
- **Tactical**: the battle gate (184 on `main`) and C6, with the strip cells
  and TAC-05-C023 as on `main`.
- **Original game**, Wine on the Mini, for any behaviour the PR changes and
  Phase 2 could not settle from the trace.
- Every run silent; `pmset -g assertions | grep -B1 audio-out` clean on
  both machines; close the browser and server after each run.

### Phase 6: Findings and recommendation

- One findings ledger in the scratchpad: severity, file:line, failure
  scenario, verification verdict, lane.
- A recommendation to Tom with the four outcomes and their cost.
- After Tom decides: the review comment for Will (drafted for Tom's
  approval), the repair commits if we repair, our own WASM build replacing
  the contributor's, and beads for findings left open, under the epic this
  plan becomes.
- Merge is squash, as for PR #45 and the earlier windowing PRs, and only on
  Tom's word.

## Acceptance criteria

- [ ] `gates.sh` passes on the PR head, or each failure is a ledger finding.
- [ ] The workspace tests pass in parallel and serially, or the difference is a finding.
- [ ] The Linux-only code compiles for `x86_64-unknown-linux-gnu`, or Tom waives the check.
- [ ] Each behaviour the 35 deleted tests guarded holds on the PR head, is changed with the original's agreement, or is a finding.
- [ ] Lanes A–H and the Codex Sol review have reported, and every finding has a verification verdict.
- [ ] About 30 sampled new tests go red when their code is broken.
- [ ] Mutant survivors in the scoped functions are closed or explained.
- [ ] The browser gates pass on the PR head (summary and result files read).
- [ ] The native journeys pass on the macOS Mini for the Empire and the Alliance.
- [ ] The battle gate passes all its cases, and C6's strip cells read as on `main`.
- [ ] Tom has the findings ledger and a recommendation.

## Constraints

- Cargo only through `scripts/mini-cargo.sh`, in the PR's own build
  directory; red-first only in a Mini clone; at most two Mini clones.
- Browser only through the interface-parity gates; native only through
  `launch-native.sh`; every run silent on both machines.
- Never record a check as Tom's; never treat the PR's own evidence as ours.
  It is a lead, rerun here before it counts.
- No push, comment, approval or merge without Tom's word; no force-push.

## Appendix A: tests the PR deletes

| File | Tests |
|------|-------|
| `window_focus.rs` (deleted) | `a_load_focuses_the_top_restored_window_that_is_not_a_sector`, `with_the_menu_never_opened_escape_closes_the_restored_top_window_then_reaches_the_sector`, `a_focused_fleet_windows_escape_closes_it_through_the_manager`, `a_click_in_a_system_window_keeps_its_escape_closing_it`, `the_first_gid_press_gives_the_menu_the_focus_whose_escape_shows_the_top_window`, `a_later_gid_press_takes_the_focus_from_the_window_it_hides`, `a_new_window_leaves_the_focus_where_it_was`, `the_first_menus_escape_with_no_window_activated_only_focuses_the_view`, `a_focus_step_raises_shows_or_closes_the_window_it_names`, `a_click_on_a_window_gives_it_the_focus`, `a_frame_with_no_window_raised_leaves_the_focus_alone`, `a_closed_window_hands_the_focus_to_the_top_one_left`, `a_closure_hands_the_focus_on_even_in_a_frame_where_a_window_opens` |
| `main.rs` | `a_save_keeps_every_window_from_the_bottom_of_the_z_order_up`, `a_window_the_gid_menu_hides_saves_hidden_but_a_sector_stays_shown`, `a_load_rebuilds_the_saved_order_and_focuses_the_top_window`, `a_fleet_window_saved_on_top_comes_back_in_front_with_the_keyboard`, `a_save_from_game_options_keeps_a_shown_fleet_window_shown_and_a_minimized_one_hidden`, `a_load_keeps_fleet_windows_at_their_saved_places_and_rail_slots` |
| `system_window.rs` | `after_an_escape_the_window_behind_ranks_where_it_took_focus_for_either_side`, `a_saved_minimized_window_comes_back_on_the_rail_in_order`, `a_loads_hidden_window_takes_the_next_place_in_the_z_order`, `a_minimized_defenses_window_keeps_its_own_rail_slot_and_restores_from_it`, `a_minimized_missions_window_keeps_its_own_rail_slot_and_restores_from_it`, `a_minimized_fleet_window_keeps_one_rail_slot_beside_its_systems_window` |
| `defenses_window.rs`, `missions_window.rs` | `after_an_escape_the_window_behind_ranks_where_it_took_focus_for_either_side` (one each) |
| `strategic_windowing.rs` | `fleet_commit_publishes_content_and_rail_only_after_manager_transition`, `windowing_legacy_bridge_blocks_raw_paths_under_migrated_modal`, `windowing_no_family_production_frame_preserves_legacy_root`, `windowing_legacy_blocker_restricts_without_shadow_identity`, `retained_windows_hold_input_under_application_locks_and_legacy_modals`, `windowing_legacy_restriction_latch_remembers_ephemeral_owner` |
| `windowing_input.rs` | `windowing_legacy_bridge_blocks_raw_paths_under_migrated_modal` |
| `movies.rs` | `a_waiting_movie_closes_the_status_window_the_encyclopedia_and_the_index` |

The legacy-bridge tests guard a bridge the PR removes on purpose; for
them the question is whether the bridge's last users are gone, not whether
the behaviour survives.

## Sources

- PR #46: `gh pr view 46`; head `44704a7e`; diff against `e8ae0457`.
- PR #45 precedent: squash `9533b516` and its repair commit on the fork.
- `CLAUDE.md` ("Testing", "Invariants", "Git and Dependencies");
  `agent_docs/agent-tooling.md` ("Required Review Lanes", "Browser
  Acceptance", "Test Quality Gates"); `agent_docs/dev-commands.md`;
  `tools/interface-parity/README.md`.
- `ghidra/notes/strategic-windowing-toolkit.md` and its runtime evidence
  note; the windowing plans `2026-10-10-feat-strategic-window-modality-and-focus.md`
  and `2026-10-10-strategic-windowing-toolkit-implementation.md`.
