---
title: "Victory System"
description: "Game-ending condition detection for Alliance and Empire"
category: "agent-docs"
created: 2026-03-14
updated: 2026-09-10
tags: [victory, hq-capture, death-star, simulation]
---

# Victory System

`victory.rs` — Game-ending condition detection.

## Types

| Type | Purpose |
|------|---------|
| `VictoryState` | `{ alliance_hq, empire_hq, death_star_active, death_star_location, resolved }` |
| `VictoryOutcome` | HqCaptured / HqDestroyed / DeathStarVictory |

## API

```rust
// Each tick: check all victory conditions
if let Some(outcome) = VictorySystem::check(
    &victory_state,
    &world,
    &tick_events,
    campaign_config.victory_conditions,
) {
    victory_state.resolved = true;
    // show victory/defeat screen
}
```

## Implemented Contract

The 2026-09-10 victory checkpoint implements the asymmetric contract in the
[official campaign reference](../../docs/reference/campaign-history/official-campaign-contract.md#standard-victory).

## Current Victory Conditions

The active `VictoryConditions` selects one of two rule sets.

### Standard

Both the headquarters objective and principal leaders are required:

- **Alliance victory**: Alliance controls `empire_hq` (Coruscant) and holds
  Emperor Palpatine and Darth Vader captive.
- **Empire bombardment victory**: a successful Imperial bombardment clears the
  current Alliance HQ's `is_headquarters` flag; the Empire controls that system
  and holds Luke Skywalker and Mon Mothma captive.
- **Empire Death Star victory**: `alliance_hq.is_destroyed` is true and the
  Empire holds Luke Skywalker and Mon Mothma captive. Planet destruction does
  not require a subsequent ground occupation.

Holding the leaders without the headquarters, using the wrong captor, or
occupying the headquarters without both leaders does not end a Standard game.

### Headquarters Only

- Alliance control of `empire_hq` is sufficient.
- Empire control of `alliance_hq` is sufficient only after the mobile HQ has
  been destroyed by bombardment.
- Death Star destruction of the Alliance-HQ system is sufficient.
- Principal-leader custody is omitted. Death Star loss remains nonterminal.

Both modes evaluate from the first simulation tick. There is no uncited
campaign grace period.

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
- [`main-menu-parity.md`](../main-menu-parity.md) — original game-type selector and setup contract
- [F-016B evidence](../../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md)
- [Campaign-history source ledger](../../docs/reference/campaign-history/sources.json)
- [Victory-contract verification](../../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-victory-contract.md)
