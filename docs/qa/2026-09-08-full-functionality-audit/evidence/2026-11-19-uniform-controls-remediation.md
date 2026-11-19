---
title: "Uniform controls: Mini remediation and acceptance"
description: "Control fixes, regression tests, scoped mutation evidence and complete browser journey results"
category: qa
created: 2026-11-19
updated: 2026-11-19
---

# Uniform controls: Mini remediation and acceptance

Bead `or-ep-uniform-ui-controls-gru5.9` is **closed** with all five acceptance
criteria passing. The 25 tested source/harness files are published at
`d9797b451e4d0c5f66bad48255f7ab878f41ae89` on
`will-forster/refactor/strategic-window-components`. Will authorized source
publication and closure; WASM binaries remain local and unstaged.
No original-interface cell or protected release gate is promoted.

Before publication, all 25 file hashes and the complete source diff matched
the successful recorded run. The passing gates below were reused without
rerunning unchanged source. Task pages were regenerated immediately after
closing the bead at `2026-11-19T18:28:45.892799Z`.

The base is `refactor/strategic-window-components` at
`a4b0ae8e4e37768ca10fbd6c395fa567a16d85b3` on Will's local Mini (`warhammer`).
The tested implementation patch SHA-256 is
`7ba736a073da6203171a793f398ecf8181e529500d61b0089dae396096405f8b`. The adjacent JSON records the hashes of all
25 changed source/harness files and every matrix row and journey cell.
The prior [baseline validation](2026-11-19-uniform-controls-validation.md)
retains the original failures; this report records their remediation.

## Changes and regression evidence

- `StrobeButton` and Checkbox share the rectangular input adapter. Button
  capture uses the press origin when motion shares a frame. Encyclopedia
  and Confirmed Move preserve that origin through bitmap-mask adapters.
- Title release applies motion batched with the release before clamping.
  System drags retain the managed window's exact area identity. Managed
  Missions windows update live drag positions before copying presentation state.
- Encyclopedia gives its search field a pointer-only lease while the list
  owns keys. The old field lease reclaimed focus and painted a blinking caret:
  the failing frames differed in exactly 34 pixels at x=244–245, y=95–112.
- TabControl shares tooltip/selection completion. Defenses shares its two
  character fixture; Fleet shares routed pointer setup while retaining each
  test's semantic assertions and covering both factions.
- Tests cover scrollbar boundaries, blits, held art and degenerate travel;
  tab focus, masked selection and tooltips; StringField focus; managed/batched
  title movement; text scaling; list keys; and masked input phases.
- The redundant post-draw StringField Escape focus request is removed.
  egui clears Escape focus before drawing; the admitted lease already restores
  it through `install_focus`. The retained Escape test checks actual widget focus.
- Obsolete `clippy::assert_is_empty` attributes are removed, `AppKit` is quoted
  in doc comments, and a Fleet cancel match is simplified. Exact floating
  assertions in eight test modules retain their original assertions and gain
  `#[expect(clippy::float_cmp)]` with an explicit exact-value contract. Strict
  production lint flags are unchanged.
- The canonical Encyclopedia gate saves both unstable frames and failure
  diagnostics. Main-menu browser checks now exercise every selector physically,
  verify the production accessibility state, and verify bitmap repaint.
  No golden, tolerance, or existing assertion is weakened.

Batched button/release tests failed before their fixes. The managed System,
managed Missions and Encyclopedia focus tests also reproduced the failures
before their corresponding production changes. The System fixture was moved
inside both faction apertures so its exact movement assertion does not hit
an unrelated aperture clamp. Newly added coverage catches the four later
mutation survivors. Raw failed and successful attempts are retained.

## Readiness and mutations

All commands ran locally with the sanitized Rust toolchain path; no SSH or
`mini-cargo.sh` was used. `make build` is warning-free.

| Command | Exit | Seconds |
|---|---:|---:|
| `make fmt` | 0 | 1.33 |
| `make build` | 0 | 3.02 |
| `make check` | 0 | 36.36 |
| `./scripts/build-wasm.sh` | 0 | 118.5 |
| `bash scripts/build-interface-test-wasm.sh` | 0 | 174.51 |

`make check` passes 3,347 workspace tests, 23 asset tests, and 16 Node harness
tests. The renderer has 1,243 passing tests and three ignored tests; total
Cargo ignored count is 46. The asset gate separately runs its applicable
ignored tests. Strict Clippy uses `-W clippy::pedantic -D warnings`.

| Mutation campaign | Generated | Caught | Unviable | Missed at that run | Timeout |
|---|---:|---:|---:|---:|---:|
| mutations | 289 | 237 | 49 | 1 | 2 |
| input-mutations | 77 | 66 | 7 | 4 | 0 |
| adapter-mutations | 12 | 5 | 6 | 1 | 0 |
| survivor-recheck | 4 | 4 | 0 | 0 | 0 |

These are **382 mutation attempts across four campaigns**, including repeated
mutants; they are not 382 unique changes. The first survivor was the removed
redundant focus request. Four later survivors (two scaled fonts, list-key
forwarding, and masked button input) are caught by the final four-mutant
recheck; the adapter campaign repeats the masked-input survivor. No unexplained
non-equivalent survivor remains. The two retained timeouts replace positive
scrollbar tile increments with decrements and make the paint loops nonterminating;
the safety timeout detects those mutations. Unchanged tooltip evidence is
retained in the baseline report. Every campaign's command template, exact-command record hash, counts and
surviving outcome disposition is in the JSON. Exact machine paths are retained
in the hashed raw command records. The isolated mutation copy was removed
only after 23 changed Rust source hashes matched the unmutated working tree.

