# P58F13 tactical attack target lifecycle

P58F13 connects the recovered tactical target-list machinery to the live battle
step. Invalid typed targets now move to the first eligible hostile object of the
same requested class. Exhausted lists clear instead of redirecting to a random
or wrong-class object.

## Source contract

The owned English executable has SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The detailed recovery record is in
[`ghidra/notes/tactical-attack-target-lifecycle.md`](../../../../ghidra/notes/tactical-attack-target-lifecycle.md).

| Behavior | Recovered source |
|---|---|
| Tactical target-list manager | `FUN_005a7500` |
| Previous and next target traversal | `FUN_005a8c50`, `FUN_005a8cc0` |
| Add, remove, replace, and change paths | `FUN_005a8d30`, `FUN_005a8e30`, `FUN_005a8e80`, `FUN_005a8fc0` |
| Attack executor reset, setter, and getter | `FUN_005d0b00`, `FUN_005cf910`, `FUN_005cf920` |
| Separate capital weapon loop | `FUN_005b3a40`, `FUN_005b3f10`, `FUN_005b6530` |

The deterministic fixture first assigns both attack orders to capital and
fighter owners. It then marks the first hostile capital as retreating and the
first hostile fighter as recovered. All four owners select the second eligible
object of the requested class.

## Verification

| Gate | Result |
|---|---|
| Focused tactical renderer tests | 2 passed, 0 failed |
| Fixture decode test | 1 passed, 0 failed |
| Workspace tests | 713 passed, 0 failed, 21 ignored |
| Scoped Clippy | Renderer library and fixture-enabled app passed; existing DAT placeholder warnings remain outside this change |
| Harness unit tests | 11 passed, 0 failed |
| Catalog and production exclusion | Passed |
| Focused browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 92 of 92 passed |
| Browser isolation | 92 four-request starts, 92 fresh muted launches, 92 browser closures |
| Runtime diagnostics | No page, console, request, missing-asset, instability, or launch-timeout errors |
| Independent browser acceptance | Qualified A1 pass across both factions and viewports |
| Production WASM SHA-256 | `17f3c5283b13d03ee2a1ebaf3d403d1b2cac505e05b3756949f6588bbe144d63` |
| Fixture WASM SHA-256 | `65eb93f512a9e4c4042204ba5c79e411581952464c2779aed8dee31f6bf3c23b` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f13-tactical-attack-target-lifecycle/) retains both
factions at both viewports, machine-readable lifecycle records, focused and
complete summaries, a contact sheet, and the independent verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. All 92 comparisons are
unbaselined, so it does not prove A0 visual parity. The exact arc, range,
weapon-availability, strongest-arc, projectile-event, recharge, global RNG,
collision, formation, fighter-combat, Death Star, and battle-result contracts
remain open. All 106 strict tactical cells remain pending.
