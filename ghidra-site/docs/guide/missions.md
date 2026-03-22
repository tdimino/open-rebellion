---
title: "Missions"
description: "All 9+ mission types, probability tables, character skill matching, and outcomes"
category: "guide"
created: 2026-03-22
tags: [guide, missions, espionage, diplomacy, sabotage, assassination]
---

# Missions

Missions are the covert dimension of the war. While fleets move openly across the galaxy map, characters operating in the shadows can shift popular sentiment, destroy enemy facilities, capture key officers, or incite entire planets to revolt—all without a single shot from a capital ship.

## How Missions Work

A mission is dispatched by assigning a character as **Commander** and sending them to a target system. The mission runs for a fixed number of game-days, after which it resolves. Success is determined by a probability roll.

### The Probability Roll

Mission success probability is not a flat number—it depends on the character's relevant skill versus the difficulty encoded in the MSTB (Mission Success Table Binary) data file for that mission type. The relationship is piecewise-linear: very low skill produces very low probability; high skill pushes the probability toward near-certain success.

On top of this, the opposing faction can deploy **counter-intelligence** at the target system. If a foil agent is present, the combined probability is:

```
final probability = agent_success_chance × (1 − foil_probability)
```

This means even a highly skilled character can fail if the target system is well-covered by enemy agents.

### What Happens on Failure

A failed mission can have consequences beyond simply not achieving the objective:

- The character may be **captured** and taken prisoner by the opposing faction
- In the worst cases, the character may be **killed**
- The mission is exposed, alerting the enemy to your intent

Skill level affects not just success probability but also how likely a failed mission is to turn catastrophic.

## The Eleven Mission Types

| # | Mission | Skill | Primary Effect |
|---|---------|-------|----------------|
| 1 | **Diplomacy** | Diplomacy | Shift system popularity toward your faction |
| 2 | **Recruitment** | Leadership | Attempt to recruit a neutral character or officer |
| 3 | **Sabotage** | Espionage | Destroy a facility at the target system |
| 4 | **Abduction** | Espionage | Capture a specific enemy character |
| 5 | **Incite Uprising** | Diplomacy | Trigger a loyalty crisis on an enemy-controlled system |
| 6 | **Assassination** | Combat | Kill a specific enemy character |
| 7 | **Rescue** | Combat | Free a captured friendly character |
| 8 | **Subdue Uprising** | Diplomacy | Calm a revolting system before control flips |
| 9 | **Espionage** | Espionage | Gather intelligence on a target system (reveal assets) |

There is also a **Death Star Sabotage** mission type for the Alliance—a high-difficulty specialized operation against the Death Star itself if its location is known.

---

### Diplomacy

**Skill**: Diplomacy | **MSTB**: DIPLMSTB.DAT (10 entries)

A successful diplomacy mission shifts the target system's popularity:
- Your faction's popularity increases by 0.01 (capped at 1.0)

Over several successful missions, a neutral or lightly contested system will flip to your control without combat. This is the Alliance's primary expansion tool in the early game.

Characters best suited: Mon Mothma, Leia, and minor officers with high diplomacy. Send your best diplomats to systems with large neutral populations—they are the path to a galaxy-wide coalition.

---

### Recruitment

**Skill**: Leadership | **MSTB**: RCRTMSTB.DAT (11 entries)

Recruitment attempts to bring a neutral character into your faction's service. Success depends on Leadership. Unlike Diplomacy, Recruitment targets specific individuals rather than system populations—useful for pulling skilled officers away from the neutral pool before the enemy can do the same.

---

### Sabotage

**Skill**: Espionage | **MSTB**: SBTGMSTB.DAT (12 entries)

A successful sabotage destroys one facility at the target system. This can mean a shipyard, a training center, a mine, or a planetary shield. The target facility is selected from what is present at the system.

Sabotage is the most direct way to degrade enemy production without committing a fleet. Destroying an Imperial shipyard at a key manufacturing system forces the Empire to queue a replacement, buying weeks of reduced production.

