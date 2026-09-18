# Tactical attack target acquisition

P58F12 restores the first source-traced execution step for Attack Fighters and
Attack Capital Ships from the owned English `REBEXE.EXE`.

Executable SHA-256:
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.

## Recovered dispatch

`FUN_005ca6d0` maps tactical order code 4 to the Attack Fighters executor at
`FUN_005d0bb0` and code 5 to the Attack Capital Ships executor at
`FUN_005d0b10`. Their vtables begin at `0x0066cf78` and `0x0066cf48`,
respectively.

Both executors use the same first-target contract:

1. Continue only while the executor field at `+0xc` is zero.
2. Validate the owner through `FUN_005c91e0`.
3. Traverse the stable tactical object list returned by `FUN_005f5060`; the
   next node is at `+0x10` and the object pointer is at `+0x1c`.
4. Read the object class from vtable slot `+0x20`: zero is a capital ship and
   one is a fighter group.
5. Require the candidate predicate in `FUN_005c91c0` and choose the first
   eligible object of the requested class.
6. Set executor field `+0xc` to one and dispatch the source-target link through
   `FUN_005a8fc0`, which emits event `0x36`.

`FUN_005aaf90` returns the tactical singleton at `DAT_006bc4a0`.
`FUN_005c91e0` and `FUN_005c91c0` delegate to its policy object at `+0x2c`,
using vtable slots `+0xc` and `+0x10`.

## Implementation boundary

P58F12 models attack targets as typed capital-ship or fighter-group identities.
It preserves a valid live engagement and otherwise selects the first eligible
hostile object of the requested class in stable tactical order. Capitals and
fighters can both execute either recovered attack order.

The executor dispatch and typed first-target acquisition are source traced.
The policy vtable internals, exact weapon-resolution callbacks, target-loss
reacquisition, global RNG sequence, collision, formation behavior, Death Star
execution, and lossless A0 comparison remain open.
