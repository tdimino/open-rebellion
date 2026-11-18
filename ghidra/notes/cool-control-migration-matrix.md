---
title: "Cool Control Consumer and Migration Matrix"
description: "Original Cool* control evidence, every current production consumer, renderer object boundaries, migration beads, art ownership, exclusions, and parity journeys"
category: "ghidra"
created: 2026-11-15
updated: 2026-11-17
---

# Cool Control Consumer and Migration Matrix

This is the implementation inventory for
`or-ep-uniform-ui-controls-gru5.1`. It applies the recovered contracts in
[`cool-control-contracts.md`](cool-control-contracts.md) to the production UI
under `crates/rebellion-render/src`. The inventory follows original class
identity and behavior, not visual resemblance or clone count. The relevant
executable is the owned `REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.

## Boundary and component contracts

`rebellion-windowing` continues to own top-level `WindowId` identity,
ownership, presentation order, focus leases, modal eligibility, temporary
capture authority, one routing snapshot per physical event, and final routing.
It does not gain Win32 handles, `SC_*` flags, pixels, bitmap ids, rows, or game
commands. A renderer control receives only the event stream permitted for its
`WindowId`/`ControlId`, owns control-local state and geometry, and returns a
typed value for its feature module to interpret.

The shared renderer API is composition-based rather than a port of the native
inheritance tree:

| Object | Persistent state | Typed outcome; never a domain callback |
|---|---|---|
| `StrobeButton` / `Checkbox` | armed/captured, selected/toggled, disabled, repeat deadline | `Pressed`, `Released`, `Clicked`, `Repeated`, `Toggled(bool)`, `Cancelled` |
| `TabControl<T>` | selected and focused stable tab id | `Selected(T)` and focus traversal; each tab composes `StrobeButton` |
| `DragList<T>` | selection, first row, click timing, drag origin/capture, composed `ScrollBar` state | `SelectionChanged`, `Activated`, `ContextSelected`, `DragStarted`, `Dropped { item, point }`, `Cancelled` |
| `SelectionBox<T>` | open popup, highlighted/current row, placement, composed `DragList` | `Opened`, `Closed`, `Selected(T)`, `Dismissed`; dismissal consumes its event |
| `StringField` | text, selection/caret, overwrite, accepted-character policy and focus | `Changed`, `Submitted`, `Cancelled`, `FocusChanged`, `UnhandledKey` |
| `ScrollBar` / `Slider` | range/value, pressed part, thumb drag, repeat | `ValueChanged`, `Committed`, `Cancelled` |
| `StatusBar` | current/range, colors and optional tick interval | paint-only; no action outcome |
| `TooltipState` | registered target, hover deadline and visible target | `Shown`/`Hidden` only if a parent needs observation; text and command meaning stay outside |
| title drag adapter | live grab and feature-supplied aperture bounds | `Moved` and `Completed { position }`; manager capture remains authoritative |

Original notifications `0x29a` (outside list drop), `0x29b` (selection) and
`0x309` (activation/double click), StringField `0x407` (Enter) and `0x408`
(type-ahead), and owner command ids are evidence for typed outcomes. They are
not public Rust message numbers.

## Button and checkbox consumers

All rows derive from `CoolStrobeButton` constructor `FUN_00602150`, WndProc
`FUN_006028c0`, and painter `FUN_00602d30`, except rows explicitly identified
as Checkbox (`FUN_00603870`, WndProc `0x006039a0`).

| Production consumer | Current Rust call site and owned behavior | Original feature evidence | Shared target / bead | Art | Both-faction journey |
|---|---|---|---|---|---|
| New-game/main-menu selectors and actions | `main_menu.rs`: `MainMenuControl`, `texture_for`, `activate`, `draw_main_menu`; duplicates held/selected texture choice, exact resource hit, keyboard focus and command mapping | `FUN_00405560` constructs StrobeButtons and Checkbox variants | `StrobeButton` plus `Checkbox`; `.2`. Feature keeps difficulty, size, game type and launch actions | COMMON/STRATEGY selection art; side emblems differ and selector art is shared where the constructor says so | `game-options-smoke.mjs` traverses the menu for Alliance and Empire; final adoption must add/retain direct selector assertions |
| Strategic cockpit controls and Message Index rail lights | `cockpit.rs`: `strategic_primary_controls`, `draw_control`, `update_control_capture`, `draw_message_index_rail`; rail unread state currently chooses lit art separately | `FUN_00427270` constructs StrobeButtons for commands `0x12d..0x134` and Checkbox variants for `0x136..0x13e` | `StrobeButton`; rail `Checkbox` composition; `.2` | explicit Alliance/Empire STRATEGY tables | `run.mjs --all`, `fleet-finder.mjs`, `audit-batch.mjs`, `message-index-shell.mjs` |
| Mission Dialog chrome, tabs, arrows and actions | `mission_dialog.rs`: free `button`, page loop and action buttons | `FUN_0046a9c0`, plus common constructor/WndProc | `StrobeButton`; tabs later compose it; `.2`, `.3` | Alliance/Empire title, tab and header resources | `mission-dialog.mjs` |
| Build Selection page arrows and confirm/cancel | `build_selection.rs` calls `mission_dialog::button` and owns available/pressed policy | `FUN_00437f80`, `FUN_00455060` | `StrobeButton`; `.2` | side title plus mostly shared control resources | `build-selection.mjs` |
| Confirmed Move | `move_confirmation.rs::bitmap_button` wraps `mission_dialog::button` | `FUN_0044f180` constructs the two StrobeButtons | `StrobeButton`; `.2` | shared STRATEGY art | `fleet-move.mjs` |
| Fleet, Personnel and Troop Finders | `finder_common.rs::FinderCanvas::button`, `fleet_finder.rs`, `personnel_finder.rs`, `troop_finder.rs`; feature code owns display/close and view meaning | `FUN_00460270`, `FUN_00461960`, `FUN_004637a0`, `FUN_0046ce40` | `StrobeButton`; `.2`; tabs `.3` | side-dependent shell/rail and button sets | `fleet-finder.mjs`, `audit-batch.mjs` |
| Status and Operation Report | `status_window.rs` and `operation_report.rs` import `mission_dialog::button`; report also paints view/tab buttons locally | `FUN_00443130`, `FUN_0044c630` | `StrobeButton`; `.2`; report tabs `.3` | explicit side shell/rail art; some action art shared | `status-window.mjs`, `orbital-strike.mjs` |
| Manufacturing/System, Fleet, Defenses and Missions windows | local bitmap selection and exact-hit loops in `system_window.rs`, `fleet_window.rs`, `defenses_window.rs`, `missions_window.rs` | `FUN_00455060`, `FUN_004a4b10`, `FUN_004a8790`, `FUN_0049f540` | `StrobeButton`; `.2`; authentic strips `.3` | System/Defenses tabs are partly shared; Fleet and Missions have side offsets/sets | `sector-quadrants.mjs`, `fleet-window.mjs` |
| Encyclopedia and Message Index | local category/rail/action control loops in `encyclopedia.rs` and `message_index.rs` | `FUN_0045ddc0`, `FUN_004665f0` | `StrobeButton`; `.2`; strips `.3` | separate Alliance/Empire shell and controls where recovered | `encyclopedia-index-shell.mjs`, `encyclopedia-canonical-surface.mjs`, `message-index-shell.mjs` |
| Game Options buttons and tactical toggles | `game_options.rs`: `toggle_resource`, `toggle_tactical`, local held state and action hit tests | `FUN_00407180` creates these with `FUN_00602150`, not Checkbox | `StrobeButton`; `.2`. Keep option meaning in `game_options.rs` | COMMON art; availability differs between main-menu and live tactical origins | `game-options-smoke.mjs`; tactical availability in `run.mjs --all --scenario=production-battle-options-presentation` |
| Tactical strips, selected-unit/command panels, Battle Alert and Battle Results | `tactical_view.rs`: `draw_original_button`, assignment buttons, task-force/fighter strips, `BattleAlertTab`, `BattleResultTab`, options/results actions | strip constructors `FUN_0059bd30` through `FUN_0059c650`, panels `FUN_005c4ed0`, `FUN_005dcc70`, `FUN_005df110`, `FUN_005e1550`, `FUN_005e4110`, `FUN_005e7030`, Alert `FUN_0044f860`, Results `FUN_0046f360` | `StrobeButton`; `.2`; Alert/Results strips `.3` | TACTICAL art can vary by side/panel; Alert/Results STRATEGY shell and rails are faction-specific | `run.mjs --all` scenarios `production-battle-alert-entry`, `production-battle-results-presentation`, `production-battle-options-presentation`, command/selection presentations |

The only confirmed Checkbox constructor call sites are `FUN_00405560` and
`FUN_00427270`. Game Options toggles and tactical strip buttons may look like
checkboxes, but their constructors are StrobeButton and must not be assigned
Checkbox behavior by appearance.

## Authentic tab strips

Every row below is a `CoolTabControl` (`FUN_0060d590`, WndProc
`FUN_0060d860`) whose children are StrobeButtons. The target for all rows is
`TabControl<T>` in `.3`; the feature enum, content query, resource mapping and
domain action remain in the named module.

| Surface / current state owner | Original constructor evidence | Art and special policy | Both-faction journey |
|---|---|---|---|
| Operation Report: `operation_report.rs::ReportTab` and force sub-tabs | `FUN_0044c630` | faction rail and per-view buttons; empty/disabled policy remains feature input | `orbital-strike.mjs` |
| Battle Alert: `tactical_view.rs::BattleAlertTab` | `FUN_0044f860` | faction-authored four-tab strip | `run.mjs --all --scenario=production-battle-alert-entry` |
| Manufacturing/System: `system_window.rs::SystemWindowTab` / `manufacturing_window.rs::FacilityPage` | `FUN_00455060` | six page buttons; empty facility pages have recovered art | `sector-quadrants.mjs`, `build-selection.mjs` |
| Encyclopedia: `encyclopedia.rs::EncyclopediaTab` and category strip | `FUN_0045ddc0` | Alliance/Empire category art | `encyclopedia-index-shell.mjs`, `encyclopedia-canonical-surface.mjs` |
| Fleet/Ship Finder: `fleet_finder.rs::FinderTab` and mode/view strips | `FUN_00460270`, `FUN_00461960` | side tabs plus fleet/ship view buttons | `fleet-finder.mjs` |
| Personnel Finder: `personnel_finder.rs::PersonnelTab` | `FUN_004637a0` | side tabs and Characters/SpecForces view | `audit-batch.mjs` |
| Message Index: category/view state in `message_index.rs` | `FUN_004665f0` | faction shell/rail and category state | `message-index-shell.mjs`, `audit-batch.mjs` |
| Mission Dialog: `mission_dialog.rs::MissionDialogPage` | `FUN_0046a9c0` | two faction-specific tabs | `mission-dialog.mjs` |
| Troop Finder: `troop_finder.rs::TroopTab` | `FUN_0046ce40` | side tabs | `audit-batch.mjs` |
| Battle Results: `tactical_view.rs::BattleResultTab` | `FUN_0046f360` | four faction-authored force/result tabs | `run.mjs --all --scenario=production-battle-results-presentation` |
| Missions window: `missions_window.rs::MissionsTab` | `FUN_0049f540` | Agents/Decoys art follows mission side | `sector-quadrants.mjs` |
| Fleet window: `fleet_window.rs::FleetWindowTab` | `FUN_004a4b10` | own/other strips, four tabs each, content availability | `fleet-window.mjs` |
| System Defenses: `defenses_window.rs::DefensesPage` | `FUN_004a8790` | five tabs; selected, populated and empty art differ | `sector-quadrants.mjs` |

The main-menu selector row, tactical order buttons, and the Build Selection
class popup are not TabControl merely because they form a visual row.

### TabControl migration record (2026-11-17)

`or-ep-uniform-ui-controls-gru5.3` introduced
`controls/tab_control.rs::{TabSpec, TabControlState, TabControlOutput,
TabControl}`. The object composes `StrobeButton`, applies exact half-open hit
regions, selected/empty/disabled presentation, stable `ControlId` child
identity and disabled-skipping `Clamp` or source-specific `ResetToFirst`
keyboard traversal. The feature remains authoritative for its enum, resource
table, content and action.

| Matrix row | Migrated production path | Preserved feature policy |
|---|---|---|
| Operation Report | `operation_report.rs`: view rail, force filters and Left/Right stepping | `ReportView`/`ReportTab`, list refill and `FUN_004710f0` outer-boundary reset |
| Battle Alert | `tactical_view.rs`: exact tab hit and original-pixel child painting | `BattleAlertTab`, faction skin, retreat/simulate/command actions |
| Manufacturing/System | `system_window.rs`: six `SystemWindowTab` pages | facility content query and empty-page art |
| Encyclopedia | `encyclopedia.rs`: production category surface and original index strip through bitmap-mask input | category command ids, faction art, topic/search state |
| Fleet/Ship Finder | `finder_common.rs` side strip and `fleet_finder.rs` mode strip | finder row projection, fleet/ship meaning and open action |
| Personnel Finder | `finder_common.rs` side strip and `personnel_finder.rs` Characters/SpecForces strip | row projection and selected object action |
| Message Index | `message_index.rs`: category strip | selected category, direct draw mode and message filtering |
| Mission Dialog | `mission_dialog.rs`: Mission/Personnel pages | dialog page content and mission action application |
| Troop Finder | shared `finder_common.rs` side strip used by `troop_finder.rs` | troop counts, selected row and open action |
| Battle Results | `tactical_view.rs`: outer pages, inner result filters, exact hit, pixel painting and Left/Right stepping | result destination/actions, list refill and scroll reset, faction skin |
| Missions window | `missions_window.rs`: Agents/Decoys strip | mission-side art availability and member content |
| Fleet window | `fleet_window.rs`: four content pages | own/other availability, disabled art and local fleet actions |
| System Defenses | `defenses_window.rs`: five page strip | selected/populated/empty art and drag-list content |

The manual was reread after the migration, including Figs. 2.13, 2.31,
2.34, 2.37, 2.44, 2.45, 3.10/3.11, 3.24/3.27, 3.47/3.48, 3.55,
3.66, 3.70, 3.73, 3.80, 4.1 and 4.17. Those figures agree with the
feature-local grouping retained above; none justified turning the excluded
main-menu selectors, tactical orders or Build Selection popup into tabs.

Using the same 70-token commands and paths before and after `.3`, exact clones
fell from 3 to 2 and renamed clones (`--ignore-identifiers
--ignore-literals`) fell from 275 to 274 (3.2%). This comparison is against
the branch immediately before `.3`, not the older repository baseline below.

## Drag lists and scrollbars

Every row is backed by `CoolDragList` constructor `FUN_00607ea0` and WndProc
`FUN_006083c0`. `DragList<T>` is introduced in `.4` with Defenses and
Missions, then `.5` migrates the remaining rows. Popup lists are deliberately
owned by `.6`. The canonical `scroll_bar.rs::{ScrollBar, ScrollRange,
ScrollDrag}` already models the common base (`FUN_0060aa70`, WndProc
`FUN_0060aca0`, cleanup `FUN_0060b500`) and is composed rather than rewritten.

| Production list / pre-`.5` Rust owner | Original feature evidence | State/behavior to move into `DragList`; feature data retained | Art / scrollbar disposition | Journey |
|---|---|---|---|---|
| Defenses grid: `defenses_window.rs`, currently `LabeledCell` plus `ScrollBar` | `FUN_004a8790` | selected cells, grid wrapping, first row, right-select, threshold drag/drop | GOKRES minis, faction selection frame; current canonical bar | `sector-quadrants.mjs` |
| Missions and member lists: `missions_window.rs`, currently `LabeledCell` and local tab/list state | `FUN_0049f540` | two vertical lists, selection, member activation and drag; keep mission/member rendering | mission-side frames; source intentionally draws no separate bar in the current bounded list | `sector-quadrants.mjs` |
| Fleet left and contents lists: `fleet_window.rs` | `FUN_004a4b10` | first row, single/multi-select, double click, right select, ship/regiment drag/drop | mostly GOKRES minis plus side frame; source bars `0x29cb` | `fleet-window.mjs` |
| Manufacturing facility pages: `system_window.rs` / `manufacturing_window.rs` | `FUN_00455060` | page refill, grid selection, scrolling, build-class activation | side-independent facility minis; source list/bar configuration | `sector-quadrants.mjs`, `build-selection.mjs` |
| Fleet/Ship Finder: `finder_common.rs`, `fleet_finder.rs` | `FUN_00460270`, `FUN_00461960` | first row, bounded keys, wheel, prefix reveal, click/double click | side shell; text rows; source `0x299d` bar | `fleet-finder.mjs` |
| Personnel Finder: `finder_common.rs`, `personnel_finder.rs` | `FUN_004637a0` | same shared list mechanics; feature retains row projection and open target | side shell; text rows | `audit-batch.mjs` |
| Troop Finder: `finder_common.rs`, `troop_finder.rs` | `FUN_0046ce40` | same mechanics with six visible rows and five count columns | side shell; text rows | `audit-batch.mjs` |
| Message Index and display: `message_index.rs`, currently local row/click state plus two canonical `ScrollBar`s | `FUN_004665f0`, display fillers `FUN_00468fb0`, `FUN_004697b0`, `FUN_00469de0` | category rows, selection, double-click display, first row and text scrolling | faction shell/rail; `0x299d` bar already canonical | `message-index-shell.mjs`, `audit-batch.mjs` |
| Encyclopedia index/topics: `encyclopedia.rs` and `encyclopedia_surface.rs`, currently shared movement/reveal helpers | `FUN_0045ddc0`, list fill/search `FUN_00609650` | bounded key movement, prefix reveal, wheel, activation and current row | faction shell/category art; text index | `encyclopedia-index-shell.mjs --catalog`, `encyclopedia-canonical-surface.mjs` |
| Operation Report: `operation_report.rs`, currently local rows plus canonical `ScrollBar` | `FUN_0044c630` | tab refill, first row, wheel, selection/display | faction report shell; `0x29fc` bar already canonical | `orbital-strike.mjs` |
| Status rows: `status_window.rs` | `FUN_00443130` | bounded vertical row layout/selection behavior; keep family-specific row creation | faction title/buttons, keyed portrait; text rows | `status-window.mjs` |
| Mission Dialog Agents/Decoys: `mission_dialog.rs::member_list` | `FUN_0046a9c0` | click/double-click, selection, wheel and team/decoy drag meaning | faction dialog shell; member minis | `mission-dialog.mjs` |
| Battle Alert list: `tactical_view.rs`, currently local first-row state plus canonical `ScrollBar` | `FUN_0044f860` | tab refill, selection and scrolling | faction Alert shell; `0x29fc` list bar | `run.mjs --all --scenario=production-battle-alert-entry` |
| Battle Results lists: `tactical_view.rs`, currently result row/filter state plus canonical text bar | `FUN_0046f360`, filler `FUN_004714e0` | tab/category refill, first row, scroll and direct-route activation | faction Results shell; `0x299d` text bar | `run.mjs --all --scenario=production-battle-results-presentation` |

### `.5` shared-ownership record

The source migration is under verification on 2026-11-17. The family now has
one `DragListState`/`DragListSpec` implementation for selection, anchor,
first-row bounds, visible cells, wheel/key movement, double-click activation,
pointer capture and composed `ScrollBar` input/art. Feature modules retain only
their row projection, filtering, faction art and domain actions:

- `fleet_window.rs` retains fleet expansion, tab contents, Rename and move
  destinations; its persistent left/right list states also supply the exact
  scrolled row for external mission/move releases. Both lists compose source
  bar `0x29cb` from `FUN_004a4b10`.
- `system_window.rs` retains facility-page data and build actions while the
  source grid at (8, 77), 222 by 225 with 69-by-40 items uses the shared list.
- the three Finder families retain their row/detail projections and open
  destinations; the shared list owns first row, wheel, keys, prefix reveal,
  selection and activation with source bar `0x299d`.
- `message_index.rs` retains category filtering, Select All/Delete and message
  display policy; its multiple-selection rows use the shared list and source
  bar `0x299d`. The separately constructed displayed-message body remains a
  text control and keeps its canonical text `ScrollBar`.
- `encyclopedia.rs` and `encyclopedia_surface.rs` retain category, prefix
  search, topic and artwork policy. The recovered index is 350 by 160 with
  350-by-20 rows (eight complete rows), multiple selection and bar `0x299d`;
  click, wheel, key, reveal and activation all use the shared list. The topic
  article remains a different text control and keeps its text scrollbar.
- `operation_report.rs` retains report view/tab filtering and cell painting;
  the grid composes source bar `0x29fc` through the shared list.
- `status_window.rs` retains the family-specific label/value rows. Source
  constructor `FUN_00443130` proves one 208-by-204 list of 208-by-35 items,
  with the two 103-pixel text columns inside each item. RCDATA `0x299a` is not
  in the approved pack, so list state/input/geometry is shared while that
  already-documented bar-pixel omission remains.
- Mission Dialog retains team/decoy membership, move-arrow and double-click
  policy. Its two 107-by-59 member lists now use persistent shared list state,
  bounded wheel input and source bar `0x29fc`; `.6` separately owns only the
  mission-kind `SelectionBox` popup.
- Battle Alert and Battle Results retain battle-row construction, result
  filters and native painting; their lists use the shared native adapter and
  source bars `0x29fc` and `0x299d` respectively.

Dedicated `SelectionBox` popup internals belong to `.6` and are not `.5`
consumers. The Message Index body and Encyclopedia topic article are cited
text controls, not exclusions inferred merely from similar scrolling.

The final 70-token scan on the shared 2026-11-18 worktree reports 8 exact
clones (0.1%) and 294 normalized clones (3.4%). No match pairs two migrated
feature-local list implementations: the exact results are the pre-existing
strobe-button/test pair plus test and routing blocks, while normalized Build
Selection/Mission Dialog matches are the dedicated `.6` SelectionBox family.
Concurrent `.6` and `.7` additions mean these totals are not a direct numeric
comparison with the 263-clone pre-epic baseline.

The final scoped `cargo mutants` run exercised 90 mutations across the shared
hit testing, selection/reveal, capture cancellation, reconciliation and input
state machine: 86 were caught and three generated replacements did not
compile. The one initial survivor changed the recovered drop boundary from
strictly greater than 24 squared pixels to greater-than-or-equal; an exact
boundary test was added and the focused rerun caught it.

`ScrollBar` is also embedded in the two SelectionBox popups below. The source's
horizontal WndProc tests both cursor axes in `FUN_00610900`; this is a recovered
original bug. The Rust object deliberately advances on its active axis only.

## Selection boxes

`CoolSelectionBoxClass` constructor `FUN_0060bed0`, WndProc `FUN_0060c210`,
popup `FUN_0060cac0`/`FUN_0060cc50`, and popup WndProc `FUN_0060d020` prove
two production consumers:

| Consumer | Current Rust state | Shared target / bead | Art and journey |
|---|---|---|---|
| Build class selector | `build_selection.rs::BuildSelectionState` owns `list_open`, top, picked/hot row, wheel, popup geometry and outside dismissal | `SelectionBox<BuildClass>` composed from `DragList`; `.6` | shared class rows inside side-specific dialog; `build-selection.mjs` |
| Mission kind selector | `mission_dialog.rs::MissionDialogState` and its popup own parallel open/top/hot/selected state | `SelectionBox<MissionKind>`; `.6` | faction dialog and mission-kind art; `mission-dialog.mjs` |

No third `FUN_0060bed0` feature constructor exists in the retained decompiles.
The source popup scrollbar's still-unsettled `0x299a` art remains an explicit
`.6` evidence item rather than guessed configuration.

## String fields

All included rows use `CoolStringField` constructor `FUN_00604cf0` and
WndProc `FUN_00605160`; `.7` supplies one `controls::StringField` object and
`StringFieldState`. The object owns text, selection/caret, overwrite mode,
printable-character admission, optional maximum length and semantic child
focus. Parents retain empty-submit/cancel meaning, feature commands and
type-ahead/list behavior.

`FUN_00604cf0` initializes field `+0xa4` to zero, meaning unlimited. The `10`
passed by Fleet and Manufacturing rename callers is the font installed through
`FUN_00604f70`, not a ten-character limit. Among these consumers only Build
Selection calls `FUN_006050c0(field, 0x19)`, so its quantity edit has the
25-character maximum and every other migrated field remains unlimited.

| Consumer / migrated call site | Original feature evidence | Feature policy and art | Journey |
|---|---|---|---|
| Six Game Options save names: `game_options.rs::name_fields` | `FUN_00407180` | `StringField` owns each slot edit; save-slot validation/overwrite remains parent policy; COMMON styling | `game-options-smoke.mjs` both factions |
| Build Selection quantity: `build_selection.rs::quantity_field` | `FUN_00437f80`; limit setter `FUN_006050c0(..., 0x19)`; parent parse/clamp in `FUN_00438d30` | the field accepts printable input; the parent parses and clamps 1..255, validates the build and owns +/- commands | `build-selection.mjs` |
| Manufacturing-band rename: `system_window.rs::BandRename::field` through `draw_band_rename` | `FUN_00458640` | unlimited field; empty-name and Rename order policy stays in System window | `build-selection.mjs` manufacturing journey |
| Encyclopedia search: `encyclopedia.rs::EncyclopediaSurfaceState::search_field` | `FUN_0045ddc0`, list search `FUN_00609650` | unlimited field; category/filter and list movement remain Encyclopedia-owned | `encyclopedia-index-shell.mjs --catalog`, `encyclopedia-canonical-surface.mjs` |
| Fleet/Ship Finder name: `finder_common.rs::FinderUiState::name` | `FUN_00460270`, `FUN_00461960` | shared field emits changes/keys; Finder owns prefix selection, reveal and Enter/open | `fleet-finder.mjs` |
| Personnel Finder name: the same `FinderUiState::name` through `personnel_finder.rs` | `FUN_004637a0` | shared text behavior, feature row/open policy | `audit-batch.mjs` |
| Troop Finder name: the same `FinderUiState::name` through `troop_finder.rs` | `FUN_0046ce40` | shared text behavior, feature row/open policy | `audit-batch.mjs` |
| Fleet in-place rename: `fleet_window.rs::RenameEdit::field` through `draw_rename_edit` | `FUN_004ac7a0`, setters `FUN_00604f90`/`FUN_00605110` | unlimited field with `ControlId(0x203)`; empty Enter keeps edit open and the parent emits Rename | `fleet-window.mjs` |

The multiplayer/player name in `main_menu_destinations.rs` is replacement UI
with no confirmed `FUN_00604cf0` constructor and is excluded. Raw TextEdits in
`panels/command_palette.rs`, adapter probes and tests are developer/test UI and
are also excluded.

## Slider, status bar and tooltip

These families are consolidated in `.8`; current canonical objects are kept
where they already satisfy the contract. Checkbox is the canonical
`controls::Checkbox` composition over `StrobeButton`, while the scrollbar and
tooltip implementations remain in their focused source modules and are
re-exported through `controls`.

| Family and production consumer | Original evidence | Current Rust call site / proposed target | Art, exclusion and journey |
|---|---|---|---|
| Slider: Game Options music and effects volumes | `FUN_00407180` is the only feature caller of `FUN_00604570`; WndProc `0x00604b80` | canonical `controls/slider.rs::{Slider, SliderState}` owns capture, held updates, integer range and commit; `game_options.rs` supplies the two COMMON rails and applies values to `AudioVolumeState` | COMMON slider art; `game-options-smoke.mjs` both factions. `audio.rs::draw_audio_controls` is a port-only bottom-bar extension and therefore remains an egui control, not a false original Slider consumer |
| StatusBar: three Manufacturing band progress bars | subclass `FUN_004acec0` called three times by `FUN_00455060`; base `FUN_0060e150`, WndProc `FUN_0060e460` | canonical `controls/status_bar.rs::StatusBar`; `system_window.rs` retains black/light-gray colors and band values | shared colors, no side art; `build-selection.mjs` / `sector-quadrants.mjs` |
| StatusBar: sector planet energy, raw-material and support meters | `FUN_00459e30` calls `FUN_004acec0`; `FUN_0045c240`/`FUN_0045c450` configure values/ticks | canonical `controls/status_bar.rs::StatusBar`; `sector_window.rs::paint_meter` is now only the egui/color adapter | support foreground/background swaps by player side; `run.mjs --all --scenario=sector` and `sector-quadrants.mjs` |
| StatusBar: selected capital and alternate ship-style panels, hull and shield | `FUN_005e4110` and `FUN_005e7030` each create two `FUN_0060e150` bars | canonical `controls/status_bar.rs::StatusBar`; `tactical_view.rs::draw_tactical_selected_meter` is the macroquad/color adapter | fixed blue over red, not faction color; tactical selected/damage presentation scenarios in `run.mjs --all` |
| StatusBar: Death Star charge | `FUN_005c4ed0` creates one `FUN_0060e150` bar | canonical `controls/status_bar.rs::StatusBar`; `tactical_view.rs` supplies the charge value and gauge colors | fixed green over blue; `run.mjs --all --scenario=production-death-star-laser-journey` |
| Stock tooltip: cockpit counters, command controls, Message Index rail and game-speed controls | global `FUN_006008e0`; registrations `FUN_00600a40` from `FUN_00427270` and updates `FUN_00600cc0` | canonical `tooltip.rs::{TooltipTarget, TooltipState, draw_cockpit_tooltips}`, re-exported by `controls`; the app consumes that facade | faction-specific target geometry/text; `run.mjs --all`, `fleet-finder.mjs`, `audit-batch.mjs` |
| Stock tooltip: strategic window buttons/tabs (Move, Report, Alert, Manufacturing, Sector meters, Encyclopedia, Finders, Message Index, Mission Dialog, Troop, Results, Missions, Fleet and Defenses) | `FUN_00600a40` call sites in `FUN_0044f180`, `FUN_0044c630`, `FUN_0044f860`, `FUN_00455060`, `FUN_00459e30`, `FUN_0045ddc0`, `FUN_00460270`, `FUN_00461960`, `FUN_004637a0`, `FUN_004665f0`, `FUN_0046a9c0`, `FUN_0046ce40`, `FUN_0046f360`, `FUN_0049f540`, `FUN_004a4b10`, `FUN_004a8790` | canonical `tooltip.rs::{TooltipState, draw_registered_tooltips}` owns delay, click dismissal and stock painting. Encyclopedia categories, Operation Report controls, common Sector/Minimize/Close title controls and the configured System, Defenses, Fleet, Missions, Mission Dialog and Message Index tab strips register their feature-owned labels and rectangles through it. Other source-proven registrations not yet rendered by their feature have no alternate implementation to migrate and remain explicitly inventoried rather than receiving guessed text | use each feature's side-specific text/geometry; corresponding journeys in the tables above |
| Stock tooltip: tactical task-force/fighter strips and selected panels | `FUN_0059b130`, `FUN_0059b450`, `FUN_0059b550`, `FUN_0059b690`, `FUN_005dd080`, `FUN_005dd3a0`, `FUN_005e4110`, `FUN_005e7030`, `FUN_005ef240`; dynamic text via `FUN_00600cc0` | canonical state contract is `tooltip.rs::TooltipState`, but the current tactical renderer has no stock-tooltip consumer to migrate. The original updates dynamic text through `FUN_00600cc0`; until that per-target text and native geometry are recovered, adding a tactical painter would invent behavior forbidden by `.8`. This is a source-backed absent-feature exclusion, not a second implementation | faction/task-force text and panel geometry; command/selection scenarios in `run.mjs --all` once a dedicated parity bead implements the recovered registrations |

An egui layer created with `egui::Order::Tooltip` is not automatically an
original tooltip. The pop-up menus in `object_menu.rs`/`game_menu.rs`, targeting
cursor, move confirmation, finder/modal surface, message display and other
topmost overlays use that order for stacking and are excluded unless they also
have an original `FUN_00600a40`/`FUN_00600cc0` registration. Encyclopedia's
category hover and Operation Report's rail hover are included because their
feature evidence identifies tooltip text, not because of their egui layer.

### `.8` shared-ownership record

The family consolidation completed on 2026-11-18 without treating visual
similarity as original identity:

- every production scrollbar import now goes through `controls`, whose facade
  re-exports the existing `scroll_bar.rs` object, source art configurations and
  egui adapter. `DragList` and `SelectionBox` compose the same implementation;
  the recovered horizontal two-axis bug remains deliberately rejected.
- the two confirmed Checkbox constructor families continue to compose
  `controls::Checkbox` over `StrobeButton`. Game Options tactical switches are
  still StrobeButtons because `FUN_00407180` constructs them that way.
- `controls::Slider` owns the source capture/value/commit state for both Game
  Options volume controls. The unrelated port bottom-bar sliders remain normal
  egui controls because `FUN_00407180` is the only proved feature caller.
- one `controls::StatusBar` now computes the exact whole-pixel fill and optional
  tick positions for Manufacturing, Sector, selected-ship hull/shield and the
  Death Star charge gauge. Feature modules still own values, colors, art and
  egui/macroquad painting. The source's tick pass is independent of a zero
  fill; the new shared test caught and removed the Sector adapter's early-return
  omission.
- `TooltipState` remains the one hover-delay/click-dismiss state machine and
  the stock colors/font/border now have one painter. Cockpit, Encyclopedia,
  Operation Report, common strategic title controls and source-labelled
  strategic tab strips route implemented original tooltip behavior through
  the `controls` facade. Proved registrations whose feature is still absent
  remain explicit source-backed exclusions; neither `.8` nor the verification-
  only `.9` invents their dynamic text or native geometry.

The manual was read again after the migration: pp. 22 and 62 promise delayed
descriptions for controls; pp. 75-76 name and describe dragging both Game
Options volume sliders; p. 83 identifies each Manufacturing progress bar;
pp. 84-85 identify Sector energy/raw-material indicators; and p. 146 says
selected-ship hull and shield begin solid blue and turn red with damage. These
claims agree with the cited executable paths and the retained feature colors.

The scoped mutation run covered all 52 generated replacements in the new
`Slider` and `StatusBar` kernels. Its first pass caught 44 and rejected five
that did not compile. The three survivors exposed missing fixed-range,
zero-travel, shifted-minimum and zero-tick tests; after adding those boundaries,
the exact three-mutant rerun caught all three. No non-equivalent survivor
remains.

Local acceptance on 2026-11-18 passed `make fmt`, warning-free `make build`,
`make check`, and `bash scripts/build-interface-test-wasm.sh`; the latter
validated all 38 fixture scenarios / 152 cells and proved zero fixture tokens
in production. The muted `sector` and `popular-support` catalog runs passed
all four faction/viewport cells, `sector-quadrants.mjs --no-build` passed all
12 Alliance/Empire cases, and the strengthened `game-options-smoke.mjs`
passed both factions after directly dragging both original volume sliders and
observing their moved bitmap handles. The selected-damage functional meter
assertions passed every cell across two runs, including exact 40% and 25%
whole-pixel fills, but the aggregate commands remained red only on the generic
paused-render p95 threshold; the Death Star gauge likewise passed both native-
size cells while both letterboxed cells hit that threshold. That tactical
four-cell performance gate remains for Tom's Mini. Build Selection's separate
modal-stabilization failure is owned by the concurrently migrated StringField,
not any StatusBar surface; the passing Sector/System journey is the `.8`
Manufacturing evidence.

## CoolGameWindow title dragging

`FUN_00606380`, `FUN_00606ee0`, `FUN_006071e0` and `FUN_00607010` establish
live mode 1, capture and release-time full-window clamping. The production
renderer has four live consumers: `system_window.rs`, `fleet_window.rs`,
`defenses_window.rs` and `missions_window.rs`, sharing
`strategic_window_common::{start_title_drag, record_title_drag,
update_title_drag}` and `system_window::{TitleDrag, title_drag_step}`. `.10`
adds a renderer-side, feature-bounded release clamp and returns a typed final
position while `rebellion-windowing` retains capture/routing authority.
`fleet-move.mjs`, `fleet-window.mjs` and `sector-quadrants.mjs` are the existing
both-faction drag journeys.

No constructor in the reviewed current feature set was proved to change
`+0xf4` from its `FUN_00606380` default of 1. Outline mode 2 is therefore a
bounded exclusion: `.10` must rescan constructors, but must not add an unused
XOR-preview API without a source-backed current consumer.

## Explicit exclusions

- `rebellion-render/src/panels/**`, command-palette controls and diagnostics
  are developer UI. Their normal egui buttons, checkboxes, sliders and
  TextEdits are not original-game consumers.
- `main_menu_destinations.rs` is replacement multiplayer/session UI. Its
  player-name TextEdit has no proven CoolStringField identity.
- Object/game pop-up menus, the targeting cursor and modal/window surfaces
  may occupy `egui::Order::Tooltip`, but layer order does not make them stock
  tooltips.
- `rebellion-windowing` adapter probes and all `#[cfg(test)]` controls are test
  consumers, not migration rows.
- `SC_MOVE`, `SC_SIZE`, minimize/maximize/restore filtering is an intentional
  platform-adapter exclusion: canvas windows expose typed policy instead.
- GID palette remapping and sprite draw mode 5 are graphics primitives, not
  controls. They remain in `or-port-gid-palette-darkening-ms1p` and
  `or-port-sprite-draw-mode-5-yv8o`.

## Reproducible inventory and clone baseline

The constructor inventory can be repeated from the repository root:

```sh
rg -n 'FUN_00602150|FUN_00603870|FUN_00604570|FUN_00604cf0|FUN_00607ea0|FUN_0060bed0|FUN_0060d590|FUN_0060e150|FUN_006008e0|FUN_00600a40|FUN_00600cc0' \
  ghidra/notes/FUN_*.c ghidra/notes/*.md

rg -n 'button\(|TextEdit|Slider::|ScrollBar|TooltipState|LabeledCell|enum .*Tab|enum .*Page|paint_meter|draw_tactical_selected_meter|TitleDrag' \
  crates/rebellion-render/src --glob '*.rs'

rg -n 'egui::Order::Tooltip|ui\.checkbox|panels/command_palette|windowing_adapter_probe' \
  crates/rebellion-render/src --glob '*.rs'
```

With `jscpd 5.4.3`, the 70-token baseline on 2026-11-15 was:

```sh
jscpd --reporters ai --format rust --min-tokens 70 --no-tips \
  crates/rebellion-render/src crates/rebellion-windowing/src
# 0 exact clones; 0.0%

jscpd --reporters ai --format rust --min-tokens 70 \
  --ignore-identifiers --ignore-literals --no-tips \
  crates/rebellion-render/src crates/rebellion-windowing/src
# 263 renamed clones; 3.2%
```

The normalized results are architectural leads, not a mandate to abstract
every match. Examples among the control consumers include parallel
Build Selection/Mission Dialog popup logic and Defenses/Missions list/tab
loops. Feature lifecycle, faction resource tables and domain action mapping
remain intentionally separate even when normalization makes them look alike.

## Acceptance crosswalk

All named strategic harnesses above explicitly iterate Alliance and Empire.
The tactical catalog declares both factions and the cited `run.mjs --all`
scenarios provide a fixture code for each. These are implementation journeys,
not original A0 acceptance by themselves; the tactical audit still requires
its separate A0 evidence. Component beads must keep the relevant journey's
actual faction/viewport coverage and may not infer two-faction coverage from a
single default run.

The remaining API uncertainty is bounded rather than silently resolved:
Checkbox has two confirmed feature constructors; SelectionBox has two;
Slider has one feature with two instances; outline title drag has no proved
current consumer; horizontal ScrollBar's two-axis test is a rejected original
bug; tooltip adoption requires an original registration, not merely topmost
painting. These bounds are the input to `.2` through `.10` and the final
adoption audit `.9`.
