# P58-B15 tactical Battle Alert and audio evidence

This bundle records the source-backed pre-battle alert, tactical score, event
cue, and complete deterministic A1 crosswalk checkpoint.

The later [P58-B16 correction](../2026-09-24-tactical-weapon-audio.md)
identifies event `0x14` as torpedo impact rather than ship destruction and
restores the full WAVE `13033–13054` weapon bank.

## Inventory

- [`summary.json`](summary.json) records the matrix result, source contract,
  focused browser runs, verification gates, and representative hashes.
- [`browser-acceptance.json`](browser-acceptance.json) records the independent
  browser-only result without embedding proprietary screenshots.

Raw browser screenshots, console logs, network ledgers, and per-case results
remain in the ignored `.artifacts/interface-parity/` tree. No original-game
capture, audio, executable, DLL, or proprietary runtime asset is committed.
This is complete A1 scenario mapping, not strict A0 acceptance.
