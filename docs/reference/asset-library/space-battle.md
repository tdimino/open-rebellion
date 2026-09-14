# Space-battle graphic inventory

The original battle mode spans [106 required interface cells](../../qa/2026-09-10-interface-parity-audit/surface-ledger.json) in `TAC-01` through `TAC-07`. This page is the art lookup companion to the [standalone battle-launcher plan](../../plans/2026-09-12-tooling-standalone-space-battle-launcher.md). The launcher will exercise the same production battle scene as a campaign, but it cannot make an uncertain asset mapping authoritative.

## Extracted BMPs and known controls

The [resource inventory](resource-inventory.json) records all 288 staged `TACTICAL.DLL` BMP IDs with dimensions. Current code names some task-force and squadron frames, command-button states, five weapon-recharge frames (`1206`–`1210`), a hull/shield panel (`1302`), and mission HUD buttons (`2151`–`2158`) in [bmp_cache.rs](../../../crates/rebellion-render/src/bmp_cache.rs). Those names are implementation mappings pending full original-state verification. The [331-entity catalog](entity-catalog.json) exposes the absent tactical roles per ship and fighter.

The first runtime composition now maps full shell `1000`; faction task-force
headers `1001`/`1004`; fighter headers `1008`/`1010`; task-force states
`1005`–`1007`; squadron states `1012`–`1020`; camera/navigation art
`1026`–`1033` and `1044`–`1059`; Alliance dim/highlight `1034`/`1035`;
Imperial dim/highlight `1036`/`1037`; battle options `1038`/`1039`; pause `1060`/`1061`;
and empty/selected capital-ship panels `1301`/`1302`. Pause, highlight, zoom,
and the left/right/up/down camera directions have source-shaped hit masks.
Zoom renders held resources `1045`/`1047`; the D-pad renders `1049`, `1051`,
`1053`, and `1056` while held. P57B2A binds those inputs to the recovered field,
yaw, and pitch rules in its isolated source-camera fixture. P57B2B1 activates
center resources `1058`/`1059` for selected-object focus. P57B2B2 derives the
source battle extent and four lanes from active force counts. P57B2B3 preserves
each production participant's DAT class and fleet-roster identity, reproduces
the original X slots, and targets the selected source world coordinate. See the
[P52 evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-shell-controls.md)
and [P53 evidence](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-control-hit-states.md)
for current positions, browser source-pixel proof, and open mappings.

The current renderer calls IDs `2001`–`2130` a tactical ship-sprite block, but the extracted BMP inventory contains only **43** IDs inside that range: `2001`–`2038` and `2101`–`2105`. Its `class_to_sprite_id` formula is explicitly approximate. The original battle manager instead holds a [29-ship, eight-fighter ordinal lookup](tactical-lookup.json) into custom type-301 meshes and type-303 fighter graphics. An ordinal is not yet a proven DAT class identity. Missing BMP numbers must not be synthesized or mistaken for missing custom resource types.

## Ships, fighters, planets, and effects

| Graphic family | Current inventory | Semantic mapping status |
|---|---|---|
| Capital ships | 30 CAPSHPSD classes; 29 native ordinal → type-301 bases `2010`–`2150` and `2510`–`2640`, plus separate Death Star `5010`/`5020` | Ordinal table is original-binary-proven. The `2560`–`2562` family and source LOD predicate are proven; candidate DAT class labels, other family details, selection, and damage are not |
| Fighters | Eight FIGHTSD classes; eight native ordinals `29`–`36` → paired type-303 bases `4000`–`4134` | Ordinal table is original-binary-proven; DAT class join, side/formation offsets, launch/recovery and damage remain open |
| Faction banners and avatars | Character portrait and some mini-icon candidates exist in GOKRES; tactical faction chrome BMPs are staged | No complete banner/avatar-to-battle-state map |
| Planets and backdrops | Tactical selector produces type-303 planet `5500 + selector` and palette `5530 + selector`; source screenshots show planet and empty-space views | The battle-manager selector's initialization from `SYSTEMSD.picture_id`, backdrop, filtering, and culling remain open; P57B2C2A proves the scene light rig |
| Weapon fire, explosions, shields, damage | Tactical BMP IDs and runtime procedural effects exist | No complete original effect-frame, mesh, texture, timing, sound, or damage-state lookup |
| Battle meshes and textures | Original `TACTICAL.DLL` contains 87 type-301 meshes and 397 type-303 texture/palette resources | P54 raw staging and P55 deterministic decoding are implemented. P56 proves the first exact pair. P57A packages `2560`–`2562`, binds `SDESTI52.BMP` and `SDESTI_M.BMP`, and proves source LOD selection. P57B1 proves the cached-slot pattern. P57B2A proves the faction camera, field zoom, clip planes, handedness conversion, and four D-pad commands. P57B2B1 proves selected-object targeting. P57B2B2 proves the active-force envelope. P57B2B3 binds stable production identities and source positions. P57B2C1 proves authored coordinates and system palette selection. P57B2C2A proves the retained-mode directional and ambient light rig. Lossless view acceptance, filtering, culling, production 3D family selection, and the DAT-to-tactical resource join remain open |
| Battle audio | Original `TACTICAL.DLL` contains 66 WAVs | Not covered by this image inventory; map event and cue separately |

