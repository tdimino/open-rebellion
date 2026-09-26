---
title: "Blockade System"
description: "Fleet-presence blockade mechanics: manufacturing halt and regiments lost running the blockade"
category: "agent-docs"
created: 2026-03-14
updated: 2026-09-26
tags: [blockade, manufacturing, fleet, simulation]
---

# Blockade System

`blockade.rs` — Fleet-presence blockade halts manufacturing; regiments carried into a blockaded system roll to survive leaving it.

## Types

| Type | Purpose |
|------|---------|
| `BlockadeState` | Blockaded systems, plus each embarked regiment's orbit and withdraw percent (not saved) |
| `RunningRegiment` | A regiment whose fleet just left a system with withdraw percent below 100 |
| `BlockadeEvent` | BlockadeStarted / BlockadeEnded / TroopDestroyed |

## API

```rust
let mut events = BlockadeSystem::advance(&mut blockade_state, &world, &tick_events);
let running = BlockadeSystem::running_regiments(&mut blockade_state, &world, &movement, &transport);
events.extend(BlockadeSystem::resolve_running(&running, &rolls, tick)); // one roll per regiment
BlockadeSystem::withdraw_percent(&world, system_key) // -> u8, FUN_0050b310
// Query:
blockade_state.is_blockaded(system_key) // -> bool
blockade_state.blockaded_systems()      // -> &HashSet<SystemKey>
```

## Blockade Condition

A system is blockaded when:
- At least one **hostile** fleet is present (faction opposite to `controlling_faction`)
- **Zero** defending fleets present

Neutral/uncontrolled systems cannot be blockaded.

## Integration: Blockade → Manufacturing

`BlockadeState::blockaded_systems()` is passed to `ManufacturingSystem::advance_with_blockade()` to skip production at blockaded systems. This is the primary gameplay effect.

## Troop Destruction

Garrisons are never destroyed by a blockade. A regiment added to a blockaded system without a KDY-150 copies `withdraw_percent` (`max(0, 100 - 5 * ships - 2 * squadrons)` with shipped GNPRTB 7684/7685). Landing resets it to 100. When its fleet leaves, `resolve_running` destroys it unless `99 - floor(roll * 100) < percent`. Call `running_regiments` after arrivals and landings and again at the blockade step (a fleet can arrive and be ordered away in one tick), then roll everything collected. Apply `TroopDestroyed` with `TroopTransportState::destroy_embarked`. Full chain: `ghidra/notes/blockade-troop-withdrawal.md`.

## Source

- Ghidra RE: `economy-systems.md §4`
- `FUN_0050e820` step 8 — fleet active check gates manufacturing
- Event `0x14e` (SystemBlockadeNotif), Event `0x340` (TroopRegDestroyedRunningBlockade)
- `ghidra/notes/blockade-troop-withdrawal.md` — withdraw percent and the running-blockade roll
