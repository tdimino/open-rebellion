---
title: "Open Rebellion"
description: "Project instructions and build configuration for the Open Rebellion Rust reimplementation"
category: "reference"
created: 2026-03-11
updated: 2026-03-27
tags: [claude-code, build, conventions, workspace]
---

# Open Rebellion

Rust + macroquad + egui reimplementation of Star Wars Rebellion (1998, LucasArts). Runs native (macOS/Metal) and browser (WebAssembly/WebGL2). v0.19.0 — **Core 97%** | **UI 97%** | **Combat 99%** | **Overall ~97%**. 376 tests, zero warnings. Knesset Ereshkigal Phase 0-3 complete: effect algebra, economy system, P0 formula fixes, DS shield, officer combat rating, decoy missions, ship repair, Jabba 5-case chain, Emperor co-location, Leia Force discovery, 17 RE event IDs.

| Area | Key Features |
|------|-------------|
| **Simulation** | 15 systems, 11 mission types, dual-AI with config-driven targeting, ControlKind state machine, deterministic RNG seeding (3-system model, procedural control buckets, maintenance-budget units) |
| **Combat** | 7-phase pipeline: per-arc weapon fire, shield absorption (Phase 4), fighter launch/dogfight/recall, per-unit ground combat (TroopClassDef + facility bonus), difficulty modifiers, 25+ ship class DAT fields |
| **UI** | 9 panels, cockpit BMP sprites (3-state), galaxy overlays (facilities/sectors/blockades), tactical combat view, event screen overlays (61 BMPs), droid advisor (animated), GOKRES portraits + mini-icons |
| **Infrastructure** | BmpCache (HD PNG fallback), quad-snd audio (285 voice lines + soundtrack), REPL/CLI/JSONL telemetry, ModRuntime, WASM build with 2,231 staged BMPs |

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
| `rebellion-core` | Pure types, zero IO. `ids.rs`, `dat/`, `world/`, `effects.rs`, `economy.rs`, `repair.rs` | slotmap, serde |
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
- **Effects are the only output** (Knesset Ereshkigal Manifesto): new systems should return `Vec<GameEffect>` -- use the closed enum in `effects.rs`, not ad-hoc result types
- **Effects carry partial order**: tag with `EffectPhase` (Economy→Manufacturing→Movement→Combat→Diplomacy→Intelligence→Command→Endgame) -- within a phase, production order preserved
- **GameWorld is read-only during advance**: systems receive `&GameWorld` and produce effects -- the integrator applies mutations after all systems run
- **Economy runs before manufacturing**: `EconomySystem::advance()` in simulation tick order position 0, before manufacturing at position 1

## Known Limitations

