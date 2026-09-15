# Tactical impact-effect path

The saved `REBEXE.EXE` Ghidra project establishes the target-attached tactical
effect dispatcher and its frame lifecycle.

## Event and resource mapping

`FUN_005a7500` registers the ship-hit messages in this order:

| Message | Dispatcher code |
|---|---:|
| `SHIP_TAKE_LASER_HIT` | 1 |
| `SHIP_TAKE_ION_HIT` | 2 |
| `SHIP_TAKE_TURBO_HIT` | 3 |
| `SHIP_TAKE_TORPEDO_HIT` | 4 |

`FUN_005d3e90` combines that code with the post-hit stage:

| Stage | Weapon branch | State bit | Type-303 base | Frames | Draw size |
|---|---|---:|---:|---:|---:|
| Hit | ion | `0x04` | 3180 | 6 | 32 by 32 |
| Hit | other | `0x08` | 3060 | 6 | 32 by 32 |
| Damage | ion | `0x10` | 3240 | 16 | 64 by 32 |
| Damage | turbolaser | `0x40` | 3300 | 7 | 32 by 32 |
| Damage | other | `0x20` | 3120 | 7 | 32 by 32 |
| Destroyed | all | `0x80` | 3360 | 16 | 64 by 64 |

The dispatcher rejects an incoming state whose bit value is not greater than
the active state. States in `0x04`/`0x08` are mutually exclusive, as are states
in `0x10`/`0x20`/`0x40`. Destruction can replace a lower state.

## Timing and rendering boundary

`FUN_005d39a0` preloads the exact bases, frame counts, and draw sizes.
`FUN_005d41a0` advances `base + frame` every `0.1` seconds and removes the
effect after the stated count. The `0.1` constant is the little-endian float at
`0x0066d154`.

The flight path is a different retained-mode object. Vtable slot 8 at
`FUN_005d3de0` constructs it through `FUN_005ee590`; it is not one of these
type-303 frames. Open Rebellion therefore retains its bounded procedural beam
until that geometry path is recovered. The two looped families at 3520 and
3620 are also left open because their shield, tractor, or gravity event
semantics are not yet authoritative.

This note records conclusions only. The saved Ghidra database, generated
decompilation output, and proprietary resource bytes remain untracked.
