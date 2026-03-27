---
title: "Community Disassembly Cross-Reference Report"
date: 2026-03-26
session: 30258820
model: claude-opus-4-6
source: disassembly.zip (community RE effort)
functions_analyzed: 13,036
agents: 4 (combat-analyst, ai-mission-analyst, seeding-economy-analyst, events-save-entity-analyst)
status: complete
tags: [ghidra, reverse-engineering, parity, cross-reference]
---

# Community Disassembly Cross-Reference Report

## Executive Summary

A community reverse-engineering effort produced 13,036 decompiled C functions from REBEXE.EXE, more than double our own Ghidra campaign's 5,151 functions. 1,471 functions carry human-readable labels. Four parallel research agents cross-referenced the dump against our Rust implementation across combat, AI/missions, seeding/economy, and events/entities.

**Overall parity: ~85%.** The core simulation systems are faithfully implemented. The single biggest gap is the per-system economy tick loop (18 sub-functions), which governs popular support drift, collection rates, garrison requirements, and resource allocation. The community dump also reveals the complete multiplayer protocol (178 `net_notify_*` message types).

## Source Validation

### Address Alignment

The community dump uses slightly different Ghidra analysis settings than our own project, producing:

- **Address offsets of 0x10-0x30 bytes** for the same logical functions (e.g., our `FUN_005029a0` hull damage setter appears at `FUN_005022d0` in the community dump)
- **Finer function granularity** — 13,036 vs 5,151 functions due to more aggressive function splitting (thunks, accessors, CRT helpers included)
- **Consistent logic** where both analyses cover the same code

### Verified Cross-References

| Our Address | Community Address | Function | Logic Match |
|-------------|------------------|----------|-------------|
| `FUN_0053e190` | Referenced identically | Difficulty modifier | Exact |
| `FUN_0053c9f0` | `FUN_0053c9f0_roll_dice` | RNG: `random_in_range(range + 1)` | Exact |
| `FUN_00518f00` | `FUN_00518f00_AutoClass3_seed_galaxy` | Galaxy bucketing master | Exact (24/0/6/3/27 core + 138 rim) |
| `FUN_0055d860` | `FUN_0055d880` (offset +0x20) | Bombardment power ratio | Same Euclidean formula |
| `FUN_004ff7f0` | `FUN_004ff7f0` | Fleet blockade notif | Exact address match |
| `FUN_005029a0` | `FUN_005022d0_ship_hull_damage` | Hull value getter | Same `field96_0x60` read |

### What the Community Dump Adds

| Category | Count | Our Previous | Delta |
|----------|-------|-------------|-------|
| Total functions | 13,036 | 5,151 | +7,885 |
| Human-readable labels | 1,471 | ~50 | +1,421 |
| Networking functions | 188 | 0 | +188 |
| Story event labels | ~60 | ~10 | +50 |
| Seeding labels | 20 | 2 | +18 |
| AI decision labels | 8 | 6 | +2 |

## Function Distribution

| Address Range | System | Functions |
|---------------|--------|-----------|
| `0x0040xxxx` | Init, Galaxy Map, UI, Dialogs, Game Logic, Save/Load, Multimedia | 4,539 |
| `0x0050xxxx` | Ships, Economy, AI, Combat, Missions, Tactical, DAT Loaders | 7,471 |
| `0x0060xxxx` | Networking, CRT Runtime | 1,026 |

Top labeled prefixes: `net_` (188), `thunk_` (181), `get_` (160), `init_` (100), `find_` (91), `event_` (53), `set_` (49), `load_` (47), `create_` (32), `mission_` (31), `seed_` (20).

---

## Domain Reports

### 1. Combat (90% Parity)

**Functions analyzed**: 54

#### Faithfully Implemented

| Original | Rust | Notes |
|----------|------|-------|
| Space combat orchestrator (7-phase) | `CombatSystem::resolve_space()` | Phase ordering matches: init, eval, weapon fire, shield absorb, hull damage, fighter engage, result |
| Shield absorption (`FUN_00544130`) | `phase_shield_absorb()` | Shield recharge, ion 2x multiplier, overflow to hull. Double-rounding fix applied. |
| Hull damage (`FUN_005443f0`) | `phase_hull_damage()` | Difficulty modifier via GNPRTB, min-1 damage, alive flag at hull=0 |
| Fighter engagement (`FUN_005444e0`) | `phase_fighter_engage()` | Launch/dogfight/recall cycle, carrier capacity gating |
| Bombardment (`FUN_00556430`) | `BombardmentSystem::resolve_bombardment()` | Euclidean distance / GNPRTB[0x1400], min-1 damage, self-bombardment guard |
| Ground combat (`FUN_00560d50`) | `CombatSystem::resolve_ground()` | Family-byte classification (0x14-0x1b), regiment strength, difficulty modifier |
| CombatPhaseFlags bitfield | `CombatPhaseFlags` struct | All bit masks (0x01-0x100) confirmed |

