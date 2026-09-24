# Space-battle graphic inventory

The original battle mode spans
[106 required interface cells](../../qa/2026-09-10-interface-parity-audit/surface-ledger.json)
in `TAC-01` through `TAC-07`. This page is the art lookup companion to the
[standalone battle-launcher plan](../../plans/2026-09-12-tooling-standalone-space-battle-launcher.md).
The launcher will exercise the same production battle scene as a campaign, but
it cannot make an uncertain asset mapping authoritative.

## Extracted BMPs and known controls

The [resource inventory](resource-inventory.json) records all 288 staged
`TACTICAL.DLL` BMP IDs with dimensions. Current code names some task-force and
squadron frames, command-button states, five weapon-recharge frames
(`1206`–`1210`), a hull/shield panel (`1302`), and mission HUD buttons
(`2151`–`2158`) in
[bmp_cache.rs](../../../crates/rebellion-render/src/bmp_cache.rs). Those names
are implementation mappings pending full original-state verification. The
[331-entity catalog](entity-catalog.json) exposes the absent tactical roles per
ship and fighter.

The first runtime composition now maps full shell `1000`; faction task-force
headers `1001`/`1004`; fighter headers `1008`/`1010`; task-force states
`1005`–`1007`; squadron states `1012`–`1020`; camera/navigation art
`1026`–`1033` and `1044`–`1059`; Alliance dim/highlight `1034`/`1035`; Imperial
dim/highlight `1036`/`1037`; battle options `1038`/`1039`; pause `1060`/`1061`;
and empty/selected capital-ship panels `1301`/`1302`. Pause, highlight, zoom,
and the left/right/up/down camera directions have source-shaped hit masks. Zoom
renders held resources `1045`/`1047`; the D-pad renders `1049`, `1051`, `1053`,
and `1056` while held. P57B2A binds those inputs to the recovered field, yaw,
and pitch rules in its isolated source-camera fixture. P57B2B1 activates center
resources `1058`/`1059` for selected-object focus. P57B2B2 derives the source
battle extent and four lanes from active force counts. P57B2B3 preserves each
production participant's DAT class and fleet-roster identity, reproduces the
original X slots, and targets the selected source world coordinate. P58E binds
all eight task-force and four RGBY resources to source-shaped controls, group
selection, keyboard routes, and selected fighter panel `1307` with portraits
`2030` through `2037`. See the
[P52 evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-shell-controls.md)
and
[P53 evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-control-hit-states.md)
for current positions, browser source-pixel proof, and open mappings. P58F4
maps selected-capital portraits `2001` through `2029` from the same source
tactical ordinal, composites their lime mats over panel `1302`, and renders
live shield and hull values in its authored apertures. P58F5 maps the five
22 by 15 subsystem-condition families `1201` through `1225` and their exact
source quantization and placement.

The earlier renderer treated IDs `2001`–`2130` as a linear tactical ship-sprite
block, but the extracted BMP inventory contains only **43** IDs inside that
range: `2001`–`2038` and `2101`–`2105`. P58A removes that approximation. The
original battle manager instead holds a
[29-ship, eight-fighter ordinal lookup](tactical-lookup.json) into custom
type-301 meshes and type-303 fighter graphics. The source-named registry in
`FUN_00597610_ship_db`, correlated with unique DAT class identities, now proves
the full join. Missing BMP numbers must not be synthesized or mistaken for
missing custom resource types.

## Ships, fighters, planets, and effects

