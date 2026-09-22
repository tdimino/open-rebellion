# Tactical collision and automatic grouping

This note records the collision envelope, overlap response, fighter-group
assignment, and bounded formation facts recovered from the owned English
`REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.

## Recovered functions

| Function | Role |
|---|---|
| `FUN_005ab0e0` | Installs one mesh-derived collision envelope |
| `FUN_005b2e60` | Tests two tactical objects for strict overlap |
| `FUN_005b2f30` | Advances a capital object and restores its prior position on overlap |
| `FUN_005b8f70` | Fighter movement path using the same spatial query |
| `FUN_005ae330` | Maintains one colored fighter-group list |
| `FUN_005ae460` | Assigns and rebalances red, blue, green, and gold groups |
| `FUN_005c81d0` | Derives the formation leader count |
| `FUN_005c8300` | Produces the alternating vertical leader slots |
| `FUN_005c83c0` | Divides a source object list across formation leaders |
| `FUN_005cbbc0` | Requests a retained follower position from a formation visual |
| `FUN_005eb040` | Returns an existing follower offset or derives one from the anchor |
| `FUN_005ea250` | Retains the largest member footprint multiplied by 1.2 |
| `FUN_005ec400` | Produces the alternating lateral follower slots |

## Collision contract

The close/base mesh bounds produce two values:

```text
vertical_radius = max((max_y - min_y) * 0.5, 0.5)
planar_diameter = max(hypot(max_x - min_x, max_z - min_z), 0.5)
```

Two objects overlap only when both source comparisons are strict:

```text
abs(first_y - second_y) < first_vertical_radius + second_vertical_radius
xz_distance_squared < ((first_planar_diameter + second_planar_diameter) * 0.5)^2
```

Touching either boundary is not an overlap. After a moving object is
integrated, the source spatial query checks the new position. A collision
restores the previous position while retaining the velocity computed for that
step. The envelope comes from the close/base family and does not change with
visual LOD.

## Fighter groups

`FUN_005ae460` assigns original ordinals 29 through 32 and 33 through 36 to
red, blue, green, and gold independently for each faction. When a color is
empty, the first largest donor wins ties. The first half of that donor moves
only when the donor contains more than seven squadrons.

Capital objects are divided into three lists before task-force formation:

1. armed without laser cannons;
2. laser-armed;
3. unarmed.

The first list may use slots 0 through 3. The second starts after the first
list and may use four slots. The third consumes the remaining slots through 7.
Within each list, `floor(sqrt(n))` leaders are selected, capped by available
slots. Every `floor(n / leader_count)`th member becomes the next leader and
intervening members attach to the most recent leader.

Before that division, `FUN_005c81d0` inserts every member into an ordered list
with this key:

```text
key = -(maximum_weapon_range * 10000 + available_sublight_power)
```

The exact float at `0x0066cbb0` is `-10000.0`. `FUN_005b18d0` returns the
maximum installed ion, laser, or turbolaser range and capital vtable slot
`+0x90` resolves to `FUN_005b17f0`, the live sublight-power calculation.
Ascending key order therefore selects greater maximum range first, then
greater available sublight power. Stable source-list order resolves exact
ties.

## Formation boundary

For a source list of `n` objects, the leader count is `floor(sqrt(n))`, with
one leader for fewer than two objects and the source slot cap applied. Leader
Y slots alternate `0, +8, -8, +16, -16, +24, -24, +32`, with X fixed at zero.
The formation visual stores follower offsets relative to its anchor.
`FUN_005ea250` retains the greatest member visual footprint multiplied by the
exact float `1.2`. `FUN_005ec400` truncates that spacing toward zero and emits
the following lateral sequence relative to the leader's forward vector:

```text
0, +spacing, -spacing, +2*spacing, -2*spacing, ...
```

`FUN_005cbbc0` adds the returned X/Y/Z offset to each follower's current world
position. `FUN_005eb040` reuses an existing retained offset when present and
otherwise derives the delta from the formation anchor and member record.
`FUN_005ec400` also proves that the lateral offset rotates with the formation
visual's current forward vector rather than remaining fixed on world X.

Open Rebellion now implements the mesh envelope, strict overlap predicate,
post-integration restoration, exact colored-group assignment, the three
capital weapon categories, exact capability ordering, square-root leader
counts, stable follower task-force ownership, leader slots, largest-footprint
spacing, and the alternating follower geometry. The decoded close-mesh report
refreshes geometry while the initial battle remains paused. Movement now
reapplies the retained, forward-relative offsets after every stable-order
integration and collision pass. Lossless original-runtime comparison remains
open, so this functional contract is not an A0 visual claim.
