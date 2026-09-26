# P58-B08 tactical A0 ingestion

P58-B08 adds the fail-closed path from an owned original-game capture to the
ignored tactical A0 evidence store. It does not add or accept an original
capture.

## Contract

The Windows guest capture script can now bind a stable tactical cell to its
exact ledger requirement, setup identity, ordered input trace, capture method,
Windows build, running process, running executable SHA-256, and lossless PNG
SHA-256. The host ingester independently requires:

- a known `TAC-01` through `TAC-07` cell and byte-identical requirement text;
- the audited English `REBEXE.EXE` SHA-256;
- an unmodified-source manifest and complete VM/display provenance;
- matching 640×480 sidecar and decoded PNG dimensions;
- a valid state setup, timestamp, and non-empty ordered input trace;
- matching image bytes and sidecar SHA-256; and
- a destination confined to `.artifacts/interface-parity/a0/<cell-id>/`.

The first valid capture for a cell is copied without transformation. Repeating
the same record is idempotent; different replacement bytes or metadata fail.
The local manifest is schema-validated before and after ingestion and written
atomically.

## Verification

| Gate | Result |
|---|---|
| Interface-harness unit tests | 25 passed, 0 failed |
| Valid 640×480 record and ingest | Passed |
| Resized PNG rejection | Passed |
| PNG/sidecar hash mismatch rejection | Passed |
| Executable-identity rejection | Passed |
| Ledger-requirement drift rejection | Passed |
| Missing setup/input provenance rejection | Passed |
| Idempotent replay and replacement rejection | Passed |
| Ignored-store confinement | Passed |
| Interface harness checks | Passed |
| Ledger validator | Passed; 44 families, 569 cells, 106 tactical cells |
| Tracked proprietary A0 paths | Zero |

The owned Windows capture host was offline on Tailscale during this checkpoint,
so no original capture was produced or ingested. A0 coverage remains 0/106,
strict tactical acceptance remains 0/106, and C2 remains open. The
[artifact inventory](p58-b08-tactical-a0-ingestion/) contains the
machine-readable boundary.

No proprietary screenshot, save, audio, video, executable, or extracted game
resource is committed.
