---
title: "Shared Test Fixtures"
description: "How to reuse and extend test setup without recreating cross-file fixtures"
category: agent-docs
created: 2026-11-11
updated: 2026-11-12
tags: [testing, fixtures, refactoring]
---

# Shared Test Fixtures

Test setup is part of the codebase's maintenance surface. Repeating even a
small world builder, layout, input event, frame sequence, or routing assertion
across files makes a later feature cost more to understand and change. Reuse a
shared fixture when two tests need the same concept and semantics.

## Find the fixture before writing setup

Search the target crate and its tests first:

```bash
rg -n "test_support|fixture|RawInput|CockpitLayout|GameWorld::default" crates/<crate>
rg --files crates/<crate> | rg 'test_support|tests/common|fixture'
```

For renderer unit tests, start with
`crates/rebellion-render/src/test_support.rs`. It currently owns the canonical
Finder and full-canvas layouts, minimal explored systems and worlds, keyboard
and pointer events, click/drag frame sequences, deterministic egui frame
runners, manager-order assertions, fleet/character/mission insertion, uniform
BMP construction, and the modal-window routing contract.

Import helpers explicitly so the setup remains visible at the test site:

```rust
use crate::test_support::{
    alliance_system, click_frames, finder_layout, run_action_frames, tap_key,
};
```

Use `RenderRun<Action>` with `run_render_frames` when a window test needs the
actions plus final-frame text and bitmap traces. Use
`run_routed_render_frames` for manager-owned pointer input: it rebuilds the
frozen snapshot and normalized pointer records for every frame. Finder focus
tests that must retain one egui context can use `run_action_frames`.

For lower-level input construction, prefer `pointer_raw_input` over rewriting
placeholder pointer-button coordinates in each module. Use
`for_each_routed_pointer_frame` only when the test needs to inspect or mutate
the router between drawing steps.

Use `key_frame`, `tap_key`, `click_frames`, `immediate_click_frames`,
`right_click_frames`, and `title_drag_frames` for input sequences. Use
`raw_input`, `full_raw_input`, or `timed_raw_input` instead of repeating
`egui::RawInput` screen/time setup. Use `bitmap_fixture` for a uniform BMP and
then overwrite only the palette entries or pixels the test observes. Use
`add_standalone_system` only for row-level tests that intentionally need no
sector relation; ordinary renderer worlds should use `add_system`. Use
`add_special_force`, `assert_finder_selection`, `assert_finder_bounds`,
`assert_finder_modal_routing`, and `assert_finder_prefix_choices` for the
Finder contracts already shared by multiple families. These helpers
deliberately keep the window-specific action assertion at the call site.

Selection reveal is not a Finder or whole-window contract. The original
`CoolDragList` control implements it in `FUN_00609ed0`, so production lists
must use `crate::list_control::reveal_row` instead of repeating the
first-visible-row arithmetic in a Finder, Encyclopedia, or detail window.

For manager-owned singleton fixtures, use `open_singleton_window`,
`assert_managed_close_receipt`, and `assert_minimized_reopen` instead of
reconstructing policy specs or manager receipt sequences. Strategic save tests
can use `assert_saved_windows_restore`; row assertions can use `status_pairs`.

Finder modules should define their ordinary manager-owned constructor once
with `define_managed_finder_fixture!`. The generated helper uses the family's
`ManagedFinderFixture` implementation, so Personnel, Troop, and Fleet tests do
not each reproduce singleton preparation, manager publication, or commit
steps. Production Finder state should use `define_finder_interface!` and
`side_finder_snapshot!` instead of reproducing the same lifecycle or
side/count presentation adapters under family-specific names. Use
`assert_focused_duplicate_finder_ui` for duplicate-open coverage,
`assert_stale_finder_publications_rejected` for stale receipt coverage,
`assert_finder_count_colours` for the shared side palette, and
`assert_finder_list_focus_ignores_text` for the common list-focus contract.
Keep the family-specific tab, view, row, and selection assertions in the
calling test; only the shared lifecycle contract belongs in these helpers.

## Choose functions, traits, and macros deliberately

Prefer an ordinary function when the repeated code is behavior: geometry,
selection changes, content reporting, draw outcomes, input routing, and test
assertions should remain typed and directly callable. Use a trait when the
same fixture contract applies to several state types; `ManagedFinderFixture`
is the canonical example.

Use a macro only when Rust's privacy or field-shape rules would otherwise make
each module repeat the same structural implementation. The renderer's
`impl_finder_state_shell!`, `impl_system_keyed_window_routing!`,
`impl_system_content_reports!`, `impl_saved_system_windows!`, and
`commit_prepared_detail!` macros deliberately expand inside the owning module
so private window fields remain private. Keep such macros narrow: pass the
family-specific policy, record construction, or action type explicitly, and
delegate actual behavior to a shared function. A renamed-token detector match
between different window actions or draw signatures is not by itself a reason
to put behavior into a macro.

