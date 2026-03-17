---
title: "Diplomacy & Popularity"
description: "Faction popularity, diplomacy missions, loyalty-driven control"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "diplomacy"
sources:
  - type: "ghidra"
    file: "ghidra/notes/economy-systems.md"
  - type: "code"
    file: "crates/rebellion-core/src/missions.rs"
  - type: "prima"
    page: "Chapter 5: Diplomacy & Espionage"
related:
  - "missions"
  - "economy"
tags: ["diplomacy", "popularity", "loyalty", "uprising", "control"]
---

# Diplomacy & Popularity

## What You See

Each star system has a popularity rating for both the Alliance and the Empire, displayed as a bar in the system info panel. Diplomacy missions shift popularity toward the sending faction. When popularity drops low enough, systems can revolt — flipping control through an uprising. Characters with high diplomacy skill are more effective at swaying populations.

## How It Works

### Popularity

Each system stores two popularity values:

- `popularity_alliance` — float in `[0.0, 1.0]`
- `popularity_empire` — float in `[0.0, 1.0]`

These are not required to sum to 1.0 — both can be high (contested) or low (apathetic). The controlling faction is determined by which popularity exceeds a threshold relative to the other.

### Base Loyalty

The C++ `CStarSystem` stores `base_loyalty` at offset `+0x58`:

- **Range**: `[0, 100]` (unsigned byte semantics)
- **Write gate**: `*(uint*)(this + 0x88) & 1` must be set — system must be populated
- **Setter**: `FUN_00509c30` validates, writes, notifies both observer sides

The 2-bit faction control field at `entity+0x24 bits 6-7` is separate from the 0-100 loyalty value. Loyalty drives uprising probability; when an uprising succeeds, `ControlKindUprising` fires and may flip control bits.

### Controlling Faction

Five `ControlKind` states govern system control:

| ControlKind | Trigger | Event ID |
|-------------|---------|----------|
| `BattleWon` | Ownership transfer after combat | -- |
| `BattleWithdrew` | Attacker withdrew from combat | -- |
| `Uprising` | Loyalty-triggered control change | `0x151` |
| `BattleReady` | Forces assembled, awaiting combat | -- |
| `BattlePending` | Battle scheduled but not yet begun | -- |

`LoyaltyCausedCurrentControlKind` fires when loyalty drifts far enough to flip control without combat.

### Diplomacy Mission Effect

A successful Diplomacy mission (DIPLMSTB.DAT, skill: diplomacy) shifts the target system's popularity by a fixed delta toward the sending faction:

```
On success:
  popularity_own   += 0.1 (clamped to 1.0)
  popularity_enemy -= 0.05 (clamped to 0.0)
```

### Uprising Risk

Low loyalty creates uprising risk via two probability tables:

- **UPRIS1TB.DAT** (3 entries): maps loyalty thresholds to uprising start probability. At deeply negative loyalty delta, start probability approaches 100%.
- **UPRIS2TB.DAT** (4 entries): maps loyalty to subdue probability. Used by the Subdue Uprising mission.

An `UprisingIncident` (`0x152`) fires as a precursor warning 1-2 turns before a full uprising, giving the player a chance to respond.

## Source Material

- Ghidra: [economy-systems.md](../../ghidra/notes/economy-systems.md) — system control kinds, loyalty fields, uprising mechanics
- Rust: `crates/rebellion-core/src/missions.rs` — `MissionKind::Diplomacy`
- Rust: `crates/rebellion-core/src/uprising.rs` — uprising start/subdue probability

## Related

- [Mission System](missions.md)
- [Economy Overview](economy.md)
