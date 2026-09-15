# P58F3 tactical projectiles and fields

P58F3 replaces the procedural tactical beam path with the retained projectile
mesh constructed by the original executable and restores the original tractor
and gravity field frame families. Production combat now uses the recovered
weapon thresholds, shapes, scales, material selectors, source-to-target
interpolation, and one- or two-second lifecycles. Fields retain multiple source
counts, animate at 10 Hz, and preserve gravity-over-tractor priority.

## Implemented contract

| Family | Source contract |
|---|---|
| Laser | Red or green mesh, 28.8 threshold, variants 1 or 2, scale 0.5, one second |
| Turbolaser | Red or green mesh, 34.666668 threshold, variants 1 through 3, scale 0.65 through 1.0, one or two seconds |
| Ion | Blue mesh, 32.0 threshold, variants 1 or 3, scale 0.4 or 1.0, one or two seconds |
| Torpedo | Red or green mesh, 12.8 threshold, variants 1 or 3, scale 0.2 or 1.0, one or two seconds |
| Tractor field | Resources 3620 through 3627, 128 by 128, eight frames at 10 Hz |
| Gravity field | Resources 3520 through 3527, 128 by 128, eight frames at 10 Hz and visible priority over tractor |

The exact twelve projectile vertices and all three index lists are retained.
WebGL drops the original 0.001-unit faces at common camera distances, while old
Direct3D Retained Mode gave them visible line coverage. The renderer therefore
submits the exact mesh and restores a one-device-pixel floor along the same
projected axis. The browser gate requires red, green, and blue framebuffer
pixels around the renderer-reported positions.

The detailed executable mapping is retained in the
[Ghidra note](../../../../ghidra/notes/tactical-projectile-field-path.md).

## Verification

| Gate | Result |
|---|---|
| Focused tactical renderer tests | 33 passed, 0 failed, 1 owned-data test ignored |
| Tactical fixture test | 1 passed, 0 failed |
| Workspace tests | 687 passed, 0 failed, 21 ignored |
| Workspace check | Passed |
| Scoped strict Clippy | Passed for the touched render and app crates after allowing three recorded repository-baseline lints |
| Production fixture exclusion | Passed with zero fixture tokens in the production artifact |
| Focused tactical browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 52 of 52 passed |
| Browser isolation | 52 fresh muted launches, 52 four-request starts, 52 closed |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Projectile framebuffer proof | All three material selectors produced visible pixels in both factions and viewports |
| Astra medium browser review | Passed for the retained four-surface visual evidence with no blocking findings |
| Production WASM SHA-256 | `cc747eb29d5924b33ae4d565707e89a08aba16405a2481684f6a931ce2bbd02e` |
| Fixture WASM SHA-256 | `c32c17935426f9819cfbd930ea70c02caf20ce49cd81ddd7c5ab7bfd2ce99717` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f3-tactical-projectile-fields/) retains both factions
at 640 by 480 and 1280 by 800 letterboxed viewports, their complete result
records, the full 52-case summary, contact sheet, and Astra acceptance record.

## Acceptance boundary

This is qualified A1 implementation evidence. Projectile construction,
resource identity, field animation, source counts, priority, browser transport,
and visible production-path submission are proven. Exact combat-command field
timing, fighter emission, special weapons, selected-unit damage presentation,
audio, lossless original-executable comparison, and all 106 strict tactical
cells remain open.
