---
title: "Game Seeding Pipeline"
description: "Initial galaxy state generation: character placement, fleet distribution, support initialization"
category: "agent-docs"
created: 2026-03-25
updated: 2026-10-19
tags: [seeding, initialization, GNPRTB, SDPRTB, seeds]
---

# Game Seeding Pipeline

## Overview

`crates/rebellion-data/src/seeds.rs` handles initial galaxy state generation when starting a new game. The pipeline processes 9 seed tables from DAT files and applies them to the 3 special systems.

## 3-System Model

| System | Role | Fixed Assets |
|--------|------|-------------|
| **Coruscant** | Empire HQ | Empire fleet (CMUNEFTB), garrison (CMUNCRTB), facilities (FACLCRTB), Vader + Palpatine |
| **Yavin** | Alliance base | Alliance fleet (CMUNAFTB group 1), garrison (CMUNYVTB), Luke/Leia/Han/Wedge/Chewie/Dodonna |
| **Random Rim** | Rebel HQ | Alliance fleet (CMUNAFTB group 2), garrison (CMUNHQTB), facilities (FACLHQTB), Mon Mothma |

Rebel HQ is selected via `select_special_systems()`, deterministic with the RNG seed, always a rim system other than Yavin (SectorGroup::RimInner or RimOuter).

## Key Functions

- `apply_seeds_with_rng(gdata_path, world, system_key_map, seed_options, rng)`: main entry point
- `select_special_systems(world, system_key_map, rng) -> Option<SpecialSystems>`: picks the 3 systems
- `roll_character_stats(world, rng)`: rolls SkillPair{base, variance} → concrete values
- `place_named_characters(world, special)`: sets current_system for 8+ named characters
- `initialize_special_systems(world, special)`: sets populated, charted, controlled, support

## SeedOptions

`crates/rebellion-core/src/world/mod.rs`:

```rust
pub struct SeedOptions {
    pub galaxy_size: GalaxySize,
    pub difficulty: SeedDifficulty,
    pub player_faction: Faction,
    pub rng_seed: Option<u64>,
}
```

Threaded from Game Setup screen → `load_game_data()` → `apply_seeds_with_rng()`.

## Seeding Rules

**Facility sides.** Every seeded facility takes its system's holder once
control is final (`seeds::assign_facility_sides`, load step 8b'); a neutral
system's facilities serve nobody, and facilities do not count toward
control inference. Family `0x2c` is the mine and `0x2d` the refinery. A later
control change hands each facility to the new holder or removes it when its
class cannot serve that side (`GameWorld::hand_over_facilities`). Source:
`ghidra/notes/facility-ownership.md`.

**Rim systems.** Rim systems are excluded from maintenance-budget seeding, as in TheArchitect2018 `seed.js` Section 10, where `fetch_galaxy(session, side, ...)` returns only faction-controlled systems. Uncontrolled rim systems are excluded from the seed pool in the original game.

**Starting forces.** The budget pass spends a side's spare
maintenance, `max(capacity − load, 0) × SDPRTB 5168..5170 / 100`, on
CMUNALTB/CMUNEMTB groups picked by a 1–100 roll against each group's key,
costs every item by its class maintenance, and stops at the first group it
cannot afford; so it never takes a side below zero. The garrisons run before
it, and the classes load before seeding so it can price them. CMUNALTB and
CMUNEMTB feed nothing else. A side holding few systems (Easy, or Hard as the
Empire: 3) can still start overdrawn, from its fixed seeds and rolled yards
alone. Source: `ghidra/notes/starting-forces-seeding.md`.

**Support.** `initialize_support` seeds one 0-100 value per system: a held
system's holder gets it and the other side 100 minus it; a neutral system's
value is the Alliance's and the Empire's is 100 minus it (`FUN_00559700`
returns one value, system `+0x58`). Held core systems roll SDPRTB
7682 + rand 7683 (strong) or 7684 + rand 7685 (weak), core neutral systems
rand(GNPRTB 7764) + 50 − 7764/2 (`FUN_00559700` case 3), populated rim
systems rand(GNPRTB 7765) + 50, unpopulated systems 50; the three special
systems start at 100 for their holder (`initialize_special_systems`).

**System squadrons.** A seed group's squadrons board its lead
when the lead is a capital ship and otherwise stay on the system
(`FUN_0051bf30`), held in `System::fighters` per class and side. Every
squadron of a ship-led group rides that lead ship, with no room test and no
choice of ship (`FUN_004f7d50`; `board_lead` in `seeds.rs`). CMUNCRTB
gives Coruscant 10 TIEs, CMUNYVTB Yavin 2, CMUNHQTB the HQ 6; lone budget
squadrons land on their system too. A finished squadron also lands on its
system (`FUN_0052bee0`); no fighter-only fleet is made.

ExecPlan: `docs/plans/completed/2026-03-24-003-game-seeding-parity-execplan.md`

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
| SYFCCRTB / SYFCRMTB | Facility per energy slot (`FUN_00566de0`, a system method) | Per system |
