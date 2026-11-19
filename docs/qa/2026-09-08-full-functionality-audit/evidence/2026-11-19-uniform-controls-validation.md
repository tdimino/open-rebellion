---
title: "Uniform control adoption: local Mini validation"
description: "Final consumer inventory and failing acceptance gates for uniform original-game controls"
category: qa
created: 2026-11-19
updated: 2026-11-19
---

# Uniform control adoption: local Mini validation

`or-ep-uniform-ui-controls-gru5.9` remains **in progress**. Canonical ownership
is inventoried, but the zero-exact-clone, mutation and repository readiness
criteria do not pass. This is implementation evidence; it does not promote
any original-interface acceptance cell or establish A0 parity.

The inspected branch is `refactor/strategic-window-components`, commit
`220e0a0709a999d49cec6272776a254e2030f131`, equal to the fetched
`will-forster/refactor/strategic-window-components` tip. Validation ran locally
on Will's Apple Silicon Mac Mini (`warhammer`), with Rust 1.98.0, jscpd 5.4.3,
cargo-mutants 27.1.0 and pinned Chrome for Testing 151.0.7922.34. No SSH or
`mini-cargo.sh` invocation was used. Agent Mail identity `CreamOwl` claimed the
ready, unassigned bead after confirming that `.3`, `.5`, `.6`, `.7` and `.8`
were closed. `.10` title dragging is included in this audit.

No Rust implementation changed. No obsolete helper was identified that could
be removed within the handoff's scope. Documentation corrects stale migration
claims and records the remaining gaps. The tracked production WASM was rebuilt.
Will authorized an evidence-only commit and push after reviewing the failed
gates; the bead remains in progress. The user's earlier Mini/capture work remains in
stash `3085a5df9ac3b7005a09dc85dd7f224c8eaadc42`, verified across 33 files before
this validation. The stopped original-game VM was not changed.

Changed tracked paths are `.beads/issues.jsonl`, `tasks/INDEX.md`,
`tasks/acceptance.md`, `ghidra/notes/cool-control-migration-matrix.md`,
`ghidra/notes/cool-control-contracts.md`, `agent_docs/roadmap.md`,
`web/open-rebellion.wasm`, and this audit directory's `README.md`,
`audit-report.md`, `audit-report.json` and new validation Markdown/JSON pair.
The ignored `progress.json` records the handoff and local evidence locations.
The authorized commit includes the eleven documentation/task paths above;
`web/open-rebellion.wasm` remains a local, unstaged build artifact. It changes
no Rust source, gate, golden, dependency or acceptance threshold. Publication
targets `will-forster/refactor/strategic-window-components` and uses the
repository's `Co-authored-by: Codex <codex@openai.com>` trailer.

## Readiness and component tests

| Local command | Result |
|---|---|
| `make fmt` | Pass after installing the missing rustfmt component; initial missing-component failure retained. |
| `make build` | Pass, warning-free. |
| `make check` | **Fail** at strict Clippy after all tests passed. 118 diagnostics in rebellion-core: two unknown `clippy::assert_is_empty` allowances and 116 exact floating-point comparisons. No lint or assertion was weakened. |
| `./scripts/build-wasm.sh` | Pass. Production fixture tokens: zero. |
| `bash scripts/build-interface-test-wasm.sh` | Pass. Fixture validation: 38 scenarios / 152 catalog cells. This is build validation, not 152 runtime passes. |

The workspace test output records 3,326 passed and 46 ignored; the asset-test
commands add 23 passed, for **3,349 passed, zero failed, 46 ignored**. Renderer
library tests account for 1,222 passes and three ignored. All tests in these
commands executed on the source commit above; ignored tests are not passes.
The representative behavior names, component counts and commands are in the
[structured evidence](2026-11-19-uniform-controls-validation.json).

Production WASM SHA-256:
`a6fc8a2acdaa56279679e05c4f5f3e68a11fec14c75bdff89518f7e73b52c739`.
Interface-test WASM SHA-256:
`c450dbcc6d3177bdaa30e0969ebc8014439f786df5bd54dcbddc6cfda642cb3d`.
Both were built before the journeys; Rust source remained unchanged afterward.

