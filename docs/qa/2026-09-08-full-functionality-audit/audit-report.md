---
title: "Open Rebellion Full Functionality Audit"
description: "Repository status, verified evidence, release blockers, and feature-by-feature acceptance plan"
category: qa
created: 2026-09-08
updated: 2026-09-10
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
selection and dispatch passed in the following checkpoint. The campaign-history
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
dispatch passed in the next checkpoint. The following victory checkpoint
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
  forces, and merges anonymous same-faction arrivals deterministically. The
  then-current five-seed runs passed the fleet-arena and move/arrival bounds.
  F-007C wires
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
  Qualitative behavior is adjudicated against the historical reference.

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
| v1.0 — Protected Cloudflare release | Deploy the self-contained browser build to Cloudflare Pages with Functions middleware, `SITE_PASSWORD` and `SESSION_SECRET` secrets, signed secure cookies, asset headers, preview/production environments, and rollback instructions. | Anonymous requests cannot retrieve HTML, WASM, DAT, bitmap, save, or multiplayer endpoints; valid login survives navigation; invalid/expired/tampered sessions fail closed; logout works; Astra medium verifies gameplay and bitmap evidence through the deployed URL in current Chrome, Firefox, and Safari. |

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
| P24 | Tactical space combat | Placement, selection, formations, movement, focus fire, pause/speed, retreat, visual state, accepted formulas, and galaxy result application. [Per-hull/fighter result identity, shared entry, and muted two-faction browser entry](evidence/2026-09-12-tactical-result-identity.md) are verified partial tranches; return, interactions, original bitmap interface, and battle acceptance remain open. |
| P25 | Ground combat | Troop attack/defense, facilities, officers/difficulty, selection, casualties, conquest, visuals, and parity between automatic and interactive paths. |
| P26 | Bombardment | Eligibility, shields, losses, popularity, ownership, messages, persistence, and visual feedback. |
| P27 | Death Star | Construction, sabotage, escort, retreat, shielding, targeting, firing, cooldown, destruction, cleanup, contribution to the Imperial HQ objective, and nonterminal Alliance destruction behavior. |
| P28 | Generic events | Every condition/action branch, one-shot behavior, simultaneous events, notification art, state changes, and save/load. |
| P29 | Story and cutscenes | Every story chain, all eight cutscene mappings, heritage branches, queueing, audio/video sync, skip/end/error behavior, and replay prevention. |
| P30 | Victory and defeat | Every win/loss condition for both player factions, simulation freeze, result screen, cutscene policy, Continue, and clean restart. |
| P31 | Save/load/delete | UI actions, slot refresh, populated round-trip, native restart, browser restart, corruption, compatibility, quota errors, delete, and deterministic continuation. |
| P32 | Mods | Discovery, dependency order, cycles, versions, enable/disable/reload, New Game reapplication, save mismatch, hot reload, and additive-feature scope. |
| P33 | Audio | Music, SFX, voices, context transitions, gain/mute, missing files/devices, browser user-gesture policy, and platform scope. |
| P34 | Droid advisors | Both factions, original embedded chrome, every decoded sequence, exact frame IDs/order/timing, priority, message/audio behavior, missing frames, and packaged browser assets. The authentic idle-frame transport/rendering tranche passes; shell overlap and authored behavior remain open. |
| P35 | Encyclopedia and EData | Original Index/Topic surfaces, every category/entity, exact EDATA identity, navigation, system focus, fallback, and browser loading. WASM currently returns no encyclopedia texture. |
| P36 | Bitmap and interface sweep | Complete every required cell in the [43-family interface ledger](../2026-09-10-interface-parity-audit/surface-ledger.json), with exact resources, composition, geometry, hotspots, native/browser screenshots, and zero invented or unknown visible elements. |
| P37 | Campaign acceptance | Short smoke runs and long multi-seed campaigns for both factions/difficulties with bounded fleet/event growth, balance, diversity, victory, and full parity reports. |
| P38 | Release artifacts | Fresh native install and deployed browser package, exact artifact contents, startup/storage/media/input tests, and documentation generated from results. |
| P39 | Protected Cloudflare deployment | Preview and production Pages deployments, secret-backed password gate, signed session cookie, logout/expiry/tamper tests, cache/security headers, asset/API access denial before authentication, deployed single-player/multiplayer smoke tests, rollback, and retained Astra evidence. |
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

## Astra orchestration loop

Use GPT-6-Astra at low effort for routine reproduction and medium effort for
cross-system diagnosis. Reviewer and debugger runs remain read-only until a fix
is explicitly authorized.

```bash
ASTRA=/Users/tomdimino/.claude/skills/codex-orchestrator/scripts/codex-astra.sh

$ASTRA reviewer \
  "Audit FEATURE_ID using its acceptance case. Do not modify files. Reproduce it on native and WASM and attach command, state, console, network, and screenshot evidence." \
  --reasoning low --service-tier default --no-approve

$ASTRA debugger \
  "Diagnose failed FEATURE_ID from the attached reproduction. Identify the smallest coherent fix and adjacent regression tests. Do not modify files." \
  --reasoning medium --service-tier default --no-approve
```

After an authorized fix:

1. Rerun the original reproduction.
2. Run adjacent regression cases.
3. Have an Astra reviewer confirm the evidence independently.
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