---

### Abduction

**Skill**: Espionage | **MSTB**: ABDCMSTB.DAT (12 entries)

Abduction targets a **specific enemy character** at a specific system. The character must be present at the target system when the mission resolves. A successful abduction captures the target and brings them under your faction's control—they become a prisoner, and you gain the option to interrogate or trade them.

Losing a key character to abduction is often worse than losing a fleet. A captured Grand Admiral Thrawn means the Empire's best ship designer is sitting in an Alliance cell.

---

### Incite Uprising

**Skill**: Diplomacy | **MSTB**: INCTMSTB.DAT (13 entries)

An incited uprising shifts popularity sharply against the controlling faction at the target system. If loyalty falls below the `UPRIS1TB` threshold, a full uprising fires—the system's control flips, tying down enemy troops and halting manufacturing.

The Alliance uses this to bleed Imperial production without direct combat. A manufacturing system in constant revolt is effectively neutralized. The Empire can counter by sending agents to **Subdue Uprising** or by garrisoning troops heavy enough to suppress the population.

---

### Assassination

**Skill**: Combat | **MSTB**: ASSNMSTB.DAT (12 entries)

Assassination targets a **specific enemy character** and, on success, kills them permanently. Unlike abduction, the target is simply gone—their skills, their fleet command, their research contribution, all removed from play.

Assassination is the highest-stakes mission type. Success has enormous impact. Failure can mean the agent's own death or capture, and the enemy is alerted.

Best used against characters who are nearly irreplaceable: researchers with maximum skill, or an Admiral commanding a critical fleet with no qualified successor.

---

### Rescue

**Skill**: Combat | **MSTB**: RESCMSTB.DAT (12 entries)

Rescue frees a friendly character currently held prisoner by the opposing faction. The character must be at the target system. On success, the rescued character is returned to your faction's active roster.

Rescue missions also consult the **Decoy table** (FDECOYTB.DAT)—a decoy can redirect enemy counter-intelligence away from the rescue attempt, improving the odds when the enemy has heavy foil coverage at the target system.

---

### Subdue Uprising

**Skill**: Diplomacy | **MSTB**: SUBDMSTB.DAT (13 entries)

When one of your systems is in revolt, a Subdue Uprising mission attempts to restore loyalty before control flips. This mission is handled by the Uprising system rather than the standard Mission pipeline—it consults `UPRIS2TB.DAT` for its success probability rather than the standard MSTB tables.

Act fast. The window between an **uprising incident warning** and a full control flip is only a few game-ticks.

---

### Espionage

**Skill**: Espionage | **MSTB**: ESPIMSTB.DAT (12 entries)

Espionage reveals what is at the target system: fleets, characters, facilities, troop compositions. A successful espionage mission sets the system's exploration status to fully revealed.

This is essential for targeting deeper missions. You cannot run an assassination or sabotage against assets you do not know exist. Espionage fills the map and identifies high-value targets.

---

## The Decoy System

Characters running some mission types can use a **decoy** to misdirect enemy counter-intelligence. If a decoy is in place at or near the target system, the foil probability decreases—the enemy's agents are watching the wrong signal. The decoy system consults `FDECOYTB.DAT` for success probability.

## Character Escape

Captured characters are not held forever. Every game tick, each captive has a small chance of escaping based on `ESCAPETB.DAT`. The escape probability is modest—most characters will not escape without a Rescue mission—but it means that holding a high-value prisoner is not a guaranteed permanent advantage.

## Special Forces

Beyond named characters, each faction has access to **Special Forces** units—anonymous operatives with `mission_id` bitmasks that define which mission types they are eligible for. Special Forces serve as expendable mission runners when your named characters are occupied or too valuable to risk.

---

*Back to [Characters](characters.md) | [Tech Trees](tech-trees.md) | [Scenario Overview](scenario.md)*
