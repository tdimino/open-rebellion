---
title: "Homeworld Expansion: Recruitment Mode"
type: design
status: planned
date: 2026-03-22
project: open-rebellion
tags: [addon, homeworld, character-creation, recruitment, events]
---

# Homeworld Expansion: Recruitment Mode

## Context

The base Homeworld Expansion (Demiurgos) lets the player create a custom character at game start via the Academy system. **Recruitment Mode** is an alternative mode where custom characters are not player-designed upfront—instead, they emerge organically from the galaxy during gameplay, recruited from specific star systems when conditions are met.

The player enables one mode or the other at game start. Both use the same underlying character data structures (homeworld, archetype, skills, Force roll). The difference is *when* and *how* the character enters the game.

## Design Philosophy

The Academy system is a **power fantasy**—you choose who joins. Recruitment Mode is a **discovery fantasy**—the galaxy offers you people, and you decide whether to pursue them.

**Inspirations**:
- **Star Wars Galaxies**: NPCs in cantinas offered quests that led to companion recruitment
- **XCOM**: Soldiers recruited from specific nations, with national traits
- **Mount & Blade**: Companions found in taverns across the map, each with backstory and opinions
- **Fire Emblem**: Characters recruited mid-campaign by visiting specific locations or meeting conditions

## How It Works

### 1. Recruitment Candidates Spawn on the Galaxy Map

At game start (with Recruitment Mode enabled), 6–10 **Recruitment Candidate** markers are placed at systems across the galaxy. Each candidate has:

- A **system** (their homeworld—where they can be recruited)
- A **faction affinity** (Alliance-leaning, Empire-leaning, or Neutral)
- A **pre-rolled archetype** (Diplomat, Operative, Fleet Officer, etc.)
- A **pre-rolled skill set** (within archetype bounds, same as Academy rules)
- A **recruitment condition** (what triggers their availability)

The player does NOT see the candidate's skills or archetype until they send a character to recruit them. They see only the system, a name, and a one-line description ("A former Imperial pilot seeking purpose").

### 2. Recruitment Conditions

Each candidate has a condition that must be met before they can be recruited. This creates emergent narrative—you earn your officers through gameplay, not menus.

| Condition | Example | Frequency |
|-----------|---------|-----------|
| **Control the system** | "Recruit after your faction controls Corellia" | Common |
| **Win a battle at the system** | "Impressed by your fleet's victory at Fondor" | Common |
| **Diplomacy popularity threshold** | "Joins when your popularity at Chandrila exceeds 0.7" | Moderate |
| **Liberate from enemy control** | "Freed from Imperial occupation of Mon Calamari" | Moderate |
| **Tick threshold** | "Available after tick 500 (veteran emerges from hiding)" | Rare |
| **Character interaction** | "Recruited only if Luke Skywalker visits the system" | Rare |
| **Force-sensitive discovery** | "Appears after a Jedi character senses them (random)" | Very Rare |

### 3. Faction Affinity Determines Who Can Recruit

A candidate's affinity determines which faction can recruit them:

| Affinity | Who Can Recruit | How It Works |
|----------|----------------|--------------|
| **Alliance-leaning** | Alliance only | Standard recruitment mission |
| **Empire-leaning** | Empire only | Standard recruitment mission |
| **Neutral** | Either faction—first to recruit wins | Race condition—creates strategic tension |

**Neutral candidates** are the most interesting: both factions can see them on the map, both know the recruitment condition, and both race to meet it first. A neutral Fringer at Ord Mantell becomes a strategic objective.

### 4. Recruitment Process

Once the condition is met:

1. The candidate's marker becomes **active** (visible indicator on galaxy map)
2. The player dispatches a major character on a **Recruitment mission** to the candidate's system
3. On success: the candidate joins the faction with their pre-rolled stats
4. On failure: the candidate remains available (can retry)
5. If the enemy recruits a neutral candidate first: they join the enemy permanently

### 5. The Custom Character Joins as Either Faction

This is the key difference from Academy mode: the same character can end up on either side. A neutral operative at Nar Shaddaa might become a Rebel spy or an Imperial assassin depending on who gets there first. Their skills are identical either way—only the faction flag changes.

This creates **emergent stories**: "We lost the Corellian engineer to the Empire because we couldn't take Corellia in time."

