# Mod Runtime

`crates/rebellion-data/src/mods.rs` — Runtime mod management: discovery, validation, enable/disable, hot reload.

## Architecture

The mod system has two layers:

1. **Library layer** (existed since v0.2.0): `ModManifest`, `ModContent`, `ModLoader`, `ModWatcher`
2. **Runtime layer** (v0.6.0): `ModRuntime`, `ModConfig`, `ModError` — orchestrates the library for the running game

## ModRuntime

```rust
pub struct ModRuntime {
    pub discovered: Vec<ModManifest>,  // all mods found in mods_dir
    pub config: ModConfig,             // which mods are enabled
    pub errors: Vec<ModError>,         // validation errors
    pub mods_dir: PathBuf,
}
```

### Key Methods

```rust
// Create from filesystem scan
let runtime = ModRuntime::discover(&mods_dir);

// Get enabled mods in dependency order
let sorted = runtime.enabled_sorted();

// Apply enabled mods to world (RFC 7396 merge patch)
let errors = runtime.apply_enabled(&mut world);

// Toggle a mod on/off and persist
runtime.toggle_mod("better-star-destroyers");

// Check for file changes (native only)
if runtime.check_reload(&watcher) { runtime.refresh(); }

// Get mod list for save metadata
let mods = runtime.enabled_mod_list(); // Vec<(String, String)>
```

## ModConfig

Persisted to `mods/config.toml`:

```toml
enabled = ["better-star-destroyers", "rebel-rebalance"]
```

Loaded with `ModConfig::load(mods_dir)`, saved with `config.save(mods_dir)`.

## ModError

Structured errors (not just eprintln):

```rust
pub enum ModError {
    MissingDependency { mod_name, dep_name },
    VersionMismatch { mod_name, dep_name, required, found },
    ParseError { mod_name, message },
}
```

## DatId Matching

Mod patches match entities by `dat_id` in JSON. `DatId(u32)` is a newtype — serde serializes it as a bare number, not `{"id": N}`. The lookup path tries:
1. `entity["dat_id"]` as bare u64 (correct for serde)
2. `entity["dat_id"]["id"]` as u64 (fallback for hand-crafted patches)
3. `entity["id"]` as u64 (last resort)

## Mods Directory Resolution

`mods/` is resolved as `gdata_path.parent().parent().join("mods")` — a sibling of `data/`, not inside it:

```
open-rebellion/
├── data/base/     ← gdata_path
├── mods/          ← mod directory
│   ├── my-mod/
│   │   ├── mod.toml
│   │   └── capital_ships.json
│   └── config.toml
```

## Integration Points

- **Startup**: `load_game_data()` calls `ModRuntime::discover()` + `apply_enabled()` after base DAT loading
- **UI**: `init_mod_runtime(gdata_path)` returns `Option<ModRuntime>` for the Mod Manager panel
- **Save**: `enabled_mod_list()` provides (name, version) pairs for save metadata
- **Hot reload**: `check_reload(&watcher)` checked each tick (native only)

## WASM

All mod runtime code is `#[cfg(not(target_arch = "wasm32"))]`. No mod support in browser builds.

## Known Limitations

- Mod application serializes/deserializes the entire GameWorld to JSON for patching — works but is O(world_size) per mod
- No additive entity creation — patches can only modify existing entities, not add new ones (despite README claiming otherwise)
- `enabled_sorted()` silently returns empty on any dependency resolution error — no structured error for UI
- `ModConfig::load()` silently drops corrupted config.toml — no diagnostic
