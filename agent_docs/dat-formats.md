# DAT Binary Formats

All .DAT files are raw little-endian binary. No magic bytes, no alignment padding. Strings live in TEXTSTRA.DLL (Win32 PE string resources), not in DAT files. All fields are u32 or u16.

## Three Structural Patterns

### Pattern 1: Entity Tables (11 files)

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

Files: CAPSHPSD, FIGHTSD, TROOPSD, SPECFCSD, MJCHARSD, MNCHARSD, SYSTEMSD, SECTORSD, DEFFACSD, MANFACSD, PROFACSD

### Pattern 2: Parameter Tables (2 files)

```
field1:        u32  (always 1)
entries_count: u32
info_length:   u32
info:          [u8; info_length]  (ASCII type name, e.g. "GeneralParamTableEntry")
entries:       entries_count x fixed-size records
```

Files: GNPRTB.DAT (213 entries, 44B each -- 3 u32 + 8 i32), SDPRTB.DAT (35 entries, 76B each -- 3 u32 + 16 i32)

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

## Parsed File Inventory (22/~51)

| File | Pattern | Count | Record Size | Total Size |
|------|---------|-------|-------------|------------|
| CAPSHPSD.DAT | 1 | 30 | 200B | 6,016B |
| FIGHTSD.DAT | 1 | 8 | 168B | 1,360B |
| TROOPSD.DAT | 1 | 10 | 68B | 696B |
| SPECFCSD.DAT | 1 | 9 | 116B | 1,060B |
| MJCHARSD.DAT | 1 | 6 | 148B | 904B |
| MNCHARSD.DAT | 1 | 54 | 148B | 8,008B |
| SYSTEMSD.DAT | 1 | 200 | 44B | 8,816B |
| SECTORSD.DAT | 1 | 20 | 36B | 736B |
| DEFFACSD.DAT | 1 | 6 | 60B | 376B |
| MANFACSD.DAT | 1 | 6 | 56B | 352B |
| PROFACSD.DAT | 1 | 2 | 56B | 128B |
| GNPRTB.DAT | 2 | 213 | 44B | 9,406B |
| SDPRTB.DAT | 2 | 35 | 76B | 2,691B |
| 9 seed tables | 3 | varies | varies | ~2KB total |

### Unparsed (~29 files)
MISSNSD.DAT (missions, 2,816B), SYFC*TB (system facilities), *MSTB (mission tables), escape/decoy/foil/uprising tables. Formats unknown -- need Metasharp analysis or Ghidra RE.

## Shared Layouts

- `CharacterEntry` (148B) -- defined in `major_characters.rs`, imported directly by `minor_characters.rs`
- `ManufacturingFacility` (56B) -- shared by MANFACSD and PROFACSD via `pub use` in `manufacturing_facilities.rs`

## Adding a New DAT Parser

1. Determine pattern: check file size, look for 16-byte entity header vs info string
2. Create `tools/dat-dumper/src/types/{name}.rs` with file struct + entry struct
3. Implement `DatRecord` on the file struct
4. Add module to `tools/dat-dumper/src/types/mod.rs`
5. Register in `tools/dat-dumper/src/registry.rs` with `parse_and_dump::<NewFileType>`
6. Run: `PATH="/usr/bin:$PATH" cargo run -p dat-dumper -- --gdata data/base --file NEWFILE.DAT`
7. Round-trip validation runs automatically -- fix parser until bytes match exactly
