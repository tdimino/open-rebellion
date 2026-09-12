# Entity graphic mappings

The [entity catalog](entity-catalog.json) covers 331 base-game records: 30 capital ships, eight fighters, ten troops, nine special forces, 14 facilities, 60 characters, and 200 systems. Of these, 33 buildable records carry research fields. The catalog now records 262 `source-derived` GOKRES class and miniature relations for all 131 non-system classes. These are exact, resource-validated IDs, not yet accepted interface placements.

## Original class-art selector

The native selector in [`FUN_0042c3b0`](../../../ghidra/notes/FUN_0042c3b0.c) and the direct consumer in [`FUN_00437880`](../../../ghidra/notes/FUN_00437880.c) mask the class resource value to 12 bits and load either its base GOKRES bitmap or the `+0x4000` miniature. Across all 131 decoded DAT class records, `text_stra_dll_id & 0x0fff` yields existing base and miniature bitmaps:

```text
class_key                = text_stra_dll_id & 0x0fff
GOKRES class bitmap      = class_key
GOKRES class miniature   = class_key + 0x4000
ENCYBMAP lookup key      = class_key + 0x1000
```

The encyclopedia-key arithmetic is from [`FUN_0045d400`](../../../ghidra/notes/FUN_0045d400_encyclopedia_loader.c). It does not itself supply the EData image number. The remaining source-proof edge is the original DAT-field write into the runtime class field read by these functions. The generated catalog therefore grades class/miniature links `source-derived`; the state in which a specific panel uses either image remains unresolved.

| Family | Records | Class-key range | Still needed for interface parity |
|---|---:|---|---|
| Production, manufacturing, defense facilities | 14 | 1–2, 256–261, 512–516, 640 | Construction, damaged, disabled, research and system-window use |
| Troops and special forces | 19 | 1088–1092, 1152–1156, 1344–1347, 1408–1412 | Status, mission, ground-report and encyclopedia states |
| Fighters | 8 | 1600–1603, 1664–1667 | Squadron, tactical model/texture, formation and damage states |
| Capital ships | 30 | 1856–1870, 1920–1934 | Tactical mesh/texture, orientations, status and damage art |
| Major and minor characters | 60 | 2112–2115, 2128, 2176–2177, 2624–2649, 2688–2715 | Portrait/miniature placement, injury, capture, Force and authored-event variants |

`ENCYBMAP.DLL` maps these keys to EData filenames, including a non-linear defense case: key 4736 maps to EData 14. The metadata generator does not yet decode that DLL, so the catalog leaves `encyclopedia_image` unresolved. The port's family-offset lookup is only an implementation approximation until it follows the native map and confirms each image. Jedi Luke's alternate EData 192 also needs its state predicate.

The 200 systems use a different selector. The original 26-way switch at `FUN_0045c970` maps picture IDs 1–23 to STRATEGY BMP 10212–10234 and IDs 24–26 to 10239, 10237, 10238. The original system window consumes that switch, but the write from `SYSTEMSD.picture_id` into its runtime field remains to be traced. Tactical planet textures and EData planet images have separate lookup paths; see [space-battle graphics](space-battle.md).

The [legacy map](../../../data/resource-entity-map.json) remains a candidate list built from offset guesses. Do not treat its 162 rows as additional verified mappings. The [verification guide](ghidra-verification.md) defines the remaining Ghidra and original-runtime checks; the [interface audit](../../qa/2026-09-10-interface-parity-audit/README.md) owns visual acceptance.
