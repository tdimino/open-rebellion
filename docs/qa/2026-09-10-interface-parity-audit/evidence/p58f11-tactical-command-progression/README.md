# P58F11 tactical command progression evidence

This bundle preserves deterministic turning, maneuver completion, and fighter
recovery state for both factions and both supported viewports.

## Inventory

- [`focused-summary.json`](focused-summary.json) records the four P58F11 cases.
- [`full-summary.json`](full-summary.json) records the complete 84-case tactical matrix.
- [`contact-sheet.png`](contact-sheet.png) presents the four inspected frames.
- [`contact-sheet.html`](contact-sheet.html) links the four lossless captures.
- [`astra-browser-acceptance.json`](astra-browser-acceptance.json) records the Astra medium verdict.
- [`alliance/`](alliance/) indexes Alliance captures.
- [`empire/`](empire/) indexes Imperial captures.

Each case records a normalized source-rate capital turn, one completed maneuver
waypoint, one active Docking fighter, and one inactive Recovered fighter whose
strategic squadron count remains intact. This is qualified A1 implementation
evidence. A0 comparison, exact arrival callback recovery, attack execution,
collision, formation, Death Star execution, and all 106 strict tactical cells
remain open.
