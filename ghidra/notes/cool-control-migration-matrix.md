---
title: "Cool Control Consumer and Migration Matrix"
description: "Original Cool* control evidence, every current production consumer, renderer object boundaries, migration beads, art ownership, exclusions, and parity journeys"
category: "ghidra"
created: 2026-11-15
updated: 2026-11-15
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

## Drag lists and scrollbars

Every row is backed by `CoolDragList` constructor `FUN_00607ea0` and WndProc
`FUN_006083c0`. `DragList<T>` is introduced in `.4` with Defenses and
Missions, then `.5` migrates the remaining rows. Popup lists are deliberately
owned by `.6`. The canonical `scroll_bar.rs::{ScrollBar, ScrollRange,
ScrollDrag}` already models the common base (`FUN_0060aa70`, WndProc
`FUN_0060aca0`, cleanup `FUN_0060b500`) and is composed rather than rewritten.

| Production list / current Rust owner | Original feature evidence | State/behavior to move into `DragList`; feature data retained | Art / scrollbar disposition | Journey |
|---|---|---|---|---|
| Defenses grid: `defenses_window.rs`, currently `LabeledCell` plus `ScrollBar` | `FUN_004a8790` | selected cells, grid wrapping, first row, right-select, threshold drag/drop | GOKRES minis, faction selection frame; current canonical bar | `sector-quadrants.mjs` |
| Missions and member lists: `missions_window.rs`, currently `LabeledCell` and local tab/list state | `FUN_0049f540` | two vertical lists, selection, member activation and drag; keep mission/member rendering | mission-side frames; source intentionally draws no separate bar in the current bounded list | `sector-quadrants.mjs` |
| Fleet left and contents lists: `fleet_window.rs` | `FUN_004a4b10` | first row, single/multi-select, double click, right select, ship/regiment drag/drop | mostly GOKRES minis plus side frame; source bars `0x29fc` | `fleet-window.mjs` |
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
WndProc `FUN_00605160`; `.7` supplies one `StringField` object. Parents retain
submit/cancel meaning and type-ahead/list behavior.

| Consumer / current call site | Original feature evidence | Feature policy and art | Journey |
|---|---|---|---|
| Six Game Options save names: `game_options.rs` raw `egui::TextEdit` | `FUN_00407180` | save-slot validation/overwrite remains parent policy; COMMON styling | `game-options-smoke.mjs` both factions |
| Build Selection quantity: manual numeric text/edit state in `build_selection.rs` | `FUN_00437f80` | parent clamps quantity and build validity | `build-selection.mjs` |
| Manufacturing-band rename: `system_window.rs::draw_band_rename` raw `TextEdit`/focus state | `FUN_00458640` | empty-name and Rename order policy stays in System window | `build-selection.mjs` manufacturing journey |
| Encyclopedia search: `encyclopedia.rs` raw `TextEdit` and prefix/filter state | `FUN_0045ddc0`, list search `FUN_00609650` | category/filter meaning remains Encyclopedia-owned | `encyclopedia-index-shell.mjs --catalog`, `encyclopedia-canonical-surface.mjs` |
| Fleet/Ship Finder name: `finder_common.rs` raw `TextEdit` and type-ahead state | `FUN_00460270`, `FUN_00461960` | finder owns prefix selection and Enter/open | `fleet-finder.mjs` |
| Personnel Finder name: `finder_common.rs` through `personnel_finder.rs` | `FUN_004637a0` | shared text behavior, feature row/open policy | `audit-batch.mjs` |
| Troop Finder name: `finder_common.rs` through `troop_finder.rs` | `FUN_0046ce40` | shared text behavior, feature row/open policy | `audit-batch.mjs` |
| Fleet in-place rename: `fleet_window.rs::draw_rename_field` raw `TextEdit` and `ControlId(0x203)` focus | `FUN_004ac7a0`, setters `FUN_00604f90`/`FUN_00605110` | empty Enter keeps edit open; parent emits Rename | `fleet-window.mjs` |

The multiplayer/player name in `main_menu_destinations.rs` is replacement UI
with no confirmed `FUN_00604cf0` constructor and is excluded. Raw TextEdits in
`panels/command_palette.rs`, adapter probes and tests are developer/test UI and
are also excluded.

## Slider, status bar and tooltip

These families are consolidated in `.8`; current canonical objects are kept
where they already satisfy the contract.

