---
title: "Architecture"
description: "Crate dependency graph and module structure for the Open Rebellion codebase"
category: "agent-docs"
created: 2026-03-11
updated: 2026-03-21
tags: [architecture, crate-graph, entity-identity, simulation]
---

# Architecture

## Crate Dependency Graph

```
rebellion-app (quad-snd, rand_xoshiro)
├── rebellion-render (macroquad 0.4, egui-macroquad 0.17, image, nucleo-matcher)
│   └── rebellion-core
├── rebellion-data (notify [native only])
│   ├── rebellion-core
│   └── dat-dumper (library mode)
└── rebellion-core (slotmap 1.0, serde 1)

rebellion-playtest (clap, rand_xoshiro, serde_json)  [headless play-test binary]
├── rebellion-data
└── rebellion-core

dat-dumper (tools/) -- standalone CLI + library
└── clap 4, serde, serde_json, anyhow, pelite
```

## Simulation Modules (rebellion-core)

All simulation systems follow a stateless advance pattern:
```rust
System::advance(state, world, &[TickEvent]) -> Vec<ResultEvent>
```
Caller applies effects to GameWorld. Deterministic—same inputs produce same outputs. Testable in isolation.

```
crates/rebellion-core/src/
├── tick.rs           — GameClock, GameSpeed, TickEvent (280 LOC, 13 tests)
├── manufacturing.rs  — ProductionQueue, ManufacturingState, blockade-aware advance (520 LOC, 13 tests)
├── missions.rs       — 9 mission types, MSTB probability tables, 6 MissionEffect variants (880 LOC, 14 tests)
├── events.rs         — EventCondition/Action, chaining, deterministic rng (728 LOC, 17 tests)
├── ai.rs             — AISystem, per-fleet targeting, deconfliction, two-pass deployment, battle penalty, config-driven (1121 LOC, 13 tests)
├── tuning.rs         — GameConfig: 16 externalized AI/movement/production parameters, parity/augmentation tagged (~160 LOC)
├── movement.rs       — MovementOrder, Euclidean distance-based transit, config-aware variant (625 LOC, 19 tests)
├── fog.rs            — FogState, visibility sets, dim rendering tiers (373 LOC, 9 tests)
├── combat.rs         — Space combat 7-phase pipeline, ground combat, CombatPhaseFlags
├── bombardment.rs    — Orbital bombardment: Euclidean distance / GNPRTB[0x1400]
├── blockade.rs       — Fleet-presence blockade, manufacturing halt, troop destruction
├── uprising.rs       — Incite/subdue with UPRIS1TB/UPRIS2TB, 10-tick incident cooldown
├── death_star.rs     — Construction countdown, planet destruction, nearby-warning scan
├── research.rs       — 3 tech trees (Ship/Troop/Facility), MSTB difficulty lookup
├── jedi.rs           — 4-tier Force progression (None→Aware→Training→Experienced), detection
├── victory.rs        — HQ capture, Death Star fire/destroyed victory conditions
├── betrayal.rs       — Loyalty-driven faction defection, UPRIS1TB threshold, immunity flag
├── story_events.rs   — 4 scripted story chains (Dagobah, Final Battle, Bounty Hunters, Jabba)
├── commands.rs       — Shared command registry (16 CommandDef entries) for GUI palette + CLI
└── game_events.rs    — GameEventRecord struct + 26 event type constants for JSONL telemetry
```

## Render Modules (rebellion-render)

```
crates/rebellion-render/src/
├── lib.rs              — Galaxy map (pan/zoom/click), system info panel, context menus (system + fleet)
├── main_menu.rs        — Title screen with New Game / Load Game / Quit
├── theme.rs            — Star Wars egui theme: dark space bg, gold/amber accents, Liberation Sans font
├── message_log.rs      — Scrollable egui event feed, 7 color-coded categories (380 LOC)
├── fleet_movement.rs   — Diamond fleet icons, dashed route lines, ETA labels, fleet hover detection
├── fog.rs              — Dim overlays for unexplored/unseen systems
├── audio.rs            — AudioVolumeState, SfxKind, MusicTrack, draw_audio_controls (egui widget)
├── encyclopedia.rs     — 4-tab entity browser with BMP texture cache from EData/
├── bmp_cache.rs        — DllSource enum, BMP/PNG texture cache with HD fallback, WASM stubs
├── cockpit.rs          — Faction cockpit chrome (top/bottom bars), 9 control buttons, CockpitViewport
├── tactical_view.rs    — 2D tactical combat: BattleSession, ship placement, phased combat, targeting, retreat
├── ground_combat.rs    — Ground combat: regiment engagement, animated bars, win/loss results
├── combat_view.rs      — Combat results integration into message log
├── advisor.rs          — Animated droid advisors (C-3PO/R2-D2 or Imperial), priority message queue, frame cycling
├── victory_screen.rs   — Victory/defeat egui modal with faction narrative
└── panels/
    ├── mod.rs           — PanelAction enum (31 variants, including context menu + combat actions)
    ├── game_setup.rs    — Galaxy size, difficulty, faction selection (replaces faction_select)
    ├── officers.rs       — Character roster with skill bars, full detail view (Force, location, skills)
    ├── fleets.rs         — Fleet editor: composition, assign/remove officers, merge fleets
    ├── manufacturing.rs  — Production queue manager
    ├── missions.rs       — Mission dispatch with probability preview
    ├── research.rs       — 3 tech tree tabs, active project progress, character assignment
    ├── jedi.rs           — Force-sensitive roster, tier progression, training controls
    ├── bombardment.rs    — Orbital bombardment targeting: fleet selection, damage forecast, fire
    ├── death_star.rs     — Death Star: construction progress, superlaser targeting, movement orders
    ├── loyalty.rs        — Loyalty dashboard: per-system danger, uprising risk, betrayal risk
    ├── save_load.rs      — Save/load UI: 10 slots, auto-save
    ├── mod_manager.rs    — Mod Manager: discover, enable/disable, reload, dependency display
    └── command_palette.rs — VS Code-style fuzzy command palette (nucleo-matcher, debug-only)
```

