# Entity graphic mappings

The [entity catalog](entity-catalog.json) covers 331 base-game records: 30 capital ships, eight fighters, ten troops, nine special forces, 14 facilities, 60 characters, and 200 systems. Of these, 33 buildable records carry research fields. The catalog now records 262 `source-derived` GOKRES class and miniature relations for all 131 non-system classes. These are exact, resource-validated IDs, not yet accepted interface placements. The separate encyclopedia source profile also contains 331 reviewed family rows with title/body/art evidence; that observed equality does not make the entity catalog a complete encyclopedia inventory or prove one unique topic per source row.

## Original class-art selector

The native selector in [`FUN_0042c3b0`](../../../ghidra/notes/FUN_0042c3b0.c) and the direct consumer in [`FUN_00437880`](../../../ghidra/notes/FUN_00437880.c) mask the class resource value to 12 bits and load either its base GOKRES bitmap or the `+0x4000` miniature. Across all 131 decoded DAT class records, `text_stra_dll_id & 0x0fff` yields existing base and miniature bitmaps:

```text
class_key                = text_stra_dll_id & 0x0fff
GOKRES class bitmap      = class_key
GOKRES class miniature   = class_key + 0x4000
ENCYBMAP lookup key      = class_key + 0x1000
```

The encyclopedia-key arithmetic is from [`FUN_0045d400`](../../../ghidra/notes/FUN_0045d400_encyclopedia_loader.c). It does not itself supply the EData image number. The reviewed [unit](encyclopedia-bindings-units.md), [character](encyclopedia-bindings-characters.md), and [static](encyclopedia-bindings-static.md) source notes now connect the original DAT field writes to family-qualified definitions and the encyclopedia selectors. The generated entity catalog still grades its independent GOKRES class/miniature links `source-derived`; which non-encyclopedia panels use either GOKRES image remains a separate interface question.

| Family | Records | Class-key range | Still needed for interface parity |
|---|---:|---|---|
| Production, manufacturing, defense facilities | 14 | 1–2, 256–261, 512–516, 640 | Construction, damaged, disabled, research and system-window use |
| Troops and special forces | 19 | 1088–1092, 1152–1156, 1344–1347, 1408–1412 | Status, mission, ground-report and encyclopedia states |
| Fighters | 8 | 1600–1603, 1664–1667 | Squadron, tactical model/texture, formation and damage states |
| Capital ships | 30 | 1856–1870, 1920–1934 | Tactical mesh/texture, orientations, status and damage art |
| Major and minor characters | 60 | 2112–2115, 2128, 2176–2177, 2624–2649, 2688–2715 | Portrait/miniature placement, injury, capture, Force and authored-event variants |

The strict ENCYBMAP reader now maps language-qualified keys to exact EData filenames, including the non-linear defense case 4736 -> `EDATA.014`. The embedded source profile reconciles all 117 accepted unit and character rows plus 14 facility rows to exact lookup identities and original image hashes; it does not copy those research bindings into the entity catalog or authorize the port's older family-offset approximation. `EDATA.192` remains inventory-only and publication-deferred under `orlocal-2kq`: no alternate-character predicate, binding, or original-display claim is inferred from file presence.

The 200 encyclopedia system rows use a different, now connected selector. `SYSTEMSD.DAT`'s reader writes `picture_id` to definition `+0x44`; `FUN_00509610` reads that field through the selected system view and `FUN_0045f660` maps values 1–23 to ENCYBMAP keys `0x2b5c..0x2b72`, then 24, 25, and 26 to `0x2b75`, `0x2b73`, and `0x2b74`. Those 200 rows use 26 exact `EDATA.166..191` images; 17 of the images have explicit shared-resource groups and the other nine are singletons. The separate system-window STRATEGY switch, tactical planet textures, and EData encyclopedia images remain distinct paths; see [space-battle graphics](space-battle.md).

The combined source profile accounts for all 348 ENCYTEXT records, 191 nonempty ENCYBMAP strings, 186 referenced filenames, and 187 EData images as bound, unresolved, or explicitly publication-deferred. It does not pretend that the 331 accepted family rows are complete: command `0x73` lacks a source-family fragment, leaving 17 text resources, 34 lookup identities, and 29 referenced image files unresolved. The exact identities and next proof are recorded in the [source contract](encyclopedia-source-contract.md#combined-family-bindings-and-resource-closure); numbering gaps and filenames are not joins.

The [legacy map](../../../data/resource-entity-map.json) remains a candidate list built from offset guesses. Do not treat its 162 rows as additional verified mappings. The [verification guide](ghidra-verification.md) defines the remaining Ghidra and original-runtime checks; the [interface audit](../../qa/2026-09-10-interface-parity-audit/README.md) owns visual acceptance.
