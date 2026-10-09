---
title: "Mod Runtime"
description: "Runtime mod management architecture: discovery, validation, enable/disable, hot reload"
category: "agent-docs"
created: 2026-03-15
updated: 2026-10-07
tags: [modding, hot-reload, toml, rfc7396]
---

# Mod Runtime

`crates/rebellion-data/src/mods.rs` — Runtime mod management: discovery, validation, enable/disable, hot reload.

## Architecture

The mod system has three cooperating layers:

1. **Library layer** (existed since v0.2.0): `ModManifest`, `ModContent`, `ModLoader`, `ModWatcher`
2. **Runtime layer** (v0.6.0): `ModRuntime`, `ModConfig`, `ModError` — orchestrates the library for the running game
3. **Presentation-content layer**: native Encyclopedia overlays are retained
   outside the world patch map and installed through an immutable candidate
   session

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

// Feed the same resolved order to world and presentation consumers
let errors = runtime.apply_ordered(&mut world, &sorted);

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
Missing configuration starts with an empty enabled set. Existing unreadable or
malformed configuration is retained as a runtime diagnostic instead of being
silently treated as empty. Updates synchronize a same-directory temporary file
before atomically replacing `config.toml`.

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

## Shared Order and Encyclopedia Target

Dependency-first order is deterministic. Whenever more than one ready mod has
no dependency relationship, names provide the lexicographic tie-break. Native
startup and explicit reload resolve that order once and pass it to both world
overlay application and Encyclopedia layer preparation.

`ModContent::from_dir` reserves root `encyclopedia.json`: it retains bounded
bytes or the exact read error in `ModContent::encyclopedia` and never inserts
the target into the `GameWorld` patch map. The application parser then loads
its confined `encyclopedia/assets/*.bmp` references, validates every complete
intermediate session, and atomically replaces only the Encyclopedia snapshot.
Errors preserve the last-known-good snapshot and do not partially update the
world, save body, or simulation state. See `agent_docs/modding.md` for the
author-facing schema.

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

- **Startup**: the app discovers one runtime after base DAT loading and shares
  one resolved order between world and native Encyclopedia consumers
- **UI**: toggle/reload rebuilds native Encyclopedia content from its immutable
  base; explicit reload reuses the resolved order for world overlays
- **Save**: `enabled_mod_list()` provides (name, version) pairs for save metadata
- **Hot reload**: `check_reload(&watcher)` checked each tick (native only)

## WASM

Filesystem discovery and Encyclopedia overlays are native-only. Browser builds
retain the staged base Encyclopedia and no-op filesystem operations; no modded
native/browser parity claim is made.

## Known Limitations

- Mod application serializes/deserializes the entire GameWorld to JSON for patching — works but is O(world_size) per mod
- World patches cannot add entities; the separate Encyclopedia v1 contract can
  add or remove complete presentation topics under its stricter validation
- `enabled_sorted()` returns empty on dependency resolution failure; structured
  dependency errors are retained in `ModRuntime::errors` and surfaced by the
  native Encyclopedia installer
- Filesystem watcher recovery for burst writes and editor rename patterns is
  deferred to the optional W8 authoring-loop checkpoint
