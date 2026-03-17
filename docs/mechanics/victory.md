---
title: "Victory Conditions"
description: "HQ capture, Death Star fire, Death Star destruction -- three terminal game states"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "victory"
sources:
  - type: "ghidra"
    file: "ghidra/notes/entity-system.md"
  - type: "code"
    file: "crates/rebellion-core/src/victory.rs"
related:
  - "death-star"
  - "space-combat"
tags: ["victory", "headquarters", "death-star", "endgame"]
---

# Victory Conditions

The game ends when one of three terminal conditions is met. `VictorySystem::check` evaluates all conditions each tick and returns the first `VictoryOutcome` detected.

## What You See

- A victory/defeat screen appears when the game ends.
- The outcome describes which condition was met and which faction won.

## How It Works

### Condition 1: HQ Capture

A faction wins by capturing the enemy's headquarters system:

- **Enemy fleet present** at the HQ system AND **no defending fleet** present.
- Alliance HQ captured by Empire fleet → Empire wins.
- Empire HQ captured by Alliance fleet → Alliance wins.

If both an enemy fleet and a defending fleet are present (contested), no capture occurs.

### Condition 2: Death Star Fires on Alliance HQ

If the Death Star is active and located at the Alliance HQ system, and the planet has been destroyed (`system.is_destroyed = true`), the Empire wins with `DeathStarVictory`.

This check takes priority over the HQ capture check -- planet destruction is the terminal event.

### Condition 3: Death Star Destroyed

If the Death Star was active but no Death Star fleet remains at its last known location, the Alliance wins with `DeathStarDestroyed`. This means the Death Star was destroyed in combat.

### Resolution

Once a `VictoryOutcome` is returned, the caller sets `VictoryState::resolved = true`. Subsequent ticks skip victory checks. The game cannot produce multiple outcomes.

### Death Star Priority

Death Star checks run before HQ capture checks. If the Death Star destroys the Alliance HQ, the Empire wins even if an Alliance fleet is present at the Empire HQ.

### VictoryState Configuration

Set at game start:

| Field | Purpose |
|-------|---------|
| `alliance_hq` | SystemKey of the Alliance headquarters |
| `empire_hq` | SystemKey of the Empire headquarters |
| `death_star_active` | Whether Death Star win-condition checks are active |
| `death_star_location` | Current orbital location of the Death Star fleet |
| `resolved` | Set true after first outcome to suppress re-checking |

## Source Material

- **Ghidra RE**: `ghidra/notes/entity-system.md` §4.2 -- `SideVictoryConditionsNotif`, `FinalBattle` (`FUN_0054ba00`)
- **Code**: `crates/rebellion-core/src/victory.rs` -- `VictorySystem::check`, `VictoryOutcome` enum
- **Event IDs**: `0x12c` (RecruitmentDone), `0x180` (FleetBattle)

## Related

- [death-star.md](death-star.md) -- Construction and fire mechanics
- [space-combat.md](space-combat.md) -- Combat that can destroy the Death Star or HQ defenders
