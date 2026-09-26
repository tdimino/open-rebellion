# P60 Game Speed control evidence

This bundle records the P60 Game Speed control and its muted browser run in
both factions at 640x480 and letterboxed 1280x800. The source contract is in
the [game-speed recovery](../2026-09-24-game-speed-recovery.md); the Message
Index work is static only and lives in the
[Message Index recovery](../2026-09-24-message-index-recovery.md).

## Inventory

- [`summary.json`](summary.json) records the source contract, verification
  gates, artifact hashes, and open boundaries.
- [`browser-acceptance.json`](browser-acceptance.json) records the four-session
  journey, observed day rates, and the defects the run found and closed.

Raw screenshots, timing strips, console logs, and network ledgers remain in
the session scratchpad. No original bitmap, runtime pack, or other proprietary
asset is committed.

## Journey

Each fresh, muted session starts a new game, dismisses the opening advisor,
and then checks, in order:

1. Right-click the day readout, hover Slow, and choose Medium.
2. Alt+P mid-day: the day in progress finishes, the counter holds, and the
   "Resume Game Play?" alert opens at `(114,152)`.
3. Alt+P and Alt+NumPad+ again: nothing changes.
4. Choose Slow from the menu while paused: the game stays paused.
5. Click the checkmark: the alert closes, play resumes at Slow, and the map
   beneath does not receive the click.
6. Alt+P, then Enter: play resumes.
7. Open the speed menu and press Escape: only the menu closes.
8. Choose Fast, then Alt+NumPad− (Medium), Alt+NumPad− and Alt+NumPad+
   (Slow, then Medium).

The pause checks run before day 16, when the seeded campaign's "Battle at
Carida" dialog covers the readout.

## Defects found and closed

- Pause stopped at once instead of finishing the current day (Sol review).
- A game saved while paused lost its running speed (Sol review).
- An interrupted browser save could hide an older complete save (Sol review).
- Enter resumed play while a text field had focus (Sol review).
- Escape with the speed menu open also left the campaign (browser run).
- A second pause while held moved the stop one day later (browser run).
- The checkmark click also reached the galaxy map (browser run).
