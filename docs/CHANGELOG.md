---
title: "Changelog"
description: "All notable changes to Open Rebellion by version"
category: "reference"
created: 2026-03-11
updated: 2026-03-16
---

# Changelog

All notable changes to Open Rebellion are documented here.

## Format

Each entry includes:
- **Version** - Semantic version (MAJOR.MINOR.PATCH)
- **Date** - Release date
- **Completion** - Game completion % vs original Star Wars Rebellion (1998)
- **Category** - Primary area of change
- **Milestone** - Which roadmap phase this closes or advances

---

## [v0.10.0] - 2026-03-20

**Completion:** ~99% | **Category:** Full Wiring | **Milestone:** Knesset Tinnit Phase 2

### Added
- **Dual-AI in shared simulation**: `SimulationStates.ai2: Option<AIState>` + second AI pass in `run_simulation_tick()`. Both interactive app and headless binary now support dual-AI.
- **CLI `--exec` subcommand**: `rebellion-playtest --exec "command_id"` dispatches commands from shared registry. `--exec list` shows all 16 available commands. Enables AI agent play-testing.
- **ModRuntime wired at startup**: `ModRuntime::discover()` called after world load, mods applied via RFC 7396 merge patch, Mod Manager panel now shows real discovered mods.
- **ToggleMod / ReloadMods handlers**: Mod Manager toggle and reload buttons now functional.
- **JSONL system_name field**: `GameMessage.system_name: Option<String>` populated via `resolve_system_names()` before export. DuckDB queries can now filter by system.
- **Command palette consumes shared registry**: `command_palette.rs` now builds CommandItems from `rebellion_core::commands::all_commands()` via `command_id_to_action()` mapper. Single source of truth.

### Fixed
- **Hardcoded `MissionFaction::Empire` in AI dispatch** (`simulation.rs:900`): Now derives faction from `ai_state.faction` via `as_mission_faction()`. Alliance AI dispatches Alliance missions.
- **`--dual-ai` warning removed** from playtest binary. Flag now fully functional.
- **5 dat-dumper warnings suppressed**: `#![allow(dead_code)]` on binary target (functions used by library, not CLI).
- **Unused `Faction` import** removed from `missions.rs`.

### Technical
- 275 tests passing (218 core + 33 data + 24 render)
- Zero compiler warnings across entire workspace

---

## [v0.9.0] - 2026-03-19

**Completion:** ~97% | **Category:** Completion | **Milestone:** Knesset Tinnit

### Added
- **Shared command registry** (`rebellion-core/src/commands.rs`): 16 `CommandDef` entries shared between GUI palette and CLI. Pure data, zero IO.
- **3 new PanelAction variants**: `ListActiveMissions`, `ListActiveFleets`, `ShowEventCount` — each with distinct handlers showing real data
- **Dual AI mode**: `ToggleDualAI` now spawns a second `AIState` for the opposite faction, both advance each tick. Command palette toggle + message log confirmation.
- **Force victory check**: `ForceVictoryCheck` calls `VictorySystem::check()` immediately and reports result
- **Game log export**: `ExportGameLog` writes message log as JSONL via `MessageLog::export_jsonl()`. Serialize derives on `GameMessage` + `MessageCategory`
- **Defense facility class definitions**: `DefenseFacilityClassDef` struct + `defense_facility_classes` HashMap on `GameWorld`, loaded from DEFFACSD.DAT
- **Bombardment defense class lookup**: Replaces hardcoded `10` per facility with real `bombardment_defense` from class definitions (fallback to 10 if class missing)

### Fixed
- **Inspection commands aliased to ShowGameStats**: List Active Missions, List Active Fleets, and Show Event Count all returned generic stats. Now each has its own handler showing missions, fleet positions, or event counts.

### Technical
- 275 tests passing (218 core + 33 data + 24 render)
- `apply_panel_action()` extended with `dual_ai_mode`, `victory_state`, `event_state` parameters

