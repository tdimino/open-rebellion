---
title: "Fleet Movement"
description: "Hyperspace transit: speed model, Han Solo bonus, MovementOrder lifecycle, arrival events"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "fleet-movement"
sources:
  - type: "ghidra"
    file: "ghidra/notes/entity-system.md"
  - type: "code"
    file: "crates/rebellion-core/src/movement.rs"
related:
  - "fog-of-war"
  - "space-combat"
tags: ["fleet", "movement", "hyperdrive", "han-solo"]
---

# Fleet Movement

Fleets travel between star systems via hyperspace. Transit speed depends on the slowest ship in the fleet, modified by character bonuses.

## What You See

- Fleets in transit show a diamond icon moving along a dashed route line.
- ETA labels show remaining ticks until arrival.
- Faster hyperdrives and Han Solo's bonus shorten travel time.

## How It Works

### Speed Model

Each `CapitalShipClass` has a `hyperdrive` rating (u32, higher = faster). A fleet's transit speed is determined by its **slowest** ship:

```
ticks_per_hop = BASE_TICKS_PER_HOP / slowest_hyperdrive_rating
```

| Constant | Value | Meaning |
|----------|-------|---------|
| `BASE_TICKS_PER_HOP` | 30 | Base ticks for hyperdrive rating 1 |
| `DEFAULT_TICKS_PER_HOP` | 20 | Used for fleets with no capital ships |
| `MIN_TICKS_PER_HOP` | 5 | Floor -- prevents instant travel |

### Han Solo Speed Bonus

Characters with a non-zero `hyperdrive_modifier` (field +0x9a, `MissionHyperdriveModifierNotif`) reduce ticks per hop when assigned to a fleet:

```
final_ticks = max(base_ticks - best_modifier, MIN_TICKS_PER_HOP)
```

The best `hyperdrive_modifier` among all characters in the fleet is used. Han Solo is the canonical beneficiary.

### MovementOrder Lifecycle

1. **Dispatch**: Player issues a move order. `MovementState::order()` creates a `MovementOrder` with `fleet`, `origin`, `destination`, and `ticks_per_hop`.
2. **Advance**: Each tick, `MovementSystem::advance()` increments `ticks_elapsed` by the tick count. Progress = `ticks_elapsed / ticks_per_hop`.
3. **Arrival**: When `ticks_elapsed >= ticks_per_hop`, an `ArrivalEvent` fires containing the fleet key, origin, and destination system.
4. **Apply**: The caller updates `fleet.location = destination`, removes the fleet from `origin_system.fleets`, and adds it to `dest_system.fleets`.

At most one order per fleet -- issuing a new order cancels the previous one.

### Advance Intel

When a fleet reaches 50% transit progress, the fog-of-war system reveals the destination system to the fleet's faction (see [fog-of-war.md](fog-of-war.md)).

## Source Material

- **Ghidra RE**: `ghidra/notes/entity-system.md` §1.1 -- `HyperdriveModifier` at +0x9a, `MissionHyperdriveModifierNotif`
- **Code**: `crates/rebellion-core/src/movement.rs` -- `fleet_ticks_per_hop`, `MovementSystem::advance`
- **Community**: Prima Strategy Guide fleet speed tables

## Related

- [fog-of-war.md](fog-of-war.md) -- Advance intel at 50% transit
- [space-combat.md](space-combat.md) -- Combat triggers on fleet arrival
