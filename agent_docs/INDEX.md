---
title: "Agent Docs Index"
description: "Master index of AI agent reference documentation for Open Rebellion"
category: agent-docs
created: 2026-03-22
updated: 2026-10-20
tags: [index, agent-docs]
---

# Agent Docs Index

Reference documentation for AI agents working on the Open Rebellion codebase. Read `CLAUDE.md` at project root for which doc to load when.

## Architecture & Design

| Document | Description |
|----------|-------------|
| [architecture.md](architecture.md) | Crate graph, type system, entity identity, data flow, render architecture |
| [roadmap.md](roadmap.md) | Active milestones, the v1.0 release sequence, and addon plans |
| [simulation.md](simulation.md) | 17 simulation systems, advance() contract, integration order |
| [deterministic-replay.md](deterministic-replay.md) | Replay format, exact native/WASM artifact gate, DAT/config identity, checkpoints, and open engine-convergence work |
| [agent-tooling.md](agent-tooling.md) | Skill routing (Codex, Fable, Ghidra), browser and native (cua-driver) acceptance, silent runs, mutants and provenance gates, Mini builds, the build cache, disk report and Mini cleanup (`disk-report.sh`, `mini-clean.sh`) |
| [task-graph.md](task-graph.md) | br/bv task graph in `.beads/`: node kinds, the question-review-port-acceptance chain, commands |
| [dev-commands.md](dev-commands.md) | Developer commands for a native build: the grammar, the live channel, named controls, batch runs, saves and the run report |
| [test-tooling.md](test-tooling.md) | Telemetry query, seed calibration, native-versus-headless divergence, the pre-commit gates, trace determinism, invariants and the native scheduler checks |

## Tools and QA

| Document | Description |
|----------|-------------|
| [../docs/qa/2026-09-08-full-functionality-audit/README.md](../docs/qa/2026-09-08-full-functionality-audit/README.md) | The audit: feature ledger and evidence files; `audit-report.md` holds the evidence contract and the definition of 100% functional |
| [../tools/interface-parity/README.md](../tools/interface-parity/README.md) | Browser gates: fixture site, journey runners, A0 ingestion and comparison |
| [../tools/original-capture/README.md](../tools/original-capture/README.md) | The original game on the Mini: the capture VM, A0 captures, Wine, the save library |
| [../tools/original-capture/saves/INDEX.md](../tools/original-capture/saves/INDEX.md) | The original-game save library: each save's side, day, settings and state, and how to load one |
| [../tools/stage-ui-assets/README.md](../tools/stage-ui-assets/README.md) | Extracting the original's bitmaps, audio, movies, strings and Encyclopedia sources into the runtime layout |
| [../scripts/manual-ocr/README.md](../scripts/manual-ocr/README.md) | OCR of the game manual PDF |
| [../ghidra/notes/INDEX.md](../ghidra/notes/INDEX.md) | The Ghidra notes: function decompiles and the recovered subsystem notes (method in `ghidra-re.md`) |

## Game Knowledge

| Document | Description |
|----------|-------------|
| [game-domain.md](game-domain.md) | Galaxy, factions, units, missions, combat—game mechanics overview |
| [dat-formats.md](dat-formats.md) | DAT binary format reference, 3 structural patterns, 51 files |
| [ghidra-re.md](ghidra-re.md) | Ghidra RE status and method for REBEXE.EXE (22,741 functions), combat formulas, GNPRTB params |
| [seeding.md](seeding.md) | Initial galaxy state: character placement, fleet distribution, support (the neutral rule, `FUN_00559700`) |
| [original-save-editing.md](original-save-editing.md) | Editing copies of original-game saves to stage states play cannot reach, and accepting the edit in the original |
| [manual-cross-check.md](manual-cross-check.md) | Reading the manual before and after each interface feature; settling its claims against the trace |
| [main-menu-parity.md](main-menu-parity.md) | Original shuttle-cockpit geometry, resources, settings, actions, and browser acceptance matrix |

## Subsystems

| Document | Description |
|----------|-------------|
| [save-load.md](save-load.md) | Save format (no migrations), fingerprints, mod metadata hash |
| [mod-runtime.md](mod-runtime.md) | ModRuntime, ModConfig, enable/disable, hot reload |
| [modding.md](modding.md) | Mod loader: TOML manifest, RFC 7396, semver, load order |
| [mod-surface.md](mod-surface.md) | What the original fixes in REBEXE rather than data, area by area; rules for exposing a fixed rule as mod data |

## Assets & Media

| Document | Description |
|----------|-------------|
| [assets.md](assets.md) | 6 asset pipelines, 11 reference collections, 4 3D providers |
| [sprites.md](sprites.md) | Three deterministic sprite tracks: original-mesh direction sheets, advisor family transform, fighter textures |
| [references/INDEX.md](references/INDEX.md) | Reference image catalogs (planets, portraits, scenes, ships, weapons) named by a vision model |
| [game-media.md](game-media.md) | 18 DLLs, Smacker videos, WAV soundtrack |
| [dll-resource-catalog.md](dll-resource-catalog.md) | 2,441 BMPs + 3,223 data files across 11 DLLs |

## Per-System Detail (systems/)

| Document | System |
|----------|--------|
| [systems/ai-parity-tracker.md](systems/ai-parity-tracker.md) | The computer player's 14 modules mapped to the port, with the remaining port heuristics |
| [systems/combat.md](systems/combat.md) | Space + ground combat resolution |
| [systems/blockade.md](systems/blockade.md) | Fleet-presence blockade, running regiments, the timer `0x386` support shift |
| [systems/uprising.md](systems/uprising.md) | Incite/subdue uprising |
| [systems/death-star.md](systems/death-star.md) | Construction, fire, planet destruction, sector bit 11 |
| [systems/research.md](systems/research.md) | Points pools fed by the passive timer and Research missions |
| [systems/jedi.md](systems/jedi.md) | Force tier progression, detection |
| [systems/victory.md](systems/victory.md) | HQ capture, Death Star, terminal conditions |
| [systems/betrayal.md](systems/betrayal.md) | Loyalty-driven faction defection |
| [systems/captivity.md](systems/captivity.md) | Capture, the captor's side bits, escape timer `0x38a`, jail abodes and release |
| [systems/story-events.md](systems/story-events.md) | Scripted story chains, telemetry twins, event chaining |