`RE-TAC-01`, `RE-TAC-02`, `RE-TAC-03`, `RE-BAT-01`, and `RE-DS-02` in the [reverse-engineering ledger](../../qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md) own the missing loader, geometry, state, results, and Death Star mappings. The [screenshot ledger](../../qa/2026-09-10-interface-parity-audit/screenshot-ledger.md) provides representative tactical HUD, selected ship, damage, and result captures. These are useful to classify art, but their compression, language, altered-campaign provenance, and state gaps prevent strict acceptance.

The original ordinal table is visible in [`FUN_005ab650`](../../../ghidra/notes/FUN_005ab650.c). Its candidate DAT names in the JSON are labeled separately because the original vtable `+0x24` implementors have not yet been joined to `CAPSHPSD`/`FIGHTSD`. The same function selects Death Star bases `5010`/`5020`. `FUN_00596ad0`, `FUN_005c2e60`, and `FUN_0059a850` establish the tactical planet arithmetic, but not its DAT input. P54 supplies reproducible, content-addressed raw extraction. P55 supplies deterministic binary-X and type-303 decoding with a verified runtime store. [P56](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-render-proof.md) proves browser submission for one exact pair. [P57A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-lod-family.md) proves the complete related family and the selection predicate recovered from `FUN_005d26c0`, `FUN_005d3770`, and `FUN_005d3650`. [P57B1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-live-lod-journey.md) adds the cached-slot sequence recovered from `FUN_005c1160` and proves one live renderer changes slots without another family load. [P57B2A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md) recovers the first faction camera, projection, clip-plane, handedness, and D-pad command slice. [P57B2B1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-target-control.md) recovers and activates selected-object targeting. [P57B2B2](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-battle-layout.md) recovers the active-force extent and four tactical lanes. [P57B2B3](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-participant-placement.md) adds stable production DAT/roster identity, exact X slots, world positions, and selected-point targeting. [P57B2C1](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-transform-palette.md) proves authored coordinates and system palette selection. [P57B2C2A](../../qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md) proves the source directional and ambient light rig. The DAT-to-tactical-vtable join, filtering, culling, remaining render rules, production 3D family selection, and original-runtime confirmation remain required before the procedural battle objects can be retired.

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
IDs currently selected in [`audio.rs`](../../../crates/rebellion-app/src/audio.rs)
for re-audition; no mapping should change until the owned WAVs are identified.

The repository publishes no software license, bundles proprietary imagery,
and contains label conflicts, including tactical BMP 1034/1035 relative to our
source-pixel-verified mapping. Do not copy its code, tables, or media into Open
Rebellion. Use individual labels only as research leads and independently
verify each one against the owned original executable, DLL resources, and
auditioned audio before promoting it to this library.

## Minimum relation to add for each original graphic

Record `entity DAT ID or event`, `faction`, `resource module/type/ID`, `role`, `variant/state`, `native loader or predicate`, `render rectangle or transform`, `source reference`, and `confidence`. For ships, distinguish a strategic mini-icon, status portrait, encyclopedia EData image, tactical mesh, tactical texture, HUD silhouette, target marker, and destruction effect. For planets and battle effects, record the selected system or combat event and the frame/timing path. Do not collapse these into one “sprite ID.”
