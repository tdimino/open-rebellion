---
title: "Open Rebellion"
description: "Project instructions and build configuration for the Open Rebellion Rust reimplementation"
category: "reference"
created: 2026-03-11
updated: 2026-04-14
tags: [claude-code, build, conventions, workspace]
---

# Open Rebellion

Rust + macroquad + egui reimplementation of Star Wars Rebellion (1998, LucasArts). Runs native (macOS/Metal) and browser (WebAssembly/WebGL2). v0.22.0 — **Core 99%** | **UI 99%** | **Combat 99%** | **Overall ~99%**. 465 tests, zero warnings. All 17 simulation sections route through `PerceptionIntegrator` for mutation + telemetry; economy runs before manufacturing, build completions land in-world, AI dispatch is faction-aware. Knesset Tammuz: cutscene state machine (8 story triggers), Emperor Palpatine combat modifier, advisor BIN v2 decoder (99% parse rate), AI parity closeout (15/18 validators, troop deployment, DS multi-target).

Native cutscenes are optional. Run `bash scripts/decode-cutscenes.sh` once to expand `assets/references/ref-videos/*.webm` into ignored PNG frame sequences + WAV sidecars under `assets/references/cutscene-frames/`. If those decoded assets are missing, the app logs a skip message and continues without crashing.

