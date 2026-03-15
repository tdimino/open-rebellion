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

## [Unreleased] - Ghidra RE

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
| *next* | — | War Machine | ~68% est. | Combat, missions, victory, save/load |
| v0.3.0+RE | 2026-03-15 | Ghidra RE | ~42% | 5,127 decompiled functions, combat formulas, GNPRTB mapped |
| v0.3.0 | 2026-03-14 | War Room | ~42% | Full UI integration, audio, WASM build, fog/fleet overlays |
| v0.2.0 | 2026-03-13 | Living Galaxy | ~30% | 7 simulation systems, mod loader, 51/51 DAT parsers |
| v0.1.0 | 2026-03-11 | Galaxy Viewer | ~15% | Data viewer, galaxy map, 22 DAT parsers, WASM |
