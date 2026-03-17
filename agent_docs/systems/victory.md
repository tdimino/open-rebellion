---
title: "Victory System"
description: "Game-ending condition detection for Alliance and Empire"
category: "agent-docs"
created: 2026-03-14
updated: 2026-03-16
---

# Victory System

`victory.rs` — Game-ending condition detection.

## Types

| Type | Purpose |
|------|---------|
| `VictoryState` | `{ alliance_hq, empire_hq, death_star_active, death_star_location, resolved }` |
| `VictoryOutcome` | HqCaptured / DeathStarVictory / DeathStarDestroyed |

## API

```rust
// Each tick: check all victory conditions
if let Some(outcome) = VictorySystem::check(&victory_state, &world, &tick_events) {
    victory_state.resolved = true;
    // show victory/defeat screen
}
```

## Victory Conditions

Checked in priority order (first match wins):

### 1. Death Star Conditions (checked first, supersede fleet-presence)

- **DeathStarVictory**: Death Star at Alliance HQ AND `sys.is_destroyed` → Empire wins
- **DeathStarDestroyed**: No Empire Death Star fleet at `death_star_location` → Alliance wins

Only checked when `death_star_active = true`.

### 2. HQ Capture

- **Empire captures Alliance HQ**: Empire fleet at `alliance_hq` AND zero Alliance fleets
- **Alliance captures Empire HQ**: Alliance fleet at `empire_hq` AND zero Empire fleets

## Initialization

`VictoryState::new(alliance_hq, empire_hq)` — both SystemKeys found at startup by scanning for `sys.is_headquarters && controlling_faction == Alliance/Empire`.

## Lifecycle

1. `VictorySystem::check()` returns `None` every tick until a terminal condition is met
2. First `Some(VictoryOutcome)` → caller sets `resolved = true`
3. Subsequent calls return `None` (resolved flag prevents re-firing)

## Source

- `entity-system.md §4.2` — `SideVictoryConditionsNotif`, `FinalBattle`
- Event IDs `0x12c`/`0x180`
- `System::is_headquarters` flag
