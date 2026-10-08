---
title: "Architecture"
description: "Crate dependency graph and module structure for the Open Rebellion codebase"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-01
tags: [architecture, crate-graph, entity-identity, simulation]
---

# Architecture

## Crate Dependency Graph

```
rebellion-app (quad-snd, rand_xoshiro)
├── rebellion-render (macroquad 0.4, egui-macroquad 0.17, image, nucleo-matcher, quad-snd)
│   └── rebellion-core
├── rebellion-data (notify [native only], web-sys + wasm-bindgen [wasm32 only])
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
├── events.rs         — EventCondition (16 variants)/Action (10 variants), chaining, deterministic rng, 7 carbonite fail + permanent freeze constants (~780 LOC, 17 tests)
├── ai.rs             — AISystem, per-fleet targeting, deconfliction, two-pass deployment, battle penalty, config-driven (1121 LOC, 13 tests)
├── tuning.rs         — GameConfig: 16 externalized AI/movement/production parameters, parity/augmentation tagged (~160 LOC)
├── movement.rs       — MovementOrder, Euclidean distance-based transit, config-aware variant (625 LOC, 19 tests). `cancel_orders_to(system)` cancels all in-transit orders targeting a given system (used by Death Star cleanup).
├── fleet_join.rs     — Joining and splitting fleets: a move onto a fleet hands over its capital ships, Create Fleet, the join checks (FUN_004ffc90, FUN_004feca0; ghidra/notes/fleet-join-split.md)
├── troop_transport.rs — Fleet cargo for regiments: loading through the Fleet window, the hold, unloading by hand and regiments travelling on their own (FUN_00556390, FUN_00552300)
├── delivery.rs       — En-route delivery of manufactured objects (F-030)
├── mission_detection.rs — The mission detection manager (FUN_00547f60): decoys, detection, betrayal, exposure
├── mission_planning.rs — The AI mission planners' team and decoy selectors
├── fog.rs            — FogState, visibility sets, dim rendering tiers (373 LOC, 9 tests)
├── combat.rs         — Space combat 7-phase pipeline, ground combat, CombatPhaseFlags
├── bombardment.rs    — Orbital bombardment: Euclidean distance / GNPRTB[0x1400]
├── blockade.rs       — Fleet-presence blockade, manufacturing halt, troop destruction
├── uprising.rs       — Recovered revolt lifecycle, uprising incident (UPRIS1TB/UPRIS2TB codes), Subdue gain, disaster
├── death_star.rs     — Construction countdown, planet destruction, nearby-warning scan. `cleanup_destroyed_system()` removes all entities (fleets, troops, facilities) and cancels in-transit orders to the destroyed system.
├── research.rs       — 3 tech trees (Ship/Troop/Facility), MSTB difficulty lookup
├── jedi.rs           — 4-tier Force progression (None→Aware→Training→Experienced), detection
├── victory.rs        — current HQ/Death Star outcome checks; parity divergences documented
├── betrayal.rs       — Loyalty-driven faction defection, UPRIS1TB threshold, immunity flag
├── story_events.rs   — 4 scripted story chains (Dagobah, Final Battle, Bounty Hunters, Jabba), 5-case palace outcomes, 5-stage carbonite countdown, telemetry twins (0x200, 0x231), CharactersCoLocated condition. Notification events removed (Phase 3b — belong in economy tick).
├── commands.rs       — Shared command registry (16 CommandDef entries) for GUI palette + CLI
├── game_events.rs    — GameEventRecord struct + 57 event type constants for JSONL telemetry
├── effects.rs        — GameEffect enum (39 variants), EffectPhase ordering, monoidal composition, inversion
├── economy.rs        — Full 18-function economy tick (FUN_00508250, the community dump calls it adjust_and_deploy_each_system): resource caps, support drift, collection rate, KDY modifier, troop-based side resolution, garrison, troop/fleet summary (SystemSummary), incident state (IncidentFlags), uprising visibility. 17 GNPRTB indices. ~1200 LOC.
└── repair.rs         — Ship repair framework: RepairSystem at shipyard systems, damage_control rate
```

## Render Modules (rebellion-render)