| Graphic family                           | Current inventory                                                                                                                                | Semantic mapping status                                                                                                                                                                                                                                                                                                         |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Capital ships                            | 30 CAPSHPSD classes; 29 native ordinals → type-301 bases `2010`–`2150` and `2510`–`2640`, BMP portraits `2001`–`2029`, five condition families `1201`–`1225`, plus separate Death Star `5010`/`5020`                  | P58A proves every DAT identity join. P58B installs the corpus and renders joined production families. P58C aligns selection and targeting with projected bounds. P58F4 restores selected portraits and live shield/hull meters. P58F5 restores all five subsystem-condition families and exact quantization. P58F6 connects source-derived live damage and tractor cancellation. P58F7 restores subsystem repair plus engine-condition and tractor-drag mobility. P58F8 restores maneuver-state production, velocity, and physical integration. P58F9 restores authentic command assignment. P58F10 restores maneuver waypoints, Hold, and the first Recover carrier state. Turning and waypoint completion, full fighter recovery, attack executors, collision and formation behavior, power allocation, destroyed, and Death Star views remain open |
| Fighters                                 | Eight FIGHTSD classes; eight native ordinals `29`–`36` → paired type-303 bases `4000`–`4134`, with indicator family `4200`–`4204`                 | P58A proves every DAT identity join. P58C renders close, far, and indicator resources through the exact view-span thresholds. P58D proves every independent transition in a centered production journey. P58E restores RGBY presentation and selected fighter portraits. Automatic group distribution, side/formation offsets, launch/recovery, and damage remain open                                                     |
| Faction banners and avatars              | Character portrait and some mini-icon candidates exist in GOKRES; tactical faction chrome BMPs are staged                                        | No complete banner/avatar-to-battle-state map                                                                                                                                                                                                                                                                                   |
| Planets and backdrops                    | Tactical selector produces type-303 planet `5500 + selector` and palette `5530 + selector`; source screenshots show planet and empty-space views | P58F1 source-binds `SYSTEMSD.picture_id` to the exact planet and palette pair, renders it in production battles, and proves both factions and viewports. Exact retained-frame placement and visibility remain open                                                                                                              |
| Weapon fire, explosions, shields, damage | Six target-sprite families occupy bases 3060, 3120, 3180, 3240, 3300, and 3360; retained projectile meshes and field families 3520/3620 are separate | P58F2 proves the target-sprite mapping and lifecycle. P58F3 proves all projectile geometry variants, thresholds, scales, faction/ion colors, interpolation and duration, plus tractor/gravity frames and 10 Hz animation. P58F5 proves exact field-source identities, source-side capacities, gravity priority, and frame reset. P58-B16 routes the matching weapon fire and impact audio from production capital and fighter combat. A0 comparison remains open |
| Battle meshes and textures               | Original `TACTICAL.DLL` contains 87 type-301 meshes and 397 type-303 texture/palette resources                                                   | P54 through P58F10 prove staging, deterministic decoding, full browser transport, exact DAT joins, and live capital, fighter, planet, target-effect, projectile, field, selected-capital damage, subsystem condition, combat-driven condition, repair, mobility, maneuver-state, velocity, physical integration, command assignment, maneuver waypoints, Hold, and initial Recover state. Lossless view acceptance remains open |
| Battle audio                             | Original `TACTICAL.DLL` contains 66 WAVs; events `0x0d–0x14` map to 22 weapon variants at `13033–13054`                                         | P58-B16 proves exact resource extraction, runtime-pack transport, typed production dispatch, and muted browser routing. Remaining non-weapon/voice events, exact shared-RNG sequencing, mixing, ducking, interruption, audible native comparison, and A0 acceptance remain open                                                                                                                               |

`RE-TAC-01`, `RE-TAC-02`, `RE-TAC-03`, `RE-BAT-01`, and `RE-DS-02` in the
[reverse-engineering ledger](../../qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md)
own the missing loader, geometry, state, results, and Death Star mappings. The
[screenshot ledger](../../qa/2026-09-10-interface-parity-audit/screenshot-ledger.md)
provides representative tactical HUD, selected ship, damage, and result
captures. These are useful to classify art, but their compression, language,
altered-campaign provenance, and state gaps prevent strict acceptance.

