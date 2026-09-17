# P58F9 tactical command assignment

P58F9 restores the original bitmap-driven Maneuvers and Tactics and Missions
panels for selected capital ships and fighter groups. The runtime now retains
the executable's shared order code and tactic code on each selected unit and
applies changes only when the source check button is activated.

## Implemented contract

| Surface | Mapping |
|---|---|
| Selected-unit controls | Missions `1107/1108` and Maneuvers and Tactics `1105/1106` at the recovered panel-local positions |
| Maneuver panel | Background `1309`; Left Hook, Right Hook, Hammer, Anvil, and Hold Position with original normal and pressed BMPs |
| Maneuver graphics | `2101` through `2105` at the recovered confirmation aperture |
| Tactic toggle | Stand Off `1117`, code 2; Surround `1118`, code 1 |
| Mission panel | Background `1308`; faction-specific Attack Capital Ships, Attack Fighters, and Recover controls |
| Mission graphics | Alliance `2151` through `2154`; Empire `2155` through `2157`; empty `2158` |
| Disabled states | Recover is disabled for capital ships; Attack Death Star remains disabled pending its recovered enablement contract |
| Shared order codes | Recover 2, Attack Fighters 4, Attack Capital Ships 5, Attack Death Star 6, Left Hook 7, Right Hook 8, Hammer 9, Anvil 10, Hold Position 11 |
| Commit behavior | Check applies the pending order and tactic to every selected member; cancel returns without mutation |

The exact executable functions, rectangles, message IDs, resources, and field
offsets are recorded above and in the tracked
[reverse-engineering map](../../../reference/space-battle-launcher/reverse-engineering-map.md).

## Verification

| Gate | Result |
|---|---|
| Focused tactical renderer tests | 51 passed, 0 failed, 1 owned-data test ignored |
| Focused fixture test | 1 passed, 0 failed |
| Workspace tests | 702 passed, 0 failed, 21 ignored |
| Focused command browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 76 of 76 passed |
| Browser isolation | 76 fresh muted launches, 76 four-request starts, 76 stable captures, 76 closed |
| Capital maneuver journey | Cancel preserves state; Hammer code 9 plus Surround code 1 commits to one selected capital |
| Capital mission journey | Recover and Attack Death Star are disabled; Attack Capital Ships code 5 commits |
| Fighter mission journey | Recover enables for the selected group and code 2 commits to the group |
| Runtime diagnostics | No page, console, request, missing-asset, or browser-cleanup errors |
| Astra medium visual acceptance | Pass across both factions and both viewports with no severity findings |
| Production WASM SHA-256 | `dc2ba421fdc21e0e46bdb176f0156a6a40195ddb98c8875da110205dc6a8671b` |
| Fixture WASM SHA-256 | `4d7dbee40f3a10a842bf138c712cdfbb244201a7adeb55ceaeaae926ddc6c465` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f9-tactical-command-assignment/) retains every
inspected command state for both factions and both viewports, per-case result
records, focused and complete summaries, a contact sheet, and the Astra verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. Exact panel geometry, original
bitmap states, source-shaped hit testing, mutual selection, disabled states,
cancel behavior, and command delivery are proven. Order execution, Death Star
enablement, current-to-desired turning, collision and formation behavior,
lossless original comparison, and all 106 strict tactical cells remain open.
