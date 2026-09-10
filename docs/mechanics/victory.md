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

`VictorySystem::check` currently evaluates three terminal outcomes each tick.
The 2026-09-10 source review found that this implementation is not yet faithful
to the original victory contract. The behavior below is descriptive, not an
acceptance claim.

## What You See

- A victory/defeat screen appears when the game ends.
- The outcome describes which condition was met and which faction won.

## How It Works

### Current Condition 1: System Control

A faction currently wins by controlling the enemy headquarters system:

- **Enemy fleet present** at the HQ system AND **no defending fleet** present.
- Alliance HQ captured by Empire fleet → Empire wins.
- Empire HQ captured by Alliance fleet → Alliance wins.

If both an enemy fleet and a defending fleet are present (contested), no capture occurs.

For the Alliance, taking Coruscant is the correct headquarters objective. For
the Empire, system control alone is insufficient: the mobile Alliance HQ must
be destroyed.

### Current Condition 2: Death Star Fires on Alliance HQ

If the Death Star is active and located at the Alliance HQ system, and the planet has been destroyed (`system.is_destroyed = true`), the Empire wins with `DeathStarVictory`.

This check currently takes priority and can bypass the Standard leader
requirements. That is a known divergence.

### Current Condition 3: Death Star Destroyed

If the Death Star was active but no Death Star fleet remains at its last known
location, the Alliance currently wins with `DeathStarDestroyed`. The original
manual does not make Death Star loss an independent victory condition.

### Resolution

Once a `VictoryOutcome` is returned, the caller sets `VictoryState::resolved = true`. Subsequent ticks skip victory checks. The game cannot produce multiple outcomes.

### Death Star Priority

Death Star checks run before HQ capture checks. If the Death Star destroys the Alliance HQ, the Empire wins even if an Alliance fleet is present at the Empire HQ.

### Minimum Tick

The current checker ignores all victory conditions before tick 200. No located
manual rule supports this grace period.

## Accepted Original Contract

- Alliance: capture and hold Coruscant; Standard also requires Palpatine and
  Vader in Alliance custody.
- Empire: destroy the mobile Alliance HQ by bombardment followed by assault and
  system control, or destroy its system with a Death Star; Standard also
  requires Mon Mothma and Luke in Imperial custody.
- Headquarters Only removes the leader requirements but preserves the distinct
  headquarters objectives.
- Death Star destruction is nonterminal by itself.

See the [source-backed campaign contract](../reference/campaign-history/official-campaign-contract.md#standard-victory).

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
