# Research System

`research.rs` — Three independent tech trees per faction (Ship/Troop/Facility).

## Types

| Type | Purpose |
|------|---------|
| `ResearchState` | `alliance: ResearchLevels`, `empire: ResearchLevels`, `projects: Vec<ResearchProject>` |
| `ResearchProject` | `{ tech_type, character, faction_is_alliance, ticks_remaining, total_ticks }` |
| `ResearchResult` | `TechUnlocked { faction_is_alliance, tech_type, new_level }` |
| `TechType` | Ship / Troop / Facility |
| `ResearchLevels` | `{ ship: u32, troop: u32, facility: u32 }` |

## API

```rust
// Each tick: advance all active research projects
let results = ResearchSystem::advance(&mut research_state, &world, &tick_events);

// Query cost for next level:
let cost = ResearchSystem::ticks_for_next_level(&world, true, TechType::Ship, current_level);

// Check if a ship class is available:
ResearchSystem::ship_class_is_available(&world, &state, true, class_key); // -> bool

// Dispatch a project (replaces existing for same faction + tech):
state.dispatch(ResearchProject { ... });
```

## Tech Unlock Rule

A class is buildable when `class.research_order <= faction_level_for_tech_type`. Level 0 units are always available.

## Tick Cost Calculation

`max(research_difficulty)` among all classes at `research_order == current_level + 1`, clamped to `RESEARCH_MIN_TICKS = 10`. Falls back to `RESEARCH_DEFAULT_TICKS = 30` when no classes exist at that order.

## Known Deviation

`advance()` both mutates `ResearchState` (applies level-ups internally) AND returns events. This breaks the pure advance() contract. Deferred fix (v0.5.0) — choose either pure results (caller applies) or stateful (return notifications only).

## Source

- `rebellion2/Faction.cs` — tech tree structure
- `CapitalShipClass::research_order` and `research_difficulty` from DAT