The original ordinal table is visible in
[`FUN_005ab650`](../../../ghidra/notes/FUN_005ab650.c). P58A joins it to the
source-named registry in
[`FUN_00597610_ship_db`](../../../ghidra/notes/FUN_00597610_ship_db.c) and the
unique `CAPSHPSD`/`FIGHTSD` identities. The same path selects Death Star bases
`5010`/`5020`. `FUN_0040b0e0`, `FUN_00509610`, `FUN_00595d60`,
`FUN_0059a850`, `FUN_00596ad0`, and `FUN_005c2e60` establish the tactical
planet selector, resource arithmetic, and paired palette. P58F1 binds that
selector to `SYSTEMSD.picture_id` and renders the exact production pair while
retaining placement as provisional A1 evidence.
`FUN_005a7500`, `FUN_005d39a0`, `FUN_005d3e90`, and `FUN_005d41a0`
establish the six target-effect families, message and stage branches, priority,
frame counts, draw sizes, and 0.1-second cadence. P58F2 renders those source
frames at live target positions. `FUN_005d3de0`, `FUN_005ee590`, and
`LAB_005eeb90` establish the retained projectile geometry, interpolation, and
duration. `FUN_005d3ac0` and `FUN_005d3cc0` establish the tractor/gravity frame
families, source timing, and shared priority. P58F3 implements those
presentation paths while retaining exact command timing as open work.
P54 supplies reproducible, content-addressed raw extraction. P55 supplies
deterministic binary-X and type-303 decoding with a verified runtime store.
[P56](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-render-proof.md)
proves browser submission for one exact pair.
[P57A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-lod-family.md)
proves the complete related family and the selection predicate recovered from
`FUN_005d26c0`, `FUN_005d3770`, and `FUN_005d3650`.
[P57B1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-live-lod-journey.md)
adds the cached-slot sequence recovered from `FUN_005c1160` and proves one live
renderer changes slots without another family load.
[P57B2A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md)
recovers the first faction camera, projection, clip-plane, handedness, and D-pad
command slice.
[P57B2B1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-target-control.md)
recovers and activates selected-object targeting.
[P57B2B2](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-battle-layout.md)
recovers the active-force extent and four tactical lanes.
[P57B2B3](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-participant-placement.md)
adds stable production DAT/roster identity, exact X slots, world positions, and
selected-point targeting.
[P57B2C1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-transform-palette.md)
proves authored coordinates and system palette selection.
[P57B2C2A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md)
proves the source directional and ambient light rig.
[P57B2C2B](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-render-state.md)
proves retained-mode device and material state.
[P58A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-resource-join.md)
proves the complete DAT identity join and transports all 87 meshes and 397
textures. [P58B](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-production-participants.md)
renders live capital ships from their exact joined families and source
positions. [P58C](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighters-selection.md)
renders original fighter detail resources and aligns capital interaction with
projected mesh bounds. [P58D](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighter-detail-journey.md)
proves the live fighter transition journey.
[P58E](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-group-presentation.md)
restores task-force and RGBY presentation.
[P58F1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-planets.md)
restores system-selected planet resources and their exact palettes.
[P58F2](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-impact-effects.md)
restores the six target-impact sprite families.
[P58F3](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-projectile-fields.md)
restores retained projectiles and tractor/gravity fields.
[P58F4](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-selected-damage.md)
restores panel 1302, capital portraits, matte transparency, and live shield and
hull meters.
[P58F5](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-field-commands.md)
restores the five subsystem-condition families plus exact field-source identity
and capacity.
[P58F6](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-live-subsystem-damage.md)
connects those families to live capital and fighter damage and tractor-source
cancellation.
[P58F7](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-repair-mobility.md)
restores the source repair cadence and selection plus engine-condition and
active tractor-drag mobility.
[P58F8](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-16-tactical-maneuver-movement.md)
restores maneuver-state production, effective-power velocity, signed faction
movement, and 250-millisecond position integration.
[P58F9](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-command-assignment.md)
restores the two original command panels, exact order and tactic codes,
source-shaped controls, disabled states, and commit/cancel behavior.
[P58F10](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-order-execution.md)
restores the four maneuver waypoint constructors, Hold stop behavior, and the
first Recover carrier state.
[P58F11](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-command-progression.md)
restores source-rate turning, waypoint completion, docking, and recovery.
[P58F12](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-attack-targeting.md)
restores typed capital and fighter target acquisition.
[P58F13](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-19-tactical-attack-target-lifecycle.md)
restores valid-target retention, stable same-class replacement, and exhausted
target-list clearing. Automatic group distribution, exact planet placement,
exact global RNG sequencing, exact weapon behavior, fighter combat, collision
and formation behavior, power allocation, special objects, and original-runtime
confirmation remain required before every procedural battle object can be
retired.

## External editor leads

A read-only audit of
[`StarWarsRebellionEditor.NET`](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET/tree/db63592cc90322e18ea5b357181828639f4332b3)
found useful candidate labels, but no renderer state, DAT-to-tactical join, or
battle launcher. Its
[`NamesBitmap`](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET/blob/db63592cc90322e18ea5b357181828639f4332b3/SwRebellionEditor/ResourceHelpers/Tactical.cs#L995-L1207)
and
[`Names303`](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET/blob/db63592cc90322e18ea5b357181828639f4332b3/SwRebellionEditor/ResourceHelpers/Tactical.cs#L98-L443)
tables can accelerate semantic triage of the 288 tactical BMPs and the effect,
shield, fighter, swatch, Death Star, and system resource families. Its
[`Voicefxa`](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET/blob/db63592cc90322e18ea5b357181828639f4332b3/SwRebellionEditor/ResourceHelpers/Voicefxa.cs#L9-L166)
and
[`Voicefxe`](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET/blob/db63592cc90322e18ea5b357181828639f4332b3/SwRebellionEditor/ResourceHelpers/Voicefxe.cs#L9-L143)
tables provide candidate tactical voice labels, including Death Star and
Alliance trench-run ranges. Those labels also flag the generic tactical voice
IDs currently selected in
[`audio.rs`](../../../crates/rebellion-app/src/audio.rs) for re-audition; no
mapping should change until the owned WAVs are identified.

The repository publishes no software license, bundles proprietary imagery, and
contains label conflicts, including tactical BMP 1034/1035 relative to our
source-pixel-verified mapping. Do not copy its code, tables, or media into Open
Rebellion. Use individual labels only as research leads and independently verify
each one against the owned original executable, DLL resources, and auditioned
audio before promoting it to this library.

## Minimum relation to add for each original graphic

Record `entity DAT ID or event`, `faction`, `resource module/type/ID`, `role`,
`variant/state`, `native loader or predicate`, `render rectangle or transform`,
`source reference`, and `confidence`. For ships, distinguish a strategic
mini-icon, status portrait, encyclopedia EData image, tactical mesh, tactical
texture, HUD silhouette, target marker, and destruction effect. For planets and
battle effects, record the selected system or combat event and the frame/timing
path. Do not collapse these into one “sprite ID.”
