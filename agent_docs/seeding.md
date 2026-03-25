---
title: "Game Seeding Pipeline"
description: "Initial galaxy state generation — character placement, fleet distribution, support initialization"
category: "agent-docs"
created: 2026-03-25
tags: [seeding, initialization, GNPRTB, SDPRTB, seeds]
---

# Game Seeding Pipeline

## Overview

`crates/rebellion-data/src/seeds.rs` handles initial galaxy state generation when starting a new game. The pipeline processes 9 seed tables from DAT files and applies them to the 3 special systems.

## 3-System Model

| System | Role | Fixed Assets |
|--------|------|-------------|
| **Coruscant** | Empire HQ | Empire fleets (CMUNEFTB), facilities (FACLHQTB), Vader + Palpatine |
| **Yavin** | Alliance base | Alliance troops (CMUNAFTB group 1), garrison (YAVNGRTB), Luke/Leia/Han/Wedge/Chewie/Dodonna |
| **Random Rim** | Rebel HQ | Alliance HQ fleet (CMUNHQTB), HQ facilities (FACLHQTB), Mon Mothma |

Rebel HQ is selected via `select_special_systems()` — deterministic with RNG seed, always a rim system (SectorGroup::RimOuter).

## Key Functions

- `apply_seeds_with_rng(world, tables, gnprtb, seed_options, rng)` — main entry point
- `select_special_systems(world, rng) -> SpecialSystems` — picks the 3 systems
- `roll_character_stats(world, rng)` — rolls SkillPair{base, variance} → concrete values
- `place_named_characters(world, special)` — sets current_system for 8+ named characters
- `initialize_special_systems(world, special)` — sets populated, charted, controlled, support

## SeedOptions

```rust
pub struct SeedOptions {
    pub galaxy_size: GalaxySize,
    pub difficulty: Difficulty,
    pub player_faction: Faction,
    pub rng_seed: u64,
}
```

Threaded from Game Setup screen → `load_game_data()` → `apply_seeds_with_rng()`.

## Parity Status (~50%)

**Done (M1-M4):**
- SeedOptions threading, load ordering fix
- System struct extended (is_populated, total_energy, raw_materials)
- 3-system model replacing proximity spread
- Character stat rolling + named placement
- 8 tests (deterministic, seed-reproducible)

**Remaining (M5-M8):**
- M5: Support/popularity initialization (SDPRTB 7682-7685, GNPRTB 7764-7765)
- M6: Energy, raw materials, procedural facilities (GNPRTB 7721-7727)
- M7: Maintenance-budget common unit seeding (SDPRTB 5168-5170)
- M8: Integration wiring + regression tests

ExecPlan: `docs/plans/2026-03-24-003-game-seeding-parity-execplan.md`
Audit: `.subdaimon-output/seeding-parity-audit.md`

## 9 Seed Tables

| Table | Content | Target System |
|-------|---------|---------------|
| CMUNEFTB | Empire fleet composition | Coruscant |
| CMUNHQTB | Alliance HQ fleet | Rebel HQ |
| CMUNAFTB | Alliance troops (2 groups) | Yavin + Rebel HQ |
| FACLHQTB | HQ facilities | Coruscant + Rebel HQ |
| YAVNGRTB | Yavin garrison | Yavin |
| CMUNEMTB | Empire army distribution | Budget-based (M7) |
| CMUNALTB | Alliance army distribution | Budget-based (M7) |
| SYFCCRTB | Core facility generation | Per-system (M6) |
| SYFCRMTB | Rim facility generation | Per-system (M6) |
