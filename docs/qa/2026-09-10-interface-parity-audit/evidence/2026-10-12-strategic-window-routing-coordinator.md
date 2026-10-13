# Strategic Window Routing and Coordinator Checkpoint

Date: 2026-10-12

This checkpoint combines the immutable routing kernel and the production
application coordinator in one implementation slice. It establishes the
shared authority needed by later strategic-window migrations without changing
the acceptance status of any visual interface cell.

## Implemented contract

- `RoutingSnapshot` owns a frame-stable copy of active window identity, order,
  focus, capture, modal eligibility, and presentation history.
- Pointer routing selects exactly one topmost eligible hit, capture owner,
  modal blocker, dismissible menu, or strategic root.
- Keyboard routing preserves focused-control ownership and stops unhandled
  keys at modal or transient boundaries, including exact control restoration
  after a temporary scene suspension. Content adapters explicitly invalidate
  removed controls so later restoration downgrades safely to the owner window.
- Scene membership drives both command eligibility and snapshot construction;
  inactive modals cannot block another scene.
- Content reconciliation distinguishes ready, unpaintable, absent, and
  suspended states. Missing or contradictory reports fail closed, and an
  absent modal remains a blocker until an explicit close.
- The render boundary records current-frame hit regions, stable egui child and
  control IDs, paint receipts, and per-button event-local input ownership.
- The application owns one coordinator and frame router. It freezes scene and
  application holds before input, applies queued commands after the frame, and
  delivers capture-cancellation receipts to the gesture router.
- A narrow legacy bridge prevents an active managed modal from exposing older
  raw cockpit or map paths during incremental migration.

## Verification

The implementation includes named regression coverage for the Task 4 routing
contracts and the foundational Task 5B coordinator, gesture, hold, and legacy
gate. Backend event normalization and feature-family permits remain part of
the migration slices.

| Gate | Result |
|---|---|
| `cargo test -p rebellion-windowing` | pass: 115 tests |
| `cargo test -p rebellion-render windowing --features interface-test-fixtures` | pass: 33 selected tests |
| `cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures` | pass: 8 selected tests |
| `cargo test --workspace` | pass; existing owned-asset tests remain ignored by their declared gates |
| `cargo clippy -p rebellion-windowing --all-targets -- -D warnings` | pass |
| Render/app strict Clippy | existing audited baseline findings remain; no new finding points at this slice |
| `scripts/build-wasm.sh` | pass: optimized 7.3 MiB WASM and deterministic 122,932,981-byte runtime pack |
| `scripts/package-web.sh dev` | pass: hashed 72 MiB development package |
| `node scripts/validate-interface-parity-ledgers.mjs` | pass: 44 families, 633 cells, 27 recovery packages |

The production build used the ignored owned-game data and EData source. No
proprietary input, generated runtime pack, package, or rebuilt WASM artifact is
included in the commit.

## Acceptance boundary

This is an architecture and behavioral checkpoint. Sector, detail, Finder,
Message Index, Encyclopedia, contextual dialogs, and tactical modal surfaces
still retain their feature-local content stores until their planned migration
slices. Exact rail thumbnails, native bitmap compositions, original-runtime
comparison, and the complete multiwindow browser matrix remain open.

Accordingly, `UIP-F-011` remains `partial` and no `CMD-04` acceptance cell is
promoted by this report alone.
