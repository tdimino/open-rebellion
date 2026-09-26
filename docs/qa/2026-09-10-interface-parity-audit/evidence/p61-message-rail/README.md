# P61 Message Index rail illumination evidence

This bundle records the P61 rail illumination and its muted 640x480 browser
check. The rule and the producer mapping are in the
[Message Index recovery](../2026-09-24-message-index-recovery.md).

## Inventory

- [`summary.json`](summary.json) records the source contract, gates, hashes,
  and open boundaries.
- [`browser-acceptance.json`](browser-acceptance.json) records each rail
  control's pixel distance to its resting and illuminated STRATEGY bitmaps.

Screenshots and logs stay in the session scratchpad. No original bitmap or
runtime pack is committed.

## Method

Each session starts a new game, runs at Medium and then Fast, and crops the
nine 27x22 rail controls at native 640x480. Each crop is compared with both
source bitmaps; a mean absolute channel difference of 0.0 is an exact match.
The seeded campaign's "Battle at Carida" dialog covers the rail from day 16,
so samples stop before it.
