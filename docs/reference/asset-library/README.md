# Original Graphic Reference Library

This library separates three questions that must not be conflated: **does a resource exist**, **which game entity or UI state does it depict**, and **does the game render it in the right place and state**. A numeric ID or a successful image load answers only the first question.

## Browse the inventories

| Reference | What it records | Current coverage |
|---|---|---|
| [DLL resource inventory](resource-inventory.json) | Exact IDs, dimensions, bit depth, and byte lengths of locally staged BMPs; IDs and byte lengths of type-302 frames | 2,303 BMPs and 3,988 advisor frames across six DLLs |
| [EData inventory](edata-inventory.json) | Exact IDs and metadata from the owned GOG payload | 187 400×200 images with IDs 1–192; 144, 145, 147, 159, and 165 are absent |
| [Entity catalog](entity-catalog.json) | Every decoded base-game DAT entity, research fields, GOKRES class keys, and unresolved graphic roles | 331 entities; 262 source-derived class/miniature links for 131 non-system classes |
| [Entity graphics](entity-graphics.md) | Original class-bitmap selector and remaining portrait, status, encyclopedia, facility, and research joins | Partial; the DAT-to-runtime initialization edge and display behavior still need verification |
| [Space-battle graphics](space-battle.md) and [native tactical lookup](tactical-lookup.json) | Tactical BMP groups, exact native ordinal-to-mesh/texture bases, candidate ship/fighter identities, and remaining effects | 29 ship and eight fighter ordinal mappings; DAT identity and render-state joins remain open |
| [Ghidra verification](ghidra-verification.md) | Evidence grades, original loader tracing, whole-family validation, and the asset-mapping work queue | Active; source-proven joins must be recorded before parity acceptance |

The inventory JSON contains metadata, not game images. Resource keys use `module/resource_type/id`, for example `tactical-dll/BMP/1206`. Contributors stage the underlying artwork from their own installation using [stage-ui-assets](../../../tools/stage-ui-assets/README.md). Regenerate the metadata with:

```sh
node scripts/build-asset-reference-inventory.mjs --edata=/path/to/owned-game/EData
node scripts/build-asset-reference-inventory.mjs --edata=/path/to/owned-game/EData --check
node scripts/build-asset-entity-catalog.mjs
node scripts/build-asset-entity-catalog.mjs --check
node scripts/validate-tactical-lookup.mjs --source=/path/to/owned-game/REBEXE.EXE
```

The original executable selects GOKRES class art with `text_stra_dll_id & 0x0fff`, and its miniature at that key plus `0x4000`. All 262 IDs exist in the owned resource inventory. The remaining DAT-field-to-runtime-class initialization edge is not traced, so these are `source-derived`, not fully `source-proven` under our evidence contract. The separate [legacy resource-to-entity JSON](../../../data/resource-entity-map.json) names 162 IDs from candidate offsets and remains a discovery aid.

For visual browsing, use the staged [GOKRES gallery](../../../assets/references/ref-ui-full/gokres-dll/INDEX.md), [TACTICAL gallery](../../../assets/references/ref-ui-full/tactical-dll/INDEX.md), and [DLL resource catalog](../../../agent_docs/dll-resource-catalog.md). Their broad range labels are discovery aids; use the Ghidra verification route for exact entity and state identity.

## What remains outside the staged inventory

The [reverse-engineering ledger](../../qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md) records 87 `TACTICAL.DLL` type-301 meshes, 397 type-303 textures, 66 tactical WAVs, briefing controls and animation, original dialogs, and other unstaged resource families. The staged 288 tactical BMPs therefore are not a complete inventory of battle graphics. We have no verified one-to-one map from all 30 capital ships and eight fighters to tactical meshes, textures, orientations, damaged variants, planet backdrops, weapon effects, banners, or control-state art.

Every new semantic relation should record its source function or table, exact resource key, entity DAT ID, faction, role, state, and evidence grade. Use the [verification ladder](ghidra-verification.md); never promote a filename pattern to original behavior. The [interface audit](../../qa/2026-09-10-interface-parity-audit/README.md) remains the acceptance authority.
