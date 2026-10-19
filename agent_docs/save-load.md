---
title: "Save/Load System"
description: "Native and browser save v53, canonical fingerprints, campaign setup, continuation state, troop cargo, embarked tracking, deliveries, and the no-migration rule"
category: "agent-docs"
created: 2026-03-15
updated: 2026-10-16
tags: [save-load, bincode, migration, serialization, wasm, determinism]
---

# Save/Load System

`crates/rebellion-data/src/save.rs` owns native files and browser storage.
`crates/rebellion-app/src/main.rs` converts between a live campaign and the
serializable snapshot. The current format is v53, and it is the only one
that loads.

## Native format (v53)

```text
[magic: 8 bytes "OPENREB\0"]
[version: u32 LE]             — SAVE_VERSION = 53
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
| `game_config` | `GameConfig` |
| `campaign_config` | `CampaignConfig` |
| `troop_transport` | `TroopTransportState` |
| `deliveries` | `DeliveryState` |
| `player_agent` | `PlayerAgent` |

Loading restores the RNG, dual-AI state and repair episodes instead of
reseeding or clearing them. v36 drops the per-system combat cooldowns: the
original re-fights a system every pass while both sides remain
(`FUN_00517270`). `campaign_config` keeps the selected
difficulty, galaxy-size label, player faction, and victory mode. The clock
keeps its original Game Speed and partial day, so a game saved while paused
reloads paused. The save also holds blockade embarked-regiment tracking,
the fleets holding regiments loaded through the Fleet window (v24),
regiments travelling on their own (v25) and squadrons travelling on their
own (v35), the fleet a moving fleet joins on
arrival (v26), the fleet naming mode (v28), and the player's Manage
Garrisons and Manage Production automation, the Agent Advice setting, renamed
fleets and ships, and each production area's destination (v29), each side's
last number per capital ship class (v38; `ghidra/notes/fleet-names.md`),
each character's officer rank, the fleet or system it commands and its
ranked names (v39; `ghidra/notes/character-roster.md`), each system's
side that last brought a fleet or squadron in (v40;
`ghidra/notes/system-squadrons-combat.md`),
`UprisingState`, queue destinations and en-route deliveries, missions with
their member lists, phase, and timer, each travelling member's arrival day
(v41; Move missions and their fleet targets, `ghidra/notes/move-order.md`),
each AI side's character tasks and their counts (v42;
`ghidra/notes/ai-order-emission.md`, "Character tasks (module 12)"), each side's research points pools, passive
timer and exhausted trees, with Research missions in `MissionState` (v43;
`ghidra/notes/research.md`), each AI side's production state (module
phases and lists, goal token, targets and role bits) and production
destinations that name a fleet (v44; `ghidra/notes/ai-production.md`,
`ghidra/notes/production-destination.md`), each character's and special
force's en route flag, set while it stands at its destination before
arriving, and a special force's hidden-mission flag (v45,
`ghidra/notes/move-order.md`, `ghidra/notes/decoy-roll.md`), each AI side's strategic
state: the marks its records keep, module 13's mode and controller with its
sector plan, the regional records with their postures and class I task
counts, the fleet tasks, and the class I character tasks (v46;
`ghidra/notes/ai-order-emission.md`), each side's Research planners with their
claimed members (v47; `ghidra/notes/research.md`), and the MISSNSD and SPECFCSD
records. Each capital ship carries its six damage counters (`ShipInstance::damage_counters`)
and its repair timer `0x380` (`ShipInstance::repair_timer`: due tick and
the fast-repair flag), added in v49 (`ghidra/notes/ship-damage-repair.md`).
`game_config` carries the AI tempo, `AiConfig::day_orders` and
`AiConfig::rounds_per_day` by speed (v52).
Each production module keeps the orders of its last phase still to issue,
one a round, and whether a bundled Destination and Build still owes its
Build round, and each AI side's scheduler keeps its round-robin cursor, its
priority scan start, each module's priority, the orders issued today, the
last round's in-day step, whether the day ended since that round, and the
order of its last round still to apply; the player's advisor agent keeps
its round-robin cursor and the order of its last step still to apply (serde
defaults); each mission keeps how many outcome passes it has run (v52,
the AI round scheduler; `ghidra/notes/ai-order-emission.md`, "Scheduler and
executor"). Each system's economy record keeps the side that last held it by
support alone (`SupportHolder`, `FUN_0055a080`), and the blockade state keeps
each blockaded system's timer `0x386`, its minimum delay and due tick (v52,
`ghidra/notes/timer-scheduler.md`). System `+0x88` bit 11, a usable Death
Star in the sector (`GameWorld::death_star_sectors`), is derived each tick
and after a load, and saved nowhere; the economy record keeps no copy of it
(v53). A native save taken mid-day keeps the clock's part of the day and
each side's last in-day step, so a load resumes at the next due step and
applies the pending order first. A headless save leaves the day open with its
last step run, so the next tick's prologue only closes it.

Each of the world's eleven slotmaps is written as `{slots, free}` (v48,
`crates/rebellion-core/src/world/slot_order.rs`): `slots` is slotmap's own
serde form, which keeps every vacant slot and its version, and `free` is the
order in which inserts will take the vacant slots. slotmap rebuilds its free
list in descending slot order on load, so without `free` a slot freed before
the save would be reused in another order than in the uninterrupted game and
anything walking a map in slot order would play on differently. Loading
rebuilds each map with remove/insert cycles until every slot has its saved
version, then frees the vacant ones in reverse pop order, and rejects a save
whose `free` is not a permutation of its vacant slots. An arena with no live
value is probed and rebuilt with `Default::default()` of its value type as the
inserted placeholder (v51), so an emptied arena keeps its free order too.
The free order is part of the canonical fingerprint.

## Deterministic fingerprints

Unordered sets and maps use stable human-readable serialization for the
fingerprint. Typed-key maps become sorted key/value sequences because JSON
object keys must be strings. The adapters intentionally leave non-human
serialization unchanged, preserving the historical bincode layout. Two saves
of one state therefore differ in bytes (the header timestamp, and map entry
order in the body, which follows each process's `HashMap` seed) while their
fingerprints match.

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
