# P58E tactical task-force and fighter-group presentation

P58E restores the original top task-force and fighter-group strips for both
factions. It also connects source-backed group selection, keyboard routing,
and selected fighter detail presentation to the production tactical view.

## Implemented contract

- The HUD uses TACTICAL resources `1001` and `1004` for faction task-force
  headers, `1005` through `1007` for task-force states, `1008` and `1010` for
  fighter headers, `1012` through `1019` for RGBY states, and `1020` for an
  unassigned fighter slot.
- Eight task-force controls occupy the recovered 25 by 17 source rectangles at
  `x = 60 + 26n`, `y = 2`. Four fighter controls use the same dimensions at
  `x = 330 + 26n`, `y = 2`. Transparent source pixels do not receive clicks.
- Task-force and fighter-group selection are mutually exclusive. `F1` through
  `F8` select task forces. `F9` through `F12` select Red, Blue, Green, and Gold.
- Ctrl-selection can add player ships to an existing task force or create only
  the next blank task-force slot, matching the manual's assignment rule.
- Player fighter resources select the recovered RGBY family offsets. Opposing
  fighters retain the white family. Selected fighters use TACTICAL `1307` and
  their exact `2030` through `2037` detail portrait.
- The tactical pass prewarms its complete Latin glyph set at every active text
  size before drawing. This prevents Macroquad 0.4.14 from invalidating a
  queued WebGL font-atlas texture at scaled viewports.

The preserved manual, pages 143 and 148, supplies the task-force, fighter-group,
Ctrl-selection, and keyboard contract. Ghidra traces through `FUN_005c7150`,
`FUN_005c63f0`, `FUN_0059f680`, `FUN_005ab650`, and `FUN_005cfec0` establish the
fighter group field, unassigned value `4`, valid RGBY values `0` through `3`,
and side-dependent resource selection.

## Source-safe boundary

The original automatic task-force and fighter-group distribution algorithm is
not yet proven. Production therefore initializes capital ships in the first
task force and fighters as unassigned. The deterministic interface fixture
alone fills all eight task forces and four fighter groups so every visible
state can be tested without inventing production behavior.

## Verification

| Gate | Result |
|---|---|
| Focused tactical renderer tests | 25 passed, 0 failed, 1 owned-data test ignored |
| Tactical fixture tests | 5 passed, 0 failed |
| Workspace tests | 679 passed, 0 failed, 21 ignored |
| Scoped Clippy | Exit 0 with audited baseline warnings only |
| Tactical browser matrix | 44 of 44 passed |
| Browser isolation | 44 fresh muted launches, 44 four-request starts, 44 closed |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium | Passed after inspecting 32 P58E screenshots and all four result records |
| Production WASM SHA-256 | `544614187aa075db979c9a34c38c103c162144944583577c02b7c815ee90dc49` |
| Fixture WASM SHA-256 | `358601fc09e563abe1b4fb83e49b8cf9716366ecc4c52db2eb1516ff8fff0a56` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58e-tactical-group-presentation/) retains both factions
and both viewports at rest, held, selected, and keyboard-selected states. It
also retains exact transparent-matte probes and the complete result records.
The [Astra record](p58e-tactical-group-presentation/astra-browser-acceptance.json)
documents the visual and runtime-evidence verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. It does not provide a lossless
owned-original A0 baseline, prove automatic production group assignment, or
accept any of the 106 strict tactical cells. Damage, effects, planets, Death
Star states, remaining commands, results, audio, and strategic return remain
open.