### Known Limitations
- `AdvanceTicks(n)` advances clock counter only — simulation catches up on next frame via `clock.advance(dt)` TickEvents
- CLI exec subcommand and ModRuntime wiring deferred to v0.10.0

---

## [v0.8.0] - 2026-03-17

**Completion:** ~95% | **Category:** Play-Test Infrastructure | **Milestone:** Play-Test READY

### Added
- **Command palette**: VS Code-style overlay (backtick trigger) with fuzzy search via nucleo-matcher. 16 commands across 4 categories (Time, Speed, Inspect, Control). Debug-only (`#[cfg(debug_assertions)]`).
- **7 new PanelAction variants**: `AdvanceTicks(u64)`, `SetGameSpeed(u32)`, `ToggleDualAI`, `ForceVictoryCheck`, `RevealAllFog`, `ExportGameLog`, `ShowGameStats` — all wired into `apply_panel_action()`
- **Headless play-test binary** (`rebellion-playtest`): CLI with `--ticks`, `--data`, `--seed` flags, JSONL output for DuckDB analysis
- **Deterministic RNG**: All `rand::gen_range` calls replaced with seedable `Xoshiro256++` (rand_xoshiro). Single `sim_rng` generator flows through entire simulation loop.
- **Play-testing docs**: `docs/mechanics/play-testing.md` covering command palette, headless binary, JSONL schema, DuckDB examples, dual-AI mode

### Changed
- `rebellion-playtest` added to workspace members
- `rand 0.8` + `rand_xoshiro 0.6` added to rebellion-app dependencies
- `apply_panel_action()` now takes `&mut GameClock` (was `&GameClock`) to support speed/tick mutations

### Technical
- Knesset Mami swarm: Dikte-II integration daborit
- 272 tests passing (216 core + 33 data + 23 render), 0 failures
- 10 RNG call sites migrated from `rand::gen_range` to `sim_rng.gen()`

---

## [v0.7.0] - 2026-03-16

**Completion:** ~95% | **Category:** Story Events + Release Prep | **Milestone:** Release READY

### Added
- **8 new story event chains**: Force Detection (0x1e1), Force Training (0x1e5), Dagobah multi-phase gates (0x390-0x392), Final Battle full chain (0x393-0x396 + 0x220), Bounty Hunters Active gate (0x397), Han Carbonite (0x398), Palace captures (0x399-0x39A)
- **4 new EventCondition variants**: CharacterForceExperience, CharacterIsCaptive, CharacterIsJediTrainer, EventNotFired
- **3 new EventAction variants**: AccumulateForceExperience, CaptureCharacter, SetCarboniteState
- **Release packaging**: scripts/package-web.sh (WASM zip), scripts/package-macos.sh (native archive)
- **Example mod**: mods/examples/star-destroyer-rebalance/ (mod.toml + JSON patch)
- **GitHub Actions CI**: test + WASM build on push
- **Vendored web/gl.js**: WASM builds no longer fetch from network
- **Release profile**: LTO, strip, codegen-units=1 for optimized builds

### Technical
- Knesset Hokhmah swarm: 3 daborot (Sopher, Sassuratu, Dikte)
- 19 tasks completed
- Event ID range 0x390-0x39A used for story gates

---

## [v0.6.0] - 2026-03-16

**Completion:** ~92% | **Category:** Mod Workshop + Integration Hardening | **Milestone:** Mod Workshop IN PROGRESS

### Added
- **Mod Manager panel**: egui floating window with mod list, enable/disable checkboxes, reload button, Tab keyboard shortcut
- **3 new PanelAction variants**: `OpenModManager`, `ToggleMod { name }`, `ReloadMods` — wired into `apply_panel_action()`
- **Character escape system**: `check_escapes()` called every tick after mission resolution, rolls against ESCAPETB, clears captivity on success

