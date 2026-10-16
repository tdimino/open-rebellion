---
title: "Agent Docs Index"
description: "Master index of AI agent reference documentation for Open Rebellion"
category: agent-docs
created: 2026-03-22
updated: 2026-10-16
tags: [index, agent-docs]
---

# Agent Docs Index

Reference documentation for AI agents working on the Open Rebellion codebase. Read `CLAUDE.md` at project root for which doc to load when.

## Architecture & Design

| Document | Description |
|----------|-------------|
| [architecture.md](architecture.md) | Crate graph, type system, entity identity, data flow, render architecture |
| [roadmap.md](roadmap.md) | Phase breakdown with status, addon plans, AI parity gaps |
| [simulation.md](simulation.md) | 15 simulation systems, advance() contract, integration order |
| [deterministic-replay.md](deterministic-replay.md) | Replay format, exact native/WASM artifact gate, DAT/config identity, checkpoints, and open engine-convergence work |
| [agent-tooling.md](agent-tooling.md) | Claude Code Minoan and Codex skill routing, including browser acceptance, Fable, and Ghidra |
| [task-graph.md](task-graph.md) | br/bv task graph in `.beads/`: node kinds, the question-review-port-acceptance chain, commands |
| [dev-commands.md](dev-commands.md) | Developer commands for a native build: the grammar, the live channel, named controls, batch runs, saves and the run report |
| [test-tooling.md](test-tooling.md) | Telemetry query, seed calibration and native-versus-headless divergence scripts |

## Game Knowledge

| Document | Description |
|----------|-------------|
| [game-domain.md](game-domain.md) | Galaxy, factions, units, missions, combat — game mechanics overview |
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
| [systems/story-events.md](systems/story-events.md) | Scripted story chains, telemetry twins, event chaining |
