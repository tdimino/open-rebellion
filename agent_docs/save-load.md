---
title: "Save/Load System"
description: "Native and browser save v30, canonical fingerprints, campaign setup, continuation state, troop cargo, embarked tracking, deliveries, and the no-migration rule"
category: "agent-docs"
created: 2026-03-15
updated: 2026-10-06
tags: [save-load, bincode, migration, serialization, wasm, determinism]
---

# Save/Load System

`crates/rebellion-data/src/save.rs` owns native files and browser storage.
`crates/rebellion-app/src/main.rs` converts between a live campaign and the
serializable snapshot. The current format is v30, and it is the only one
that loads.

## Native format (v30)

```text
[magic: 8 bytes "OPENREB\0"]
[version: u32 LE]             — SAVE_VERSION = 30
[save_name: u32 len + UTF-8]
[timestamp_secs: u64 LE]
[mod_count: u32 LE]
  for each mod:
    [name_len: u32 + name: UTF-8]
    [version_len: u32 + version: UTF-8]
[mod_hash: u64 LE]            — FNV-1a over sorted name/version pairs
[fingerprint_version: u16 LE]
[state_fingerprint: u64 LE]
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
| `manufacturing` | `ManufacturingState`: one queue and one Destination per system and production area (v30) |
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
| `troop_transport` | `TroopTransportState` |
| `deliveries` | `DeliveryState` |
| `player_agent` | `PlayerAgent` |

Loading restores the RNG, dual-AI state, repair episodes, and combat memory
instead of reseeding or clearing them. `campaign_config` keeps the selected
difficulty, galaxy-size label, player faction, and victory mode. The clock
keeps its original Game Speed and partial day, so a game saved while paused
reloads paused. The save also holds blockade embarked-regiment tracking,
the fleets holding regiments loaded through the Fleet window (v24),
regiments travelling on their own (v25), the fleet a moving fleet joins on
arrival (v26), the fleet naming mode (v28), and the player's Manage
Garrisons and Manage Production automation, the Agent Advice setting, renamed
fleets and ships, and each production area's destination (v29),
`UprisingState`, queue destinations and en-route deliveries, missions with
their member lists, phase, and timer, members travelling to a mission
target, and the MISSNSD and SPECFCSD records.

## Deterministic fingerprints

Unordered sets and maps use stable human-readable serialization for the
fingerprint. Typed-key maps become sorted key/value sequences because JSON
object keys must be strings. The adapters intentionally leave non-human
serialization unchanged, preserving the historical bincode layout.

Sequence order that carries gameplay meaning remains ordered. A matching
fingerprint proves only that one recorded logical snapshot matches. F-011B4
adds exact-artifact native/WASM checkpoint equivalence for the seed-42 fixture;
interactive app/playtest and combat-path convergence remain open.

## No migration

The port has no released saves, so the loader reads only `SAVE_VERSION`. Any
other version is rejected with a message to start a new game.

Changing `SaveState`, or any type it holds, including `GameWorld`, changes the
bincode layout. Bincode is positional, so `#[serde(default)]` does not keep old
files readable. Bump `SAVE_VERSION` for every layout change. Once saves are
released, restore versioned migration before the next layout change.

## Browser storage

WASM stores base64 bincode and versioned JSON metadata in `localStorage`:

```text
rebellion_save_v26_<slot>
rebellion_meta_v26_<slot>
```

Metadata includes the full save name, game tick, and fingerprint with its
`u64` value encoded as a decimal string so JavaScript cannot truncate it. The
key prefix follows `SAVE_VERSION`, so entries from older builds are ignored.
Their stale entries remain in `localStorage` until the browser clears them.

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

1. Add the field to `SaveState` and the live snapshot/restore path.
2. Bump `SAVE_VERSION`. The browser key prefixes follow it.
3. Add round-trip, corruption, and exact-continuation tests.
4. Regenerate the seed-42 replay golden, since the version is hashed into
   every fingerprint, and name the cause in the commit.
5. Run workspace tests, the seeded fingerprint probe, WASM/package checks, and
   an independent browser save/reload/load/continue pass with bitmap and error gates.

Current verification evidence:
[F-011A fingerprints](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-state-fingerprints.md)
[F-011B1 continuation](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-state-continuation.md),
[F-011B2 replay contract](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-contract.md),
[F-011B3 replay execution](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-execution.md),
[F-011B4 native/WASM replay equivalence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md),
[F-016B campaign setup](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md),
and [F-007E troop transport and occupation](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-troop-transport-occupation.md).
