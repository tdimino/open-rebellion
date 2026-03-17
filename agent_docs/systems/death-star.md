---
title: "Death Star System"
description: "Construction countdown, planet destruction, and nearby-warning scan mechanics"
category: "agent-docs"
created: 2026-03-14
updated: 2026-03-16
---

# Death Star System

`death_star.rs` — Construction countdown, planet destruction, and nearby-warning scan.

## Types

| Type | Purpose |
|------|---------|
| `DeathStarState` | `under_construction: Option<DeathStarConstruction>`, `death_star_fleet: Option<FleetKey>` |
| `DeathStarEvent` | ConstructionCompleted / PlanetDestroyed / NearbyWarning |

## API

```rust
// Each tick: advance construction, scan for nearby warnings
let events = DeathStarSystem::advance(&mut ds_state, &world, &tick_events);

// Player/AI fires superlaser (precondition-checked):
let fired = DeathStarSystem::fire(&world, target_system, tick); // Option<DeathStarEvent>

// Start construction at a system:
DeathStarSystem::start_construction(&mut ds_state, system); // -> bool (false if already building)
```

## Construction

- Default duration: `DEATH_STAR_CONSTRUCTION_TICKS = 1825` (~5 in-game years)
- `ticks_remaining` decrements each tick via `saturating_sub`
- Self-clears `under_construction = None` on completion (prevents re-emission every tick)

## Superlaser Fire Preconditions

From `FUN_005617b0` + `FUN_0055f650`:
1. Target not already destroyed (`!sys.is_destroyed`)
2. Empire Death Star fleet present at target system
3. Target not Empire-controlled (no self-destruction)

## Nearby Warning

Scans all Alliance-controlled systems within `NEARBY_WARNING_RADIUS = 300` coordinate units of the Death Star fleet. Emits `NearbyWarning` for each. Used for Alliance intelligence messages.

## Source

- `ghidra/notes/annotated-functions.md` § FUN_005617b0
- `ghidra/notes/economy-systems.md` § SystemDeathStarNearbyNotif
- `entity-system.md §4.2` — alive_flag inverted semantics