#### Partial / Divergent

| Area | Gap | Impact |
|------|-----|--------|
| **Per-arc weapon fire** | We aggregate all arcs into single total; original uses trigonometric quadrant selection per 4 arcs | Medium — affects tactical view damage distribution |
| **Fighter dogfight** | We use statistical power-ratio; original has per-unit 3D proximity with state machine (chase/ram/evade) | Low — auto-resolve approximation is acceptable |
| **Who Won logic** | Missing Emperor Palpatine special check (`FUN_00542050_is_emperor_palpatine`) | Medium — Emperor presence modifies battle outcomes |
| **Shield threshold** | `GENERAL_PARAM_CAT15_7683` not implemented; shields absorb all damage until depleted | Low |
| **Fighter-vs-ship shields** | We use `shield_nibble/15`; original uses vtable-dispatched calculation | Low |

#### Missing Systems

| System | Description | Impact |
|--------|-------------|--------|
| **Ship repair** | `CombatUnitUnderRepair` / `CombatUnitFastRepair` — damaged ships repaired at shipyard systems | High — affects multi-battle campaign attrition |
| **Death Star shield** | Entity family byte `0x25` — separate shield generator must be destroyed first | High — core gameplay mechanic |
| **Character combat rating** | `+0x86` offset not factored into ground combat; commanding officers should modify troop effectiveness | Medium |
| **Battle narrative text** | TEXTSTRA resource IDs (0x7001-0x7016) for nuanced outcome messages | Low |
| **36 GNPRTB combat params** | `0x1401-0x1445` with faction overrides — purpose unknown | Unknown |

### 2. AI & Missions (75% Parity)

**Functions analyzed**: 49

#### Faithfully Implemented

| Original | Rust | Notes |
|----------|------|-------|
| Diplomacy probability | `MissionKind::Diplomacy.compute_table_input()` | `DIPLMS_TABLE[(enemy_pop - our_pop) + diplomacy]` — exact |
| Espionage probability | `MissionKind::Espionage.compute_table_input()` | Direct `ESPIMS_TABLE[skill]` — exact |
| Subdue uprising probability | `MissionKind::SubdueUprising.compute_table_input()` | Same formula as diplomacy — exact |
| Death Star sabotage | `MissionKind::DeathStarSabotage.compute_table_input()` | `DSSBMS_TABLE[(espionage + combat) / 2]` — exact |
| Rescue probability | `MissionKind::Rescue.compute_table_input()` | Direct `RESCMS_TABLE[skill]` — exact |
| Mission type registry | `MissionKind` enum (11 variants) | All type codes mapped (0x51-0x6a) |
| Galaxy evaluation | `evaluate_galaxy_state()` | 7 buckets + control_ratio + aggression |
| Garrison strength | `system_strength()` | Ships + troops + facilities |

#### Divergent Formulas (P0 — incorrect probabilities)

| Mission | Original Formula | Our Formula | Fix |
|---------|-----------------|-------------|-----|
| **Sabotage** | `SBTGMS_TABLE[(espionage + combat) / 2]` | `SBTGMS_TABLE[espionage]` | Add combat to input |
| **Incite Uprising** | `INCTMS_TABLE[(diplomacy - pop_support) - espionage_rating]` | `INCTMS_TABLE[(enemy_pop - our_pop) + diplomacy]` | Add counter-intelligence factor |
| **Abduction** | `ABDCMS_TABLE[espionage - target_defense]` | `ABDCMS_TABLE[espionage]` | Subtract target defense |
| **Assassination** | `ASSNMS_TABLE[combat - target_defense]` | `ASSNMS_TABLE[combat]` | Subtract target defense |
| **Recruitment** | `RCRTMS_TABLE[leadership - target_resistance]` | Approximation from system values | Use character resistance, not system |

#### Missing AI Systems

