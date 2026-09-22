# Tactical fighter combat

This note records the fighter-combat contract recovered from the owned English
`REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
It covers fighter construction, live hull and shield state, weapon output,
range, target modifiers, torpedoes, maneuver defense, and strategic squadron
transport.

## Recovered functions

| Function | Role |
|---|---|
| `FUN_0040be90` | Transports FIGHTSD values into the tactical creation call |
| `FUN_005964c0` | Allocates and registers one tactical fighter object |
| `FUN_005b9c60` | Fighter constructor and exact field initialization |
| `FUN_005b9c00` | Derives live weapon output from current hull |
| `FUN_005b9c30` | Derives live shield recharge from current hull |
| `FUN_005b49e0` | Emits laser, ion, and turbolaser events with float strength |
| `FUN_005b3f10` | Emits weapon and shield-gated torpedo events in source order |
| `FUN_005b7780` | Scores one concrete target, including maneuver ratios |
| `FUN_005b7a80` | Scores a target class without a concrete object |
| `FUN_005b23a0` | Produces torpedo event strength |
| `FUN_005b1650` | Returns target shield ratio |
| `FUN_005b5100` | Laser hit and overflow damage |
| `FUN_005b5750` | Ion hit and overflow damage |
| `FUN_005b5b60` | Turbolaser hit and overflow damage |
| `FUN_005b5f50` | Torpedo hit and overflow damage |
| `FUN_005b5700` | Direct floating hull damage path |

## Object and DAT transport

`FUN_005b9c60` constructs one tactical object per strategic fighter squadron.
All eight records in the owned `FIGHTSD.DAT` use only their fore battery. Their
aft, port, and starboard weapon counts are zero. Open Rebellion therefore
expands an aggregate strategic roster count into individual tactical
squadrons, while preserving the aggregate roster slot and a squadron ordinal
for the return path.

The constructor applies these exact scalar values:

| State | Formula or value |
|---|---|
| Maximum and current hull | `craft_count * 8.0` |
| Sublight power | `FIGHTSD.sub_light_engine * 0.4` |
| Initial weapon energy | sum of fore laser, ion, and turbolaser counts |
| Initial weapon readiness | true when the sum is positive |
| Weapon output factor | `current_hull * 3.751 / 8.0` |
| Shield recharge per second | `current_hull * 0.025 / 8.0` |
| Effective maneuverability | `raw + 1`, clamped at 9 |

FIGHTSD laser, ion, turbolaser, and torpedo ranges are direct tactical ranges.
The owned data confirms squadron size 12 for every original class.

## Weapon events and target modifiers

`FUN_005b49e0` emits each populated family as a separate float-strength event.
Its strength is:

```text
weapon_count * live_weapon_output * family_coefficient * target_modifier
```

The shared family coefficient is the exact float `0x3e888889`. Ion cannons do
not damage fighters. Lasers retain a 1.0 fighter modifier and use the exact
`0x3e2b020c` modifier against capital ships. Turbolasers use 0.5 against
fighters and 1.0 against capital ships. The source order is ion, laser,
turbolaser, then the conditional torpedo event.

Torpedo strength is `FIGHTSD.torpedoes * 0x3f088889`. The value is a combat
rating rather than a decremented ammunition counter. A fighter can emit the
torpedo event only against a capital target whose current shield ratio is zero
and only within `FIGHTSD.torpedoes_range`.

## Fighter damage and maneuverability

The laser, turbolaser, and torpedo receivers compare the target and source
effective maneuverability. The target-to-source ratio is clamped to at least
0.1, then incoming float damage is divided by that ratio. A more maneuverable
target therefore receives less damage. Ion damage against fighters is excluded
by the target modifier before dispatch.

Damage first subtracts from the floating shield pool. Negative shield overflow
continues into the floating hull pool with a minimum of one hull point at the
crossing. Live craft count is the ceiling of current hull divided by eight.
Zero hull removes the tactical object.

## Strategic return boundary

The strategic world stores whole squadrons rather than partial craft. The
return path counts each tactical squadron with positive hull, including a
successfully recovered squadron, back into its original aggregate roster slot.
Destroyed tactical squadrons do not return. Partial craft damage remains
tactical-only because the current strategic schema has no partial-squadron
field.

## Remaining boundary

The implementation preserves the recovered construction, coefficients,
target modifiers, ranges, event order, float hull and shield state, and typed
target paths. The executable applies `FUN_005b1770` through its global random
stream to some fighter values. Exact global RNG sequencing remains open for the
consolidated battle gate. Fighter collision, formation, authored arrival
callbacks, and lossless original visual comparison also remain open.
