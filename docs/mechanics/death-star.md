---
title: "Death Star"
description: "Construction countdown, fire preconditions, nearby warning, planet destruction"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "death-star"
sources:
  - type: "ghidra"
    file: "ghidra/notes/entity-system.md"
  - type: "code"
    file: "crates/rebellion-core/src/death_star.rs"
related:
  - "victory"
tags: ["death-star", "construction", "destruction", "victory"]
---

# Death Star

The Death Star is the Empire's superweapon. It must be constructed, moved into position, and fired at a target system to destroy it.

## What You See

- The Empire can start a Death Star construction project at a system.
- Construction takes 1,825 ticks (~5 in-game years).
- Once complete, the Death Star fleet can move to a target system and fire.
- Alliance systems within 300 coordinate units receive a nearby warning.
- Firing destroys the target planet permanently.

## How It Works

### Construction

`DeathStarSystem::start_construction` begins a project at a system. The `DeathStarConstruction` record tracks `ticks_remaining`, decremented each tick by `DeathStarSystem::advance`.

| Constant | Value | Notes |
|----------|-------|-------|
| `DEATH_STAR_CONSTRUCTION_TICKS` | 1,825 | ~5 in-game years at 365 days/year |

When ticks reach 0, `DeathStarEvent::ConstructionCompleted` fires. The construction record self-clears to prevent re-emitting. Only one construction project can be active at a time.

### Fire Preconditions

`DeathStarSystem::fire()` mirrors `FUN_005617b0` / `FUN_0055f650` from the binary. All must be true:

1. Target system is **not already destroyed** (`!system.is_destroyed`).
2. An Empire Death Star fleet is **present** at the target system (`has_death_star && !is_alliance`).
3. Target is **not Empire-controlled** (no self-destruction).

On success, `DeathStarEvent::PlanetDestroyed` fires. The caller sets `system.is_destroyed = true`.

### Nearby Warning

Each tick, if the Death Star fleet is deployed, `DeathStarSystem::advance` scans all Alliance-controlled systems within `NEARBY_WARNING_RADIUS` (300 coordinate units) and emits `NearbyWarning` events. This triggers intelligence messages for the Alliance player.

Distance uses integer Euclidean geometry:

```
dist_sq = (sys.x - ds.x)^2 + (sys.y - ds.y)^2
warned = dist_sq <= 300^2
```

### Death Star Fleet

The Death Star is tracked as a fleet with `has_death_star = true` (family `0x34` in the binary). Movement uses the standard `MovementState` from `movement.rs`.

## Source Material

- **Ghidra RE**: `ghidra/notes/annotated-functions.md` §FUN_005617b0, `economy-systems.md` §SystemDeathStarNearbyNotif
- **Code**: `crates/rebellion-core/src/death_star.rs` -- `DeathStarSystem`, `DeathStarState`, `DeathStarConstruction`
- **Community**: Prima Strategy Guide Death Star construction timeline

## Related

- [victory.md](victory.md) -- Death Star fire and destruction are victory conditions
