---
title: "feat: War Machine — Combat + Victory + Save/Load (v0.4.0)"
description: "Combat, victory conditions, and save/load implementation for v0.4.0"
category: "plan"
status: "complete"
created: 2026-03-15
updated: 2026-03-16
tags: ["combat", "victory", "save-load", "v0.4.0"]
---

# War Machine: Combat + Victory + Save/Load

## Context

Open Rebellion v0.3.0 is at ~42% completion. Living Galaxy and War Room are COMPLETE — all peacetime strategy mechanics work. Ghidra RE of REBEXE.EXE is **COMPLETE**: 5,127 functions decompiled, 111 GNPRTB parameters mapped, bombardment formula decoded, space combat 7-phase pipeline traced, C++ class hierarchy reconstructed, 7 scholar documents (6,049 lines) reviewed at 90%+ accuracy.

War Machine is **UNBLOCKED**. All combat formulas are documented. Implementation can proceed.

## Phase A: Ghidra RE — COMPLETE

Results:
- 5,127 decompiled C files (every function >100 bytes in REBEXE.EXE)
- Bombardment formula: `damage = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²) / GNPRTB[0x1400]`
- Space combat: 7-phase pipeline (weapon→shield→hull→fighter→result), vtable dispatch
- Ground combat: troop iteration at +0x96, per-unit via vtable +0x330
- 111 GNPRTB parameters: 34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side)
- Game object layout: hull(+0x60), shield/weapon nibbles(+0x64), strength(+0x96), alive(+0xac)
- C++ hierarchy: CRebObject → CNotifyObject → CCombatUnit → CCapitalShip/CFighterSquadron/CDeathStar
- Mission dispatch: FUN_0050d5a0 13-case switch with entity family range filters
- All stored in `ghidra/notes/` with modular docs and indexes

## Phase B: Knesset Tiamat — Implementation

### Prerequisites (Task 0 — lead or first daborit)

Before any combat code compiles, the world model needs expansion. From `rust-implementation-guide.md`:

1. **CapitalShipClass** needs 15 new fields promoted from DAT:
   - Weapon arcs: `turbolaser[4]`, `ion_cannon[4]`, `laser_cannon[4]` (4 arcs × 3 types)
   - Shield: `shield_strength`, `shield_recharge_rate`
   - Combat: `bombardment`, `damage_control`, `detection`, `maneuverability`

2. **New types**:
   - `ShipInstance` — per-hull combat state (hull_current, shield_weapon_packed, alive) replacing count-only `ShipEntry`
   - `GnprtbParams` — loaded from GNPRTB.DAT, accessed as `world.gnprtb[param_id].value(difficulty)`
   - `MstbTable` — loaded from *MSTB.DAT files, `table.lookup(skill_score) -> probability`

3. **GameWorld additions**:
   - `gnprtb: GnprtbParams`
   - `mission_tables: HashMap<String, MstbTable>`
   - Promote GNPRTB/MSTB loading in `rebellion-data/src/lib.rs`

### Team: Knesset Tiamat (4 daborot)

| Daborit | Role | File Ownership | Must Read |
|---------|------|---------------|-----------|
| **Tehom** | Space + ground combat | `rebellion-core/src/combat.rs`, `rebellion-core/src/bombardment.rs` | `ghidra/notes/space-combat.md`, `ground-combat.md`, `bombardment.md`, `rust-implementation-guide.md` §2-4 |
| **Al-Uzza** | 7 mission types + research + Jedi | `rebellion-core/src/espionage.rs`, `rebellion-core/src/research.rs` | `ghidra/notes/mission-event-cookbook.md`, `entity-system.md` §1, `rust-implementation-guide.md` §5 |
| **Manat** | Victory + Death Star + blockades + uprising | `rebellion-core/src/victory.rs`, `rebellion-core/src/blockade.rs` | `ghidra/notes/economy-systems.md`, `entity-system.md` §4, `COMBAT-SUMMARY.md` |
| **Karme** | World model expansion + save/load + combat UI | `rebellion-core/src/world/mod.rs` (field additions), `rebellion-data/src/save.rs`, `rebellion-render/src/combat_view.rs` | `ghidra/notes/cpp-class-hierarchy.md`, `annotated-functions.md`, `rust-implementation-guide.md` §1 |

