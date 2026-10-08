---
title: "Mod System"
description: "TOML manifest + JSON overlay mod system with directory layout and hot reload"
category: "agent-docs"
created: 2026-03-15
updated: 2026-10-07
tags: [modding, toml, semver, merge-patch]
---

# Mod System

Open Rebellion supports user mods via a TOML manifest and JSON overlay system.
World overlays are implemented in `crates/rebellion-data/src/mods.rs`; native
Encyclopedia presentation overlays use the same manifests and dependency order
without entering `GameWorld` or campaign saves.

## Directory Layout

```
mods/
├── better-star-destroyers/
│   ├── mod.toml                     # manifest (required)
│   ├── capital_ship_classes.json    # entity patches
│   └── fighter_classes.json
├── rebel-rebalance/
│   ├── mod.toml
│   └── troop_classes.json
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

Each `.json` file in the mod directory patches one entity category. `ModRuntime::apply_enabled` serializes the live `GameWorld` to JSON and looks up each overlay by literal top-level key — so the filename stem must exactly match a `GameWorld` field name (`crates/rebellion-core/src/world/mod.rs`), e.g. `capital_ship_classes.json` targets `GameWorld::capital_ship_classes`. There is no aliasing: a mismatched stem (e.g. `capital_ships.json`) is silently skipped with an `unknown arena` warning.

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

The reserved root filename `encyclopedia.json` is not a world arena. It is
parsed by the native Encyclopedia content path described below.

## Encyclopedia Overlay (Native Only)

An enabled mod may provide `encyclopedia.json` and referenced bitmap files:

```text
mods/my-encyclopedia-mod/
├── mod.toml
├── encyclopedia.json
└── encyclopedia/assets/my-cruiser.bmp
```

```json
[
  {
    "id": 335544384,
    "title": "Renamed cruiser"
  },
  {
    "id": 335544385,
    "action": "add",
    "text_resource_id": 10049,
    "title": "Escort frigate",
    "body": "Author-supplied description.",
    "image": {"path": "encyclopedia/assets/escort.bmp"}
  }
]
```

Each object selects one numeric Encyclopedia object ID. Fields distinguish
three states: an absent field inherits the lower layer, a value replaces it,
and `null` explicitly removes it where the complete session remains legal.
Unknown fields, duplicate selectors, unsafe paths, and invalid candidates are
rejected.

Supported actions are:

- `patch` (the default): update only present `title`, `body`, or `image` fields;
- `replace`: require replacement values for `title`, `body`, and `image`;
- `add`: create a new topic in a supported non-system family, requiring a new
  object ID, unique nonzero `text_resource_id`, title, body, and image;
- `remove`: remove the whole selected topic and include no replacement fields.

An existing topic's `text_resource_id` is immutable, and a title cannot be
removed. Explicitly removing body or image is accepted only when the resulting
topic is one of the source-approved empty records; every layer must produce a
complete valid session before the next layer is considered.

Image paths must remain beneath `encyclopedia/assets/`, use a lowercase `.bmp`
extension, and traverse no symlink. The referenced file must be a regular
400-by-200 uncompressed 8-bit indexed BMP. Inputs are bounded to 16 MiB for the
overlay, 32 MiB per image, 128 MiB of retained images per mod, and 10,000 topic
patches.

The native runtime validates the immutable base first, applies enabled layers
in the shared dependency order, and publishes the complete candidate in one
atomic session swap. Later layers win per field. Any dependency, JSON, artwork,
or final-session error preserves the last-known-good session; disabling all
layers rebuilds the exact base. Browser builds intentionally use the staged
base catalog and do not load filesystem mods.

## Load Order

Mods are sorted topologically via Kahn's algorithm:

1. `ModLoader::discover(mods_dir)` — scan for `mod.toml` in subdirectories
2. `ModLoader::resolve_load_order(manifests)` — validates and sorts:
   - All declared dependencies must exist in the discovered set
   - Installed versions must satisfy semver requirements
   - No dependency cycles allowed (detected via in-degree analysis)
   - Ready mods with no ordering relationship use a lexicographic name tie-break
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
| `ModContent` | World patch map plus a separately retained reserved Encyclopedia target |
| `ModContentTarget` | Missing, bounded bytes, or a retained read error for `encyclopedia.json` |
| `ModLoader` | Stateless namespace: `discover()`, `resolve_load_order()`, `apply()` |
| `ModWatcher` | File system watcher (native) / no-op (WASM) |
| `ModRuntime` | Runtime orchestrator: discover, validate, apply enabled, toggle, refresh |
| `ModConfig` | Persisted enable/disable state (`mods/config.toml`) |
| `ModError` | Structured error: MissingDependency, VersionMismatch, ParseError |
| `ModInfo` | Display-only struct for UI panel (rebellion-render, no data dep) |

## Integration Points

1. **Startup**: the app discovers one `ModRuntime`, resolves one enabled order, and feeds it to world and native Encyclopedia consumers after base loading
2. **UI**: the Mod Manager toggles the persisted enabled set; native Encyclopedia content is rebuilt from its immutable base
3. **Save**: `ModRuntime::enabled_mod_list()` provides (name, version) pairs for save metadata
4. **Hot reload**: `ModRuntime::check_reload(&watcher)` checked each tick (native only)

For full runtime details see `agent_docs/mod-runtime.md`.

## Creating a Mod

1. Create a directory under `mods/` with a `mod.toml`
2. Add JSON overlay files named after entity categories
3. Each overlay is an array of patch objects with `"id"` matching `dat_id`
4. Test: run the game — mod loader auto-discovers and applies on startup
5. Use the Mod Manager reload action after editing files; filesystem-watcher
   recovery for the complete Encyclopedia authoring loop remains W8 work
