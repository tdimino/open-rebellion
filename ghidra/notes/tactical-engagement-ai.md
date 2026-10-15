---
title: "Tactical engagement AI"
description: "How each side's task forces pick targets by tactic and close to weapon range, and what a stalemate does to the loser"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-10
---

# Tactical engagement AI

Traced 2026-10-10 in the saved `REBEXE.EXE` project (headless, read-only, on a
scratch copy). It explains why a task force with no orders still fights: both
sides run the same per-step target assignment, whether a player commands the
side or not. Simulate runs the side step below with nothing moving; a played
battle steers each task force and fires its batteries instead.

## The side step

Each battle side (`battle+0x24c` / `+0x250`, constructed by `FUN_005af5f0`
through `FUN_005c7cb0`, vtables `0x0066c428` / `0x0066c3f8`) is stepped by
vtable `+0x28` = `FUN_005d14d0` (dt from `FUN_005ae9c0`). Every step it:

1. calls `FUN_005c9200`, the target assignment below;
2. for each task force in `side+0x70`, takes its target (`FUN_005d2190`
   on the force's `+0x8c` list) and applies the force's damage rate times dt
   (`FUN_005cb650` sums `FUN_005b7780` per unit) through `FUN_005b54d0`,
   removing a target whose hull reaches 0 (`FUN_005af7a0`);
3. when the side is flagged to retreat (vtable `+0x2c` = `FUN_005d19a0`),
   sends hyperdrive units out (`FUN_005af910`, state 9), docks the rest in an
   escaping carrier (`FUN_005afe40`) or destroys them (`FUN_005af7a0`);
4. recharges shields (`FUN_005b6320`).

`FUN_005b54d0(unit, damage)` is the damage routine: shield `+0x3c0` absorbs
first, overflow (at least 1.0, `_DAT_0066c500` = -1.0) goes to hull `+0x3cc`,
with subsystem-hit rolls at 50/65/80/90/95/100 (`_DAT_0066c520`…`c51c`).
`FUN_005af7a0` applies 100000.0 damage and sets state `0xb`: destruction.

## Target assignment (`FUN_005c9200`)

The side keeps an opponent list at `side+0x90`. `FUN_005c9200` splits it into
capitals-first (`this`) and fighters-first (`local_2c`). It collects the
side's task forces that need a target (policy `+0x2c`, vtable `+0xc`:
`FUN_005ed5f0` / `FUN_005ed370`). A force with order 2, 3 or 6 is left
alone, and so is a fighter group launching or recovering. A force with no
target needs one, as does a force whose kill time is 300 s or more
(`_DAT_0066da84`) or unreachable while another force shares the target. Forces
whose leader has laser cannons (`FUN_005b7ce0`, arc `+0x00`) are matched
against the fighters-first list; the rest against the capitals-first list.
Each set goes to the side's tactic object `side+0x34`, vtable `+4`, with
`DAT_0066cbac` = 180.0.

The tactic object is read from the side record (`FUN_005c77c0`): code 1
builds vtable `0x0066cbf0` (Surround), code 2 builds `0x0066cbe8`
(Stand Off), matching the panel's codes (`FUN_005dd3a0`).

- Surround, `FUN_005ec6d0`: every force takes the list's first target.
- Stand Off, `FUN_005ec740`: for each eligible target in order, every free
  force is keyed by its kill time (`FUN_005c9160` → policy `+4` =
  `FUN_005ed170`), or by 100000 minus its damage when outside [0, 180].
  A lone force takes the target and the pass ends. Otherwise it starts from
  the best force's damage and walks from the worst force upward, adding
  each force's damage, until the combined kill time falls inside [0, 180].
  It then commits that force and every worse one. Forces left at the end
  take the first eligible target.

`FUN_005a8fc0` links a force to its target (`FUN_005cb950`) and emits event
`0x36`.

`FUN_005ed170(target, force)`: effective hull = hull × 0.65 + shield, at
least 1.0. Kill time = effective hull / (force damage − target's vtable
`+0x64` value). Travel adds max(distance / speed, 1.0) × 0.001, using the
force's formation speed (formation vtable `+0x34`). The result is -1.0 when
the force cannot out-damage the target.

## Steering (`FUN_005cc300`)

Each task force (vtable `0x0066bf70`, base ctor `FUN_005c9db0`) owns a
formation anchor at `+0x54` (`FUN_005ec310` / `FUN_005ea030`, vtable
`0x0066da08`). Its update, vtable `+0x14` = `FUN_005cc300`, does one of two
things. With a waypoint list (`+0x74`) it follows the waypoints; that is how
maneuvers move. With a target it steers:

