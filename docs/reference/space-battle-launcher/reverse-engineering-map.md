# Space-battle reverse-engineering map

This map records what the owned Windows files and saved Ghidra project can
recover for the production space-battle path. It is subordinate to the
[interface audit](../../qa/2026-09-10-interface-parity-audit/README.md) and does
not replace native 640×480 A0 acceptance captures.

## Target relationship graph

```text
surface state or control
  -> constructor, rectangle, and enabled predicate
  -> handler vtable and tactical event
  -> entity DAT identity and tactical ordinal
  -> mesh, texture, bitmap, effect, text, and audio resource
  -> tactical result and strategic return
```

The original files can recover most nodes and edges in this graph. They cannot
alone prove final DirectDraw/Direct3DRM pixels, interaction timing, audio
mixing, or genuine two-peer ordering.

## Ranked static-recovery queue

| Order | Targets | Unlocks |
|---:|---|---|
| 1 | `FUN_005c14d0`; root vtable `0x0066c960`; subordinate vtables `0x0066ca10`, `0x0066c9f8`, `0x0066c9f0`, `0x0066c9e8`, `0x0066c9d8`, `0x0066c9d0`, `0x0066c9c0`, `0x0066c9b8`, `0x0066c9b0`, `0x0066c9a0`, `0x0066c998`, `0x0066c990`; TEXTTACT cross-references | Code-built controls, geometry, visibility/enabled predicates, pause/wait/observe overlays, modals, and actions across `TAC-01`, `TAC-02`, `TAC-03`, `TAC-05`, and `TAC-06` |
| 2 | `FUN_005a7500`; vtables `0x0066c2d0`, `0x0066bdd0`, `0x0066bdb8`, `0x0066c390`, `0x0066c380`, `0x0066c370`; registered event slots | Concrete handlers for groups, targets, missions, formations, navigation, damage, recovery, retreat, pause, results, and Death Star events |
| 3 | `FUN_00597610_ship_db`; registry order consumed by `FUN_005ab650`; `CAPSHPSD.DAT`; `FIGHTSD.DAT` | Complete in P58A: definitive ship/fighter ordinal-to-DAT identity and removal of the approximate `class_to_sprite_id` path |
| 4 | `FUN_005ab650`, `FUN_005a9030`, `FUN_005adfa0`, `FUN_005c1100`, `FUN_005c12a0`, `FUN_00595be0`, `FUN_005c1080`, `FUN_005d9640`, `FUN_005c7150`, `FUN_005c63f0`, `FUN_0059f680`, `FUN_005cfec0`, `0x005d4af0`; preserve `SYSTEMSD.picture_id` | Production placement on the recovered force-dependent extent and lanes, fighter groups, bounds, planet selection, resource family, production camera distance, and stable selected-object frame binding. Authored coordinates and handedness are proven in P57B2C1, live capital-family drawing in P58B, fighter resources plus projected capital interaction in P58C, all independent fighter-detail transitions in P58D, source-backed task-force and RGBY presentation in P58E, and the exact system-selected planet pair in P58F1. Automatic production grouping and exact planet placement remain open. |
| 5 | `TACTICALRESULT_UPDATE`; callers and setters around `FUN_0040a700` states `7`–`9` and `0x15`–`0x17`; `FUN_005445d0_combat_result`; Battle Alert and Death Star callbacks | Results composition, simulate/observe flow, reports, media routing, and strategic return |
| 6 | `FUN_005bae60`; vtable `0x0066c748`; audio-manager callers; TACTICAL WAVE `13000`–`13065`; TEXTTACT `MDATA.401`–`MDATA.406` | Event-to-SFX/voice mapping, faction variants, and the missing tactical-audio staging path |
| 7 | `FUN_00596ad0`, `FUN_005c2e60`, `FUN_005d4d10`, `FUN_005d9eb0`, `FUN_005da150`, `FUN_005d59e0`; retained-mode COM slots | Back plane, quality, filtering, culling, material, and lighting rules. Palette realization is proven in P57B2C1, the light rig in P57B2C2A, and remaining device/material state in P57B2C2B. |
| 8 | `FUN_005a7500`, `FUN_005d39a0`, `FUN_005d3e90`, `FUN_005d41a0`, `FUN_005d3de0`, `FUN_005ee590`, `LAB_005eeb90`; field handlers `FUN_005b23e0` through `FUN_005b25d0`, `FUN_005d3ac0`, `FUN_005d3cc0`; subsystem constructors `FUN_005e45f0`, `FUN_005e7540`, `FUN_005e77c0`; damage path `FUN_005b54d0`, `0x005b1970`, `0x005b1ab0`, `FUN_005b05c0`, `FUN_0040b8a0`; repair and mobility `FUN_005b0330`, `FUN_005b1490`, `FUN_005b16b0`, `FUN_005b1790`, `FUN_005b17f0`; maneuver and movement `FUN_005ad750`, `FUN_005afb70`, `FUN_005b2f30`, `FUN_005cd640`, `FUN_005cd2d0`, `FUN_005cd460`; command panels `FUN_005e4110`, `FUN_005e7030`, `FUN_005dcc70`, `FUN_005dd080`, `FUN_005dd140`, `FUN_005dd290`, `FUN_005dd3a0`, `FUN_005def00`, `FUN_005df110`, `FUN_005df4f0`, `FUN_005df5f0`, `FUN_005df660`, `FUN_005df6b0`; order execution `FUN_005ca6d0`, `FUN_005cef00`, `FUN_005cf1a0`, `FUN_005cf410`, `FUN_005cf680`, `FUN_005cf930`, `FUN_005cf940`, `FUN_005cf980`, `FUN_005d21b0`, `FUN_005cda40`, `FUN_005a9530`, `FUN_005a8f70`; progression `FUN_005b9c60`, `FUN_005ba270`, `FUN_005cf190`, `FUN_005cf8f0`, `FUN_005b1e10`, `FUN_005b84e0`, `FUN_005b8630`, `_DAT_0066d088`; attack acquisition `FUN_005d0b10`, `FUN_005d0bb0`, `FUN_005c91e0`, `FUN_005c91c0`, `FUN_005f5060`, `FUN_005a8fc0`, vtables `0x0066cf48`, `0x0066cf78`; BMP families `1105`–`1195`, `1201`–`1225`, `1308`, `1309`; type-303 families `3020`–`3375`, `3500`–`3527`, `3600`–`3627`, `4200`–`4204`, `5030` | P58F2 completes target-attached hit, damage, and destruction sprite selection and timing. P58F3 completes retained projectile geometry, thresholds, colors, interpolation and duration, plus tractor/gravity frame selection and animation. P58F5 completes subsystem resources, quantization, placement, exact field-source identity, capacity, priority, and frame reset. P58F6 completes live shield overflow, hull loss, subsystem selection and limits, condition recalculation, and tractor cancellation. P58F7 completes repair cadence and selection plus engine-condition and active tractor-drag mobility. P58F8 completes maneuver-state production, effective-power velocity, signed movement, and millisecond position integration. P58F9 completes authentic command-panel presentation, exact shared order and tactic codes, disabled states, and commit/cancel delivery. P58F10 completes the four maneuver waypoint constructors, Hold stop behavior, and the first Recover carrier state. P58F11 completes source-rate signed turning, deterministic waypoint arrival, and the Returning, Docking, and Recovered fighter states. P58F12 completes typed Attack Fighters and Attack Capital Ships target acquisition for both capital and fighter owners. Exact global RNG sequencing, exact original arrival callbacks and recovery trajectories, weapon-resolution callbacks, target-loss reacquisition, collision and formation behavior, fighter emission, and Death Star effects remain open. |
| 9 | Fixture schema and shared result path | Stable seed/clock, explicit system/fleet/class DAT IDs, original ordinal/resources, state fingerprints, event trace, losses, winner, reports, and destination |
| 10 | `FUN_005f9860`, `FUN_005f7ea0`, DirectPlay packet/ack helpers and imports | Original wait, pause, departure, timeout, and synchronization semantics; not a standalone-launcher prerequisite |