| System | Description | Impact |
|--------|-------------|--------|
| **Decoy mission** | TDECOYTB/FDECOYTB lookup with `GNPRTB[3588]` penalty (35%). Agent draws enemy attention before real mission. | Medium — missing gameplay mechanic |
| **AI uprising prevention** | `FUN_00519d20` — AI proactively garrisons endangered systems | Medium |
| **AI resource balancing** | `FUN_00558660` — RESRC_TABLE with 4 outcomes (25% each) to adjust energy/materials | Medium |
| **Reconnaissance** | Mission type code `0x54` distinct from Espionage (`0x52`) — two variants | Low |
| **AI manufacturing randomization** | Original randomly distributes production across systems; we build at every idle system | Low |
| **Faction deployment budgets** | Per-faction, per-galaxy-size GNPRTB parameters for fleet size targets | Low |

### 3. Seeding & Economy (Seeding 95%, Economy 15%)

**Functions analyzed**: 37

#### Faithfully Implemented (Seeding)

| Original | Rust | Notes |
|----------|------|-------|
| Galaxy control bucketing (`FUN_00518f00`) | `assign_control_buckets()` | SDPRTB 7680/7681 percentages, Coruscant -1 adjustment — exact |
| 3-system model (`FUN_00519c50`) | `apply_seeds_with_rng()` | Coruscant/Yavin/random-rim-HQ routing — exact |
| Core energy (`FUN_00557df0`) | `initialize_energy_and_raw_materials()` | `clamp(10 + random(0..4), 0, 15)` — exact |
| Core raw materials (`FUN_00557e20`) | Same function | Double-clamp with energy cap — exact |
| Rim energy (`FUN_005580f0`) | Same function | Two-rand sum `1 + random(0..4) + random(0..9)` — exact |
| Procedural facilities (`FUN_00557fd0`/`FUN_005581e0`) | `generate_procedural_facilities()` | Mine chance = `(raw - mines) * GNPRTB[7766/7767]`, else SYFCCRTB/SYFCRMTB — exact |
| Common unit bundles (`FUN_0055bf00`/`FUN_0055bf50`) | `seed_maintenance_budget_units()` | CMUNALTB/CMUNEMTB weighted random — equivalent |
| Character stat rolling | `roll_character_stats()` | `base + random(0..variance)`, fix variance to 0 — exact |
| Sector support (`FUN_00557e80`) | `initialize_support()` | All 5 bucket types with per-faction SDPRTB params — exact |

#### Seeding Divergences (Minor)

| Area | Gap | Impact |
|------|-----|--------|
| Alliance HQ energy boost | Original ensures HQ energy/materials cover seeded facilities; we don't | Low |
| Budget overshoot | Original overshoots by one bundle then exits; we skip if over budget | Minimal |
| Populated filter | Original deploys to ANY controlled system; we filter by `is_populated` | Low |

#### Missing Economy Systems (Critical)

| System | Function | Description | Impact |
|--------|----------|-------------|--------|
| **Per-system economy tick** | `FUN_005073d0_adjust_and_deploy_each_system` | 18-step pipeline per system, every tick | **Critical** — the core territorial control loop |
| **Popular support drift** | `FUN_005583c0_calculate_adjusted_support_value` | Troops/fleets/fighters actively drift support toward controlling faction | **Critical** |
| **Collection rate** | `FUN_00558390_calculate_collection_rate` | `(GNPRTB[7763] * 100) / max(support, 1)` — inversely proportional to support | High |
| **Energy/material allocation** | 4 fields per system: total + allocated for each resource | High — manufacturing doesn't consume resources |
| **Mine activation** | Per-tick mine toggle based on raw material needs | Medium |
| **KDY modifier** | `FUN_0050a480` — Kuat Drive Yards fleet presence gives production bonus | Medium |
| **Garrison requirement** | Per-tick calculation using GNPRTB[7761-7762] with faction modifiers | Medium |
| **Production modifier** | `system+0x6c` (-100 to +100) adjusted by fleets/KDY | Medium |
| **AI resource rebalancing** | RESRC_TABLE 4 outcomes for energy/material fluctuation | Medium |
| **Maintenance shortfall** | Event 0x304 when costs exceed income | Low |

#### Popular Support Drift Formula (from `FUN_005583c0`)

