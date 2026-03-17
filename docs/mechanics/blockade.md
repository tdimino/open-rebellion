---
title: "Blockade Mechanics"
description: "Hostile fleet blockade: manufacturing halt, troop destruction, blockade state transitions"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "blockade"
sources:
  - type: "ghidra"
    file: "ghidra/notes/economy-systems.md"
  - type: "code"
    file: "crates/rebellion-core/src/blockade.rs"
related:
  - "manufacturing"
  - "uprising"
tags: ["blockade", "manufacturing", "troops", "fleet"]
---

# Blockade Mechanics

A blockade occurs when a hostile fleet is present at a system with no defending fleet. Blockades halt manufacturing and destroy defending troops.

## What You See

- A system under blockade stops all production queue advancement.
- Troops at the blockaded system belonging to the defending faction are destroyed when the blockade begins.
- Sending a defending fleet to the system lifts the blockade.

## How It Works

### Blockade Conditions

A system is blockaded when **all** of these are true:

1. The system has a `controlling_faction` (not neutral/contested).
2. At least one **hostile** fleet is present (fleet faction != controlling faction).
3. **Zero** defending fleets are present (no fleet matching the controlling faction).

### State Transitions

Each tick, `BlockadeSystem::advance` recomputes the blockade set from fleet disposition:

- **BlockadeStarted**: System transitions from unblockaded to blockaded. Fires event `0x14e` (334).
- **BlockadeEnded**: System transitions from blockaded to unblockaded (defender arrived or attacker withdrew).

### Manufacturing Halt

`BlockadeState::is_blockaded(system)` is consulted by `ManufacturingSystem::advance`. Blockaded systems skip queue advancement entirely -- no items are produced while the blockade is active.

### Troop Destruction

When a blockade starts, all troops at the system belonging to the **defending faction** (opposite of the blockader) are destroyed:

```
TroopDestroyed { system, troop, tick }
```

This corresponds to `FUN_00504a00` (`TroopRegDestroyedRunningBlockade`), event `0x340` (832). The caller removes the troop from `GameWorld::troops` and from the system's `ground_units` list.

Troops belonging to the **blockading faction** are not destroyed -- they are the occupiers.

### Neutral Systems

Neutral or contested systems (no `controlling_faction`) cannot be blockaded. The blockade logic only applies when a clear faction ownership exists.

## Source Material

- **Ghidra RE**: `ghidra/notes/economy-systems.md` §4 -- `FUN_0050e820` step 8 (fleet active check gates manufacturing)
- **Code**: `crates/rebellion-core/src/blockade.rs` -- `BlockadeSystem::advance`, `BlockadeState::is_blockaded`
- **Event IDs**: `0x14e` (SystemBlockadeNotif), `0x340` (TroopRegDestroyedRunningBlockade)

## Related

- [manufacturing.md](manufacturing.md) -- Production queue halted during blockade
- [uprising.md](uprising.md) -- Blockades can destabilize loyalty leading to uprisings
