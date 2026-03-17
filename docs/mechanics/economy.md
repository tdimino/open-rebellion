---
title: "Economy Overview"
description: "Energy/material production, maintenance, resource flow from mines to manufacturing"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "economy"
sources:
  - type: "ghidra"
    file: "ghidra/notes/economy-systems.md"
  - type: "code"
    file: "crates/rebellion-core/src/manufacturing.rs"
  - type: "prima"
    page: "Chapter 3: Economy & Systems"
related:
  - "manufacturing"
  - "diplomacy"
tags: ["economy", "resources", "energy", "materials", "maintenance"]
---

# Economy Overview

## What You See

Each star system produces energy and raw materials. These resources power manufacturing facilities that build ships, troops, and structures. Maintaining your fleet and garrison costs resources every turn. If a system is blockaded, its production halts. Managing the resource pipeline — from mines to refineries to shipyards — is the backbone of your war effort.

## How It Works

### Resource Fields

Each `CStarSystem` tracks four resource fields (confirmed from Ghidra at C++ offsets):

| Offset | Field | Range | Purpose |
|--------|-------|-------|---------|
| `+0x5c` | `system_energy` | >= 0 | Current energy pool |
| `+0x60` | `system_energy_allocated` | >= 0 | Energy committed to build queues |
| `+0x64` | `system_raw_material` | >= 0 | Current raw material pool |
| `+0x68` | `system_raw_material_allocated` | >= 0 | Materials committed to build queues |

Allocated fields prevent over-commitment — when a build queue reserves resources, they move from the pool to the allocated counter.

### Additional System Stats

| Offset | Field | Range |
|--------|-------|-------|
| `+0x58` | `base_loyalty` | 0-100 |
| `+0x6c` | `production_modifier` | -100 to +100 |
| `+0x70` | `smuggling_percent` / `troop_withdraw_percent` | >= 0 |
| `+0x74` | `troop_reg_surplus` | 0-100 |

### Resource Flow

```
Mines (production facilities)
  -> Raw materials generated per tick
    -> Refineries process raw -> refined materials
      -> Manufacturing facilities consume refined materials
        -> Ships, troops, facilities produced
```

Energy flows in parallel:
```
Power generators
  -> Energy generated per tick
    -> Manufacturing facilities consume energy during production
```

### Maintenance Costs

Every unit in the field has a maintenance cost (`maintenance_cost` on `CapitalShipClass`). Each tick, the total maintenance across all units is deducted from the faction's resource pool. Units that cannot be maintained suffer readiness penalties.

### Production Modifier

The `production_modifier` field (offset `+0x6c`, range -100 to +100) scales a system's output up or down. Positive values boost production; negative values penalize it. Sabotage missions and system events can set this field.

### Blockade Effect on Economy

A blockaded system's manufacturing queues freeze entirely — `ManufacturingSystem::advance_with_blockade()` skips blockaded systems. Additionally, troop regiments caught transiting through a blockade are destroyed (event `0x340`: `TroopRegDestroyedRunningBlockade`).

The blockade→production link from the C++ source: `BattlePendingCausedCurrentBlockade` fires when a hostile fleet arrives, simultaneously setting both blockade and battle pending states. `BlockadeAndBattlePendingManagementRequired` fires when both conditions are active, telling the game to halt manufacturing and resolve combat before normal operations resume.

### System Notification Architecture

All resource changes propagate through a notification/observer pattern:

1. **Setter validates** the new value (range check)
2. **Writes** the field
3. **Notifies** both faction observers
4. **Fires vtable dispatch** with old/new values
5. **Registers event** with the scripted event system (if applicable)

This allows the UI, AI, and event system to all react to economy changes without polling.

### Event IDs (Economy)

| ID | Event |
|----|-------|
| `0x141` | SystemEnergy changed |
| `0x142` | SystemEnergyAllocated changed |
| `0x143` | SystemRawMaterial changed |
| `0x144` | SystemRawMaterialAllocated changed |
| `0x148` | TroopRegSurplus changed |
| `0x340` | TroopRegDestroyedRunningBlockade |

## Source Material

- Ghidra: [economy-systems.md](../../ghidra/notes/economy-systems.md) — complete field layout, notification architecture, production eligibility
- Rust: `crates/rebellion-core/src/manufacturing.rs` — `ManufacturingState`, blockade-aware advance

## Related

- [Manufacturing & Production](manufacturing.md)
- [Diplomacy & Popularity](diplomacy.md)