```
GNPRTB params:
  7732 = 40  (drift threshold)
  7736 = 20, 7733 = 25, 7734 = 30, 7735 = 50
  7737 = 75  (base drift rate)
  7686 = 10  (fleet influence)
  7687 = 5   (fighter influence)
  7688 = 2   (troop influence)

if support <= 40 AND no_fleet1:
    if support > 20 AND support <= 30: base = 50
    elif support > 20: base = 25
    else: base = 75
    drift = clamp(base - fighters*5 - troops*2*strong_mult - fleet2*10, 0, 100)
if empire_controlled: drift = -drift
```

### 4. Events & Story (70% Parity)

**Functions analyzed**: 49

#### Faithfully Implemented

| Original | Rust | Notes |
|----------|------|-------|
| Victory conditions | `VictorySystem::check()` | HQ capture + Death Star paths |
| Uprising system | `UprisingSystem::advance()` | UPRIS1TB/UPRIS2TB, 10-tick cooldown, event IDs match |
| Blockade | `BlockadeSystem::advance()` | Event ID match, control kind bits |
| Research completion | `ResearchSystem::advance()` | Events 0x125-0x127 (ship/troop/facility), max level 15 |
| Jedi 4-tier system | `JediSystem::advance()` | None→Aware→Training→Experienced, detection check |
| Betrayal | `BetrayalSystem` | UPRIS1TB threshold, immunity flag |
| Dagobah chain | `story_events.rs` Chain 1 | Events 0x390→0x392→0x221→0x391→0x210 match |
| Han Solo speed | `movement.rs` | Base 100, Han modifier 50 via GNPRTB |

#### Partial Story Chains

| Chain | Gap | Detail |
|-------|-----|--------|
| **Bounty Hunters** | Missing unit spawn | Original creates BOUNTY_HUNTERS special forces entity at Han's system; we use tick/probability gate |
| **Jabba's Palace** | Simplified outcomes | Original has 5-case switch (escape self, escape rescue, captured by faction, captured generic, Jabba captures during rescue) with per-character dispatch (Leia, Luke, Chewbacca each get dedicated palace missions). We have 3 events. |
| **Final Battle** | Missing Emperor | Original requires Luke + Vader + Emperor co-located. We omit Emperor. Heritage-known flag gates different text. |
| **Leia Uses Force** | Not implemented | Event 0x362 — Leia-specific Force discovery |

#### Missing Event IDs

| Event ID | Handler | Description |
|----------|---------|-------------|
| 0x100 | `event_system_support` | Support change notification |
| 0x105 | `event_fleet_arrived` | Fleet arrival at system |
| 0x106 | `event_character_health` | Character health/injury |
| 0x107 | `event_units_deployed` | Units deployed to system |
| 0x128 | — | Alliance HQ destroyed / Coruscant captured |
| 0x153 | `event_informants_provide_info` | Espionage intel |
| 0x154/0x155 | `event_new_resources_or_natural_disaster` | Resource discovery vs disaster |
| 0x160 | `event_manufacturing_idle` | Queue empty notification |
| 0x200 | `event_han_solo_rescue_attempt` | Han rescue from palace |
| 0x230 | `event_emperor_arrives_at_coruscant` | Emperor arrival (one-shot) |
| 0x231 | `event_jabba_delivers_prisoners` | Jabba prisoner delivery |
| 0x300 | `event_recruitment_done` | All candidates recruited |
| 0x304 | `event_maintenance_shortfall` | Resource deficit |
| 0x305 | `event_saboteurs_strike_at` | Sabotage notification |
| 0x306 | `event_character_killed` | Character death |
| 0x361 | `event_character_discovers_traitor` | Traitor discovery |
| 0x386 | — | System side-change check |

### 5. Multiplayer Protocol (0% — Deferred)

**178 distinct `net_notify_*` message types** covering:

| Category | Count | Examples |
|----------|-------|---------|
| Character state | ~30 | force, skills, capture, escape, healing, injury |
| Game objects | ~25 | create, deploy, destroy, repair, enroute, ETA |
| System state | ~30 | control, energy, loyalty, uprising, blockade, explored |
| Mission management | ~15 | objectives, targets, teams, phases, decoys |
| Manufacturing | ~10 | production, deployment, completion, overflow |
| Story events | ~15 | Luke/Vader/Han/Leia/Dagobah/Jabba/Final Battle |
| Roles/assignments | ~15 | mission roles, decoy, adrift, resign |
| Fleet operations | ~5 | assault, battle, blockade, bombard |
| Facilities | ~5 | processing, startup, suspend |
| Research | ~4 | shipyard, training facility, construction yard |

