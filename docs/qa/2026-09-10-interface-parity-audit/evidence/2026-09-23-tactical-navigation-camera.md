# P58-B10 tactical navigation and camera

P58-B10 restores the original navigation-set and camera controls to the
production tactical scene. It also expands the command-panel journey to cover
Anvil and Stand Off.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 96 |
| Journey-mapped cells | 78 |
| Snapshot-mapped cells | 18 |
| Cells needing a new scenario | 10 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The production scene now supports all four original navigation-set buttons,
ordered route replacement and append/remove input, ordered multiple targets,
camera memorize/recall, task-force and fighter-group chase shortcuts, and
manual exit from chase. Exact resources `1026` through `1043` provide the
normal, selected, and pressed states. Route and target state advance when the
current waypoint arrives or the current hostile target becomes invalid.

The manual defines the interaction behavior, but the navigation-point world
coordinates are still provisional. All six newly mapped `TAC-04` cells and the
Anvil/Stand Off cells remain pending until A0 comparison passes.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 248 passed, 1 ignored |
| App tests | 16 passed |
| Harness unit tests | 26 passed |
| Tactical catalog and matrix checks | Passed; 96 mapped, 10 missing |
| Navigation/camera browser journey | 4 of 4 passed |
| Anvil/Stand Off browser journey | 4 of 4 passed |
| Startup request count | Four in every browser case |
| Muted launches and browser cleanup | 8 of 8 |
| A0 acceptance | 0 of 106 |

The browser journeys cover Alliance and Empire at 640×480 and letterboxed
1280×800. The ignored raw captures remain under `.artifacts/interface-parity/`;
the durable hashes and run summaries are indexed in the
[artifact bundle](p58-b10-tactical-navigation-camera/README.md).
