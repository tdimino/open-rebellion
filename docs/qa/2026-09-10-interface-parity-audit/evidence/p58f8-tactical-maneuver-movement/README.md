# P58F8 tactical maneuver and movement evidence

This bundle preserves the deterministic maneuver and movement browser run for
both factions and both supported viewports.

## Inventory

- [`focused-summary.json`](focused-summary.json) records the four P58F8 cases.
- [`full-summary.json`](full-summary.json) records the complete 72-case tactical
  regression matrix.
- [`contact-sheet.png`](contact-sheet.png) presents all four P58F8 captures.
- [`astra-browser-acceptance.json`](astra-browser-acceptance.json) records the
  Astra medium visual-evidence verdict.
- [`alliance/`](alliance/) indexes Alliance captures.
- [`empire/`](empire/) indexes Imperial captures.

Each case proves state value 4, maneuver bonus 5, one engine hit, effective
power 78.75, alignment 1, velocity with the correct faction sign, and a
19.6875-unit position change over one 250-millisecond production step. Every
complete-matrix case starts in a fresh muted browser, makes four successful
requests, records stable frames and clean diagnostics, and closes its browser.

This is passing A1 implementation evidence. Interactive maneuver delivery,
turning, collision handling, exact global RNG equivalence, original A0
comparison, and all 106 strict tactical cells remain open.
