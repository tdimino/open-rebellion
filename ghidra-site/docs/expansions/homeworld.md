# Homeworld Expansion

<div style="text-align:center; margin: 1em 0;">
  <img src="/open-rebellion/assets/rebellion-starbird.png" alt="Rebel Alliance" width="50" style="margin: 0 10px;">
  <img src="/open-rebellion/assets/imperial-cog.png" alt="Galactic Empire" width="50" style="margin: 0 10px;">
</div>

The Homeworld Expansion is the first major addon for Open Rebellion—enriching every character with a connection to a home system and introducing player-created characters via the Academy system.

**Design inspirations**: The homeworld bonuses draw from **Star Wars Galaxies** (where your declared city of residence affected crafting and combat buffs), **Rome: Total War** (where generals gained traits tied to their province of origin), and **Crusader Kings** (where landed titles created emergent loyalty dynamics). The goal is the same: make geography *personal*.

## Homeworld System (Universal)

Every character gains a `homeworld` field linking them to a star system. This is not addon-only—it enriches the base game for all 60+ original characters.

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

## Create Your Own Character (Demiurgos)

The original Rebellion gave you ~60 fixed characters. You never chose who joined your cause. **Create Your Own Character** breaks this contract deliberately—following the LucasArts golden age principle: *give the player a meaningful choice that has consequences they can't fully predict*.

### The Academy System

The character graduates from an **Academy**—Imperial Academy (Carida) or Rebel Training Facility (Yavin IV). The academy determines baseline capabilities. The player customizes *within* those constraints.

### Six Archetypes

| Archetype | Primary Skills | Admiral | General | Bonus Pts |
|-----------|---------------|---------|---------|-----------|
| Diplomat | Diplomacy, Leadership | No | No | 40 |
| Operative | Espionage, Combat | No | No | 40 |
| Fleet Officer | Leadership, Combat | Yes | No | 35 |
| Ground Commander | Combat, Troop Training | No | Yes | 35 |
| Engineer | Ship Design, Facility Design | No | No | 35 |
| Fringer | Balanced (jack of all trades) | No | No | 50 |

### Creation Flow (7 Steps)

1. **Choose Faction**—Alliance starbird or Imperial cog
2. **Choose Archetype**—6 cards with skill radar charts
3. **Allocate Skills**—Point-buy within archetype floors/ceilings
4. **Pick Homeworld**—Galaxy map, faction-controlled systems only
5. **Force Roll**—8–16% chance (NOT player-controlled)
6. **Name & Portrait**—Entry field + portrait browser
7. **Review & Commission**—Final confirmation

### Hard Balance Caps

| Constraint | Value | Rationale |
|-----------|-------|-----------|
| Max custom characters per game | 3 | No army-of-clones |
| Skill ceiling (any single) | 85 | Below canon best (Luke=90, Thrawn=95) |
| Total skill points | ~200–220 | Below canon majors (~280) |
| jedi_probability max | 30 | Luke = 100 |
| Loyalty | NOT player-chosen | Random within archetype range |

### The Three LucasArts Principles

1. **Every choice has consequences the player can't fully predict**—loyalty random, homeworld can fall, Force rolls rare
2. **The flavor IS the mechanic**—academy archetypes aren't just stat templates, they're identity
3. **The best characters are the ones the galaxy gives you**—custom characters supplement the roster, never replace it

## Expanded Galaxy

- Additional star systems from the Expanded Universe (200 → 400)
- New sectors: Unknown Regions, Corporate Sector, Hapes Consortium
- Hyperspace lane network (optional overlay on the Euclidean distance model)

## Status

| Component | Status |
|-----------|--------|
| Homeworld field on Character | Planned |
| Canon homeworld JSON | Planned |
| Homeworld skill modifiers | Planned |
| Academy archetypes | Planned |
| 7-step creation UI | Planned |
| Force sensitivity roll | Planned |
| Portrait generation (Galactic Daguerreotype) | Planned |
| Expanded Galaxy systems | Future |

Full design document: [Create Your Own Character plan](https://github.com/tdimino/open-rebellion/blob/main/docs/plans/2026-03-21-addon-create-your-own-character.md)
