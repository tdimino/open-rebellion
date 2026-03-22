---
title: "DAT Binary Formats"
description: "Binary format specifications for all .DAT files in Star Wars Rebellion"
category: "agent-docs"
created: 2026-03-11
updated: 2026-03-16
tags: [dat-format, binary-parsing, round-trip-validation]
---

# DAT Binary Formats

All .DAT files are raw little-endian binary. No magic bytes, no alignment padding. Strings live in TEXTSTRA.DLL (Win32 PE string resources), not in DAT files. All fields are u32 or u16.

## Three Structural Patterns

### Pattern 1: Entity Tables

```
Header (16 bytes):
  field1:    u32  (always 1)
  count:     u32  (number of records)
  family_id: u32  (entity class identifier, becomes DatId high byte)
  field4:    u32  (preserved, meaning unknown)

Body:
  count x fixed-size records
```

Common record prefix (first 24 bytes, shared by most entity tables):
```
id: u32, field2: u32, production_family: u32, next_production_family: u32,
family_id: u32, text_stra_dll_id: u16, field7: u16
```

Files: CAPSHPSD, FIGHTSD, TROOPSD, SPECFCSD, MJCHARSD, MNCHARSD, SYSTEMSD, SECTORSD,
       DEFFACSD, MANFACSD, PROFACSD, MISSNSD, FLEETSD, ALLFACSD, UNIQUESD, ABODESD, BASICSD, MANMGRSD

### Pattern 2: Parameter Tables

```
field1:        u32  (always 1)
entries_count: u32
info_length:   u32
info:          [u8; info_length]  (ASCII type name, e.g. "GeneralParamTableEntry")
entries:       entries_count x fixed-size records
```

Sub-variants by info string:
- `"GeneralParamTableEntry"` (44B each): GNPRTB, SDPRTB — 3 u32 + 8/16 i32
- `"IntTableEntry"` (16B each): all *MSTB and gameplay tables — u32 id, u32 field2, i32 threshold, u32 value
- `"SeedTableEntry"` (16B each): SYFCCRTB, SYFCRMTB — u32 id, u32 field2, u32 system_id, u32 packed_facility

### Pattern 3: Seed Tables (9 files)

```
field1:       u32  (always 1)
groups_count: u32
info_length:  u32
info:         [u8; info_length]  (ASCII, "SeedFamilyTableEntry")
groups:       groups_count x SeedGroup

SeedGroup:
  entry: u32, field2: u32, entry_bis: u32, field4: u32, field5: u32
  items_count: u32
  items: items_count x SeedItem  (field1: u32, field2: u32, item_id: u32)
```

Files: CMUNAFTB, CMUNALTB, CMUNCRTB, CMUNEFTB, CMUNEMTB, CMUNHQTB, CMUNYVTB, FACLCRTB, FACLHQTB

## Codec

`ByteReader` / `ByteWriter` in `tools/dat-dumper/src/codec.rs`:
- Reader: `read_u32()`, `read_i32()`, `read_u16()`, `read_bytes(n)`, `position()`, `remaining()`, `assert_exhausted()`
- Writer: `with_capacity(n)`, `write_u32()`, `write_i32()`, `write_u16()`, `write_bytes()`, `into_bytes()`
- All little-endian, sequential cursor

## DatRecord Trait

`tools/dat-dumper/src/dat_record.rs`:
```rust
pub trait DatRecord: Serialize + Sized {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self>;
    fn write_bytes(&self, w: &mut ByteWriter);
}
```

Implemented by file-level structs (e.g. `CapitalShipsFile`). Entry-level structs use `parse_entry`/`write_entry` methods called from the file-level impl.

## Complete File Inventory (51/51)

### Pattern 1 — Entity Tables

| File | Parser | Count | Record Size | Notes |
|------|--------|-------|-------------|-------|
| CAPSHPSD.DAT | `capital_ships` | 30 | 200B | Capital ship classes |
| FIGHTSD.DAT | `fighters` | 8 | 168B | Fighter classes |
| TROOPSD.DAT | `troops` | 10 | 68B | Troop types |
| SPECFCSD.DAT | `special_forces` | 9 | 116B | Special forces types |
| MJCHARSD.DAT | `major_characters` | 6 | 148B | Major characters |
| MNCHARSD.DAT | `minor_characters` | 54 | 148B | Minor characters |
| SYSTEMSD.DAT | `systems` | 200 | 44B | Star systems |
| SECTORSD.DAT | `sectors` | 20 | 36B | Galaxy sectors |
| DEFFACSD.DAT | `defense_facilities` | 6 | 60B | Defense facilities |
| MANFACSD.DAT | `manufacturing_facilities` | 6 | 56B | Manufacturing facilities |
| PROFACSD.DAT | `production_facilities` | 2 | 56B | Production facilities |
| MISSNSD.DAT | `missions` | 25 | 112B | Mission definitions |
| FLEETSD.DAT | `fleets_seed` | 2 | 24B | Fleet seed templates |
| ALLFACSD.DAT | `all_facilities` | 1 | 52B | All-facilities aggregate |
| UNIQUESD.DAT | `entity_table` | 6 | 24B | Unique objects (Death Star, etc.) |
| ABODESD.DAT | `entity_table` | 5 | 24B | Abode/location types |
| BASICSD.DAT | `entity_table` | 16 | 24B | Basic entity catalog |
| MANMGRSD.DAT | `entity_table` | 3 | 24B | Manager entity types |

### Pattern 2 — Parameter and Lookup Tables

