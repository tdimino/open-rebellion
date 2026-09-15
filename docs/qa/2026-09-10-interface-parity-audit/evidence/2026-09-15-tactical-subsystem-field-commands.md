# P58F5 tactical subsystem and field-command presentation

P58F5 restores all five selected-capital subsystem-condition bitmap families
and replaces anonymous field-source counts with the exact tractor and gravity
source identities retained by the original executable.

## Implemented contract

| Surface | Mapping |
|---|---|
| Selected panel | TACTICAL bitmap 1302 at native position 481,27 |
| Subsystem order | Shields, weapons, tractor beam, sublight engines, hyperdrive |
| Bitmap families | 1201 to 1205, 1206 to 1210, 1211 to 1215, 1216 to 1220, 1221 to 1225 |
| Quantization | 0, 1 to 24, 25 to 49, 50 to 74, and 75 or greater map to offsets 0 through 4 |
| Placement | x 491, 518, 545, 572, and 599 at y 130; each bitmap is 22 by 15 |
| Tractor identity | One target per source; several exact sources may hold one target |
| Gravity identity | Up to four exact targets per source |
| Shared field presentation | Gravity 3520 to 3527 preempts tractor 3620 to 3627; visible-kind changes reset the frame |

The executable path and recovered structure fields are recorded in the
[Ghidra note](../../../../ghidra/notes/tactical-subsystem-field-command-path.md).
The deterministic fixture uses percentage boundaries 0, 24, 25, 50, and 75,
which select resources 1201, 1207, 1213, 1219, and 1225.

## Verification

| Gate | Result |
|---|---|
| Focused render and app tests | 17 passed, 0 failed |
| Workspace tests | 689 passed, 0 failed, 21 owned-data or documentation examples ignored |
| Formatting, touched-library strict Clippy, app no-deps strict Clippy, workspace check | Pass |
| Broader all-target strict Clippy | Existing dat-dumper placeholder-name and constant-test baseline remains open |
| Harness unit tests | 11 passed, 0 failed |
| JSON, production-fixture exclusion, parity-ledger validation | Pass; zero production fixture tokens and all 43 required surfaces retain recovery packages |
| Focused subsystem/field browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 60 of 60 passed |
| Browser isolation | 60 fresh muted launches, 60 four-request starts, 60 closed |
| Subsystem bitmap proof | Five exact 22 by 15 source bitmaps per faction, 330 pixels each at native size |
| Field identity proof | Tractor sources 3 and 5 share target 1; tractor source 0 and gravity source 2 share target 4 with gravity visible |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium visual acceptance | Pass across both factions and both viewports; all five tiles and both field effects are visible without blank, corrupt, or clipped surfaces |
| Production WASM SHA-256 | `4ac0e22a9bc698b5d49ddc31be2461bf51aec139282ed20fb7294aa5aff9b31d` |
| Fixture WASM SHA-256 | `b9b3c98eea518d8ef4014af67c67f711ea6814b32425464d1207641538f0e91b` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f5-tactical-subsystem-field-commands/) retains both
factions at 640 by 480 and 1280 by 800 letterboxed viewports, their result
records, focused and complete summaries, a contact sheet, and the passing
Astra acceptance record.

## Acceptance boundary

This is qualified A1 implementation evidence. Resource identity,
quantization, placement, exact field-source retention, capacity limits,
priority, frame reset, and browser presentation are proven in the production
render path. Combat-driven subsystem damage, interactive field-command input,
event and audio delivery, lossless A0 original-runtime comparison, and all 106
strict tactical cells remain open.