```text
delta = target.position - anchor.position
band  = range object (+0x40, vtable 0x0066ccd0) slot +0x10
if distance + 2.0 is outside [band.near, band.far]:
    if distance < band.near: delta = side facing (FUN_005a7490, side+0x48) * near * -1.1
    reach = near + f * (far - near), f = 0.5 (<3 units), 0.75 (<6), 0.9; at least 2.0
    goal  = anchor + normalize(delta) * reach
    anchor moves toward goal (formation vtable +0xc, stop distance 2.0)
else:
    anchor velocity = 0
```

The two sides face +Z and -Z (`FUN_005af5f0` stores (0,0,±1) at `+0x48`).

## The band (`FUN_005edf80`)

The range object recomputes the band against the force's target. Each unit
adds `[0, range]` intervals for its laser, ion and turbolaser families. Each
is weighted by count × condition × the family's factor (`FUN_005b49e0`). The
target's families are subtracted. `FUN_005b7e50` merges the intervals. The
band is the interval with the greatest net strength whose far edge the
force's longest weapon reaches (`FUN_005b18d0`). Its near edge is at least
5.0 (`DAT_0066daa8`). A force with no weapons holds from max(1.2 × the
target's reach, 10.0) out to 1e6.

## Stalemate, corrected

`FUN_005ae9c0`'s 1200-second rule does not move units away. For the weaker
side (both when equal) it checks vtable `+0xc` = `FUN_005d0f50`: does the
side have a unit with a working hyperdrive (`FUN_005b1b70`)? If so the side
is flagged `+0xb0` and retreats through step 3 above. If not,
`FUN_005af7a0` destroys every unit. When `FUN_005a66a0` finds a Death Star
and `FUN_005ba420` calls it operational, side `+0x24c`'s figure becomes -1.0
before the comparison: the side facing the Death Star always loses the
stalemate (port: `advance_stalemate_watch`).

## The priority list (`FUN_005d1a70`)

The side step rebuilds `side+0x90` before assigning targets. Each opponent is
keyed by the policy value `FUN_005ed8e0`: 15.0 × its firepower against
capitals (`FUN_005b7a80`), minus 0.01 × its shield-recharge ratio
(`FUN_005b1710`), minus 0.01 × its hull ratio (`FUN_005b1640`). Eligible
targets come first. The list's remaining terms are not ported.

## Damage rate (`FUN_005b7780`)

Unit vtable `+0x84` = `FUN_005b4c60` picks the arc with the most firepower
against the target's class. The rate sums count × 0x3e888889 × the family's
modifier over that arc's families, times the unit's weapon recharge (vtable
`+0x60`), divided by the arc's weapon count. Against a fighter, lasers and
turbolasers scale by max(own / target maneuverability, 0.1) and ions do
nothing. A fighter against an unshielded capital ship adds its torpedoes ×
0x3f088889.

Vtable `+0x60` / `+0x64` by class:

| Class | Vtable | `+0x60` (weapon recharge) | `+0x64` (shield recharge) |
|---|---|---|---|
| Capital (0) | `0x0066c140` | `FUN_005b0400` | `FUN_005b0460` |
| Fighter (1) | `0x0066c028` | hull × 3.751 / 8 | hull × 0.025 / 8 |

The kill time's `+0x64` term is therefore the target's effective shield
recharge.

## Side strength (`FUN_005d0ce0`)

Σ `FUN_005b7a80` over the side's squadrons, plus each capital ship not already
out-damaged (a ship whose attacker's damage meets its shield recharge is left
out). The stalemate watch tracks this value.

## Subsystem hits in played battles

The windowed damage receivers (`FUN_005b5100` laser, `FUN_005b5750` ion,
`FUN_005b5b60` turbolaser, `FUN_005b5f50` torpedo) roll subsystem hits
(events 0x1e–0x22) only when unit `+0x124` is 0. `FUN_005c0170` sets
`+0x124` = 1 for every unit whose side (`+0x168`) is not the player's
(manager `+0x9ec`), so the computer's units never lose subsystems in a played
battle.

## Port

`rebellion-core` `tactical.rs`: `headless_step` and `apply_headless_damage` (Simulate),
`original_unit_damage_rate`, `engagement_priorities`,
`engagement_target_value`, `stalemate_strength`; for played battles
`advance_original_engagement` (called from `step` before movement),
`engagement_band`, `engagement_kill_seconds`, `assign_engagement_targets`,
`steer_engagement_group` and `original_windowed_damage_rolls`; and
`stalemate_resolve_side`.

## Open

- Which of `battle+0x24c` / `+0x250` is stepped first in the port's terms;
  the port steps the defender first.
- The priority list's terms beyond the policy value.
- The morale retreat in `FUN_005d19a0` (side strength below 0.33 of its
  start, `side+0x24`) and when `side+0x80` permits it.