## Inventory and boundaries

The three searches in the [migration matrix](../../../../ghidra/notes/cool-control-migration-matrix.md#reproducible-inventory-and-clone-baseline)
produced 648 original-constructor references, 206 renderer matches and 24
exclusion matches before documentation updates. A second source inventory
traced actual shared-object calls and executed behavior tests. Those text
match counts are discovery evidence, not numbers of controls.

There are 62 primary rows: 11 button/checkbox, 13 tab, 14 list, two popup,
eight text field, eight slider/status/tooltip and six title-drag dispositions.
The table below maps every row. Paths are relative to
`crates/rebellion-render/src`; the structured evidence retains exact call
locations and representative executed test names. A row marked adopted means
shared ownership, not complete feature parity.

The controls and focused scrollbar/tooltip modules have no GameWorld,
GameEffect, WindowManager, WindowCommand or domain-action dependency. Feature
modules retain row data, faction resources, commands and lifecycle;
`rebellion-windowing` retains top-level identity, focus, capture and routing.
`move_confirmation::bitmap_button` is an active mask-input adapter over
StrobeButton, not the removed `mission_dialog::button`. Finder macros route
Personnel and Troop consumers through the same canonical objects.

The bounded exclusions remain developer panels/command palette/probes,
replacement multiplayer name entry, text articles versus DragLists, overlay
layer order versus registered tooltips, platform SC_* messages, palette/sprite
primitives, immovable Sector detail and uninstantiated XOR outline mode.
The original's horizontal scrollbar two-axis test is an explicitly rejected
original bug; the port follows the active axis.

| Row | Consumer | Canonical path / production owner | Disposition and observable evidence |
|---|---|---|---|
| B01 | New-game/main-menu selectors and actions | `controls/strobe_button.rs`; `main_menu.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Direct browser selector assertions are absent; menu traversal is insufficient. Tests: `main_menu`; journey listed in the matrix. |
| B02 | Strategic cockpit controls and Message Index rail lights | `controls/strobe_button.rs`; `cockpit.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `cockpit`; journey listed in the matrix. |
| B03 | Mission Dialog chrome, tabs, arrows and actions | `controls/strobe_button.rs`; `mission_dialog.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `mission_dialog`; journey listed in the matrix. |
| B04 | Build Selection page arrows and confirm/cancel | `controls/strobe_button.rs`; `build_selection.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `build_selection`; journey listed in the matrix. |
| B05 | Confirmed Move | `controls/strobe_button.rs`; `move_confirmation.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `move_confirmation`; journey listed in the matrix. |
| B06 | Fleet, Personnel and Troop Finders | `controls/strobe_button.rs`; `finder_common.rs`, `fleet_finder.rs`, `personnel_finder.rs`, `troop_finder.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_finder, personnel_finder, troop_finder`; journey listed in the matrix. |
| B07 | Status and Operation Report | `controls/strobe_button.rs`; `status_window.rs`, `operation_report.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `status_window, operation_report`; journey listed in the matrix. |
| B08 | Manufacturing/System, Fleet, Defenses and Missions windows | `controls/strobe_button.rs`; `system_window.rs`, `fleet_window.rs`, `defenses_window.rs`, `missions_window.rs`, `strategic_window_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window, fleet_window, defenses_window`; journey listed in the matrix. |
| B09 | Encyclopedia and Message Index | `controls/strobe_button.rs`; `encyclopedia.rs`, `message_index.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `encyclopedia, message_index`; journey listed in the matrix. |
| B10 | Game Options buttons and tactical toggles | `controls/strobe_button.rs`; `game_options.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `game_options`; journey listed in the matrix. |
| B11 | Tactical strips, selected-unit/command panels, Battle Alert and Battle Results | `controls/strobe_button.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| T01 | Operation Report | `controls/tab_control.rs`; `operation_report.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `operation_report`; journey listed in the matrix. |
| T02 | Battle Alert | `controls/tab_control.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| T03 | Manufacturing/System | `controls/tab_control.rs`; `system_window.rs`, `manufacturing_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window, manufacturing_window`; journey listed in the matrix. |
| T04 | Encyclopedia | `controls/tab_control.rs`; `encyclopedia.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `encyclopedia`; journey listed in the matrix. |
| T05 | Fleet/Ship Finder | `controls/tab_control.rs`; `fleet_finder.rs`, `finder_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_finder`; journey listed in the matrix. |
| T06 | Personnel Finder | `controls/tab_control.rs`; `personnel_finder.rs`, `finder_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `personnel_finder`; journey listed in the matrix. |
| T07 | Message Index: category/view state in `message_index.rs` | `controls/tab_control.rs`; `message_index.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `message_index`; journey listed in the matrix. |
| T08 | Mission Dialog | `controls/tab_control.rs`; `mission_dialog.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `mission_dialog`; journey listed in the matrix. |
| T09 | Troop Finder | `controls/tab_control.rs`; `troop_finder.rs`, `finder_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `troop_finder`; journey listed in the matrix. |
| T10 | Battle Results | `controls/tab_control.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| T11 | Missions window | `controls/tab_control.rs`; `missions_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `missions_window`; journey listed in the matrix. |
| T12 | Fleet window | `controls/tab_control.rs`; `fleet_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_window`; journey listed in the matrix. |
| T13 | System Defenses | `controls/tab_control.rs`; `defenses_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `defenses_window`; journey listed in the matrix. |
| L01 | Defenses grid | `controls/drag_list.rs + scroll_bar.rs`; `defenses_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `defenses_window`; journey listed in the matrix. |
| L02 | Missions and member lists | `controls/drag_list.rs + scroll_bar.rs`; `missions_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Source constructs two 0x29fc bars; current bar pixels remain omitted. Tests: `missions_window`; journey listed in the matrix. |
| L03 | Fleet left and contents lists | `controls/drag_list.rs + scroll_bar.rs`; `fleet_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_window`; journey listed in the matrix. |
| L04 | Manufacturing facility pages | `controls/drag_list.rs + scroll_bar.rs`; `system_window.rs`, `manufacturing_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window, manufacturing_window`; journey listed in the matrix. |
| L05 | Fleet/Ship Finder | `controls/drag_list.rs + scroll_bar.rs`; `finder_common.rs`, `fleet_finder.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_finder`; journey listed in the matrix. |
| L06 | Personnel Finder | `controls/drag_list.rs + scroll_bar.rs`; `finder_common.rs`, `personnel_finder.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `personnel_finder`; journey listed in the matrix. |
| L07 | Troop Finder | `controls/drag_list.rs + scroll_bar.rs`; `finder_common.rs`, `troop_finder.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `troop_finder`; journey listed in the matrix. |
| L08 | Message Index and display | `controls/drag_list.rs + scroll_bar.rs`; `message_index.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `message_index`; journey listed in the matrix. |
| L09 | Encyclopedia index/topics | `controls/drag_list.rs + scroll_bar.rs`; `encyclopedia.rs`, `encyclopedia_surface.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `encyclopedia, encyclopedia_surface`; journey listed in the matrix. |
| L10 | Operation Report | `controls/drag_list.rs + scroll_bar.rs`; `operation_report.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `operation_report`; journey listed in the matrix. |
| L11 | Status rows | `controls/drag_list.rs + scroll_bar.rs`; `status_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. 0x299a status scrollbar art is absent from the approved pack. Tests: `status_window`; journey listed in the matrix. |
| L12 | Mission Dialog Agents/Decoys | `controls/drag_list.rs + scroll_bar.rs`; `mission_dialog.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `mission_dialog`; journey listed in the matrix. |
| L13 | Battle Alert list | `controls/drag_list.rs + scroll_bar.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| L14 | Battle Results lists | `controls/drag_list.rs + scroll_bar.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| P01 | Build class selector | `controls/selection_box.rs`; `build_selection.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. 0x299a popup pixels omitted; parent popup keyboard focus remains unwired. Tests: `build_selection`; journey listed in the matrix. |
| P02 | Mission kind selector | `controls/selection_box.rs`; `mission_dialog.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. 0x299a popup pixels omitted; parent popup keyboard focus remains unwired. Tests: `mission_dialog`; journey listed in the matrix. |
| F01 | Six Game Options save names | `controls/string_field.rs`; `game_options.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `game_options`; journey listed in the matrix. |
| F02 | Build Selection quantity | `controls/string_field.rs`; `build_selection.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `build_selection`; journey listed in the matrix. |
| F03 | Manufacturing-band rename | `controls/string_field.rs`; `system_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window`; journey listed in the matrix. |
| F04 | Encyclopedia search | `controls/string_field.rs`; `encyclopedia.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `encyclopedia`; journey listed in the matrix. |
| F05 | Fleet/Ship Finder name | `controls/string_field.rs`; `finder_common.rs`, `fleet_finder.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_finder`; journey listed in the matrix. |
| F06 | Personnel Finder name: the same `FinderUiState::name` through `personnel_finder.rs` | `controls/string_field.rs`; `personnel_finder.rs`, `finder_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `personnel_finder`; journey listed in the matrix. |
| F07 | Troop Finder name: the same `FinderUiState::name` through `troop_finder.rs` | `controls/string_field.rs`; `troop_finder.rs`, `finder_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `troop_finder`; journey listed in the matrix. |
| F08 | Fleet in-place rename | `controls/string_field.rs`; `fleet_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `fleet_window`; journey listed in the matrix. |
| S01 | Slider: Game Options music and effects volumes | `controls/slider.rs`; `game_options.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `game_options`; journey listed in the matrix. |
| S02 | StatusBar: three Manufacturing band progress bars | `controls/status_bar.rs`; `system_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window`; journey listed in the matrix. |
| S03 | StatusBar: sector planet energy, raw-material and support meters | `controls/status_bar.rs`; `sector_window.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `sector_window`; journey listed in the matrix. |
| S04 | StatusBar: selected capital and alternate ship-style panels, hull and shield | `controls/status_bar.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| S05 | StatusBar: Death Star charge | `controls/status_bar.rs`; `tactical_view.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `tactical_view`; journey listed in the matrix. |
| S06 | Stock tooltip: cockpit counters, command controls, Message Index rail and game-speed controls | `tooltip.rs`; `cockpit.rs`, `tooltip.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `cockpit, tooltip`; journey listed in the matrix. |
| S07 | Stock tooltip: strategic window buttons/tabs (Move, Report, Alert, Manufacturing, Sector meters, Encyclopedia, Finders, Message Index, Mission Dialog, Troop, Results, Missions, Fleet and Defenses) | `tooltip.rs`; `strategic_window_common.rs`, `encyclopedia.rs`, `operation_report.rs`, `mission_dialog.rs`, `system_window.rs`, `fleet_window.rs`, `defenses_window.rs`, `missions_window.rs`, `message_index.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Only configured registrations are rendered; other source-proven labels/geometry remain absent. Tests: `encyclopedia, operation_report, mission_dialog`; journey listed in the matrix. |
| S08 | Stock tooltip: tactical task-force/fighter strips and selected panels | `none: absent feature`; `tactical_view.rs` | Source-backed absent-feature exclusion: tactical registrations exist in original; no stock-tooltip renderer consumer or second implementation exists. Text/geometry recovery remains open. Tests: `source exclusion`; journey listed in the matrix. |
| D01 | System detail | `strategic_window_common.rs + system_window.rs`; `system_window.rs`, `strategic_window_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window`; journey listed in the matrix. |
| D02 | Fleet detail | `strategic_window_common.rs + system_window.rs`; `fleet_window.rs`, `strategic_window_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window`; journey listed in the matrix. |
| D03 | Defenses detail | `strategic_window_common.rs + system_window.rs`; `defenses_window.rs`, `strategic_window_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window`; journey listed in the matrix. |
| D04 | Missions detail | `strategic_window_common.rs + system_window.rs`; `missions_window.rs`, `strategic_window_common.rs` | Canonical object adopted; feature retains domain actions, resources and lifecycle. Tests: `system_window`; journey listed in the matrix. |
| D05 | Sector detail | `none: source-backed exclusion`;  | Source-backed exclusion: FUN_004591d0 -> FUN_004ac120 policy 0; Sector is immovable. Tests: `source exclusion`; journey listed in the matrix. |
| D06 | XOR outline | `none: source-backed exclusion`;  | Source-backed exclusion: no retained current constructor sets +0xf4 mode 2; no XOR consumer is invented. Tests: `source exclusion`; journey listed in the matrix. |

## Clone disposition

The same-flags scans cover `crates/rebellion-render/src` and
`crates/rebellion-windowing/src`, without excluding tests:

```sh
jscpd --reporters ai --format rust --min-tokens 70 --no-tips crates/rebellion-render/src crates/rebellion-windowing/src
jscpd --reporters ai --format rust --min-tokens 70 --ignore-identifiers --ignore-literals --no-tips crates/rebellion-render/src crates/rebellion-windowing/src
```

The reviewed baseline was zero exact pairs and 263 normalized candidates
(3.2%). The current unchanged source has **nine exact pairs (0.1%) and 297
normalized candidates (3.4%)**. This audit removed no clone family. The
migration already replaces feature-local button, tab, list, popup and text
mechanics with shared objects; the count increase alone does not assess those
migrations. The zero-exact criterion nevertheless fails without an exception.

| Exact pair | Disposition |
|---|---|
| `controls/strobe_button.rs:453–462` / `652–661` | Active StrobeButton/Checkbox egui input gathering; not obsolete. |
| `controls/tab_control.rs:321–346` / `371–396` | Active rectangular/masked child painting, tooltip and selected-output tails; not obsolete. |
| `defenses_window.rs:2215–2224` / `2252–2261` | Faction fixture setup in tests. |
| `fleet_window.rs:4068–4077` / `4136–4145` | Routed-frame test setup. |
| `fleet_window.rs:4071–4077` / `4237–4243` | Routed-frame test setup. |
| `fleet_window.rs:4104–4122` / `4203–4221` | Routed-frame test setup. |
| `fleet_window.rs:4106–4117` / `4156–4167` | Routed-frame test setup. |
| `fleet_window.rs:4139–4145` / `4237–4243` | Routed-frame test setup. |
| `fleet_window.rs:4201–4224` / `4270–4292` | Routed-frame test setup. |

The normalized candidate index in the structured evidence retains all 297
locations and dispositions. Relevant production survivors are:

| Family / locations | Why retained |
|---|---|
| Build Selection `777–836`; Mission Dialog `869–934` | Calls into `StrobeButton::show_pair` with different art, geometry, quantity/team actions and popup commands. They no longer contain two popup kernels. |
| Defenses `275–289,429–443,934–1098`; Missions `236–250,459–473,1016–1150` | Feature state, coordinate adapters, routed snapshots and lifecycle; shared DragList/TabControl own control mechanics. |
| Defenses `1141–1166`; System `1666–1691` | Feature-owned TabSpec art/configuration and a common TabControl call. |
| Encyclopedia `1286–1305,1697–1716` | Feature surface/original bitmap-mask adapters. Geometry and source art stay with the feature. |
| Fleet/System/Missions managed draw, commit and identity structures | Window/domain lifecycle is intentionally outside the control kernel. Test helpers in these regions remain tests. |
| Fleet/Personnel/Troop configuration and shared Finder calls | Row projection, destination policy and faction art remain in their feature; shared Finder code owns control composition. |
| Tactical Alert `7372–7405` / Results `7614–7647` | Typed control notifications map to distinct domain actions and list reset policies; both use shared tab/list objects. |
| Tactical control/resource enums, meter drawing, keyboard/action maps; cockpit resource/label tables; tooltip text tables | Resource selection, graphics adaptation and domain mapping differ even when identifiers/literals are erased. |
| Imports, overlapping self-matches, macros, test fixtures and graphics helpers | No independent obsolete control implementation is established by these normalized matches. No abstraction is made merely to lower the percentage. |

## Mutation evidence

The unchanged child-bead records were inspected. Their retained summaries are
historical component evidence, not fresh local runs or proof that all adapters
were mutation-tested:

| Family / bead | Recorded scope and result |
|---|---|
| StrobeButton / Checkbox `.2` | 17 generated: 16 caught, one unviable. |
| DragList `.4` | 419 generated: 379 caught, 37 unviable, three explained/limited survivors. The exact drag-threshold weakness was subsequently covered by `.5`; paint coverage was browser-scoped. |
| Remaining list migration `.5` | 90 generated: 86 caught, three unviable, one threshold survivor; a focused rerun caught that survivor, yielding 87 caught for the same set. |
| SelectionBox `.6` | Initial 214: 182 caught, 15 unviable, 17 missed; follow-up 173: 159 caught, nine unviable, five explained (constant equivalence, clamp equality, unreachable cell edges and an unported scrollbar paint path). |
| Slider / StatusBar `.8` | 52 generated: 44 caught, five unviable, three missed; the three targeted reruns were caught after boundary tests, yielding 47 caught for that set. |
| Title clamp `.10` | 37 generated in its filter: 34 caught, two unviable, one unrelated font-field survivor. No title-clamp survivor was recorded. |

TabControl had no completed prior mutation run, StringField had no usable
counted local evidence, and ScrollBar/Tooltip kernel evidence was missing.
Fresh runs therefore used an isolated APFS copy of the same source and local
build cache. No branch/worktree was created. The first TabControl/StringField
baseline lacked copied fonts and failed before mutation; that failed attempt
is retained separately and contributes no passes.

```sh
cargo mutants --in-place -p rebellion-render --file crates/rebellion-render/src/controls/tab_control.rs --file crates/rebellion-render/src/controls/string_field.rs --output "$EVIDENCE/mutations-tab-string-with-fonts" -- --lib
cargo mutants --in-place -p rebellion-render --file crates/rebellion-render/src/scroll_bar.rs --file crates/rebellion-render/src/tooltip.rs --re 'ScrollBar|ScrollRange|ScrollDrag|TooltipState|TooltipPresentation' --output "$EVIDENCE/mutations-scroll-tooltip" -- --lib
```

`EVIDENCE` denotes the local validation evidence directory. Each run retained
its baseline, generated diffs, per-mutant build/test logs and outcome JSON.
The structured evidence lists every survivor and its disposition. Surviving
non-equivalent changes are recorded as **unresolved test gaps**, not accepted
exceptions. Passing browser runs used unmutated WASM and do not kill mutants.

| Fresh family | Generated | Caught | Unviable | Missed | Timeout |
|---|---:|---:|---:|---:|---:|
| StringField | 97 | 79 | 17 | 1 | 0 |
| TabControl | 57 | 22 | 29 | 6 | 0 |
| ScrollBar / ScrollRange | 150 | 103 | 2 | 43 | 2 |
| TooltipState / TooltipPresentation | 20 | 14 | 6 | 0 | 0 |
| Total | 324 | 218 | 54 | 50 | 2 |

StringField's survivor reverses the Escape focus predicate. TabControl's six
survivors cover masked selection/painting, fallback keyboard selection and
both tooltip guards. ScrollBar's 43 cover thumb/hit boundaries, normalized
first-row use, zero travel, blit generation and held-arrow bitmap predicates.
Two subtract-instead-of-add tile-loop mutants time out after 22 seconds,
consistent with nontermination. The `travel > 0` / `>= 0` candidate is not
certified equivalent: ordinary overflowing rows have positive travel, but a
large usize count rounded to f32 can collapse it to zero without an explicit
production bound. All 50 missed changes remain unresolved for acceptance.

## Browser execution scope

Every command ran through `tools/interface-parity` with the pinned Chromium
arguments `--mute-audio --disable-audio-output`; no agent-browser or native
GUI launch was used. The fixed strategic harnesses support 640×480, Game
Options supports 1280×960, the catalog runner supports 640×480 and 1280×800
letterboxed, and canonical Encyclopedia additionally declares a longest-topic
case at 800×600. Coverage is recorded exactly at those sizes; none is inferred
from another viewport.

The tactical selection below covers the matrix's strips, selected/status
panels, command interactions, Death Star charge, Battle Alert/Results and Game
Options. The six strategic catalog scenarios cover cockpit controls, sector
meters, title drag, selection and hover. Mesh-only and unrelated rendering
catalog scenarios add no control consumer to this matrix and are outside this
adoption audit. Ordinary named window harnesses cover the remaining rows.

All catalog commands below append `--no-build`; tactical commands also include
`--battle`. Named strategic scripts append `--no-build`, except
`game-options-smoke.mjs`, which consumes the production web build. The catalog
variant is exactly `encyclopedia-index-shell.mjs --catalog --no-build`.

| Journey command | Faction / viewport result | Exit |
|---|---|---:|
| `game-options-smoke.mjs` | alliance 1280×960: 1 pass; empire 1280×960: 1 pass | 0 |
| `build-selection.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `mission-dialog.mjs --no-build` | alliance 640×480: 3 pass; empire 640×480: 3 pass | 0 |
| `fleet-finder.mjs --no-build` | alliance 640×480: 9 pass; empire 640×480: 9 pass | 0 |
| `fleet-move.mjs --no-build` | alliance 640×480: 6 pass; empire 640×480: 6 pass | 0 |
| `fleet-window.mjs --no-build` | alliance 640×480: 20 pass; empire 640×480: 20 pass | 0 |
| `sector-quadrants.mjs --no-build` | alliance 640×480: 6 pass; empire 640×480: 6 pass | 0 |
| `status-window.mjs --no-build` | alliance 640×480: 5 pass; empire 640×480: 5 pass | 0 |
| `orbital-strike.mjs --no-build` | alliance 640×480: 11 pass; interrupted, not accepted as a complete run | -9 |
| `orbital-strike.mjs --no-build` | alliance 640×480: 11 pass; empire 640×480: 11 pass | 0 |
| `encyclopedia-index-shell.mjs --no-build` | alliance 640×480: 1 fail; empire 640×480: 1 fail | 1 |
| `encyclopedia-canonical-surface.mjs --no-build` | alliance 640×480: 1 fail, 4 not_run; alliance 800×600: 1 not_run; empire 640×480: 5 not_run; empire 800×600: 1 not_run | 1 |
| `message-index-shell.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `audit-batch.mjs --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `encyclopedia-index-shell.mjs --catalog --no-build` | alliance 640×480: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=galaxy --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=sector --no-build` | alliance 1280×800: 1 pass; alliance 640×480: 1 pass; empire 1280×800: 1 pass; empire 640×480: 1 pass | 0 |
| `run.mjs --all --scenario=system --no-build` | alliance 1280×800: 1 fail; alliance 640×480: 1 fail; empire 1280×800: 1 fail; empire 640×480: 1 fail | 1 |
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

There are 39 distinct planned commands: 36 pass and three fail. The extra
Orbital Strike attempt was interrupted and is retained separately. Excluding
that partial attempt, cell results are 226 passed, 7 failed and
11 not run. All 19 tactical scenarios pass all four faction/viewport
cells. Every result records its command, cell identity and hash in the JSON.

## Failures, missing evidence and cleanup

- Encyclopedia shell at 640×480 fails three comparisons for each faction.
  Alliance differing pixels: 1,555 outside-to-inside press, 1,527
  inside-to-outside press and 3,082 underlying cockpit blocking. Empire:
  1,540 / 1,537 / 3,077. Inspected crops show category-button art differences;
  these assertions remain failures. No golden or tolerance was changed.
- Canonical Encyclopedia fails the first Alliance 640×480 index journey at
  `selection-last`: hashes `ea1e9c2d75a752301e10bcc0923f0036de691908bcee0e56a3995f2b40014f18`
  and `8657a0a511ff0d88bfdf929ae86103633c242707cc957c54833cb696e71e0553` differ.
  The fail-fast harness leaves eleven later cells unrun, including all Empire
  and both 800×600 cells, and does not persist its buffered console/network
  diagnostics on this failure. Earlier screenshots remain. A clean boot or
  catalog pass does not replace the missing canonical evidence.
- System title drag fails all four catalog cells. The 640×480 Alliance
  release frame retains the window at its original (225,76), while the probe
  expects (185,56). Unit-tested release clamping does not settle this runtime
  gesture failure. Its cause remains unresolved.
- Main-menu traversal has no direct selector-choice assertions. Existing
  main-menu unit tests are not claimed as browser selector acceptance.
- Missing Missions/0x299a bars, popup keyboard focus and unrecovered tooltip
  registrations remain explicit limits of the migration inventory.

The completed retained case diagnostics show no page/console/network errors
and no stale WASM hashes. Canonical Encyclopedia diagnostics are unavailable
as described above. Game Options' local server logged BrokenPipe exceptions
when clients aborted downloads; its page/network logs record no errors.
Screenshots were inspected for actual rendered faction shells and feature
states, with original PNGs retained; empty fixture content is not treated as
a black boot. No A0 equality is inferred.

The Orbital Strike monitor stopped the first attempt at 46.56 seconds after
seeing BuiltInSpeakerDevice `audio-out`. macOS logs attribute the 4.812-second
assertion to Apple Phone pid 25180, not Chromium. Only the owned test process
group was stopped; unrelated applications were left alone. The successful
retry has 70 clean audio samples. All other completed attempts have clean
samples. Final `pmset -g assertions` has no `audio-out`; all test browsers and
harness servers are closed. The owned mutation copy was removed after its
four source hashes matched the unmutated checkout.

Retained evidence contains 3585 files, including 257 inspected screenshots,
console/network JSON, assertion failures, mutation diffs/logs, silence samples
and exact command logs. The SHA-256 manifest is
`005e829494e13551ecee8b41e2faa25723536e6ae7d75901e30b648d1748464a`. The local recovery record
`progress.json` locates the ignored evidence directory; this report identifies
evidence by source commit, fixture/campaign values, faction, viewport and hash.
For example, Game Options restores day/tick 0 slots 0 and 5 with fingerprints
`v1:6fce726c78199e5b` (Alliance) and `v1:f3f786033a35ffe0` (Empire).

## Manual cross-check and acceptance state

Manual printed pages 22, 62, 75–76, 83–85 and 146 were read before and after
the audit. The text was read in full for those pages; decompiled handlers were
spot-checked against the existing control contracts.

| Manual promise | Trace and current disposition |
|---|---|
| Delayed hover tips (22,62) | `FUN_006008e0` / `FUN_00600a40` / `FUN_00600cc0` confirm stock registration; configured strategic targets present, other registrations partial/missing. |
| Select, activate, right-menu and drag (62) | `FUN_006083c0` confirms list notifications; shared behavior tests pass, runtime System title gesture remains failed. |
| Six save-name fields and volume drag (75–76) | `FUN_00407180`, `FUN_00604cf0`, `FUN_00604570`; present, both-faction Game Options smoke passes. |
| Build quantity, popup, facility tabs and progress (83–85) | `FUN_00437f80`, `FUN_00455060`, `FUN_0060bed0`, `FUN_004acec0`; shared controls present with popup-bar/focus limitations. |
| Hull/shield blue becoming red (146) | `FUN_005e4110` / `FUN_005e7030`; shared StatusBar present, four selected-damage cells pass. |

Acceptance items 1 (complete inventory) and 2 (ownership/obsolete-helper audit)
are checked. Items 3 (clones), 4 (mutation completeness) and 5 (readiness and
all interface journeys) stay unchecked. The bead stays **in progress** with
these exact blockers. Audit feature passes, strict interface cell counts,
release readiness and the protected Pages release are unchanged.

Final evidence checks pass: all 3,585 manifest entries match size and SHA-256,
the audit JSON parses, changed Markdown links resolve, and
`python3 scripts/task-index.py --check` succeeds. Only this bead differs in
the task database export. `git diff --check -- . ':!tasks/acceptance.md'`
passes; the full `git diff --check` exits 2 on the task generator's two
Markdown hard-break spaces at `tasks/acceptance.md:12` and `:13`. Generated
output is preserved. These checks precede evidence staging; the generated WASM
is excluded from the evidence commit.