```
crates/rebellion-render/src/
├── lib.rs              — Galaxy map (pan/zoom/click), system info panel
├── game_menu.rs        — The original Game Menu Window (FUN_00442860), shared by the speed menu and the object pop-up menu
├── object_menu.rs      — The right-click pop-up menu for a system window's characters, special forces, fleets and regiments, and the Fleet window's capital ships (FUN_004ac5c0)
├── targeting.rs        — The galaxy view's targeting mode after Mission: pointer capture and REBEXE cursor 1002
├── mission_dialog.rs   — The original mission dialog (FUN_0046a750), opened by targeting
├── system_window.rs / sector_window.rs — The original modeless system and sector windows
├── quadrant_icons.rs   — The four icons a sector window draws around each planet (FUN_00459e30): which rule shows each, and its art
├── fleet_window.rs     — The Fleet window (type 4, FUN_004a2630): a system's fleets, their contents under four tabs, loading, unloading, joining and splitting
├── defenses_window.rs  — The System Defenses window (type 10, FUN_004a7790)
├── missions_window.rs  — The Missions window (type 11, FUN_0049f130)
├── fleet_finder.rs     — The Fleet and Ship Finder (type 0x15, FUN_00461750), opened by the cockpit control or F3
├── move_confirmation.rs — The Confirmed Move window (FUN_0044f060)
├── game_speed.rs       — The cockpit's day readout and its speed menu (FUN_00422ce0)
├── main_menu.rs        — Title screen with New Game / Load Game / Quit
├── video_player.rs     — Native cutscene playback from decoded PNG frame sequences + WAV sidecars; wasm32 stub returns finished immediately
├── theme.rs            — Star Wars egui theme: dark space bg, gold/amber accents, Liberation Sans font
├── message_log.rs      — Scrollable egui event feed, 7 color-coded categories (380 LOC)
├── fleet_movement.rs   — Diamond fleet icons, dashed route lines, ETA labels, fleet hover detection
├── fog.rs              — Dim overlays for unexplored/unseen systems
├── audio.rs            — AudioVolumeState, SfxKind, MusicTrack, draw_audio_controls (egui widget)
├── encyclopedia.rs     — 4-tab entity browser with BMP texture cache from EData/
├── bmp_cache.rs        — DllSource enum, BMP/PNG texture cache with HD fallback, WASM path rebasing, named DLL resource ID catalog
├── cockpit.rs          — Faction cockpit chrome (top/bottom bars), 9 control buttons, CockpitViewport
├── tactical_view.rs    — 2D tactical combat: BattleSession, ship placement, phased combat, targeting, retreat
├── ground_combat.rs    — Ground combat: regiment engagement, animated bars, win/loss results
├── event_screen.rs     — Full-screen event overlays for story events. event_id_to_resource() maps story IDs to STRATEGY.DLL BMP offsets with heritage_known branching for Final Battle variants.
├── advisor.rs          — Animated droid advisors (C-3PO/R2-D2 or Imperial), priority message queue, BIN-driven frame sequencing with BMP modulo fallback
└── panels/
    ├── mod.rs           — PanelAction enum: panel, mission, save, and combat actions
    ├── game_setup.rs    — Galaxy size, difficulty, faction selection (replaces faction_select)
    ├── officers.rs       — Character roster with skill bars, full detail view (Force, location, skills)
    ├── fleets.rs         — Fleet roster: composition, assign/remove officers (joining is on the Fleet window)
    ├── manufacturing.rs  — Production queue manager
    ├── missions.rs       — Active missions with progress and cancel; missions start from the object pop-up menu
    ├── research.rs       — 3 tech tree tabs, active project progress, character assignment
    ├── jedi.rs           — Force-sensitive roster, tier progression, training controls
    ├── bombardment.rs    — Orbital bombardment targeting: fleet selection, damage forecast, fire
    ├── death_star.rs     — Death Star: construction progress, superlaser targeting, movement orders
    ├── loyalty.rs        — Loyalty dashboard: per-system danger, uprising risk, betrayal risk
    ├── save_load.rs      — Save/load UI: 10 slots, auto-save
    ├── mod_manager.rs    — Mod Manager: discover, enable/disable, reload, dependency display
    └── command_palette.rs — developer command palette: simulation and per-system interface commands (debug or native builds)
```

## App Modules (rebellion-app)