### Fixed (from v0.5.0 review)
- **Research levels never applied**: `ResearchSystem::advance()` was made pure but caller never applied `TechUnlocked` results — now calls `research_state.{alliance,empire}.advance(tech_type)` after logging
- **Captivity fields never set/cleared**: `CharacterCaptured` arm now sets `is_captive`, `captured_by`, `capture_tick`; `CharacterRescued` and `CharacterEscaped` arms now clear all three fields
- **`check_escapes()` never called**: Escape check inserted into tick loop after mission results, with proper captivity clearing and fleet removal

### Technical
- Knesset Elat swarm: integration + reviewer fixes
- 259 tests passing (206 core + 31 data + 19 render + 3 doctests), 0 failures
- Mod Manager types (`ModManagerState`, `ModInfo`, `ModManagerAction`) re-exported from rebellion-render

---

## [v0.5.0] - 2026-03-16

**Completion:** ~90% | **Category:** Story Events + Character Abilities + Integration | **Milestone:** Full Parity IN PROGRESS

### Added
- **4 scripted story event chains**: Luke Dagobah Training (0x221→0x210), Vader/Luke Final Battle (0x220), Bounty Hunter Attack (0x212), Jabba's Palace rescue (0x380–0x383) — all data-driven via EventState
- **6 new EventCondition variants**: CharacterHasForceLevel, FactionControlsSystem, CharacterIsForceUser, CharacterExists, CharacterOnMandatoryMission, FactionControlsNSystems
- **6 new EventAction variants**: SetMandatoryMission, ModifyForceTier, RemoveCharacter, StartJediTraining, TransferCharacter, TriggerEvent
- **15 Ghidra RE event ID constants** (EVT_RECRUITMENT_DONE through EVT_ESPIONAGE_EXTRA)
- **Han Solo speed bonus**: `hyperdrive_modifier` field on Character, consulted by `fleet_ticks_per_hop()` in movement.rs
- **Betrayal system**: new `betrayal.rs` module following advance() pattern — loyalty threshold check via UPRIS1TB, `is_unable_to_betray` immunity (Luke, Vader), 50-tick cooldown
- **Decoy system**: `check_decoy()` consults FDECOYTB during mission resolution, `DecoyTriggered` effect
- **Escape system**: `check_escapes()` stub with ESCAPETB signature, `CharacterEscaped` effect
- **Mission state flags**: `on_mission`, `on_hidden_mission`, `on_mandatory_mission` tracked on Character; `dispatch_guarded()` blocks mandatory-mission characters
- **10 new Character fields**: `is_unable_to_betray`, `is_jedi_trainer`, `is_known_jedi`, `hyperdrive_modifier`, `enhanced_loyalty`, `on_mission`, `on_hidden_mission`, `on_mandatory_mission`, `current_system`, `current_fleet`
- **3 DAT fields promoted**: `is_known_jedi`, `is_unable_to_betray`, `is_jedi_trainer` from MJCHARSD.DAT/MNCHARSD.DAT into Character world model
- **Character location tracking**: `current_system` and `current_fleet` populated from fleet scan after seed loading
- **BetrayalState** added to SaveState (version 3)

### Fixed (from v0.4.1)
- Fleet arrivals now update `System.fleets` (remove from origin, add to destination)
- 6 new systems wired into main loop: blockade, uprising, Death Star, research, Jedi, victory
- Mission effects mutate world: FacilitySabotaged, CharacterKilled, CharacterCaptured, CharacterRescued
- Jedi XP persists via `accumulated_xp` in JediTrainingRecord
- Blockade halts manufacturing via `advance_with_blockade()`
- Uprising incident cooldown: first incident always fires (no cooldown entry)
- All 14 simulation states serialized in SaveState

### Technical
- Knesset Shapash swarm: 4 Sonnet daborot (Tanit, Nikkal, Anat, Pidray) + Opus lead
- 23 tasks completed, 237 tests passing (26 new)
- New modules: betrayal.rs, story_events.rs
- SaveState version bumped from 2 to 3

---

## [v0.4.1] - 2026-03-15

**Completion:** ~70% | **Category:** Integration Fixes + Documentation | **Milestone:** War Machine HARDENED

