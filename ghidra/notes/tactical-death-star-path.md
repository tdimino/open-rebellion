# Tactical Death Star path

This note records the tactical Death Star state recovered from the owned
English `REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The object is separate from the 29-entry capital-ship mesh registry.

## Recovered functions and resources

| Function | Role |
|---|---|
| `FUN_005ab0e0`, `FUN_005ab650` | Store the station decal resource at `battle+0x8d4` (5010, 5020 for `+0x74`, 0 without a station) and pass it to `FUN_005c10a0` |
| `FUN_005ba420` | Death Star operational predicate |
| `FUN_005ba5e0` | Death Star retreat (message `0x45`, sent only by `FUN_005c8780` when the Empire's retreat countdown expires) |
| `FUN_005ba7f0` | Advances the laser charge |
| `FUN_005b8b10`, `FUN_005b8bd0` | Route weapon/damage events to the separate Death Star object |
| `FUN_005afe40` | Recovery docking of a launched squadron (state 2): into its target carrier, any own carrier, or for side 1 the Death Star hangar; sets state 5 |
| `FUN_005c0b60` | Battle-start docking: Empire squadrons in state 1, 5 or 9 dock into a same-side carrier hangar, else the Death Star hangar when operational or `+0x78 != 0` |
| `FUN_005caf20`, `FUN_005caf50` | Decode `TACTICALRESULT_UPDATE` and forward its state to the live result object |
| `FUN_005cfec0` | Arms the 120-second trench-run presentation and dispatches result states 6 and 7 |
| `FUN_005d04e0` | Selects participants, applies maneuver damage, rolls the commander rating, and chooses success or failure |
| `FUN_005d03f0`, `FUN_005d0430` | Emit the nine thresholded chatter slots, advance the timer, and enter state 6 or 7 |
| `FUN_005ad7e0` | Returns the tactical commander rating clamped to 1 through 9, with 1 as the unassigned fallback |
| `FUN_0061a310`, `FUN_005a8a70` | MSVC-compatible tactical random stream and inclusive range helper |
| `FUN_005c0fb0`, `FUN_0059cc60` | State 6 wrapper; requests original movie ID `0x71` |
| `FUN_005c0ff0`, `FUN_0059cc80` | State 7 wrapper; requests original movie ID `0x70` |
| `FUN_00421ba0` | Maps `0x71` to `MDATA\\MDATA.201` and `0x70` to `MDATA\\MDATA.202` |
| `FUN_005c4ed0` | Builds the laser gauge and 1021/1022/1023 control |
| `FUN_005df110` | Builds Attack Death Star resources 1176/1177/1178 |
| `FUN_005d4d10` | Builds the scene surround: the 5030 background, the planet decal and the non-pickable station decal (`tactical-death-star.md`, 19) |
| `FUN_005c2d20` | Viewport pick; accepts only objects with `+0x34 != 0`, which the surround clears |
| `FUN_005f5700`, `FUN_005f5750` | The surround's local generator and its clock seed (`srand(time(NULL))`) |
| `FUN_005d6700` | Constructs the tactical 3D window with the standard arrow cursor (`IDC_ARROW`) |

The exact manager fields are:

| Offset | Meaning established by use |
|---|---|
| `+0x68` | float remaining hull (slot 0x248 minus 0x274); zero makes the object non-operational |
| `+0x6c` | charge, starts at 100.0, capped at 100 |
| `+0x70` | shield flag (a family 0x25 facility in the system) |
| `+0x74` | not completed (ship `+0x50` bit 2 clear); selects resource 5020 |
| `+0x78` | Committed-action flag; value 1 makes the object non-operational |

`FUN_005ba420` is true only when `+0x74 == 0`, `+0x68 != 0`, and
`+0x78 != 1`. `FUN_005ba450` (attackable) also requires `+0x70 == 0`.

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

The tactical session creates one separate Death Star from the first
fleet holding a family `0x18`-`0x1b` ship (`Fleet::has_death_star`, `FUN_005003d0`),
transports its faction and battle side, applies the exact operational and charge predicates, routes
the selected fighter-group Attack Death Star order to it, and persists its
destruction to the strategic fleet by killing that ship
(`BattleSession::death_star_ship`, `tactical::apply::apply_results`). The production renderer also decodes resource 5030 as the sparse
440 by 438 tactical star surface, composes the owner-only 1024 gauge and
1021/1022/1023 control, arms a right-click capital target, commits the manager
action, resets charge, renders a delayed visible beam, and resolves destruction.
The deterministic browser journey proves the ready, held, armed, committed,
loading, and resolved states for both ownership cases and viewports.

The port draws the scene surround of `tactical-death-star.md`, 19: the
station decal (5010 or 5020, origin texel 64,64, half the extent square) on
its frame at `(-2.0 * extent, y, z)` (`TacticalDeathStar::placed`,
`tactical_station_decal`, `TacticalAssetRenderer::draw_station_decal`) and,
under Show Planet, the planet decal (origin 128,128, the extent square, black
transparent) at `(2.0 * extent, y, z)` (`tactical_planet_decal_position`,
`draw_planet`), both scaled by depth and clipped by the camera's planes
(`decal_screen_rect`). The station decal leaves the scene once a run
succeeds or the station departs (`FUN_005c1200` from `FUN_005cfec0` state 6
and `FUN_005ba5e0`). The superlaser leaves the station's child frame
(`TacticalDeathStar::superlaser_source`). The y and z offsets come from a
generator seeded like the battle's instead of the clock, so they repeat across
battles. The impact-effect pass carries only `FUN_005d3e90` frames keyed by
their target ship (`tactical_effect_render_objects`), and no hit-test, strip
or panel arrow reaches the station: they walk `BattleSession::ships`, which
seeding fills without Death Star ships.

## Trench-run result routing

The executable distinguishes the films from campaign victory and defeat.
`FUN_005caf20` decodes a `TACTICALRESULT_UPDATE` payload as an object ID plus
state, `FUN_005caf50` forwards it to the active tactical result object, and
`FUN_005cfec0` dispatches the result:

| Result state | Wrapper | Internal movie ID | File | Meaning |
|---:|---|---:|---|---|
| 6 | `FUN_005c0fb0` / `FUN_0059cc60` | `0x71` | `MDATA.201` | Successful trench run; Death Star destroyed |
| 7 | `FUN_005c0ff0` / `FUN_0059cc80` | `0x70` | `MDATA.202` | Failed trench run; attacking fighter participants destroyed |

`FUN_005cfec0` initializes a 120,000 ms timer. `FUN_005d04e0` pops the lowest-id participant, draws `rand(1,10)` against `trunc(maneuver)` (slot `+0xbc`); on a miss it damages by 100000 when `trunc(hp/8) < 2`, else by `rand(1, trunc(hp/8)) * 8`; a survivor then succeeds when `rand(1,100) <= trunc((hp/hpmax * (+0x134) * 0.5333 + min(rating, 9)) * 0.5)`; a failed survivor is re-inserted under the same id, up to 1001 passes; success restores the state-5 snapshot (`tactical-death-star.md`, 3). The commander rating (`FUN_005ad7e0`) is clamped to 1 through 9, and is 1 when the commander slot is unassigned.

The result object selects one of three exact nine-slot chatter sequences. The
slots occur at 10% intervals over the two-minute timer; `0x13d` is the silent
sentinel. State 6 destroys the Death Star and removes the first bounded half of
the participants: none for groups of zero or one, one for groups of two or
three, and `floor(n / 2)` thereafter. State 7 destroys every participant. The
result then routes to MDATA 201 or 202 and returns to the paused tactical view.

Open Rebellion follows that producer with one source-compatible tactical
RNG stream, the exact timer, ordered chatter, the success formula of `tactical-death-star.md`, 3, maneuver damage,
casualty rules, and movie route. Campaign entry seeds the tactical stream once.
The wider strategic model does not yet persist the executable's tactical
commander assignment slot, so an ordinary campaign battle uses the source's
unassigned rating of 1; deterministic fixtures set the rating explicitly.

Exact retained-mode beam raster comparison, whole-process RNG continuity,
strategic commander-slot binding, audible native playback, and lossless A0
comparison remain open. The current beam shape is bounded A1 implementation
evidence. A custom target cursor is not an open requirement:
`FUN_005d6700` calls `LoadCursorA` with `IDC_ARROW` for the native tactical 3D
window.
