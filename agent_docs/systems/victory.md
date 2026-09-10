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
| `VictoryOutcome` | HqCaptured / DeathStarVictory / DeathStarDestroyed |

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

## Current Implementation and Parity Status

The implementation below is not yet the accepted original-game contract. The
2026-09-10 campaign-history review reopened victory parity after finding three
divergences: occupation is treated as destruction of the mobile Alliance HQ,
Death Star outcomes can bypass Standard leader requirements, and a 200-tick
grace period has no located historical basis. Use the
[official campaign contract](../../docs/reference/campaign-history/official-campaign-contract.md#standard-victory)
as the target behavior.

## Current Victory Conditions

The active `VictoryConditions` selects one of two rule sets.

### Standard

Checked in priority order (first match wins):

### 1. Death Star Conditions (known divergence)

- **DeathStarVictory**: Death Star at Alliance HQ AND `sys.is_destroyed` → Empire wins
- **DeathStarDestroyed**: No Empire Death Star fleet at `death_star_location` → Alliance wins

Only checked when `death_star_active = true`. These immediate terminal outcomes
are not parity-correct: Death Star fire may satisfy the Empire's HQ-destruction
component but Standard mode still requires Luke and Mon Mothma, while loss of a
Death Star is not an independent Alliance win.

### 2. HQ and principal-leader capture

- **Empire victory**: Empire occupies `alliance_hq` and holds both Luke Skywalker and Mon Mothma captive. This incorrectly equates occupation with destruction of the mobile headquarters.
- **Alliance victory**: Alliance occupies `empire_hq` and holds both Emperor Palpatine and Darth Vader captive.

Holding the leaders without the headquarters, using the wrong captor, or
occupying the headquarters without both leaders does not end a Standard game.

### Headquarters Only

- Empire occupation of `alliance_hq` is sufficient for an Empire victory. This
  is a known divergence because the headquarters must be destroyed.
- Alliance occupation of `empire_hq` is sufficient for an Alliance victory.
- Death Star outcomes are ignored; “Only” is enforced literally.

Both modes are also suppressed before `MIN_VICTORY_TICK = 200`. No located
manual rule supports that threshold; it must be removed from the parity profile
or exposed as an optional enhancement.

## Accepted Original Contract

- Alliance headquarters objective: capture and hold Coruscant.
- Imperial headquarters objective: destroy the mobile Alliance headquarters by
  bombardment followed by system assault/control, or destroy its system with a
  Death Star.
- Standard: the headquarters objective and both opposing principal leaders are
  required together.
- Headquarters Only: omit only the principal-leader requirements.
- Destroying a Death Star is consequential but nonterminal by itself.

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