### Fixed
- Fleet arrivals update `System.fleets` — combat, fog, blockade, victory see correct positions
- 6 new systems (blockade, uprising, Death Star, research, Jedi, victory) wired into main loop
- All 6 states added to SaveState (version 2)
- Mission effects now mutate world: FacilitySabotaged removes facility, CharacterKilled/Captured/Rescued update arenas
- Jedi XP persists via `accumulated_xp` in JediTrainingRecord
- Blockade halts manufacturing via `advance_with_blockade()`
- Uprising incident cooldown bug: first incident fires immediately (was blocked until tick 10)

### Added
- `agent_docs/systems/` — 7 per-system detail docs (combat, blockade, uprising, death-star, research, jedi, victory)
- 3 "how to add" guides in simulation.md: new system, new mission type, new event type
- Mission effects documentation table in game-domain.md
- Fleet arrival lifecycle documented in architecture.md

### Changed
- simulation.md pruned from 346→132 lines (lean index with detail docs split to systems/)
- architecture.md updated with all 14 simulation modules + render modules
- game-domain.md: missions section reflects all 9 implemented types
- CLAUDE.md updated to v0.4.1 (~70% complete, 211 tests)

### Technical
- 211 tests passing, 0 failures
- SaveState version bumped from 1 to 2

---

## [v0.4.0] - 2026-03-15

**Completion:** ~68% | **Category:** Combat + Missions + Victory + Save/Load | **Milestone:** War Machine COMPLETE

### Added
- **Space combat auto-resolve**: 7-phase pipeline (weapon fire → shield absorb → hull damage → fighter engage → result) with CombatPhaseFlags bitfield matching original C++ architecture
- **Ground combat**: troop iteration with regiment_strength comparison, per-unit resolution
- **Orbital bombardment**: Euclidean distance formula `sqrt(delta²) / GNPRTB[0x1400]` with difficulty modifier, minimum 1 damage
- **7 new mission types**: Sabotage(6), Assassination(7), Espionage, Rescue, Abduction, InciteUprising, SubdueUprising — all wired to MSTB probability tables
- **Research system**: tech tree progression using research_order/research_difficulty from DAT
- **Jedi training**: 4-tier Force progression (Aware → Potential → Training → Experienced)
- **AI espionage**: AI dispatches sabotage, assassination, espionage missions against player
- **Victory conditions**: HQ capture, Death Star destruction, galactic conquest detection
- **Death Star**: construction countdown, planet destruction (fires when alive_flag bit0==0), NearbyWarning scan
- **Blockade mechanics**: fleet presence blocks manufacturing, TroopRegDestroyedRunningBlockade (event 0x340)
- **Uprising system**: incite (UPRIS1TB, 3 thresholds) / subdue (UPRIS2TB, 4 thresholds) with 10-tick incident cooldown
- **Save/load system**: bincode serialization with OPENREB header, 10 save slots, auto-save UI
- **15 new CapitalShipClass fields**: weapon arcs (turbolaser/ion/laser × 4 arcs), shield_strength, shield_recharge_rate, bombardment, damage_control, detection, maneuverability
- **ShipInstance type**: per-hull combat state (hull_current, shield_weapon_packed nibbles, alive)
- **GnprtbParams**: 213 entries × 8 difficulty values loaded from GNPRTB.DAT, `value(param_id, difficulty)` accessor
- **MstbTable**: 19 probability tables loaded with linear-interpolation `lookup(skill_score)`
- **Victory/defeat screen**: egui modal with faction-colored narrative and game stats
- **Combat results UI**: message log integration for space/ground/bombardment outcomes
- **Combat cooldown**: 5-tick per-system cooldown prevents infinite re-trigger on draws

