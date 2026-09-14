# P58C tactical fighters and projected selection

Date: 2026-09-14

Status: implementation gate passed; original-parity acceptance pending

## Result

P58C replaces the synthetic tactical fighter triangles with original type-303
resources and aligns capital-ship selection, targeting, focus lines, and corner
markers with the source-projected mesh bounds. Both faction fixtures contain a
live Mon Calamari Cruiser, Strike Cruiser, A-wing group, and TIE Fighter group.

Static tracing establishes the fighter resource and detail contract:

- `FUN_005c7150` exposes the fighter color state;
- `FUN_005c63f0` initializes retained detail to far and loads the close, far,
  and indicator resources;
- `FUN_005ab650` chooses the player-side and opposing-side resource bases;
- `0x005d4af0` selects close below view span 5, far from 5 through less than
  10, and indicator at 10 or greater. Low detail uses far below 10 and
  indicator at 10 or greater;
- constants are anchored at `0x0066d1d4` (5), `0x0066d1d0` (10), and
  `0x0066d1e4` (0.5).

The representative joins are A-wing DAT 1 to `4024` close, `4029` far, and
`4204` indicator; and TIE Fighter DAT 5 to `4104` close, `4109` far, and
`4204` indicator. The initial recovered view spans of 30.324 and 41.756 select
the authentic 2x2 indicator resource. Close and far resources remain cached for
later camera movement at their native 32x32 and 16x16 dimensions. Rendering
uses nearest filtering and the selected system palette.

## Verification

- Focused Rust tests: 24 passed, 0 failed, 2 ignored across the tactical
  resource, decoder, renderer, and fixture scopes.
- Workspace tests: 671 passed, 0 failed, 21 ignored; doc tests add 3 passed and
  16 ignored.
- Browser gate: 36 of 36 scenarios passed across both factions and the 640x480
  and 1280x800 letterboxed viewports.
- Every browser scenario used exactly four HTTP 200 startup requests, launched
  muted in an isolated process, produced stable paused-frame hashes, recorded
  no browser/runtime errors, and closed its process.
- Per-object framebuffer probes found source-rendered pixels for both capital
  ships and both fighter groups. Paired controls suppress the mapped assets.
- Selection journeys record distinct deselected, selected, and targeted hashes
  for both factions. The harness clicks the source-projected bounds rather than
  the former synthetic 2D positions.
- The production WASM SHA-256 is
  `7ced3077b8737c1d427c038bdcab8b81f9df4e8421a8eae2573272194c208809`.
- The fixture WASM SHA-256 is
  `8dcf085197a226aba370a70a54b5e5ebb83fdaf019d57a0dc39183f3e9997ff8`.
- The runtime pack SHA-256 is
  `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631`.

[Astra medium reviewed every retained image and result](p58c-tactical-fighters-selection/astra-browser-acceptance.json)
and returned qualified A1 acceptance. It confirmed the cockpit, participant
pixels, negative controls, interaction journey, muting, stability, requests,
and error-free records. It also corrected the large viewport description to
1280x800 letterboxed. It did not perform a live rerun.

## Boundary

The evidence is A1 implementation proof only. All 36 comparisons are
unbaselined. It does not establish exact color, position, scale, timing,
animation, or interaction parity with the original executable. Task-force and
squadron panels, close/far camera journeys, damage, effects, planets, Death
Star paths, commands, results, and tactical audio remain open. All 106 strict
`TAC-01` through `TAC-07` cells therefore remain pending until lossless, owned
A0 captures support their comparisons.

The durable artifact inventory is in
[`p58c-tactical-fighters-selection/`](p58c-tactical-fighters-selection/README.md).
