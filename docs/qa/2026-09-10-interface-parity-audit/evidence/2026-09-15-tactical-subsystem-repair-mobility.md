# P58F7 tactical subsystem repair and mobility

P58F7 closes the live condition loop started by P58F6. Production tactical
ships now attempt source-derived subsystem repair every 50 seconds, select one
outstanding hit in the executable's category order, and refresh engine
condition from engine damage and all active tractor sources.

P58F8 supersedes this checkpoint's provisional zero-default maneuver seam with
the recovered one-through-nine producer and physical integration contract. The
retained P58F7 artifact remains the historical evidence for this gate.

## Implemented contract

| Surface | Mapping |
|---|---|
| Repair cadence | `50.0 * 1000.0` milliseconds, represented by 200 combat steps at four steps per second |
| Repair chance | Inclusive 1 through 100 roll no greater than CAPSHPSD damage control |
| Hit selection | Uniform outstanding-hit ordinal in engine, shield, hyperdrive, tractor, weapon order |
| Repair mutation | Decrement exactly one selected counter and clamp at zero |
| Base sublight | CAPSHPSD sublight rating multiplied by `0.4 * 0.6`, or `0.24` |
| Engine damage | Subtract 25% of base plus current mode bonus per engine hit; target hull does not scale engines |
| Tractor penalty | Sum each active source's hull-scaled tractor maximum minus 25% of maximum per tractor hit |
| Browser proof state | One engine repair changes hits from two to one; base 100 becomes 75 before a 25-point tractor penalty yields 50 effective power |
| Visible condition | Hits `[1,2,1,1,1]` produce percentages `[75,50,75,50,50]` and resources `[1205,1209,1215,1219,1224]` |

The executable paths and constants are recorded in the
[Ghidra note](../../../../ghidra/notes/tactical-subsystem-repair-mobility.md).
The deterministic fixture directly prepares one damaged selected ship, invokes
the production repair and field paths, and centers the selected source object.
It does not prove an interactive command journey.

## Verification

| Gate | Result |
|---|---|
| Focused tactical render and fixture tests | 26 passed, 0 failed |
| Workspace tests | 699 passed, 0 failed, 21 ignored |
| Formatting and touched render-library strict warnings | Pass |
| Broader app/workspace Clippy | Existing dat-dumper placeholder-name baseline remains open |
| Harness unit and static checks | 11 passed; catalog and production-exclusion checks pass |
| Focused repair/mobility browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 68 of 68 passed |
| Browser isolation | 68 fresh muted launches, 68 four-request starts, 68 closed |
| Source-state proof | Repair `engines 2 -> 1`; base 100, active tractor 25, effective engine 50 |
| Native-size bitmap proof | Each of resources 1205, 1209, 1215, 1219, and 1224 checks 330 pixels per faction |
| Field containment | Entire 128 by 128 tractor source bounds remain inside the tactical aperture in both viewports |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium visual acceptance | First candidate rejected for Alliance field clipping; corrected centered evidence passes both factions and viewports |
| Production WASM SHA-256 | `0b4a5aba62dcdb2e3daf674f1b61c38410869c0d303b294e2b7ace4ba1ecf93f` |
| Fixture WASM SHA-256 | `deac5d89a8008adbeb73c0b88d19412b620f279e142eff4ef46dbefa18daa280` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f7-tactical-subsystem-repair-mobility/) retains both
factions at 640 by 480 and 1280 by 800 letterboxed viewports, result records,
focused and complete summaries, a contact sheet, and the Astra verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. The repair timer, chance range,
category order, counter decrement, sublight conversion, engine-hit penalty,
tractor-source penalty, visible condition resources, and browser rendering are
proven in the production tactical state path. Exact global RNG equivalence,
interactive command delivery, turning, collision and formation behavior,
lossless A0 comparison, and all 106 strict tactical cells remain open. The
maneuver producer and physical integration are closed by P58F8.