| File | Parser | Count | Entry Size | Info String |
|------|--------|-------|------------|-------------|
| GNPRTB.DAT | `general_params` | 213 | 44B | GeneralParamTableEntry |
| SDPRTB.DAT | `side_params` | 35 | 76B | SideParamTableEntry |
| DIPLMSTB.DAT | `int_table` | 10 | 16B | IntTableEntry — diplomacy mission success |
| ESPIMSTB.DAT | `int_table` | 12 | 16B | IntTableEntry — espionage mission success |
| ASSNMSTB.DAT | `int_table` | 12 | 16B | IntTableEntry — assassination success |
| INCTMSTB.DAT | `int_table` | 13 | 16B | IntTableEntry — incite uprising success |
| DSSBMSTB.DAT | `int_table` | 12 | 16B | IntTableEntry — death star sabotage success |
| ABDCMSTB.DAT | `int_table` | 12 | 16B | IntTableEntry — abduction success |
| RCRTMSTB.DAT | `int_table` | 11 | 16B | IntTableEntry — recruitment success |
| RESCMSTB.DAT | `int_table` | 12 | 16B | IntTableEntry — rescue success |
| SBTGMSTB.DAT | `int_table` | 12 | 16B | IntTableEntry — sabotage success |
| SUBDMSTB.DAT | `int_table` | 13 | 16B | IntTableEntry — subdue uprising success |
| ESCAPETB.DAT | `int_table` | 9 | 16B | IntTableEntry — escape probability |
| FDECOYTB.DAT | `int_table` | 14 | 16B | IntTableEntry — fleet decoy success |
| FOILTB.DAT | `int_table` | 14 | 16B | IntTableEntry — mission foil probability |
| INFORMTB.DAT | `int_table` | 8 | 16B | IntTableEntry — informant probability |
| CSCRHTTB.DAT | `int_table` | 5 | 16B | IntTableEntry — covert search probability |
| UPRIS1TB.DAT | `int_table` | 3 | 16B | IntTableEntry — uprising start |
| UPRIS2TB.DAT | `int_table` | 4 | 16B | IntTableEntry — uprising end |
| RLEVADTB.DAT | `int_table` | 14 | 16B | IntTableEntry — rebel evasion probability |
| RESRCTB.DAT | `int_table` | 4 | 16B | IntTableEntry — resource threshold |
| TDECOYTB.DAT | `int_table` | 14 | 16B | IntTableEntry — troop decoy success |
| SYFCCRTB.DAT | `syfc_table` | 8 | 16B | SeedTableEntry — Empire Coruscant facilities |
| SYFCRMTB.DAT | `syfc_table` | 7 | 16B | SeedTableEntry — Empire Coruscant Rim facilities |

### Pattern 3 — Seed Tables (SeedFamilyTableEntry)

| File | Parser | Notes |
|------|--------|-------|
| CMUNAFTB.DAT | `seed_table` | Alliance fleet seed (HQ or Yavin 4) |
| CMUNALTB.DAT | `seed_table` | Alliance army seed |
| CMUNCRTB.DAT | `seed_table` | Empire Coruscant army seed |
| CMUNEFTB.DAT | `seed_table` | Empire fleet seed (Coruscant) |
| CMUNEMTB.DAT | `seed_table` | Empire army seed |
| CMUNHQTB.DAT | `seed_table` | Alliance HQ army seed |
| CMUNYVTB.DAT | `seed_table` | Alliance Yavin army seed |
| FACLCRTB.DAT | `seed_table` | Empire Coruscant facilities seed |
| FACLHQTB.DAT | `seed_table` | Alliance HQ facilities seed |

## Shared Layouts

- `CharacterEntry` (148B) -- defined in `major_characters.rs`, imported directly by `minor_characters.rs`
- `ManufacturingFacility` (56B) -- shared by MANFACSD and PROFACSD via `pub use` in `manufacturing_facilities.rs`
- `EntityTableFile` (24B records) -- generic Pattern 1 for UNIQUESD, ABODESD, BASICSD, MANMGRSD
- `IntTableFile` (16B entries) -- generic Pattern 2 for all *MSTB and gameplay tables
- `SyfcTableFile` (16B entries) -- Pattern 2 for SYFCCRTB and SYFCRMTB

## MISSNSD Field Notes

- `special_force_eligibility`: bitmask matching SPECFCSD `mission_id` bitmask field
- `target_flags`: 0x10000 when mission can target enemy-controlled systems, 0 otherwise
- `max_officers`: maximum officers assignable to mission
- `base_duration`: base duration in game turns (modified by officer skills at runtime)
- `flag_col6..flag_col21`: 16 boolean flags — semantics require Ghidra RE of STRATEGY.DLL

## IntTableEntry Semantics

The `threshold` field (i32) is a signed skill delta: negative = below average, 0 = average, positive = above average. The `value` field is a probability (0-100) or modifier at that threshold level. Tables are keyed by mission type and read at mission execution time.

## Adding a New DAT Parser

1. Determine pattern: check file size, look for 16-byte entity header vs info string
2. Create `tools/dat-dumper/src/types/{name}.rs` with file struct + entry struct
3. Implement `DatRecord` on the file struct
4. Add module to `tools/dat-dumper/src/types/mod.rs`
5. Register in `tools/dat-dumper/src/registry.rs` with `parse_and_dump::<NewFileType>`
6. Run: `PATH="/usr/bin:$PATH" cargo run -p dat-dumper -- --gdata data/base --file NEWFILE.DAT`
7. Round-trip validation runs automatically -- fix parser until bytes match exactly
