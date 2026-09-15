# P58F2 tactical impact effects

P58F2 replaces the target-impact circles with the six source-selected original
type-303 hit, damage, and destruction sequences. The production combat path
now chooses each family from the original hit-message ordering and post-hit
stage, preserves the native priority rules, advances frames at 10 Hz, and
attaches the sprite to the target's source-world position.

## Implemented contract

| Event | Resource family | Frames | Draw size |
|---|---:|---:|---:|
| Standard hit | 3060 | 6 | 32 by 32 |
| Ion hit | 3180 | 6 | 32 by 32 |
| Standard damage | 3120 | 7 | 32 by 32 |
| Ion damage | 3240 | 16 | 64 by 32 |
| Turbolaser damage | 3300 | 7 | 32 by 32 |
| Destruction | 3360 | 16 | 64 by 64 |

- `FUN_005a7500` proves the laser, ion, turbolaser, and torpedo message codes.
- `FUN_005d3e90` proves the event and stage branches, resource bases, and
  state-priority groups.
- `FUN_005d39a0` proves the exact frame counts and draw sizes.
- `FUN_005d41a0` proves `base + frame`, 0.1-second cadence, and expiry.
- The indexed decoder uses the uniform border index as transparent matte and
  preserves nearest-neighbor sampling. Missing or invalid frames remain on the
  logged bounded fallback path.

The detailed executable mapping is retained in the
[Ghidra note](../../../../ghidra/notes/tactical-impact-effect-path.md).

## Verification

| Gate | Result |
|---|---|
| Effect dispatch and cadence tests | 2 passed, 0 failed |
| Indexed effect decoder test | 1 passed, 0 failed |
| Tactical fixture test | 1 passed, 0 failed |
| Workspace tests | 683 passed, 0 failed, 21 ignored |
| Workspace check | Passed |
| Scoped Clippy | Exit 0 with audited baseline warnings only |
| Production fixture exclusion | Passed with zero fixture tokens in the production artifact |
| Tactical browser matrix | 48 of 48 passed |
| Browser isolation | 48 fresh muted launches, 48 four-request starts, 48 closed |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium | Passed all four effect views with no findings |
| Production WASM SHA-256 | `2e657f7b748062a59cecfd15a648539b35f109f58d7d9f72462673320ea7cd19` |
| Fixture WASM SHA-256 | `e8b7ada06e3d16827b315a20ff75cb7430eb7c56ec874d355f307ec7262b5ba1` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f2-tactical-impact-effects/) retains both factions at
640 by 480 and 1280 by 800 letterboxed viewports, their complete result
records, the full 48-case summary, contact sheet, and
[Astra acceptance record](p58f2-tactical-impact-effects/astra-browser-acceptance.json).

## Acceptance boundary

This is qualified A1 implementation evidence. Exact event selection, resource
identity, frame count, timing, priority, browser transport, transparent matte,
and visible target attachment are proven. The retained-mode projectile path,
shield or field families, fighter impact emission, exact planet placement,
sound mapping, lossless original-executable comparison, and all 106 strict
tactical cells remain open.
