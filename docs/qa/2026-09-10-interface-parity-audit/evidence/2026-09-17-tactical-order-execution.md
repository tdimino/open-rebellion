# P58F10 tactical order execution

P58F10 connects committed tactical orders to recovered executor behavior. The
four capital maneuver orders now construct source-shaped waypoints and desired
directions, Hold Position clears movement intent, and fighter Recover reserves
a compatible carrier and enters the original Returning state.

## Implemented contract

| Order | Execution state |
|---|---|
| Left Hook, code 7 | Selects an opposing capital, rotates the source-to-target vector by positive pi over eight in XZ, scales it by 0.75, and assigns the resulting waypoint |
| Right Hook, code 8 | Uses the matching negative pi over eight XZ rotation and 0.75 scale |
| Hammer, code 9 | Uses the recovered negative pi over eight YZ rotation and 0.75 scale |
| Anvil, code 10 | Uses the recovered positive pi over eight YZ rotation and 0.75 scale |
| Hold Position, code 11 | Clears the active waypoint, desired direction, and velocity |
| Recover, code 2 | Resets recovery state, reserves one same-side carrier slot, records the carrier, and enters Returning state 2 |

The executable dispatcher is `FUN_005ca6d0`. Its executor initializers are
`FUN_005cef00`, `FUN_005cf1a0`, `FUN_005cf410`, `FUN_005cf680`,
`FUN_005cf930`, and `FUN_005cf940`. Target and waypoint assignment follow
`FUN_005d21b0`, `FUN_005cda40`, `FUN_005a9530`, and `FUN_005a8f70`.

## Verification

| Gate | Result |
|---|---|
| Tactical renderer tests | 33 passed, 0 failed |
| Fixture tests | 1 passed, 0 failed |
| Workspace tests | 704 passed, 0 failed, 21 ignored |
| Workspace check | Passed |
| Scoped strict Clippy | Renderer library and fixture-enabled app passed |
| Harness unit tests | 11 passed, 0 failed |
| Catalog and production exclusion | Passed |
| Focused browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 80 of 80 passed |
| Browser isolation | 80 four-request starts, 80 stable captures, 80 fresh muted launches, 80 browser closures |
| Runtime diagnostics | No page, console, request, missing-asset, launch-timeout, or cleanup errors |
| Astra medium browser acceptance | Qualified A1 pass with no critical, major, or minor findings |
| Production WASM SHA-256 | `db2a971ab4a0ff8e556c7347f94e38c9a27740d3647683ed58b4b5ef72fbb77d` |
| Fixture WASM SHA-256 | `f7fcfa98b1138010a0ad2b5d388a3f233a25b77b4440d8f819920bc8b5c4344e` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f10-tactical-order-execution/) retains both factions
at both viewports, per-case state and browser records, the focused and complete
summaries, a contact sheet, and the Astra verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. It does not prove A0 visual
parity. Exact turn-rate integration, waypoint arrival and executor completion,
fighter ingress and docking, collision and formation behavior, Attack Fighters,
Attack Capital Ships, Attack Death Star, and all 106 strict tactical cells
remain open.
