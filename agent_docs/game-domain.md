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

See `@agent_docs/simulation.md` for full API reference on all 15 systems.

### Core Loop (v0.2.0)
- **Tick system** (`tick.rs`): Frame-independent GameClock, GameSpeed (Paused/Normal/Fast/Faster), TickEvent markers
- **Manufacturing** (`manufacturing.rs`): Per-system production queues with overflow propagation; blockade-aware (`advance_with_blockade`)
- **Events** (`events.rs`): Conditional triggers (TickReached, CharacterAtSystem, Random, EventFired), event chaining
- **AI** (`ai.rs`): Rule-based opponent — officer assignment, production priority, fleet deployment, espionage dispatch. Re-evaluates every 7 ticks.
- **Movement** (`movement.rs`): Fleet hyperspace transit with speed from slowest hyperdrive rating
- **Fog of war** (`fog.rs`): Per-faction monotonic visibility with advance intel at 50% transit
- **Mod loader** (`rebellion-data/src/mods.rs`): TOML manifests, RFC 7396 merge patch, semver, hot reload

### War Machine (v0.4.0)
- **9 mission types** (`missions.rs`): Diplomacy, Recruitment, Sabotage, Assassination, Espionage, Rescue, Abduction, InciteUprising, Autoscrap — MSTB probability tables with quadratic fallback
- **Space combat** (`combat.rs`): 7-phase pipeline (weapon fire → shield absorb → hull damage → fighter engage → result), CombatPhaseFlags, per-system 5-tick cooldown
- **Ground combat** (`combat.rs`): Troop-by-troop resolution with regiment_strength comparison
- **Orbital bombardment** (`bombardment.rs`): `damage = sqrt(delta²) / GNPRTB[0x1400]`, minimum 1
- **Blockade** (`blockade.rs`): Hostile fleet without defender halts manufacturing, destroys in-transit troops
- **Uprising** (`uprising.rs`): UPRIS1TB (3 thresholds) start / UPRIS2TB (4 thresholds) subdue, 10-tick cooldown
- **Death Star** (`death_star.rs`): Construction countdown, superlaser fire (precondition checks from RE), nearby-warning scan
- **Research** (`research.rs`): 3 tech trees (Ship/Troop/Facility) per faction, `research_order` + `research_difficulty`
- **Jedi training** (`jedi.rs`): 4-tier Force progression (None→Aware→Training→Experienced), XP accumulation, detection checks
- **Victory conditions** (`victory.rs`): HQ capture, Death Star fire/destroyed, `resolved` flag prevents re-trigger
- **Save/load** (`rebellion-data/src/save.rs`): Bincode with OPENREB header, v4 format with mod metadata + FNV-1a hash, migration framework, 10 slots. See `@agent_docs/save-load.md`.

### Full Parity (v0.5.0)
- **4 scripted story chains** (`story_events.rs`): Luke Dagobah (0x221→0x210), Final Battle (0x220), Bounty Hunters (0x212), Jabba's Palace (0x380-0x383)
- **Han Solo speed bonus** (`movement.rs`): `hyperdrive_modifier` on Character, consulted by `fleet_ticks_per_hop()`
- **Betrayal system** (`betrayal.rs`): loyalty threshold via UPRIS1TB, `is_unable_to_betray` immunity, 50-tick cooldown
- **Decoy system** (`missions.rs`): FDECOYTB consultation during mission resolution
- **Escape system** (`missions.rs`): ESCAPETB per-tick check for captive characters, `check_escapes()` in main loop
- **Mission state flags**: `on_mission`, `on_hidden_mission`, `on_mandatory_mission` tracked on Character; `dispatch_guarded()` prevents double-dispatch
- **6 new EventConditions** + **6 new EventActions** for story event mechanics
- **15 Ghidra RE event ID constants** (0x12c–0x370)

### Mod Workshop (v0.6.0)
- **Sensor-radius fog** (`fog.rs`): fleets with detection capability reveal nearby systems within `detection * 15.0` coordinate radius
- **Captivity state** (`world/mod.rs`): `is_captive`, `captured_by: Option<Faction>`, `capture_tick: Option<u64>` on Character — set on capture, cleared on rescue/escape
- **Research pure contract** (`research.rs`): `advance()` returns results only, caller applies level-ups
- **Mod runtime** (`mods.rs`): `ModRuntime`, `ModConfig`, enable/disable with `config.toml` persistence, structured `ModError`, hot reload integration. See `@agent_docs/mod-runtime.md`.
- **Mod Manager panel** (`panels/mod_manager.rs`): egui floating window with mod list, enable/disable, reload, Tab shortcut
- **Save migration framework** (`save.rs`): versioned dispatch, mod metadata in header, FNV-1a hash. See `@agent_docs/save-load.md`.

### Mission Effects (applied in main.rs)
| Effect | World Mutation |
|--------|---------------|
| `PopularityShifted` | Clamp `sys.popularity_alliance` / `_empire` |
| `CharacterRecruited` | No-op (covered by popularity) |
| `FacilitySabotaged` | Remove facility from system + arena by index |
| `CharacterKilled` | Remove from all fleets, remove from character arena |
| `CharacterCaptured` | Set captivity fields, flip faction, remove from fleets |
| `CharacterRescued` | Restore faction, clear captivity fields |
| `CharacterEscaped` | Restore home faction (`is_alliance`), clear captivity |
| `SystemIntelligenceGathered` | Set exploration_status to Explored |
| `UprisingStarted` | Shift popularity against controller |
| `CharacterBusy` | Set `on_mission = true` |
| `CharacterAvailable` | Clear `on_mission`, `on_hidden_mission` |
| `DecoyTriggered` | Log message (no world mutation) |

## Unimplemented Mechanics

- **Video playback**: Smacker to WebM, victory cutscenes
- **Sound effects / music**: engine exists (quad-snd), asset coverage incomplete
- **Remaining story events**: ~15 additional story beats documented in RE but not implemented (Noghri, special forces chains)
- **WASM save/load**: no IndexedDB/localStorage fallback
- **WASM mod loading**: no browser file picker integration
