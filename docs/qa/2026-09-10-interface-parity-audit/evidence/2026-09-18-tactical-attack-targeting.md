# P58F12 tactical attack target acquisition

P58F12 connects Attack Fighters and Attack Capital Ships to typed, stable-order
target acquisition for both capital ships and fighter groups. Existing valid
engagements remain intact. Invalid targets are replaced with the first eligible
hostile object of the requested class.

## Source contract

The owned English executable has SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The detailed recovery record is in
[`ghidra/notes/tactical-attack-targeting.md`](../../../../ghidra/notes/tactical-attack-targeting.md).

| Behavior | Recovered source |
|---|---|
| Tactical order dispatch | `FUN_005ca6d0` |
| Attack Capital Ships executor | `FUN_005d0b10`, vtable `0x0066cf48` |
| Attack Fighters executor | `FUN_005d0bb0`, vtable `0x0066cf78` |
| Stable tactical object list | `FUN_005f5060` |
| Owner and candidate policy checks | `FUN_005c91e0`, `FUN_005c91c0` |
| Target-link dispatch | `FUN_005a8fc0`, event `0x36` |
| Object class contract | vtable slot `+0x20`: capital 0, fighter 1 |

The deterministic fixture proves all four owner and target-class combinations:
capital to capital, capital to fighter, fighter to capital, and fighter to
fighter. The retained frame selects a capital executing Attack Fighters and
shows its line ending on the chosen fighter marker.

## Verification

| Gate | Result |
|---|---|
| Focused tactical renderer tests | 2 passed, 0 failed |
| Fixture decode test | 1 passed, 0 failed |
| Workspace tests | 711 passed, 0 failed, 21 ignored |
| Scoped strict Clippy | Renderer library and fixture-enabled app passed |
| Broader renderer all-target Clippy | Existing constant-assertion test baseline remains open |
| Harness unit tests | 11 passed, 0 failed |
| Catalog and production exclusion | Passed |
| Focused browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 88 of 88 passed |
| Browser isolation | 88 four-request starts, 88 fresh muted launches, 88 browser closures |
| Runtime diagnostics | No page, console, request, missing-asset, instability, or launch-timeout errors |
| Astra medium browser acceptance | Qualified A1 pass with no P0 or P1 findings |
| Final correction visual identity | All four retained PNG hashes match the Astra-inspected captures exactly |
| Production WASM SHA-256 | `62159685f7bafe1c8907164261517a4403d7b5e24e0f3f41f12abd5682bb86a4` |
| Fixture WASM SHA-256 | `c6a859f5ab8f3c9b223b21e4ae4ce61f241ccc1c84133b57deb00e8220062f1f` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f12-tactical-attack-targeting/) retains both factions
at both viewports, machine-readable attack state and browser records, focused
and complete summaries, a contact sheet, and the Astra verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. All 88 comparisons are
unbaselined, so it does not prove A0 visual parity. Exact weapon-resolution
callbacks, target-loss reacquisition, policy predicate internals, global RNG
sequencing, collision and formation behavior, Death Star execution, and all
106 strict tactical cells remain open.
