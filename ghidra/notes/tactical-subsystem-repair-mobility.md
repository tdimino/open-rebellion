# Tactical subsystem repair and mobility

The saved `REBEXE.EXE` project establishes the repair cadence, repair-category
selection, and sublight penalties implemented by P58F7.

## Repair timer and selection

`FUN_005b0330` counts down the repair timer and resets it to
`DAT_0066c46c * DAT_0066c490`. Those floats are `50.0` and `1000.0`, so the
source interval is 50,000 milliseconds. Open Rebellion advances tactical
combat four times per second and therefore attempts repair every 200 steps.

`FUN_005b1490` first rejects ships with no outstanding hits or no damage-control
chance. It draws an inclusive integer from 1 through 100 and continues when
that value is no greater than the CAPSHPSD damage-control value. It then draws
one ordinal uniformly across every outstanding hit in this order:

1. Sublight engines
2. Shield generators
3. Hyperdrive
4. Tractor beam
5. Weapons

Capital-ship vtable target `FUN_005b1ab0` decrements the selected counter and
clamps it to zero. The original global RNG sequence remains unresolved; P58F7
preserves the recovered ranges, selection order, interval, and deterministic
session replay.

## Sublight and tractor formulas

`FUN_005b05c0` stores the CAPSHPSD sublight rating after multiplying it by the
source constants `0.4` and `0.6`, for a combined scale of `0.24`.

`FUN_005b17f0` computes available sublight power as:

```text
undamaged = base_sublight_power + current_mode_bonus
available = undamaged - (undamaged * 0.25 * engine_hits)
available = available - sum(effective_power of each active tractor source)
available = max(available, 0)
```

Unlike shield, weapon, and tractor condition, sublight power is not multiplied
by the target ship's hull ratio. `FUN_005b16b0` divides available power by the
base sublight value for the HUD ratio.

`FUN_005b1790` computes one tractor source's effective power as:

```text
effective = tractor_max * source_hull / source_hull_max
effective = effective - (tractor_max * 0.25 * tractor_hits)
effective = max(effective, 0)
```

P58F7 modeled the current mode bonus as an explicit zero-default seam. P58F8
supersedes that provisional boundary with the recovered one-through-nine
producer documented in [tactical-maneuver-movement.md](tactical-maneuver-movement.md).

## P58F7 boundary

P58F7 installs the 50-second repair cadence, inclusive chance boundary,
per-hit category selection, decrement event, CAPSHPSD sublight and
damage-control values, engine damage, active tractor penalties, and visible
condition refresh in the production battle session. The browser fixture proves
one engine repair from two hits to one, followed by a 25-point tractor penalty:
base power 100, repaired engine power 75, and effective power 50.

Exact global RNG equivalence, interactive command delivery, turning, collision
and formation behavior, and lossless original-runtime pixels remain open. The
maneuver producer and physical integration are closed by P58F8.