## App Modules (rebellion-app)

```
crates/rebellion-app/src/
├── main.rs   — Entry point, simulation loop, effect application helpers (323 LOC)
└── audio.rs  — quad-snd AudioEngine: load, play_sfx, play_music, volume sync
```

## Data Modules (rebellion-data)

```
crates/rebellion-data/src/
├── seeds.rs      — Game seeding: 3-system model, character stat rolling, named placement (~1200 LOC, 8 tests)
├── save.rs       — Save/load: bincode snapshots, save slots, version migration (v5)
├── mods.rs       — Mod loader + ModRuntime: TOML manifest, RFC 7396 merge patch, semver, hot reload
└── simulation.rs — Shared simulation tick: SimulationStates bundle + run_simulation_tick() → Vec<GameEventRecord>
```

## Type System: Two Layers

### Layer 1: Binary mirror (`rebellion-core/src/dat/`)
Structs that exactly match .DAT file field layout. Used only for import/export. No game logic.
- `Faction` (Alliance, Empire, Neutral)
- `GalaxySize` (Standard=1, Large=2, Huge=3)
- `SectorGroup` (Core=1, RimInner=2, RimOuter=3)
- `ExplorationStatus` (Explored=0x90, Unexplored=0x92)

### Layer 2: Runtime world (`rebellion-core/src/world/`)
Rich types used by game logic, rendering, save/load. Slotmap keys for all inter-entity references.
- `GameWorld` -- root aggregate, 11 SlotMap arenas (systems, sectors, capital_ship_classes, fighter_classes, characters, fleets, troops, special_forces, defense_facilities, manufacturing_facilities, production_facilities) + `GnprtbParams` + `mission_tables: HashMap<String, MstbTable>`
- `System` -- position, sector ref, popularity (alliance/empire f32), asset lists (fleets, units, facilities)
- `Sector` -- named region, SectorGroup, position, child system list
- `CapitalShipClass` / `FighterClass` -- class templates, not instances
- `Character` -- 8 `SkillPair` (base+variance), Jedi fields, role flags, major/minor
- `Fleet` -- ships + characters at a system location

## Entity Identity (Dual-Key Pattern)

Every entity carries both:

1. **`DatId(u32)`** -- original binary ID. Encodes `(family_byte << 24) | sequential_index`. Family byte identifies entity class (0x90=explored system, 0x92=unexplored system). Other entity family bytes are encoded in each file's header `family_id` field. Preserved for serialization and mod cross-references.

2. **Slotmap key** (`SystemKey`, `SectorKey`, etc.) -- runtime arena handle with generational index. Only meaningful within the `GameWorld` that created it. Used for all inter-entity references at runtime.

Lookup maps (`HashMap<u32, *Key>`) bridge DatId to slotmap keys during loading in `rebellion-data/src/lib.rs`.

## Data Flow

```
.DAT binary files
    | ByteReader (little-endian cursor, tools/dat-dumper/src/codec.rs)
    v
dat-dumper types (DatRecord trait, tools/dat-dumper/src/types/)
    | rebellion-data::load_game_data() (crates/rebellion-data/src/lib.rs)
    v
world types (GameWorld with SlotMap arenas)
    | rebellion-render (crates/rebellion-render/src/lib.rs)
    v
macroquad drawing + egui panels
```

