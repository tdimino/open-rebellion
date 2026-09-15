# Tactical subsystem damage path

The saved `REBEXE.EXE` project establishes how live damage changes the five
selected-capital subsystem indicators. This note records the bounded findings
implemented by P58F6.

## Damage dispatch

`FUN_005b54d0` subtracts incoming damage from shield energy first. If the ship
had positive shields, the shield-generator hit probability is the percentage
of the prior shield value removed by the hit, with a minimum probability of
one. The executable draws an inclusive integer from 0 through 100 and damages
the shield subsystem when the draw is less than or equal to that probability.

Negative shield overflow transfers to hull. The crossing branch applies at
least one hull point, clears shield energy to zero, and computes this score:

```text
inclusive_random_0_to_100 + ((new_hull - old_hull) * 100 / old_hull)
```

Only scores greater than 50 damage a subsystem. The exact upper bounds are:

| Score | Category passed to vtable slot `+0xa0` |
|---|---:|
| 50 or less | None |
| Greater than 50 through 65 | Shield generator, 0 |
| Greater than 65 through 80 | Weapons, 1 |
| Greater than 80 through 90 | Tractor beam, 2 |
| Greater than 90 through 95 | Sublight engines, 3 |
| Greater than 95 through 100 | Hyperdrive, 4 |

## Counters and component limits

Capital-ship vtable slot `+0xa0` resolves to the routine at `0x005b1970`. It
increments these source fields:

| Category | Object field | Hit limit |
|---|---:|---:|
| Sublight engines | `+0x36c` | 4 |
| Shield generators | `+0x370` | 4 |
| Hyperdrive | `+0x374` | Installed component count |
| Tractor beam | `+0x378` | 4 |
| Weapons | `+0x37c` | 4 |

The tactical constructor at `FUN_005b05c0` stores the hyperdrive component
count at `+0x3ec`. The caller at `FUN_0040b8a0` computes it as the number of
nonzero CAPSHPSD `hyperdrive` and `hyperdrive_if_damaged` fields. Original
ships with both fields therefore accept two hyperdrive hits. A tractor hit
that reduces available tractor power to zero cancels that source's live
tractor command. The inverse repair routine is at `0x005b1ab0`.

## Condition formulas

`FUN_005b1770` scales a maximum subsystem value by current hull divided by
maximum hull. Shield generators, weapons, and tractor beam subtract 25 percent
of their maximum value per hit and clamp at zero. Their ratio getters are
`FUN_005b1710`, `FUN_005b1680`, and `FUN_005b1740`.

`FUN_005b17f0` establishes a separate sublight rule. It does not scale the
target's engine power by hull. It subtracts 25 percent of base power plus the
current mode bonus for each engine hit, then subtracts the effective tractor
power of every source holding the ship. `FUN_005b16b0` returns that value over
base engine power. P58F7 supersedes the earlier hull-scaling interpretation;
see [the repair and mobility note](tactical-subsystem-repair-mobility.md).

`FUN_005b1bc0` subtracts the hyperdrive hit count from its installed component
count. `FUN_005b16e0` divides the remainder by the component count. The HUD
multiplies all five ratios by 100 and truncates them before the P58F5 bitmap
quantizer selects resources 1201 through 1225.

## P58F6 boundary

P58F6 routes live capital-ship and fighter damage through the recovered shield,
hull, threshold, counter, component-capacity, condition, and tractor-cancel
branches. The Rust session retains deterministic 0-through-100 draws using its
existing combat seed. P58F7 adds the recovered repair cadence and selection
plus engine and active tractor condition penalties. Joining the executable's
global RNG sequence, the maneuver-bonus producer, physical movement
integration, interactive command delivery, and lossless original-runtime
comparison remains open.
