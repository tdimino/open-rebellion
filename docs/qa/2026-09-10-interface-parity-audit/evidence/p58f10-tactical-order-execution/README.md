# P58F10 tactical order execution evidence

This bundle preserves the deterministic order-execution state for both
factions and both supported viewports.

## Inventory

- [`focused-summary.json`](focused-summary.json) records the four P58F10 cases.
- [`full-summary.json`](full-summary.json) records the complete 80-case tactical matrix.
- [`contact-sheet.png`](contact-sheet.png) presents the four inspected frames.
- [`astra-browser-acceptance.json`](astra-browser-acceptance.json) records the Astra medium verdict.
- [`alliance/`](alliance/) indexes Alliance captures.
- [`empire/`](empire/) indexes Imperial captures.

Each case records Left Hook with Surround and a finite source waypoint, Hold
with cleared movement intent, and Recover in Returning state with a compatible
same-faction carrier. This is passing A1 implementation evidence. A0 comparison,
turning and arrival, complete fighter recovery, attack executors, Death Star
execution, and all 106 strict tactical cells remain open.
