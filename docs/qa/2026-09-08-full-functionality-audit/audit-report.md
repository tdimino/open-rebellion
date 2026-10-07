---
title: "Open Rebellion Full Functionality Audit"
description: "Repository status, verified evidence, release blockers, and feature-by-feature acceptance plan"
category: qa
created: 2026-09-08
updated: 2026-10-06
commit: fc25634be905839dfa6fb477d5fff0faa49d8ae9
tags: [qa, audit, functionality, parity, bitmap, wasm, astra, fable]
---

# Open Rebellion Full Functionality Audit

## Executive conclusion

Open Rebellion is feature-rich and has substantial unit-test coverage, but it
is not yet demonstrably 100% functional. The repository is at post-implementation
integration and acceptance closeout. Several user-facing paths are incomplete,
the long-running campaign still fails combat-distribution and full-loop gates,
and the existing CI pipeline does not establish browser or visual correctness.
The [original-interface sister audit](../2026-09-10-interface-parity-audit/)
additionally confirms that the in-campaign strategy, object, report, tactical,
and multiplayer surfaces do not yet match the original game.

This audit was performed against:

- Commit: `fc25634be905839dfa6fb477d5fff0faa49d8ae9`
- Branch: `main`, synchronized with `origin/main`
- Audit date: 2026-09-08
- Platform: macOS, Apple Silicon
- Independent reviews:
  - GPT-6-Astra through `codex-orchestrator`, medium effort, read-only
    researcher profile
  - Claude Fable 5.1 through the verified Fable CLI lane, repository-read-only
    audit

No tracked source files were modified during the audit. Four pre-existing
evaluation files were untracked: `EVALUATE.md`, `edd.config.json`, `program.md`,
and `run-eval.sh`.

## Where work left off

PR #11 [corrects the cockpit command routing](../2026-09-10-interface-parity-audit/evidence/2026-09-14-cockpit-routing-correction.md): `0x131` and F7 identify Encyclopedia, `0x132` opens and closes GID, and the side-globe `0x133` plus F1 identify Game Options for both factions. Encyclopedia deliberately fails closed. The [2026-09-26 Game Options review checkpoint](../2026-09-10-interface-parity-audit/evidence/2026-09-26-game-options-review.md) replaces the options placeholder with the shared bitmap surface; strict original-evidence acceptance remains open.

The current continuation has verified the authentic bitmap main menu,
save/load/delete, deterministic browser packaging, save continuation, exact
native/WASM replay, fleet redispatch protection, and authoritative fleet
position with compatible-arrival consolidation. Ordinary player fleet dispatch
passes both-faction browser acceptance through the shared departure helper.
F-007D resolves all opposing fleets at system scope and closes the permanent
five-tick combat backlog. The first F-007E checkpoint also closes production
ownership, friendly cycling, transit, repair-start, and troop-class defects.
The second F-007E checkpoint adds save-v13 troop transport, landing, continuing
ground combat, occupation, and provisional character capture. Player troop
selection and dispatch passed in the following checkpoint (reopened 2026-10-02:
its fleet-panel picker lost its caller in 23d15da and was deleted in F-007C
phase 6a; restored 2026-10-04 through the original order, a regiment's Move
released on the Fleet window). The campaign-history
review then reopened the victory and capture model. The next checkpoint now
distinguishes Alliance control of Coruscant from Imperial destruction and
occupation of the mobile Alliance HQ, preserves Standard leader conjunctions,
makes Death Star loss nonterminal, removes the uncited grace period, and covers
every current bombardment entry path. M1 continues with capture/evasion,
faction liveness, the wider campaign loop, target acquisition, and five-seed
cross-runtime proof.

The current campaign screenshots also exposed a separate visual truth. The
original shuttle main menu remains a passing surface, but the strategy screen
still uses synthetic map primitives, incomplete system-window contents,
approximate rail thumbnails, and incomplete advisor-shell integration. The
first original sector and system shells now replace the sidebar. The sister
audit inventories 43 surface families,
retains 370 original references, and makes UIP-T01 Strategic Cockpit Truth the
highest-priority presentation tranche. Earlier scoped browser checks retain their
behavioral and loading evidence; they are not evidence of original interface
identity.

The repository's records describe different scopes and are not a unified
acceptance record:

| Record | Actual status |
|--------|---------------|
| `README.md` | Explicitly distinguishes implementation estimates from final release acceptance and links to this audit. |
| `archive/progress.archived-2026-04-07-native-video.json` | Archived record of the April 7 native-video task; it reports lint as false and is superseded by this audit. |
| April 12 Tammuz plan | Described as completed elsewhere, but its functional and quality acceptance checkboxes remain open. |
| April 6 test-infrastructure plan | Baseline investigation is complete; implementation milestones remain open. |
| March 24 QA inventory | Historical v0.15 browser observations, not acceptance evidence for the current build. |

## Reproduced audit baseline (2026-09-08)

| Gate | Result | Evidence |
|------|--------|----------|
| Workspace tests | PASS | 465 passed, 0 failed, 17 ignored. |
| App/playtest automated tests | GAP | Neither binary has direct test coverage. |
| Native BMP decode | PASS | All 2,231 staged BMP files decoded with ImageMagick. |
| HD PNG decode | PASS | All 234 PNG files decoded with ImageMagick. |
| Production HD replacements | PARTIAL | 74 production DLL PNGs: Common 53, Gokres 13, Strategy 7, Tactical 1. |
| Raw WASM release compile | PASS WITH WARNINGS | `wasm32-unknown-unknown` build completed with 30 warnings. |
| Formatting | FAIL | Formatting drift was detected across approximately 83 files. |
| Strict clippy | FAIL | At least 98 errors in `rebellion-core`, with additional workspace findings. |
| Packaged WASM browser boot | NOT RUN | The raw compilation result does not exercise `scripts/build-wasm.sh` or a browser. |
| Visual bitmap correctness | NOT PROVEN | Decode success establishes file integrity, not in-game display or identity. |

The local `cc` command is a tmux/Claude launcher rather than the system C
compiler. Verification commands must use a sanitized `PATH` so Rust links with
Apple clang:

```bash
env PATH=/Users/tomdimino/.cargo/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test --workspace
```

## Campaign evaluation

A fresh 5,000-tick dual-AI campaign was run with seed 42.

| Metric | Observed |
|--------|----------|
| Quality score | 0.2851 |
| Parity result | FAIL: 19 pass, 5 fail, 1 skip |
| Victory | None by tick 5,000 |
| Total events | 2,001,963 |
| Initial fleets | 5 |
| Fleets in transit at end | 980 |
| AI attack orders | 306,012 |
| Empire AI actions | 326,092 |
| Alliance AI actions | 6,332 |
| Space/ground/bombardment events | 655 / 2 / 1 |
| Battles at Yavin | 655 of 658 combat events |

The run also reported missing `TroopClassDef` entries. Parity failures covered
the original AI interval, Death Star construction/fire/shield events, and
research completion events; victory evidence was skipped because no victory
occurred.

This is release-blocking behavior even though the evaluator did not classify
the run as formally degenerate.

F-007A reran the same seed after protecting active fleet orders. Accepted
moves fell to 153,462 against 152,513 arrivals, and attack orders fell to
78,946. The remaining 950-fleet arena and 949 in-transit fleets confirm that
production, arrival merging, and combat backlog are separate open causes
([evidence](evidence/2026-09-09-fleet-redispatch.md)).

F-007B then made transit orders authoritative, rebuilt orbit indexes before
production, and consolidated compatible arrivals. Across five 5,000-tick
seeds, final fleet arenas are no more than 1.4 times initial size and accepted
moves are no more than 1.002 times arrivals. Combat remains concentrated in at
most two systems and no seed reaches victory
([evidence](evidence/2026-09-09-fleet-position-consolidation.md)).

F-007D resolves every hostile task force at a system in one bounded engagement,
persists fighter losses, corrects fighter launch and shield handling, and stops
unchanged stalemates until system composition changes. Seed 42 falls from 603
space events, including 602 recurring Xyquine draws, to two decisive
engagements with no backlog
([evidence](evidence/2026-09-10-system-combat-resolution.md)).

The first F-007E checkpoint then gates AI production by system ownership,
removes friendly cycling and transit fan-in, retains HQ defense and surface
garrisons, resolves compound troop-class IDs, and persists repair episodes in
save v12. Five current runs finish with 0% transit, exactly one move per
arrival, at most 1.2× their initial fleets, and 0–5 repair starts
([evidence](evidence/2026-09-10-ai-campaign-logistics.md)).

The second F-007E checkpoint carries regiments within living ship capacity,
preserves cargo through transit and consolidation, lands after orbital control,
continues unresolved surface battles, and occupies systems. Its deterministic
character capture and occupation-based Imperial HQ result are verified current
behavior but not parity-correct. Save v13, all 567 workspace tests, the exact
native/WASM replay, five 5,000-tick transport runs, and Astra medium two-faction
bitmap/browser acceptance pass for the scoped transport path. Player troop
dispatch passed in the next checkpoint (reopened 2026-10-02 with F-007C;
restored 2026-10-04 through the Fleet window). The
following victory checkpoint
corrected the Imperial HQ, Death Star, Standard-conjunction, and timing rules;
capture/evasion, faction liveness, battle diagnostics, and five-seed
cross-runtime proof remain open
([transport evidence](evidence/2026-09-10-troop-transport-occupation.md);
[victory evidence](evidence/2026-09-10-victory-contract.md)).

## Confirmed findings

### F-001: Save, Load, and Delete UI wiring

- Severity: P0
- Status: remediated; browser UI gate passes, native GUI smoke remains
- Evidence: the app now handles `PanelAction::SaveGame`, `LoadGame`, and
  `DeleteSave` against the full live campaign before the general action
  dispatcher. Main-menu Load Game opens the slot picker.
- Verified tranche: Astra-medium r1 passed 27/28 checks and found that an empty
  slot could be selected in load mode. The panel now disables empty load rows
  and verifies the selected slot is occupied before enabling Load. All 486
  workspace tests pass, including three focused panel regressions.
  Astra-medium r2 then passed 33/33 real-UI save/delete/reload assertions with
  intact bitmaps, complete v10 key removal, an unselectable empty slot, and zero
  strict browser/network/asset errors. See
  `evidence/2026-09-09-save-delete.md`.
- Acceptance: perform UI-driven save, restart, load, compare full campaign
  state, delete the slot, and verify disk/browser storage changes. Browser is
  verified; retain a native GUI restart smoke test in P31.

### F-002: Native faithful-HD acceptance remains open

- Severity: P1
- Status: profile and manifest foundation implemented; visual acceptance pending
- Evidence: original-parity rendering is now the default, uses nearest sampling,
  and ignores `data/hd`. Native faithful HD is opt-in and accepts only approved
  source-bound manifest records. Verified-byte decode, mutation fallback, and
  original-parity browser regression gates pass in
  `evidence/2026-09-10-faithful-hd-foundation.md`. Visual acceptance using a
  reviewed asset from the owned corpus remains open.
- Acceptance: approve a provenance-complete HD asset, render it in the explicit
  faithful-HD profile, revoke it, and prove original-BMP fallback on the same
  resource without changing original-parity output.

### F-003: Browser faithful-HD packs remain absent

- Severity: P1
- Status: confirmed
- Evidence: WASM now fails closed to original parity. The runtime pack does not
  yet carry approved HD manifest records or optional surface chunks.
- Acceptance: load a reviewed optional HD surface pack, verify its manifest and
  source identity, render its approved asset, and atomically fall back to the
  original family when approval or payload validation fails.

### F-004: WASM omits troop data prefetch

- Severity: P1
- Status: remediated
- Evidence: `TROOPSD.DAT` is included in the deterministic runtime pack. Native
  and browser integration resolve the original troop-class records without
  missing-class fallback diagnostics. This does not establish combat-formula
  parity, which remains P25.
- Acceptance: native and browser worlds report matching troop-class counts and
  no fallback diagnostics.

### F-005: Victory modal is unreachable in normal play

- Severity: P1
- Status: confirmed
- Evidence: normal victory handling transitions directly to a cutscene or main
  menu; the compiler reports `VictoryModal` is never constructed.
- Acceptance: every win and loss route reaches the result UI, freezes play,
  follows the accepted cutscene policy, and returns cleanly to the menu.

### F-006: Interactive combat does not establish core combat parity

- Severity: P1
- Status: confirmed design divergence
- Evidence: tactical space and interactive ground combat use simplified
  calculations separate from core auto-resolution.
- Acceptance: deterministic fixtures prove that all entry paths use the
  accepted mechanics and apply identical permanent losses and conquest state.

### F-007: Long-running AI simulation exhibits runaway fleet behavior

