# Homeworld Expansion

<div style="text-align:center; margin: 1em 0;">
  <img src="/open-rebellion/assets/rebellion-starbird.png" alt="Rebel Alliance" width="50" style="margin: 0 10px;">
  <img src="/open-rebellion/assets/imperial-cog.png" alt="Galactic Empire" width="50" style="margin: 0 10px;">
</div>

The Homeworld Expansion is the first major addon for Open Rebellion. It does three things: gives every character a homeworld that matters mechanically, lets players create custom officers, and introduces a second mode where characters emerge organically from the galaxy itself.

**Design inspirations**: **Star Wars Galaxies** (city-of-residence buffs), **Rome: Total War** (generals with provincial traits), **Crusader Kings** (landed titles driving loyalty dynamics), **XCOM** (national recruitment pools), **Fire Emblem** (mid-campaign character discovery). The throughline: geography should be *personal*.

## Homeworld System (Universal)

Every character gains a `homeworld` field linking them to a star system. This enriches the base game for all 60+ original characters—not just custom ones.

### Canon Assignments

| Character | Homeworld |
|-----------|-----------|
| Luke Skywalker | Tatooine |
| Darth Vader | Tatooine |
| Leia Organa | Alderaan |
| Han Solo | Corellia |
| Mon Mothma | Chandrila |
| Thrawn | Csilla |
| Admiral Ackbar | Mon Calamari |

### Mechanical Effects

- **+10%** to all skill checks when operating at homeworld
- **-5%** to all skill checks when the enemy controls your homeworld
- **-15 loyalty** one-time hit when homeworld falls to the enemy
- **+10 loyalty** recovery when homeworld is liberated
- Special dialogue and events when defending or liberating a homeworld

When Alderaan falls, Leia's loyalty drops. When Corellia is liberated, Han fights harder. The map becomes personal.

---

## Two Modes of Character Creation

The expansion offers two ways to bring custom characters into the game. The player chooses at game start—or enables both.

### Mode 1: Academy (Design Your Officer)

The original Rebellion gave you ~60 fixed characters. You never chose who joined your cause. **Academy Mode** breaks this contract deliberately—you design up to 3 officers from scratch.

The character graduates from an **Academy**—Imperial Academy (Carida) or Rebel Training Facility (Yavin IV). The academy determines baseline capabilities. The player customizes *within* those constraints.

#### Six Archetypes

| Archetype | Primary Skills | Admiral | General | Loyalty | Bonus Pts |
|-----------|---------------|---------|---------|---------|-----------|
| Diplomat | Diplomacy, Leadership | No | No | 55–80 | 40 |
| Operative | Espionage, Combat | No | No | 45–75 | 40 |
| Fleet Officer | Leadership, Combat | Yes | No | 55–85 | 35 |
| Ground Commander | Combat, Troop Training | No | Yes | 55–85 | 35 |
| Engineer | Ship Design, Facility Design | No | No | 55–80 | 35 |
| Fringer | Balanced (jack of all trades) | No | No | 35–65 | 50 |

#### Creation Flow (7 Steps)

1. **Choose Faction**—Alliance or Empire
2. **Choose Archetype**—6 cards with skill radar charts
3. **Allocate Skills**—Point-buy within archetype floors/ceilings
4. **Pick Homeworld**—Galaxy map, faction-controlled systems only
5. **Force Roll**—8–16% chance (NOT player-controlled)
6. **Name & Portrait**—Entry field + portrait browser
7. **Review & Commission**—Final confirmation

### Mode 2: Recruitment (Discover Your Officer)

Instead of designing characters upfront, **Recruitment Mode** scatters 6–10 recruitable candidates across the galaxy. Each has a pre-rolled archetype, a homeworld, and a **condition** that must be met before they can be recruited.

You don't know their skills until you recruit them. You earn your officers through gameplay, not menus.

#### Faction Affinity

Each candidate leans toward a faction—or is neutral:

| Affinity | Who Can Recruit |
|----------|----------------|
| Alliance-leaning | Alliance only |
| Empire-leaning | Empire only |
| **Neutral** | **Either faction—first to recruit wins** |

Neutral candidates are the most interesting: both factions see them on the map, both know the condition, and both race to meet it first. A neutral Fringer at Ord Mantell becomes a strategic objective.

#### Recruitment Conditions

| Condition | Example |
|-----------|---------|
| Control the system | "Joins after your faction controls Corellia" |
| Win a battle there | "Impressed by your victory at Fondor" |
| Popularity threshold | "Joins when your popularity at Chandrila exceeds 0.7" |
| Liberate from enemy | "Freed from Imperial occupation of Mon Calamari" |
| Tick threshold | "Available after tick 500 (veteran emerges from hiding)" |
| Character visits | "Recruited only if Luke Skywalker visits the system" |
| Force sensed | "Appears after a Jedi senses them (random)" |

#### The Same Character, Either Side

The key difference from Academy: the same candidate can end up on either side. A neutral operative at Nar Shaddaa might become a Rebel spy or an Imperial assassin depending on who gets there first. Their skills are identical—only the faction flag changes.

This creates emergent stories: *"We lost the Corellian engineer to the Empire because we couldn't take Corellia in time."*

### Mode 3: Both

Create 1 character at game start via the Academy + 4–6 candidates spawn on the map. The best of both worlds.

---

## Hard Balance Caps (Both Modes)

| Constraint | Value | Rationale |
|-----------|-------|-----------|
| Skill ceiling (any single) | 85 | Below canon best (Luke=90, Thrawn=95) |
| Total skill points | ~200–220 | Below canon majors (~280) |
| jedi_probability max | 30 | Luke = 100 |
| Loyalty | NOT player-chosen | Random within archetype range |
| Max Academy characters | 3 | No army-of-clones |
| Max Recruitment candidates | 6–10 | Spread across galaxy |

## Expanded Galaxy

- Additional star systems from the Expanded Universe (200 → 400)
- New sectors: Unknown Regions, Corporate Sector, Hapes Consortium
- Hyperspace lane network (optional overlay on the Euclidean distance model)

## The Three LucasArts Principles

1. **Every choice has consequences the player can't fully predict**—loyalty random, homeworld can fall, Force rolls rare, neutral candidates can be lost to the enemy
2. **The flavor IS the mechanic**—academy archetypes are identity; recruitment conditions are narrative
3. **The best characters are the ones the galaxy gives you**—Recruitment Mode makes this literal

## Status

| Component | Status |
|-----------|--------|
| Homeworld field on Character | Planned |
| Canon homeworld JSON | Planned |
| Homeworld skill modifiers | Planned |
| Academy Mode (7-step UI) | Planned |
| Recruitment Mode (candidates + conditions) | Planned |
| Portrait generation (Galactic Daguerreotype) | Planned |
| Expanded Galaxy systems | Future |

Design documents:

- [Academy Mode (Demiurgos)](https://github.com/tdimino/open-rebellion/blob/main/docs/plans/2026-03-21-addon-create-your-own-character.md)
- [Recruitment Mode](https://github.com/tdimino/open-rebellion/blob/main/docs/plans/2026-03-22-design-homeworld-recruitment-mode.md)