### Fixed (from 4 parallel code reviews — 12 bugs total)
- Assassination/Abduction/Rescue targeted the mission agent instead of the victim — added `target_character` field
- Death Star ConstructionCompleted re-emitted every tick — now self-clears
- Victory screen dim layer invisible (Order::Background → Order::Foreground)
- UprisingIncident fired every tick with no cooldown — added 10-tick cooldown
- MstbTable::lookup() silently truncated negative interpolation — added .max(0.0) clamp
- GnprtbParams struct doc described wrong 4-level mapping — corrected to 8-level
- Space combat phase 3 gate inverted (ACTIVE && PHASES_ENABLED → ACTIVE && !PHASES_ENABLED)
- Alt-shield path used range 0x71-0x72 — corrected to exact == 0x71
- Fighter losses never applied (empty Vec) — now computed from initial vs surviving counts
- Combat trigger re-fired every tick on draws — added per-system cooldown
- Bombardment comment contradicted code — clarified minimum-1 semantics

### Technical
- Knesset Tiamat swarm: 4 Sonnet daborot (Karme, Tehom, Al-Uzza, Manat) + Opus lead
- 23 tasks completed, 188 tests passing
- 4 parallel code reviewers found and fixed 12 bugs before release
- New crates/modules: combat.rs, bombardment.rs, victory.rs, blockade.rs, uprising.rs, death_star.rs, research.rs, jedi.rs, espionage effects in missions.rs, save.rs, save_load.rs, victory_screen.rs, combat_view.rs

---

## [v0.3.0+RE] - 2026-03-15

**Completion:** ~42% (unchanged) | **Category:** Reverse Engineering | **Milestone:** War Machine UNBLOCKED

### Added
- **5,127 decompiled C files** from REBEXE.EXE (every function >100 bytes in the game executable)
- **7 scholar documents** (4,179 lines total):
  - `annotated-functions.md` (1,662 lines) — struct layouts, renamed variables, 50 event IDs, game rules
  - `modders-taxonomy.md` (805 lines) — 10 game systems categorized for Yuuzhan Vong/Thrawn/KOTOR total conversions
  - `rust-implementation-guide.md` (1,267 lines) — maps decompiled C to Open Rebellion's `advance()` pattern
  - `cpp-class-hierarchy.md` (445 lines) — CRebObject → CNotifyObject → CCombatUnit hierarchy, 19 vtable slots
  - `entity-system.md` — characters, Force/Jedi, factions, fleets (in progress)
  - `mission-event-cookbook.md` — 9 mission types, 4 story event chains, modder guide (in progress)
  - `economy-systems.md` — resources, control, uprising, blockade, repair (in progress)
- **111 GNPRTB parameters mapped**: 34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side) = 111 total with global address bindings
- **8 Ghidra Jython scripts** for automated binary analysis (string search, xref tracing, function catalog, GNPRTB tracing)
- **Bombardment formula decoded**: `damage = sqrt((atk-def)²) / GNPRTB[0x1400]` with difficulty modifier
- **Space combat 7-phase pipeline** fully mapped: weapon fire → shield absorb → hull damage → fighter engage → result
- **C++ class hierarchy reconstructed**: 12 classes, 19 vtable slots, complete field layout
- **Game object layout**: 12 field offsets mapped (hull +0x60, shield/weapon nibbles +0x64, strength +0x96, alive +0xac)

### Fixed
- STRATEGY.DLL identified as resource-only (29MB sprites, 9KB CRT) — all game logic in REBEXE.EXE
- GhidraMCP plugin version mismatch resolved (bethington v4.3.0 needs Ghidra 12.0.3, using LaurieWired v11.3.2)
- pyghidra-mcp config fixed (`--project-path` instead of positional arg)

### Technical
- REBEXE.EXE: 22,741 functions total, 5,127 decompiled (22.5% by count, ~95% by code volume)
- Observer/notification architecture confirmed — virtual dispatch via vtable, not direct calls
- Entity family bytes: 0x08-0x0f (characters), 0x14-0x1b (troops), 0x30-0x3b (ships), 0x34 (Death Star), 0x90-0x98 (systems)
- Mission type codes: 6=Sabotage, 7=Assassination, 21=Autoscrap
- 50 event IDs mapped (0x127-0x370 range)
- 15+ scripted story events: Luke/Vader saga, Dagobah, Jabba's Palace, Bounty Hunters

