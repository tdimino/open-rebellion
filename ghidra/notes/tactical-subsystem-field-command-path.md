# Tactical subsystem and field-command path

The saved `REBEXE.EXE` project establishes the five selected-capital subsystem
indicators and the exact source identity retained by tractor and gravity
commands. This note records the bounded findings used by P58F5.

## Selected-capital subsystem indicators

`FUN_005e45f0` and `FUN_005e7540` construct five 22 by 15 bitmap controls in
the selected-capital panel. Their panel-local positions are x 10, 37, 64, 91,
and 118 at y 103. Panel 1302 places them at tactical-canvas positions x 491,
518, 545, 572, and 599 at y 130.

The source-call order and bitmap families are:

| Subsystem | Resource family |
|---|---:|
| Shields | 1201 through 1205 |
| Weapons | 1206 through 1210 |
| Tractor beam | 1211 through 1215 |
| Sublight engines | 1216 through 1220 |
| Hyperdrive | 1221 through 1225 |

`FUN_005e77c0` converts each condition percentage to one of five resource
offsets. Zero uses offset 0. Values 1 through 24 use offset 1, 25 through 49
use offset 2, 50 through 74 use offset 3, and 75 or greater use offset 4.
These values are subsystem condition, not the live shield-energy or hull
meters already restored by P58F4.

## Tractor source identity

`FUN_005b23e0` starts a tractor command. It stores one target ID at source
offset `+0x380`, calls `FUN_005b2570` on the target, and clears the source
command timer at `+0x3ac`. `FUN_005b2570` adds the exact source ID to the
target collection at `+0x360`; the first source starts the message and visual.

`FUN_005b2440` stops the command. It clears the source target and timer, then
calls target-side `FUN_005b25d0`. That function removes the exact source ID.
The last removal stops the message and visual. One tractor source therefore
holds at most one target, while one target can retain several exact sources.

## Gravity source identity

`FUN_005b24d0` starts gravity control by placing an exact target ID in the
first free member of the source's four-slot array at `+0x384`. It clears the
source timer at `+0x3a8` and calls target-side `FUN_005b2520`, which enables
the target state at `+0x3a4` and starts its presentation.

`FUN_005b2480` removes the exact target ID, clears the source timer, and calls
`FUN_005b2550` to disable the target state. One gravity source therefore holds
at most four exact targets.

`FUN_005d3ac0` owns the shared visible field slot. Gravity resources 3520
through 3527 take priority over tractor resources 3620 through 3627. A visible
kind change resets the frame. `FUN_005d3cc0` advances eight frames at 0.1
seconds per frame.

## Acceptance boundary

P58F5 proves the five resource families, source quantization, placement, exact
field source identity, one-target tractor capacity, four-target gravity
capacity, visible-kind priority, frame reset, and browser presentation.
Combat-driven subsystem mutation, interactive command input, event and audio
delivery, and lossless original-runtime comparison remain open.
