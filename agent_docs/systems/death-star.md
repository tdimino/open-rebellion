---
title: "Death Star System"
description: "Construction countdown, planet destruction, and nearby-warning scan mechanics"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-16
tags: [death-star, construction, planet-destruction, simulation]
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
let fired = DeathStarSystem::fire(&ds_state, &world, target_system, tick); // Option<DeathStarEvent>

// Start construction at a system:
DeathStarSystem::start_construction(&mut ds_state, system); // -> bool (false if already building)
```

## Construction

- Default duration: `DEATH_STAR_CONSTRUCTION_TICKS = 1825` (~5 in-game years)
- `ticks_remaining` decrements each tick via `saturating_sub`
- Self-clears `under_construction = None` on completion (prevents re-emission every tick)

## Superlaser Fire Preconditions

The port's checks:
1. Target not already destroyed (`!sys.is_destroyed`)
2. Empire Death Star fleet present at target system
3. Target not Empire-controlled (no self-destruction)

## Nearby Warning

Scans all Alliance-controlled systems within `NEARBY_WARNING_RADIUS = 300` coordinate units of the Death Star fleet. Emits `NearbyWarning` for each. Used for Alliance intelligence messages.

## Sector bit 11

`death_star::refresh_sectors` sets `GameWorld::death_star_sectors`, system
`+0x88` bit 11: a usable Death Star (complete, not en route) in the sector
(`FUN_005178b0`). It runs at the start of each tick, after arrivals, after
the battles, and after a load or a new game (`simulation.md`), and is not
saved. Readers: the garrison and Incite halving
(`uprising.md`) and the support-change halving (`blockade.md`). A kill ends
with the bombardment finalize (`FUN_0058e320`, `combat.md`, "Bombardment").

## Original Mechanism

None of the checks above is the original's. `docs/mechanics/death-star.md` § Port lists each divergence and its node; `ghidra/notes/death-star-strategic.md` is the source.

## Source

- `ghidra/notes/death-star-strategic.md`
- `ghidra/notes/bombardment.md` — the Destroy System run
- `ghidra/notes/economy-systems.md` § SystemDeathStarNearbyNotif