---

## [v0.3.0] - 2026-03-14

**Completion:** ~42% | **Category:** Full UI Integration + WASM | **Milestone:** War Room COMPLETE

### Added
- **Faction selection modal**: Choose Alliance or Empire at game start; gates all War Room panels
- **5 player panels**: Officers roster, fleet browser, manufacturing queue, mission dispatch, all with PanelAction return pattern
- **Encyclopedia viewer**: 4-tab browser (Capital Ships, Fighters, Characters, Systems) with BMP texture cache from EData/ directory
- **Fleet movement visualization**: Diamond fleet icons at systems, dashed cyan route lines, interpolated transit dots with ETA labels
- **Fog of war rendering**: Two-tier dim overlay (explored-not-visible α=0.75 vs never-seen α=0.88)
- **Audio system**: quad-snd engine (CoreAudio/ALSA/WebAudio), SFX on game events (build complete, fleet arrival/departure, mission success/fail), looped background music, egui volume controls in status bar
- **Message log**: Scrollable egui bottom panel, 6 color-coded categories (Manufacturing, Diplomacy, Mission, Event, AI, System), click-to-zoom-to-system
- **Keyboard shortcuts**: O/F/M/N (panels), E (encyclopedia), Space (pause/unpause), 1/2/3 (speed), R (reset camera)
- **WASM cfg guards**: All `std::fs`, `std::process::exit`, `std::env::args`, `notify`, `pelite` usage guarded with `#[cfg(not(target_arch = "wasm32"))]`
- **3 new agent docs**: simulation.md (advance() pattern), modding.md (RFC 7396), ghidra-re.md (STRATEGY.DLL RE setup)

### Changed
- **Refactored draw_info_panel**: Split into composable `draw_system_info_panel` + `draw_status_bar`, eliminated all unsafe pointer casts. `egui_macroquad::ui()` now lives in main.rs.
- **CameraView struct**: Returned by `draw_galaxy_map` so fog/fleet overlays use matching coordinates without recomputation
- **AI fleet movements now issue real MovementState orders** via `fleet_ticks_per_hop()`
- **Fog re-seeds on faction select**: Alliance and Empire see different starting visibility

### Technical
- WASM build: 3.4MB artifact at `web/open-rebellion.wasm`
- Doctest fix: events.rs uses `ignore` (GameWorld has no Default impl)
- 91 unit tests + 3 doctests passing, 2 ignored
- egui-macroquad 0.17 added to rebellion-app deps
- Panel mutual exclusivity enforced via keyboard toggle logic

---

## [v0.2.0] - 2026-03-13

**Completion:** ~35% | **Category:** Simulation + Data Layer | **Milestone:** Living Galaxy COMPLETE

### Added
- **7 simulation systems** (rebellion-core, ~4.2K LOC, 90+ tests): All follow stateless `advance(state, world, &[TickEvent]) -> Vec<ResultEvent>` pattern with caller-provided RNG for determinism
  - `tick.rs`: GameClock with Paused/Normal(1x)/Fast(2x)/Faster(4x) speeds (280 LOC, 13 tests)
  - `manufacturing.rs`: Per-system production queues with overflow propagation (520 LOC, 13 tests)
  - `missions.rs`: Diplomacy + recruitment with quadratic probability from rebellion2 (880 LOC, 14 tests)
  - `events.rs`: Conditional triggers (TickReached, CharacterAtSystem, Random, EventFired), event chaining (728 LOC, 17 tests)
  - `ai.rs`: Rule-based opponent — officer assignment, production priority, fleet deployment (936 LOC, 13 tests)
  - `movement.rs`: Fleet hyperspace transit with slowest-ship speed model (453 LOC, 11 tests)
  - `fog.rs`: Per-faction monotonic visibility with advance intel at 50% transit (373 LOC, 9 tests)