```
crates/rebellion-app/src/
├── main.rs   — Entry point, interactive loop, panel action handling (~7,300 LOC; no headless tests, see agent-tooling.md)
├── audio.rs  — quad-snd AudioEngine: load, play_sfx, play_music, volume sync, WASM audio base-path resolution
├── dev_commands.rs — The command palette's gate (`OPEN_REBELLION_DEV`), and behind it the native command script (`OPEN_REBELLION_COMMANDS`) and campaign seed (`OPEN_REBELLION_SEED`)
├── interface_test_fixture.rs — Test-only interface fixture scenarios and the observations the browser gates in tools/interface-parity read
├── tactical_flow.rs / tactical_test_fixture.rs — The production tactical-battle entry and its test-only direct entry
├── runtime_pack.rs — Parser for the browser runtime asset pack
└── web_accessibility.rs / web_replay.rs — The browser accessibility bridge and the native/WASM replay runner

Decoded cutscene assets are intentionally kept out of git. `scripts/decode-cutscenes.sh` expands `assets/references/ref-videos/*.webm` into `assets/references/cutscene-frames/<name>/frame-*.png`, `metadata.json`, and sibling `<name>.wav` files for the native `VideoPlayer`.
```

## Data Modules (rebellion-data)

```
crates/rebellion-data/src/
├── seeds.rs      — Game seeding: 3-system model, character stat rolling, named placement (~1200 LOC, 8 tests)
├── save.rs       — Save/load: bincode snapshots and save slots; only the current SAVE_VERSION loads, with no migrations (save-load.md). Native uses filesystem saves; WASM uses browser localStorage through the gl.js loader's imports, with separate metadata keys for fast slot listing.
├── mods.rs       — Mod loader + ModRuntime: TOML manifest, RFC 7396 merge patch, semver, hot reload
├── simulation.rs — Tick orchestrator: SimulationStates bundle + run_simulation_tick() (~449 LOC)
└── integrator.rs — PerceptionIntegrator: all world mutation + telemetry emission (~1,200 LOC, 17 apply methods). Betrayal emits reveal-before-flip (EVT_TRAITOR_REVEALED) + side-change-after-flip (EVT_SIDE_CHANGE).
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
- `System` -- position, sector ref, popularity (alliance/empire f32), asset lists (fleets, units, facilities), `espionage_rating: f32` (reduces incite uprising probability)
- `Sector` -- named region, SectorGroup, position, child system list
- `CapitalShipClass` / `FighterClass` -- class templates, not instances
- `Character` -- 8 `SkillPair` (base+variance), Jedi fields, role flags, major/minor
- `Fleet` -- `capital_ships: Vec<ShipInstance>` (per-hull records: hull_current, alive, shield_weapon_packed, faction_is_alliance) + characters at a system location. Helper methods: `ship_count()`, `ship_counts_by_class()`, `is_empty()`. `ShipEntry` (aggregate class+count) was removed in Knesset Hephaestus.
- `ManufacturingFacilityInstance` -- runtime facility record with `is_shipyard: bool` (used by economy to detect shipyard systems)
- `ProductionFacilityInstance` -- runtime facility record with `is_mine: bool` (used by economy to count raw material sources)

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

### Simulation Loop (rebellion-data/src/simulation.rs → integrator.rs)
```
run_simulation_tick():
  integrator = PerceptionIntegrator::new(tick, wall_ms)
  0. EconomySystem::advance       → integrator.apply_economy_events()
  1. ManufacturingSystem::advance  → integrator.apply_build_completions()
  2. MovementSystem::advance       → integrator.apply_arrivals()
  3. CombatSystem::resolve_*       → integrator.apply_space_combat() / apply_ground_combat()
  4. FogSystem::advance            → integrator.emit_fog_reveals()
  5. MissionSystem::advance        → integrator.apply_mission_result()
  5b. check_escapes()              → integrator.apply_escape_effects()
  6. EventSystem::advance          → integrator.apply_fired_events()
  7. AISystem::advance             → integrator.apply_ai_actions()
  8. BlockadeSystem::advance       → integrator.apply_blockade_events()
  9. UprisingSystem::advance       → integrator.apply_uprising_events()
  10. BetrayalSystem::advance      → integrator.apply_betrayal_events()
  11. DeathStarSystem::advance     → integrator.apply_death_star_events()
  12. ResearchSystem::advance      → integrator.apply_research_results()
  13. JediSystem::advance          → integrator.apply_jedi_events()
  14. VictorySystem::check         → integrator.apply_victory()
  15. Campaign snapshot            → integrator.emit_campaign_snapshot()
  return integrator.finish()

Interactive game (main.rs):
  tick_events = clock.advance(dt)
  if tick_events not empty:
    EconomySystem::advance → apply support drift + control resolution
    ManufacturingSystem::advance → apply_build_completion_inner() + message log
    MovementSystem::advance → update fleet.location + system.fleets
    (remaining systems identical to simulation.rs pattern)
  draw_galaxy_map → draw_fog_overlay → draw_fleet_overlays
  egui_macroquad::ui: panels + object_menu + targeting + encyclopedia + system_info + message_log
```

### Save/Load Flow
```
egui save/load panel
    | rebellion-data::save::{save_slot, load_slot, list_saves, delete_slot}
    v
SaveState + SaveMeta
    | native: filesystem `saves/slotNN.sav`
    | wasm32: `web_sys::Storage` localStorage
    |         - payload stored as base64 snapshot
    |         - metadata stored under separate keys for fast `list_saves()`
    v
Interactive session restores GameWorld + simulation state bundle
```

### Parity Eval Flow
```
rebellion-playtest --jsonl / campaign snapshot output
    | scripts/eval_parity.py
    | loads scripts/golden_values.json
    v
Golden-value oracle
    - 111 mapped GNPRTB bindings
    - combat/economy/research/AI/movement/victory constants
    v
Pass/fail parity report
```

### Fleet Arrival Lifecycle
When `MovementSystem::advance` returns an `ArrivalEvent`:
1. `fleet.location = arrival.system` — update the fleet's position
2. `origin_system.fleets.retain(|k| k != fleet)` — remove from origin
3. `dest_system.fleets.push(fleet)` — add to destination
All systems that query `System.fleets` (combat, fog, blockade, victory) see correct positions.

See `agent_docs/simulation.md` for full API reference on the advance() pattern.

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
4. `hovered_fleet(world, movement, cam, mx, my) -> Option<FleetKey>` -- fleet hit detection
5. `draw_system_info_panel(ctx, world, state)` -- egui right panel (selected system)
6. `object_menu::draw_object_menu(ctx, menu, ...)` -- the original right-click pop-up menu for a system window's objects
7. `targeting::{capture_pointer, draw_targeting_cursor}` -- the galaxy view's targeting mode after the menu's Mission
8. `draw_status_bar(ctx, world, clock, audio_vol)` -- bottom bar with speed/audio controls
9. `draw_message_log(ctx, log, state)` -- scrollable event feed
10. Panel functions: `draw_officers`, `draw_fleets`, `draw_missions`, `draw_research`, `draw_jedi`
11. `draw_encyclopedia(ctx, world, state)` -- floating 4-tab entity browser
12. `draw_main_menu(ctx) -> MainMenuAction` -- title screen
13. `draw_game_setup(ctx, state) -> GameSetupAction` -- new game config

`GalaxyMapState` holds all mutable UI state: camera position, zoom, selected/hovered system, drag tracking, the targeting flag, right-click start position.

Input: mouse within map area only. Right-drag pans, scroll zooms (0.3x-5.0x). Left-click selects nearest system within hover radius, except while targeting, when the release targets it. A right-click on the map opens nothing; what the original's galaxy view does on a right-click is not traced yet. A system window's Personnel and Troops lists open the object pop-up menu.

## Adding a New Entity Type

1. Add slotmap key to `crates/rebellion-core/src/ids.rs` via `new_key_type!`
2. Add dat/ enum or struct if needed (`crates/rebellion-core/src/dat/mod.rs`)
3. Add world struct to `crates/rebellion-core/src/world/mod.rs`, add SlotMap arena to `GameWorld`
4. Add dat-dumper parser in `tools/dat-dumper/src/types/`, add module to `types/mod.rs`, register in `registry.rs`
5. Add conversion step in `crates/rebellion-data/src/lib.rs`
6. Update rendering if the entity is visible on the map
