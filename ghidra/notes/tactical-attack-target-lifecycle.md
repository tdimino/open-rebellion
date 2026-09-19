# Tactical attack target lifecycle

P58F13 restores the source-bounded invalidation and same-class replacement
lifecycle for Attack Fighters and Attack Capital Ships from the owned English
`REBEXE.EXE`.

Executable SHA-256:
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.

## Recovered boundary

The two attack executors are one-shot target-list producers. Their shared reset,
setter, and getter paths include `FUN_005d0b00`, `FUN_005cf910`, and
`FUN_005cf920`. `FUN_005a8fc0` links the first eligible typed target to the
owner's tactical group and emits event `0x36`.

The tactical manager created by `FUN_005a7500` separately registers add,
remove, replace-list, and change-target messages. `FUN_005a8c50` and
`FUN_005a8cc0` traverse adjacent target entries. `FUN_005a8d30`,
`FUN_005a8e30`, `FUN_005a8e80`, and `FUN_005a8fc0` cover the recovered
mutation and event paths. This places target loss and replacement in shared
group machinery rather than inside the one-shot order executor.

## Implemented contract

An invalid capital or fighter target is replaced with the first eligible
hostile object of the same requested class in stable tactical order. A depleted
class clears the target. Attack Fighters never redirects to a capital ship, and
Attack Capital Ships never redirects to a fighter group. Inactive owners also
clear their target.

## Open boundary

`FUN_005b3a40`, `FUN_005b3f10`, and `FUN_005b6530` expose a separate capital
weapon loop with directional arcs, range, per-weapon availability, strongest-arc
selection, projectile events, and recharge accounting. P58F13 does not claim
that loop. Exact target-removal callback ordering, global RNG sequencing,
fighter weapon resolution, collision, formation behavior, Death Star execution,
and lossless A0 comparison remain open.
