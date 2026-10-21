---
title: "Blockade System"
description: "Fleet-presence blockade mechanics: manufacturing halt and regiments lost running the blockade"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-17
tags: [blockade, manufacturing, fleet, simulation]
---

# Blockade System

`blockade.rs`: Fleet-presence blockade halts manufacturing, shifts the blockader's support every 30 days, and makes regiments carried into a blockaded system roll to survive leaving it.

## Types

| Type | Purpose |
|------|---------|
| `BlockadeState` | Blockaded systems, the loss counters (`FUN_00508c60`) and the battle bits (`+0x88` `0x1000`, `FUN_0050a900`), and each embarked regiment's orbit and withdraw percent, all saved |
| `RunningRegiment` | A regiment whose fleet just left a system with withdraw percent below 100 |
| `SupportShiftTimer` | Timer `0x386` of one blockaded system: its minimum delay and due tick (saved) |
| `BlockadeEvent` | BlockadeStarted / BlockadeEnded / TroopDestroyed / SupportShift |

## API

```rust
let mut events = BlockadeSystem::advance(&mut blockade_state, &world, &tick_events);
let running = BlockadeSystem::running_regiments(&mut blockade_state, &world, &movement, &transport);
events.extend(BlockadeSystem::resolve_running(&running, &rolls, tick)); // one roll per regiment
BlockadeSystem::withdraw_percent(&world, system_key) // -> u8, FUN_0050b310
// Query:
blockade_state.is_blockaded(system_key) // -> bool
blockade_state.blockaded_systems()      // -> &HashSet<SystemKey>
blockade_state.battle_changes(&world)   // -> Vec<(SystemKey, bool)>: each battle bit that changed
```

Each change of a battle bit sweeps the system's adrift characters
(`death_star::sweep_battle_changes`, event `0x304`).

## Blockade Condition

A system is blockaded when:
- At least one **hostile** fleet is present (the side opposite the system's holder, `System::control`)
- **Zero** defending fleets present

A fleet is present only while it is usable (`Fleet::usable`, `FUN_004fe540`):
one of its live capital ships is not on its own leg. A fleet holding only
travelling or destroyed ships, or no ship, neither blockades nor defends.

A neutral (uncontrolled) system is the original's side 3 (`FUN_0055a080`,
`FUN_0050b610`): while it is populated, any one side's fleets blockade it,
and both sides' fleets set its battle bit, as at a held system
(`FUN_0050b8e0`). An unpopulated neutral system is never blockaded.

## Integration: Blockade → Manufacturing

`BlockadeState::blockaded_systems()` is passed to `ManufacturingSystem::advance_with_blockade()` to skip production at blockaded systems. This is the primary gameplay effect.

## Support Shift (timer `0x386`)

A blockade arms timer `0x386` at the system (`BlockadeState::support_shift`); its end disarms it. Every GNPRTB 7694/7696 days (30) the blockader's support moves by 7693 (+1) when it holds the majority, the Alliance's points against 100 minus them (`FUN_00509930`), else by 7695 (-1); a tie counts as no majority. A changed delay restarts the timer. Only a populated system shifts. Apply `SupportShift` with `uprising::apply_support_change`, which halves a change toward the Alliance with a Death Star in the sector. Telemetry: `blockade_support_shift`. Full chain: `ghidra/notes/timer-scheduler.md`, "Timer `0x386`".

A lifted blockade also shifts its whole sector by the losses taken under it, weighed by each side's strength sum (`support_shift::blockade_ended`, `FUN_0055dbf0`). The `blockade_ended` record carries those inputs and their result: `blockader`, `alliance_losses`, `empire_losses`, `alliance_strength`, `empire_strength` and `shift` (the record's `direction` and `amount`; `blockader` and `shift` are null when no loss record or blockader exists). Formula: `ghidra/notes/support-shifts.md`.

## Troop Destruction

Garrisons are never destroyed by a blockade. A regiment added to a blockaded system without a KDY-150 copies `withdraw_percent` (`max(0, 100 - 5 * ships - 2 * squadrons)` with shipped GNPRTB 7684/7685). Landing resets it to 100. When its fleet leaves, `resolve_running` destroys it unless `99 - floor(roll * 100) < percent`. Call `running_regiments` after arrivals and landings and again at the blockade step (a fleet can arrive and be ordered away in one tick), then roll everything collected. Apply `TroopDestroyed` with `TroopTransportState::destroy_embarked`; the app files event `0x340`'s Evacuation Losses notice (`FUN_00495e90`, TEXTSTRA `0x70a8`-`0x70aa`), one per system and tick listing its regiments (`evacuation_losses_messages`). Full chain: `ghidra/notes/blockade-troop-withdrawal.md`.

## Source

- Ghidra RE: `economy-systems.md §4`
- `FUN_0050e820` step 8: fleet active check gates manufacturing
- Event `0x14e` (SystemBlockadeNotif), Event `0x340` (TroopRegDestroyedRunningBlockade)
- `ghidra/notes/blockade-troop-withdrawal.md`: withdraw percent and the running-blockade roll