- dat-dumper lives in `tools/` but is a library dependency of rebellion-data
- CapitalShipClass has 25+ promoted DAT fields; FighterClass has 20+. Remaining ~15 fields are decorative/unused by combat formulas
- Save v5 format (v3/v4 rejected) — no backward migration
- Droid Advisor BIN animation format partially decoded — uses frame-cycling fallback (see advisor.rs)
- Legacy seed fallback collapses Alliance HQ to Yavin (only when 3-system model can't identify Coruscant)

## Agent Docs

@agent_docs/architecture.md -- Crate graph, type system layers, entity identity, data flow, render architecture. Read when adding entity types or crates.
@agent_docs/roadmap.md -- Phase breakdown with status, what's next, what's blocked. Read when planning work.
agent_docs/simulation.md -- 15 simulation systems index, advance() pattern, integration order, "how to add" guides. Read when touching game logic.
agent_docs/systems/*.md -- Per-system detail docs (combat, blockade, uprising, death-star, research, jedi, victory, betrayal). Read when modifying a specific system.
agent_docs/save-load.md -- Save format (v5), migration framework, mod metadata hash, WASM stubs. Read when touching save/load.
agent_docs/mod-runtime.md -- ModRuntime, ModConfig, enable/disable, hot reload, structured errors. Read when wiring mod features.
agent_docs/dat-formats.md -- DAT binary format reference, all 3 structural patterns, file inventory, codec API. Read when parsing new DAT files.
agent_docs/game-domain.md -- Game mechanics glossary, entity relationships, implemented vs unimplemented systems. Read when implementing simulation logic.
agent_docs/modding.md -- Mod loader: TOML manifest, RFC 7396 merge patch, semver, load order, hot reload. Read when working on mod features.
agent_docs/ghidra-re.md -- REBEXE.EXE reverse engineering COMPLETE: 5,127 decompiled functions, combat formulas, 111 GNPRTB params, C++ class hierarchy, modder taxonomy. Read when implementing combat or designing mods.
agent_docs/systems/ai-parity-tracker.md -- Maps every original AI function to our implementation. Status: DONE/FAITHFUL/AUGMENTED/PARTIAL/MISSING. Read when modifying AI code.
agent_docs/assets.md -- Asset pipeline: HD upscaling (Real-ESRGAN + Gemini + Vertex), 3D models, audio, 2,231 DLL BMPs. Tools: Real-ESRGAN x4plus (~/tools/realesrgan/), PyMuPDF for PDF extraction, batch-rename-references.py. 21 reference collections (518 files + 1,097 extracted pages). Read when working on visual assets or the upscale pipeline.
agent_docs/game-media.md -- Game media overview: 18 DLLs, Smacker videos, WAV soundtrack. Read for high-level media inventory.
agent_docs/dll-resource-catalog.md -- Granular DLL resource catalog: 2,441 BMPs + 3,223 data files across 11 DLLs. Per-DLL resource ID ranges, image dimensions, content descriptions. Read when working with specific DLL resources.
agent_docs/references/INDEX.md -- Reference image catalogs: 5 files (portraits, ships, weapons, planets, scenes) listing 1,097 vision-LLM-named images. Load the relevant catalog when selecting Gemini reference images for upscaling.
agent_docs/seeding.md -- Game seeding pipeline: 3-system model, character stat rolling, named placement, 9 seed tables. Read when modifying initial galaxy state.

docs/mechanics/ -- Game mechanics wiki with 19 system docs + INDEX. Read for player/modder-facing mechanics reference.
docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md -- Knesset Ereshkigal: 6-phase eval-driven parity sprint + Open Souls functional refactor. Includes Functional Programming Manifesto (10 principles), effect algebra spec, modularity violations audit, Codex work packages. Read when planning Phase 3+ work.
docs/reports/2026-03-26-community-disassembly-cross-reference.md -- 13,036 decompiled functions cross-referenced against our implementation. P0-P3 gap inventory with GNPRTB parameters. Read when implementing missing game mechanics.

## Reports

- [Knesset Athirat (2026-03-13)](docs/reports/2026-03-13-knesset-athirat-swarm-report.md) — Living Galaxy + War Room. 4 daborot, ~11.7K LOC, 105+ tests.
- Knesset Demiurge (2026-03-24) — UI parity swarm. 5 daborot, 13 tasks, ~3.8K LOC. Cockpit, tactical combat, audio, overlays, portraits.
- Knesset Sassuratu (2026-03-24) — Visual QA + SEO/GEO. 47 test scenarios, 25 screenshots, 5 defects found and fixed.
- [Game Seeding Audit](docs/plans/2026-03-24-003-game-seeding-parity-execplan.md) — 56 gaps found vs TheArchitect2018 wiki. All M1-M8 implemented, seeding COMPLETE.
- Knesset Kothar (2026-03-25) — Final parity sprint. 4 daborot, 9 tasks. Seeding M5-M8, 25 ship class fields, droid advisor, WASM fixes, HD assets.
- Knesset Ma'at (2026-03-25) — Combat + asset wave. 5 daborot. Shield phase, fighter combat, ground combat + difficulty, cockpit sprites, HUD overlays.
- [Community Disassembly Cross-Reference (2026-03-26)](docs/reports/2026-03-26-community-disassembly-cross-reference.md) — 13,036 decompiled functions vs our 5,151. 4 domain agents. Overall parity: ~85%. Biggest gap: economy tick loop.
- [Knesset Ereshkigal Plan (2026-03-26)](docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md) — Eval-driven parity sprint + Open Souls refactor. Phase 0-3 complete: effect algebra, economy system, P0 formula fixes, DS shield, Jabba 5-case, Emperor co-location, Leia Force. 6 phases total.

## External References

- Metasharp editor (C# DAT parsers): `~/Desktop/Programming/StarWarsRebellionEditor.NET/`
- rebellion2 (C# game models): `~/Desktop/Programming/rebellion2/`
- Extracted game data: `~/Desktop/Programming/star-wars-rebellion/GData/`
- Project plan: `~/.claude/plans/2026-03-12-claude-md-agent-docs-for-open-rebellion.md`
