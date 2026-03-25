---
title: "Open Rebellion"
description: "Project instructions and build configuration for the Open Rebellion Rust reimplementation"
category: "reference"
created: 2026-03-11
updated: 2026-03-25
tags: [claude-code, build, conventions, workspace]
---

# Open Rebellion

Rust + macroquad + egui reimplementation of Star Wars Rebellion (1998, LucasArts). Runs native (macOS/Metal) and browser (WebAssembly/WebGL2). v0.18.0 — **Core 97%** | **UI 95%** | **Combat 90%** | **Overall ~93%**. GameMode state machine (MainMenu → GameSetup → Galaxy), Star Wars egui theme (dark space + gold/amber accents), Liberation Sans font (faithful to original Arial), TEXTSTRA.DLL string extraction (1,347 entity names), right-click context menus (system + fleet), interactive fleet editor (assign/remove officers, merge fleets), 9 player control panels (officers, fleets, manufacturing, missions, research, jedi training, bombardment, Death Star, loyalty/uprising), character detail view with Force progression, expanded system info panel, BMP texture cache with DllSource enum + HD PNG fallback, faction cockpit chrome with 9 clickable control buttons, galaxy map overlays (facility icons, sector boundaries, blockade indicators), tactical combat view (2D arena, ship placement, phased combat, targeting/retreat, ground combat), audio integration (quad-snd soundtrack + 285 voice lines + SFX), entity portraits from GOKRES.DLL, 15 simulation systems, 11 mission types, fleet combat + blockades working, dual-AI with per-fleet targeting + deconfliction + battle penalty, config-driven AI (`tuning.rs` + `--config` flag, 16 tunable parameters), REPL mode for LLM agent play (`--repl`, `--jsonl`), CLI `--exec` (16 + 3 REPL commands), ModRuntime with toggle/reload, shared command registry, JSONL telemetry with human-readable payloads, `ControlKind` state machine, distance-based fleet transit, force distribution (10 Empire / 3 Alliance), eval_game_quality.py + autoresearch_loop.py, 308 tests, zero warnings. Game seeding: 3-system model, character stat rolling + named placement, procedural control buckets, support initialization, energy/raw materials, facility generation, maintenance-budget unit seeding, deterministic RNG. Droid Advisor (animated C-3PO/R2-D2 or Imperial protocol droid), 25 promoted ship class DAT fields for combat parity, 32 HD upscaled assets.

## Build

**Critical**: `~/.local/bin/cc` shadows Apple clang. Prefix all cargo commands with `PATH="/usr/bin:$PATH"`.

```bash
# Native (interactive galaxy map)
PATH="/usr/bin:$PATH" cargo run -p rebellion-app -- data/base

# WASM (browser build, then serve web/)
bash scripts/build-wasm.sh

# dat-dumper (DAT binary files to JSON)
PATH="/usr/bin:$PATH" cargo run -p dat-dumper -- --gdata data/base --output data/base/json

# Extract TEXTSTRA.DLL entity names to JSON (for WASM builds)
PATH="/usr/bin:$PATH" cargo run -p dat-dumper -- --gdata data/base --extract-strings --output web/data/base

# Type check
PATH="/usr/bin:$PATH" cargo check
```

## Workspace

| Crate | Role | Key Deps |
|-------|------|----------|
| `rebellion-core` | Pure types, zero IO. `ids.rs`, `dat/`, `world/` | slotmap, serde |
| `rebellion-data` | DAT to GameWorld loader. Uses dat-dumper as library | rebellion-core, dat-dumper |
| `rebellion-render` | macroquad 0.4 + egui-macroquad 0.17 galaxy map | rebellion-core, macroquad |
| `rebellion-app` | Entry point and main loop | all crates above |
| `rebellion-playtest` | Headless play-test binary with JSONL logging | rebellion-core, rebellion-data, rand, rand_xoshiro, clap |
| `dat-dumper` (tools/) | 51/51 DAT parsers with round-trip byte validation. `--extract-strings` for TEXTSTRA.DLL (1,347 entity names). CLI + library | clap, serde, pelite |

## Conventions

- **Two-layer types**: `dat/` structs mirror binary layout (import/export only); `world/` structs are the runtime model used by game logic and rendering
- **Entity identity**: `DatId(u32)` preserves original binary IDs; slotmap `*Key` types are runtime-only arena handles -- never serialize slotmap keys directly
- **DAT parsers**: Implement `DatRecord` trait (`parse` + `write_bytes`). Every parser must pass round-trip byte validation -- if reserialized bytes differ from original, the parser is wrong
- **Entity table pattern**: 16-byte header (`field1`, `count`, `family_id`, `field4`) then `count` fixed-size records
- **Never add rendering deps to rebellion-core** -- it must remain headless-testable
- **Never use `fieldN` for a field whose meaning is known** -- name it semantically

