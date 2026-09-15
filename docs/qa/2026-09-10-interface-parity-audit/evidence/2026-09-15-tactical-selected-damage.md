# P58F4 tactical selected-unit damage

P58F4 restores the selected-capital-ship portrait and live damage meters inside
the original TACTICAL 1302 panel. It removes the old assumption that the
capital portrait could be derived from CAPSHPSD order. The portrait now uses
the source tactical registry ordinal already proven for each capital ship.

## Implemented contract

| Surface | Mapping |
|---|---|
| Selected panel | TACTICAL bitmap 1302 at native position 481,27 |
| Capital portrait | `2001 + tactical_ordinal`, using the 29-entry registry constructed by `FUN_00597610` |
| Portrait composite | Pure lime matte becomes transparent while the 1302 green grid remains opaque |
| Shield meter | Live `shield / shield_max` in the left blue aperture |
| Hull meter | Live `hull_current / hull_max` in the right faction-colored aperture |

The source registry and resource names independently correlate all 29 capital
classes with bitmaps 2001 through 2029. Original Alliance and Imperial captures
locate the portrait and meters within panel 1302 and show blue Alliance hulls,
red Imperial hulls, and blue shields. The five lower icons are power and
command states. They remain open rather than being derived from damage values.

The detailed resource inventory is in the
[space-battle asset map](../../../reference/asset-library/space-battle.md), and
the pinned third-party label table is linked there as a research aid.

## Verification

| Gate | Result |
|---|---|
| Focused render and app tests | 200 passed, 0 failed, 14 ignored |
| Workspace tests | 688 passed, 0 failed, 21 owned-data or documentation examples ignored |
| Formatting, scoped strict Clippy, workspace check | Pass |
| JSON, production-fixture exclusion, parity-ledger validation | Pass; 0 production fixture tokens and all 43 required surfaces retain recovery packages |
| Focused selected-unit browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 56 of 56 passed |
| Browser isolation | 56 fresh muted launches, 56 four-request starts, 56 closed |
| Native panel proof | 23,686 exact unobscured pixels per faction against bitmap 1302 |
| Portrait proof | Alliance 1,130 of 1,220 and Empire 994 of 1,028 opaque pixels remained source-exact; text accounts for the intentional overlap |
| Damage-meter proof | Exact blue shield and faction-colored hull pixels at both viewports |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium visual acceptance | Pass across both factions and both viewports; panel 1302, portraits, matte transparency, and faction-correct meters are visible without blank or clipped surfaces |
| Production WASM SHA-256 | `92f9ef5ae80f51b4bad4b577a4340b6da4558314e31ecb6ceb883c3f2bef39ce` |
| Fixture WASM SHA-256 | `85469dff574b72d01df73ec142cc7c2facc5823315b60af23e3cdc8286626e72` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f4-tactical-selected-damage/) retains both factions at
640 by 480 and 1280 by 800 letterboxed viewports, their result records, both
summaries, a contact sheet, and the passing Astra acceptance record.

## Acceptance boundary

This is qualified A1 implementation evidence. The selected panel, capital
portrait identity, matte composition, and live shield and hull presentation
are proven in the production render path. Power allocation, passengers,
maneuvers, missions, destroyed selection, Death Star presentation, exact
command timing, audio, lossless original-executable comparison, and all 106
strict tactical cells remain open.