- Severity: P0
- Status: partially remediated
- Evidence: seed 42 grew from 5 initial fleets to 980 fleets in transit,
  generated 306,012 attack orders, and produced no victory by tick 5,000.
  F-007A now rejects replacement orders, preserves elapsed travel, excludes
  transit and same-pass reservations from AI dispatch, and emits AI telemetry,
  movement messages, or departure audio only for accepted moves. Explicit
  player retries retain accurate in-transit feedback. Its 5,000-tick rerun
  produced 78,946 attack orders and a 1.006 accepted-move/arrival ratio.
  F-007B removes transit fleets from orbit indexes, reconciles stale saves,
  attaches production only to orbiting garrisons, preserves significant task
  forces, and merges anonymous same-faction arrivals deterministically
  (retired 2026-10-05: no original rule merges fleets the player did not
  join, `ghidra/notes/fleet-join-split.md`). The
  then-current five-seed runs passed the fleet-arena and move/arrival bounds.
  F-007C (reopened 2026-10-01: its only entry, the egui system context
  menu's Move Fleet Here, lost its caller in 23d15da, so the player cannot
  move a fleet; the original entry is the Move order; restored 2026-10-02
  through the Fleet pop-up menu's Move and Confirmed Move (0x201/0x202,
  FUN_0044f060's confirmation window) and the system window drag (0x214),
  with a 12-case two-sided browser gate and 8 of 8 hand-applied main-loop
  mutants caught; the blockade bit follows FUN_0050b8e0, under which the
  confirmation's blockade branch meets only a stale bit, as in the original;
  regiments load through the original Fleet window (type 4, FUN_004a2630),
  opened from the sector window's fleet icon, with an 8-case two-sided gate;
  the sector window's other quadrant icons open the System window, the
  System Defenses window (type 10, FUN_004a7790) and the Missions window
  (type 11, FUN_0049f130), with a 10-case two-sided gate; a regiment
  dragged out of the Fleet window's Troops tab unloads or boards at once in
  its own system and otherwise travels on its own at GNPRTB 1 (FUN_00556390,
  FUN_00556430; another side's populated destination is refused,
  FUN_0053d430), with a 16-case two-sided gate; fleets join and split on
  the Fleet window, a ship's or fleet's Move onto a fleet joining it
  (FUN_004ffc90, FUN_004feca0) and a ship's Create Fleet (0x270) making it a
  fleet of its own, with a 34-case two-sided gate, and F-007B's merge of
  compatible arrivals is retired as unsourced; the Fleet Finder (window
  type 0x15, FUN_00461960) opens from the cockpit or F3 and opens a chosen
  fleet's or ship's Sector and Fleet windows (FUN_00429440), with an 18-case
  two-sided gate. The Finder stays above a modeless Fleet window, and an
  empty-list double-click over a covered sector planet leaves the selected
  system unchanged. Remaining transport-limited native journeys stay open)
  wired
  ordinary player dispatch through the same validated departure helper, fixes
  map/context-menu click ordering and stale targets, and reports authoritative
  destinations and countdowns in the fleet panel. Astra medium passed both
  factions with four requests each, intact bitmaps, and zero runtime or asset
  errors. F-007D now resolves every hostile fleet in one bounded system
  engagement, writes fighter losses back, corrects fighter launch/shield
  handling, and suppresses unchanged five-tick stalemates. The 541-test
  workspace, exact native/WASM replay, and Astra bitmap regression gates pass.
  The first F-007E checkpoint now passes fleet-arena, transit, move/arrival,
  repair-start, and troop-class gates. All 551 workspace tests and the updated
  exact native/WASM replay pass. Astra medium also passed the two-faction bitmap
  flow, save-v12 reload, replay 9/9, exact music-control boundary, muted test
  output, and clean network/runtime gates. The second F-007E checkpoint adds the
  authoritative troop transport and occupation path, exact tactical survivor
  persistence, provisional enemy-character capture, and political occupation.
  Five 5,000-tick runs now emit landings, ground battles, control changes, and 5–6
  captures. All 567 workspace tests, save-v13 migration, exact replay, and Astra
  bitmap/browser gates pass for that scoped implementation. Source review then
  reopened Imperial HQ destruction, Death Star terminal outcomes,
  capture/evasion, and the 200-tick grace period. The next F-007E checkpoint
  (reopened 2026-10-02: no player path loads regiments onto a fleet since the
  chooser's picker was deleted; restored 2026-10-04 through the original
  regiment-to-fleet order, a regiment's Move released on the Fleet window)
  adds player regiment selection to the bitmap fleet chooser, enforces 0/0,
  2/2, and 3/3 live capacity in the browser, and carries selected cargo through
  dispatch and automatic landing. All 568 workspace tests and the packaged
  WASM pass; Astra observed 12/12 HTTP 200 responses and zero runtime or asset
  errors. The victory-contract checkpoint adds separate `HqCaptured`,
  `HqDestroyed`, and `DeathStarVictory` outcomes; preserves both Standard
  leader conjunctions; makes Death Star loss nonterminal; removes the
  unsupported 200-tick delay; and applies HQ destruction before contested,
  unopposed, manual, auto-resolved, or tactical occupation paths. Its 574-test
  workspace, two original-data fixtures, nine-checkpoint native/WASM replay,
  final package, and Astra source/browser acceptance pass. Terminal victory UI
  was not visually exercised. Alliance attacks and battle spread remain below M1 bounds, and the AI
  does not yet acquire every randomized Standard victory target. See
  `evidence/2026-09-10-victory-contract.md`.
- Historical baseline: the cited
  [Rebellion/Supremacy campaign-history reference](../../reference/campaign-history/)
  now defines the official campaign contract, human campaign chronology, and
  observed computer-opponent profile. The 50–400 battle range and geographic
  spread remain provisional engineering guards because no located source
  provides original AI-versus-AI telemetry or those numeric constants.
- Acceptance remains open: fleet, transit, and player troop-dispatch bounds
  pass. Faction liveness, target diversity, battle diagnostics, complete
  capture/evasion, the wider campaign loop, and five-seed cross-runtime proof
  do not yet pass. The source-backed victory rules pass dedicated fixtures;
  P30 remains open for end-to-end result-screen and campaign acceptance.

### P58-B11: Tactical trench-run launch and strategic persistence

- Status: partial A1 coverage; strict acceptance remains open
- Evidence: the production Attack Death Star control now proves that committed
  source order `6` enters the live trench-run lifecycle. Battle Results proves
  exact three-to-two capital and fighter persistence for both strategic fleets
  before destination routing.
- Matrix: 98 of 106 cells have deterministic A1 scenarios through 80 browser
  journeys and 18 snapshots. Eight scenarios and all 106 lossless A0 captures
  remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-trench-persistence.md`.
  Qualitative behavior is adjudicated against the historical reference.

### P58-B12: Tactical Game Options and empty space

- Status: partial A1 coverage; strict acceptance remains open
- Evidence: the original COMMON Game Options surface now serves the shuttle,
  command center, and tactical Battle Options routes. Tactical display controls
  are disabled during a battle, and the empty-space fixture retains its
  starfield while omitting the planet.
- At this checkpoint, 100 of 106 cells had deterministic A1 scenarios through
  81 browser journeys and 19 snapshots. P58-B15 later completed the A1 mapping
  at 106 of 106; all 106 lossless A0 captures remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-game-options-empty-space.md`.

### P58-B13: Tactical withdrawal confirmation

- Status: partial A1 coverage; strict acceptance remains open
- Evidence: the original TACTICAL 1310 panel, TEXTTACT title and prompt, and
  controls 1113 through 1116 now mediate withdrawal. Cancel closes without
  retreat; confirm starts withdrawal; the disabled repeat remains inert.
- Matrix: 101 of 106 cells have deterministic A1 scenarios through 82 browser
  journeys and 19 snapshots. Five scenarios and all 106 lossless A0 captures
  remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-withdraw-confirmation.md`.

### P58-B14: Tactical detail and Escort

- Status: partial A1 coverage; strict acceptance remains open
- Evidence: TACTICAL 3368 supplies a complete destroyed presentation, panel
  1302 displays compact GOKRES assignments, and direct friendly right-click
  assigns source order-code 1 Escort with retained target, marker, follow,
  opportunity fire, and invalid-target cleanup.
- Matrix: 104 of 106 cells have deterministic A1 scenarios through 85 browser
  journeys and 19 snapshots. Battle Alert entry, tactical audio, and all 106
  lossless A0 captures remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-detail-escort.md`.

### P58-B15: Tactical Battle Alert and audio

- Status: complete deterministic A1 mapping; strict acceptance remains open
- Evidence: the source-built faction Battle Alert now precedes tactical
  command, its four tabs and three command families use recovered resources,
  Take Command enters paused combat, MDATA 307 supplies the battle score, and
  TACTICAL WAVE 13054 is extracted, packaged, and routed. P58-B16 corrects the
  provisional label from ship destruction to torpedo impact and restores all
  eight weapon events and 22 WAVE variants at 13033–13054.
- Matrix: all 106 cells have deterministic A1 scenarios through 87 browser
  journeys and 19 snapshots. A0 coverage and strict acceptance remain 0/106;
  the strict gate was not run. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-24-tactical-battle-alert-audio.md`.

### P58-B16: Tactical weapon audio bank

- Status: complete within the deterministic A1 boundary; strict acceptance
  remains open.
- Evidence: production capital and fighter combat emit the recovered laser,
  turbolaser, ion, and torpedo fire/impact events. Exact TACTICAL WAVE
  13033–13054 resources are extracted and routed through native and browser
  backends.
- Verification: workspace tests, pack tests, harness checks, and the focused
  four-case muted browser journey pass. Exact original RNG sequencing, audible
  native comparison, and strict A0 acceptance remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-24-tactical-weapon-audio.md`.

### P58-B17: Tactical command voice bank

- Status: complete within the deterministic A1 boundary; strict acceptance
  remains open.
- Evidence: the original faction battle-ready calls and all task-force/RGBY
  maneuver, attack, formation, and mission acknowledgements now select exact
  VOICEFXA/VOICEFXE events and WAVEs. Unsupported strategic aliases to these
  tactical recordings were removed.
- Verification: focused Rust tests, exact 90-file owned-DLL extraction, the
  packaged WASM build, four muted browser cases, and independent visual review
  pass. Remaining tactical voice families, audible native comparison, mixing,
  interruption, and strict A0 acceptance remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-24-tactical-command-voice.md`.

### P58-B18: Complete tactical voice-bank transport

- Status: complete transport and selected production dispatch within the
  deterministic A1 boundary; strict acceptance remains open.
- Evidence: the executable's continuous event table now maps and transports
  all 285 VOICEFXA/VOICEFXE resources: 153 Alliance and 132 Imperial. Tactical
  withdrawal, battle result, selected Death Star, and RGBY trench-run
  transitions emit their source event in addition to the command paths from
  P58-B17.
- Verification: the complete workspace, exact owned-DLL extraction and hash
  cross-check, mapping validator, packaged WASM, four muted browser cases, and
  independent visual review pass. Remaining production completion,
  destruction, recovery, warning, and ordered trench chatter callers, audible
  native comparison, mixing, interruption, and strict A0 acceptance remain
  open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-25-tactical-complete-voice-bank.md`.

### P58-B19: Mixed-task-force target rejection

- Status: complete within the deterministic A1 boundary; strict acceptance
  remains open.
- Evidence: `FUN_005a24d0` requires one task-force ordinal before a hostile
  focus target can be assigned. A mixed capital selection now preserves its
  manual, active, and Escort targets and queues exact Alliance event `0x84` /
  WAVE `14101` or Imperial event `0x102` / WAVE `15105`.
- Verification: behavior-first and focused Rust tests, the scoped mutation
  gate, packaged production and fixture WASM, four fresh muted browser cases,
  and independent visual review pass. Audible native comparison and strict A0
  acceptance remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-25-tactical-mixed-task-force-target.md`.

### P58-B20: Tactical recovery and withdrawal feedback

- Status: complete within the deterministic A1 boundary; strict acceptance
  remains open.
- Evidence: `FUN_005b8630` now queues the exact RGBG acknowledgement after the
  last live player fighter-group member recovers. Recover queues one exact
  RGBG capacity warning when no friendly carrier slot remains. Confirmed
  withdrawal follows `FUN_005a0240` and `FUN_005b1b70` to queue the exact
  faction warning when a hyperdrive-disabled capital must remain behind.
- Verification: the complete workspace, focused renderer and fixture tests,
  packaged production and fixture WASM, twelve fresh muted browser cases, and
  independent visual review pass. Audible native comparison and strict A0
  acceptance remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-26-tactical-recovery-withdrawal-feedback.md`.

### P58-B21: Shared tactical post-battle orchestration

- Status: complete within the deterministic A1 boundary; strict acceptance
  remains open.
- Evidence: played and automatic space-battle outcomes enter one production
  route for bombardment, troop landing, contested ground continuation,
  unopposed occupation, and Battle Results destination routing.
- Verification: seven focused tactical-flow tests, 794 complete-workspace
  tests, packaged production and fixture WASM, four fresh muted browser cases,
  and independent review of all 40 retained PNGs pass. Exact sequencing,
  native playback, remaining special-state and audio paths, and strict A0
  acceptance remain open. See
  `../2026-09-10-interface-parity-audit/evidence/2026-09-27-tactical-post-battle-orchestration.md`.

### P58-B22: Bounded tactical source completion

- Status: practical launcher implementation complete within the deterministic
  A1 boundary; strict original-interface acceptance remains open at 0/106.
- Evidence: one source-compatible tactical random stream, persisted shield and
  weapon power nibbles, exact completion and destruction voice families, and
  the source-derived 120-second trench-run producer now share the production
  battle path. The trench run includes ordered chatter, maneuver damage,
  commander-rating resolution, casualty rules, and MDATA 201/202 routing.
- Verification: see the retained
  [P58-B22 evidence](../2026-09-10-interface-parity-audit/evidence/2026-09-28-tactical-source-completion.md).
  The workspace and WASM/package gates pass, as do all 144 fresh-process muted
  browser cases. Independent review of 88 full-resolution captures found no
  P0/P1 visual blocker.
  Owned lossless A0 comparison, exact native beam pixels, audible native
  playback and mixing, whole-process random-state continuity, strategic
  commander binding, and rare warning/ejection callers remain open.

### F-008: Browser media and mods are incomplete

- Severity: P1 if browser parity is claimed
- Status: confirmed platform gap
- Evidence: original advisor idle frames now render in the browser, but
  briefing frames, EData images, cutscene playback, mods, advisor action/voice,
  and parts of audio integration still use stubs, empty collections, inferred
  behavior, or immediate completion.
- Acceptance: implement and demonstrate each feature, or explicitly exclude it
  from the supported browser contract and qualify all completion claims.

### F-009: Release quality gates are not enforced

- Severity: P1
- Status: confirmed
- Evidence: the former CI ran native check/tests and raw WASM compilation only.
  Its GitHub Actions definitions were intentionally removed from tracking on
  2026-09-08, so all gates are currently manual. Formatting, strict clippy,
  packaging, browser execution, screenshots, parity, coverage, and asset
  integrity are not enforced by a hosted pipeline.
- Acceptance: the release pipeline runs the same versioned commands and fixtures
  used by local acceptance, retains artifacts, and blocks regressions.

### F-010: Bitmap files decode, but display correctness is unproven

- Severity: P1
- Status: open acceptance gap
- Evidence: all staged files decode, but no current resource ledger connects
  every consumer to a runtime cache hit and screenshot. On 2026-09-08, an
  Astra-medium browser check proved that the proposed Strategy 11016–11027
  cockpit mapping loads successfully but displays character portraits rather
  than the intended control art. Common 11001–11275 are grouped cockpit
  animations, not sequential logical-button triplets. The unverified mapping
  was therefore rejected; controls retain explicit labels while the authentic
  command-to-animation table remains open.
- Evidence bundle: `evidence/2026-09-08-cockpit-pr-audit.md` retains the
  resource adjudication, package hash, Astra R7 results, and screenshots.
- Verified tranche F-010B: commit `4589d2e` maps all 38 fighter and capital-ship
  records to their authentic GOKRES miniatures. Astra-medium R9 verified seven
  representative resources across both factions, zero matte-blue pixels,
  correct aspect ratio, successful requests and manifest identity, and working
  expand/collapse/navigation interactions. The failing R8 baseline, R9
  screenshots, hashes, pixel scans, and test log are retained in
  `evidence/2026-09-08-fleet-miniatures.md`.
- Acceptance: complete the bitmap proof protocol below for every image-bearing
  surface on every claimed platform.

### F-011: Deterministic replay is not established

- Severity: P0 for multiplayer; P1 for reproducible single-player acceptance
- Status: partially remediated; F-011A and F-011B1 through F-011B4 pass
- Evidence: replay format v1 now records and executes command streams with
  per-command state checkpoints. Exact-artifact native/WASM execution now
  matches for the seed-42 fixture, but headless and interactive paths still
  consume randomness differently, interactive seeds are wall-clock-derived,
  and automatic and tactical execution paths are not yet proven equivalent.
- Verified tranche F-011A: save format v9 persists a versioned canonical
  logical-state fingerprint, rejects mismatches, preserves native v8 saves as
  explicitly unverified, and stores lossless browser metadata. The seeded
  two-run probe and 16 save tests pass. Astra-medium r3 passed all 34
  save/reload/load, metadata, bitmap, network, and error assertions with the
  matching fingerprint `v1:e2ad73730e68434a`; evidence is retained in
  `evidence/2026-09-08-state-fingerprints.md`.
- Verified tranche F-011B1: save format v10 persists the simulation RNG,
  optional second AI, repair state, automatic-combat cooldowns, and active
  configuration. Human-readable fingerprints now canonicalize populated
  typed-key maps without changing historical bincode layout. All 483 workspace
  tests pass, including an actual 718-byte v9 artifact and eight-value RNG
  continuation proof. Astra-medium r2 passed 40/40 browser storage, reload,
  load, continuation, bitmap, network, and error assertions with fingerprint
  `v1:6bb217229d3c4c60`; evidence is retained in
  `evidence/2026-09-09-state-continuation.md`.
- Verified tranche F-011B2: replay format v1 records the seed, RNG and roll
  contract, configuration identity, typed `{tick, sequence, actor}` commands,
  and unambiguous checkpoint positions. The shared native/WASM data-manifest
  function fingerprints all 51 simulation DATs in canonical order; the local
  fixture covers 50,597 bytes with aggregate `5facb1c7ba0e81ad`. Seven focused
  unit tests, the original-data fixture, and the WASM app check pass. Evidence
  is retained in `evidence/2026-09-09-replay-contract.md`.
- Verified tranche F-011B3: the recorder derives command positions from the
  runtime and captures a state fingerprint after every command. The executor
  independently rejects engine, seed, data, configuration, initial-state,
  command-position, and checkpoint mismatches. Stable key ordering now covers
  manufacturing completions, simultaneous arrivals, blockade transitions, and
  equal-count AI reinforcement choices. A nine-command, 25-tick original-data
  campaign matched after save-v11 reload in five fresh native processes.
  F-007B subsequently changed tick-10 onward state as intended; the same
  reviewed stream now ends at `v1:b8a40a56246c1314` and passes the native
  fixture. All 518 workspace tests and the supported WASM compile gate passed
  at the original tranche. Evidence is retained in
  `evidence/2026-09-09-replay-execution.md`.
- Verified tranche F-011B4: native and packaged browser WASM decode the same
  13,482-byte replay artifact and independently reconstruct its 51-DAT seed-42
  campaign. The initial fingerprint, all nine ordered checkpoints, tick-25
  final fingerprint, and original artifact text match exactly. The automated
  gate also proves invalid queries and missing packs fail without partial
  state, while normal startup retains four requests and reaches the semantic
  bitmap menu. Astra medium independently passed the gate with zero browser
  errors. The F-007B revalidation passes all 530 workspace unit tests. Evidence
  is retained in `evidence/2026-09-09-replay-wasm-equivalence.md`.
- Acceptance: repeated native runs and native-versus-WASM runs produce the same
  versioned state fingerprints for the same seed and command stream, including
  after save/load.

### F-012: Interactive and headless simulation loops diverge

- Severity: P0
- Status: confirmed design divergence
- Evidence: the app duplicates simulation sequencing instead of calling the
  headless tick entry point. Ground follow-up, bombardment, battle-memory
  updates, AI availability, Death Star effects, telemetry, and `AdvanceTicks`
  behavior differ between paths.
- Acceptance: one authoritative simulation entry point produces identical
  results from the same seed and commands in app, playtest, native, and WASM.

### F-013: The packaged browser artifact omits runtime data

- Severity: P0 for browser release
- Status: remediated by F-014A
- Evidence: the deterministic release package carries 52 game-data entries,
  2,231 bitmaps, and five audio files in `runtime.orpk`; a clean browser boot
  loads it in four requests and verifies the expected hashes. See
  [F-014A evidence](evidence/2026-09-08-runtime-pack.md).
- Acceptance: a clean unpacked artifact boots offline from its own contents,
  loads the expected data hashes, and passes browser smoke tests.

### F-014: Browser startup, memory, and rendering are not release-scaled

- Severity: P1
- Status: partially remediated; F-014A verified 2026-09-08
- Closed tranche: a deterministic pack now carries 52 game-data entries and
  2,231 bitmaps. The release package boots with four total requests and one
  `runtime.orpk` request, retains lazy bitmap decode, includes hashes for all
  five shipped files, and preserves a loose-file development fallback. Astra
  medium passed both factions, fleet interactions, and ten zoom steps per
  faction with zero loading, bitmap, console, or WebGL errors. See the
  [F-014A evidence](evidence/2026-09-08-runtime-pack.md).
- Remaining evidence: the pack is not compressed; raw bytes and decoded
  textures have no eviction policy; HD assets are absent; high-DPI mode is not
  enabled; the galaxy can run two egui passes per frame; sector hulls are
  recomputed each frame; and Firefox/Safari and memory/frame budgets have not
  passed.
- Acceptance: the browser asset pack, lazy decode, LRU limits, one UI pass, and
  cached geometry meet the budgets below in Chrome, Firefox, and Safari.

### F-015: Browser persistence is not production-safe

- Severity: P1
- Status: confirmed design gap
- Evidence: synchronous base64-encoded bincode in `localStorage` is vulnerable
  to quota limits and main-thread stalls. Save v12 captures the currently known
  deterministic continuation envelope, campaign setup, and active repair
  episodes, but browser
  persistence remains synchronous and quota failures are not yet exercised end
  to end.
- Acceptance: versioned, compressed, asynchronous IndexedDB saves round-trip
  complete state, expose quota/corruption errors, and preserve fingerprints.

### F-016: Original main-menu parity and documented extension pass

- Severity: P1
- Status: remediated; P03 and P04 pass
- Evidence: the former stretched, dimmed bitmap and replacement text buttons
  are gone. The browser now composes COMMON 20001 with the binary-mapped control
  resources, exact logical hit regions, 4:3 scaling, direct faction starts, and
  the original menu audio. Astra-medium R2 exercised the full matrix and
  exposed a URL-plugin ABI diagnostic plus audio surviving Quit; after both
  fixes, R3 passed 11/11 focused assertions with zero console/page/request/
  texture errors and zero active audio loops after Quit. F-016B then persisted
  faction, difficulty, galaxy size, and victory mode through save v11 and wired
  Standard principal-leader capture versus Headquarters Only rules. Workspace
  F-016C adds Credits, the explicit M4 multiplayer destination, gain/mute,
  return-to-menu audio, clean second-campaign reset, the correct 35.34-second
  `MDATA.300` *Return of the Jedi* cue, and four original `COMMON.DLL` button
  effects. Workspace tests passed 500/500; Astra-medium verified the actual
  WebAudio buffers, navigation, reset, and error-free packaged artifact; the
  final focused R4 passed 14/14 and marked the candidate safe to commit. See
  `evidence/2026-09-09-main-menu-cockpit.md` and
  `evidence/2026-09-09-game-setup-propagation.md`, and
  `evidence/2026-09-09-main-menu-completion.md`. F-016D adds one clipped
  semantic navigation landmark for all 14 authentic hotspots without visible
  replacement controls. Workspace tests now pass 502/502; Astra-medium passed
  nine of nine browser gates, including 84/84 focus transitions, six exact
  keyboard activations, all destinations, the campaign return, `MDATA.300`,
  all four original effects, four-request startup, and zero runtime errors. See
  `evidence/2026-09-09-main-menu-semantics.md`. F-016E records binary evidence
  that the original had no standalone mute control, then adds one clearly
  separated Open Rebellion extension. Its 30×22 hard-beveled housing and cyan
  projection use the Jiff Gorda/SWG Project Thorn reference; verified Fable 5.1
  review set the canopy clearance, restrained motion, color, and exact hit-area
  rules. The user accepted the native presentation, all 504 tests pass, and
  Astra-medium passed 10/10 browser gates across four viewports with 24/24
  outside-edge probes inert, music gain zero while muted, SFX gain one, and no
  runtime errors. A narrow follow-up found a one-pixel bevel overflow below
  native size; clipping the full device to the shared hit rectangle corrected
  it, and Astra R2 passed 10/10 at 320×240 and 480×360 with 8/8 outside-edge
  probes inert. See `evidence/2026-09-09-main-menu-music-toggle.md`.
- Reference: [`agent_docs/main-menu-parity.md`](../../../agent_docs/main-menu-parity.md)
  records the binary-confirmed geometry, resources, commands, settings, music,
  responsive transform, and Astra matrix.
- Acceptance: native and browser reproduce the assembled cockpit, every mapped
  hover/click/keyboard behavior and selection, direct faction start, menu music,
  4:3 scaling, and all navigation with no blank aperture or missing resource.
  The original 14-control contract remains intact; the optional music-only
  control is documented as an extension. P03 and P04 are complete.

### F-017: The Death Star can never fire

- Severity: P1
- Status: partially remediated; construction timing, sabotage, and browser fire pass remain
- Evidence: `DeathStarSystem::fire` (`crates/rebellion-core/src/death_star.rs`)
  refuses while `shield_generator_active` is true, and the only other writer
  (`crates/rebellion-app/src/tactical_flow.rs`) sets it to true;
  `DeathStarState::destroy_shield` has no caller. Nothing calls
  `start_construction`, so no Death Star fleet is ever built. The shield gate
  may itself be a wrong model: in the original the shield protects the
  Death Star rather than preventing it from firing. Found by the 2026-09-25
  test-pruning pass.
- Acceptance: construction, completion, planet destruction, and the shield's
  real role follow recovered Ghidra evidence, with tests that fail without
  each rule and a browser pass of the fire command.
- Fix (2026-09-25): the superlaser no longer waits on the shield generator.
  `FUN_005617b0` never references it; the shield only absorbs hull damage in
  battle. The Death Star is CAPSHPSD record 136 (TEXTSTRA 10120), an ordinary
  research-order-0 capital ship built at shipyards. Every Death Star check
  tested DatId family `0x34`, but seeded classes carry the record id, so no
  real Death Star was ever recognized in combat or firing.
  `CapitalShipClass::is_death_star` accepts both; a completed Death Star
  build now marks its fleet so it can fire, and a destroyed one clears it.
  Tests fail without each change.
- Open: `DeathStarState::start_construction` and its uncited 1,825-tick timer
  are now a second construction path beside ordinary manufacturing, so
  Death Star Sabotage delays a timer nothing starts. The fire command still
  needs a browser pass. Battle shield absorption stays limited to family-`0x34` ids:
  nothing destroys the shield generator (`destroy_shield` has no caller), so
  applying it to the seeded Death Star would make it unkillable in
  auto-resolved battles.
- Correction (2026-09-26): `FUN_005617b0` is not a superlaser check, and the
  2026-09-25 fix and the fire() docs were wrong to cite it. It recomputes the
  CharacterMgr SeatOfPower flag: `FUN_005070d0` finds `0x34000280`, which
  MJCHARSD and TEXTSTRA 10368 name Emperor Palpatine, and the flag is set when
  he is alive (`+0xac` bit 0 clear) and active (`+0x50` bit 0) at `0x90000109`
  (Coruscant) under Empire control (`+0x24 & 0xc0 == 0x80`). `FUN_0055f650`
  is decompiled: it stores the flag in `+0x58` bit 0 and notifies
  `CharacterMgrSeatOfPowerNotif` (`FUN_00562450`, event `0x230`). The real
  superlaser path is unrecovered (`DEATHSTAR_FIRE` appears only in the
  tactical constructor `FUN_005a7500`), so every `fire()` precondition lacks
  a source. Family `0x34` is the Empire major characters, not the Death Star;
  see F-025.
- Search (2026-09-26): the binary has no Death Star construction or planet
  destruction notifier. Its only Death Star strings are the tactical
  `DEATHSTAR_FIRE`, `DEATHSTAR_UPDATE`, and `DEATHSTAR_WITHDRAW` and the
  system flag `SystemDeathStarNearbyNotif` (`FUN_00512480`). The destroyed
  reasons are `DestroyedSabotage`, `DestroyedAssassination`,
  `DestroyedAutoscrap`, and `DestroyedOnArrival`, raised through
  `GameObjDestroyedNotif` (`FUN_004fc080`, event `0x302`). A sabotage success
  therefore destroys an object; nothing in the binary delays a build, so the
  port's `add_sabotage_delay` has no source either. The superlaser, the
  1,825-tick timer, and sabotage stay open until the tactical fire path
  (`FUN_005a7500`) and the sabotage mission handler are decompiled.

### F-018: Research never limits which ships can be built

- Severity: P1
- Status: partially remediated; troop and facility trees remain
- Evidence: `ResearchSystem::ship_class_is_available` and
  `fighter_class_is_available` (`crates/rebellion-core/src/research.rs`) had
  no callers, so manufacturing offered every class regardless of research level.
- Fix (2026-09-25): the manufacturing panel and AI production offer only
  capital ships and fighters whose `research_order` is at or below the side's
  Ship level. Fighters use their own FIGHTSD.DAT order. Three tests fail
  without the gate, scoped `cargo mutants` catches all 13 gate mutants, and
  the seed-42 golden changes from tick 10 for this cause.
- Open: troop and facility classes do not load `research_order` yet. The
  `<=` comparison and the level-0 start rest on the `rebellion2` prototype
  and the DAT data.
- Correction (2026-09-26): `FUN_0052e4f0` and `FUN_0052e510` are not empty and
  are not buildability checks. Each returns `FUN_005839e0(this+0x88 / +0x8c,
  key)`, a count of matching list entries. Their only caller, `FUN_005330b0`,
  validates that each object in families `0x2d..0x2e` and `0x28..0x2b` sits in
  its side's list exactly when its `+0x58` is 1. No recovered code gates
  research.
- Acceptance: build lists and manufacturing orders respect the recovered
  research-order gate for both factions.

### F-019: Subdue, guarded dispatch, and initial Force awakening are never called

- Severity: P1
- Status: partially remediated; phases 0-7 of the mission port have landed
  (the decoy rule and the Subdue success check now run, and the dialog opens
  from its original entry and passes its browser gate). A native check of
  that entry, and targets other than systems, remain
- Evidence (before the 2026-09-25 fix): `UprisingSystem::try_subdue`
  (`uprising.rs`), `MissionSystem::dispatch_guarded` and `check_decoy`
  (`missions.rs`), and `JediSystem::apply_initial_awakening` (`jedi.rs`) had
  no production callers.
  Subdue Uprising missions never end a revolt, dispatch does not refuse busy or
  mandatory-mission characters through the guarded path, and no character
  starts Force-aware from `jedi_probability`.
- Acceptance: each path runs in the simulation with recovered rules and a test
  that fails without the call.
- Fix (2026-09-25): player, AI, and integrator dispatch now use
  `dispatch_guarded`, which refuses a character already on a mission or a
  mandatory mission and marks the dispatched character busy, matching the
  original role flags (`RoleOnMissionNotif` `FUN_00536b00`,
  `RoleOnMandatoryMissionNotif` `FUN_00536b80`). Cancelling releases the
  character. The seed-42 golden changes for this cause.
- Corrections: seeding already rolls `jedi_probability` for initial Force
  awareness, so the duplicate `apply_initial_awakening` was removed and the
  seeding roll gained a test. A successful Subdue Uprising mission ended the
  revolt only in the headless integrator; the app's own mission handler
  dropped both the uprising clear and Death Star sabotage delay. Both paths
  now share `apply_mission_state_effects`.
- Review follow-up: the guard also refuses a character who already has an
  active mission, which covers saves written before dispatch set the flag,
  and the player's commander list omits busy characters, with a message if a
  dispatch is still refused.
- Pending: the app's call to `apply_mission_state_effects` runs inside the
  frame loop, which no unit test reaches; a Subdue Uprising success needs a
  browser pass to confirm the uprising clears.
- Recovery (2026-09-26): `FUN_0058b420` assigns the table ids (resource
  `0x642` `GDATA\` plus an `RT_RCDATA` file name): TDECOYTB 10, FDECOYTB 11,
  the ten mission tables `0x14..0x1d`, UPRIS1TB `0x28`, UPRIS2TB `0x29`,
  ESCAPETB `0x2c`. See `ghidra/notes/uprising-incident.md`.
- Correction: UPRIS1TB and UPRIS2TB are not subdue probabilities.
  `FUN_00559ce0` reads both during the uprising incident (system `+0x88`
  bit 16, slot `+0x248` `FUN_00511840`, `FUN_0050d030`), turning one score
  into two outcome codes that `FUN_0050d150` applies as facility or regiment
  losses and character effects (F-026). `try_subdue`'s premise is
  contradicted, so it was removed (see the F-026 follow-up below); the Subdue
  Uprising mission table is SUBDMSTB (`FUN_0055c780`).
- Decoy recovery (2026-09-26, `ghidra/notes/decoy-roll.md`): `FUN_0055e410`
  rolls TDECOYTB, or FDECOYTB when the checked object is in a fleet, on
  `decoy espionage - b - counterpart espionage * GNPRTB[3588] / 100`
  (GNPRTB 3588 is 35), and succeeds on `random(0..=99) < value`. Both
  espionage values are character slot `+0x1e0`, `FUN_004edc00`: the short at
  `+0x7e`, which `FUN_004eecf0` sets to `base(+0x5a) * (100 + +0x8c) / 100`.
  The decoy is a random pool member (`FUN_00588700`); the counterpart is the
  first system-holder object whose `+0x96` matches slot `+0x1bc` of the
  checked object (`FUN_00509330`), and `b` is its slot `+0x1c4`.
  `FUN_00589620`, the only caller of `FUN_00588b90`, then had no known
  reference (resolved below). The port's `check_decoy` fed the defender's
  espionage straight into FDECOYTB, which is the wrong input, and was never
  wired.
  The `is_decoy` mission branch is invented too: it cuts FDECOYTB by a flat
  GNPRTB 3588 percent, and only tests set `is_decoy`.
- Decoy call chain (2026-09-27, `ghidra/notes/decoy-roll.md`): `FUN_00589620`
  is slot `+4` of the functor vtable `0x0066a878`, a pointer Ghidra did not
  turn into a reference. Mission execution `FUN_00547f60` runs phase
  `FUN_0058a020`, which walks every defender at the target system
  (`FUN_00587640`: special forces, regiments, and each fleet's ships and
  special forces) and lets a random decoy character from the mission's pool
  roll TDECOYTB or FDECOYTB against each. The original decoy is a character
  attached to a mission. The port has none, so the invented `is_decoy` roll
  and `check_decoy` are removed with their two tests; the field stays for the
  save layout. Porting mission decoys needs decoy characters on
  `ActiveMission`, a save-format change.
- F-026 follow-up: `try_subdue` and the invented UPRIS1TB start roll are
  removed. A Subdue Uprising success now raises support by `FUN_0055cb10`
  (1..20 on its own side's system, 1..10 when contested) and ends the revolt
  through `FUN_0050c910` once regiments cover the undoubled garrison. Two
  divergences remain: the port's Subdue success check uses diplomacy, while
  `FUN_00569b90` rolls SUBDMSTB on leadership, support, and the Stormtrooper
  count; and `BetrayalSystem` still reads UPRIS1TB as a loyalty table.
- Review fix (2026-09-27): the Subdue gain draw took the next completing
  mission's outcome roll, and the budget of one roll per mission left the
  last mission on the 0.5 fallback. Callers now reserve `ROLLS_PER_MISSION`
  (2) rolls per mission; outcomes read the first half and gains the second.
  A test fails without the change.
- Recovery pass 3 (2026-09-28, `ghidra/notes/decoy-roll.md`): the decoy and
  detection phases run on mission phases 2, 3, 7, and 9. The mode word is
  the code constant `0x4112` (`FUN_005236e0`) for every class except Adrift,
  not a DAT column. Betrayal runs only in phase 9. The MSTB success roll is
  phase 10 (`FUN_00592f50`), once per team member through slot `+0x274`,
  characters and then special forces; decoys and captives never roll. The
  requester supplies decoys as an explicit key list (`FUN_0054bb90`, command
  `0x250`). Open: the sender of that command.
- Recovery pass 4 (2026-09-28, `ghidra/notes/ai-mission-planning.md`,
  `mission-lifecycle.md`): both the player and the AI fill the decoy list.
  - A mission order (`0x240`..`0x242`) holds a team `+0x2c` and decoys
    `+0x58`. `FUN_004f4a00` copies them into command `0x250`.
  - The player's dialog (`FUN_0046c3c0`, cases `0xca`/`0xcb`) moves chosen
    characters between the two lists.
  - The AI planners (`FUN_0042f830`, `FUN_004bced0`) pick decoys for
    Sabotage, Rescue, Incite, Espionage, DS Sabotage, Abduction, and
    Assassination, up to `(record.+0xbc + 2) / 2`. Diplomacy, Recruitment,
    Subdue, Recon, and Research pick none.
  - Phases chain within one tick except phase 4 (transit) and phase 8
    (timer `0x38b`, MISSNSD min + rand(spread)).
  - The port lands per phase and covers the port's 10 agent kinds.
- Phase 1 (2026-09-28, save v17): missions hold team, decoy, and captured
  lists of characters or special forces.
  - Guarded dispatch applies `FUN_0054bb90`: prisoners go to the captured
    list and an empty team is refused.
  - It also applies `FUN_00522b30`: members must be on one side, have no
    current mission, share one location, and appear only once.
  - MISSNSD records load with their timer and flags. SPECFCSD classes load,
    and seeded special forces roll their skills (`FUN_00535e40`).
  - The uprising leadership term averages every member (`FUN_00520cd0`).
  - The resolver still rolls one lead character until phase 4. A 1500-tick
    dual-AI playtest produces identical telemetry before and after.
  - Review fixes (2026-09-28): a killed character is refused and hidden from
    the mission panel. Prisoners leave both lists before the team, decoys,
    and captured are added in `FUN_0054c200` order. MISSNSD records are found
    by id, since Research and Vacation share families across records. Save
    v17 has a round-trip test for the new fields.
- Phase 2 (2026-09-29, save v18): missions step through the recovered
  phases instead of counting down.
  - The stepper `FUN_005227d0` chains phases in one tick except phase 4
    (members travel from the origin to the target, `FUN_00556430`, and stay
    there) and phase 8 (timer `0x38b`, MISSNSD min + rand(spread)).
  - Phase 2 sets Han Solo's speed, GNPRTB 3083, when he is a free member and
    no special force is (`FUN_00548370`). Phase 10 resolves and repeats to
    phase 8 when the record repeats; phase `0xb` ends with code 1 when no
    rule fired.
  - The validator `FUN_00522480` reads the MISSNSD target columns: lost
    container 7, target side 8, lost, travelling, or departed target 6,
    unpopulated container `0xd`, the uprising and prisoner columns, and
    Diplomacy's full support `0xf`. A destroyed container or target runs it
    at once (`FUN_00545240`).
  - Open: Recruitment's `0x10` (the recruit pick, phase 4), resign (5), and
    the end's observation level (phase 3). The roll is still the interim
    lead-character roll.
  - A 1500-tick seed-42 dual-AI playtest resolves 2644 missions instead of
    1823, mostly repeating Incite on its 2..12-day timer. The seed-42 golden
    is regenerated for this cause.
  - Review fixes (2026-09-29, save v19):
    - `FUN_00520ac0` reads the mission's phase (`+0x54 -> +0x1c`), not a
      class, so a destroyed container gives code 7 only past phase 6. The
      check runs once per destruction.
    - Phase 4 ends unless every living member still travels
      (`FUN_00522280`), and a null origin moves no one (`FUN_00556430`).
    - Leaving phase 8 validates before disarming the timer. The app frees
      both AIs' mission members through one shared helper.
    - Open: before phase 5 the validator should read the side's own copy of
      the target (`FUN_00521160`, `FUN_005211c0`), which the port does not
      keep.
    - The 1500-tick playtest now resolves 3076 missions. Of the 60 seeded
      characters, 51 unrecruited ones have no location, and they now stay
      nowhere instead of landing at their first target. The recruit pool
      (phase 4) takes them out of play. The golden moves to
      `v1:d8edefc22cfdf7cd` -> `v1:6f404bcf7868244a` for save v19 and the
      new mission field.
- Phase 3 (2026-09-29, save v20): the detection run `FUN_00547f60` follows
  each phase change (`crates/rebellion-core/src/mission_detection.rs`).
  - Setup `FUN_00589a40` picks mode 2, 1, or 4 for phases 2, 3 and 7, and 9,
    and which members and defenders count.
  - Decoys draw off defenders on TDECOYTB/FDECOYTB (espionage - detection,
    `FUN_00589620`) or are exposed. Defenders roll FOILTB on the team's
    average espionage - detection - special forces - G3584 (`FUN_005896e0`).
    Phase 9 finds a traitor on `draw(0..99) < 100 - loyalty`
    (`FUN_00589f10`).
  - A detected team ends 3, or 4 past phase 4, when members may resign.
    Each member faces a random defender and rolls RLEVADTB: an evader
    resigns and takes the injury roll (`FUN_0053e990`), a captured
    character is injured and held, and a captured special force is
    destroyed (`FUN_00503eb0`). Validator rule 1 ends 5 once no team member
    stands without a resign request.
  - port: no officer ranks, so the officer terms are 0; the draws come from a
    SplitMix64 stream seeded by a fifth per-mission roll.
  - The seed-42 playtest never detects: its teams are unplaced pool
    characters that no location filter counts (phase 4's recruit pool). It
    resolves 2949 missions instead of 3076 because the fifth roll shifts the
    later systems' draws. The golden moves to `v1:acbd440f1734677c` ->
    `v1:276e317d8c392b5e` for save v20 and the fifth roll.
- Phase 4a (2026-09-29): phase 10 rolls every team member
  (`FUN_00592f50`). Team characters roll first, then team special forces.
  Decoys and captives never roll.
  - The chance (slot `+0x274`) is a step lookup (`FUN_00595090`) of the
    class's MSTB table on its recovered input (`member_chance`). A missing
    table, row, or target gives 0. The draw `0..99 < chance` comes from the
    mission's seeded stream.
  - Each success raises the base skill by GNPRTB 6156..6168. Special forces
    never raise, because their slot `+0x1d8` returns 0 (`FUN_006158b0`).
  - The in-roll actions:
    - Rescue frees the target.
    - Abduction captures it with the member as captor.
    - Assassination kills it.
    - Incite runs the uprising incident `FUN_0050d030`. The result is 2
      while the holder keeps a regiment (`FUN_00509020`).
    - Subdue rolls only during an uprising, adds the `FUN_0055cb10` gain,
      and runs `FUN_0050c910`.
  - Slot `+0x280` turns result 0 into 2. Diplomacy wins `FUN_0055cac0`
    support: G6183 + rand 6184 at its side's own system, G6185 + rand 6186 at
    a neutral one (side bits 3, `FUN_004f8c60`), and none at the opponent's.
    Espionage reveals the system.
  - Deleted: the invented quadratic, foil probability, defense score,
    covert flags, the interpolating MSTB lookup, fixed effect sizes,
    `MissionOutcome::Foiled`, and the integrator's control flip. The AI and
    the panel preview read `member_chance`. The provenance baseline shrinks
    by 9, to 119.
  - The earlier Subdue gain mapped side 3 to contested. It is neutral.
  - port, interim until phases 4b and 4c: Recruitment's recruit, and the
    fixed Sabotage and Death Star Sabotage effects.
  - Roll slots per mission drop from 5 to 3: the creation timer, the repeat
    timer, and the stream seed.
  - The 1500-tick seed-42 playtest resolves 1212 missions instead of 2949.
    Incite and Recruitment now end with code 8, because the invented
    uprising start and control flip no longer run. The golden moves to
    `v1:acbd440f1734677c` -> `v1:f5bfb82d01bda045` for the per-member roll
    and the three roll slots.
- Phase 4b (2026-09-29, save v21): Recruitment signs a pool character
  (`FUN_0056b9a0`).
  - A character carries `recruited`, the original's `+0x50` bit 1.
    port: a character placed at game start begins recruited. The other 51
    of the 60 form the pool.
  - Each successful member picks `draw(0..=n-1)` among its side's living
    minor characters (families `0x38..0x3c`, MNCHARSD) without that bit (`FUN_0055ef30`, `FUN_0055fc80`,
    `FUN_0053e290`). The recruit joins at the target (`FUN_0055fe70`: slot
    `+0xa8`, then `FUN_004f7480` sets the bit). The result becomes 3, and
    leadership rises by GNPRTB 6159. An empty pool leaves the result unset,
    so the member fails.
  - Taking the last one sets the side's `+0xb8` (`FUN_0052f590`), and the
    validator then ends Recruitment with `0x10` (`FUN_0056b370`).
  - port: the pool is walked in `DatId` order. A later mission in the same
    step skips an earlier mission's pick, because the world applies recruits
    after the step.
  - The 1500-tick seed-42 playtest resolves 1053 missions instead of 1212,
    fewer Diplomacy (327, was 444) and Incite (160, was 207). A run that
    marks recruits without placing them reproduces 1212 exactly, so placing
    the recruits at their targets accounts for the whole change. The golden
    moves to `v1:c6a7cdfec25f7b55` -> `v1:2ceb75524906c42b` for save v21 and
    the recruit pool.
  - Review fix: the pool holds minor characters only (families
    `0x38..0x3c`, MNCHARSD, through `FUN_0056f450`), never a major.
- Phase 4c (2026-09-30, save v22): Sabotage and DS Sabotage name a target
  object (order `+0x4c`, read back by `FUN_00521030`).
  - The object is a defense, manufacturing, or production facility, a
    regiment, a special force, or a fleet's Death Star hull (class `0x88`,
    family `0x18`).
  - Dispatch refuses a Sabotage without an object or naming the Death Star
    (`FUN_0056a110`, `0x40`/`0x28`), and a DS Sabotage naming anything else
    (`FUN_005744c0`, `0x40`/`0x29`).
  - On result 3, `FUN_005746e0` destroys the object once (slot `+0xac(6)`).
    The validator's object rules (`FUN_00593500`) read its side, its system,
    and whether it stands.
  - Deleted: the interim removal of a system's first facility, and the
    invented 50-tick Death Star construction delay (`add_sabotage_delay`).
  - port: capital ships other than the Death Star, and fighter squadrons,
    have no identity in the port and cannot be named. A removed object
    reads as the opponent's and destroyed, so rule 4 ends the mission with
    6. The AI names the first enemy manufacturing facility at its chosen
    system (the planner's own choice is phase 5).
  - The 1500-tick seed-42 playtest resolves 1069 missions instead of 1053.
    In 4b the AI sabotaged Wistril 109 times, because removing the system's
    first facility left the enemy yard standing. Naming the yard destroys it
    within 7 missions, so Wistril completes 17 builds instead of 32. The
    golden moves to `v1:8ff65dd8cf6fd7b2` -> `v1:41144ee79484fa1a` for save
    v22 and the target objects.
- Phase 5a (2026-09-30, save v23): mission legality and the AI planners'
  team and decoy selectors (`ghidra/notes/ai-mission-planning.md`).
  - MISSNSD records gain their member rules (`+0x40..+0x4c`). Dispatch
    refuses members the record does not admit (`FUN_005830a0`,
    `FUN_00583320`): special-force mission bits outside `+0x48`, a character
    where `+0x4c` lacks `0x10000`, mixed sides, or a side the record does not
    run for. DS Sabotage is Alliance-only and Assassination Empire-only. A
    running mission is never ended by this check.
  - `crates/rebellion-core/src/mission_planning.rs` ports the AI records
    (`FUN_00401d20`, `FUN_00402230`), the candidate query (`FUN_00403460`),
    the ranking (ascending by skill, a coin flip on ties, positions from
    `FUN_0041c230`), the pick (`FUN_004357b0`: the highest sum of positions,
    only candidates every query found), and the ten kinds' selectors with
    their sizes, thresholds, and decoy caps (4 for Rescue, Abduction, and DS
    Sabotage, `FUN_004047d0`).
  - `AIAction::DispatchMission` carries the team and decoy lists. Deleted:
    the invented skill thresholds, the 30% success gate, and the Jedi skip;
    the `diplomacy_skill_threshold`, `espionage_skill_threshold`,
    `covert_min_success_prob`, and `covert_target_popularity_threshold`
    tuning fields. Assassination and Abduction go to the target character's
    system instead of the enemy's most popular one (`FUN_004bd0a0`).
  - port: the mission kind and target stay the port's (side `+0x318` is
    untraced). The planners run in one cycle, so the nearness query never
    joins and an order keeps only the members standing with its first (the
    original gathers them in state 7). A plan with no team keeps no decoys,
    a member one plan sends is not taken by the next, and the tie draws come
    from a stream seeded by the day and side.
  - hyp: the posture that sets `+0xbc` is not ported, so Espionage, Incite,
    Assassination, and Sabotage take one decoy. The side controller's leader
    and research bits are read as clear, their constructor values, so the
    four leaders stay off Incite, Rescue, Sabotage, Assassination, and DS
    Sabotage teams, and every character legal for Research (all of them) is
    reserved from Incite, Rescue, Sabotage, and Abduction teams. A character
    in a fleet holds an officer rank.
  - The 1500-tick seed-42 playtest resolves 480 missions instead of 1069
    (Diplomacy 341, Espionage 89, Recruitment 32, Sabotage 18), from 372
    orders instead of 1971. Characters reserved for research leave Sabotage
    and Incite to special forces, and Espionage takes special forces only.
    Manufacturing completes 62 builds instead of 33. The golden moves to
    `v1:cff529f786c2ecc1` -> `v1:7e1061cadd1e6f22` for save v23 and the AI
    selection.
- Phase 5b (2026-09-30): the player's original mission dialog
  (`ghidra/notes/mission-dialog.md`).
  - `crates/rebellion-render/src/mission_dialog.rs` ports the 259 by 355
    window (`FUN_0046a750`, `FUN_0046a9c0`, `FUN_0046c3c0`). It uses the
    STRATEGY page panels, title bars, tabs and buttons, the GOKRES mission
    icons and minis, and the TEXTSTRA text. The first tab holds the
    mission-kind box, its "Missions" drop-down, and the target; the second
    holds the agents and decoys with the move arrows (and double clicks).
    "Begin Mission" (`0x66`) submits and closes, "Cancel" (`0x65`) and the
    close box (`0x64`) discard the order, and "Encylopedia" (`0x67`) opens
    the Encyclopedia.
  - `missions::available_kinds` lists the kinds as `FUN_005422f0` does: the
    records in MISSNSD order that are not hidden, whose member rules admit
    the team and decoys, and whose class validator accepts a temporary
    mission (`FUN_0054c590`). With no kind, nothing opens (`FUN_0042a320`).
  - `PanelAction::DispatchMission` carries the agents and decoys. The
    missions panel's pickers open the dialog instead of dispatching, and
    lose their invented kind buttons and success preview.
  - port: until phase 7 builds the drag, those pickers stand in for the
    drop, so the target is a system and only system-target kinds are
    listed. The scroll bars and item status overlays are not drawn; the
    wheel scrolls one item. The Encyclopedia opens without the kind's entry.
  - hyp: the window is centered in the galaxy view (`FUN_00606980`), and
    choosing a drop-down item closes it.
  - The replay golden is unchanged (`v1:7e1061cadd1e6f22`); the dialog is
    player input only.
- Phase 6 (2026-09-30): telemetry and browser acceptance.
  - `EVT_MISSION_DISPATCHED` was defined but never emitted. Each applied AI
    `DispatchMission` now reports its kind, side, target, team, and decoys;
    a refused order reports nothing. `integrator::member_name` names a
    special force by its class.
  - A new test catches the pre-existing mutant in the fleet-move rollback:
    troops embarked for a move that cannot depart land again.
  - `FUN_00602150` buttons blit their bitmaps at native size
    (`FUN_00602d30` -> `FUN_005fc140` with a zero width), clipped by the
    control. The 66 by 33 "Encylopedia", "Begin Mission", and "Cancel"
    bitmaps sit in 64 by 33 controls, so they show their left 64 columns.
    The dialog had stretched them. It also sits on whole pixels now; both
    640 by 480 galaxy views center it on a half pixel (hyp placement).
  - Fixture scenarios `MissionDialogMission` and `MissionDialogAgents` (codes
    43 and 44) open the dialog for the fixture's first character at its
    primary system. `tools/interface-parity/mission-dialog.mjs` runs four fresh
    muted Chrome for Testing 151.0.7922.34 processes (two sides, two pages)
    and clicks the Agents tab once. 277,588 of 277,588 static-chrome pixels
    match 23 owned STRATEGY resources (aggregate `19e85900…f9f3ce`); text,
    the kind item, the target art, and the lists are masked and kept as
    captures. 16 of 16 requests return 200, with zero console or page
    errors. With the bitmaps stretched again, five captures differ by 1,169
    pixels each. The sixth also missed its tab click, because one-frame
    clicks were flaky; the gate now spreads a click over frames, and three
    runs in a row pass. The fixture WASM is `020a8010…cfa7853` and the runtime pack
    `02244b24…cf7d90f4`. Evidence is kept under the ignored
    `.artifacts/interface-parity/mission-dialog-2026-10-01T06-42-20-347Z-31048/`.
  - Finding: in the 1500-tick seed-42 dual-AI playtest, 372 dispatches
    (Diplomacy 220, Espionage 78, Recruitment 32, Sabotage 23, Incite 13,
    Assassination 6) carry one member and no decoys. The selectors picked
    only 4 decoys and 2 second members, and phase 5a's co-location rule
    dropped them all, because the research reservation (hyp) and the few
    special forces leave tiny candidate pools. Not fixed here.
  - Native: the dialog's input is covered by the egui harness tests. A live
    native capture waits for the phase 7 drag, which is the dialog's only
    original entry point.
- Phase 7 (2026-10-01): the original entry and cleanup.
  - Correction: a drag from the system window onto the map is a move
    (`0x201`/`0x202`), not the dialog's entry. A mission starts with a right
    click on an object, Mission, the targeting cursor, and a click on the
    target (manual pp. 39, 57, 100; `ghidra/notes/object-popup-menu.md`).
  - 7a: `game_menu.rs` draws the Game Menu Window (`FUN_00442860`) for any
    owner; the speed menu uses it, and its first two items were swapped.
  - 7b: a right click on a character or special force in a system window
    opens the object pop-up menu (`FUN_004ac5c0`, `FUN_0051d990`) with the
    STRATEGY records' rows. Mission follows `FUN_0051fe20`; Encyclopedia
    opens the Encyclopedia. port: Move, Confirmed Move, Command, Status, and
    Retire are drawn disabled.
  - 7c: Mission starts the galaxy view's targeting mode (`FUN_00429320`,
    `FUN_00422ce0`): the view holds the pointer, REBEXE.EXE cursor 1002 is
    staged and drawn with its hotspot on the pointer, and a release on a map
    system opens the dialog through `available_kinds`. port: only a map
    system is a target (no `+0x68` object hit test), Shift's pass-through is
    not ported, and Escape cancels.
  - 7d: the invented entry points are gone: Send Diplomat, Send Spy,
    `PanelAction::OpenMissionTo`, the missions panel's Dispatch tab, and
    `PanelAction::OpenMissionDialog`.
  - 7e: a refused order shows no text in the original; the side's advisor
    schedules a reaction (`FUN_00487c90` to advisor slot `+0xc`,
    `ghidra/notes/mission-dialog.md`, "Refusal"). port: the message-log lines
    stand in until P34 plays the reactions.
  - 7f: the unreachable egui system and fleet context menus and
    `PanelAction::InitiateFleetMove` are deleted (see F-007C). The browser
    gate found the pop-up menu and the dialog drawn under the system window
    they open over; both now take the order above the modeless windows.
    Fixture scenario `MissionTargeting` (code 45) and the gate's third
    scenario reach the dialog through the original entry for both sides: six
    of six fresh muted Chrome for Testing 151.0.7922.34 runs pass, 378,406
    of 378,406 checked pixels (dialog chrome and cursor 1002's 89 opaque
    pixels), 24 of 24 requests, zero console or page errors. Fixture WASM
    `b384974078a2…`, runtime pack `d1989cd84df5…`, cursor `d4196586faef…`.
    Evidence: the ignored
    `.artifacts/interface-parity/mission-dialog-2026-10-01T18-07-25-702Z-95298/`.
  - Open: a native check of the entry and of the speed menu's colors;
    character and object targets
    (Rescue, Assassination, and Abduction need them); the advisor's refusal
    reactions (P34).

### F-020: A mod with a missing dependency fails silently

- Severity: P2
- Status: remediated; native GUI check pending (mods do not load in the browser)
- Evidence: `ModRuntime::enabled_sorted` (`crates/rebellion-data/src/mods.rs`)
  prints load-order errors to stderr and returns an empty list;
  `ModError::MissingDependency` was never constructed, so `ModRuntime::errors`
  and the Mod Manager showed nothing.
- Fix (2026-09-25): `discover`, `toggle_mod`, and `refresh` now record
  `MissingDependency` and `VersionMismatch` for each enabled mod, counting a
  disabled dependency as missing. The Mod Manager matches errors by
  `ModError::mod_name()` and shows text such as "requires 'x', which is not
  installed and enabled" instead of a Debug dump. A dependency cycle or
  duplicate name records a `LoadOrder` error on every enabled mod it blocks.
  Five tests fail without the change, and scoped `cargo mutants` catches every
  mutant in the new code.
- Acceptance: missing-dependency and version-mismatch failures reach
  `ModRuntime::errors` and the Mod Manager names the mod and dependency.

### F-021: The blockade troop-destruction event is never raised

- Severity: P1
- Status: remediated; browser pass pending
- Evidence: before the fix, `BlockadeEvent::TroopDestroyed` (event `0x340`,
  `FUN_00504a00`) was matched by the integrator and the app but never built.
- Recovery (`ghidra/notes/blockade-troop-withdrawal.md`): `FUN_00504a00` only
  notifies. The loss is the regiment's withdraw-percent roll. `FUN_0050b310`
  keeps a system at 100 unless it is blockaded with no active KDY-150; then it
  is `max(0, 100 - ships * GNPRTB[7684] - fighters * GNPRTB[7685])`. A regiment
  added to a system copies that value, landing resets it to 100, and entering
  transit keeps it only if `random(0..=99) < percent` (`FUN_00504990`).
- Correction: `FUN_00504a00` is regiment vtable slot `+0x1f8`, not `+0x204`;
  `FUN_004ff3c0`'s `+0x204` call reaches `FleetBlockadeNotif` on fleet views.
- Fix: `BlockadeSystem::running_regiments` observes embarked regiments after
  arrivals and landings and again at the blockade step, and `resolve_running`
  rolls those leaving; both run in the headless step and the app frame loop.
  Destroyed regiments leave their fleet's cargo and emit
  `blockade_troop_destroyed` telemetry. Nine tests, all 37 viable mutants
  caught, seed-42 golden unchanged.
- Fix (2026-09-26): save v15 persists `BlockadeState`'s embarked-regiment
  tracking, so a regiment keeps its orbit and withdraw percent across a load;
  a v14 save migrates with empty tracking. Tests cover the round trip and the
  migration.
- Review fix (2026-09-27): the browser's `slot_occupied` skipped the v14
  keys, so a slot holding only a v14 save was overwritten without
  confirmation. Occupancy and deletion now derive their keys from
  `STORED_VERSIONS`.
- Open: the model has no system-based fighters outside fleets, and the app
  path needs a browser pass.
- Acceptance: a regiment carried into a blockaded system without a KDY-150
  rolls against the recovered percent when it leaves, garrisons and
  surface-loaded regiments are never rolled, and a browser pass shows the
  loss message.

### F-022: Two tactical tests needed untracked bitmaps

- Severity: P3
- Status: remediated in `9901c34`
- Evidence: two `tactical_view` hit-mask tests read the gitignored
  `data/base/ui` and failed in a clean checkout. They are now `#[ignore]`d with
  a reason, and `make test-assets` runs them when extracted bitmaps exist.
- Acceptance: met.

### F-023: The economy applies the blockade troop-withdraw formula as a KDY production modifier

- Severity: P2
- Status: open
- Evidence: `economy.rs` step 4 computes
  `clamp(100 - capships * GNPRTB[7684] - fighters * GNPRTB[7685])` as a
  production modifier, citing `FUN_0050a480`, a community-dump address inside
  a `CALL` operand in our binary. The community function is our
  `FUN_0050b310` + `FUN_0055a020`, the blockade withdraw percent ported under
  F-021. No recovered code applies it to production; the value feeds only
  telemetry.
- Acceptance: the modifier is dropped or re-sourced, with any golden change
  named.

### F-024: The Emperor's 1.5x battle damage bonus has no source

- Severity: P2
- Status: open
- Evidence: `combat.rs` multiplies the Empire's pending weapon damage by 1.5
  when a character named Palpatine or Emperor is in the fleet, citing
  `FUN_00542050`. Ours is a two-line thunk; the community function maps to our
  `FUN_005438a0`, a named-character check (Leia, Luke, Han, the Emperor, Vader,
  Chewbacca). Neither shows a damage modifier.
- Acceptance: the bonus is removed or traced to recovered code.

### F-025: Death Star combat checks key on the Empire major-character family

- Severity: P2
- Status: open
- Evidence: `combat.rs` derives `is_death_star` and `CombatEntityKind::DeathStar`
  from DatId family `0x34`. MJCHARSD and TEXTSTRA show `0x34000280` is Emperor
  Palpatine and `0x35000281` Darth Vader; `FUN_00560d50` routes family `0x34`
  to the SeatOfPower check. No shipped hull uses `0x34`, so shield absorption
  never runs on real data.
- Acceptance: Death Star detection uses the recovered class identity and the
  shield's battle role follows recovered code.

### F-026: The disaster and uprising incidents have no effect

- Severity: P2
- Status: remediated; browser pass pending
- Evidence: `economy.rs` raised uprising and disaster flags from invented
  thresholds and only emitted messages; `UprisingSystem` started revolts on an
  invented UPRIS1TB roll and handed the system to the rebels.
- Correction: the earlier ledger text used system vtable base `0x0065e638`;
  the base is `0x0065e640` (constructor `FUN_00507130`), so bit 16 is the
  uprising incident (`FUN_0050aa50`, slot `+0x248`) and bit 18 the disaster
  (`FUN_0050ab30`, slot `+0x250` `FUN_00511930`).
- Recovery (`ghidra/notes/uprising-incident.md`): a revolt (`+0x88` bit 2)
  starts when a held, populated system is short of troops (`FUN_0050b800`)
  and never changes control (`FUN_0050a130`). It ends only on a Subdue
  Uprising success once all regiments cover the undoubled requirement
  (`FUN_0050c910`), or when the system is lost or emptied. Its incident timer
  fires every 30 to 100 ticks (GNPRTB 7701/7702, `FUN_00586130`).
  `FUN_00559ce0` scores two draws of `1..10`, the support shortfall below 60,
  regiments (doubled for a strongly held Empire system), Stormtroopers, and
  the Incite and Subdue agents' leadership; UPRIS1TB and UPRIS2TB step
  lookups give two codes, which `FUN_0050d150` applies as a lost facility or
  regiment, an injured character, or freed prisoners, before a -2 support
  change while an Incite mission is active (`FUN_0050c9f0`, halved per
  `FUN_00559be0`). A disaster (event `0x38f`, every 1 to 400 ticks) picks a
  random system with energy or raw materials, erodes both (`FUN_00559e10`),
  and destroys each facility not en route, of either side, at 10 percent.
  The garrison requirement is halved only for a strongly supported Empire
  system and doubled only in a revolt (`FUN_00559fe0`).
- Fix (2026-09-26): `UprisingSystem::advance` implements the lifecycle, the
  incident, and the disaster timer; `apply_uprising_event` applies the losses,
  freed prisoners, support, and resources in the headless integrator and the
  app. Save v15 stores each revolt's incident timer and the disaster timer.
  The invented economy uprising and disaster triggers and
  `EconomyEvent::NaturalDisaster` are removed. Each timer fires at most once
  per advance, so a second fire never reads the world before the first one's
  losses. The Uprisings filter, loyalty panel, and sector window read the
  revolt from `UprisingState`. Twenty-eight uprising tests and
  four economy tests fail without the change; scoped `cargo mutants`
  over the bundle's diff catches 307 of 331 viable
  mutants. The 24 survivors: the two `UprisingState` migrations, caught by the
  `rebellion-data` save tests; boundary swaps equivalent at a zero change or a
  shipped injury of at least 1; the informant and resource triggers (F-029);
  the unrecovered strong-support boundary; WASM-only save paths; and egui
  draw paths awaiting the browser pass. The seed-42 golden changes for this named cause.
- Review fixes (2026-09-27): the troop surplus that starts a revolt counted
  only the holder's regiments; `FUN_0050b500` counts every regiment at a
  held system (`FUN_00504c40`, no side filter) and leaves 0 elsewhere, as the
  end check already did. The uprising roll slice was one roll per system,
  so a busy tick ran dry and later draws returned their maximum;
  `UprisingSystem::roll_budget` now reserves each due incident's and the
  disaster's worst case. New tests pin the Empire regiment weight, the
  Empire-only halving of the Incite loss, code 5 freeing every prisoner, the
  gain-before-check order, and the step-lookup clamp. Scoped `cargo mutants`
  over the fixes catches 28 of 34 viable mutants; the seed-42 golden catches
  the two mission-budget multiplier swaps, and the four WASM-only save
  mutants need the browser pass. The seed-42 golden changes from tick 15 for
  these causes and now ends at `v1:301752058a3627d8`. The port has no en-route facility state, so every
  listed facility is a disaster candidate.
- Open: the injury at character `+0x94` has no port field, so an injured
  character is reported but unchanged. The app path needs a browser pass.
- Character pick (2026-09-27, `ghidra/notes/object-state-flags.md`): codes 3
  to 5 walk only the system's direct children (`FUN_00513120`,
  `FUN_005130d0`) in mode 1, `+0x50` bit 0, usable: existing, complete, and
  not en route (`0x004f7b80`). Characters in a fleet or on a mission belong to
  that fleet or mission, so the pick now also skips mission agents, and the
  prisoner codes skip captives aboard fleets. Two tests fail without it.
- En route: `FUN_00511930` spares a facility with `+0x50` bit 4. The port
  places a finished facility at once, so it never has one en route; the
  missing delivery phase is F-030.
- Acceptance: both incidents trigger and apply their recovered effects with
  failing-without tests, and a browser pass shows a revolt, an incident, and
  a disaster.

### F-027: Community-dump citations named the wrong functions

- Severity: P3
- Status: remediated
- Evidence: `disassembly.zip` came from a different REBEXE.EXE build; its
  addresses shift by region (`+0x360` to `+0x19c0`). Thirty-two cited
  addresses were not function entries in our binary.
  `ghidra/notes/community-address-remap.md` matches 43 community functions to
  ours, and the citations now name our addresses. A five-slice check labelled
  716 citation sites: 505 confirmed, 131 vague, 23 unsupported, and 57
  contradicted. Review overturned 2 (the `FUN_005c81d0` formation order is
  correct); the other 55 are corrected or ledgered above.
- Follow-up (2026-09-27): render tests of original behaviour gained comment-only
  source citations, about 90 in the three tactical files and 40 across ten
  other render files, each naming a function, resource, or manual page already
  recorded by the runtime code or the parity evidence. Three tactical tests stay
  uncited for lack of a recovered source: capital-selection wrap-around,
  one-shot trench-run recording, and the command-panel hit regions. The
  auto-resolve projection and canvas letterbox tests cover our own code.
- Review follow-up (2026-09-27): the order-code and target-control tests now
  cite `FUN_005ca6d0` and resources 1058/1059; the mesh Z reflection is our
  own conversion. Five render tests named "original" or "recovered" with no
  recovered source are renamed, and the light-rig test notes that its
  intensities are the port's reading.
- Acceptance: met for citations.

### F-028: Probability tables interpolate where the original steps

- Severity: P2
- Status: partially remediated
- Evidence: `MstbTable::lookup` interpolates between thresholds. The original
  lookup, `FUN_00595090`, returns the value of the largest threshold at or
  below the argument and clamps below the first row.
- Fix (2026-09-26): `MstbTable::step_lookup` implements the step lookup, and
  the uprising incident uses it for UPRIS1TB and UPRIS2TB, with a test that
  fails under interpolation.
- Open: every other table consumer (missions, betrayal, escape) still calls
  the interpolating `lookup`; switching them changes their outcomes and the
  seed-42 golden, so it is a separate change.
- Acceptance: every table lookup follows `FUN_00595090`, with any golden
  change named.

### F-029: The informant and resource incidents fire on invented triggers

- Severity: P3
- Status: open
- Evidence: `economy.rs` `evaluate_incident_flags` raises the informant and
  resource flags from a troop deficit and from overcapped energy or raw
  materials. The original runs them
  on timers: resource (bit 19) on event `0x390` every 1 to 500 ticks
  (GNPRTB 7719/7720) through `FUN_00556be0` and RESRCTB; informant (bit 17) on
  a per-system timer (`+0x40`, GNPRTB 7703/7704) that rolls support in
  `FUN_0050cbe0` before INFORMTB (`FUN_0050d510`).
- Acceptance: both incidents follow their recovered timers and tables with
  failing-without tests.

### F-030: Manufactured objects arrive without an en-route phase

- Severity: P3
- Status: remediated, browser pending
- Evidence: the integrator's manufacturing completion inserts a finished
  facility, regiment, or ship straight into its system or fleet. The original
  keeps an en-route state (`+0x50` bit 4, `GameObjEnrouteNotif` `FUN_004fc240`;
  bit 5, `GameObjEnrouteActiveNotif`; `GameObjDestroyedOnArrivalNotif` for an
  object lost on arrival), and an en-route object is not usable
  (`0x004f7b80`), so the disaster (`FUN_00511930`) and the incident pick skip
  it. See `ghidra/notes/object-state-flags.md`.
- Open: the delivery rule (who sets bit 4, the travel time, and the arrival
  handler) is not yet recovered. Storing en-route objects changes the save
  format.
- Remediated 2026-09-28: a queue item may name a destination. On completion it
  departs (`FUN_0052bee0`) and arrives after the `FUN_0055d8c0` transit at its
  own speed: capital ship `FUN_00500820`, fighter FIGHTSD hyperdrive
  `FUN_00502f80`, others GNPRTB 1. An object whose destination is destroyed is
  lost on arrival (`0x303`, `FUN_004fc080`). Save v16 stores destinations and
  `DeliveryState` (`delivery.rs`, `ghidra/notes/build-delivery.md`).
  - hyp: the port holds a travelling object outside the world and places it
    on arrival, so every walk skips it. The original keeps it in the
    destination container, marked en route.
  - port: the production panel picks the destination from a list of held
    systems. The original order path is unrecovered, and AI builds stay at
    their facility.
- Acceptance: completed objects travel en route under recovered rules, the
  disaster and incident skip them, and a test fails without the phase.

### F-031: Simulation rules carry numbers with no source in the original

- Severity: P2
- Status: open
- Evidence: `tools/provenance-scan` finds 135 uncited items carrying 289
  literals or roll draws (baseline `scripts/provenance-baseline.json`).
  - `missions.rs` `foil_prob` decides every covert mission with a quadratic
    from rebellion2 `Mission.cs`; the original rolls FOILTB (table id 12) in
    `FUN_0058a130` (`ghidra/notes/decoy-roll.md`).
  - `MissionKind::coefficients` are rebellion2 or placeholder curves. They run
    only when an MSTB table is missing, so they should fail closed instead.
  - `ai.rs` has 18 uncited items, among them the production queue cap
    (`queue_len >= 3`), the troop-deployment radius (`150 * 150`), and
    `ESPIONAGE_SKILL_THRESHOLD` 50.
  - `economy.rs` `SupportTier::from_support_int` splits support at
    20/30/40/60 with no source.
- Acceptance: each item is recovered and cited, or tagged `port:`/`hyp:` with a
  reason; `provenance-scan check` passes and the baseline shrinks to the
  port-owned remainder.

### F-032: Fleet transit time and bombardment rest on an invented formula and a misread

- Severity: P1
- Status: partially remediated (transit)
- Evidence:
  - movement.rs computes ceil(distance * DISTANCE_SCALE 2 / slowest
    hyperdrive) with MIN_TRANSIT_TICKS 10 and DEFAULT_FIGHTER_HYPERDRIVE 60,
    none sourced
  - The original moves each object on its own: FUN_00514a60 -> FUN_00556430 ->
    FUN_0055d8c0, ticks = max(1, isqrt(dx^2+dy^2) / GNPRTB 5120 * speed(+0x34)
    / 100), arrival event 0x387 (ghidra/notes/build-delivery.md)
  - bombardment.rs and ghidra/notes/bombardment.md read that transit chain as
    a bombardment formula: the short pair from FUN_00509620 is the system
    position, not combat strength; the real bombardment function is
    unrecovered
  - blockade.rs matches its decompiles; fog.rs has no source and is port-owned
    until tagged
  - evidence/invention-review-r1/movement.md
  - Transit remediated 2026-09-28: movement.rs ports FUN_0055d8c0 with the
    original isqrt (FUN_0053e1d0), GNPRTB 5120 and 1, the ship speed
    FUN_00500820, and the fleet speed FUN_004fd900. A fleet without a capital
    ship cannot enter hyperspace (FUN_004fda10). The per-ship damage nibble is
    not modelled. Bombardment and fog remain open.
- Acceptance: Transit follows FUN_0055d8c0 per object with a failing-without
  test; bombardment.rs is removed or rebuilt on a recovered function; fog.rs
  is tagged port:.

### F-033: Strategic combat formulas are port-authored

- Severity: P1
- Status: open
- Evidence:
  - combat.rs reads GNPRTB 5120 (0x1400) as a combat difficulty modifier in
    space and ground combat; 5120 is the transit distance divisor
  - Weapon variance, fighter dogfight (atk_str + maneuver / 2, 0.4 attrition),
    anti-fighter screening, and the ground hit formula have no source; the
    original math sits in unrecovered ship-class slots +0x1c4, +0x1c8, +0x1d0,
    +0x1d4
  - The Emperor 1.5x bonus cites FUN_00542050, a one-line runtime thunk;
    FUN_004ee350 is a strength setter, not the ground calculator
  - death_star.rs construction (1825 ticks) and warning radius (300) are
    estimates
  - evidence/invention-review-r1/combat.md
- Acceptance: The four slot handlers and the ground caller are decompiled and
  ported with citations, the 5120 misuse is removed, and each formula has a
  failing-without test.

### F-034: Mission resolution departs from the recovered phases

- Severity: P1
- Status: open
- Evidence:
  - Detection collapses into one defense score fed to a rebellion2 quadratic;
    the original rolls FOILTB per defender with a composite input
    (FUN_0058a130)
  - Escape uses loyalty alone as its ESCAPETB input; the original input is a
    four-operand composite (community address, needs remap to our build)
  - Sabotage destroys a facility where the effect carries ticks_lost; effect
    sizes (ticks_lost 10, popularity 0.05, Death Star delay 50, diplomacy
    0.01) and MISSNSD tick ranges are unverified
  - Uprising leadership reads one agent; FUN_00520cd0 averages every team,
    decoy, and captured member; the incite effect skips the support machine
    FUN_0050c9f0
  - evidence/invention-review-r1/missions.md
- Acceptance: Detection, escape, and effects follow their decompiles with
  failing-without tests; mission members become a roster (with F-019).

### F-035: Manufacturing and repair use invented structure and rates

- Severity: P2
- Status: open
- Evidence:
  - One ProductionQueue per system; the original keeps a manager per facility,
    so two yards build in parallel
  - No queue acceptance filter (FUN_00528890: created, not completed, not
    destroyed, same side); build progress rate per tick is unverified against
    FUN_0052b960 (now decompiled)
  - Repair uses the ship class damage_control; the original runs timer 0x386
    with GNPRTB 7693/7695 and a facility-derived rate (FUN_00509890.c:26),
    gates on the shipyard bit (+0x88 bit 5) rather than any facility, and
    repair runs per frame, not per tick
  - ai.rs:1325 treats refined_material_cost as build ticks
  - evidence/invention-review-r1/manufacturing.md
- Acceptance: Queues, progress, and repair follow FUN_0052b960 and timer 0x386
  with failing-without tests.

### F-036: The AI polls on an invented cadence with invented weights

- Severity: P2
- Status: open
- Evidence:
  - The original AI is driven by notifications and timers; there is no daily
    AI tick and 0x1f0 is a mission UI message (ghidra/notes/timer-
    scheduler.md)
  - AI_TICK_INTERVAL 5 and AiConfig tick_interval 7 are invented; 15 further
    constants (deploy budgets, force ratios, scoring weights, 11 duration_roll
    0.5 placeholders) have no source, and DIPLOMACY_SKILL_THRESHOLD comes from
    rebellion2
  - The aggression curve cites FUN_0053e190 but is linear; system_strength
    weights facilities by 10 where FUN_00502020 does not
  - evidence/invention-review-r1/ai.md
- Acceptance: Every AI constant is recovered, or tagged port: where the polled
  architecture has no counterpart, and the two wrong formulas follow their
  decompiles.

### F-037: Economy and research run on an invented cadence and miss recovered terms

- Severity: P2
- Status: open
- Evidence:
  - The port recomputes support every tick; the original runs FUN_00508250
    once at setup (stage 0x17) and then updates through field hooks
    (FUN_00510820, FUN_005109f0, FUN_00511740) and timers 0x381, 0x383-0x386
  - FUN_00559be0 divides positive Alliance and negative Empire support shifts
    by GNPRTB 7681; the port omits it, and FUN_0050c9f0 adds the current
    support before clamping
  - resolve_system_control's GNPRTB 7760 energy threshold has no source;
    FUN_0050b610 resolves control from loyalty (FUN_0055a080)
  - FUN_0050b310 is the blockade withdraw percent, not a production modifier;
    the maintenance 0x304 cadence is invented; timer 0x381 (resource tally)
    has no port counterpart
  - research.rs is taken from rebellion2 and cites nothing in REBEXE.EXE
  - evidence/invention-review-r1/economy.md (row 9 on Empire doubling is
    excluded; see the README)
- Acceptance: Support follows the hook and timer chain with failing-without
  tests, and research is recovered or tagged.

### F-038: Character timers are missing and story events are port-authored

- Severity: P2
- Status: open
- Evidence:
  - Injury recovery (timer 0x388, character +0x94, GNPRTB 2563/2564) has no
    port field or timer; timers 0x389 and 0x38a are unported
  - Jedi detection uses flat per-tier probabilities in place of FUN_0055e4d0,
    FUN_0055ff60, and FUN_0058a530; betrayal (interval 50, loyalty - 50), XP
    thresholds, and detection intervals have no source
  - The Jabba chain, carbonite countdown, Dagobah, Emperor arrival, and Final
    Battle sequences use invented tick thresholds and probability gates
  - events.rs reuses real timer ids for story events: 0x384, 0x386, 0x387, and
    the unarmed 0x396
  - Nine notification ids match their decompiles
  - evidence/invention-review-r1/characters.md
- Acceptance: Character timers are ported, story events are recovered from the
  original or tagged port:, and event ids no longer collide with timer ids.

### F-039: Ghidra notes and agent docs repeat unverified claims

- Severity: P2
- Status: open
- Evidence:
  - FUN_004927c0 is called the master tick and 0x1f0 the daily AI trigger
    (agent_docs/ghidra-re.md:155, ai-parity-tracker.md:52,92,123);
    FUN_004927c0 formats notification text and only the mission code sends
    0x1f0
  - FUN_00508250 is described as per tick; it runs once at setup
  - Reviews and notes cite community-disassembly addresses, which name
    different functions in our build (ghidra/notes/community-address-remap.md)
  - entity-system.md:383 conflicts with two iterator recoveries on the fleet
    type range
- Acceptance: Each false claim is corrected at its source and every note
  citation resolves to a .c line in our build.

### F-040: Original mission kinds the port lacks

- Severity: P2
- Status: open
- Evidence:
  - MISSNSD holds 25 records; the port has 10 agent kinds plus Autoscrap.
    The missing kinds, with their outcome slots:
    - Reconnaissance `0x54` (always succeeds, `56bec0`)
    - Research `0x53` (three records, repeat, `56cdb0`)
    - Palace `0x64` (rescue plus free every prisoner there, `56e650`)
    - Bounty `0x65` (`575de0`)
    - Jedi Training `0x58` (`5712b0`)
    - Dagobah `0x71`, Vacation `0x72` and Pickup `0x73`
    - the movement pseudo-missions `0x41..0x44`

    See `ghidra/notes/mission-lifecycle.md`.
  - F-019 ports phases, decoys, and the per-member roll for the port's 10
    kinds only (decision 2026-09-28).
- Acceptance: Each original kind runs through the F-019 lifecycle with its
  recovered slots, AI planner, and legality rules, or is documented as
  deliberately excluded with evidence.

## Fable 5.1 audit synthesis

The Fable review confirmed the original blockers and sharpened several
interpretations:

- For the non-original menu-music extension, Fable judged the behavior
  shippable after a one-function visual pass: six-pixel canopy clearance,
  Project Thorn-style cyan confined to the projection, a rectangular inset
  status lamp, static rest art, restrained 5 Hz hover interference, pressed
  displacement, gold-only focus, and congruent paint/hit bounds. F-016E
  applies and verifies each requirement.
- The `19 pass / 5 fail / 1 skip` parity score is not five equivalent gameplay
  failures. The original AI interval is a documented augmentation, three Death
  Star checks expose structural AI/event gaps, and the research mismatch may be
  an oracle-ID mismatch. The run remains practically degenerate despite the
  evaluator's `false` flag because it only detects the absence of combat.
- The Fable finding that main-menu Load Game skipped slot selection and faction
  restoration has since been remediated and browser-verified under F-001.
- Browser main-menu music and effects are now packaged, but advisor voice and
  cutscene/media parity remain incomplete. Release packaging must continue to
  stage owned runtime assets explicitly.
- Auto-resolve and tactical calculations can still produce different outcomes.
  Tactical ground results now persist exact survivor damage and occupation
  captures characters, but full path convergence remains M2.
- Autoresearch parameter tuning should remain paused until faction liveness,
  capture/evasion, the wider campaign loop, and target acquisition are
  established.

## Optimization and parity roadmap

The order below makes the acceptance ledger executable. Milestones are gated;
later work must not hide failures in an earlier invariant.

| Milestone | Scope | Exit criteria |
|-----------|-------|---------------|
| M0 — Truth and bleeding | Wire save/load/delete and Load Game selection; fix native HD root and `TROOPSD.DAT`; ship browser data; attach evidence to README claims; add deterministic replay gates. Browser Save/Load/Delete, paths, a self-contained four-request package, F-011A fingerprints, the F-011B1 continuation envelope, the F-011B2/B3 replay pipeline, and F-011B4 native/WASM fixture equivalence are verified. Native GUI restart and persistence hardening remain open. | Persistence works on native/WASM, the packaged site boots from a clean directory, and claims link to current evidence. |
| M1 — Simulation correctness | F-007A–D close redispatch, fleet-position, player-dispatch, and combat-backlog defects. F-007E closes ownership, friendly cycling, transit fan-in, troop-class lookup, blockade garrison, repair state, troop transport, political occupation, and the source-backed asymmetric victory contract. Capture/evasion, faction liveness, target acquisition, and the wider campaign loop remain open. | Across five fully identified 5,000-tick seeds: transit ≤10% of fleets, move orders ≤1.5× arrivals, fleet arena ≤3× initial, both factions remain capable of productive action, every contract-correct victory fixture passes, and native/WASM checkpoints match. Record the provisional encounter volume/spread numbers as diagnostics until calibrated. Qualitative behavior matches the historical campaign reference. |
| M2 — One game engine | Route app and playtest through one tick API and event sink; make combat resumable from core state; construct victory UI; remove or correctly simulate `AdvanceTicks`. | Same seed plus command stream yields identical checkpoints and final state across interactive, headless, native, WASM, auto, and tactical paths. |
| M3 — Browser excellence | Extend the verified deterministic `runtime.orpk` foundation with Brotli compression, bounded raw/decoded caches, HD entries, high DPI, one egui pass, cached geometry, IndexedDB, gesture-unlocked audio, owned advisor assets, and cross-browser input suites. | Cold start ≤3 s at 50 Mbps/30 ms, ≤4 requests before menu, combined heap/WASM ≤256 MB after 10 minutes, no visual/input failures in current Chrome/Firefox/Safari. |
| M4 — Multiplayer | Introduce validated, tick-stamped commands; authoritative host simulation; faction-filtered fog-safe deltas and snapshots; secure WSS transport; prediction/reconciliation; reconnect; persistence and observability. | Two clients run 5,000 ticks with matching server checkpoints every 250 ticks; at 200 ms RTT there are no input stalls and ≤1 reconciliation per 100 commands; reconnect within 60 s; all illegal commands rejected; hidden state absent from client memory. |
| M5 — Continuous proof | Enforce format/clippy/build/browser checks; short and long campaign gates; resource and screenshot ledgers; app integration tests; package boot and data/save hashes. | Every supported P00–P40 pass is green from release artifacts, with reproducible evidence retained by CI. |
| v1.0 — Protected Cloudflare release | Deploy the self-contained browser build to Cloudflare Pages with Functions middleware, `SITE_PASSWORD` and `SESSION_SECRET` secrets, signed secure cookies, asset headers, preview/production environments, and rollback instructions. | Anonymous requests cannot retrieve HTML, WASM, DAT, bitmap, save, or multiplayer endpoints; valid login survives navigation; invalid/expired/tampered sessions fail closed; logout works; independent browser acceptance verifies gameplay and bitmap evidence through the deployed URL in current Chrome, Firefox, and Safari. |

### Browser performance budgets

- Startup: menu interactive within 3 seconds at 50 Mbps and 30 ms RTT.
- Simulation: ≤4 ms per native tick and ≤12 ms per WASM tick at 1,000 fleets.
- Rendering: ≤8 ms per 1280×800 galaxy frame at the reference fixture.
- Distribution: optimized WASM ≤5 MB; initial resource requests ≤4.
- Memory: combined JS heap and WASM memory ≤256 MB after 10 minutes.
- Regression policy: fail CI when a tracked metric regresses by more than 10%.

### Multiplayer target architecture

Single-player should execute through the same validated `Command` boundary used
by multiplayer. A server owns the authoritative simulation and emits
fog-filtered `Delta` or `Snapshot` messages per faction; clients never receive a
complete hidden `GameWorld`. Each command carries `{player, tick, sequence}` and
is validated and rate-limited server-side. Begin with an in-process transport
for deterministic tests and secure WebSockets for deployment. Add WebRTC only
as an optional trusted lockstep mode after deterministic replay is proven.

The existing `net_protocol.rs` is a notification vocabulary, not a complete
network envelope. Isolate protocol and server responsibilities in dedicated
`rebellion-net` and `rebellion-server` crates rather than coupling sockets to UI
panel actions.

## Feature-by-feature acceptance plan

Each row is a bounded pass. A pass closes only after the user-visible behavior
and underlying state mutation are both demonstrated.

| Pass | Features | Required acceptance |
|------|----------|---------------------|
| P00 | Supported scope | Define native and WASM contracts. Explicitly include or exclude browser audio, video, advisor, EData, and mods. |
| P01 | Build infrastructure | Formatting, warning-free all-target check, strict clippy, all required tests, native build, actual WASM packaging, and artifact inspection. |
| P02 | Data and startup | Every required/optional DAT table, string names, entity counts, troop classes, clean boot, and missing/corrupt-data errors. |
| P03 | Main menu | **Pass.** Assemble and operate the original 14 cockpit controls, animation, navigation, keyboard and screen-reader access, `MDATA.300` music, original button effects, responsive 4:3 hit testing, and the documented optional music-only extension per [`main-menu-parity.md`](../../../agent_docs/main-menu-parity.md). Native and browser acceptance pass. |
| P04 | Game setup | Use the cockpit controls for both factions, three difficulties, three original galaxy sizes, both game types, direct campaign start, correct state propagation, and clean second-campaign reset per [`main-menu-parity.md`](../../../agent_docs/main-menu-parity.md). |
| P05 | Clock | Pause and every speed, focus loss, browser background/resume, modal/combat/cutscene tick behavior. |
| P06 | Galaxy navigation | Reproduce the original galaxy/GID, sector, and system-window graph, selection, object menus, pan/zoom semantics, resizing, high-DPI transform, and cockpit input boundaries per the [interface audit](../2026-09-10-interface-parity-audit/). First-pass sector and system shells plus the rail lifecycle work; the synthetic galaxy, complete item compositions, commands, uncommon states, and exact rail thumbnails remain open. |
| P07 | Fog and overlays | Both factions, original intelligence/control colors, sensor/recon visibility, every GID filter and matching legend, fleet/facility/system markers, and no hidden-information leakage. Current primitive markers fail visual parity. |
| P08 | Personnel | Reproduce Personnel Finder, Character Status, special forces, every portrait/state, selection, assignment, availability, captivity, injury, death, Jedi state, item menus, and detail refresh. The current Officers panel is a replacement. |
| P09 | Fleets | Reproduce Fleet/Ship Finders, original Fleet and status windows, tabs, icons, selection, move, cancel, split/merge/transfer, cargo, transit, invalid operations, arrivals, duplicate prevention, and post-combat refresh. Current custom panels remain visual failures. |
| P10 | Economy | Income, collection, support drift, maintenance, shortfall, incidents, nonnegative invariants, and faction ownership effects. |
| P11 | Manufacturing | Enqueue, cancel, prioritize, capacity, costs, each product category, blocked production, completion, and usable world insertion. |
| P12 | Diplomacy | Legal targets, probability boundaries, success/failure, support/control change, cancellation, messages, and persistence. |
| P13 | Recruitment | Eligibility, success/failure, unique recruitment, repeated-order prevention, cancellation, telemetry, and persistence. |
| P14 | Espionage | Legal targeting, intelligence visibility, success/failure, foiling, informants, messages, and persistence. |
| P15 | Sabotage | Normal and Death Star sabotage, damage/delay, defense interaction, success/failure, telemetry, and persistence. |
| P16 | Character operations | Assassination, abduction, rescue, capture, health, death cleanup, escape, decoys, foiling, and autoscrap. |
| P17 | Uprisings and betrayal | Incite, subdue, thresholds, occupation, control transitions, allegiance changes, suppression, and telemetry. |
| P18 | Research | Ship/troop/facility trees, assignment exclusivity, progression, unlock events, no duplicate projects, and mid-project save/load. |
| P19 | Jedi | Force discovery, eligibility, training tiers, trainer loss, captive/dead states, story interactions, and persistence. |
| P20 | Movement | Distance timing, Han bonus, order/cancel/reorder, references, arrival, battle triggering, and no duplicate orders. |
| P21 | Blockade and repair | Enter/exit, ownership/economy effects, breach outcomes, hull recovery, cost/cap, interruptions, and persistence. |
| P22 | AI | Both factions; validator pass/reject boundaries; budgets; research; production; troop deployment; recon; defense; retreat; target deconfliction; Death Star escort/targeting. |
| P23 | Core space combat | Seven phases, weapon classes, shields, ion effects, recharge, carriers, fighters, officers, Emperor, retreat, destruction, and result application. |
| P24 | Tactical space combat | Placement, selection, formations, movement, focus fire, pause/speed, retreat, visual state, accepted formulas, and galaxy result application. [Per-hull/fighter result identity and shared entry](evidence/2026-09-12-tactical-result-identity.md) are verified partial tranches. P54 through P57B2C2B stage and decode the original 3D corpus and recover its camera, placement, transform, palette, light, and retained-mode state. [P58A](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-resource-join.md) joins every ship and fighter DAT identity to its original resources and transports all 87 meshes and 397 textures. P58B through P58F13 render live capital/fighter families and restore their interaction and command paths. The [P58-B06 checkpoint](../2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-completion-bundle.md) adds capital and fighter combat, collision, automatic groups, retained formations, the separate Death Star, original result/options panels, superlaser journey, both trench-run routes, and exact strategic roster, capture, and Death Star-state application. P58-B15 through P58-B21 restore Battle Alert, weapon and command voice banks, mixed-task-force rejection, recovery/withdrawal feedback, and shared post-battle orchestration. [P58-B22](../2026-09-10-interface-parity-audit/evidence/2026-09-28-tactical-source-completion.md) completes the bounded practical launcher with source-compatible tactical randomness, persisted power allocation, completion/destruction callbacks, and the timed source-derived trench-run producer. [P58-B07](../2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-106-matrix-contract.md) generates and validates the exact 106-cell denominator, and [P58-B08](../2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-a0-ingestion.md) adds fail-closed original-capture provenance and ingestion. Deterministic A1 mapping is runnable at 106/106, but owned lossless A0 coverage and acceptance remain 0/106. Exact native beam pixels, audible native playback and mixing, whole-process random-state continuity, strategic commander binding, exact planet framing, rare warning/ejection callers, and strict battle acceptance remain open. |
| P25 | Ground combat | Troop attack/defense, facilities, officers/difficulty, selection, casualties, conquest, visuals, and parity between automatic and interactive paths. |
| P26 | Bombardment | Eligibility, shields, losses, popularity, ownership, messages, persistence, and visual feedback. |
| P27 | Death Star | Construction, sabotage, escort, retreat, shielding, targeting, firing, cooldown, destruction, cleanup, contribution to the Imperial HQ objective, and nonterminal Alliance destruction behavior. |
| P28 | Generic events | Every condition/action branch, one-shot behavior, simultaneous events, notification art, state changes, and save/load. |
| P29 | Story and cutscenes | Every story chain, all eight cutscene mappings, heritage branches, queueing, audio/video sync, skip/end/error behavior, and replay prevention. |
| P30 | Victory and defeat | Every win/loss condition for both player factions, simulation freeze, result screen, cutscene policy, Continue, and clean restart. |
| P31 | Save/load/delete | UI actions, slot refresh, populated round-trip, native restart, browser restart, corruption, compatibility, quota errors, delete, and deterministic continuation. |
| P32 | Mods | Discovery, dependency order, cycles, versions, enable/disable/reload, New Game reapplication, save mismatch, hot reload, and additive-feature scope. |
| P33 | Audio | Music, SFX, voices, context transitions, gain/mute, missing files/devices, browser user-gesture policy, and platform scope. |
| P34 | Droid advisors | Both factions, original embedded chrome, every decoded sequence, exact frame IDs/order/timing, priority, message/audio behavior, missing frames, and packaged browser assets. The authentic idle-frame transport/rendering tranche passes; shell overlap and authored behavior remain open, including the mission-refusal reactions F-019 traced (`ghidra/notes/mission-dialog.md`, "Refusal"). |
| P35 | Encyclopedia and EData | Original Index/Topic surfaces, every category/entity, exact EDATA identity, navigation, system focus, fallback, and browser loading. [P62](../2026-09-10-interface-parity-audit/evidence/2026-09-28-encyclopedia-artwork-transport.md) transports all 187 owned images. Source cross-checking reclassifies [P63](../2026-09-10-interface-parity-audit/evidence/2026-09-28-message-index-shell.md) as Message Index evidence. [P64](../2026-09-10-interface-parity-audit/evidence/2026-09-30-encyclopedia-index-shell.md) proves the production-dormant index shells and controls through 32 exact browser states. [P65](../2026-09-10-interface-parity-audit/evidence/2026-09-30-encyclopedia-index-catalog.md) adds all 356 source entries, seven English labels and family filters, stable selection, scrolling, and the selected-category no-op through 22 two-faction browser states. [P66A](../2026-09-10-interface-parity-audit/evidence/2026-10-01-encyclopedia-topic-source-bindings.md) strictly extracts 348 topic texts and 191 artwork mappings, binds 346 complete topics per faction, and exposes ten source-empty mission records without fallback. [W2](../2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-content-session.md) adds an inactive immutable session and atomic last-known-good publication contract over that authority. [W3](../2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-presenter.md) adds pure source-ordered composition and exact return routes. [W4](../2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-topic-surface.md) drives the authentic shell and selected-only texture lifecycle through ten fixture-gated browser A1 cases. Canonical packaged readers, contextual production routing, A0 comparison, and the strict `OBJ-01` matrix remain open. |
| P36 | Bitmap and interface sweep | Complete every required cell in the [43-family interface ledger](../2026-09-10-interface-parity-audit/2026-10-06-surface-ledger.json), with exact resources, composition, geometry, hotspots, native/browser screenshots, and zero invented or unknown visible elements. |
| P37 | Campaign acceptance | Short smoke runs and long multi-seed campaigns for both factions/difficulties with bounded fleet/event growth, balance, diversity, victory, and full parity reports. |
| P38 | Release artifacts | Fresh native install and deployed browser package, exact artifact contents, startup/storage/media/input tests, and documentation generated from results. |
| P39 | Protected Cloudflare deployment | Preview and production Pages deployments, secret-backed password gate, signed session cookie, logout/expiry/tamper tests, cache/security headers, asset/API access denial before authentication, deployed single-player/multiplayer smoke tests, rollback, and retained browser evidence. |
| P40 | GitHub Pages documentation | Publish the maintained project documentation from `main`; verify Jekyll-safe Markdown, working internal links, current README/audit/roadmap content, successful deployment, and a public smoke test. |

P40 passed on 2026-09-08. Pages run `34306480934` completed from `main` at
`aaf428d286e482471662881124fbad78062fce6f`, and the public landing page,
lowercase audit directory index, and fleet evidence page returned HTTP 200.
See `evidence/2026-09-08-github-pages.md`. This documentation pass does not
imply that the gameplay and release-artifact passes above are complete.

## Bitmap proof protocol

The [interface parity audit](../2026-09-10-interface-parity-audit/) is the
canonical visual-identity protocol. It adds stable surface/state IDs, authority
tiers, exact pixel and hotspot thresholds, navigation-graph coverage, provenance
tracing, a 370-image source corpus, and the hard prohibition on invented visible
UI. The checks below remain the minimum integration subset.

A bitmap passes only when all of the following evidence exists:

1. A resource-ledger entry identifies the screen, state, DLL source, resource ID,
   expected entity or artwork, dimensions, and platform.
2. The manifest entry reconciles with an existing file and the file decodes.
3. Runtime logs show the correct native path or browser HTTP request and cache key.
4. A screenshot shows the correct image, aspect ratio, state, layering, and fallback.
5. The related control or state transition works; rendering alone is insufficient.
6. Original-only, valid-HD, missing-HD, and corrupt-HD cases are exercised.

Required bitmap-bearing surfaces:

- Galaxy cockpit background and both factions' cockpit buttons
- Officer portraits
- Capital-ship and fighter miniatures in fleet panels
- Encyclopedia miniatures and EData art
- Story and generic event artwork
- Tactical task-force panels, gauges, hull/shield detail, and ship art
- Alliance and Imperial droid-advisor frames
- Cutscene display frames where applicable

## Evidence contract

Every feature result must record:

- Feature ID and commit SHA
- Platform, OS/browser version, build profile, data hashes, and asset hashes
- Faction, difficulty, galaxy size, seed, and fixture
- Initial state and exact user inputs
- Expected and observed state mutation
- Command, output, and exit status
- Browser console and network logs when applicable
- Screenshot or recording for visual/input behavior
- Negative and error-path case
- Final disposition: `pass`, `fail`, `blocked`, or explicitly `excluded`

No feature passes merely because a panel opens, code compiles, or a placeholder
or fallback appears.

## Independent browser acceptance loop

Use the browser-acceptance workflow selected in
[`agent-tooling.md`](../../../agent_docs/agent-tooling.md). Reviewer and
debugger runs remain read-only until a fix is explicitly authorized.

After an authorized fix:

1. Rerun the original reproduction.
2. Run adjacent regression cases.
3. Have an independent browser reviewer confirm the evidence.
4. Update the JSON ledger.
5. Close only that feature ID.

## Definition of 100% functional

The claim is permitted only when:

- Every supported feature ID through P40 is `pass` on every claimed platform.
- All P0 and P1 findings are closed.
- Required tests have no failures or unexplained skips.
- Format, warning-free check, strict clippy, native build, and packaged WASM
  build all pass.
- Bitmap/resource reconciliation has zero unexplained misses.
- Multi-seed campaigns satisfy bounded-growth, balance, diversity, parity, and
  victory criteria.
- The same results are reproduced from release artifacts rather than development
  directories.
- Any unsupported platform feature is explicitly documented and excluded from
  the corresponding completion percentage.