## Known Limitations

- dat-dumper lives in `tools/` but is a library dependency of rebellion-data
- CapitalShipClass/FighterClass world models carry only ~10 of 50+ DAT fields — 15 more needed for full combat parity
- Save v5 format (v3/v4 rejected) — no backward migration
- Droid Advisor BIN animation format partially decoded — uses frame-cycling fallback (see advisor.rs)
- Legacy seed fallback collapses Alliance HQ to Yavin (only when 3-system model can't identify Coruscant)

## Agent Docs

@agent_docs/architecture.md -- Crate graph, type system layers, entity identity, data flow, render architecture. Read when adding entity types or crates.
@agent_docs/roadmap.md -- Phase breakdown with status, what's next, what's blocked. Read when planning work.
agent_docs/simulation.md -- 15 simulation systems index, advance() pattern, integration order, "how to add" guides. Read when touching game logic.
agent_docs/systems/*.md -- Per-system detail docs (combat, blockade, uprising, death-star, research, jedi, victory, betrayal). Read when modifying a specific system.
agent_docs/save-load.md -- Save format (v4), migration framework, mod metadata hash, WASM stubs. Read when touching save/load.
agent_docs/mod-runtime.md -- ModRuntime, ModConfig, enable/disable, hot reload, structured errors. Read when wiring mod features.
agent_docs/dat-formats.md -- DAT binary format reference, all 3 structural patterns, file inventory, codec API. Read when parsing new DAT files.
agent_docs/game-domain.md -- Game mechanics glossary, entity relationships, implemented vs unimplemented systems. Read when implementing simulation logic.
agent_docs/modding.md -- Mod loader: TOML manifest, RFC 7396 merge patch, semver, load order, hot reload. Read when working on mod features.
agent_docs/ghidra-re.md -- REBEXE.EXE reverse engineering COMPLETE: 5,127 decompiled functions, combat formulas, 111 GNPRTB params, C++ class hierarchy, modder taxonomy. Read when implementing combat or designing mods.
agent_docs/systems/ai-parity-tracker.md -- Maps every original AI function to our implementation. Status: DONE/FAITHFUL/AUGMENTED/PARTIAL/MISSING. Read when modifying AI code.
agent_docs/assets.md -- Asset pipeline: 6 pipelines (HD upscaling, 3D models, encyclopedia, reference image generation, audio, UI upscaling). 11 reference collections, 4 3D providers, 6 audio tools, 450+ UI BMPs. Read when working on visual or audio assets.
agent_docs/game-media.md -- Game media overview: 18 DLLs, Smacker videos, WAV soundtrack. Read for high-level media inventory.
agent_docs/dll-resource-catalog.md -- Granular DLL resource catalog: 2,441 BMPs + 3,223 data files across 11 DLLs. Per-DLL resource ID ranges, image dimensions, content descriptions. Read when working with specific DLL resources.
agent_docs/seeding.md -- Game seeding pipeline: 3-system model, character stat rolling, named placement, 9 seed tables. Read when modifying initial galaxy state.

docs/mechanics/ -- Game mechanics wiki with 19 system docs + INDEX. Read for player/modder-facing mechanics reference.

## Reports

- [Knesset Athirat (2026-03-13)](docs/reports/2026-03-13-knesset-athirat-swarm-report.md) — Living Galaxy + War Room. 4 daborot, ~11.7K LOC, 105+ tests.
- Knesset Demiurge (2026-03-24) — UI parity swarm. 5 daborot, 13 tasks, ~3.8K LOC. Cockpit, tactical combat, audio, overlays, portraits.
- Knesset Sassuratu (2026-03-24) — Visual QA + SEO/GEO. 47 test scenarios, 25 screenshots, 5 defects found and fixed.
- [Game Seeding Audit](docs/plans/2026-03-24-003-game-seeding-parity-execplan.md) — 56 gaps found vs TheArchitect2018 wiki. M1-M4 implemented (8 tests).

## External References

- Metasharp editor (C# DAT parsers): `~/Desktop/Programming/StarWarsRebellionEditor.NET/`
- rebellion2 (C# game models): `~/Desktop/Programming/rebellion2/`
- Extracted game data: `~/Desktop/Programming/star-wars-rebellion/GData/`
- Project plan: `~/.claude/plans/2026-03-12-claude-md-agent-docs-for-open-rebellion.md`
