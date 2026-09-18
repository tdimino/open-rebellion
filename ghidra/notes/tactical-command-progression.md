# Tactical command progression

P58F11 completes the source-traced capital-turn, maneuver-arrival, and fighter
recovery state transitions recovered from the owned English `REBEXE.EXE`.

Executable SHA-256:
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.

## Capital turn and movement

`FUN_005ba270` converts raw capital maneuverability to an effective value of
`raw + 1`, clamped at nine. `FUN_005b9c60` installs the slowest-hull fallback
of pi over twelve radians per second. Faster hulls use:

```text
turn_rate = pi * 0.5 * effective_maneuverability / 9
```

`FUN_005cd460` compares current and desired directions in the source XZ plane,
applies `turn_rate * delta_milliseconds * 0.001`, normalizes the result, and
snaps to the desired direction when the available step covers the remaining
angle. `FUN_005b2f30` then derives velocity from effective engine power and
alignment. `FUN_005cd640` integrates position in milliseconds.

`FUN_005cf190` and `FUN_005cf8f0` are maneuver-executor completion paths. The
current implementation completes a finite waypoint when the remaining distance
fits the next integration step, snaps to the waypoint, and clears the order,
desired direction, waypoint, and velocity. That deterministic crossing rule is
a bounded reconstruction. The exact original target-arrival callback chain is
not yet fully recovered.

## Fighter recovery

`FUN_005cf980` advances a recovering fighter in state 2 toward its reserved
carrier. `_DAT_0066d088` is the strict docking-distance threshold `2.0`. A
fighter closer than that threshold enters state 3 and emits source event
`0x1a`. The callback in `FUN_005b8630` enters state 4 and removes the tactical
object. `FUN_005b84e0` installs state 1, `FUN_005b1e10` installs state 2, and
`FUN_005ba2f0` plus `FUN_005cf940` reset recovery to state 0.

P58F11 preserves the strategic squadron count while state 4 removes the group
from the active tactical scene. Invalid or destroyed carriers reset recovery
rather than consuming the squadron.

## Acceptance boundary

The state codes, strict `2.0` gate, turn-rate formula, signed XZ turn, snap
branch, normalization, and millisecond integration are source traced. The
fighter ingress speed is bound to the source sublight rating through the
existing tactical scale, but a complete original trajectory comparison remains
open. Exact global RNG sequencing, the complete arrival callback chain,
collision, formation, attack execution, and lossless A0 visual comparison also
remain open.
