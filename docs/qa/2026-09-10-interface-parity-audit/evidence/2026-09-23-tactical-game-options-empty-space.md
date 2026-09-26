# P58-B12 tactical Game Options and empty-space presentation

P58-B12 restores the original COMMON `20002` Game Options surface as the
shared destination from the shuttle, command center, and tactical Battle
Options panel. It also adds a deterministic empty-space battle that retains
the original starfield while omitting the planet.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 100 |
| Journey-mapped cells | 81 |
| Snapshot-mapped cells | 19 |
| Cells needing a new scenario | 6 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The in-battle Game Options journey verifies the original background and
disabled tactical-display controls, then returns to the same live battle.
Display flags default to the original on state and drive starfield, planet,
pyrotechnic, and model-detail presentation outside the mid-battle-disabled
context. The empty-space snapshot proves the starfield remains present with no
planet draw.

Restart and exit confirmation dialogs, complete save/load naming and error
semantics, and authoritative A0 comparison remain outside this checkpoint.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 251 passed, 1 ignored |
| App tests | 16 passed |
| Harness unit tests | 26 passed |
| Tactical catalog and matrix checks | Passed; 100 mapped, 6 missing |
| Game Options browser journey | 2 of 2 passed |
| Empty-space browser snapshot | 2 of 2 passed |
| Command-center Game Options regression | 2 of 2 passed |
| Startup request count | Four in every browser case |
| Muted launches and browser cleanup | 6 of 6 |
| Independent visual review | No P0, P1, or P2 finding |
| A0 acceptance | 0 of 106 |

Raw screenshots, network records, console logs, and per-case JSON remain in
the ignored `.artifacts/interface-parity/` tree. Durable hashes and summaries
are indexed in the
[artifact bundle](p58-b12-tactical-game-options-empty-space/README.md).
