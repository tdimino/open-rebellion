---
title: "Agent Docs Index"
description: "Master index of AI agent reference documentation for Open Rebellion"
category: agent-docs
created: 2026-03-22
updated: 2026-03-22
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

## Game Knowledge

| Document | Description |
|----------|-------------|
| [game-domain.md](game-domain.md) | Galaxy, factions, units, missions, combat — game mechanics overview |
| [dat-formats.md](dat-formats.md) | DAT binary format reference, 3 structural patterns, 51 files |
| [ghidra-re.md](ghidra-re.md) | Ghidra RE summary: 5,127 functions, combat formulas, GNPRTB params |

## Subsystems

| Document | Description |
|----------|-------------|
| [save-load.md](save-load.md) | Save format v4, migration, mod metadata hash |
| [mod-runtime.md](mod-runtime.md) | ModRuntime, ModConfig, enable/disable, hot reload |
| [modding.md](modding.md) | Mod loader: TOML manifest, RFC 7396, semver, load order |

## Assets & Media

| Document | Description |
|----------|-------------|
| [assets.md](assets.md) | 6 asset pipelines, 11 reference collections, 4 3D providers |
| [game-media.md](game-media.md) | 18 DLLs, Smacker videos, WAV soundtrack |
| [dll-resource-catalog.md](dll-resource-catalog.md) | 2,441 BMPs + 3,223 data files across 11 DLLs |

## Per-System Detail (systems/)

| Document | System |
|----------|--------|
| [systems/ai-parity-tracker.md](systems/ai-parity-tracker.md) | AI function-by-function parity mapping — 6/6 core pipeline DONE |
| [systems/combat.md](systems/combat.md) | Space + ground combat resolution |
| [systems/blockade.md](systems/blockade.md) | Fleet-presence blockade mechanics |
| [systems/uprising.md](systems/uprising.md) | Incite/subdue uprising |
| [systems/death-star.md](systems/death-star.md) | Construction, fire, planet destruction |
| [systems/research.md](systems/research.md) | 3 tech trees, skill-based progression |
| [systems/jedi.md](systems/jedi.md) | Force tier progression, detection |
| [systems/victory.md](systems/victory.md) | HQ capture, Death Star, terminal conditions |
| [systems/betrayal.md](systems/betrayal.md) | Loyalty-driven faction defection |
