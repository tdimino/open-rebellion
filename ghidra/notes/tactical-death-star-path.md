# Tactical Death Star path

This note records the tactical Death Star state recovered from the owned
English `REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The object is separate from the 29-entry capital-ship mesh registry.

## Recovered functions and resources

| Function | Role |
|---|---|
| `FUN_005ab0e0`, `FUN_005ab650` | Select manager sprite 5010 or 5020 from field `+0x74` |
| `FUN_005ba420` | Death Star operational predicate |
| `FUN_005ba5e0` | Commits the Death Star action and updates attached objects |
| `FUN_005ba7f0` | Advances the laser charge |
| `FUN_005b8b10`, `FUN_005b8bd0` | Route weapon/damage events to the separate Death Star object |
| `FUN_005afe40` | Assigns the Attack Death Star executor and target |
| `FUN_005c0b60` | Adds the operational Death Star to the target list |
| `FUN_005caf20`, `FUN_005caf50` | Decode `TACTICALRESULT_UPDATE` and forward its state to the live result object |
| `FUN_005cfec0` | Dispatches tactical result states 6 and 7 to the two trench-run branches |
| `FUN_005c0fb0`, `FUN_0059cc60` | State 6 wrapper; requests original movie ID `0x71` |
| `FUN_005c0ff0`, `FUN_0059cc80` | State 7 wrapper; requests original movie ID `0x70` |
| `FUN_00421ba0` | Maps `0x71` to `MDATA\\MDATA.201` and `0x70` to `MDATA\\MDATA.202` |
| `FUN_005c4ed0` | Builds the laser gauge and 1021/1022/1023 control |
| `FUN_005df110` | Builds Attack Death Star resources 1176/1177/1178 |
| `FUN_005d4d10` | Builds the separate 5030 authored sequence surface |
| `FUN_005d6700` | Constructs the tactical 3D window with the standard arrow cursor (`IDC_ARROW`) |

The exact manager fields are:

| Offset | Meaning established by use |
|---|---|
| `+0x68` | Live value; zero makes the object non-operational |
| `+0x6c` | Laser charge, capped at 100 |
| `+0x74` | Destroyed/state flag; selects resource 5020 instead of 5010 |
| `+0x78` | Committed-action flag; value 1 makes the object non-operational |

`FUN_005ba420` is true only when `+0x74 == 0`, `+0x68 != 0`, and
`+0x78 != 1`.

## Charge and controls

`FUN_005ba7f0` advances the charge only while the live value is positive and
the destroyed flag is clear:

```text
charge += elapsed_milliseconds * (1 / 3) * 0.001
charge = min(charge, 100)
```

This is a 300-second zero-to-full charge. The laser control uses 1021 at rest,
1022 when fired/pressed, and 1023 while disabled or loading. The mission panel
uses 1176 at rest, 1177 pressed, and 1178 disabled. It is initially disabled;
the reconstructed mission control becomes active only for a selected fighter
group with a hostile operational Death Star.

## Implemented boundary

The tactical session now creates one separate Death Star from
`Fleet::has_death_star`, transports its faction and battle side, renders the
5010/5020 resource at its retained position, applies the exact operational and
charge predicates, routes the selected fighter-group Attack Death Star order
to it, persists destruction to the strategic fleet, and keeps a Death-Star-only
fleet alive. The production renderer also decodes resource 5030 as the sparse
440 by 438 tactical star surface, composes the owner-only 1024 gauge and
1021/1022/1023 control, arms a right-click capital target, commits the manager
action, resets charge, renders a delayed visible beam, and resolves destruction.
The deterministic browser journey proves the ready, held, armed, committed,
loading, and resolved states for both ownership cases and viewports.

## Trench-run result routing

The executable distinguishes the films from campaign victory and defeat.
`FUN_005caf20` decodes a `TACTICALRESULT_UPDATE` payload as an object ID plus
state, `FUN_005caf50` forwards it to the active tactical result object, and
`FUN_005cfec0` dispatches the result:

| Result state | Wrapper | Internal movie ID | File | Meaning |
|---:|---|---:|---|---|
| 6 | `FUN_005c0fb0` / `FUN_0059cc60` | `0x71` | `MDATA.201` | Successful trench run; Death Star destroyed |
| 7 | `FUN_005c0ff0` / `FUN_0059cc80` | `0x70` | `MDATA.202` | Failed trench run; attacking fighter participants destroyed |

Open Rebellion now records a one-shot tactical trench-run outcome when fighter
fire destroys the separate Death Star object, routes success to 201, and
routes the loss of every committed Attack Death Star fighter while the station
survives to 202. Both films return to the paused tactical result after
playback. Campaign headquarters victory no longer reuses either film. The
original producer condition that chooses result state 6 versus 7 has not yet
been recovered, so production does not invent an additional probability roll.

The exact beam dimensions and timing, original state-6 versus state-7 producer
condition, exact Death Star damage receiver, and lossless native comparison
remain open. The current beam shape is bounded A1 implementation evidence, not
a claim that the native raster or timing has been recovered. A custom target
cursor is not an open requirement: `FUN_005d6700` calls `LoadCursorA` with
`IDC_ARROW` for the native tactical 3D window.
