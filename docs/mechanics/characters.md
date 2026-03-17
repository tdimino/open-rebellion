---
title: "Characters & Skills"
description: "Character system: 8 skill pairs, major/minor roles, loyalty, betrayal, Force potential"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "characters"
sources:
  - type: "ghidra"
    file: "ghidra/notes/entity-system.md"
  - type: "code"
    file: "crates/rebellion-core/src/world/mod.rs"
related:
  - "jedi"
  - "missions"
  - "uprising"
tags: ["characters", "skills", "loyalty", "betrayal"]
---

# Characters & Skills

Characters are named leaders who command fleets, run missions, and advance the story. Each faction recruits both **major** characters (Luke, Vader -- fixed identities) and **minor** characters (generic officers generated at scenario start).

## What You See

- Characters appear in the Officers panel with 8 skill bars.
- A character can be assigned as **Admiral** (fleet command), **Commander** (mission dispatch), or **General** (ground combat leadership).
- Loyalty determines whether a character can be turned by the enemy. Some major characters (Luke, Vader) are immune to betrayal.
- Characters with Force potential may awaken as Jedi -- see [jedi.md](jedi.md).

## How It Works

### Skill System

Each character has 8 skills stored as `SkillPair { base, variance }`. At scenario start, the effective skill value is `base + rng(0..=variance)`. Major characters typically have `variance = 0` (locked stats).

| Skill | Role | Used By |
|-------|------|---------|
| Diplomacy | Mission success | Diplomacy missions |
| Espionage | Mission success | Espionage, sabotage, assassination |
| Ship Design | Research speed | Ship R&D |
| Troop Training | Research speed | Troop R&D |
| Facility Design | Research speed | Facility R&D |
| Combat | Battle effectiveness | Space and ground combat |
| Leadership | Fleet/troop morale | Fleet command, ground battles |
| Loyalty | Faction allegiance | Betrayal checks, uprising thresholds |

### Major vs Minor Characters

- **Major**: Fixed name, fixed skills, unique story events. Loaded from `MJCHARSD.DAT`. Examples: Luke Skywalker, Darth Vader, Han Solo.
- **Minor**: Generic names, randomized skills via variance. Loaded from `MNCHARSD.DAT`. Reusable across scenarios.

### Roles

Characters can serve as:
- **Admiral** (`can_be_admiral`): Commands a fleet; Leadership skill affects fleet morale.
- **Commander** (`can_be_commander`): Dispatched on missions; relevant skill determines success probability.
- **General** (`can_be_general`): Leads ground troops; Combat + Leadership affect ground battles.

### Loyalty & Betrayal

Two loyalty fields exist in the binary (entity-system.md §1.1):
- `base_loyalty` (offset +0x66): Drives diplomacy missions and betrayal checks. Range 0--100.
- `enhanced_loyalty` (offset +0x8a): Mission bonus loyalty. Range 0--100, clamped.

The `is_unable_to_betray` flag (from DAT data) gates major characters like Luke and Vader from ever switching sides. For others, when base loyalty drops below thresholds defined in `UPRIS1TB.DAT`, a betrayal check fires (see [uprising.md](uprising.md)).

### Mission State Flags

| Flag | Meaning |
|------|---------|
| `on_mission` | Character is currently assigned to a mission |
| `on_hidden_mission` | Mission is concealed from the opposing faction's intelligence |
| `on_mandatory_mission` | Story-driven forced assignment (e.g., Luke on Dagobah); blocks resignation |

### Force Potential

Each character has a `jedi_probability` (0--100) determining whether they may awaken as Force-sensitive. Characters with `is_known_jedi = true` start as publicly known Force users. `is_jedi_trainer = true` (Yoda) allows training other Jedi. See [jedi.md](jedi.md) for the full training system.

### Hyperdrive Modifier

The `hyperdrive_modifier` field (offset +0x9a) provides a fleet speed bonus when the character is assigned as admiral. Han Solo is the canonical example -- see [fleet-movement.md](fleet-movement.md).

## Source Material

- **Ghidra RE**: `ghidra/notes/entity-system.md` §1 (Character System), §1.3 (Force/Jedi), §1.4 (Mission Role Assignment)
- **Code**: `crates/rebellion-core/src/world/mod.rs` -- `Character` struct with all fields
- **DAT files**: `MJCHARSD.DAT` (major characters), `MNCHARSD.DAT` (minor characters)
- **Community**: Prima Strategy Guide character tables; Mechanics Inside Rebellion skill analysis

## Related

- [jedi.md](jedi.md) -- Force training progression
- [missions.md](missions.md) -- Mission dispatch and skill-based probability
- [uprising.md](uprising.md) -- Loyalty thresholds and betrayal mechanics
