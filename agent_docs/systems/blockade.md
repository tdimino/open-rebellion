# Blockade System

`blockade.rs` — Fleet-presence blockade halts manufacturing and destroys in-transit troops.

## Types

| Type | Purpose |
|------|---------|
| `BlockadeState` | `HashSet<SystemKey>` of currently blockaded systems |
| `BlockadeEvent` | BlockadeStarted / BlockadeEnded / TroopDestroyed |

## API

```rust
let events = BlockadeSystem::advance(&mut blockade_state, &world, &tick_events);
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

When a blockade **starts** (not ongoing), troops belonging to the defending faction at that system are destroyed (`TroopDestroyed` event). Caller removes from `sys.ground_units` and `world.troops`.

## Source

- Ghidra RE: `economy-systems.md §4`
- `FUN_0050e820` step 8 — fleet active check gates manufacturing
- Event `0x14e` (SystemBlockadeNotif), Event `0x340` (TroopRegDestroyedRunningBlockade)
