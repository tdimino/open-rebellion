# P58F1 system-selected tactical planets

P58F1 restores the original system-selected planet image to production space
battles. Alliance and Imperial fixtures now request different source planets
from their actual system picture identities and pair each image with its exact
TACTICAL palette.

## Implemented contract

- `FUN_0040b0e0` obtains the current system picture selector through
  `FUN_00509610` and forwards it through the tactical scene construction path.
- `FUN_0059a850` stores that selector, `FUN_00596ad0` returns
  `5500 + selector`, and `FUN_005c2e60` realizes the paired
  `5530 + selector` palette.
- Production uses `SYSTEMSD.picture_id` as that selector. The renderer loads
  one exact 256 by 256 type-303 planet and one exact 256-entry palette, applies
  opaque alpha and nearest sampling, and rejects a mismatched pair.
- Test-only isolated mesh, LOD, and camera proof scenes suppress the planet so
  their established framebuffer and family-load controls remain independent.
  Normal production-participant scenes and their participant-only 3D-off
  controls retain the common planet backdrop.
- The fixture schema records the system picture, planet resource, and palette
  resource. The harness verifies the arithmetic, one load, one scene
  submission, and source-function chain.

The detailed static-analysis chain is retained in the
[Ghidra note](../../../../ghidra/notes/tactical-planet-resource-path.md).

## Verification

| Gate | Result |
|---|---|
| Focused tactical renderer tests | 12 passed, 0 failed, 2 owned-data tests ignored |
| Focused tactical fixture test | 1 passed, 0 failed |
| Workspace tests | 680 passed, 0 failed, 21 ignored |
| Scoped Clippy | Exit 0 with audited baseline warnings only |
| Production fixture exclusion | Passed with zero fixture tokens in the production artifact |
| Tactical browser matrix | 44 of 44 passed |
| Browser isolation | 44 fresh muted launches, 44 four-request starts, 44 closed |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium | Passed the contact sheet and four complete result records with no findings |
| Production WASM SHA-256 | `1433bbdc27fa30faa8e2efbdf957266dee0abc3781aaebf882637cb97c3cef37` |
| Fixture WASM SHA-256 | `b8b4418936d4d3838cd25fc38a137fad9dd74a66b565c188ce297220264dbc2e` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f1-tactical-planets/) retains both faction-selected
planets at 640 by 480 and 1280 by 800 letterboxed viewports, their complete
result records, the full 44-case summary, and the
[Astra acceptance record](p58f1-tactical-planets/astra-browser-acceptance.json).

## Acceptance boundary

This is qualified A1 implementation evidence. Resource identity, palette
pairing, browser transport, and visible production submission are proven. The
left-edge transform remains provisional because available screenshots are not
lossless owned-original baselines. Exact placement and visibility, damage and
effect resources, Death Star states, remaining commands, results, audio, and
all 106 strict tactical cells remain open.