## Proven starting points

- [`FUN_005ab650`](../../../ghidra/notes/FUN_005ab650.c) maps 29 capital-ship
  and eight fighter ordinals to tactical resource families and selects Death
  Star bases `5010` and `5020`.
- [`FUN_00597610_ship_db`](../../../ghidra/notes/FUN_00597610_ship_db.c)
  supplies the original ordered ship/fighter names and relevant vtables.
- [`FUN_005c14d0`](../../../ghidra/notes/FUN_005c14d0.c) constructs the
  640×480, 8-bit tactical window, subordinate handlers, TEXTTACT labels, and
  tactical palette activation.
- [`FUN_005a7500`](../../../ghidra/notes/FUN_005a7500.c) is the tactical battle
  manager/controller registry. It is not merely an individual ship constructor.
- [`FUN_005c2e60`](../../../ghidra/notes/FUN_005c2e60.c) selects and realizes a
  type-303 palette. `TACTICAL.DLL` also contains 66 WAVE resources (`13000`–
  `13065`) that the current audio staging path does not yet ingest.
- The current fixture selects exact system DAT identities for its two faction
  cases and emits stable system, participant, layout, palette, tactical
  ordinal, and resource-base identities. P58A browser-proves four
  representative joins and complete 87-mesh/397-texture pack transport. P58B
  browser-proves production drawing for joined capital families `2010` and
  `2510` at their source positions. P58C browser-proves exact fighter resource
  triplets and projection-aligned capital interaction. P58D browser-proves the
  full production indicator/far/close transition sequence without family
  reload. P58E browser-proves all eight task-force and four RGBY controls,
  selected portraits, source-shaped hit testing, and keyboard routes while
  keeping automatic production grouping unclaimed.
