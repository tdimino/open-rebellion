# P58F6 tactical live subsystem-damage evidence

This bundle preserves the deterministic live subsystem-damage browser run for
both factions and both supported viewports.

## Inventory

- [`focused-summary.json`](focused-summary.json) records the four P58F6 cases.
- [`full-summary.json`](full-summary.json) records the complete 64-case tactical
  regression matrix.
- [`contact-sheet.png`](contact-sheet.png) presents all four P58F6 captures.
- [`astra-browser-acceptance.json`](astra-browser-acceptance.json) records the
  passing Astra medium visual-evidence verdict.
- [`alliance/`](alliance/) indexes Alliance captures.
- [`empire/`](empire/) indexes Imperial captures.

Each native case proves all 330 pixels in resources 1204, 1208, 1212, 1216,
and 1221. The selected ship records source-derived hit limits `[4,4,4,4,2]`,
hit counts `[1,2,3,4,2]`, condition percentages `[74,49,24,0,0]`, an empty
shield meter, and a nearly full faction-color hull meter. Every complete-matrix
case starts in a fresh muted browser, makes four successful requests, records
stable frames and clean diagnostics, and closes its browser.

This is passing A1 implementation evidence. The executable RNG sequence,
engine mode bonuses, active tractor speed penalties, repair, interactive
command delivery, original A0 comparison, and all 106 strict tactical cells
remain open.
