---
title: "Research & Tech Trees"
description: "Three independent tech trees, research order, character assignment, and what unlocks"
category: "guide"
created: 2026-03-22
tags: [guide, research, technology, ships, troops, facilities]
---

# Research & Tech Trees

Research is how both factions gain access to more advanced military hardware. Without it, you are stuck with your starting units for the entire game. With it, you can eventually field Mon Calamari Star Cruisers, AT-ATs, and planetary shield generators—but only if you assign the right people to drive the work.

## Three Independent Trees

Research splits into three completely independent tracks. Each track is driven by a different character skill:

| Tree | Driven By | What It Unlocks |
|------|-----------|-----------------|
| **Ship** | `ship_design` | New capital ship and fighter classes |
| **Troop** | `troop_training` | Advanced ground unit types |
| **Facility** | `facility_design` | Advanced construction—shipyards, shields, defensive emplacements |

Each tree has an integer level per faction, tracked independently. The maximum level is **15**, matching the tech tier range in the original game's binary.

## How Availability Works

Every unit class—every ship, every troop type, every facility—has a `research_order` value. This is the tech level at which that class becomes available for manufacturing. The rule is simple:

```
class is buildable if: class.research_order <= faction_tech_level[tree]
```

Classes with `research_order = 0` are available from the start of the game. Most basic units—TIE Fighters, Stormtroopers, basic shipyards—fall into this tier. As you advance a tree from level 1 toward 15, you unlock progressively more powerful hardware.

## Research Duration

Advancing from level N to level N+1 costs a number of ticks determined by the most difficult class sitting at that research step:

```
ticks = max(
    max(research_difficulty for all classes at research_order N+1),
    minimum floor of 10 ticks
)
```

If no classes exist at the next order level (the tree has a gap), the fallback is 30 ticks. This means some levels advance quickly and others take much longer depending on what is waiting to unlock.

Currently, the Ship tree draws `research_difficulty` values directly from the original binary data. Troop and Facility trees use the fallback value.

## Assigning Characters to Research

Only one project per (faction × tree) can run at a time. A character with the relevant skill is assigned to the project and spends their time there until it completes. This means:

- A character running Ship research cannot simultaneously command a fleet or run a mission.
- If a new project is dispatched on the same tech tree, the previous project is replaced and all invested ticks are lost.
- The AI automatically assigns characters to research based on skill thresholds: `ship_design ≥ 30`, `troop_training ≥ 30`, or `facility_design ≥ 30` qualifies a character for assignment to the relevant tree.

Higher skill values do not speed up research directly—the duration is fixed by the `research_difficulty` of the class being unlocked. But characters with high skills are more valuable for research because they are more reliable and there are fewer of them.

## Tier Highlights by Tree

### Ship Tree

The Ship tree unlocks the largest visible changes. Both factions begin with their most basic vessels. Research reveals increasingly powerful hulls:

**Alliance progression** leads toward:
- Medium transports and corvettes at early levels
- Mon Calamari Star Cruisers and Nebulon-B Frigates at mid levels
- The Calamari Star Cruiser and Home One equivalents at high levels

**Empire progression** leads toward:
- Victory-class Star Destroyers at early levels
- Imperial-class Star Destroyers at mid levels
- Super Star Destroyer variants at high levels

Fighters (X-Wings, TIE Fighters) are treated as available at any tech level and unlock alongside their parent capital ship tier in the original game—they are not gated by `research_order` separately.

### Troop Tree

The Troop tree unlocks specialized ground combat units beyond the basic infantry and stormtrooper regiments both factions start with:

- **Alliance**: from basic rebel troopers to SpecForces operatives and heavy assault units
- **Empire**: from stormtroopers to AT-ST walkers to AT-AT walkers at higher tiers

Ground combat is rarely decisive on its own, but controlling surface defense matters for blockade resistance and for holding systems you've captured via fleet action.

### Facility Tree

The Facility tree governs what you can build on a system's surface:

- **Low levels**: basic mines, small refineries, standard training centers
- **Mid levels**: orbital shipyards with higher processing rates, planetary shield generators, ion cannon batteries
- **High levels**: fully fortified systems with multiple defense layers

A well-researched Facility tree means your key systems are much harder to bombard and capture. A system with a planetary shield deflects orbital bombardment. Ion cannon emplacements damage incoming ships before they can engage ground forces.

## Research and the Long Game

Research is a compound investment. A faction that consistently staffs all three research tracks pulls steadily ahead in military capability. A faction that strips its researchers to run missions or command fleets will find itself still fielding first-generation ships when the enemy has switched to second- or third-generation hardware.

The strategic decision is always: whose skills are most valuable where? Admiral Ackbar's Ship Design is best spent unlocking the Mon Cal Cruiser—not commanding a patrol fleet in the Outer Rim. Thrawn's Ship Design is a wasted assignment on routine missions.

---

*Next: [Missions](missions.md)—the covert operations that shape the war without a fleet.*
