# Homeworld Expansion

<div style="text-align:center; margin: 1em 0;">
  <img src="/open-rebellion/assets/rebellion-starbird.png" alt="Rebel Alliance" width="50" style="margin: 0 10px;">
  <img src="/open-rebellion/assets/imperial-cog.png" alt="Galactic Empire" width="50" style="margin: 0 10px;">
</div>

The Homeworld Expansion is the first major addon for Open Rebellion. It does three things: gives every character a homeworld that matters mechanically, lets players create custom officers, and introduces a second mode where characters emerge organically from the galaxy itself.

**Design inspirations**: The homeworld bonuses draw from **Star Wars Galaxies** (where your declared city of residence affected crafting and combat buffs), **Rome: Total War** (where generals gained traits tied to their province of origin), and **Crusader Kings** (where landed titles created emergent loyalty dynamics). The goal is the same: make geography *personal*.

## Homeworld System (Universal)

Every character gains a `homeworld` field linking them to a star system. This enriches the base game for all 60+ original characters—not just custom ones.

### Canon Assignments

| Character | Homeworld |
|-----------|-----------|
| Luke Skywalker | Tatooine |
| Darth Vader | Tatooine |
| Leia Organa | Alderaan (destroyed) |
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

Leia's homeworld is already gone—she carries a permanent -5% penalty and can never recover it. Han fights harder when Corellia is liberated. Vader is conflicted at Tatooine. The map becomes personal.

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

### Mode 2: Recruitment (Earn Your Officer)

You still design your characters in the Academy—same 7-step flow, same archetypes, same skill allocation. But instead of joining your roster immediately, **each character is placed somewhere on the galaxy map** with a condition that must be met before they'll join.

You designed them. Now you have to earn them.

#### Recruitment Conditions

Conditions are assigned based on the character's archetype and the galaxy's strategic situation at game start. The recruitment location might be their homeworld—or it might not. A Corellian operative could be waiting at Nar Shaddaa. A Chandrilan diplomat could be stationed at Mon Calamari. The placement is at the discretion of the system.

| Condition | Example |
|-----------|---------|
| Control a system | "Your operative joins after you control Corellia" |
| Win a battle | "Your fleet officer joins after a victory at Fondor" |
| Popularity threshold | "Your diplomat joins when popularity at Chandrila exceeds 0.7" |
| Liberate from enemy | "Your engineer joins after Mon Calamari is liberated" |
| Character visits | "Your Fringer joins when Han Solo visits Nar Shaddaa" |
| Tick threshold | "Your veteran joins after tick 500" |
| Homeworld controlled | "Your pilot joins after you control their homeworld" |

Homeworld is one possible condition among many—not the default.

#### Faction Affinity

Each character has a faction leaning based on their homeworld and archetype:

| Affinity | Who Can Recruit |
|----------|----------------|
| Alliance-leaning | Alliance only (unless enemy meets a special condition) |
| Empire-leaning | Empire only (unless enemy meets a special condition) |
| **Neutral** | **Either faction—first to recruit wins** |

Most characters lean toward the faction that controls their homeworld at game start. But some—Fringers especially—are genuinely neutral. A neutral operative at Nar Shaddaa can be recruited by either side. If the enemy gets there first, they join the other side with the skills you designed.

This creates real strategic stakes around homeworld choice in Step 4 of the Academy flow. Picking a homeworld deep in friendly territory is safe. Picking a contested border world risks losing your character to the enemy—but the homeworld bonus in a contested zone makes them more valuable if you hold it.

*"We lost the Corellian engineer to the Empire because we couldn't take Corellia in time."*

### Mode 3: Both

Design up to 3 characters through the Academy. The first joins your roster immediately at game start—your "founding officer." The remaining characters are placed on the galaxy map with recruitment conditions, just like Recruitment Mode.

This gives you one officer to rely on from day one while the others become strategic objectives you pursue over the course of the campaign. It's the mode that most closely mirrors how the original game's recruitment missions worked—except now you know *who* you're looking for.

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
