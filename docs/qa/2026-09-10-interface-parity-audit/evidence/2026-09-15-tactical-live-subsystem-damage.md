# P58F6 tactical live subsystem damage

P58F6 connects the five selected-capital subsystem indicators restored by
P58F5 to live capital-ship and fighter damage. One shared production function
now owns shield overflow, hull loss, subsystem-hit selection, hit limits,
condition recalculation, and tractor-command cancellation.

## Implemented contract

| Surface | Mapping |
|---|---|
| Shield damage | Inclusive 0 through 100 roll against the percentage of prior shield energy removed, with a minimum probability of one |
| Hull overflow | Negative shield energy transfers to hull, with at least one hull point on the crossing branch |
| Hull subsystem score | Inclusive 0 through 100 roll plus signed hull percentage change |
| Hull thresholds | Greater than 50 through 65 shields; through 80 weapons; through 90 tractor; through 95 engines; through 100 hyperdrive |
| Hit limits | Four each for shields, weapons, tractor, and engines; normal plus damaged CAPSHPSD drive fields give hyperdrive zero through two components |
| Condition | Hull ratio minus 25 percentage points per hit for the first four systems; remaining installed components for hyperdrive |
| Tractor cancellation | A tractor hit that reduces available tractor power to zero removes that exact source from its live target |
| Browser proof state | Hit counts `[1,2,3,4,2]` produce percentages `[74,49,24,0,0]` and resources `[1204,1208,1212,1216,1221]` |

The executable path, object fields, formulas, and remaining movement terms are
recorded in the [Ghidra note](../../../../ghidra/notes/tactical-subsystem-damage-path.md).
The browser fixture directly prepares a deterministic battle state, then sends
every recorded hit through the production damage function. It does not stand
in for an interactive combat journey.

## Verification

| Gate | Result |
|---|---|
| Focused tactical render tests | 22 passed, 0 failed |
| Workspace tests | 695 passed, 0 failed, 21 ignored |
| Formatting and touched-library strict Clippy | Pass |
| Broader all-target strict Clippy | Existing dat-dumper placeholder-name baseline remains open |
| Harness unit tests and static validation | 11 passed; catalog, production-exclusion, JSON, and ledger validation pass |
| Focused live-damage browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 64 of 64 passed |
| Browser isolation | 64 fresh muted launches, 64 four-request starts, 64 closed |
| Source-state proof | Limits `[4,4,4,4,2]`, hits `[1,2,3,4,2]`, percentages `[74,49,24,0,0]`, resources `[1204,1208,1212,1216,1221]` |
| Native bitmap proof | Panel 1302 checks 21,736 pixels; each of five subsystem resources checks 330 pixels per faction |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium visual acceptance | Pass across both factions and both viewports; panels, five tiles, portraits, meters, shells, planets, stars, and controls are coherent |
| Production WASM SHA-256 | `96bd154120f2a2c76284a16808d12191c65e65036c81b935be83ac5517983c1e` |
| Fixture WASM SHA-256 | `f0c85fe4611eebfbfaf65887bc06a3bd6b5a9f5b72092ebf2313f87a388c74b5` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f6-tactical-live-subsystem-damage/) retains both
factions at 640 by 480 and 1280 by 800 letterboxed viewports, result records,
focused and complete summaries, a contact sheet, and the passing Astra record.

## Acceptance boundary

This is qualified A1 implementation evidence. The source damage branches,
thresholds, counters, component capacities, condition resources, and
tractor-cancel side effect are proven in the production tactical state path.
The executable RNG sequence, engine mode bonuses, active tractor speed
penalties, repair, interactive command delivery, lossless A0 comparison, and
all 106 strict tactical cells remain open.
