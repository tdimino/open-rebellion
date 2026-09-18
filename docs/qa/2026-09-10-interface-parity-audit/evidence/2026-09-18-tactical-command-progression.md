# P58F11 tactical command progression

P58F11 advances committed tactical commands through recovered runtime state.
Capital ships now turn toward maneuver waypoints using the executable-derived
rate and signed XZ rotation, completed waypoints clear their movement intent,
and fighters progress from Returning through Docking to Recovered without
losing their strategic squadron count.

## Source contract

The owned English executable has SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The detailed recovery record is in
[`ghidra/notes/tactical-command-progression.md`](../../../../ghidra/notes/tactical-command-progression.md).

| Behavior | Recovered source |
|---|---|
| Maneuverability clamp and turn-rate construction | `FUN_005ba270`, `FUN_005b9c60` |
| Signed XZ turn, normalization, and snap branch | `FUN_005cd2d0`, `FUN_005cd460` |
| Velocity and millisecond position integration | `FUN_005b2f30`, `FUN_005cd640` |
| Maneuver executor completion | `FUN_005cf190`, `FUN_005cf8f0` |
| Returning, Docking, and Recovered transitions | `FUN_005cf980`, `FUN_005b1e10`, `FUN_005b8630` |
| Reserved and reset states | `FUN_005b84e0`, `FUN_005ba2f0`, `FUN_005cf940` |
| Strict fighter docking distance | `_DAT_0066d088 = 2.0` |

The deterministic fixture proves raw maneuverability 4 produces a
250-millisecond turn of `5pi/72` radians, or `0.2181661564992912`. It also
proves waypoint arrival clears the order and movement fields, state 3 remains
active at 1.5 source units from the carrier, and state 4 removes the tactical
object while preserving the strategic count.

## Verification

| Gate | Result |
|---|---|
| Tactical renderer tests | 35 passed, 0 failed |
| Fixture tests | 1 passed, 0 failed |
| Workspace tests | 706 passed, 0 failed, 5 ignored |
| Workspace check | Passed |
| Scoped strict Clippy | Renderer library and fixture-enabled app passed |
| Harness unit tests | 11 passed, 0 failed |
| Catalog and production exclusion | Passed |
| Focused browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 84 of 84 passed |
| Browser isolation | 84 four-request starts, 84 stable captures, 84 fresh muted launches, 84 browser closures |
| Runtime diagnostics | No page, console, request, missing-asset, or launch-timeout errors |
| Astra medium browser acceptance | Qualified A1 pass with no P0 or P1 findings |
| Production WASM SHA-256 | `bb3654f42466a9ad46dec1f8f9f9a3e189201663d60b2e7505a574f5d632f65b` |
| Fixture WASM SHA-256 | `1ef421bc833df6312a6f1bb225ea7b6be2f5b8690f72ce44d64c6f108c02de3e` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f11-tactical-command-progression/) retains both
factions at both viewports, machine-readable state and browser records, focused
and complete summaries, a contact sheet, and the Astra verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. All 84 comparisons are
unbaselined, so it does not prove A0 visual parity. The exact original
target-arrival callback, complete fighter trajectory, global RNG sequence,
attack executors, collision and formation behavior, Death Star execution, and
all 106 strict tactical cells remain open.
