---
title: "Manufacturing & Production"
description: "Per-system production queues, build types, blockade halt"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "manufacturing"
sources:
  - type: "ghidra"
    file: "ghidra/notes/economy-systems.md"
  - type: "code"
    file: "crates/rebellion-core/src/manufacturing.rs"
  - type: "prima"
    page: "Chapter 4: Production & Resources"
related:
  - "economy"
  - "research"
tags: ["manufacturing", "production", "queue", "blockade"]
---

# Manufacturing & Production

## What You See

Each star system with a manufacturing facility can build ships, troops, fighters, and other facilities. Items are queued and constructed one at a time. A progress bar shows how far along the active item is. Blockaded systems cannot produce anything — their queues freeze until the blockade is lifted.

## How It Works

### Production Queues

Each system has an ordered `ProductionQueue`. Items are processed front-to-back:

- **Active item** (index 0): its `ticks_remaining` decrements by 1 each game-day.
- **Queued items** (index 1+): waiting. They advance when the item ahead completes.
- When `ticks_remaining` reaches 0, the item completes and a `CompletionEvent` is emitted.
- Multiple completions can cascade if several items have small remaining costs and multiple ticks fire in one frame.

### BuildableKind Types

```
CapitalShip(key)           — Star Destroyers, Mon Cal Cruisers, etc.
Fighter(key)               — X-Wings, TIE Fighters, etc.
Troop(key)                 — Ground regiment units
DefenseFacility(key)       — Planetary defense installations
ManufacturingFacility(key) — Shipyards, training centers
ProductionFacility(key)    — Mines, refineries
```

Each kind wraps the slotmap key of the class definition in `GameWorld`. The actual unit instance is created when construction completes.

### Build Duration

Duration is derived from `refined_material_cost` and the system's facility `processing_rate` at queue time. It is stored as `ticks_remaining` on the `QueueItem`, so the queue is self-contained and survives facility changes mid-build.

### Production Eligibility (FUN_0050e820)

The C++ production eligibility checker (1,658 bytes) runs these gates:

1. **System validity**: object alive and initialized
2. **Allegiance gate**: three resource validators must pass (energy, raw material, refined)
3. **Manufacturing facility check**: valid facility count/type at offsets `+0x17`/`+0x18`
4. **Production facility check**: valid facility pair at offsets `+0x19`/`+0x1a`
5. **Entity type filter**: iterates entities, filters by DatId family byte ranges
6. **Flight count check**: in-flight entities must be < 2
7. **Unit readiness check**: all garrisoned units must belong to the controlling faction
8. **Fleet active check**: no active hostile fleet blockade

### Blockade Halts Production

When a system is blockaded (`blocked_systems` set), `ManufacturingSystem::advance_with_blockade()` skips that system's queue entirely. Production freezes — `ticks_remaining` does not decrement — until the blockade ends.

From the C++ source: fleet active flag (`entity+0x50 & 1`) combined with faction mismatch check causes `uVar2 = 0`, aborting the production loop at `LAB_0050edcc`.

### Queue Operations

- **Enqueue**: append to the back of the queue
- **Cancel**: remove item at any position (no refund for active item)
- **Prioritize**: swap item with the one ahead of it (move earlier in queue)

## Source Material

- Ghidra: [economy-systems.md](../../ghidra/notes/economy-systems.md) — `FUN_0050e820` production eligibility, resource validators
- Rust: `crates/rebellion-core/src/manufacturing.rs` — `ProductionQueue`, `ManufacturingState`, `ManufacturingSystem`

## Related

- [Economy Overview](economy.md)
- [Research & Tech Trees](research.md)