- Camera switch case 9 at `0x005d97c0` calls `FUN_00595be0`, stores the
  selected object ID, and clears its cached frame. `FUN_005d9640` then calls
  `FUN_005c1080`, reads the object's frame, and supplies it to `LookAt`.
  P57B2B1 implements this contract. P57B2B3 supplies stable production
  participant IDs, P58A joins their tactical resources, and P58B draws the
  capital families. P58C uses the projected family bounds for capital
  selection, targeting, focus lines, and corner framing.
- `FUN_005ab650` begins with extent 100, counts active capital ships and
  non-docked fighter groups per force, adds three units for the larger force,
  and derives lanes at `+extent/2`, `-extent/2`, `20 - extent/2`, and
  `extent/2 - 20`. P57B2B2 implements and browser-checks that calculation;
  P57B2B3 binds production DAT and fleet-roster identities to those lanes.
- [`FUN_005a9030`](../../../ghidra/notes/FUN_005a9030.c) writes source X/Y/Z
  coordinates and the `-1.0` retained-object field. P57B2B3 reproduces its
  signed-zero alternating X slots, faction lanes, and selected-world-point
  target. P58A completes the DAT-to-tactical-resource family join; P58B renders
  the joined capital families. P58C renders fighter detail resources at the
  recovered positions and moves capital interaction to source projections.
- `FUN_005a9030`, `FUN_005adfa0`, and `FUN_005caf70` pass authored coordinates
  directly to retained-mode objects; P57B2C1 therefore removes the provisional
  center-and-fit transform and keeps only Z reflection and winding reversal.
- `FUN_0040b0e0` obtains the current system picture selector through
  `FUN_00509610`, then forwards it through `FUN_00595d60` to
  `FUN_0059a850`. `FUN_00596ad0` returns type-303 planet
  `5500 + selector`; `FUN_005c2e60` adds 30, copies 256 RGB triples from the
  paired palette, and realizes flags `0x44`. P57B2C1 packages all 27 palettes.
  P58F1 binds the selector to `SYSTEMSD.picture_id` and browser-proves planets
  5501 and 5502 with palettes 5531 and 5532. Exact retained-frame placement
  and visibility remain open.
- `FUN_005d4d10` creates directional light type `3` at RGB `0.8`, positions
  its frame at source `(5,5,-1)`, aims at the origin with
  `D3DRMCONSTRAIN_Z`, and adds ambient light type `0` at RGB `0.5`.
  P57B2C2A preserves that contract in the WebGL/Metal material and browser
  evidence.
- `FUN_005d6e10` creates the retained-mode device from Direct3D.
  `FUN_005c1c10` calls `SetDither(FALSE)` and does not override shades,
  render quality, or texture quality. P57B2C2B preserves the resulting Gouraud,
  nearest/no-mip, `D3DCULL_CCW`, LessEqual depth, depth-write, specular-off,
  diffuse, and emissive contract in WebGL and Metal.
- `FUN_00597610_ship_db` constructs the source-named capital-ship and fighter
  registry in the ordinal space consumed by `FUN_005ab650`. P58A correlates
  those unique names with `CAPSHPSD` and `FIGHTSD`, implements every join, and
  fails closed for unknown or cross-family DAT IDs.