Production WASM SHA-256:
`5f800c7b6f5c71e9521fa58d4b76657c1b3ec2b1321b449b9ac8a8f36e48147e`

Interface-test WASM SHA-256:
`db3fd4e32e4f1a1ae4ad34b2b9fbcafb60d3d9bf52f5fc140587ffa194261683`

## Inventory and clones

All 62 primary rows retain a canonical object or source-backed exclusion.
The JSON includes production calls, owning modules, representative observable
tests and each disposition. The constructor, renderer and exclusion searches
produce 648, 213 and 24 matches. Source-backed exclusions remain the immovable
Sector window, unused outline drag mode, and absent tactical stock-tooltip consumer.

The exact same `jscpd 5.4.3` flags (`--reporters ai --format rust --min-tokens
70 --no-tips`, plus `--ignore-identifiers --ignore-literals` for normalized)
produce **0 exact clones (0.0%) and 289 normalized candidates (3.3%)**. The
baseline for this audit was 9 exact / 297 normalized; the earlier reviewed
baseline was 0 / 263. The selected rectangular input, tab completion, Defenses
character fixture and Fleet routed-pointer fixture duplicates are gone.

Every normalized candidate is enumerated with locations and disposition in
the JSON: 157 test/setup candidates, 47 overlapping self-matches, 14 import or
configuration candidates and 71 feature-owned candidates. Build/Mission popup
adapters keep distinct domain choices, Defenses/Missions keep typed window
and list projections, and Battle Alert/Results keep different page/command
policies around canonical controls. Resource tables, graphics and domain
routing are not replaced with unrelated abstractions to reduce a percentage.

## Browser journeys

Each row below uses the fresh hashes above. All browsers launch through the
interface-parity harness with both mute flags. The JSON retains each case's
faction, viewport, status, console/network diagnostics and evidence hash.

| Journey | Faction / viewport results | Exit |
|---|---|---:|
| `game-options-smoke.mjs` | alliance 1280×960: 1 pass; empire 1280×960: 1 pass | 0 |
| `build-selection.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `mission-dialog.mjs --no-build` | alliance 640×480: 3 pass; empire 640×480: 3 pass | 0 |
| `fleet-finder.mjs --no-build` | alliance 640×480: 9 pass; empire 640×480: 9 pass | 0 |
| `fleet-move.mjs --no-build` | alliance 640×480: 6 pass; empire 640×480: 6 pass | 0 |
| `fleet-window.mjs --no-build` | alliance 640×480: 20 pass; empire 640×480: 20 pass | 0 |
| `sector-quadrants.mjs --no-build` | alliance 640×480: 6 pass; empire 640×480: 6 pass | 0 |
| `status-window.mjs --no-build` | alliance 640×480: 5 pass; empire 640×480: 5 pass | 0 |
| `orbital-strike.mjs --no-build` | alliance 640×480: 11 pass; empire 640×480: 11 pass | 0 |
| `encyclopedia-index-shell.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `encyclopedia-canonical-surface.mjs --no-build` | alliance 640×480: 5 pass; alliance 800×600: 1 pass; empire 640×480: 5 pass; empire 800×600: 1 pass | 0 |
| `message-index-shell.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `audit-batch.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `encyclopedia-index-shell.mjs --catalog --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=galaxy --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=sector --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=system --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=popular-support --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=selection --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=hover --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-group-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-selected-damage-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-subsystem-field-command-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-live-subsystem-damage-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-subsystem-repair-mobility-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-command-assignment-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-command-execution-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-command-progression-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-attack-targeting-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-attack-target-lifecycle-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-death-star-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-battle-results-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-battle-options-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-battle-options-withdrawal --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-death-star-laser-journey --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-selected-navigation-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-navigation-camera-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-detail-escort-presentation --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --battle --all --scenario=production-battle-alert-entry --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |

Cell totals: {'pass': 244}. Retained evidence contains 3932 files and 258 inspected screenshots.
Manifest SHA-256: `52cb782d0435ed422305b1fe5cde5ed074482f301caeac9dff6fc219b07b6b81`. `progress.json` locates the ignored raw evidence on this Mini.

## Manual, limits and cleanup

Manual printed pages 22, 62, 75–76, 83–85 and 146 were reread in full after
remediation; page 67 was also read for selector assertions. Existing contracts
and `FUN_006028c0`, `FUN_006071e0`, `FUN_00607010`, `FUN_00605160` and
`FUN_00405560` settle press coordinates, live movement/release, parent-owned
field focus and menu selectors. No new simulation rule is introduced.

This is implementation acceptance. No native GUI or fresh original-game A0
claim is made. Missing Missions/0x299a bars, popup keyboard focus and unrecovered
tooltip registrations remain explicit inventory limits. Browsers and servers
are closed at completion and the final audio-out assertion check is clean.
The production server saw client-aborted transfer BrokenPipe exceptions during
movie skipping; retained page/network diagnostics are assessed separately.
