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
| 4 | `FUN_005ab650`, `FUN_005a9030`, `FUN_005adfa0`, `FUN_005c1100`, `FUN_005c12a0`, `FUN_00595be0`, `FUN_005c1080`, `FUN_005d9640`; preserve `SYSTEMSD.picture_id` | Production placement on the recovered force-dependent extent and lanes, fighter groups, bounds, planet selection, resource family, production camera distance, and stable selected-object frame binding. Authored coordinates and the handedness transform are proven in P57B2C1. |
| 5 | `TACTICALRESULT_UPDATE`; callers and setters around `FUN_0040a700` states `7`–`9` and `0x15`–`0x17`; `FUN_005445d0_combat_result`; Battle Alert and Death Star callbacks | Results composition, simulate/observe flow, reports, media routing, and strategic return |
| 6 | `FUN_005bae60`; vtable `0x0066c748`; audio-manager callers; TACTICAL WAVE `13000`–`13065`; TEXTTACT `MDATA.401`–`MDATA.406` | Event-to-SFX/voice mapping, faction variants, and the missing tactical-audio staging path |
| 7 | `FUN_00596ad0`, `FUN_005c2e60`, `FUN_005d4d10`, `FUN_005d9eb0`, `FUN_005da150`, `FUN_005d59e0`; retained-mode COM slots | Back plane, quality, filtering, culling, material, and lighting rules. Palette realization is proven in P57B2C1, the light rig in P57B2C2A, and remaining device/material state in P57B2C2B. |
| 8 | Fire/hit/down/fix handlers from the event registry; type-303 families `3020`–`3375`, `3500`–`3527`, `3600`–`3627`, `4200`–`4204`, `5030` | Weapon, shield, impact, explosion, subsystem, repair, and Death Star effect predicates and timing |
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
  representative joins and complete 87-mesh/397-texture pack transport.
- Camera switch case 9 at `0x005d97c0` calls `FUN_00595be0`, stores the
  selected object ID, and clears its cached frame. `FUN_005d9640` then calls
  `FUN_005c1080`, reads the object's frame, and supplies it to `LookAt`.
  P57B2B1 implements this contract with provisional fixture IDs; the stable
  production DAT and tactical identity join remains open.
- `FUN_005ab650` begins with extent 100, counts active capital ships and
  non-docked fighter groups per force, adds three units for the larger force,
  and derives lanes at `+extent/2`, `-extent/2`, `20 - extent/2`, and
  `extent/2 - 20`. P57B2B2 implements and browser-checks that calculation;
  P57B2B3 binds production DAT and fleet-roster identities to those lanes.
- [`FUN_005a9030`](../../../ghidra/notes/FUN_005a9030.c) writes source X/Y/Z
  coordinates and the `-1.0` retained-object field. P57B2B3 reproduces its
  signed-zero alternating X slots, faction lanes, and selected-world-point
  target. P58A completes the DAT-to-tactical-resource family join; production
  family drawing remains open.
- `FUN_005a9030`, `FUN_005adfa0`, and `FUN_005caf70` pass authored coordinates
  directly to retained-mode objects; P57B2C1 therefore removes the provisional
  center-and-fit transform and keeps only Z reflection and winding reversal.
- `FUN_00596ad0` supplies base `5530`; `FUN_005c2e60` loads
  `5530 + SYSTEMSD.picture_id`, copies 256 RGB triples, and realizes flags
  `0x44`. P57B2C1 packages all 27 palettes and browser-proves selectors 1 and 2.
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

## A0-only boundary

Static analysis cannot accept final rasterized pixels, perceived framing,
overlap, uncommon visual states, held-control cadence, random effect and voice
variants, audio ducking/interruption, real two-peer timing, or live campaign
transition routing. Those require owned native-runtime captures and traces.
Compressed screenshots can guide reconstruction but cannot close strict cells.

Use the bounded scripts in [`ghidra/scripts`](../../../ghidra/scripts) against
the saved project. Commit scripts and conclusions, never the saved database,
generated decompilation output, or original proprietary resources.
