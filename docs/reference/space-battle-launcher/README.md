# Space-battle launcher reference

This is the operational reference for opening and verifying Open Rebellion's
space-battle scene. The [implementation plan](../../plans/2026-09-12-tooling-standalone-space-battle-launcher.md)
owns future work. The [interface audit](../../qa/2026-09-10-interface-parity-audit/README.md)
remains the acceptance authority.

## Entry contract

| Route | Entry | Purpose |
|---|---|---|
| Campaign | `crates/rebellion-app/src/tactical_flow.rs::begin_player_battle` | Validates two opposing fleets at one system, begins the production `TacticalState`, records the cooldown/message, and enters `GameMode::TacticalCombat`. |
| Browser fixture | `crates/rebellion-app/src/tactical_test_fixture.rs::apply` | Creates a deterministic encounter, then calls the same production entry function. It exists only with `interface-test-fixtures`. |
| Renderer | `crates/rebellion-render/src/tactical_view.rs::draw_tactical_view` | Draws the production tactical scene used by both routes. |
| Harness | `tools/interface-parity/run.mjs --battle` | Launches one fresh muted Chromium process per faction, scenario, and viewport, records screenshots/logs/hashes, then closes it. |

Fixture code layout is `(family << 16) | (faction << 8) | scenario`. Tactical
family is `1`; Alliance is `1`; Empire is `2`. The canonical catalog is
[`tactical.catalog.json`](../../../tools/interface-parity/scenarios/tactical.catalog.json).

| Scenario | Alliance | Empire | Contract |
|---|---:|---:|---|
| `battle-entry` | 65793 | 66049 | Production entry with the isolated 3D proof enabled |
| `battle-entry-proof-off` | 65794 | 66050 | Paired negative control |
| `lod-close` | 65795 | 66051 | Resource 2560 |
| `lod-medium` | 65796 | 66052 | Resource 2561 |
| `lod-far` | 65797 | 66053 | Resource 2562 |
| `lod-journey` | 65798 | 66054 | Live 2560, 2561, 2562, 2561, 2560 journey with one family load |
| `camera-journey` | 65799 | 66055 | Source camera entry plus zoom, yaw, pitch, and selected-object target sequence |
| `production-participants` | 65800 | 66056 | Live capital ships, fighters, projected interaction, and system-selected planet |
| `production-participants-3d-off` | 65801 | 66057 | Participant-only framebuffer control with the common planet retained |
| `production-fighter-detail-journey` | 65802 | 66058 | Full indicator, far, and close fighter sequence |
| `production-group-presentation` | 65803 | 66059 | Eight task-force and four RGBY control journeys |
| `production-effect-presentation` | 65804 | 66060 | Six source-selected hit, damage, and destruction frames |
| `production-projectile-field-presentation` | 65805 | 66061 | Retained projectile variants plus tractor and gravity fields |
| `production-selected-damage-presentation` | 65806 | 66062 | Panel 1302, source-ordinal capital portrait, and live shield/hull meters |

## Run it

From `tools/interface-parity`:

```sh
node run.mjs --battle --scenario=battle-entry
node run.mjs --battle --all --scenario=lod-journey --no-build
node run.mjs --battle --all --scenario=camera-journey --no-build
node run.mjs --battle --all --no-build
```

The first command builds current production and fixture WASM artifacts. Use
`--no-build` only after that build succeeds. The harness always supplies
Chromium `--mute-audio`, verifies four-request startup, uses a temporary profile,
and closes every process. Raw runs live under ignored
`.artifacts/interface-parity/`.

## Current verified boundary

- The original 640×480 `TACTICAL.DLL` shell is active.
- Bitmap pause, faction highlight/dim, and zoom controls work with source hit
  masks; zoom has resting and held pressed art.
- All 87 type-301 meshes and 397 type-303 texture/palette resources can be
  staged and decoded from an owned installation.
- Exact resources 2560, 2561, and 2562 plus `SDESTI52.BMP` and `SDESTI_M.BMP`
  form the first source-proven LOD family.
- The source predicate and cached-slot cycle are recovered from
  `FUN_005d26c0`, `FUN_005d3770`, `FUN_005d3650`, and `FUN_005c1160`.
- P57B2A recovers the faction camera, near/far planes, field zoom, handedness
  conversion, and four D-pad commands from `FUN_005c1d30`, `FUN_005d9490`,
  `FUN_005d9640`, and the switch at `0x005d97c0`.
- P57B2B1 activates switch case 9 and the center target resources 1058/1059.
  It centers the selected two-dimensional fallback ship and records the
  isolated proof's selected object before its retained-frame `LookAt`.