Other fixture homes are intentionally narrower:

- `rebellion_core::test_support` owns minimal `system`, `system_at`,
  `sector`, `insert_system`, `insert_system_in_sector`, `world_with_system`,
  and `world_with_systems` constructors; canonical GNPRTB/SDPRTB and
  Diplomacy mission records; one-ship and multi-ship fleet insertion;
  completed Death Star ships and fleets; Cruiser/Wing report classes; queued
  capital-ship builds; ground-regiment insertion; and the
  completed-fleet-arrival assertion. It is available to downstream tests
  through the crate's `test-support` feature.
- `rebellion_data::test_support` owns canonical Encyclopedia catalogs,
  entries, indexed and RGB BMP bytes, the shared indexed artwork map,
  complete save snapshots, and TEXTSTRA RCDATA template tables.
  `text_templates` accepts readable placeholders, while
  `assault_report_template_entries` supplies the entries shared by assault and
  strike tests. Enable its `test-support` feature only in a dev-dependency or
  acceptance-only feature.
- `crate::test_support` in `rebellion-app` owns the shared order world and the
  four-resource uniform Encyclopedia artwork map.
- `crates/rebellion-data/tests/common/mod.rs` owns integration-test path
  discovery; `crates/rebellion-windowing/tests/common/mod.rs` owns the shared
  window-spec/open helpers.
- `rebellion_render::test_support` is normally unit-test-only. Downstream
  crate tests may enable `rebellion-render/test-support`; production defaults
  must leave that feature disabled. Its `scrap_confirmation` fixture is the
  canonical one-regiment modal payload for renderer and app tests.

## Where fixture code belongs

- Keep setup local when it expresses behavior unique to one test module.
- Move setup used by two or more source files into the crate's
  `src/test_support.rs`, behind `#[cfg(test)]` or a narrowly scoped
  `test-support` feature in `lib.rs`.
- For integration tests, use `tests/common/mod.rs` or a dedicated support
  library so Cargo does not treat the helper as another integration-test
  binary.
- Keep browser/interface fixtures behind their existing
  `interface-test-fixtures` feature. A unit-test helper must not add a new
  production behavior path.
- Prefer a small typed builder or named function over a large mutable fixture
  with hidden defaults.

## Fixture design rules

1. Name the domain concept, not the test that first needed it: `add_system`,
   `scaled_full_layout`, and `title_drag_frames` are reusable; `setup_test_7`
   is not.
2. Make important state explicit in parameters. Faction, control, identity,
   and input phase should not be surprising defaults.
3. Return stable keys and identities that tests need; do not make callers
   rediscover inserted entities.
4. Keep assertions that define a shared contract in a clearly named assertion
   helper. Keep feature-specific assertions in the test so its behavior remains
   readable.
5. Do not weaken or generalize assertions merely to share code. If two setups
   differ semantically, keep separate helpers or build both from a smaller
   common primitive.
6. Add a doc comment describing the fixture's invariants and why its defaults
   are canonical.
7. Extend the nearest shared fixture before introducing another local helper.
   A second copy is the signal to extract the common concept in the same
   change, even when the repeated setup is only a few lines.

For simulation-state setup, start with `SimulationStates::new` and change only
the states the scenario observes. For a complete serializable test snapshot,
use `rebellion_data::test_support::save_state`. Production replay and playtest
code should assemble snapshots through `SaveState::from_simulation`; a raw
`SaveState { ... }` field list should be reserved for code that truly maps a
different runtime representation.

The strategic-window helpers already cover title-button press art, keyed close
and raise behavior, manager visibility transitions, minimize/reopen identity,
saved-window restore loops, exact hit routing, singleton publication rejection,
close-receipt teardown, routed press/release/drag frame construction, and
single-phase manager pointer admission. Use `restore_shown_detail_system!` for
the private-field portion of a saved detail restore. Extend those helpers when
a new window is meant to obey the same contract; do not reproduce the full
sequence in its module.

Application order tests should start from
`crate::test_support::order_world(holder, is_populated)`. Tactical tests should
start with `tactical::test_support::test_session` and its canonical
`BattleSession::baseline` shell, then override only behavior-specific ships or
session fields.

## Adding or extending a renderer fixture

1. Confirm the repeated setup has the same meaning in every caller.
2. Add the smallest common primitive to
   `crates/rebellion-render/src/test_support.rs`.
3. Replace every matching caller in the same change; avoid leaving two ways to
   construct the same canonical fixture.
4. Run `rg` again for the removed local helper or setup pattern.
5. Run `cargo fmt --all`, then the repository readiness gates:

   ```bash
   make check
   make build
   ```

`make build` must be warning-free. If a test needs a new behavior-specific
world, start with the shared minimal world/system primitive and add only the
entities the test observes.
