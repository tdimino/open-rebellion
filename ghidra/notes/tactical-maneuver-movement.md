# Tactical maneuver and movement

The saved `REBEXE.EXE` project establishes the maneuver-state producer,
effective-power velocity, and millisecond position integration implemented by
P58F8.

## Maneuver-state contribution

`FUN_005ad750` and `FUN_005afb70` read the active state record at offset
`+0x4c`. A missing record returns `1.0`. An active record returns `9.0 - value`,
clamped to the inclusive range `1.0` through `9.0`.

`FUN_005b17f0` adds that contribution to the CAPSHPSD-derived base sublight
power, subtracts 25 percent of the undamaged total for each engine hit,
subtracts active tractor power, and clamps the result at zero.

## Direction and velocity

The capital constructor `FUN_005b0f70` calls `FUN_0059fb10` to initialize the
current direction to `(0,0,+1)`. Its desired direction remains zero after the
constructor's `FUN_0059faf0` initialization. Production ships therefore do not
move until a command supplies a desired direction.

`FUN_005b2f30` normalizes current and desired directions, clamps their dot
product at a minimum of zero, and multiplies the current forward vector by
that alignment and `FUN_005b17f0`'s effective engine power. The resulting
velocity is stored at offsets `+0x6c` through `+0x74`.

`FUN_005cd640` integrates each component as:

```text
position += velocity * delta_milliseconds * 0.001
```

The production combat loop advances four times per second, so P58F8 uses an
exact 250-millisecond step.

`FUN_005cd2d0` derives the angular difference from the clamped dot product, and
`FUN_005cd460` turns the current direction toward the desired direction. Their
exact turn-rate, collision, and command-delivery branches remain open.

## P58F8 boundary

P58F8 stores current direction, desired direction, velocity, and maneuver-state
value on every production tactical ship. It derives the one-through-nine
bonus, updates velocity from live subsystem mobility, integrates source world
position, and feeds that position back to the production renderer. Unit tests
cover the bonus clamp, aligned motion in both faction directions, and zero
motion for opposing directions. The deterministic browser fixture proves one
250-millisecond step from both signed faction lanes.

Exact global RNG equivalence, interactive command delivery, turning,
collision and formation behavior, and lossless original-runtime pixels remain
open.
