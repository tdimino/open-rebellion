# P58-B09 tactical A1 crosswalk

P58-B09 connects the existing deterministic tactical browser catalog to the
canonical 106-cell denominator. It classifies each scenario as a journey,
snapshot, or negative control and maps only behavior that the current probe
actually exercises.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 82 |
| Journey-mapped cells | 64 |
| Snapshot-mapped cells | 18 |
| Cells needing a new scenario | 24 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The original `empty space` cell remains unmapped because the current entry
fixture includes a planet surface. Negative controls cannot claim acceptance
cells. Duplicate, unknown, missing-kind, or negative-control mappings fail the
validator.

The remaining work is grouped into Battle Alert/audio entry, empty-space
presentation, group hover/unassigned/disabled states, selected-object lifecycle
and navigation, camera routes, missing maneuver/tactic/mission commands,
production trench-run launch, authentic withdrawal and Game Options surfaces,
and strategic result persistence. The exact list is retained in the
[artifact bundle](p58-b09-tactical-a1-crosswalk/remaining-cells.json).

## Verification

| Gate | Result |
|---|---|
| Interface-harness unit tests | 26 passed, 0 failed |
| Tactical catalog schema | Passed; 30 scenarios |
| Matrix crosswalk | 82 mapped, 24 missing |
| Scenario split | 64 journey cells, 18 snapshot cells |
| Duplicate/unknown/negative-control claims | Fail closed |
| Interface harness checks | Passed |
| Ledger validator | Passed; 44 families, 569 cells, 106 tactical cells |
| Production fixture exclusion | Passed |
| Proprietary A0 paths tracked | Zero |

This crosswalk makes existing coverage measurable. It does not convert A1
implementation evidence into original visual acceptance, and it does not mark
any ledger cell passed.
