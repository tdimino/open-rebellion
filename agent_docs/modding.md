---
title: "Mod System"
description: "TOML manifest + JSON overlay mod system with directory layout and hot reload"
category: "agent-docs"
created: 2026-03-15
updated: 2026-03-16
---

# Mod System

Open Rebellion supports user mods via a TOML manifest + JSON overlay system. Implementation: `crates/rebellion-data/src/mods.rs` (637 LOC, 15 tests).

## Directory Layout

```
mods/
├── better-star-destroyers/
│   ├── mod.toml              # manifest (required)
│   ├── capital_ships.json    # entity patches
│   └── fighters.json
├── rebel-rebalance/
│   ├── mod.toml
│   └── troops.json
```

## Manifest (mod.toml)

```toml
name = "better-star-destroyers"
version = "1.2.0"
author = "ObiWanModder"
description = "Rebalances Imperial capital ships."

[dependencies]
"rebel-units" = ">=1.0.0"
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | Yes | kebab-case, unique across all mods |
| `version` | String | Yes | Semver (e.g. `"1.2.0"`) |
| `author` | String | No | Display only |
| `description` | String | No | Display only |
| `dependencies` | Map | No | mod name → semver requirement string |

## JSON Overlay (RFC 7396 Merge Patch)

Each `.json` file in the mod directory patches one entity category. The filename stem maps to a GameWorld arena (e.g. `capital_ships.json` → `world.capital_ship_classes`).

```json
[
  { "id": 5, "hull": 2500, "shield_strength": 1800 },
  { "id": 12, "is_alliance": true, "obsolete_field": null }
]
```

Rules (per RFC 7396):
- `"id"` field required — matches entity's `dat_id` numeric value
- Present values **overwrite** the target field
- `null` values **delete** the target field
- Absent fields are **preserved unchanged**

## Load Order

Mods are sorted topologically via Kahn's algorithm:

1. `ModLoader::discover(mods_dir)` — scan for `mod.toml` in subdirectories
2. `ModLoader::resolve_load_order(manifests)` — validates and sorts:
   - All declared dependencies must exist in the discovered set
   - Installed versions must satisfy semver requirements
   - No dependency cycles allowed (detected via in-degree analysis)
   - Returns manifests in dependency-first order

Load sequence: base game data → mods in resolved order. Later mods can override entities set by earlier mods.

## Hot Reload (Native Only)

`ModWatcher` wraps `notify::RecommendedWatcher` behind `#[cfg(not(target_arch = "wasm32"))]`.

```rust
let mut watcher = ModWatcher::new(Path::new("mods/"))?;

// In game loop:
if watcher.changed() {
    // Re-discover, resolve, and re-apply all mods
}
```

On WASM targets, `ModWatcher` is a no-op stub. Browser mod loading would use a file picker or URL fetch (not yet implemented).

## Key Types

| Type | Purpose |
|------|---------|
| `ModManifest` | Parsed `mod.toml` — name, version, author, description, dependencies, enabled |
| `ModContent` | `HashMap<String, Vec<Value>>` — entity type → patch objects from JSON files |
| `ModLoader` | Stateless namespace: `discover()`, `resolve_load_order()`, `apply()` |
| `ModWatcher` | File system watcher (native) / no-op (WASM) |
| `ModRuntime` | Runtime orchestrator: discover, validate, apply enabled, toggle, refresh |
| `ModConfig` | Persisted enable/disable state (`mods/config.toml`) |
| `ModError` | Structured error: MissingDependency, VersionMismatch, ParseError |
| `ModInfo` | Display-only struct for UI panel (rebellion-render, no data dep) |

## Integration Points

1. **Startup**: `load_game_data()` calls `ModRuntime::discover()` + `apply_enabled()` after base DAT loading
2. **UI**: `init_mod_runtime(gdata_path)` returns `Option<ModRuntime>` for the Mod Manager panel
3. **Save**: `ModRuntime::enabled_mod_list()` provides (name, version) pairs for save metadata
4. **Hot reload**: `ModRuntime::check_reload(&watcher)` checked each tick (native only)

For full runtime details see `@agent_docs/mod-runtime.md`.

## Creating a Mod

1. Create a directory under `mods/` with a `mod.toml`
2. Add JSON overlay files named after entity categories
3. Each overlay is an array of patch objects with `"id"` matching `dat_id`
4. Test: run the game — mod loader auto-discovers and applies on startup
5. On native: edits to mod files trigger hot reload automatically
