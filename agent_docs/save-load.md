---
title: "Save/Load System"
description: "Native and browser save v63, canonical fingerprints, campaign setup, continuation state, troop cargo, embarked tracking, deliveries, and the no-migration rule"
category: "agent-docs"
created: 2026-03-15
updated: 2026-10-20
tags: [save-load, bincode, migration, serialization, wasm, determinism]
---

# Save/Load System

`crates/rebellion-data/src/save.rs` owns native files and browser storage.
`crates/rebellion-app/src/main.rs` converts between a live campaign and the
serializable snapshot. The current format is v63, and it is the only one
that loads.

## Native format (v63)

```text
[magic: 8 bytes "OPENREB\0"]
[version: u32 LE]             — SAVE_VERSION = 63
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
[bincode body: SavedInterface] — GID mode and open windows, outside the fingerprint
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
| `stockpiles` | `StockpileState` |

Loading restores the RNG, both AI states and repair episodes instead of
reseeding or clearing them. No combat cooldown is saved: the original
re-fights a system every pass while both sides remain (`FUN_00517270`).
What each holder keeps:

| Holder | Saved |
|---|---|
| `GameClock` | Game Speed and the partial day, so a paused save reloads paused |
| `CampaignConfig` | difficulty, galaxy-size label, player faction, victory mode |
| `GameConfig` | the AI tempo: `AiConfig::day_orders`, `AiConfig::rounds_per_day` by speed |
| `GameWorld` | each slotmap's free order (below); the MISSNSD and SPECFCSD records; `system_families` (SYSTEMSD family), `shift_excluded` (setup bit 3), `troop_held` (troop bits, `FUN_0050b610`); `control_change_cap` keyed by day and in-day step (`support_shift::step`, `FUN_00518040`); `support_values` (garrison requirement, support drift, collection rate: `+0x80`, `+0x6c`, `+0x70`); `revolting` (`+0x88` bit 2); `spare_fleets` (fleet `+0x58` bit 1, `FUN_005097d0`); `squadron_transits`; each system's side that last brought a fleet or squadron in (`ghidra/notes/system-squadrons-combat.md`); the fleet naming mode, renamed fleets and ships, each side's last number per capital ship class (`ghidra/notes/fleet-names.md`) |
| `System` | its control, which with both sides' regiments keeps the earlier troop holder (`ControlKind`, `FUN_0050b610`); no stalemate state |
| `Fleet` | `bombarding` (`+0x58` bit 6, `FUN_004fec00`); the fleet a moving fleet joins on arrival |
| `ShipInstance` | `damage_counters` (six); `repair_timer` (`0x380`: due tick, fast flag; `ghidra/notes/ship-damage-repair.md`); `completed` (`+0x50` bit 2: an ordered ship exists from its order, unfinished, and its yard completes it in place, `FUN_0052a140`, `FUN_0052bee0`); `transit` (yard to fleet: the system it left, its destination and arrival day, `+0x38`, `+0x3c`, `+0x44`, `FUN_00556430`) |
| `CapitalShipClass` | `family` (0x18-0x1b is the Death Star) |
| `Character` | officer rank, the fleet or system it commands and its ranked names (`ghidra/notes/character-roster.md`); `adrift` (role bit 6, `FUN_00534790`); `abode` (the jail holding it, `FUN_00549580`); `escape_due` (timer `0x38a`); the en route flag |
| Special force | the en route flag; the hidden-mission flag (`ghidra/notes/decoy-roll.md`) |
| `TroopUnit` | class and side only; a fleet's cargo holds only its regiments and their ships |
| `ManufacturingState` | one queue and one Destination per system and production area, including destinations that name a fleet; `QueueItem::carrier` (the ship a squadron or regiment ordered for a fleet boards, `FUN_005385f0`), `QueueItem::ship` (an ordered capital ship's tag) |
| `DeliveryState` | en-route deliveries with their `carrier`, and with `via` and the arrival day when a destroy cascade moved them (`FUN_00515480` -> `FUN_00556390`; the next leg starts there, `FUN_00556620`) |
| `TroopTransportState` | fleet cargo and regiments travelling on their own, each with its `carrier` (`FUN_00552300`) and `via` |
| `MissionState` | members, phase, timer, outcome passes, `set_out_from` (each member's `+0x70`, `FUN_00536490`), each travelling member's arrival day (`ghidra/notes/move-order.md`); Research missions |
| `AIState` (each side) | character tasks and their counts; production modules (phases, lists, goal token, targets, role bits, the last phase's orders still to issue, a bundled Destination and Build's owed round); module 13's mode, controller and sector plan; regional records with postures and class I task counts; fleet tasks; record marks; Research planners and their members; the scheduler's cursor, priority scan start, module priorities, orders today, last in-day step, day-ended flag and pending order (`ghidra/notes/ai-order-emission.md`) |
| `PlayerAgent` | Manage Garrisons and Manage Production, the Agent Advice setting, round-robin cursor and pending order |
| `ResearchState` | points pools, passive timer `0x381`, exhausted trees (`ghidra/notes/research.md`) |
| `EconomyState` | the side that last held each system by support alone (`SupportHolder`, `FUN_0055a080`) |
| `BlockadeState` | blockaded systems; embarked regiments; timer `0x386` per system with its minimum delay and due tick (`ghidra/notes/timer-scheduler.md`); loss counters (`FUN_00508c60`); battle bits (`FUN_0050a900`) |
| `UprisingState` | revolts and each revolt's next `0x384` day (`ActiveUprising::next_support_tick`) |
| `VictoryState` | the HQs and the win word (`FUN_005180e0`) |
| `StockpileState` | each side's raw and refined materials and the facility cycles |

Saved nowhere: bit 11 (`GameWorld::death_star_sectors`, rebuilt after a
load; `systems/death-star.md`), whether a fleet has the Death Star
(`Fleet::has_death_star` asks its capital ships, `FUN_005003d0`), and the
posted support shifts, regiment events and holder changes (served by the
operation that raises them; see "API"). The Death Star has no state of its
own: it is the capital ship of family 0x18-0x1b.

A native save taken mid-day keeps the clock's part of the day and each
side's last in-day step, so a load resumes at the next due step and
applies the pending order first. A headless save leaves the day open with its
last step run, so the next tick's prologue only closes it.

Each of the world's eleven slotmaps is written as `{slots, free}`
(`crates/rebellion-core/src/world/slot_order.rs`): `slots` is slotmap's own
serde form, which keeps every vacant slot and its version, and `free` is the
order in which inserts will take the vacant slots. slotmap rebuilds its free
list in descending slot order on load, so without `free` a slot freed before
the save would be reused in another order than in the uninterrupted game and
anything walking a map in slot order would play on differently. Loading
rebuilds each map with remove/insert cycles until every slot has its saved
version, then frees the vacant ones in reverse pop order, and rejects a save
whose `free` is not a permutation of its vacant slots. An arena with no live
value is probed and rebuilt with `Default::default()` of its value type as the
inserted placeholder, so an emptied arena keeps its free order too.
The free order is part of the canonical fingerprint.

## Interface section

The body ends with a `SavedInterface` (`rebellion_core::saved_interface`):
the GID display mode's command byte and the galaxy view's child windows,
bottom of the z-order first. Each window is a `SavedWindowKind` (Sector,
System, Fleet with its tab, Defenses, Missions; subjects by `DatId`), its
top-left on the 640x480 canvas, and whether it is shown or on the minimized
rail. It mirrors the original's third save section, which follows the world
(`FUN_00440ea0` writes it, `FUN_00440f80` reads it back), and leaves out the
original's ending code and its "Battle in Progress..." overlay window, which
the port keeps elsewhere or lacks. `compute_state_fingerprint` reads only
`SaveState`, so two saves of one campaign with different windows open share
a fingerprint; the decoder refuses any bytes after the interface. A load
gives the focus to the top restored window that is shown and not a sector
window (`GalaxyFocusState::after_load`); a Fleet window there also takes the
keyboard (`FUN_00440f80.c:31-35`).

## Deterministic fingerprints

Unordered sets and maps use stable human-readable serialization for the
fingerprint. Typed-key maps become sorted key/value sequences because JSON
object keys must be strings. The adapters intentionally leave non-human
serialization unchanged, preserving the bincode layout. Two saves
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
rebellion_save_v63_<slot>
rebellion_meta_v63_<slot>
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
let fingerprint = save_slot_with_interface(saves_dir, slot, name, &state, &interface, &active_mods)?;
let (meta, state, interface) = load_slot_with_interface(saves_dir, slot)?;
let occupied = list_saves(saves_dir);
```

Native saves live at `<saves_dir>/<slot>.reb`. The save layer keeps
`MAX_SAVE_SLOTS` (6) slots, the Game Options window's six rows (`SAVE_ROWS`,
rows 1-6 of `FUN_00407180`); a compile-time check in `rebellion-app` holds
the two equal; `save_slot` and `load_slot` refuse a slot past the sixth.
The save layer has no delete; no Game Options control removes a save.

A save refuses a world holding raised events not yet served
(`GameWorld::support_events`, `GameWorld::manager_side_changes`,
`GameWorld::autoscrap_systems`): none is saved, and every operation that
raises them serves them before it returns (`integrator::fire_holder_changes`
for the first two; `death_star::serve_holder_requests`, run right after it
at each of the operation's call sites, for the third), so a save holding
them would drop work the running game still does.

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
