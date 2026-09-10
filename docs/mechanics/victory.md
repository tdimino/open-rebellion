---
title: "Victory Conditions"
description: "Current victory implementation, original-game contract, and open parity divergences"
category: "mechanics"
created: 2026-03-16
updated: 2026-09-10
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

`VictorySystem::check` evaluates the original faction-specific headquarters
objectives and applies the selected game type's leader requirements.

## What You See

- A victory/defeat screen appears when the game ends.
- The outcome describes which condition was met and which faction won.

## How It Works

### Alliance objective

Alliance control of Coruscant produces `HqCaptured`. Standard mode also
requires Emperor Palpatine and Darth Vader to be alive in Alliance custody.

### Imperial objective

A successful Imperial bombardment of the current mobile Alliance headquarters
clears that system's persisted `is_headquarters` flag. The Empire must then
take political control of the surviving system to produce `HqDestroyed`.
Occupation without prior destruction and bombardment without occupation are
both nonterminal.

Destroying the entire Alliance-HQ planet with the Death Star produces
`DeathStarVictory` and does not require a subsequent ground occupation.
Standard mode still requires Mon Mothma and Luke Skywalker to be alive in
Imperial custody. Losing a Death Star does not end the campaign.

### Headquarters Only

Headquarters Only uses the same asymmetric headquarters objectives and omits
only the principal-leader requirements.

### Timing and resolution

There is no minimum-day grace period. A valid objective may resolve on the
first simulation tick. Once a `VictoryOutcome` is returned, the caller sets
`VictoryState::resolved = true`; subsequent checks remain silent.

See the [source-backed campaign contract](../reference/campaign-history/official-campaign-contract.md#standard-victory).

### VictoryState Configuration

Set at game start:

| Field | Purpose |
|-------|---------|
| `alliance_hq` | SystemKey of the Alliance headquarters |
| `empire_hq` | SystemKey of the Empire headquarters |
| `death_star_active` | Persisted Death Star campaign status |
| `death_star_location` | Persisted last known Death Star location |
| `resolved` | Set true after first outcome to suppress re-checking |

## Source Material

- **Ghidra RE**: `ghidra/notes/entity-system.md` §4.2 -- `SideVictoryConditionsNotif`, `FinalBattle` (`FUN_0054ba00`)
- **Code**: `crates/rebellion-core/src/victory.rs` -- objective detection and bombardment application
- **Event IDs**: `0x12c` (RecruitmentDone), `0x180` (FleetBattle)

## Related

- [death-star.md](death-star.md) -- Construction and fire mechanics
- [space-combat.md](space-combat.md) -- Combat that can destroy the Death Star or HQ defenders