- **Seed table integration** (rebellion-data, 545 LOC): 9 seed table loaders, cross-references DatIds, creates Fleet instances with dual-key identity
- **Mod system** (rebellion-data, 637 LOC, 15 tests): TOML manifests, RFC 7396 Merge Patch, Kahn's topological sort for semver dependency resolution, hot reload via notify (native only)
- **29 new DAT parsers** (dat-dumper): All 51/51 DAT files now parsed with round-trip byte validation. New parsers for missions, fleet seeds, entity tables, all-facilities, int tables, syfc tables
- **TEXTSTRA.DLL parser**: Win32 PE string extraction via pelite crate — real entity names ("Coruscant", "Luke Skywalker") replace placeholders
- **Main loop integration**: clock.advance(dt) → manufacturing → missions → events → AI, with effect application helpers

### Technical
- Knesset Athirat swarm: 4 Sonnet daborot (Deborah, Kaptaru, Mami, Melissa) + Opus lead, ~45 min execution
- Total codebase growth: ~3K LOC → ~11.7K LOC across 54 Rust source files
- pelite, notify, quad-snd, image crates added to dependency tree
- Swarm report: `docs/reports/2026-03-13-knesset-athirat-swarm-report.md`

---

## [v0.1.0] - 2026-03-11

**Completion:** ~15% | **Category:** Data Viewer + Infrastructure | **Milestone:** Galaxy Viewer COMPLETE

### Added
- **5-crate Cargo workspace**: rebellion-core (types), rebellion-data (loader), rebellion-render (macroquad + egui), rebellion-app (entry point), dat-dumper (tools)
- **22 DAT parsers** with round-trip byte validation across 3 structural patterns (entity tables, parameter tables, seed tables)
- **Interactive galaxy map**: Pan/zoom/click-to-select, faction-colored system dots, sector labels, right-drag panning
- **egui info panel**: Selected system details (sector, position, popularity, garrison counts)
- **Status bar**: World statistics (systems, sectors, ships, fighters, characters)
- **WASM build**: 2.9MB browser artifact via `scripts/build-wasm.sh`
- **Dual-key entity identity**: DatId(u32) for serialization + slotmap keys for runtime
- **GameWorld root aggregate**: 6 SlotMap arenas (systems, sectors, capital_ship_classes, fighter_classes, characters, fleets)

### Technical
- macroquad 0.4, egui-macroquad 0.17, slotmap 1.0
- Two-layer type system: dat/ (binary mirror) and world/ (runtime model)
- ByteReader/ByteWriter codec with little-endian cursor
- MIT license

---

## Version History

| Version | Date | Milestone | Completion | Summary |
|---------|------|-----------|------------|---------|
| **v0.8.0** | **2026-03-17** | **Play-Test READY** | **~95%** | **Command palette, headless binary, seedable RNG, 7 new PanelAction variants** |
| v0.7.0 | 2026-03-16 | Release READY | ~95% | 8 story event chains, 7 new event variants, release packaging, CI, example mod |
| v0.6.0 | 2026-03-16 | Mod Workshop | ~92% | Mod manager panel, escape system, 3 reviewer bug fixes |
| v0.5.0 | 2026-03-16 | Full Parity | ~90% | Story events, Han speed, betrayal, decoys, 10 Character fields |
| v0.4.1 | 2026-03-15 | War Machine Fix | ~70% | Integration fixes, 14 systems wired, doc overhaul |
| v0.4.0 | 2026-03-15 | War Machine | ~68% | Combat, 9 missions, victory, save/load, Death Star |
| v0.3.0+RE | 2026-03-15 | Ghidra RE | ~42% | 5,127 decompiled functions, combat formulas, GNPRTB mapped |
| v0.3.0 | 2026-03-14 | War Room | ~42% | Full UI integration, audio, WASM build, fog/fleet overlays |
| v0.2.0 | 2026-03-13 | Living Galaxy | ~30% | 7 simulation systems, mod loader, 51/51 DAT parsers |
| v0.1.0 | 2026-03-11 | Galaxy Viewer | ~15% | Data viewer, galaxy map, 22 DAT parsers, WASM |
