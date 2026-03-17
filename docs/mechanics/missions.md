---
title: "Mission System"
description: "9 mission types, MSTB probability tables, dispatch and resolution"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "missions"
sources:
  - type: "ghidra"
    file: "ghidra/notes/mission-event-cookbook.md"
  - type: "code"
    file: "crates/rebellion-core/src/missions.rs"
  - type: "prima"
    page: "Chapter 5: Diplomacy & Espionage"
related:
  - "diplomacy"
  - "gnprtb"
tags: ["missions", "espionage", "probability", "characters"]
---

# Mission System

## What You See

Characters can be assigned to missions — diplomatic envoys, espionage runs, sabotage operations, assassination attempts, and more. Each mission takes several game-days to complete. Success depends on the character's relevant skill and the MSTB probability curve, modified by counter-intelligence from the opposing faction. A failed mission may result in the character's capture or death.

## How It Works

### The Nine Mission Types

Dispatched via a 13-case switch in `FUN_0050d5a0` (503 lines):

| Code | Mission | MSTB File | Skill | Entries |
|------|---------|-----------|-------|---------|
| 1 | Diplomacy | DIPLMSTB.DAT | diplomacy | 10 |
| 2 | Recruitment | RCRTMSTB.DAT | leadership | 11 |
| 3 | Sabotage | SBTGMSTB.DAT | espionage | 12 |
| 4 | Abduction | ABDCMSTB.DAT | espionage | 12 |
| 5 | Incite Uprising | INCTMSTB.DAT | diplomacy | 13 |
| 6 | Assassination | ASSNMSTB.DAT | combat | 12 |
| 7 | Rescue | RESCMSTB.DAT | combat | 12 |
| 8 | Subdue Uprising | SUBDMSTB.DAT | diplomacy | 13 | *Handled by `UprisingSystem::try_subdue()`, not MissionSystem* |
| 9 | Espionage | ESPIMSTB.DAT | espionage | 12 |

### Probability Tables (MSTB Format)

All `*MSTB.DAT` files use the `IntTableEntry` format (16 bytes per entry):

```
id:        u32  — entry index
field2:    u32  — reserved
threshold: i32  — signed skill delta (negative = below average)
value:     u32  — probability at this threshold (0-100)
```

**Two resolution paths** (in priority order):

1. **MSTB piecewise-linear lookup**: interpolate between threshold/value pairs from the DAT table.
2. **Quadratic fallback**: `prob = clamp(a * score^2 + b * score + c, min%, max%)` — coefficients from rebellion2's `Mission.cs`. Used when MSTB tables are not loaded.

### Combined Probability

```
total_prob = agent_prob * (1 - foil_prob)
```

Where `foil_prob` comes from `FOILTB.DAT` (14 entries) — the counter-intelligence check. If an enemy agent is running counter-intel at the target system, the foil probability reduces the mission's chance of success.

### Mission Lifecycle

1. **Dispatch**: caller creates `ActiveMission` with target system, assigned character, and duration (sampled from `tick_range`).
2. **Tick**: each game-day, `ticks_remaining` decrements.
3. **Resolve**: at zero ticks, `MissionSystem` rolls success vs. the combined probability. Emits a `MissionResult`.
4. **Effects**: the caller applies effects — shift popularity, destroy facility, capture/kill character, etc.

### Additional Probability Tables

| File | Entries | Purpose |
|------|---------|---------|
| FOILTB.DAT | 14 | Counter-intelligence foil probability |
| ESCAPETB.DAT | 9 | Character escape after capture |
| FDECOYTB.DAT | 14 | Fleet decoy success |
| TDECOYTB.DAT | 14 | Troop decoy success |
| RLEVADTB.DAT | 14 | Rebel evasion |

### Decoy System

Missions 7 (Rescue) and others with `bVar3=true` check `FDECOYTB.DAT` for fleet decoy success via `FUN_0050eea0`. Decoys redirect enemy attention, improving mission survival odds.

### Escape System

Captured characters have a per-tick chance of escape based on `ESCAPETB.DAT`. The escape check runs each game tick for all captive characters. Higher skill values improve escape probability.

### Target Character Missions

Assassination (`target_character` required), Abduction, and Rescue all target a specific character entity. The mission dispatch validates the target is present at the target system and belongs to the correct faction.

## Source Material

- Ghidra: [mission-event-cookbook.md](../../ghidra/notes/mission-event-cookbook.md) — full dispatch flow, notification lifecycle, DAT file reference
- Rust: `crates/rebellion-core/src/missions.rs` — `MissionKind`, `ActiveMission`, `MissionSystem::advance()`

## Related

- [Diplomacy & Popularity](diplomacy.md)
- [Game Balance Parameters](gnprtb.md)
