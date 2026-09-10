---
title: "Save/Load System"
description: "Native and browser save v12, canonical fingerprints, campaign setup, continuation state, and historical migration"
category: "agent-docs"
created: 2026-03-15
updated: 2026-09-10
tags: [save-load, bincode, migration, serialization, wasm, determinism]
---

# Save/Load System

`crates/rebellion-data/src/save.rs` owns native files and browser storage.
`crates/rebellion-app/src/main.rs` converts between a live campaign and the
serializable snapshot. The current format is v12.

## Native format (v12)

```text
[magic: 8 bytes "OPENREB\0"]
[version: u32 LE]             — SAVE_VERSION = 12
[save_name: u32 len + UTF-8]
[timestamp_secs: u64 LE]
[mod_count: u32 LE]           — v4+
  for each mod:
    [name_len: u32 + name: UTF-8]
    [version_len: u32 + version: UTF-8]
[mod_hash: u64 LE]            — FNV-1a over sorted name/version pairs
[fingerprint_version: u16 LE] — v9+
[state_fingerprint: u64 LE]   — v9+
[bincode body: SaveState]
```

The fingerprint is versioned separately from the save format. It is a
domain-separated FNV-1a digest of canonical JSON and is intended for replay
comparison and corruption detection, not authentication.

## SaveState

Every mutable campaign subsystem required by the app is serialized:

| Field | Type |
|-------|------|
| `world` | `GameWorld` |
| `clock` | `GameClock` |
| `manufacturing` | `ManufacturingState` |
| `missions` | `MissionState` |
| `events` | `EventState` |
| `ai` | `AIState` |
| `movement` | `MovementState` |
| `fog_alliance`, `fog_empire` | `FogState` |
| `player_is_alliance` | `bool` |
| `blockade` | `BlockadeState` |
| `uprising` | `UprisingState` |
| `death_star` | `DeathStarState` |
| `research` | `ResearchState` |
| `jedi` | `JediState` |
| `victory` | `VictoryState` |
| `betrayal` | `BetrayalState` |
| `economy` | `EconomyState` |
| `sim_rng` | `Xoshiro256PlusPlus` |
| `ai2` | `Option<AIState>` |
| `repair` | `RepairState` |
| `combat_cooldowns` | `HashMap<SystemKey, u64>` |
| `game_config` | `GameConfig` |
| `campaign_config` | `CampaignConfig` |

The five fields ending with `game_config` began as the v10 continuation
envelope. Loading restores them instead of reseeding RNG or clearing dual-AI,
repair, and combat memory. Save v11 added `campaign_config`, preserving the
selected difficulty, original galaxy-size label, player faction, and Standard
versus Headquarters Only victory mode. Save v12 persists the fleet set already
under repair so `RepairCheckPerformed` remains a true episode-start event
across save/load.

## Deterministic fingerprints

Unordered sets and maps use stable human-readable serialization for the
fingerprint. Typed-key maps become sorted key/value sequences because JSON
object keys must be strings. The adapters intentionally leave non-human
serialization unchanged, preserving the historical bincode layout.

Sequence order that carries gameplay meaning remains ordered. A matching
fingerprint proves only that one recorded logical snapshot matches. F-011B4
adds exact-artifact native/WASM checkpoint equivalence for the seed-42 fixture;
interactive app/playtest and combat-path convergence remain open.

## Migration rules

- v12 is read directly and its stored fingerprint must match.
- v11 is decoded through the exact historical `SaveStateV11` body. Its stored
  fingerprint is checked before migration, repair episodes begin empty because
  v11 stored a unit `RepairState`, and the migrated fingerprint is unverified.
- v10 is decoded through the exact historical `SaveStateV10` body. Its stored
  fingerprint is checked before migration. Faction and difficulty are inferred
  from preserved state; galaxy size and victory mode use explicit Standard
  defaults because v10 did not retain them. The migrated v12 fingerprint is
  reported as unverified.
- v9 is decoded through the exact historical `SaveStateV9` body. Its v9
  fingerprint is checked before migration; v10 continuation fields and v11
  campaign setup receive explicit defaults, and the migrated fingerprint is
  reported as unverified.
- v8 uses the same historical body without a stored fingerprint. It migrates
  with explicit defaults and is reported as unverified.
- v3–v7 are recognized but rejected with an incompatibility explanation.
- Versions newer than v12 and versions older than v3 fail closed.

Do not rely on `#[serde(default)]` to migrate bincode. Bincode is positional.
Changing `SaveState` requires a version bump and an exact legacy body struct.
The checked-in 718-byte v9 fixture was produced by the old writer and protects
the real migration boundary.

## Browser storage

WASM stores base64 bincode and versioned JSON metadata in `localStorage`:

```text
rebellion_save_v12_<slot>
rebellion_meta_v12_<slot>
```

Metadata includes the full save name, game tick, and fingerprint with its
`u64` value encoded as a decimal string so JavaScript cannot truncate it. The
reader falls back through v11, v10, and v9 keys, validates any stored
fingerprint, migrates the body, and writes new saves only under v12 keys.
Delete removes all four generations.

This path is functional but not the production persistence target: base64 and
synchronous `localStorage` can block the main thread or hit quota limits. M3
moves it to versioned, compressed, asynchronous IndexedDB and adds browser
quota/corruption acceptance tests.

## API

```rust
let fingerprint = save_slot(saves_dir, slot, name, &state, &active_mods)?;
let fingerprint = save_slot_no_mods(saves_dir, slot, name, &state)?;
let (meta, state) = load_slot(saves_dir, slot)?;
let occupied = list_saves(saves_dir);
delete_slot(saves_dir, slot)?;
```

Native saves live at `<saves_dir>/<slot>.reb`. The UI exposes ten slots.

## Safe change checklist

1. Add the field to the current `SaveState` and the live snapshot/restore path.
2. Preserve the previous body exactly in a versioned legacy struct.
3. Bump `SAVE_VERSION` and browser key prefixes.
4. Validate the old stored fingerprint before migration.
5. Define explicit migration defaults and mark migrated fingerprints honestly.
6. Update native and WASM load/list/delete paths.
7. Add current round-trip, corruption, exact-continuation, and real-artifact
   migration tests.
8. Run workspace tests, the seeded fingerprint probe, WASM/package checks, and
   an Astra browser save/reload/load/continue pass with bitmap and error gates.

Current verification evidence:
[F-011A fingerprints](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-state-fingerprints.md)
[F-011B1 continuation](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-state-continuation.md),
[F-011B2 replay contract](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-contract.md),
[F-011B3 replay execution](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-execution.md),
[F-011B4 native/WASM replay equivalence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md),
and [F-016B campaign setup](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md).