| Area | Key Features |
|------|-------------|
| **Simulation** | 15 systems, 11 mission types, dual-AI with config-driven targeting, ControlKind state machine, deterministic RNG seeding (3-system model, procedural control buckets, maintenance-budget units) |
| **Combat** | 7-phase pipeline: per-arc weapon fire, shield absorption (Phase 4), fighter launch/dogfight/recall, per-unit ground combat (TroopClassDef + facility bonus), difficulty modifiers, 25+ ship class DAT fields |
| **UI** | 13 egui panels, cockpit BMP sprites (3-state), galaxy overlays (facilities/sectors/blockades), tactical combat view, event screen overlays (61 BMPs), droid advisor (BIN-driven sequencing), GOKRES portraits + mini-icons, native cutscene playback |
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
| `rebellion-core` | Pure types, zero IO. `ids.rs`, `dat/`, `world/`, `effects.rs`, `economy.rs`, `repair.rs`, `net_protocol.rs` | slotmap, serde |
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
- Save v7 format (v3/v4/v5/v6 rejected). v6→v7: ShipInstance promotion (Fleet.capital_ships now per-hull Vec<ShipInstance>)
- Droid Advisor BIN format simple variant (`u16 count + u16 ids`) decoded — covers ~24% of files; the remaining ~76% declare inconsistent lengths and indicate one or more undocumented header variants. The unparsed files fall back to legacy sorted-frame cycling (see advisor.rs)
- Legacy seed fallback collapses Alliance HQ to Yavin (only when 3-system model can't identify Coruscant)

## Agent Docs

@agent_docs/architecture.md -- Crate graph, type system layers, entity identity, data flow, render architecture. Read when adding entity types or crates.
@agent_docs/roadmap.md -- Phase breakdown with status, what's next, what's blocked. Read when planning work.
agent_docs/simulation.md -- 15 simulation systems index, advance() pattern, integration order, "how to add" guides. Read when touching game logic.
agent_docs/systems/*.md -- Per-system detail docs (combat, blockade, uprising, death-star, research, jedi, victory, betrayal, story-events). Read when modifying a specific system.
agent_docs/save-load.md -- Save format (v7), migration framework, mod metadata hash, WASM stubs. Read when touching save/load.
agent_docs/mod-runtime.md -- ModRuntime, ModConfig, enable/disable, hot reload, structured errors. Read when wiring mod features.
agent_docs/dat-formats.md -- DAT binary format reference, all 3 structural patterns, file inventory, codec API. Read when parsing new DAT files.
agent_docs/game-domain.md -- Game mechanics glossary, entity relationships, implemented vs unimplemented systems. Read when implementing simulation logic.
agent_docs/modding.md -- Mod loader: TOML manifest, RFC 7396 merge patch, semver, load order, hot reload. Read when working on mod features.
agent_docs/ghidra-re.md -- REBEXE.EXE reverse engineering COMPLETE: 5,127 decompiled functions, combat formulas, 111 GNPRTB params, C++ class hierarchy, modder taxonomy. Read when implementing combat or designing mods.
agent_docs/systems/ai-parity-tracker.md -- Maps every original AI function to our implementation. Status: DONE/FAITHFUL/AUGMENTED/PARTIAL/MISSING. Read when modifying AI code.
agent_docs/assets.md -- Asset pipeline: HD upscaling via UltraSharp V2 (DAT2, Spandrel+MPS, $0, shootout winner across all 5 categories). 235/2,231 DLL BMPs done. Scripts: local-upscale-batch.py, model-shootout.py. 1,320 audited reference images across 21 collections. Read when working on visual assets or the upscale pipeline.
agent_docs/game-media.md -- Game media overview: 18 DLLs, Smacker videos, WAV soundtrack. Read for high-level media inventory.
agent_docs/dll-resource-catalog.md -- Granular DLL resource catalog: 2,441 BMPs + 3,223 data files across 11 DLLs. Per-DLL resource ID ranges, image dimensions, content descriptions. Read when working with specific DLL resources.
agent_docs/references/INDEX.md -- Reference image catalogs: 5 catalog files + 4 new collections (SWCCG cards, SWG TCG art, McQuarrie, Wookieepedia OT). 1,320 audited images. All non-OT portraits removed, text pages quarantined. Read when working with reference images for upscaling.
agent_docs/seeding.md -- Game seeding pipeline: 3-system model, character stat rolling, named placement, 9 seed tables. Read when modifying initial galaxy state.

docs/mechanics/ -- Game mechanics wiki with 20 system docs + INDEX. Read for player/modder-facing mechanics reference.
docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md -- Knesset Ereshkigal: 6-phase eval-driven parity sprint + Open Souls functional refactor. All 6 phases COMPLETE. Includes Functional Programming Manifesto (10 principles), effect algebra spec, modularity violations audit, PerceptionIntegrator architecture.
docs/plans/2026-03-28-001-feat-knesset-ptah-todo-resolution-plan.md -- Knesset Ptah: 5-phase TODO resolution sprint. 12/13 TODOs resolved. Telemetry 15/17, DS victory + entity cleanup, espionage_rating, UI wiring, facility type promotion. ShipInstance deferred to Knesset Hephaestus.
docs/reports/2026-03-26-community-disassembly-cross-reference.md -- 13,036 decompiled functions cross-referenced against our implementation. P0-P3 gap inventory with GNPRTB parameters. Read when implementing missing game mechanics.

## Reports

- [Knesset Athirat (2026-03-13)](docs/reports/2026-03-13-knesset-athirat-swarm-report.md) — Living Galaxy + War Room. 4 daborot, ~11.7K LOC, 105+ tests.
- Knesset Demiurge (2026-03-24) — UI parity swarm. 5 daborot, 13 tasks, ~3.8K LOC. Cockpit, tactical combat, audio, overlays, portraits.
- Knesset Sassuratu (2026-03-24) — Visual QA + SEO/GEO. 47 test scenarios, 25 screenshots, 5 defects found and fixed.
- [Game Seeding Audit](docs/plans/2026-03-24-003-game-seeding-parity-execplan.md) — 56 gaps found vs TheArchitect2018 wiki. All M1-M8 implemented, seeding COMPLETE.
- Knesset Kothar (2026-03-25) — Final parity sprint. 4 daborot, 9 tasks. Seeding M5-M8, 25 ship class fields, droid advisor, WASM fixes, HD assets.
- Knesset Ma'at (2026-03-25) — Combat + asset wave. 5 daborot. Shield phase, fighter combat, ground combat + difficulty, cockpit sprites, HUD overlays.
- [Community Disassembly Cross-Reference (2026-03-26)](docs/reports/2026-03-26-community-disassembly-cross-reference.md) — 13,036 decompiled functions vs our 5,151. 4 domain agents. Overall parity: ~85%. Biggest gap: economy tick loop.
- [Knesset Ereshkigal Plan (2026-03-26)](docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md) — Eval-driven parity sprint + Open Souls refactor. All 6 phases COMPLETE. Phase 4: PerceptionIntegrator extraction (simulation.rs 73% reduction). Phase 5: 179 NetMessage protocol types + telemetry coverage test (10/17 required systems verified).
- [Knesset Ptah (2026-03-28)](docs/plans/2026-03-28-001-feat-knesset-ptah-todo-resolution-plan.md) — TODO resolution sprint. 12 of 13 TODOs resolved across 5 phases. Telemetry 15/17, DS victory fix, entity cleanup, espionage_rating, save v6, UI wiring, facility type promotion. 1 deferred (ShipInstance promotion → Knesset Hephaestus).
- Knesset Hephaestus (2026-03-30) — ShipInstance promotion. Fleet.capital_ships promoted from aggregate Vec<ShipEntry> to per-hull Vec<ShipInstance> (hull_current, alive, shield_weapon_packed, faction_is_alliance). ShipEntry removed. Fleet helper methods added (ship_count, ship_counts_by_class, is_empty). RepairSystem now emits real ShipRepaired events with hull deltas — last TODO resolved. Fleet merge and combat damage indexing simplified. Repair wired into interactive main.rs. Save format bumped to v7. 403 tests.
- [Knesset Resheph (2026-04-03)](docs/plans/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md) — Final parity sprint. 10 shipped tasks across combat, AI, WASM, UI, and eval: per-weapon fire strength, 10/18 dispatch validators, faction deploy budgets, uprising prevention, Death Star escort, browser localStorage saves, WASM asset/audio path fixes, DLL resource IDs, and the golden-value parity oracle.
- Knesset Kothar wa Khasis (2026-04-07) — Resheph deferrals closed. **U2**: native cutscene playback (`crates/rebellion-render/src/video_player.rs`) via PNG-frame + WAV sidecar streaming, no ffmpeg/libvpx runtime deps. `scripts/decode-cutscenes.sh` is the one-time local decode (atomic temp-dir rename on success). New `GameMode::Cutscene` plays `000.webm` on startup and `201.webm`/`202.webm` on victory/defeat with ESC/SPACE skip; missing decoded assets log a skip and continue. WASM build untouched via cfg-gated stub. New dep: `quad-snd` on rebellion-render. **C1**: `parse_advisor_bin()` decodes the simple `u16 count + u16 frame_id[]` format with real-error rejection; `AdvisorState` walks authored sequences from idle/normal/critical bands and falls back to legacy cycling when BINs are absent or malformed. ~24% of advisor BINs (183 of ~752 per faction) match the simple format — the rest declare inconsistent lengths and are logged at load time as a per-faction summary (valid / parse-failed / empty / io-failed), indicating one or more undocumented header variants. 417 tests passing across the workspace (322 core + 50 data + 42 render + 3 doc).
- Knesset Tammuz (2026-04-12) — Full parity sprint. 7 phases: SpecialForceSpawned wiring, mission telemetry (R6-R8, R11), Emperor Palpatine 1.5x combat modifier, cutscene state machine (8 story triggers 101-108), advisor BIN v2 cascading decoder (12%→99% parse rate), AI parity closeout (15/18 validators, troop deployment, DS multi-target), code hygiene. 465 tests.
- Knesset Shamash-Bet Dabora 3 (2026-04-12) — Story events + betrayal telemetry sprint. 8 R-tasks shipped: **R1** EVT_HAN_RESCUE (0x200) telemetry twin via EventFired chaining; **R2** EVT_JABBA_PRISONERS (0x231) consolidator with 3 OR-branch variants and self-guard; **R3** EVT_HAN_PERMANENT_FREEZE (0x39B) 5-stage carbonite countdown using existing primitives only; **R4** Final Battle heritage gate via render layer (single 0x220, heritage_known BMP branching in event_screen.rs); **R5** Bounty Hunters real SpawnSpecialForce + CharacterAssignedToFleet precondition; **R9/R10** EVT_TRAITOR_REVEALED before faction flip + EVT_SIDE_CHANGE after in integrator; **R13** stale comment cleanup. 447 tests (348 core + 50 data + 46 render + 3 doc), zero warnings.

- HD Upscaling Shootout (2026-04-14) — 8-model comparison on 20 representative BMPs (5 categories). UltraSharp V2 (DAT2, Spandrel+MPS) won all categories. Models tested: Real-ESRGAN, PBRify, UltraSharp, GTAV_dither, FSDedither Riven, Vertex AI, Topaz CGI, palette recon+PBRify. Batch pipeline `scripts/local-upscale-batch.py` built. 235/2,231 BMPs upscaled. Plan: `~/.claude/plans/2026-04-12-hd-upscaling-pipeline-model-research-execution-plan.md`.

## External References

- Metasharp editor (C# DAT parsers): `~/Desktop/Programming/StarWarsRebellionEditor.NET/`
- rebellion2 (C# game models): `~/Desktop/Programming/rebellion2/`
- Extracted game data: `~/Desktop/Programming/star-wars-rebellion/GData/`
- Project plan: `~/.claude/plans/2026-03-12-claude-md-agent-docs-for-open-rebellion.md`