### Task Breakdown (24 tasks)

#### Karme — World Model + Save/Load (6 tasks, START FIRST — blocks others)
1. Promote 15 CapitalShipClass DAT fields to world model (weapon arcs, shields, bombardment, detection)
2. Create `ShipInstance` type with per-hull combat state (hull_current: i32, shield_weapon_packed: u8, alive: bool)
3. Create `GnprtbParams` struct, load from GNPRTB.DAT in rebellion-data, add to GameWorld
4. Create `MstbTable` struct, load all 19 *MSTB.DAT files in rebellion-data, add to GameWorld
5. Save/load: bincode serialization of GameWorld + all simulation state to file
6. Save/load UI: egui panel with save slots, auto-save on quit

#### Tehom — Combat (6 tasks, after Karme #1-2)
7. Space combat auto-resolve: `CombatSystem::resolve_space()` following the 7-phase pipeline from `space-combat.md`
   - Phase gate flags as `CombatPhaseFlags` bitfield (ACTIVE=0x01, WEAPON_TYPE=0x04, SHIELD_TYPE=0x20, HULL_TYPE=0x80, FIGHTER_TYPE=0x100)
   - Inner gate mask via type code, outer wrapper via ACTIVE & ~PHASES_ENABLED
   - Per-side resolution with caller-provided RNG rolls
8. Weapon damage: iterate attacker's ships, sum turbolaser + ion + laser per arc, apply to defender's shields then hull
9. Shield absorption + hull damage: shield absorbs up to shield_strength, remainder hits hull. Shield recharge per tick (4-bit nibble from +0x64 bits 0-3)
10. Fighter engagement: squadron-vs-squadron, torpedo damage to capital ships
11. Ground combat: `CombatSystem::resolve_ground()` — iterate troops at system, compare attack vs defense ratings at +0x96
12. Orbital bombardment: `sqrt((atk_bombardment - def_defense)²) / gnprtb[0x1400]` with difficulty modifier from `gnprtb.value(difficulty)`

#### Al-Uzza — Missions + Research (6 tasks, after Karme #3-4)
13. Extend `MissionKind` enum from 2 to 9 variants: Espionage, Sabotage(6), Assassination(7), Rescue(7), Abduction, InciteUprising, SubdueUprising, DeathStarSabotage
14. Implement MstbTable probability lookup: `table.lookup(skill_score)` interpolates between threshold entries
15. Wire all 19 *MSTB probability tables to mission types (DIPLMSTB→Diplomacy, ESPIMSTB→Espionage, ASSNMSTB→Assassination, etc.)
16. Research system: `ResearchSystem::advance()` — tech tree progression using research_order/research_difficulty from DAT
17. Jedi training: ForceAware→ForcePotential→ForceTraining→ForceExperience progression (4 tiers from entity_system.md, `entity[9]>>6&3`)
18. Death Star sabotage mission using DSSBMSTB probability table

#### Manat — Victory + Mechanics (6 tasks, independent)
19. Victory condition detection: `VictorySystem::check()` — HQ capture (check system ownership + IsHeadquarters flag), conquest (all systems)
20. Death Star: construction via manufacturing queue (buildable entity), movement, planet destruction (fires when `alive_flag bit0 == 0` on target — from annotated-functions.md)
21. Blockade mechanics: fleet presence blocks manufacturing (from economy-systems.md: `FUN_0050e820` production eligibility gate), `TroopRegDestroyedRunningBlockade` event 0x340
22. Uprising system: `UprisingSystem::advance()` — incite uses UPRIS1TB (3 thresholds), subdue uses UPRIS2TB (4 thresholds), fires at loyalty thresholds, blocked on `is_populated` bit at +0x88
23. Combat results UI: message log integration for battle outcomes (space/ground/bombardment results)
24. Victory/defeat screen: egui modal with faction-specific message

### File Ownership Matrix

