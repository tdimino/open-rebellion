---
title: "Roadmap"
description: "Development milestones from Galaxy Viewer through Release packaging"
category: "agent-docs"
created: 2026-03-11
updated: 2026-03-16
---

# Roadmap

## Galaxy Viewer -- COMPLETE
*Interactive galaxy data viewer -- native + WASM*

Commits: `4e258bb` (scaffold) > `5905497` (dat-dumper + types) > `fc968ec` (renderer + WASM) > `22b61a6` (review fixes)

Delivered:
- Cargo workspace with 5 crates
- dat-dumper: 22/22 documented DAT files parsed, 3 structural patterns, round-trip byte validated
- rebellion-core: DatId newtype, 11 slotmap keys, dat/ enums, world/ structs, GameWorld root
- rebellion-data: DAT to GameWorld loader (sectors > systems > ships > fighters > characters)
- rebellion-render: macroquad galaxy map with pan/zoom/click-to-select, egui info panel + status bar
- WASM build: 2.9MB browser artifact via scripts/build-wasm.sh

## Living Galaxy -- COMPLETE
*Automated galaxy simulation — shipped 2026-03-13 by Knesset Athirat*

All 11 deliverables shipped. ~8.7K LOC, 105+ unit tests. See `docs/reports/2026-03-13-knesset-athirat-swarm-report.md`.

Delivered:
- TEXTSTRA.DLL string extraction via pelite (real entity names)
- GameClock with pause/1x/2x/4x speed controls
- 9 seed table loaders + fleet instantiation
- All 51/51 DAT parsers with round-trip validation (29 new)
- Manufacturing system with production queues
- Mission system (diplomacy, recruitment) -- ported from rebellion2's Mission.cs
- Event system (conditional triggers, chaining, deterministic rng)
- AI manager (officer assignment, production priority, fleet deployment)
- Message log UI (egui bottom panel, 6 color-coded categories)
- Mod loader: TOML manifests, RFC 7396 merge patch, semver, hot reload
- Main loop integration (all systems wired)

## War Room -- COMPLETE
*Full strategy game without combat*

Committed: `6d47a10` (integration wiring — fog/fleet/panels/encyclopedia/audio into main loop)

Delivered:
- Player faction selection + 5 UI panels (officers, fleets, manufacturing, missions)
- Fleet movement on galaxy map (diamond icons, dashed routes, ETA labels)
- Fog of war with monotonic reveal + advance intel at 50% transit
- Encyclopedia viewer with 4 tabs + BMP texture cache from EData/
- Audio system via quad-snd (CoreAudio/ALSA/WebAudio)
- Main loop integration: all draw calls + event hooks wired

## War Machine -- COMPLETE
*Complete strategy game*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Victory conditions (capture enemy HQ, destroy Death Star / find Rebel base)
5. 3D tactical models: nano-banana-pro concept art → Hunyuan3D Pro/Meshy → Blender sprite sheets. Pipeline proven in World War Watcher (14 models). See `@agent_docs/assets.md`.

Ghidra RE of REBEXE.EXE complete: 5,127 functions decompiled, combat call chain fully traced, bombardment formula decoded, 111 GNPRTB parameters mapped, C++ class hierarchy reconstructed. See `ghidra/notes/` for 7 scholar docs (4,179 lines) and 5,127 decompiled C files.

## Full Parity -- COMPLETE
*Feature-complete — shipped 2026-03-16 by Knesset Shapash*

Delivered (v0.5.0):
- 4 scripted story chains (Luke Dagobah, Final Battle, Bounty Hunters, Jabba's Palace)
- Han Solo speed bonus, betrayal mechanics, decoy system, escape system
- 10 new Character fields promoted from DAT, mission state flags
- 6 new EventConditions + 6 new EventActions, 15 RE event ID constants
- All 9 mission types with world-mutating effects (sabotage, assassination, rescue, abduction, incite uprising)

## Mod Workshop -- COMPLETE
*Release candidate — Knesset Elat (v0.6.0)*

Delivered:
- Sensor-radius fog (detection field on CapitalShipClass)
- Captivity state tracking (is_captive, captured_by, capture_tick)
- Research pure advance() contract (caller applies level-ups)
- Save format v4 with mod metadata + FNV-1a hash, migration framework
- ModRuntime: discovery, enable/disable, structured errors, config persistence
- Mod Manager egui panel (discover, toggle, reload)
- ESCAPETB per-tick escape check wired into main loop

## Release -- READY
*Release packaging — Knesset Hokhmah (v0.7.0)*

Delivered:
- 8 new story event chains (0x1e1, 0x1e5, 0x390-0x39A)
- 4 new EventCondition variants + 3 new EventAction variants
- Release packaging scripts (macOS + web)
- Example mod (star-destroyer-rebalance)
- GitHub Actions CI (test + WASM build)
- Vendored web/gl.js
- Release profile (LTO, strip, codegen-units=1)

Remaining:
- HD asset pack bulk execution
- Video playback (Smacker → WebM)
- Distribution: itch.io (web), Homebrew (macOS), GitHub Releases

## AI Overhaul -- COMPLETE
*v0.13.0 — AI balance, observability, autoresearch*

Delivered:
- Distance-based fleet transit (Euclidean, DISTANCE_SCALE=2, MIN_TRANSIT_TICKS=10)
- Starting force distribution (Empire 10 systems, Alliance 3)
- Garrison strength scoring (ships + troops + facilities)
- Galaxy-wide strategic bucketing (7 categories)
- Per-fleet attack targeting with deconfliction + battle repeat penalty
- Two-pass deployment (original parity: primary assign + secondary redistribute)
- Faction-asymmetric doctrine (Empire→HQ strike, Alliance→guerrilla)
- Role-based character AI (Jedi reserved, diplomats prioritized)
- Production doctrine (capital ships > fighters > yards)
- Build completion wiring (manufactured items added to GameWorld)
- HQ garrison defense (first fleet defends HQ)
- ControlKind state machine (Uncontrolled, Controlled, Contested, Uprising)
- Human-readable JSONL event payloads (system/character names)
- Enhanced --summary (galaxy control, transit state, combat diagnostics)
- --jsonl streaming flag for stdout
- REPL commands: systems, transit, events N
- eval_game_quality.py (8 sub-metrics, degenerate detection)
- autoresearch_loop.py (Karpathy self-improvement loop)
- AI parity tracker (maps 6 original functions to our implementation)
- Death Star fleet exemption (always targets enemy HQ)

Campaign results: VICTORY at tick 1188, 211 battles, eval score 0.59

## Immediate Next Steps

1. **Telemetry enrichment** -- control_changed events, 250-tick snapshots, research progress
2. **Config-driven AI** -- externalize constants to JSON, parity/augmentation split
3. **Combat spread** -- expand territory via diplomacy for diverse battle locations
4. **Run autoresearch** -- 10+ iterations with 3 seeds, analyze optimal config
5. **Custom character addon** -- Demiurgos: academy archetypes, homeworld, Force roll

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data -- works but unconventional
- WASM cfg guards added but browser data loading returns error stub (no fetch API yet)
- `web/gl.js` fetched from external URL on first build -- should vendor or pin version
- Save v3 files rejected (bincode layout incompatible) -- no migration possible without SaveStateV3
- Mod Manager panel passes empty mod list -- needs ModRuntime data population in main.rs
- `enabled_sorted()` silently returns empty on dependency resolution errors
- `ModConfig::load()` silently drops corrupted config.toml
- No additive entity creation in mods -- patches only modify existing entities
- BuildableKind type safety -- class arenas needed for troops/facilities
