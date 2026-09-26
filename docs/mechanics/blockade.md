---
title: "Blockade Mechanics"
description: "Hostile fleet blockade: manufacturing halt, regiments lost running the blockade, blockade state transitions"
category: "mechanics"
created: 2026-03-16
updated: 2026-09-26
game_system: "blockade"
sources:
  - type: "ghidra"
    file: "ghidra/notes/economy-systems.md"
  - type: "ghidra"
    file: "ghidra/notes/blockade-troop-withdrawal.md"
  - type: "code"
    file: "crates/rebellion-core/src/blockade.rs"
related:
  - "manufacturing"
  - "uprising"
tags: ["blockade", "manufacturing", "troops", "fleet"]
---

# Blockade Mechanics

A blockade occurs when a hostile fleet is present at a system with no defending fleet. Blockades halt manufacturing, and regiments carried into a blockaded system can be lost when their fleet leaves.

## What You See

- A system under blockade stops all production queue advancement.
- Garrisons are never destroyed by a blockade. A regiment that arrives aboard a fleet while the system is blockaded may be destroyed when the fleet leaves without landing it ("Troops destroyed by blockade").
- A KDY-150 at the system protects every departing regiment.
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

### Running the Blockade

Recovered from the binary on 2026-09-26; the full chain is in `ghidra/notes/blockade-troop-withdrawal.md`.

1. Each system has a **withdraw percent** (`FUN_0050b310`). It is 100 unless the system is blockaded and has no active KDY-150 (DEFFACSD record 1). Otherwise:

   ```
   withdraw = max(0, 100 - capital_ships * GNPRTB[7684] - fighter_squadrons * GNPRTB[7685])
   ```

   The shipped values are 5 and 2, and every fleet at the system counts.
2. A regiment **added to a system**, for example by arriving aboard a fleet, copies that value. Later changes to the system's value are not pushed to regiments already there.
3. **Landing** (activating) a regiment resets it to 100, so garrisons and troops loaded from the surface are always safe.
4. When the regiment **enters transit**, it survives if `random(0..=99) < withdraw`. Otherwise it is destroyed, the side is told `TroopRegDestroyedRunningBlockade` (`FUN_00504a00`), and event `0x340` (832) is raised.

```
TroopDestroyed { system, troop, tick }
```

`BlockadeSystem::running_regiments` tracks embarked regiments each tick and `BlockadeSystem::resolve_running` rolls the ones leaving. The caller removes a destroyed regiment from its fleet's cargo and from `GameWorld::troops`. The tracking is not saved (saves are positional bincode), so a regiment restarts at 100 after a load.

### Neutral Systems

Neutral or contested systems (no `controlling_faction`) cannot be blockaded. The blockade logic only applies when a clear faction ownership exists.

## Source Material

- **Ghidra RE**: `ghidra/notes/economy-systems.md` §4 -- `FUN_0050e820` step 8 (fleet active check gates manufacturing)
- **Code**: `crates/rebellion-core/src/blockade.rs` -- `BlockadeSystem::advance`, `BlockadeState::is_blockaded`
- **Ghidra RE**: `ghidra/notes/blockade-troop-withdrawal.md` -- `FUN_0050b310`, `FUN_0055a020`, `FUN_00504990`, `FUN_00504a00`
- **Event IDs**: `0x14e` (SystemBlockadeNotif), `0x340` (TroopRegDestroyedRunningBlockade)

## Related

- [manufacturing.md](manufacturing.md) -- Production queue halted during blockade
- [uprising.md](uprising.md) -- Blockades can destabilize loyalty leading to uprisings
