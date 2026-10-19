---
title: "Victory Conditions"
description: "Current victory implementation, original-game contract, and open parity divergences"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-17
game_system: "victory"
sources:
  - type: "ghidra"
    file: "ghidra/notes/victory-conditions.md"
  - type: "code"
    file: "crates/rebellion-core/src/victory.rs"
related:
  - "death-star"
  - "space-combat"
tags: ["victory", "headquarters", "death-star", "endgame"]
---

# Victory Conditions

The game ends when the win word `FUN_005180e0` builds sets a side's win bit
(`ghidra/notes/victory-conditions.md`; the port's rules are in
`agent_docs/systems/victory.md`).

## What You See

- No victory screen. The advisor droid speaks the side's ending line, the
  game is torn down, and one of four ending movies plays by winner and
  player side (MDATA 105-108); its end returns to the main menu.
- When the Alliance headquarters goes, or the Alliance takes Coruscant, a
  movie with the message as its caption plays (MDATA 102, 103), each time the
  word changes while it holds.

## How It Works

### Alliance objective (AW)

Coruscant held by the Alliance (CA: its holder bits, which a stalemate
leaves with the earlier holder) or destroyed (CD). Standard also needs Darth
Vader and Emperor Palpatine prisoners (DC and EC), whoever holds them.

### Imperial objective (EW)

The Alliance headquarters gone (AD): destroyed with its system, hit as a
military bombardment target, or removed when its system leaves Alliance
control. No occupation is needed. Standard also needs Luke Skywalker and Mon
Mothma prisoners (LC and MC). Losing a Death Star does not end the campaign.

### Headquarters Only

EW is AD alone; AW is CA or CD alone.

### Both at once

A side whose win bit the previous word held keeps it; otherwise the
Alliance wins.

### Timing and resolution

There is no minimum-day grace period: a valid objective may resolve on the
first simulation tick. A missing leader or a missing Coruscant leaves the
word unwritten. Once the word holds GO the game evaluates it no more
(`VictoryState::resolved`).

See the [source-backed campaign contract](../reference/campaign-history/official-campaign-contract.md#standard-victory).

### VictoryState

| Field | Purpose |
|-------|---------|
| `alliance_hq` | SystemKey of the Alliance headquarters |
| `empire_hq` | SystemKey of Coruscant |
| `word` | The win word last delivered (notification `0x128`) |
| `resolved` | Set once the word holds GO |

## Source Material

- **Ghidra RE**: `ghidra/notes/victory-conditions.md`, `game-end-path.md`, `movies.md`
- **Code**: `crates/rebellion-core/src/victory.rs` (the word), `crates/rebellion-app/src/movies.rs` (the movies and the ending)
- **Event IDs**: `0x128` (the win word's notification)

## Related

- [death-star.md](death-star.md) -- Construction and fire mechanics
- [space-combat.md](space-combat.md) -- Combat that can destroy the Death Star or HQ defenders
