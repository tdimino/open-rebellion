# P58-B14 tactical detail and Escort

P58-B14 adds deterministic A1 coverage for a destroyed capital ship, the
selected-ship contents aperture, and the original direct Escort command. It
uses the existing tactical bitmap shell and source-defined command semantics.
It does not substitute a new menu or claim strict original-game acceptance.

## Source contract

| Source | Recovered behavior |
|---|---|
| TACTICAL `1302` | Selected-capital detail panel and two compact contents apertures at panel-local `(10, 128)` and `(73, 128)` |
| GOKRES compact assignment families | Major-character DAT indices map through offset `17920`; minor-character indices map through offset `18176` |
| TACTICAL `3368` | Destroyed-capital visual effect used by the deterministic presentation fixture |
| Original manual | Escort is assigned by right-clicking a friendly capital ship, not through the Missions panel |
| `FUN_005ca6d0` | Direct Escort dispatch uses order code `1` and retains the selected friendly target |
| `FUN_005d0af0`, `FUN_005cf910`, `FUN_005cf920` | Escort target state is installed, retained, and cleared with target validity |
| `FUN_005ceda0`, `FUN_005cee20` | Order code and target state participate in tactical command serialization |

Selected capital ships display up to two compact GOKRES-backed assignments in
the original 1302 apertures. A direct right-click assigns selected capital and
fighter units to follow the friendly target while retaining autonomous
opportunity fire. The protected target receives the original white target box;
the link clears when that target is destroyed or retreats.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 104 |
| Journey-mapped cells | 85 |
| Snapshot-mapped cells | 19 |
| Cells needing a new scenario | 2 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The focused browser journey proves the selected-capital contents resource,
the complete destroyed effect, direct projected right-click assignment, order
code `1`, the white target marker, stable follow behavior, both factions, and
native plus letterboxed viewports. Every run uses four-request startup, muted
audio, no browser errors, and a closed browser process.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 253 passed, 1 ignored |
| App tests | 16 passed |
| Harness unit tests | 26 passed |
| Tactical catalog and matrix checks | Passed; 104 mapped, 2 missing |
| Detail/Escort browser journey | 4 of 4 passed |
| Startup request count | Four in every case |
| Muted launches and browser cleanup | 4 of 4 |
| Independent visual review | Pass; no P0–P3 visual finding |
| A0 acceptance | 0 of 106 |

Raw screenshots, network records, console logs, and per-case JSON remain in
the ignored `.artifacts/interface-parity/` tree. Durable hashes and summaries
are indexed in the
[artifact bundle](p58-b14-tactical-detail-escort/README.md).