### Main Game State Machine (from `FUN_00409cf0_main_game_loop`)

| State | Purpose |
|-------|---------|
| 1 | Start game |
| 2 | Strategy mode (main galaxy map) |
| 3-4 | State transition / combat check |
| 5-6 | Pre-tactical setup |
| 7 | Tactical battle (space) — bitmap/cursor setup |
| 8 | Tactical battle (ground) |
| 9-11 (0x9-0xb) | Post-tactical cleanup |
| 12 (0xc) | Return to strategy |
| 20-23 (0x14-0x17) | Tactical combat rendering / ship database states |

---

## Priority Implementation Roadmap

### P0 — Formula Corrections (Small, High Impact)

1. **Sabotage input**: `(espionage + combat) / 2` not raw espionage (1-line fix)
2. **Incite Uprising input**: Subtract system espionage rating as counter-intelligence
3. **Abduction input**: Subtract target defense rating
4. **Assassination input**: Subtract target defense rating
5. **Recruitment input**: Use target character's resistance, not system-derived value

### P1 — Missing Core Systems (Medium, Critical Impact)

6. **Per-system economy tick loop** — 18 sub-functions governing support drift, collection rate, garrison requirements, resource allocation. This is the largest single gap.
7. **Popular support drift formula** — GNPRTB 7686-7688, 7732-7737 for troop/fleet/fighter influence
8. **Death Star shield generator** — Entity type 0x25, must be destroyed before DS is vulnerable
9. **Ship repair system** — `CombatUnitUnderRepair` / `CombatUnitFastRepair` at shipyard systems
10. **Decoy mission subsystem** — TDECOYTB/FDECOYTB with GNPRTB[3588] penalty

### P2 — Story & Event Completeness (Medium, Moderate Impact)

11. **Jabba's Palace full chain** — 5-case outcome switch, per-character palace missions
12. **Final Battle with Emperor** — 3-character co-location check
13. **Missing event IDs** — 15+ notification events without handlers
14. **Character combat rating in ground combat** — Officer skill modifies troop effectiveness

### P3 — Multiplayer Foundation (Large, Future)

15. **net_notify message protocol** — 178 message types fully cataloged for multiplayer implementation
16. **State synchronization architecture** — Each message maps to a specific game state mutation

---

## GNPRTB Parameters Catalog

### Implemented

| Param | Value | Used In |
|-------|-------|---------|
| 0x1400 | 5 | Bombardment divisor |
| 5168-5170 | varies | Maintenance budget by galaxy size |
| 7680-7685 | varies | Core support percentages (SDPRTB) |
| 7711-7714 | 0/15/0/15 | Resource min/max bounds |
| 7721-7729 | varies | Energy/raw material formulas |
| 7730-7731 | 100/31 | Population percentages |
| 7764-7767 | varies | Neutral spread, mine multipliers |

### Missing (Discovered in Community Dump)

| Param | Value | Purpose |
|-------|-------|---------|
| 3076 | 50 | Seat of power bonus (Empire) |
| 3081 | 1 | Luke Force increment base |
| 3082 | varies | Leia Force parameter |
| 3588 | 35 | Decoy penalty multiplier (35%) |
| 7686 | 10 | Fleet influence on support drift |
| 7687 | 5 | Fighter influence on support drift |
| 7688 | 2 | Troop influence on support drift |
| 7732 | 40 | Support drift threshold |
| 7733 | 25 | Drift base (mid bracket) |
| 7734 | 30 | Drift threshold 2 |
| 7735 | 50 | Drift base (low-mid bracket) |
| 7736 | 20 | Drift threshold 1 |
| 7737 | 75 | Drift base (low bracket) |
| 7760 | 60 | Support threshold param |
| 7763 | 100 | Collection rate base |
| 0x1401-0x1445 | unknown | 36 combat params with faction overrides |

---

## Conclusion

The community disassembly is a valid and valuable complement to our own Ghidra analysis. The function logic is consistent where both analyses overlap. The key contributions are: (1) 1,471 human-readable labels that dramatically improve navigability, (2) the complete multiplayer protocol, and (3) the per-system economy tick loop which is our most critical implementation gap.

The recommended next step is implementing the P0 formula corrections (5 one-line fixes in `missions.rs`) followed by the P1 economy tick loop, which would bring overall parity from ~85% to ~93%.
