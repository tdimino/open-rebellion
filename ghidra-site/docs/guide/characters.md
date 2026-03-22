---
title: "Characters & the Officer Corps"
description: "Major characters, skill system, Force sensitivity, and Jedi training"
category: "guide"
created: 2026-03-22
tags: [guide, characters, skills, jedi, loyalty, betrayal]
---

# Characters & the Officer Corps

Characters are not just flavor. Every significant action in the game—running a mission, commanding a fleet, leading a ground assault, advancing research—is executed by a named officer. Who you assign, and where, determines what gets done and how well.

## The Roster

Characters come in two tiers: **major** characters (six per faction, with fixed identities and locked stats) and **minor** characters (generic officers generated with randomized skills at scenario start).

### Alliance Major Characters

| Character | Role | Notable Skills |
|-----------|------|----------------|
| **Luke Skywalker** | Jedi candidate, fleet officer | Combat, Leadership, Force potential |
| **Han Solo** | Admiral, smuggler | Leadership; hyperdrive bonus for fleet speed |
| **Princess Leia** | Diplomat, leader | Diplomacy, Leadership |
| **Mon Mothma** | Political leader | Diplomacy |
| **Admiral Ackbar** | Fleet commander | Ship Design, Leadership |
| **Chewbacca** | General, infiltrator | Combat |

### Imperial Major Characters

| Character | Role | Notable Skills |
|-----------|------|----------------|
| **Darth Vader** | Admiral, commander | Combat, Leadership; experienced Force user |
| **Grand Admiral Thrawn** | Fleet commander | Ship Design, Leadership, Combat |
| **Emperor Palpatine** | Political supremacy | Diplomacy, Espionage |
| **Boba Fett** | Bounty hunter, operative | Espionage, Combat |
| **Mara Jade** | Agent | Espionage, Combat |
| **General Veers** | Ground commander | Combat, Troop Training |

### Minor Characters

Each faction starts with a pool of approximately 27 generic officers. Their skill values are randomized within a base + variance range at game start—two games will never have identical minor characters. They are expendable in the sense that their deaths are setbacks, not narrative disasters, but losing a character with high ship_design mid-research is a real strategic cost.

## The Eight Skills

Every character has **8 skill pairs**, each stored as a base value plus a variance component. At scenario start, the actual skill is drawn from `base + random(0, variance)`. Major characters typically have zero variance—their stats are fixed.

| Skill | What It Does |
|-------|--------------|
| **Diplomacy** | Mission success for Diplomacy and Incite Uprising missions; population persuasion |
| **Espionage** | Mission success for Sabotage, Abduction, and Espionage missions |
| **Ship Design** | Drives the Ship tech tree—how fast new capital ship classes unlock |
| **Troop Training** | Drives the Troop tech tree—how fast advanced ground units become available |
| **Facility Design** | Drives the Facility tech tree—shipyards, shields, defensive structures |
| **Combat** | Effectiveness in space and ground combat; required for Assassination and Rescue missions |
| **Leadership** | Fleet morale in battle; ground troop effectiveness; also used for Recruitment missions |
| **Loyalty** | Resistance to betrayal; affects uprising thresholds for the systems a character occupies |

Skills range from 0 to 100. A skill of 30 or above in Ship Design, Troop Training, or Facility Design qualifies a character for assignment to a research project. Mission success probability scales with the relevant skill via the MSTB probability tables—there is no hard threshold for most mission types.

## Character Roles

A character can be assigned to one of three roles at a time:

- **Admiral**—commands a fleet. Leadership affects fleet combat morale. Han Solo's `hyperdrive_modifier` bonus applies here, making his fleet faster than the slowest ship's rating.
- **Commander**—dispatched on a mission. The mission's relevant skill determines success probability.
- **General**—leads ground troops in planetary assault or defense. Combat and Leadership both matter.

A character on a mission cannot simultaneously command a fleet. Choosing who to assign where is a constant strategic tension—the character most useful to you is usually needed in three places at once.

## Loyalty and Betrayal

Loyalty is not just a number—it is a risk gauge. Characters with low loyalty are vulnerable to betrayal, which means they may defect to the opposing faction if their loyalty drops below a threshold defined by the `UPRIS1TB` table.

Key facts about betrayal:

- **Luke Skywalker** and **Darth Vader** carry an `is_unable_to_betray` flag—they will never switch sides regardless of loyalty.
- Minor characters can be turned by enemy diplomacy missions. A highly skilled character defecting to the enemy is a double loss: you lose them and the enemy gains them.
- There is a 50-tick cooldown between betrayal checks for any given character.
- The enemy can also run **Abduction missions** to forcibly capture one of your officers.

## Force Sensitivity and Jedi Training

Each character has a `jedi_probability` value from 0 to 100. This is the chance that character has Force sensitivity—a roll made at game start. A character who passes the roll becomes a **Force-aware** candidate for Jedi training.

Jedi training progresses through four tiers:

| Level | Status |
|-------|--------|
| 0 | No Force sensitivity |
| 1 | Force-Aware—potential identified |
| 2 | In Training—actively studying |
| 3 | Jedi—fully realized Force user |

Training requires a trainer. For the Alliance, this is Yoda (if Luke reaches Dagobah—a scripted story event) or another experienced Jedi. Training takes time and keeps the Jedi candidate off other assignments.

Once trained, a Jedi character is significantly more effective in combat and some mission types. But Jedi can also be detected by the opposing faction's Force-sensitive characters. Vader can sense a trained Jedi across a sector.

**Luke Skywalker** has `jedi_probability = 100`—he is always Force-sensitive. His progression through the Dagobah training chain (a scripted story event sequence beginning at event `0x221`) mirrors the original trilogy.

## Mission State Flags

Characters track three mission flags that prevent double-assignment:

| Flag | Meaning |
|------|---------|
| `on_mission` | Character is currently executing a mission—cannot be reassigned |
| `on_hidden_mission` | The mission is concealed from enemy intelligence |
| `on_mandatory_mission` | Story-driven—cannot resign (e.g., Luke on Dagobah) |

The dispatch system enforces these: a character already on a mandatory mission cannot be pulled for fleet command or a different mission.

## Capture and Escape

Characters can be captured during failed missions or enemy combat. A captured character:

- Is moved to the enemy faction
- Appears in their roster as a prisoner
- Can be rescued via a Rescue mission
- Has a per-tick escape probability based on `ESCAPETB.DAT`

If Mon Mothma is captured, the Alliance loses its most powerful diplomat. If Darth Vader is somehow captured... well, the game doesn't expect that to happen.

---

*Next: [Tech Trees](tech-trees.md)—how research unlocks advanced ships, troops, and facilities.*