### 6. Force Sensitivity

Force-sensitive candidates are especially rare and valuable:

- Only 1–2 of the 6–10 candidates have any Force probability
- Their Force roll happens at recruitment time (not at game start)
- The roll uses the same 8–16% table as Academy mode
- A Force-sensitive recruit is a major strategic asset—worth diverting resources to secure

## Data Structures

```rust
pub struct RecruitmentCandidate {
    pub name: String,
    pub description: String,  // one-line flavor text
    pub system: SystemKey,    // homeworld / recruitment location
    pub affinity: FactionAffinity,
    pub archetype: AcademyArchetype,
    pub skills: CharacterSkills,  // pre-rolled within archetype bounds
    pub force_probability: u32,   // 0-16, rolled at recruitment
    pub condition: RecruitmentCondition,
    pub recruited_by: Option<Faction>,
    pub active: bool,  // condition met?
}

pub enum FactionAffinity {
    Alliance,
    Empire,
    Neutral,  // either faction can recruit
}

pub enum RecruitmentCondition {
    ControlSystem(SystemKey),
    BattleWonAt(SystemKey),
    PopularityThreshold { system: SystemKey, faction: Faction, threshold: f32 },
    LiberateSystem(SystemKey),
    TickReached(u64),
    CharacterVisits { character: CharacterKey, system: SystemKey },
    ForceSensed,  // random event when Jedi visits nearby system
}
```

## Game Start Configuration

```
┌─────────────────────────────────────────┐
│  HOMEWORLD EXPANSION                    │
│                                         │
│  Choose your mode:                      │
│                                         │
│  ○ Academy Mode                         │
│    Create up to 3 custom characters     │
│    at game start via the Academy.       │
│                                         │
│  ○ Recruitment Mode                     │
│    6–10 recruitable characters appear   │
│    across the galaxy. Meet conditions   │
│    to unlock and recruit them.          │
│                                         │
│  ○ Both (Academy + Recruitment)         │
│    Create 1 character at start +        │
│    4–6 candidates spawn on the map.     │
│                                         │
│  [Continue]                             │
└─────────────────────────────────────────┘
```

## Balance

| Constraint | Academy Mode | Recruitment Mode |
|-----------|-------------|-----------------|
| Max custom characters | 3 | 6–10 (but harder to get) |
| Skill ceiling | 85 | 85 |
| Total skill points | ~200–220 | ~200–220 |
| Available from | Game start | Conditional (mid-game) |
| Faction choice | Player chooses | Affinity-determined or race |
| Force probability | 8–16% | 8–16% (rolled at recruitment) |
| Player agency | High (design the character) | Low (discover what the galaxy offers) |
| Strategic depth | Low (menu interaction) | High (map control, timing, races) |

## Integration with Existing Systems

- **EventSystem**: Recruitment conditions map to existing `EventCondition` variants (TickReached, FactionControlsSystem, CharacterAtSystem)
- **MissionSystem**: Uses existing `MissionKind::Recruitment` with the candidate's system as target
- **FogSystem**: Candidate markers visible only if the system is explored
- **AISystem**: AI evaluates neutral candidates as strategic targets—dispatches recruitment missions when conditions are close to being met

## Implementation Phases

1. **RecruitmentCandidate struct + condition evaluation** (~80 LOC)
2. **Candidate spawning at game start** (random placement, affinity distribution) (~60 LOC)
3. **Galaxy map markers** (egui overlay for candidate locations) (~40 LOC)
4. **Condition checking in simulation tick** (~50 LOC)
5. **Recruitment mission wiring** (dispatch to candidate system) (~30 LOC)
6. **AI recruitment logic** (evaluate and pursue candidates) (~50 LOC)
7. **Mode selection UI** (game start modal) (~40 LOC)

**Total: ~350 LOC + tests**

## The Three LucasArts Principles (Applied)

1. **Every choice has consequences the player can't fully predict**—you don't know a candidate's skills until you recruit them. Neutral candidates can be lost to the enemy.
2. **The flavor IS the mechanic**—recruitment conditions create narrative ("the Corellian engineer joined after we liberated her homeworld").
3. **The best characters are the ones the galaxy gives you**—Recruitment Mode makes this literal.