| Family and production consumer | Original evidence | Current Rust call site / proposed target | Art, exclusion and journey |
|---|---|---|---|
| Slider: Game Options music and effects volumes | `FUN_00407180` is the only feature caller of `FUN_00604570`; WndProc `0x00604b80` | `audio.rs` has two raw `egui::Slider`s; replace with one source-shaped `Slider` object while `AudioSettings` keeps volume application | COMMON slider art; `game-options-smoke.mjs` both factions |
| StatusBar: three Manufacturing band progress bars | subclass `FUN_004acec0` called three times by `FUN_00455060`; base `FUN_0060e150`, WndProc `FUN_0060e460` | `system_window.rs` paints each black/gray fraction locally; use `StatusBar` | shared colors, no side art; `build-selection.mjs` / `sector-quadrants.mjs` |
| StatusBar: sector planet energy, raw-material and support meters | `FUN_00459e30` calls `FUN_004acec0`; `FUN_0045c240`/`FUN_0045c450` configure values/ticks | `sector_window.rs::paint_meter` is a second complete implementation; use `StatusBar` with optional ticks | support foreground/background swaps by player side; `run.mjs --all --scenario=sector` and `sector-quadrants.mjs` |
| StatusBar: selected capital and alternate ship-style panels, hull and shield | `FUN_005e4110` and `FUN_005e7030` each create two `FUN_0060e150` bars | `tactical_view.rs::draw_tactical_selected_meter`; use `StatusBar` | fixed blue over red, not faction color; tactical selected/damage presentation scenarios in `run.mjs --all` |
| StatusBar: Death Star charge | `FUN_005c4ed0` creates one `FUN_0060e150` bar | `tactical_view.rs` charge-gauge painter/tests; use `StatusBar` | fixed green over blue; `run.mjs --all --scenario=production-death-star-laser-journey` |
| Stock tooltip: cockpit counters, command controls, Message Index rail and game-speed controls | global `FUN_006008e0`; registrations `FUN_00600a40` from `FUN_00427270` and updates `FUN_00600cc0` | keep `tooltip.rs::{TooltipTarget, TooltipState, draw_cockpit_tooltips}` canonical and extend registrations through configuration | faction-specific target geometry/text; `run.mjs --all`, `fleet-finder.mjs`, `audit-batch.mjs` |
| Stock tooltip: strategic window buttons/tabs (Move, Report, Alert, Manufacturing, Sector meters, Encyclopedia, Finders, Message Index, Mission Dialog, Troop, Results, Missions, Fleet and Defenses) | `FUN_00600a40` call sites in `FUN_0044f180`, `FUN_0044c630`, `FUN_0044f860`, `FUN_00455060`, `FUN_00459e30`, `FUN_0045ddc0`, `FUN_00460270`, `FUN_00461960`, `FUN_004637a0`, `FUN_004665f0`, `FUN_0046a9c0`, `FUN_0046ce40`, `FUN_0046f360`, `FUN_0049f540`, `FUN_004a4b10`, `FUN_004a8790` | `operation_report.rs` and `encyclopedia.rs` have feature-local hover painters; most other targets are absent. Register them through canonical `TooltipState`; keep text/resource mapping in each feature | use each feature's side-specific text/geometry; corresponding journeys in the tables above |
| Stock tooltip: tactical task-force/fighter strips and selected panels | `FUN_0059b130`, `FUN_0059b450`, `FUN_0059b550`, `FUN_0059b690`, `FUN_005dd080`, `FUN_005dd3a0`, `FUN_005e4110`, `FUN_005e7030`, `FUN_005ef240`; dynamic text via `FUN_00600cc0` | configure canonical `TooltipState` from `tactical_view.rs`; strip assignment text stays tactical-owned | faction/task-force text and panel geometry; command/selection scenarios in `run.mjs --all` |

An egui layer created with `egui::Order::Tooltip` is not automatically an
original tooltip. The pop-up menus in `object_menu.rs`/`game_menu.rs`, targeting
cursor, move confirmation, finder/modal surface, message display and other
topmost overlays use that order for stacking and are excluded unless they also
have an original `FUN_00600a40`/`FUN_00600cc0` registration. Encyclopedia's
category hover and Operation Report's rail hover are included because their
feature evidence identifies tooltip text, not because of their egui layer.

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