- P58B installs all 87 meshes and 397 textures in the production renderer,
  loads only joined capital families used by the battle, and keeps missing
  families on the explicit fallback path.
- `FUN_005c63f0` loads close, far, and indicator fighter resources;
  `FUN_005c7150` supplies color state; and `0x005d4af0` selects close below
  view span 5, far from 5 through less than 10, and indicator at 10 or greater.
  P58C implements that bounded contract. P58D proves all nine independent
  transitions caused by the two recovered fighter lanes through the original
  zoom controls.
- `FUN_005a7500` proves laser, ion, turbolaser, and torpedo hit-message codes.
  `FUN_005d3e90` maps those codes and the post-hit stage to six state/resource
  families. `FUN_005d39a0` supplies exact frame counts and draw sizes, and
  `FUN_005d41a0` advances them every 0.1 seconds. P58F2 implements the target
  sprites. `FUN_005d3de0`, `FUN_005ee590`, and `LAB_005eeb90` establish the
  retained projectile mesh, source-to-live-target interpolation, and duration.
  `FUN_005d3ac0` and `FUN_005d3cc0` establish the shared 3520/3620 field slot,
  resources, and 10 Hz cadence. P58F3 implements those presentation contracts
  while keeping combat-command delivery open. `FUN_005e45f0`, `FUN_005e7540`,
  and `FUN_005e77c0` establish the five subsystem families, placement, and
  percentage bands. `FUN_005b23e0` through `FUN_005b25d0` retain exact field
  source IDs and source-side capacity. P58F5 implements those contracts while
  keeping combat-driven subsystem mutation and interactive commands open.
- `FUN_005b54d0` establishes live shield-loss probability, negative shield
  overflow, hull loss, the signed hull-delta score, and all five subsystem
  selection thresholds. Capital vtable target `0x005b1970` applies hits,
  `0x005b1ab0` removes them during repair, and `FUN_0040b8a0` counts the normal
  and damaged hyperdrive fields. P58F6 implements live damage and tractor
  cancellation.
- `FUN_005b0330` and `FUN_005b1490` establish the repair timer, inclusive
  damage-control roll, uniform outstanding-hit selection, and subsystem order.
  `FUN_005b16b0`, `FUN_005b1790`, and `FUN_005b17f0` establish engine condition,
  effective tractor power, and tractor drag. P58F7 implements those contracts.
- `FUN_005ad750` and `FUN_005afb70` establish the maneuver-state bonus.
  `FUN_005b2f30` establishes aligned effective-power velocity, and
  `FUN_005cd640` integrates it using millisecond deltas. P58F8 implements those
  contracts. `FUN_005dcc70` and `FUN_005def00` build the maneuver and mission
  panels; their handlers preserve the shared order at `+0x68`, tactic at
  `+0x38`, disabled states, and commit/cancel semantics. P58F9 implements that
  assignment boundary. `FUN_005ca6d0` dispatches committed orders to distinct
  executor vtables. P58F10 implements the four recovered maneuver waypoint
  constructors, Hold stop behavior, and Recover carrier reservation through
  Returning state 2. `FUN_005b9c60`, `FUN_005ba270`, `FUN_005cd460`,
  `FUN_005cf980`, and `FUN_005b8630` establish turn rate, signed rotation,
  docking threshold, and recovery completion. P58F11 implements those bounded
  contracts. `FUN_005d0b10` and `FUN_005d0bb0` walk the stable tactical
  object list and dispatch event `0x36` for Attack Capital Ships and Attack
  Fighters. P58F12 implements typed first-eligible acquisition for capital and
  fighter owners. Exact global RNG sequencing, original arrival callbacks and
  recovery trajectories, weapon-resolution callbacks, target-loss
  reacquisition, collision, and formation behavior remain open.

## A0-only boundary

Static analysis cannot accept final rasterized pixels, perceived framing,
overlap, uncommon visual states, held-control cadence, random effect and voice
variants, audio ducking/interruption, real two-peer timing, or live campaign
transition routing. Those require owned native-runtime captures and traces.
Compressed screenshots can guide reconstruction but cannot close strict cells.

Use the bounded scripts in [`ghidra/scripts`](../../../ghidra/scripts) against
the saved project. Commit scripts and conclusions, never the saved database,
generated decompilation output, or original proprietary resources.
