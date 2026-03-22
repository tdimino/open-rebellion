---
title: "Save/Load System"
description: "Bincode serialization with versioned headers, mod metadata, and migration framework"
category: "agent-docs"
created: 2026-03-15
updated: 2026-03-16
tags: [save-load, bincode, migration, serialization]
---

# Save/Load System

`crates/rebellion-data/src/save.rs` — Bincode serialization with versioned headers, mod metadata, and migration framework.

## Format (v4)

```
[magic: 8 bytes "OPENREB\0"]
[version: u32 LE]             — SAVE_VERSION = 4
[save_name: u32 len + UTF-8]
[timestamp_secs: u64 LE]
[mod_count: u32 LE]           — number of active mods (v4+)
  for each mod:
    [name_len: u32 + name: UTF-8]
    [version_len: u32 + version: UTF-8]
[mod_hash: u64 LE]            — FNV-1a hash of sorted (name, version) pairs
[bincode body: SaveState]
```

## SaveState

All simulation states serialized:

| Field | Type |
|-------|------|
| `world` | `GameWorld` (all 11 SlotMap arenas + GNPRTB + mission tables) |
| `clock` | `GameClock` |
| `manufacturing` | `ManufacturingState` |
| `missions` | `MissionState` |
| `events` | `EventState` |
| `ai` | `AIState` |
| `movement` | `MovementState` |
| `fog_alliance` | `FogState` |
| `fog_empire` | `FogState` |
| `player_is_alliance` | `bool` |
| `blockade` | `BlockadeState` |
| `uprising` | `UprisingState` |
| `death_star` | `DeathStarState` |
| `research` | `ResearchState` |
| `jedi` | `JediState` |
| `victory` | `VictoryState` |
| `betrayal` | `BetrayalState` |

## Migration Framework

```rust
match version {
    SAVE_VERSION => bincode::deserialize(&body),  // current version
    3 => bail!("v3 incompatible — Character struct changed"),  // reject
    v if v > SAVE_VERSION => bail!("newer build"),  // future version
    v => bail!("too old to migrate (minimum: 3)"),  // ancient
}
```

**Why v3 can't migrate**: Bincode is a positional format. `#[serde(default)]` is inoperative — it only works with self-describing formats (JSON, TOML). Adding fields to `Character` (captivity fields) changes the byte layout. A true v3→v4 migration would require a `SaveStateV3` struct matching the old layout.

## Mod Metadata

- `compute_mod_hash(mods: &[(String, String)]) -> u64` — FNV-1a over sorted (name:version) pairs
- On load: compare saved `mod_hash` with current active mods. Mismatch is a warning, not a rejection.
- `SaveMeta` includes `mod_names: Vec<String>` and `mod_hash: u64` for UI display.

## API

```rust
// Save (with mod metadata)
save_slot(saves_dir, slot, name, &state, &active_mods)?;

// Save (no mods — backward compat)
save_slot_no_mods(saves_dir, slot, name, &state)?;

// Load (with migration)
let (meta, state) = load_slot(saves_dir, slot)?;

// List all occupied slots
let metas = list_saves(saves_dir);

// Delete
delete_slot(saves_dir, slot)?;
```

## WASM

All IO functions are `#[cfg(not(target_arch = "wasm32"))]`. WASM stubs return `Err("not supported in the browser build")`. No IndexedDB/localStorage fallback yet.

## Adding Fields to SaveState

1. Add the field with `#[serde(default)]` — but note this only helps JSON, NOT bincode
2. Bump `SAVE_VERSION`
3. Add a migration arm in `load_slot()` that either converts or rejects older saves
4. Update `minimal_save_state()` in tests
5. Add a fixture test for the migration path