### Simulation Loop (rebellion-app/src/main.rs)
```
Each frame:
  tick_events = clock.advance(dt)
  if tick_events not empty:
    ManufacturingSystem::advance_with_blockade → CompletionEvents (skips blockaded systems)
    MovementSystem::advance     → ArrivalEvents    → update fleet.location + system.fleets
    CombatSystem (per system)   → SpaceCombat/Ground/Bombardment → apply damage
    FogSystem::advance          → RevealEvents
    MissionSystem::advance      → MissionResults   → apply effects to GameWorld
    EventSystem::advance        → FiredEvents       → apply actions to GameWorld
    AISystem::advance           → AIActions         → apply to MissionState, ManufacturingState
    BlockadeSystem::advance     → BlockadeEvents   → destroy troops, update blockade set
    UprisingSystem::advance     → UprisingEvents   → flip controlling_faction
    DeathStarSystem::advance    → DSEvents         → construction, planet destruction, warnings
    ResearchSystem::advance     → ResearchResults  → advance tech levels
    JediSystem::advance         → JediEvents       → tier advancement, detection
    VictorySystem::check        → VictoryOutcome?  → end game if terminal condition met
  draw_galaxy_map → draw_fog_overlay → draw_fleet_overlays → hovered_fleet (fleet context menu detection)
  egui_macroquad::ui: panels + context_menus + encyclopedia + system_info + message_log + status_bar
```

### Fleet Arrival Lifecycle
When `MovementSystem::advance` returns an `ArrivalEvent`:
1. `fleet.location = arrival.system` — update the fleet's position
2. `origin_system.fleets.retain(|k| k != fleet)` — remove from origin
3. `dest_system.fleets.push(fleet)` — add to destination
All systems that query `System.fleets` (combat, fog, blockade, victory) see correct positions.

See `@agent_docs/simulation.md` for full API reference on the advance() pattern.

### Loading Order (rebellion-data/src/lib.rs)
1. Sectors (SECTORSD.DAT) -- must come first, systems reference sectors
2. Systems (SYSTEMSD.DAT) -- registers each system in its parent sector
3. Capital ships (CAPSHPSD.DAT)
4. Fighters (FIGHTSD.DAT)
5. Major characters (MJCHARSD.DAT)
6. Minor characters (MNCHARSD.DAT)

### Round-Trip Validation
Round-trip validation is enforced inside `parse_and_dump` in `tools/dat-dumper/src/registry.rs`. The byte comparison utility is in `tools/dat-dumper/src/validate.rs`. If reserialized bytes differ at any offset, the parse is wrong. This guarantees complete format understanding.

## Rendering Architecture

`rebellion-render/src/lib.rs` exposes composable functions called each frame:
1. `draw_galaxy_map(world, state) -> CameraView` -- star map with pan/zoom/click, returns camera params
2. `draw_fog_overlay(world, fog, cam)` -- dim non-visible systems
3. `draw_fleet_overlays(world, movement, cam)` -- fleet icons and route lines
4. `hovered_fleet(world, movement, cam, mx, my) -> Option<FleetKey>` -- fleet hit detection for context menus
5. `draw_system_info_panel(ctx, world, state)` -- egui right panel (selected system)
6. `draw_system_context_menu(ctx, world, state, faction) -> Option<PanelAction>` -- right-click system popup
7. `draw_fleet_context_menu(ctx, world, movement, state) -> Option<PanelAction>` -- right-click fleet popup
8. `draw_status_bar(ctx, world, clock, audio_vol)` -- bottom bar with speed/audio controls
9. `draw_message_log(ctx, log, state)` -- scrollable event feed
10. Panel functions: `draw_officers`, `draw_fleets`, `draw_manufacturing`, `draw_missions`, `draw_research`, `draw_jedi`
11. `draw_encyclopedia(ctx, world, state)` -- floating 4-tab entity browser
12. `draw_main_menu(ctx) -> MainMenuAction` -- title screen
13. `draw_game_setup(ctx, state) -> GameSetupAction` -- new game config

`GalaxyMapState` holds all mutable UI state: camera position, zoom, selected/hovered system, drag tracking, context menu state (system + fleet), right-click start position.

Input: mouse within map area only. Right-drag pans, scroll zooms (0.3x-5.0x). Left-click selects nearest system within hover radius. Right-click (no drag) opens context menu on hovered system or fleet.

## Adding a New Entity Type

1. Add slotmap key to `crates/rebellion-core/src/ids.rs` via `new_key_type!`
2. Add dat/ enum or struct if needed (`crates/rebellion-core/src/dat/mod.rs`)
3. Add world struct to `crates/rebellion-core/src/world/mod.rs`, add SlotMap arena to `GameWorld`
4. Add dat-dumper parser in `tools/dat-dumper/src/types/`, add module to `types/mod.rs`, register in `registry.rs`
5. Add conversion step in `crates/rebellion-data/src/lib.rs`
6. Update rendering if the entity is visible on the map
