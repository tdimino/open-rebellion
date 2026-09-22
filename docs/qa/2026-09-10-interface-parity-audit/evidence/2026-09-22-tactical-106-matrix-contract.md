# P58-B07 tactical 106-cell matrix contract

P58-B07 completes C1 of the tactical acceptance train. It replaces a narrative
106-cell target with an executable contract generated directly from the
authoritative surface ledger.

## Contract

The validator requires exactly:

| Family | Cells |
|---|---:|
| TAC-01 | 13 |
| TAC-02 | 13 |
| TAC-03 | 20 |
| TAC-04 | 14 |
| TAC-05 | 23 |
| TAC-06 | 9 |
| TAC-07 | 14 |
| Total | 106 |

Unknown or duplicate catalog mappings fail immediately. Strict mode also fails
on any missing mapping or original capture. The A0 manifest schema requires the
audited original executable hash, unmodified English source, 640×480 lossless
client-area PNGs at DPR 1, VM and display provenance, ordered input traces,
artifact hashes, and paths restricted to the ignored local A0 store.

`106/106 runnable` and `106/106 accepted` are separate gates. The former means
every cell has a deterministic scenario and authoritative original capture.
The latter additionally requires every canonical ledger cell to be passed.

## Verification

| Gate | Result |
|---|---|
| Harness unit tests | 21 passed, 0 failed |
| Tactical denominator | 106 |
| Family distribution | 13/13/20/14/23/9/14 |
| Current catalog mappings | 0/106 |
| Current A0 captures | 0/106 |
| Current strict acceptance | 0/106 |
| Interface ledger validator | Passed; 44 families, 569 cells, 106 tactical cells |
| Tracked-path exclusion | Passed; all tracked paths checked, zero forbidden A0 paths |
| Production fixture exclusion | Passed; zero production fixture tokens |
| Diff check | Passed |

The zeroes are intentional. C1 defines and guards the denominator; C2 supplies
original captures and C5 supplies exact deterministic scenario mappings. No
cell is accepted by inference or by the existing 120-case implementation gate.

The [artifact inventory](p58-b07-tactical-matrix-contract/) contains the
machine-readable checkpoint summary. No proprietary screenshot, save, audio,
video, executable, or extracted resource is committed.
