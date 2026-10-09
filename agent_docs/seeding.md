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
| **Coruscant** | Empire HQ | Empire fleets (CMUNEFTB), facilities (FACLCRTB), Vader + Palpatine |
| **Yavin** | Alliance base | Alliance troops (CMUNAFTB group 1), garrison (CMUNYVTB), Luke/Leia/Han/Wedge/Chewie/Dodonna |
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

## Parity Status (~95%)

**All 8 milestones complete (M1-M8):**
- M1: SeedOptions threading, GNPRTB/SDPRTB load ordering
- M2: System struct extended (is_populated, total_energy, raw_materials)
- M3: 3-system model (Coruscant/Yavin/random Rebel HQ)
- M4: Character stat rolling + named placement (8 named characters)
- M5: Support/popularity initialization from SDPRTB 7682-7685 + GNPRTB 7764-7765
- M6: Energy/raw materials + procedural facility generation from SYFCCRTB/SYFCRMTB
- M7: Maintenance-budget common unit seeding from SDPRTB 5168-5170
- M8: Integration wiring (GameSetup → SeedOptions → loader) + regression tests
- 23 seeding-specific tests (deterministic, seed-reproducible)

**Facility sides.** Every seeded facility takes its system's holder once
control is final (`seeds::assign_facility_sides`, load step 8b'); a neutral
system's facilities serve nobody, and facilities no longer count toward
control inference. Family `0x2c` is the mine and `0x2d` the refinery. A later
control change hands each facility to the new holder or removes it when its
class cannot serve that side (`GameWorld::hand_over_facilities`). Source:
`ghidra/notes/facility-ownership.md`.

**Confirmed complete.** Rim systems excluded from maintenance-budget seeding — verified against TheArchitect2018 `seed.js` Section 10: `fetch_galaxy(session, side, ...)` only returns faction-controlled systems. Uncontrolled rim systems are excluded from the seed pool in the original game.

**Starting forces (2026-10-09).** The budget pass spends a side's spare
maintenance, `max(capacity − load, 0) × SDPRTB 5168..5170 / 100`, on
CMUNALTB/CMUNEMTB groups picked by a 1–100 roll against each group's key,
costs every item by its class maintenance, and stops at the first group it
cannot afford; so it never takes a side below zero. The garrisons run before
it, and the classes load before seeding so it can price them. CMUNALTB and
CMUNEMTB feed nothing else: the old step that placed every group at the
capitals had no original. A side holding few systems (Easy, or Hard as the
Empire: 3) can still start overdrawn, from its fixed seeds and rolled yards
alone. Source: `ghidra/notes/starting-forces-seeding.md`.

ExecPlan: `docs/plans/2026-03-24-003-game-seeding-parity-execplan.md`
Audit: `.subdaimon-output/seeding-parity-audit.md`

## Seed Tables

Names from REBEXE RCDATA, read by the unit seeding `FUN_0051aa50` in this
order (`ghidra/notes/starting-forces-seeding.md`):

| Table | Content | Target |
|-------|---------|--------|
| — | Low-support garrisons (`FUN_0051ab20`) | Every held system, first |
| CMUNYVTB | Yavin forces | Yavin |
| CMUNHQTB | HQ forces | Rebel HQ |
| CMUNCRTB | Coruscant forces | Coruscant |
| CMUNAFTB | Alliance fleets | Yavin + Rebel HQ |
| CMUNEFTB | Empire fleet | Coruscant |
| CMUNALTB / CMUNEMTB | Budget bundles (`FUN_0051b1f0`) | Random held systems, last |

Facilities come from other tables, outside `FUN_0051aa50`:

| Table | Content | Target |
|-------|---------|--------|
| FACLHQTB / FACLCRTB | HQ and Coruscant facilities (registry ids `0x55`/`0x56`) | Rebel HQ, Coruscant |
| SYFCCRTB / SYFCRMTB | Facility per energy slot (`FUN_00566de0`, a system method) | Per system (M6) |
