---
title: "Addon: Create Your Own Character (Demiurgos)"
type: feat
status: planned
date: 2026-03-21
project: open-rebellion
designer: Nomos (soul architect subdaimon)
---

# Create Your Own Character — First Addon

## Design Philosophy

The original Rebellion gave you ~60 fixed characters. You never chose who joined your cause. **Create Your Own Character** breaks this contract deliberately. The LucasArts golden age principle: *give the player a meaningful choice that has consequences they can't fully predict*.

### Core Tensions
1. **Power fantasy vs. balance**: Custom character must matter but not outclass Luke/Vader
2. **Star Wars flavor vs. creative freedom**: Should feel like they belong in the galaxy
3. **Simplicity vs. depth**: Point-buy engaging for min-maxers, tedious for casuals

### Resolution: The Academy System
The character graduates from an **Academy** — Imperial Academy (Carida) or Rebel Training Facility (Yavin IV). The academy determines baseline capabilities. Player customizes *within* those constraints. The framing device simultaneously explains balance, delivers flavor, and structures the UI.

## Homeworld System (Universal — All Characters)

Every character gets a `homeworld: Option<SystemKey>`. This is a structural addition, not addon-only.

### Effects
- **+10%** to all skill checks when operating at homeworld
- **-5%** to all skill checks when enemy controls homeworld
- **-15 loyalty** one-time hit when homeworld falls to enemy
- **+10 loyalty** recovery when homeworld is liberated
- Special dialogue/events when defending or liberating homeworld

### Canon Assignments
`data/homeworlds.json` maps existing characters to EU homeworlds:
- Luke Skywalker → Tatooine
- Darth Vader → Tatooine
- Leia Organa → Alderaan
- Han Solo → Corellia
- Mon Mothma → Chandrila
- Thrawn → Csilla
- Admiral Ackbar → Mon Calamari

## Academy Archetypes (6)

| Archetype | Primary Skills | Admiral | General | Loyalty | Bonus Pts |
|-----------|---------------|---------|---------|---------|-----------|
| Diplomat | Diplomacy, Leadership | No | No | 55-80 | 40 |
| Operative | Espionage, Combat | No | No | 45-75 | 40 |
| Fleet Officer | Leadership, Combat | Yes | No | 55-85 | 35 |
| Ground Commander | Combat, Troop Training | No | Yes | 55-85 | 35 |
| Engineer | Ship Design, Facility Design | No | No | 55-80 | 35 |
| Fringer | Balanced (jack of all trades) | No | No | 35-65 | 50 |

**Skill ceiling: 85** (Luke's combat is 90, Thrawn's leadership 95 — custom < canon best).
**Total skill points: ~200-220** (canon majors average ~280).
**Loyalty: NOT player-chosen** — random within archetype range.

## Creation Flow (7 Steps)

1. **Choose Faction** — Alliance starbird or Imperial cog
2. **Choose Archetype** — 6 cards with skill radar charts
3. **Allocate Skills** — Point-buy within archetype floors/ceilings (5-point increments)
4. **Pick Homeworld** — Galaxy map, faction-controlled systems only
5. **Force Roll** — 8-16% chance based on species/archetype (NOT player-controlled)
6. **Name & Portrait** — Entry field + portrait browser (128 BMPs)
7. **Review & Commission** — Final confirmation, character inserted at HQ

## Force Sensitivity Balance

| Factor | Probability |
|--------|------------|
| Base rate | 8% |
| Species: Zabrak | +5% |
| Species: Twilek | +2% |
| Archetype: Fringer | +3% |
| **Maximum possible** | **16%** |

On success: `jedi_probability` = 15-30 (Luke = 100). Must still train in-game. Custom Force user needs 100+ training ticks to become relevant. Never rivals Luke.

## Hard Balance Caps

| Constraint | Value | Rationale |
|-----------|-------|-----------|
| Max custom characters per game | 3 | No army-of-clones |
| Skill ceiling (any single) | 85 | Below canon best |
| Total skill points | ~200-220 | Below canon majors (~280) |
| jedi_probability max | 30 | Luke = 100 |
| Loyalty | NOT player-chosen | Random, betrayal possible |
| hyperdrive_modifier | 0 (Fringer: +1) | Han Solo's bonus unique |
| is_unable_to_betray | Always false | Only Luke/Vader |
| is_jedi_trainer | Always false | Only Yoda |

## Data Structures

```rust
// Character additions:
pub homeworld: Option<SystemKey>,
pub origin: CharacterOrigin, // Dat | Custom

// New enums:
pub enum CharacterOrigin { Dat, Custom }
pub enum AcademyArchetype { Diplomat, Operative, FleetOfficer, GroundCommander, Engineer, Fringer }
pub enum Species { Human, MonCalamari, Bothan, Sullustan, Twilek, Wookiee, Rodian, Zabrak, Duros }
```

## Implementation Phases

1. **Homeworld System** — add field to Character, load from JSON, integrate into missions + betrayal, bump save v5
2. **Character Creation Core** — archetypes, 7-step UI, commission logic
3. **Force Sensitivity** — probability table, wire to Jedi system
4. **Portraits & Flavor** — 128 BMPs, backstory templates, encyclopedia
5. **Mod Integration** — extend mod loader for `"action": "add"` entity creation

## Files to Modify

- `crates/rebellion-core/src/world/mod.rs` — homeworld, origin, enums
- `crates/rebellion-core/src/homeworld.rs` — NEW: HomeworldModifier
- `crates/rebellion-core/src/missions.rs` — homeworld probability modifier
- `crates/rebellion-core/src/betrayal.rs` — homeworld loyalty penalty
- `crates/rebellion-core/src/events.rs` — HomeworldCaptured/Liberated conditions
- `crates/rebellion-render/src/panels/character_creation.rs` — NEW: 7-step UI
- `crates/rebellion-data/src/mods.rs` — extend for entity addition
- `data/homeworlds.json` — NEW: canon homeworld mappings

## The Three LucasArts Principles

1. **Every choice has consequences the player can't fully predict** — loyalty random, homeworld can fall, Force rolls rare
2. **The flavor IS the mechanic** — academy archetypes aren't just stat templates, they're identity
3. **The best characters are the ones the galaxy gives you** — custom characters supplement the roster, never replace it
