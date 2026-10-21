---
title: "Architecture"
description: "Crate dependency graph and module structure for the Open Rebellion codebase"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-20
tags: [architecture, crate-graph, entity-identity, simulation]
---

# Architecture

## Crate Dependency Graph

```
rebellion-app (quad-snd, rand_xoshiro)
├── rebellion-render (macroquad 0.4, egui-macroquad 0.17, image, nucleo-matcher, quad-snd)
│   ├── rebellion-windowing
│   └── rebellion-core
├── rebellion-windowing (no dependencies)
├── rebellion-data (bincode, toml, semver, sha2; notify [native only])
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

The advance contract and its exceptions: `simulation.md`, "The Advance Contract".

```
crates/rebellion-core/src/
├── tick.rs           — GameClock, GameSpeed, TickEvent
├── manufacturing.rs  — ProductionQueue, ManufacturingState, blockade-aware advance, a holder change's lost products (`serve_manager_sides`)
├── missions.rs       — 13 mission kinds, MSTB probability tables, the MissionEffect set (simulation.md)
├── events.rs         — EventCondition/EventAction, chaining, deterministic rng, the carbonite fail and permanent freeze constants
├── ai.rs             — AISystem: the side's round over the modules below (ai_scheduler.rs), rate+1 rounds a day, plus the port's mission-target picks (systems/ai-parity-tracker.md)
├── ai_scheduler.rs   — The computer player's round scheduler (FUN_00485b40, FUN_004be980, FUN_004bede0; ghidra/notes/ai-order-emission.md)
├── ai_records.rs     — The side AI's ship, fleet and system records, and the marks that persist between rounds
├── ai_strategy.rs    — Module 13 and the strategy controller: the mode pick and the sector plan
├── ai_regions.rs     — Module 8: the regional records and their postures
├── fleet_tasks.rs    — Module 9: the fleet tasks that gather warships at headquarters systems
├── character_tasks.rs — Module 12: the character tasks, classes G and I
├── ai_research.rs    — The AI's Research planners, one persistent planner a tree (systems/research.md)
├── ai_production.rs  — Production modules 1, 2, 3, 5, 6, 7, 10 and 11
├── tuning.rs         — GameConfig: AiConfig mission caps and scoring targets
├── movement.rs       — MovementOrder, Euclidean distance-based transit, config-aware variant
├── fleet_join.rs     — Joining and splitting fleets: a move onto a fleet hands over its capital ships, Create Fleet, the join checks (FUN_004ffc90, FUN_004feca0; ghidra/notes/fleet-join-split.md)
├── troop_transport.rs — Fleet cargo for regiments: loading through the Fleet window, unloading by hand and regiments travelling on their own (FUN_00556390, FUN_00552300)
├── delivery.rs       — En-route delivery of manufactured objects (F-030)
├── mission_detection.rs — The mission detection manager (FUN_00547f60): decoys, detection, betrayal, exposure
├── mission_planning.rs — The AI mission planners' team and decoy selectors
├── fog.rs            — FogState, visibility sets, dim rendering tiers
├── combat.rs         — side_present, CombatForce and CombatSide, shared with the tactical engine
├── assault.rs        — The Planetary Assault order `0x234`: enable (FUN_005814d0) and the run (FUN_00563390 -> FUN_0058c120: defence fire, engagements, collateral, landing); the integrator resolves each order (ghidra/notes/ground-assault.md)
├── tactical.rs       — The tactical battle engine: BattleSession, placement, engagement AI, weapons, fighters, Death Star, stalemate; played, simulated or headless (FUN_0040a050)
├── tactical/         — apply.rs (write-back: apply_results, squadron_homes, seat_morale), resource_ids.rs (TACTICAL.DLL resource joins), test_support.rs and the tests (tests.rs, station_tests.rs, hangar_tests.rs, repair_tests.rs, withdrawal_tests.rs)
├── bombardment.rs    — The Planetary Bombardment orders `0x220`-`0x223`: gate (FUN_00582440), per-hit run (FUN_0058d800), character roll and finalize (FUN_0058e320); the integrator resolves each order (ghidra/notes/bombardment.md)
├── operation_report.rs — The Operation Report's six object lists, picture and fill walks (message kind `0xf`, FUN_00496770, FUN_00496f50; ghidra/notes/operation-report-window.md)
├── support_shift.rs  — Sector support shifts (event `0x373`): control change with the troop bits and a stalemate's earlier holder, Coruscant, blockade end, HQ removal, bombardment and kill; the support setter's hook, which reruns control at once (`support_changed`, `FUN_005109f0`), and the queue that serves both (`fire`) (ghidra/notes/support-shifts.md)
├── blockade.rs       — Fleet-presence blockade, manufacturing halt, troop destruction, the blockader's support shift on timer `0x386` (ghidra/notes/support-shifts.md)
├── uprising.rs       — Recovered revolt lifecycle, uprising incident (UPRIS1TB/UPRIS2TB codes), Subdue gain, disaster
├── death_star.rs     — Sector bit 11, the destroy cascade (`cleanup_destroyed_system`: fleets survive, the system's own units die, characters go adrift, uncompleted units autoscrap), the same autoscrap walk for a holder change (`serve_holder_requests`, ghidra/notes/facility-ownership.md) and the adrift sweep (ghidra/notes/death-star-strategic.md).
├── research.rs       — per-side points pools, passive timer `0x381`, spending into levels (systems/research.md)
├── jedi.rs           — 4-tier Force progression (None→Aware→Training→Experienced), detection
├── victory.rs        — current HQ/Death Star outcome checks; parity divergences documented
├── betrayal.rs       — Loyalty-driven faction defection, UPRIS1TB threshold, immunity flag
├── story_events.rs   — 4 scripted story chains (Dagobah, Final Battle, Bounty Hunters, Jabba), 5-case palace outcomes, 5-stage carbonite countdown, telemetry twins (0x200, 0x231), CharactersCoLocated condition.
├── commands.rs       — Shared command registry (CommandDef entries) for GUI palette + CLI
├── game_events.rs    — GameEventRecord struct + the EVT_ event type constants for JSONL telemetry
├── effects.rs        — GameEffect enum, EffectPhase ordering, monoidal composition, inversion
├── economy.rs        — Full 18-function economy tick (FUN_00508250, the community dump calls it adjust_and_deploy_each_system): resource caps, support drift (system `+0x6c`, the output diversion roll's odds; never added to support), collection rate, KDY modifier, the holder rule `system_control` (regiments, else support; FUN_0050b610, run by `support_shift`'s hooks), garrison, troop/fleet summary (SystemSummary), incident state (IncidentFlags), uprising visibility. 17 GNPRTB indices.
├── repair.rs         — Ship repair, timer `0x380`: the damage counters, a repair every 30 days, or every 2 at the side's own working yard (ghidra/notes/ship-damage-repair.md)
├── post_battle.rs    — The pass after a battle: the loser's fleets and squadrons withdraw or are lost (FUN_0051ddf0, FUN_00515480)
├── scrap.rs          — Scrapping for half the refined cost, and the overdraft's candidates (FUN_00530270, FUN_0052e530)
├── stockpiles.rs     — Each side's raw and refined materials and the facility cycles that fill and draw them; tick step 0b (ghidra/notes/top-bar-resource-counters.md)
├── resources.rs      — A side's raw materials, refined materials and maintenance, the top strip's three numbers
├── battle_summary.rs — The Battle Summary window's contents (window kind 8, FUN_0046f140; ghidra/notes/tactical-results.md)
├── build_selection.rs — What a production manager can build and what an order costs (ghidra/notes/manufacturing-build-selection.md)
├── carriage.rs       — Which capital ship carries a fleet's squadrons, regiments and characters
├── officer_rank.rs   — Officer rank through the Command submenu (orders 0x260..0x263; ghidra/notes/character-roster.md)
├── fleet_name_bank.rs — Canonical fleet names, a port extension (the original only numbers fleets; ghidra/notes/fleet-names.md)
├── agent_automation.rs — Player-side Agent automation: Manage Garrisons and Manage Production
├── saved_interface.rs — The interface a save carries: the GID display mode and the open child windows
├── net_protocol.rs   — Multiplayer protocol message definitions (NetMessage)
├── serde_ordered.rs  — Deterministic human-readable serde adapters for unordered collections
└── trace.rs          — Opt-in decision traces: the inputs a rule read and the choice it made, for telemetry
```

## Render Modules (rebellion-render)

```
crates/rebellion-render/src/
├── lib.rs              — Galaxy map (click and hover; no pan or zoom, FUN_00422ce0) and its GID overlays
├── game_menu.rs        — The original Game Menu Window (FUN_00442860), shared by the speed menu and the object pop-up menu
├── object_menu.rs      — The right-click pop-up menu for a system window's characters, special forces, fleets and regiments, and the Fleet window's capital ships (FUN_004ac5c0)
├── targeting.rs        — The galaxy view's targeting mode after Mission: pointer capture and REBEXE cursor 1002
├── mission_dialog.rs   — The original mission dialog (FUN_0046a750), opened by targeting
├── system_window.rs / sector_window.rs — The original modeless system and sector windows
├── quadrant_icons.rs   — The four icons a sector window draws around each planet (FUN_00459e30): which rule shows each, and its art
├── control_registry.rs — Named hit rects the developer commands click by name in a dev run
├── fleet_window.rs     — The Fleet window (type 4, FUN_004a2630): a system's fleets, their contents under four tabs, loading, unloading, joining and splitting
├── defenses_window.rs  — The System Defenses window (type 10, FUN_004a7790)
├── missions_window.rs  — The Missions window (type 11, FUN_0049f130)
├── operation_report.rs — The Operation Report window (type 0x1c, FUN_0044c410): Summary, the fleet side's and the system's forces under six tabs, Display
├── fleet_finder.rs     — The Fleet and Ship Finder (type 0x15, FUN_00461750), opened by the cockpit control or F3
├── move_confirmation.rs — The Confirmed Move window (FUN_0044f060)
├── game_speed.rs       — The cockpit's day readout and its speed menu (FUN_00422ce0)
├── main_menu.rs        — Title screen with New Game / Load Game / Quit
├── video_player.rs     — Movie playback: native from decoded PNG frame sequences + WAV sidecars; in the browser through the shell's `<video>` overlay (`or_movie_play`, `or_movie_stop`, `or_movie_ended`)
├── theme.rs            — Star Wars egui theme: dark space bg, gold/amber accents, Liberation Sans font
├── message_log.rs      — Scrollable egui event feed, 7 color-coded categories
├── fleet_movement.rs   — Diamond fleet icons, dashed route lines, ETA labels, fleet hover detection
├── fog.rs              — Dim overlays for unexplored/unseen systems
├── audio.rs            — AudioVolumeState, SfxKind, MdataTrack, MusicSelector, draw_audio_controls (egui widget); track choice is the port's, not the original selector (ghidra/notes/audio-dispatch.md)
├── encyclopedia.rs     — the original Galactic Encyclopedia index and topic window over the DAT catalog
├── bmp_cache.rs        — DllSource enum, BMP/PNG texture cache with HD fallback, WASM path rebasing, named DLL resource ID catalog
├── cockpit.rs          — Faction cockpit chrome (top/bottom bars), 9 control buttons, CockpitViewport
├── tactical_view.rs    — The battle window: drawing, input, HUD and panels over rebellion_core::tactical (re-exported), and the Battle Summary window over core::battle_summary
├── advisor.rs          — Droid advisors: the advice agent's slot table, both droids' players, voices, cockpit steps and the ending slot (`ghidra/notes/droid-advisor-triggers.md`)
├── advisor_script.rs   — Action records → droid commands (play, hold, turn), the agent's pass, skip and chatter
├── briefing_tour.rs    — Opening briefing tour: each cockpit step's galaxy display or highlight, per faction
├── advice_topics.rs    — The agent's advice topics: TEXTSTRA's topic table (ghidra/notes/droid-advisor-triggers.md)
├── agent_menu.rs       — The Agent menu (STRATEGY RT_RCDATA 0xdead, items 0x110..0x11e; ghidra/notes/agent-menu.md)
├── build_selection.rs  — The original Build Selection window (FUN_00437df0)
├── manufacturing_window.rs — Window type 9, Manufacturing and Production: the overview's three producer bands and five facility pages
├── game_options.rs     — The original bitmap-driven Game Options window, Save and Load
├── status_window.rs / status_rows.rs — The Status window (type 0x1a, FUN_00442d70) and each object family's rows
├── personnel_finder.rs / troop_finder.rs — The Personnel Finder (type 0x17) and Troop Finder (type 0x16), FUN_0042a4d0
├── fleet_registry.rs   — The Fleet Registry, a port extension: how a new game names fleets
├── gid_legend.rs       — The galaxy display's detailed legend (ghidra/notes/gid-legend.md)
├── message_index.rs    — The original Message Index bitmap shell and row interaction
├── main_menu_destinations.rs — The screen the cockpit's Multiplayer control reaches
├── scroll_bar.rs       — The original scroll bar (FUN_0060aa70)
├── sector_hover.rs     — The galaxy view's sector name under the pointer
├── tooltip.rs          — The original's help balloons over command-center controls
├── window_focus.rs     — One focus across the galaxy view's child windows
├── windowing.rs / windowing_input.rs / windowing_policy.rs / windowing_adapter_probe.rs — Strategic windows over rebellion-windowing: snapshots to egui, input admission, policy, a test-only adapter probe
├── encyclopedia_surface.rs / encyclopedia_textures.rs — The Encyclopedia's surface data and interactions, and its selected-topic textures
├── tactical_assets.rs / tactical_asset_cache.rs — Decoding and rendering the original tactical type-301/303 assets, and their decoded store
└── panels/
    ├── mod.rs           — PanelAction enum: panel, mission, save, and combat actions
    ├── game_setup.rs    — Galaxy size, difficulty, faction selection
    ├── fleets.rs         — Fleet roster: composition and officers (characters board by Move)
    ├── missions.rs       — Active missions with progress and cancel; missions start from the object pop-up menu
    ├── mod_manager.rs    — Mod Manager: discover, enable/disable, reload, dependency display
    └── command_palette.rs — developer command palette: simulation and per-system interface commands (debug or native builds)
```

## App Modules (rebellion-app)

```
crates/rebellion-app/src/
├── main.rs   — Entry point, interactive loop, panel action handling (no headless tests, see agent-tooling.md)
├── audio.rs  — quad-snd AudioEngine: load, play_sfx, play_music, volume sync, WASM audio base-path resolution
├── dev_policy.rs — What a developer-command wait does when a modal screen stops the clock
├── movies.rs — Movie ids to MDATA files (`FUN_00421ba0`), the application states' movies (intro `0x65` then `0x66`, the side's start movie `0x68`/`0x69`, Credits `0x6e`) and where each movie's end leads (`FUN_004138a0`), the kind-6 message movies and the ending: the clock stop, the code by winner and viewer, its movie (systems/victory.md)
├── strike_orders.rs — The object menu's Planetary Bombardment orders (`0x220`-`0x223`) through `integrator::order_strike`, the messages a strike files, and the win word's movie messages
├── assault_orders.rs — The Planetary Assault order (`0x234`) the object menu posts and the messages its run files (ghidra/notes/ground-assault.md)
├── dev_actions.rs / dev_channel.rs / dev_input.rs — The world-changing developer commands, the live developer channel, and scripted pointer and key input
├── encyclopedia_content.rs / encyclopedia_hd.rs / encyclopedia_mods.rs / encyclopedia_surface.rs — Encyclopedia byte readers, artwork selection, mod overlays, and the adapter to render DTOs
├── strategic_windowing.rs — The application's strategic window coordinator and commit boundary
├── dev_commands.rs — The command palette's gate (`OPEN_REBELLION_DEV`), and behind it the native command script (`OPEN_REBELLION_COMMANDS`) and campaign seed (`OPEN_REBELLION_SEED`)
├── interface_test_fixture.rs — Test-only interface fixture scenarios and the observations the browser gates in tools/interface-parity read
├── tactical_flow.rs / tactical_test_fixture.rs — The production tactical-battle entry and its test-only direct entry
├── runtime_pack.rs — Parser for the browser runtime asset pack
└── web_accessibility.rs / web_replay.rs — The browser accessibility bridge and the native/WASM replay runner
```

Decoded cutscene assets are intentionally kept out of git. `scripts/decode-cutscenes.sh` expands `assets/references/ref-videos/*.webm` into `assets/references/cutscene-frames/<name>/frame-*.png`, `metadata.json`, and sibling `<name>.wav` files for the native `VideoPlayer`.

The browser plays movies outside the canvas. `VideoPlayer::open` passes the
movie's MDATA number (the path's stem, `video_player::movie_number`), a play
number, the volume and a rectangle to the `or_movie_play` import; the shell's
`open_rebellion_movies` plugin (`web/index.html`) shows
`data/movies/<number>.webm` in a `<video>` over that rectangle, given as
fractions of the screen (`movies::video_bounds`: a message movie's picture,
which leaves its caption to the canvas, else the whole window), and calls the `or_movie_ended` export with the play number when the
movie ends, fails to load, or is skipped (Esc, the left button, or any other
key except in the trench-run movies 201 and 202, as `FUN_00421790` and
`FUN_005f7240` close the movie window). `stop` and dropping the player take
the movie down through `or_movie_stop`. `scripts/stage-web-movies.sh`
converts the original Smacker `MDATA.<number>` files to WebM (VP9 + Opus)
under the ignored `web/data/movies/`; `scripts/package-web.sh` runs it and
ships the files beside `runtime.orpk`. The stage script exits 1 unless it
staged all fifteen; a `dev` package and the Docker build accept a partial
set (the browser skips a missing movie), and any other version fails unless
`stage-web-movies.sh --verify` finds every movie in the package.

The canvas is a WebGL context made with `{ antialias: false, alpha: false }`
(`web/gl.js`, `context_attributes`): the original draws through DirectDraw
and Direct3D Retained Mode's software rasterizer, neither of which
antialiases, onto an opaque primary surface. With the default alpha a
blended glyph edge leaves the framebuffer's alpha below 1 and the page
shows through.

## Data Modules (rebellion-data)

```
crates/rebellion-data/src/
├── seeds.rs      — Game seeding: 3-system model, character stat rolling, named placement
├── save.rs       — Save/load: bincode snapshots and save slots; only the current SAVE_VERSION loads, with no migrations (save-load.md). Native uses filesystem saves; WASM uses browser localStorage through the gl.js loader's imports, with separate metadata keys for fast slot listing.
├── mods.rs       — Mod loader + ModRuntime: TOML manifest, RFC 7396 merge patch, semver, hot reload
├── simulation.rs — Tick orchestrator: SimulationStates bundle + run_simulation_tick()
├── telemetry.rs  — Telemetry payloads shared by the integrator and the native game, trace records, the native JSONL sink
├── readouts.rs   — Plain-text state readouts for the developer commands and the playtest REPL
├── replay.rs / replay_fixture.rs — Versioned, platform-neutral replay artifacts, and the shared fixture for native and browser replay equivalence (deterministic-replay.md)
├── text_templates.rs — TEXTSTRA.DLL's RT_RCDATA message templates (FUN_0060b9d0) and advice topic tables
├── encyclopedia_{catalog,topics,session,presenter,overlay}.rs — The Galactic Encyclopedia: catalog loading, topic bindings, immutable sessions, presentation, overlays
├── bin/          — replay-gate (the native leg of the replay equivalence gate, deterministic-replay.md) and encyclopedia-source-audit
└── integrator.rs — PerceptionIntegrator: all world mutation + telemetry emission. Betrayal emits reveal-before-flip (EVT_TRAITOR_REVEALED) + side-change-after-flip (EVT_SIDE_CHANGE).
```

## Type System: Two Layers

### Layer 1: Binary mirror (`rebellion-core/src/dat/`)
Structs that exactly match .DAT file field layout. Used only for import/export. No game logic.
- `Faction` (Alliance, Empire, Neutral)
- `GalaxySize` (Standard=1, Large=2, Huge=3)
- `SectorGroup` (Core=1, RimInner=2, RimOuter=3)
- `ExplorationStatus` (core systems, family `0x90`, start Explored; rim systems, `0x92`, Unexplored)

### Layer 2: Runtime world (`rebellion-core/src/world/`)
Rich types used by game logic, rendering, save/load. Slotmap keys for all inter-entity references.
- `GameWorld` -- root aggregate, 11 SlotMap arenas (systems, sectors, capital_ship_classes, fighter_classes, characters, fleets, troops, special_forces, defense_facilities, manufacturing_facilities, production_facilities) + `GnprtbParams` + `mission_tables: HashMap<String, MstbTable>`
- `System` -- position, sector ref, popularity (alliance/empire f32), asset lists (fleets, units, facilities), `espionage_rating: f32` (reduces incite uprising probability)
- `Sector` -- named region, SectorGroup, position, child system list
- `CapitalShipClass` / `FighterClass` -- class templates, not instances
- `Character` -- 8 `SkillPair` (base+variance), Jedi fields, role flags, major/minor
- `Fleet` -- `capital_ships: Vec<ShipInstance>` (per-hull records: class, hull_current, damage_counters, alive, completed, name, tag, repair_timer, transit) + characters at a system location. Helper methods: `ship_count()`, `ship_counts_by_class()`, `is_empty()`, `has_death_star(classes)` (a live ship of family 0x18-0x1b, `FUN_005003d0`).
- `ManufacturingFacilityInstance` -- runtime facility record with `is_shipyard: bool` (used by economy to detect shipyard systems)
- `ProductionFacilityInstance` -- runtime facility record with `is_mine: bool` (used by economy to count raw material sources)

## Entity Identity (Dual-Key Pattern)

Every entity carries both:

1. **`DatId(u32)`** -- the id the DAT record stores. The original's compound ids are `(family_byte << 24) | index`. Loaded systems and characters keep their file's id with no family byte, so a match against a compound id goes through `DatId::is_record`. A system's SYSTEMSD family (`0x90` core, `0x92` rim) is kept in `GameWorld::system_families`. Preserved for serialization and mod cross-references.

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

`run_simulation_tick` runs the systems in the order `simulation.md`,
"Integration Order", lists; the native loop in `main.rs` still repeats that
order by hand (F-012). Economy precedes manufacturing.

### Save/Load Flow
```
Game Options window (game_options.rs), Save and Load on rows 1-6
    | rebellion-data::save::{save_slot, load_slot, list_saves}
    v
SaveState + SaveMeta
    | native: filesystem `<saves_dir>/<slot>.reb`
    | wasm32: localStorage through the gl.js loader's imports
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
1. `fleet.location = arrival.system`: update the fleet's position
2. `origin_system.fleets.retain(|k| k != fleet)`: remove from origin
3. `dest_system.fleets.push(fleet)`: add to destination
All systems that query `System.fleets` (combat, fog, blockade, victory) see correct positions.

See `agent_docs/simulation.md` for full API reference on the advance() pattern.

### Loading Order (rebellion-data/src/lib.rs)
1. Sectors (SECTORSD.DAT) -- must come first, systems reference sectors
2. Systems (SYSTEMSD.DAT) -- registers each system in its parent sector
3. Capital ships (CAPSHPSD.DAT)
4. Fighters (FIGHTSD.DAT)
5. Major characters (MJCHARSD.DAT)
6. Minor characters (MNCHARSD.DAT)
7. GNPRTB, SDPRTB
8. SPECFCSD, TROOPSD, DEFFACSD, ALLFACSD, MANFACSD, PROFACSD
9. The *MSTB tables and MISSNSD

### Round-Trip Validation
Round-trip validation is enforced inside `parse_and_dump` in `tools/dat-dumper/src/registry.rs`. The byte comparison utility is in `tools/dat-dumper/src/validate.rs`. If reserialized bytes differ at any offset, the parse is wrong. This guarantees complete format understanding.

## Rendering Architecture

`rebellion-render/src/lib.rs` exposes composable functions called each frame:
1. `draw_galaxy_map(world, state, cache, gid_mode, faction, gid) -> CameraView` -- the fixed starfield transform (`galaxy_camera`) and the system and GID overlays
2. `draw_fog_overlay(world, fog, cam)` -- dim non-visible systems
3. `draw_fleet_overlays(world, movement, cam)` -- fleet icons and route lines
4. `hovered_fleet(world, movement, cam, mx, my) -> Option<FleetKey>` -- fleet hit detection
5. `object_menu::draw_object_menu(ctx, menu, ...)` -- the original right-click pop-up menu for a system window's objects
6. `targeting::{capture_pointer, draw_targeting_cursor}` -- the galaxy view's targeting mode after the menu's Mission
7. `draw_status_bar(ctx, world, clock, audio_vol)` -- bottom bar with speed/audio controls
8. `draw_message_log(ctx, log, state)` -- scrollable event feed
9. Panel functions that only debug-build letter keys open: `draw_fleets`, `draw_missions`
10. `draw_encyclopedia_surface(ctx, cache, origin, scale, state, surface)` -- the original Galactic Encyclopedia index and topic window
11. `draw_main_menu(ctx) -> MainMenuAction` -- title screen
12. `draw_game_setup(ctx, state) -> GameSetupAction` -- new game config

`GalaxyMapState` holds the selected, hovered and activated system, the pointer block, the targeting flag, the sector-label and grid toggles, the viewport and the display scale.

Input: mouse within map area only. The view neither pans nor zooms (FUN_00422ce0). Left-click selects nearest system within hover radius, except while targeting, when the release targets it. A right-click on the map opens nothing; what the original's galaxy view does on a right-click is not traced yet. A system window's Personnel and Troops lists open the object pop-up menu.

## Adding a New Entity Type

1. Add slotmap key to `crates/rebellion-core/src/ids.rs` via `new_key_type!`
2. Add dat/ enum or struct if needed (`crates/rebellion-core/src/dat/mod.rs`)
3. Add world struct to `crates/rebellion-core/src/world/mod.rs`, add SlotMap arena to `GameWorld`
4. Add dat-dumper parser in `tools/dat-dumper/src/types/`, add module to `types/mod.rs`, register in `registry.rs`
5. Add conversion step in `crates/rebellion-data/src/lib.rs`
6. Update rendering if the entity is visible on the map
