---
title: "Game Domain: Star Wars Rebellion"
description: "Game mechanics overview covering galaxy structure, factions, units, missions, and combat"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-19
tags: [reference, game-mechanics, factions, missions]
---

# Game Domain: Star Wars Rebellion

Star Wars Rebellion (1998, Coolhand/LucasArts) is a 4X galactic strategy game. Alliance vs Empire, controlling systems, building fleets, training characters, running missions.

## Galaxy Structure

- **Galaxy**: 200 star systems across 20 sectors
- **Sector**: Named region with `SectorGroup` (`Core`, `RimInner`, `RimOuter`). Contains multiple systems.
- **System**: Atomic unit of territory. X/Y position (range ~100-840), belongs to one sector. Holds facilities, ground units, orbiting fleets.
- **Popularity**: Each system has Alliance/Empire popularity (0.0-1.0). Shifts via diplomacy, occupation, events.

## Factions

Alliance and Empire only. Each has:
- Headquarters system (Alliance: variable start, Empire: Coruscant)
- Characters, fleets, manufacturing queues, tech tree
- Victory: the win word (`systems/victory.md`): the Empire on the Alliance HQ gone, the Alliance on Coruscant held or destroyed; Standard also needs the enemy leaders captured

## Characters

- **Major** (6 in base): Luke, Vader, etc. Fixed stats, unique abilities, plot events.
- **Minor** (54): Generic officers. Stats use base+variance: `final = base + rng(0..=variance)` at game start.
- **8 skill pairs**: diplomacy, espionage, ship_design, troop_training, facility_design, combat, leadership, loyalty
- **Jedi**: `jedi_probability` (0-100), `jedi_level` (base+variance). `is_known_jedi`, `is_jedi_trainer` (from MJCHARSD)
- **Roles**: `can_be_admiral` (fleet command), `can_be_commander` (missions), `can_be_general` (ground combat)
- **Loyalty**: `loyalty` skill pair + `is_unable_to_betray` flag (Luke, Vader cannot switch sides)

## Military Units

### Capital Ships (30 classes)
Class templates, not instances. Key fields:
- Weapons on 4 arcs (fore/aft/port/starboard) x 3 types (turbolaser/ion/laser)
- Hull, shields (strength + recharge), engines, maneuverability, hyperdrive
- Fighter capacity, troop capacity, tractor beam, gravity well projector
- Detection, bombardment modifier, damage control

### Fighters (8 classes)
Squadron-based (`squadron_size` craft per squadron). Torpedoes for bombing. Same weapon arc structure as capital ships.

### Troops (10 types)
Ground combat: attack_strength, defense_strength, bombardment_defense.

### Special Forces (9 types)
Mission specialists with character-like skill pairs + `mission_id` bitmask for eligible mission types.

## Facilities

Built on system surfaces. Three categories:
- **Defense** (6 types): planetary shields, turbolasers, ion cannons. bombardment_defense, attack_strength, shield_strength.
- **Manufacturing** (6 types): shipyards, training centers. processing_rate determines build speed.
- **Production** (2 types): mines, refineries. Same binary layout as manufacturing.

## Production System

- Every buildable entity has: `refined_material_cost`, `maintenance_cost`, `research_order`, `research_difficulty`
- `production_family` / `next_production_family` -- linked list through the tech tree
- Production facilities generate raw materials; manufacturing facilities convert them to units

## Game Balance Parameters

### GNPRTB (213 entries)
Master balance table. Each parameter has 8 values: Development, Alliance Easy/Med/Hard, Empire Easy/Med/Hard, Multiplayer. Controls travel time, combat recovery, mission rewards, diplomacy effects, events, economy, uprising mechanics, Jedi training. 29% documented, 71% unknown (Ghidra RE target).

### SDPRTB (35 entries)
Per-faction modifiers, same difficulty breakdown as GNPRTB.

## Seed Tables (9 files)
Starting unit/facility deployments for new games:
- CMUNAFTB/CMUNEFTB -- fleet seeds (Alliance/Empire)
- CMUNALTB/CMUNEMTB -- army seeds
- CMUNCRTB -- Empire Coruscant garrison
- CMUNHQTB/CMUNYVTB -- Alliance HQ and Yavin
- FACLCRTB/FACLHQTB -- starting facilities

## Implemented Systems

[simulation.md](simulation.md) lists every simulation system, its file and
its effects; [systems/](systems/) holds the detail. Saves:
[save-load.md](save-load.md). Mods: [mod-runtime.md](mod-runtime.md).

## Unimplemented Mechanics

- **Sound effects / music**: engine exists (quad-snd), asset coverage incomplete
- **Remaining story events**: ~15 additional story beats documented in RE but not implemented (Noghri, special forces chains)
- **WASM save/load**: localStorage only; IndexedDB is M3 (`save-load.md`, "Browser storage")
- **WASM mod loading**: no browser file picker integration
