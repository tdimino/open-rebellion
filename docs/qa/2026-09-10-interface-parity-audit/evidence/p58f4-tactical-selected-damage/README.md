# P58F4 tactical selected-unit evidence

This bundle preserves the deterministic selected-capital-ship browser run for
both factions and both supported viewports.

## Inventory

- [`focused-summary.json`](focused-summary.json) records the four selected-unit
  cases.
- [`full-summary.json`](full-summary.json) records the complete 56-case
  tactical regression matrix.
- [`contact-sheet.png`](contact-sheet.png) presents all four selected-unit
  captures together.
- [`astra-browser-acceptance.json`](astra-browser-acceptance.json) records the
  passing Astra medium visual-evidence verdict.
- [`alliance/`](alliance/) indexes Alliance captures.
- [`empire/`](empire/) indexes Imperial captures.

The native-size cases compare 23,686 unobscured pixels against original bitmap
1302. They also verify the selected Mon Calamari and Strike Cruiser portraits,
the lime-matte composite over the authored grid, and exact live shield and hull
meter colors. Every case starts in a fresh muted browser, makes four successful
requests, records stable frames and clean diagnostics, and closes its browser.

This is passing A1 implementation evidence. Original A0 comparison, power allocation,
commands, destroyed selection, Death Star states, and all 106 strict tactical
cells remain open.
