# P58-B06 tactical completion checkpoint

This checkpoint consolidates P58F14 through P58F17C. It restores the current
source-backed capital and fighter combat path, collision and retained
formations, original Battle Results and Battle Options surfaces, the separate
Death Star object and superlaser journey, resource 5030 stars, and exact
result-state routing to the two trench-run films.

P58-B06 remains in progress. This report records a complete implementation
regression gate, not final original-game visual acceptance.

## Recovered contracts

| Behavior | Source evidence |
|---|---|
| Capital weapon arcs, range, firing order, recharge, and shield/hull effects | `ghidra/notes/tactical-weapon-loop.md` |
| Fighter craft count, typed fire, shields, hull, maneuver defense, and retained projectiles | `ghidra/notes/tactical-fighter-combat.md` |
| Collision envelope, capability ordering, task-force leaders, and forward-relative follower positions | `ghidra/notes/tactical-collision-formation.md` |
| Death Star ownership, charge, target, result-state dispatch, and standard arrow cursor | `ghidra/notes/tactical-death-star-path.md` |

The owned English executable used for these records has SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.

## Verification

| Gate | Result |
|---|---|
| Workspace tests | 751 passed, 0 failed, 21 ignored |
| Retained-formation regression tests | 6 passed, 0 failed |
| Browser-harness tests | 12 passed, 0 failed |
| Catalog and production exclusion | Passed; 38 CMD-02 scenarios and zero production fixture tokens |
| Production and fixture WASM build | Passed |
| Complete tactical browser matrix | 120 of 120 passed in `2026-09-22T15-12-09-825Z-34748` |
| Browser isolation | 120 four-request starts, 120 muted launches, 120 stable captures, 120 browser closures |
| Browser diagnostics | No launch, execution, screenshot, request, console, page, missing-asset, or cleanup errors |
| Independent code review | Sol extra-high; three findings closed: dual Death Stars fail closed, draw destinations are inert and hidden, and retry totals retain every execution |
| Campaign return regressions | Exact hull/fighter roster slots, destroyed-fleet officer capture, auto-resolve projection, and Death Star persistent state pass focused tests |
| Independent visual review | Qualified A1 pass; ready to commit. The reviewer inspected all 120 contact-sheet cases and 30 full-resolution frames across both factions and viewports, with no P0 or P1 findings |
| Production WASM SHA-256 | `6ef537d5708faf1fa13e0f7ad934d2cd795af73bc18b23c48a236a8dbc380018` |
| Fixture WASM SHA-256 | `1279127e13940b6482ca42185eb4c900fd25d2ae45b2eda039e8dd6c44ed167c` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact inventory](p58-b06-tactical-completion/) retains the durable
machine-readable summary. Raw screenshots and logs remain in the ignored local
artifact store because they include extracted original artwork.

## Independent visual review

The independent browser-use review matched all 120 screenshot hashes to the
case records, confirmed every four-request start, muted launch, stable capture,
and browser closure, and found no P0 or P1 defect. It left three bounded items:

- P2: the Battle Options `Game Options` route remains fail-closed until the
  authentic Game Options window is implemented;
- P2: the Alliance Death Star overview needs an original-view framing baseline;
- P3 evidence gap: the successful trench-run return proves media routing and
  return control, but not native playback or the persistent campaign outcome.

The machine-readable [browser acceptance](p58-b06-tactical-completion/browser-acceptance.json)
records that verdict without retaining proprietary screenshots.

## Acceptance boundary

The 120 cases are deterministic implementation evidence across 30 scenarios,
both factions, and two viewports. They are not the 106 original-game
acceptance cells. All 106 strict tactical cells remain open because the current
screenshots have no lossless A0 original-executable counterparts.

The native superlaser beam dimensions and timing, original trench-run result
producer, native media playback comparison, exact global RNG sequencing,
original arrival callbacks, exact planet placement, power allocation, and
complete original-view evidence remain open.
