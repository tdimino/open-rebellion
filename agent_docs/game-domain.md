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
- Victory: capture enemy HQ (or destroy Death Star / find Rebel base)

## Characters

- **Major** (6 in base): Luke, Vader, etc. Fixed stats, unique abilities, plot events.
- **Minor** (54): Generic officers. Stats use base+variance: `final = base + rng(0..=variance)` at game start.
- **8 skill pairs**: diplomacy, espionage, ship_design, troop_training, facility_design, combat, leadership, loyalty
- **Jedi**: `jedi_probability` (0-100), `jedi_level` (base+variance). DAT layer also has `is_known_jedi`, `is_jedi_trainer` (not yet promoted to world model)
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

See `@agent_docs/simulation.md` for full API reference.

- **Tick system** (`tick.rs`): Frame-independent GameClock, GameSpeed (Paused/Normal/Fast/Faster), TickEvent markers
- **Manufacturing** (`manufacturing.rs`): Per-system production queues with overflow propagation
- **Missions** (`missions.rs`): Diplomacy and recruitment — quadratic probability formula from rebellion2
- **Events** (`events.rs`): Conditional triggers (TickReached, CharacterAtSystem, Random, EventFired), event chaining
- **AI** (`ai.rs`): Rule-based opponent — officer assignment, production priority, fleet deployment. Re-evaluates every 7 ticks.
- **Movement** (`movement.rs`): Fleet hyperspace transit with speed from slowest hyperdrive rating
- **Fog of war** (`fog.rs`): Per-faction monotonic visibility with advance intel at 50% transit
- **Mod loader** (`rebellion-data/src/mods.rs`): TOML manifests, RFC 7396 merge patch, semver, hot reload

## Unimplemented Mechanics

- **Combat**: space (fleet engagement, 4-arc weapons, shields) and ground (troops + orbital bombardment). **RE COMPLETE** — 109 functions decompiled from REBEXE.EXE. Bombardment formula decoded (Euclidean distance / GNPRTB). Space combat 7-phase pipeline mapped. 71 combat GNPRTB parameters identified. See `ghidra/notes/` for full docs.
- **Espionage missions**: sabotage, assassination, abduction, rescue, incite uprising (diplomacy and recruitment ARE implemented)
- **Research**: tech tree via `research_order` + `research_difficulty` per unit class
- **Probe droids / sensor range**: fog of war currently uses fleet presence only, no sensor radius
- **Special character abilities**: Jedi training, betrayal, decoys, Han Solo speed bonus
- **Victory conditions**: capture enemy HQ, destroy Death Star / find Rebel base
