# P58-B11 tactical trench-run launch and strategic persistence

P58-B11 extends two existing production journeys instead of introducing new
fixture-only screens. The Alliance fighter mission now proves that committed
order `6` enters the live trench-run lifecycle, while the Battle Results route
proves that capital and fighter losses are applied to the strategic world
before either destination opens.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 98 |
| Journey-mapped cells | 80 |
| Snapshot-mapped cells | 18 |
| Cells needing a new scenario | 8 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The Alliance journey selects the original Attack Death Star control, commits
one selected fighter group to source order `6`, and records an active
production trench run. The Empire journey verifies that the same control stays
disabled against its friendly Death Star and commits no order.

The Battle Results fixture now carries three capital ships and three fighter
squadrons per side so the operational, damaged, and destroyed categories are
all real roster states. On return, the production integrator removes one
destroyed capital ship and one destroyed fighter squadron from both strategic
fleets. The Alliance route opens the system destination and the Empire route
opens the fleet destination only after this mutation.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 248 passed, 1 ignored |
| App tests | 16 passed |
| Harness unit tests | 26 passed |
| Tactical catalog and matrix checks | Passed; 98 mapped, 8 missing |
| Attack Death Star browser journey | 2 of 2 passed |
| Battle Results persistence journey | 2 of 2 passed |
| Startup request count | Four in every browser case |
| Muted launches and browser cleanup | 4 of 4 |
| Independent browser review | No P0 or P1 finding |
| A0 acceptance | 0 of 106 |

The review retained two P2 limits. The Empire fleet destination does not yet
show an unmistakable fleet roster, and the Alliance Death Star is clipped near
the top of the tactical aperture. These journeys are bounded A1 evidence, not
lossless original-game comparison or 106-cell acceptance.

Raw screenshots, network records, console logs, and per-case JSON remain in
the ignored `.artifacts/interface-parity/` tree. Durable hashes and run
summaries are indexed in the
[artifact bundle](p58-b11-tactical-trench-persistence/README.md).
