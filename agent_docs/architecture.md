# Architecture

## Crate Dependency Graph

```
rebellion-app
├── rebellion-render (macroquad 0.4, egui-macroquad 0.17)
│   └── rebellion-core
├── rebellion-data
│   ├── rebellion-core
│   └── dat-dumper (library mode)
└── rebellion-core (slotmap 1.0, serde 1)

dat-dumper (tools/) -- standalone CLI + library
└── clap 4, serde, serde_json, anyhow
```

## Type System: Two Layers

### Layer 1: Binary mirror (`rebellion-core/src/dat/`)
Structs that exactly match .DAT file field layout. Used only for import/export. No game logic.
- `Faction` (Alliance, Empire, Neutral)
- `GalaxySize` (Standard=1, Large=2, Huge=3)
- `SectorGroup` (Core=1, RimInner=2, RimOuter=3)
- `ExplorationStatus` (Explored=0x90, Unexplored=0x92)

### Layer 2: Runtime world (`rebellion-core/src/world/`)
Rich types used by game logic, rendering, save/load. Slotmap keys for all inter-entity references.
- `GameWorld` -- root aggregate, 6 SlotMap arenas (systems, sectors, capital_ship_classes, fighter_classes, characters, fleets). Note: `ids.rs` defines 11 keys total -- the remaining 5 (TroopKey, SpecialForceKey, DefenseFacilityKey, ManufacturingFacilityKey, ProductionFacilityKey) are stubs for future arenas.
- `System` -- position, sector ref, popularity (alliance/empire f32), asset lists (fleets, units, facilities)
- `Sector` -- named region, SectorGroup, position, child system list
- `CapitalShipClass` / `FighterClass` -- class templates, not instances
- `Character` -- 8 `SkillPair` (base+variance), Jedi fields, role flags, major/minor
- `Fleet` -- ships + characters at a system location

## Entity Identity (Dual-Key Pattern)

Every entity carries both:

1. **`DatId(u32)`** -- original binary ID. Encodes `(family_byte << 24) | sequential_index`. Family byte identifies entity class (0x90=explored system, 0x92=unexplored system). Other entity family bytes are encoded in each file's header `family_id` field. Preserved for serialization and mod cross-references.

2. **Slotmap key** (`SystemKey`, `SectorKey`, etc.) -- runtime arena handle with generational index. Only meaningful within the `GameWorld` that created it. Used for all inter-entity references at runtime.

Lookup maps (`HashMap<u32, *Key>`) bridge DatId to slotmap keys during loading in `rebellion-data/src/lib.rs`.

## Data Flow

```
.DAT binary files
    | ByteReader (little-endian cursor, tools/dat-dumper/src/codec.rs)
    v
dat-dumper types (DatRecord trait, tools/dat-dumper/src/types/)
    | rebellion-data::load_game_data() (crates/rebellion-data/src/lib.rs)
    v
world types (GameWorld with SlotMap arenas)
    | rebellion-render (crates/rebellion-render/src/lib.rs)
    v
macroquad drawing + egui panels
```

### Loading Order (rebellion-data/src/lib.rs)
1. Sectors (SECTORSD.DAT) -- must come first, systems reference sectors
2. Systems (SYSTEMSD.DAT) -- registers each system in its parent sector
3. Capital ships (CAPSHPSD.DAT)
4. Fighters (FIGHTSD.DAT)
5. Major characters (MJCHARSD.DAT)
6. Minor characters (MNCHARSD.DAT)

### Round-Trip Validation
Round-trip validation is enforced inside `parse_and_dump` in `tools/dat-dumper/src/registry.rs`. The byte comparison utility is in `tools/dat-dumper/src/validate.rs`. If reserialized bytes differ at any offset, the parse is wrong. This guarantees complete format understanding.

## Rendering Architecture

`rebellion-render/src/lib.rs` exposes two public functions called each frame:
1. `draw_galaxy_map(world, state)` -- star map with pan/zoom/click, faction-colored dots
2. `draw_info_panel(world, state)` -- egui side panel (selected system) + bottom status bar

`GalaxyMapState` holds all mutable UI state: camera position, zoom, selected/hovered system, drag tracking.

Input: mouse within map area only. Right-drag pans, scroll zooms (0.3x-5.0x). Left-click selects nearest system within hover radius.

## Adding a New Entity Type

1. Add slotmap key to `crates/rebellion-core/src/ids.rs` via `new_key_type!`
2. Add dat/ enum or struct if needed (`crates/rebellion-core/src/dat/mod.rs`)
3. Add world struct to `crates/rebellion-core/src/world/mod.rs`, add SlotMap arena to `GameWorld`
4. Add dat-dumper parser in `tools/dat-dumper/src/types/`, add module to `types/mod.rs`, register in `registry.rs`
5. Add conversion step in `crates/rebellion-data/src/lib.rs`
6. Update rendering if the entity is visible on the map