- P57B2B2 recovers `FUN_005ab650`'s active-object extent and four lane
  calculations, including its exclusion of docked fighter groups.
- P57B2B3 binds production DAT/roster identities to those lanes, reproduces
  the original X-slot sequence, and targets the selected source world point.
- P57B2C1 preserves the mesh's authored coordinates with only the recovered Z
  reflection, packages palettes 5531 through 5557, and selects
  `5530 + SYSTEMSD.picture_id`.
- P57B2C2A restores `FUN_005d4d10`'s directional RGB `0.8` light frame at
  source `(5,5,-1)`, its Z-constrained origin target, and the separate ambient
  RGB `0.5` light.
- P57B2C2B restores `FUN_005c1c10` and `FUN_005d6e10` device behavior:
  dither off, Gouraud lighting, nearest/no-mip sampling, source
  `D3DCULL_CCW`, LessEqual depth with writes, specular off, and
  diffuse-plus-emissive materials.
- P58A joins every capital ship, fighter, and Death Star DAT identity to the
  original tactical ordinal and resource bases. The four-request pack now
  carries all 87 meshes and 397 textures.
- P58B renders two live production capital participants from exact families
  `2010` and `2510`, preserving source positions and the recovered view state.
- P58C renders the exact type-303 fighter detail resources and uses projected
  capital mesh bounds for selection, focus, targeting, and framing.
- P58D centers a test-only camera on each faction's live fighter, then proves
  all nine independent indicator/far/close transitions through the original
  zoom controls without reloading either family.
- P58E restores all eight task-force controls and four RGBY fighter controls,
  source-shaped input, Ctrl assignment, F1 through F12 routing, and selected
  fighter portraits.
- P58F1 restores `5500 + SYSTEMSD.picture_id` as the production tactical
  planet with its exact `5530 + SYSTEMSD.picture_id` palette. Its current
  left-edge placement is provisional A1 evidence.
- P58F2 restores the six target-attached type-303 hit, damage, and destruction
  families selected by the source event and stage branches. It preserves exact
  frame counts, draw sizes, priority, indexed transparency, and 10 Hz cadence.
- P58F3 restores the three retained projectile mesh variants, exact weapon
  thresholds, scales, material selectors, interpolation and duration. It also
  restores the tractor and gravity frame families, source counts, 10 Hz
  animation, and shared gravity priority.
- P58F4 restores selected-capital panel 1302, maps portraits `2001` through
  `2029` from the source tactical ordinal, removes their lime matte, and
  renders live faction-correct shield and hull meters.
- The complete bundle passes 56 of 56 muted browser cases across both factions
  and the 640x480 and 1280x800 letterboxed viewports with no runtime errors.

This does not accept an original tactical surface. The fixture's zoom-to-depth
bridge, fighter focus, and filled group matrix are test-only. Automatic group
distribution, native GPU and A0 comparison, remaining commands, exact planet
placement, exact field command timing, power allocation, Death Star paths,
results, audio, and return routing remain
open. All
106 `TAC-01` through `TAC-07` cells remain pending in the
[surface ledger](../../qa/2026-09-10-interface-parity-audit/surface-ledger.json).

## Evidence and asset maps

- [Ranked Windows/Ghidra recovery map](reverse-engineering-map.md)
- [P58F4 selected-capital damage evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-selected-damage.md)
- [P58F3 tactical projectile and field evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-projectile-fields.md)
- [P58F2 tactical impact-effect evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-impact-effects.md)
- [P58F1 system-selected planet evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-planets.md)
- [P58E task-force and fighter-group evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-group-presentation.md)
- [P58D tactical fighter-detail journey evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighter-detail-journey.md)
- [P58C tactical fighter and selection evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighters-selection.md)
- [P58B production capital-participant evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-production-participants.md)
- [P58A tactical resource-join evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-resource-join.md)
- [P57B2C2B retained-mode render-state evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-render-state.md)
- [P57B2C2A retained-mode light-rig evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md)
- [P57B2C1 authored transform and system palette evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-transform-palette.md)
- [P57B2B3 participant identity and placement evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-participant-placement.md)
- [P57B2B2 active-force layout evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-battle-layout.md)
- [P57B2B1 selected-object target evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-target-control.md)
- [P57B2A camera and D-pad evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md)
- [P57B1 live LOD evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-live-lod-journey.md)
- [P57A family and predicate](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-lod-family.md)
- [Space-battle graphics inventory](../asset-library/space-battle.md)
- [Native tactical lookup](../asset-library/tactical-lookup.json)
- [Reverse-engineering ledger](../../qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md)
- [Screenshot ledger](../../qa/2026-09-10-interface-parity-audit/screenshot-ledger.md)
