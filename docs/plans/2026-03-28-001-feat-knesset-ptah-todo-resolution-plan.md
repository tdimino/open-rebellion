---
title: "Knesset Ptah — TODO Resolution Sprint"
type: feat
status: active
date: 2026-03-28
---

# Knesset Ptah — TODO Resolution Sprint

## Overview

Resolve all 13 TODOs in the Open Rebellion codebase, plus fix 2 telemetry emission bugs (SYS_STORY tagging, SYS_REPAIR empty advance) and add fixture injections for 100% telemetry coverage.

Named after Ptah, the Egyptian craftsman-god — patron of builders who finish what they start.

## Problem Statement

The codebase contains 13 TODOs across 5 files. Two are correctness bugs (#9 victory state, #10 entity cleanup). Two are economy calculation errors (#12 wrong facility arena, #13 wrong facility type). The rest range from missing fields to UI wiring to architectural promotions. Additionally, the telemetry coverage test achieves only 12/17 (70%) due to 2 emission bugs and 3 fixture gaps.

## Dependency Graph

```
Tier 0: Correctness Bugs (P0, no deps)
  #9 victory_state.death_star_location ──┐
  #10 destroyed system entity cleanup ───┤
                                         │
Tier 1: Field Additions (depends on #10 for save version batching)
  #3 System::espionage_rating ───────────┤
  #4 espionage_rating test ──────────────┤ (save version bump to v6)
                                         │
Tier 2: UI Wiring (no deps)             │
  #5 save slot population ──────────────┤
  #6 load game screen transition ───────┤
  #7 mission kind pre-selection ────────┤
  #8 fleet move selection flow ─────────┘

Tier 3: Facility Type Promotion (no deps, parallel with Tier 2)
  #12 mine vs non-mine distinction
  #13 shipyard class distinction

Tier 4: ShipInstance Promotion (SEPARATE SPRINT — Knesset Hephaestus)
  #11 per-ship hull tracking → real repair events
  Touches ~20 files, save format, combat, rendering, AI

Blocked: Ghidra RE data needed
  #1 weapon fire exact formula (combat.rs:350)
  #2 superlaser stub → DELETE (dead code, no callers)

Telemetry Coverage (parallel with Tier 0)
  SYS_STORY tagging bug in integrator.rs
  SYS_REPAIR empty advance in repair.rs
  Fixture injections: uprising, death_star, betrayal
```

## Scope Boundaries

- **In scope**: TODOs #1-#10, #12-#13, telemetry 17/17, dead code removal (#2)
- **Out of scope**: ShipInstance promotion (#11) — deferred to Knesset Hephaestus
- **Out of scope**: Full facility class definition loading from MANFACSD.DAT — use boolean `is_mine`/`is_shipyard` fields as interim solution
- **NOT doing**: Ghidra RE for #1 (weapon fire formula) — mark as known approximation, remove TODO

## Implementation Phases

### Phase 0: Telemetry Coverage + Dead Code (no save format change)

**Goal**: Reach 17/17 SYS_* telemetry coverage. Fix 2 emission bugs, add 3 fixture injections, delete 1 dead code stub.

#### Unit 0a: Fix SYS_STORY tagging bug

`apply_fired_events()` in integrator.rs tags ALL fired events as `SYS_EVENTS`. Story events (IDs 0x210-0x39A) should use `SYS_STORY`.

**Files**: `crates/rebellion-data/src/integrator.rs`
**Approach**: Add `fn is_story_event(id: u32) -> bool` that matches 0x210, 0x212, 0x220-0x221, 0x380-0x39A. Use `SYS_STORY` when true, `SYS_EVENTS` otherwise.
**LOC**: ~15
**Test**: Story events in telemetry output tagged as "story" not "events"

#### Unit 0b: Fix SYS_REPAIR empty advance

`RepairSystem::advance()` validates preconditions but never emits events. Add `RepairCheckPerformed` variant.

**Files**: `crates/rebellion-core/src/repair.rs`, `crates/rebellion-data/src/integrator.rs`
**Approach**: Add `RepairCheckPerformed { system, fleet, ships_checked }` variant. Emit when fleet at shipyard has ships with `damage_control > 0`. Update integrator to handle the new variant.
**LOC**: ~20
**Test**: Repair events appear in telemetry when fleet at shipyard
**Note**: Keep TODO for ShipInstance promotion — `RepairCheckPerformed` is the interim telemetry solution.

#### Unit 0c: Fixture injections for uprising/death_star/betrayal

Add world state injection to `telemetry_coverage.rs` after loading but before tick loop.

**Files**: `crates/rebellion-data/tests/telemetry_coverage.rs`
**Approach**:
- Uprising: Tank one non-HQ Alliance system to `popularity_alliance = 0.05`
- Death Star: Set `states.death_star.under_construction = Some(DeathStarConstruction { system, ticks_remaining: 5 })`
- Betrayal: Set one non-immune minor character's `loyalty.base = 5`
**LOC**: ~25
**Test**: All 17 SYS_* constants in required list (remove OPTIONAL_SYSTEMS)

#### Unit 0d: Delete dead code stub (#2)

`resolve_death_star()` in combat.rs is never called. Death Star fires through `DeathStarSystem::fire()`.

**Files**: `crates/rebellion-core/src/combat.rs`
**Approach**: Remove `resolve_death_star()` function and its TODO. Add comment explaining DS resolution path.
**LOC**: -15 (net deletion)
**Verification**: `cargo test`, grep for callers confirms zero

#### Unit 0e: Resolve weapon fire TODO (#1)

The ±20% variance approximation in `resolve_weapon_fire` is a known simplification, not a bug.

**Files**: `crates/rebellion-core/src/combat.rs`
**Approach**: Replace TODO with a comment documenting the approximation and citing the Ghidra function address for future exact implementation.
**LOC**: ~3 (comment change only)

**Phase 0 acceptance**: `cargo test -p rebellion-data --test telemetry_coverage -- --ignored` passes with 17/17 required systems. Zero OPTIONAL_SYSTEMS.

---

### Phase 1: Correctness Bugs (#9, #10)

**Goal**: Fix Death Star victory path and destroyed system entity cleanup.

#### Unit 1a: Fix victory_state.death_star_location (#9)

**CRITICAL BUG**: `VictorySystem::check_death_star()` short-circuits to `None` when `death_star_location` is unset. The Empire can never win by Death Star in either interactive or headless mode.

**Files**: `crates/rebellion-app/src/main.rs`, `crates/rebellion-data/src/integrator.rs`
**Approach**:
- main.rs: In `PanelAction::FireDeathStar` handler, set `victory_state.death_star_location = Some(target_system)` before marking system destroyed. Requires threading `&mut VictoryState` into the handler (currently only `&VictoryState`).
- integrator.rs: In `apply_death_star_events`, set `victory_state.death_star_location` on `DeathStarEvent::Fired`.
**LOC**: ~10
**Test**: New test in victory.rs verifying DS fire → victory check passes

#### Unit 1b: Destroyed system entity cleanup (#10)

When a system is destroyed (Death Star fire), all entities at that system must be resolved.

**Files**: `crates/rebellion-core/src/death_star.rs` (new cleanup function), `crates/rebellion-app/src/main.rs`, `crates/rebellion-data/src/integrator.rs`
**Approach**: Create `pub fn cleanup_destroyed_system(world: &mut GameWorld, system: SystemKey, movement: &mut MovementState)` that:

1. **Fleets at system**: Remove from `world.fleets` arena. Characters in those fleets are killed (removed from `world.characters`).
2. **Fleets in transit TO system**: Cancel `MovementOrder` entries where `destination == system`. Fleet remains at origin. Emit `FleetReturnedToOrigin` event.
3. **Ground units**: Remove from `world.troops` arena.
4. **Special forces**: Remove from `world.special_forces` arena.
5. **Defense facilities**: Remove from `world.defense_facilities` arena.
6. **Manufacturing facilities**: Remove from `world.manufacturing_facilities` arena.
7. **Production facilities**: Remove from `world.production_facilities` arena.
8. **Manufacturing queues**: Clear any `ManufacturingState` queues keyed to this system.
9. **Blockade state**: Clear any blockade at this system.
10. **AI targeting**: Clear any pending attack targets at this system.
11. **System vectors**: Clear `sys.fleets`, `sys.ground_units`, `sys.special_forces`, `sys.defense_facilities`, `sys.manufacturing_facilities`, `sys.production_facilities`.

**Design decisions**:
- Characters at destroyed system are **killed** (matching original game behavior)
- Fleets in transit TO destroyed system are **rerouted to origin** (cancel order)
- Fleets in transit FROM destroyed system keep their `origin` field (cosmetic only, harmless)
- Death Star construction at destroyed system is cleared if `DeathStarConstruction.system == destroyed_system`

**LOC**: ~80
**Test**: New test in death_star.rs: set up system with fleets/troops/facilities, destroy it, verify all entities removed from arenas and system vectors empty. Verify in-transit fleet orders cancelled.

**Phase 1 acceptance**: Death Star fire → victory check succeeds. Destroyed systems have no orphaned entities.

---

### Phase 2: Field Additions + Save Version (#3, #4)

**Goal**: Add `espionage_rating` to System, bump save format to v6.

#### Unit 2a: Add espionage_rating field

**Files**: `crates/rebellion-core/src/world/mod.rs`, `crates/rebellion-core/src/missions.rs`, `crates/rebellion-data/src/save.rs`
**Approach**:
- Add `pub espionage_rating: f32` with `#[serde(default)]` to `System` struct
- In `compute_table_input` for `MissionKind::InciteUprising`, subtract `system.espionage_rating` from the composite input
- Bump `SAVE_VERSION` to 6, add v5 rejection arm: `5 => return Err(anyhow!("Save v5 incompatible: System struct extended with espionage_rating. Please start a new game."))`
- Populate from DAT data during loading (check if SYSTEMSD.DAT has this field, otherwise default 0.0)
**LOC**: ~25
**Test**: Update stubbed test in missions.rs:2031 to verify espionage_rating subtraction

**Phase 2 acceptance**: `espionage_rating` wired into mission probability. Save v5 rejected with clear message.

---

### Phase 3: UI Wiring (#5, #6, #7, #8)

**Goal**: Wire save/load screen and context menu panel actions.

#### Unit 3a: Save slot population (#5)

**Files**: `crates/rebellion-app/src/main.rs`
**Approach**: Replace `let save_slots: Vec<SaveSlotInfo> = vec![];` with `let save_slots = rebellion_data::save::list_saves(&save_dir)`. Refresh when save/load panel opens.
**LOC**: ~10

#### Unit 3b: Load game screen transition (#6)

**Files**: `crates/rebellion-app/src/main.rs`
**Approach**: In `MainMenuAction::LoadGame` handler, set `show_save_load = true` and transition to `AppMode::Galaxy` with save panel visible, instead of loading directly.
**LOC**: ~15

#### Unit 3c: Mission kind pre-selection (#7)

**Files**: `crates/rebellion-render/src/panels/missions.rs`, `crates/rebellion-app/src/main.rs`
**Approach**: Add `preselected: Option<(MissionKind, SystemKey)>` to `MissionsPanelState`. In `PanelAction::OpenMissionTo` handler, set both kind and target. Missions panel draw reads `preselected` and auto-selects dropdowns.
**LOC**: ~25

#### Unit 3d: Fleet move selection flow (#8)

**Files**: `crates/rebellion-render/src/panels/fleets.rs`, `crates/rebellion-app/src/main.rs`
**Approach**: Add `pending_move_destination: Option<SystemKey>` to `FleetsPanelState`. In `PanelAction::InitiateFleetMove` handler, open fleets panel with destination pre-set. Panel shows eligible fleets with "Move" button that issues `MovementOrder`.
**LOC**: ~40

**Phase 3 acceptance**: Save slots populated from disk. Load game transitions to save panel. Context menu → missions panel pre-selects kind+target. Context menu → fleet panel shows move destination.

---

### Phase 4: Facility Type Promotion (#12, #13)

**Goal**: Correct economy calculations that use wrong facility arenas/types.

#### Unit 4a: Add facility type fields

**Files**: `crates/rebellion-core/src/world/mod.rs`
**Approach**: Add `pub is_mine: bool` to `ProductionFacilityInstance` (default false). Add `pub is_shipyard: bool` to `ManufacturingFacilityInstance` (default false). Both with `#[serde(default)]`.
**LOC**: ~5
**Note**: This is the interim boolean approach. Full class definition map (ManufacturingFacilityClassDef) deferred — would require MANFACSD.DAT field promotion and another save version bump.

#### Unit 4b: Populate during loading

**Files**: `crates/rebellion-data/src/lib.rs`
**Approach**: During facility loading, check `production_family` field from DAT record to set `is_mine`/`is_shipyard`. The dat-dumper already parses `production_family` on `ManufacturingFacility`.
**LOC**: ~15

#### Unit 4c: Fix economy calculations (#12, #13)

**Files**: `crates/rebellion-core/src/economy.rs`
**Approach**:
- #12 (line 603): Replace `manufacturing_facilities.len()` with count of `production_facilities` where `is_mine == true` for raw material allocation
- #13 (line 679): Replace `!sys.defense_facilities.is_empty()` with `sys.manufacturing_facilities.iter().any(|k| world.manufacturing_facilities.get(*k).map_or(false, |f| f.is_shipyard))`
**LOC**: ~20
**Test**: New tests verifying mine count vs total manufacturing count, and shipyard detection from correct arena

**Phase 4 acceptance**: Raw materials allocated from mines only. Shipyard detection queries manufacturing facilities, not defense facilities.

---

## Deferred Work

### Knesset Hephaestus: ShipInstance Promotion (#11)

**Scope**: Change `Fleet.capital_ships` from `Vec<ShipEntry>` to `Vec<ShipInstance>`. Per-ship hull tracking enables real `ShipRepaired` events with actual deltas.

**Blast radius** (~20 files):
- `world/mod.rs`: Fleet struct change
- `combat.rs`: Remove ShipEntry→ShipSnap expansion (ShipInstance is already per-hull)
- `movement.rs`: Hyperdrive calculation from per-instance classes
- `economy.rs` / `ai.rs`: Garrison strength from instance count
- `manufacturing.rs`: Build completion creates ShipInstance
- `seeds.rs`: Initial fleet composition creates per-hull instances
- `repair.rs`: Real ShipRepaired events with hull deltas
- `save.rs`: Save version bump (v7 if after this sprint, or batch with v6)
- `rebellion-data/src/lib.rs`: Fleet loading creates per-hull instances
- `rebellion-render/src/panels/fleets.rs`: Display grouped by class
- `rebellion-render/src/tactical_view.rs`: Per-ship display
- `rebellion-render/src/encyclopedia.rs`: Fleet composition display
- ~10 test files with fleet fixtures

**Estimated LOC**: ~500
**Save format**: Version bump required (v7 or batched)
**Recommendation**: Dedicated sprint with its own plan document

### Ghidra RE: Weapon Fire Formula (#1)

The ±20% variance at combat.rs:350 is a reasonable approximation. The exact vtable +0x1c4 formula requires additional Ghidra RE work on the weapons fire resolver. Low priority — the approximation produces correct gameplay feel.

**Action**: Replace TODO with documentation comment, mark as known approximation.

---

## Verification

1. `PATH="/usr/bin:$PATH" cargo test` — all tests pass after each phase
2. `PATH="/usr/bin:$PATH" cargo test -p rebellion-data --test telemetry_coverage -- --ignored` — 17/17 required after Phase 0
3. `grep -r "TODO" crates/` — only #11 (ShipInstance, explicitly deferred) remains after all phases
4. Death Star fire → victory detection works in both interactive and headless mode (Phase 1)
5. Save v5 → v6 rejection with clear message (Phase 2)

## Success Metrics

| Phase | Gate | Test Count |
|-------|------|-----------|
| Phase 0 | 17/17 telemetry, #2 deleted | +5 tests |
| Phase 1 | DS victory works, entities cleaned | +4 tests |
| Phase 2 | espionage_rating wired, save v6 | +3 tests |
| Phase 3 | Save/load, mission/fleet pre-select | +4 tests |
| Phase 4 | Economy correct facilities | +3 tests |
| **Total** | **0 TODOs remaining** (except #11 deferred) | **+19 tests (~420 total)** |