| Daborit | Owns | Must NOT Touch |
|---------|------|----------------|
| Tehom | `rebellion-core/src/combat.rs`, `bombardment.rs` | rebellion-render/, rebellion-data/ |
| Al-Uzza | `rebellion-core/src/espionage.rs`, `research.rs`, extend `missions.rs` | rebellion-render/, rebellion-data/ |
| Manat | `rebellion-core/src/victory.rs`, `blockade.rs`, `uprising.rs`, `rebellion-render/src/combat_view.rs` | rebellion-data/ |
| Karme | `rebellion-core/src/world/mod.rs` (field additions only), `rebellion-data/src/save.rs`, `rebellion-data/src/lib.rs` (GNPRTB/MSTB loading) | rebellion-render/ (except save UI) |

### Key References for Daborot

| Document | Lines | Essential For |
|----------|-------|--------------|
| `ghidra/notes/rust-implementation-guide.md` | 1,267 | ALL daborot — Rust struct layouts, advance() pattern, formula translations |
| `ghidra/notes/space-combat.md` | 81 | Tehom — 7-phase pipeline, vtable dispatch table, phase flags |
| `ghidra/notes/bombardment.md` | 77 | Tehom — Euclidean distance formula, GNPRTB divisor |
| `ghidra/notes/ground-combat.md` | 69 | Tehom — troop iteration, per-unit resolution, Death Star handler |
| `ghidra/notes/mission-event-cookbook.md` | 724 | Al-Uzza — 13-case dispatch, MSTB tables, story events, modder guide |
| `ghidra/notes/entity-system.md` | 668 | Al-Uzza — Force/Jedi tiers, character skills, mission roles |
| `ghidra/notes/economy-systems.md` | 478 | Manat — resources, control kinds, uprising, blockade, repair |
| `ghidra/notes/cpp-class-hierarchy.md` | 445 | Karme — struct layouts, vtable slots, field offsets for world model |
| `ghidra/notes/annotated-functions.md` | 1,662 | ALL — game rules, validation ranges, event IDs |
| `agent_docs/simulation.md` | 175 | ALL — existing advance() pattern to follow |

### Accuracy Notes (from reviewer corrections)

- Weapon fire inner gate mask is **0x04 (bit2)**, not bit0 — bit0 is the outer wrapper
- Shield/weapon packed field at +0x64 uses **XOR-mask write** `(new ^ old) & 0xf ^ old`
- Shield alt-path triggers on family **== 0x71 exactly**, not range 0x71-0x72
- Death Star alive_flag: `(bit0 == 0)` means eligible for destruction (inverted from combat units)
- Autoscrap mission type code is **0x15 (21)**, not 0
- Rescue mission (case 7) ranges are `0x08-0x10, 0x01-0xff, 0x01-0xff` — no 0x10-0x20 range
- Ground combat validates **troops (0x14-0x1b)**, not characters (0x08-0x0f)

## Verification

1. `PATH="/usr/bin:$PATH" cargo check` — full workspace compiles
2. `PATH="/usr/bin:$PATH" cargo test --workspace` — all tests pass (target: 150+)
3. Start game → dispatch espionage mission → see success/failure with correct probability
4. Move fleet to enemy system → space combat auto-resolves → message log shows outcome
5. Build Death Star → move to enemy HQ → destroy planet → victory screen
6. Save game → quit → reload → game state preserved
7. AI dispatches espionage missions against player
8. Research progresses → new ship classes unlock
9. Victory by HQ capture triggers end screen
10. `bash scripts/build-wasm.sh` — WASM build succeeds

## Estimated Completion After v0.4.0

| Dimension | Before | After | Delta |
|-----------|--------|-------|-------|
| Data Layer | 75% | 90% | +15% (GNPRTB loaded, weapon fields promoted, MSTB tables) |
| Simulation | 30% | 80% | +50% (combat, 9 mission types, research, victory, uprising) |
| UI/Rendering | 50% | 65% | +15% (combat results, save/load, victory screen) |
| Infrastructure | 45% | 70% | +25% (save/load, GNPRTB runtime, MSTB runtime) |
| Game Completeness | 15% | 65% | +50% (can fight, win, lose, save, research) |
| **Overall** | **42%** | **~68%** | **+26%** |
