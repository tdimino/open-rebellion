# Ghidra RE Notes — Index

**115 decompiled C files, 8 Jython scripts, 7 markdown docs**

## Summary Documents

| File | Lines | Content |
|------|-------|---------|
| [combat-formulas.md](combat-formulas.md) | ~200 | **Master reference** — binary overview, architecture, confirmed functions, scripted events, Ghidra scripts, next steps |
| [COMBAT-SUMMARY.md](COMBAT-SUMMARY.md) | ~115 | Combat call chain diagram, confirmed formulas, entity type codes, implementation readiness |
| [annotated-functions.md](annotated-functions.md) | 1,662 | **Annotated function reference** — renamed variables, struct layouts, cross-references, game rules |

## Combat Subsystem Docs

| File | Subsystem | Key Functions |
|------|-----------|---------------|
| [space-combat.md](space-combat.md) | Space combat auto-resolve — 7-phase pipeline | FUN_00549910, FUN_00544030, FUN_00544130, FUN_005443f0, FUN_005444e0, FUN_005445d0 |
| [ground-combat.md](ground-combat.md) | Ground combat — troop iteration + per-unit resolution | FUN_00560d50, FUN_004ee350, FUN_005617b0 |
| [bombardment.md](bombardment.md) | Orbital bombardment — Euclidean distance formula | FUN_00556430, FUN_0055d8c0, FUN_0055d860 |

## Decompiled Functions (*.c files)

### Battle Orchestration
| File | Address | Lines | Purpose |
|------|---------|-------|---------|
| FUN_0040a700_battle_start.c | 0x0040a700 | 182 | Battle state machine (18-state switch) |
| FUN_00514a60.c | 0x00514a60 | 364 | System battle orchestrator — dispatches to 4 combat subsystems |

### Space Combat Pipeline
| File | Address | Lines | Purpose |
|------|---------|-------|---------|
| FUN_005442f0_combat_init.c | 0x005442f0 | — | Combat initialization |
| FUN_00544da0_fleet_eval.c | 0x00544da0 | 96 | Fleet composition evaluation |
| FUN_00544030_weapon_damage.c | 0x00544030 | 54 | Weapon damage phase |
| FUN_00544130_shield_absorb.c | 0x00544130 | 83 | Shield absorption phase |
| FUN_005443f0_hull_damage_apply.c | 0x005443f0 | 54 | Hull damage application |
| FUN_005444e0_fighter_engage.c | 0x005444e0 | 53 | Fighter engagement |
| FUN_005445d0_combat_result.c | 0x005445d0 | 175 | Combat result determination |
| FUN_00544a20_post_combat.c | 0x00544a20 | 86 | Post-combat cleanup |

### Per-Side Resolvers (27 lines each)
| File | Address | Type Code | Vtable | Notif Handler |
|------|---------|-----------|--------|---------------|
| FUN_00543b60.c | 0x00543b60 | 0x04 (weapon) | +0x1c4 | FUN_0054a1d0 |
| FUN_00543bd0.c | 0x00543bd0 | 0x20 (shield) | +0x1c8 | FUN_0054a260 |
| FUN_00543cb0.c | 0x00543cb0 | 0x80 (hull) | +0x1d0 | FUN_0054a380 |
| FUN_00543d20.c | 0x00543d20 | 0x100 (fighter) | +0x1d4 | FUN_0054a410 |

### Stat Getters (~44-50 lines each)
| File | Address | Lines | Combat Phase |
|------|---------|-------|-------------|
| FUN_005434b0.c | 0x005434b0 | 44 | Weapon fire + shield |
| FUN_00543550.c | 0x00543550 | 44 | Hull damage |
| FUN_00543690.c | 0x00543690 | 50 | Fighter engagement |

### Notification Handlers (32 lines each)
| File | Address | Purpose | Final Vtable |
|------|---------|---------|-------------|
| FUN_0054a1d0.c | 0x0054a1d0 | Weapon damage applied | +0x1f4 |
| FUN_0054a260.c | 0x0054a260 | Shield absorbed | +0x1c8 |
| FUN_0054a380.c | 0x0054a380 | Hull damaged | +0x1d0 |
| FUN_0054a410.c | 0x0054a410 | Fighter damaged | +0x1d4 |

### Ground Combat
| File | Address | Lines | Purpose |
|------|---------|-------|---------|
| FUN_00560d50_ground_combat.c | 0x00560d50 | 232 | Ground combat main |
| FUN_004ee350_unit_combat.c | 0x004ee350 | 30 | Per-unit combat resolution |
| FUN_005617b0_death_star.c | 0x005617b0 | 68 | Death Star handler |

### Bombardment
| File | Address | Lines | Purpose |
|------|---------|-------|---------|
| FUN_00556430_bombardment.c | 0x00556430 | 78 | Bombardment handler |
| FUN_0055d8c0.c | — | — | Bombardment damage formula |
| FUN_0055d860.c | — | — | Power ratio (Euclidean distance) |
| FUN_0053e1d0_sqrt.c | 0x0053e1d0 | — | sqrt function |
| FUN_0053e170_modifier.c | 0x0053e170 | — | Difficulty modifier |
| FUN_00509620_combat_stats.c | 0x00509620 | — | Combat stat extractor (short[2]) |
| FUN_00555540.c | — | — | Fleet strength getter |

### Other Combat Functions
| File | Address | Lines | Purpose |
|------|---------|-------|---------|
| FUN_005029a0_hull_damage.c | 0x005029a0 | 16 | HullValueDamage notification setter |
| FUN_005029f0.c | 0x005029f0 | 16 | ShieldRechargeRate setter |
| FUN_00502a40.c | 0x00502a40 | 16 | WeaponRechargeRate setter |
| FUN_005038e0.c | 0x005038e0 | 16 | SquadSizeDamage setter |
| FUN_004fc080_mission_destroy.c | 0x004fc080 | 89 | Mission destruction (sabotage/assassination) |
| FUN_004fc3a0.c | 0x004fc3a0 | 16 | Damage application handler |
| FUN_00511ec0.c | 0x00511ec0 | 15 | ControlKindBattleWon |
| FUN_00511f40.c | 0x00511f40 | 16 | ControlKindUprising |
| FUN_005121e0.c | 0x005121e0 | 16 | Uprising handler |
| FUN_00512440.c | 0x00512440 | 15 | CombatUnitFastRepair |
| FUN_00532e00_victory.c | 0x00532e00 | 16 | Victory conditions (near VictoryConditions string) |
| FUN_00536740.c | 0x00536740 | 15 | BaseCombat skill |
| FUN_0054ba00.c | 0x0054ba00 | 16 | FinalBattle (Luke vs Vader) |

### Tactical Combat (2D View)
| File | Address | Lines | Purpose |
|------|---------|-------|---------|
| FUN_005a7500_weapons_fire.c | 0x005a7500 | 572 | Tactical ship object constructor — registers 20+ message handlers |

### Large Functions (Purpose TBD)
| File | Address | Lines | Notes |
|------|---------|-------|-------|
| FUN_0050d5a0_largest.c | 0x0050d5a0 | 503 | Mission dispatch (switch on 8+ mission types) |
| FUN_0052d920_second.c | 0x0052d920 | 475 | Unit description/display (TEXTSTRA string IDs) |
| FUN_00537180_third_largest.c | 0x00537180 | 381 | TBD |
| FUN_00508660.c | 0x00508660 | 304 | TBD |

## Ghidra Scripts

| Script | Purpose | Run Command |
|--------|---------|-------------|
| `../scripts/FindAllFunctions.py` | Scan .text for x86 prologues | `exec(open("...").read())` |
| `../scripts/DumpStrings.py` | Search all strings by keyword → file | Jython |
| `../scripts/DumpCombatXrefs.py` | Trace combat string → function xrefs | Jython |
| `../scripts/DumpCallers.py` | Find direct callers (found virtual dispatch) | Jython |
| `../scripts/DumpCombatRegion.py` | List all functions in 0x4f0000-0x540000 | Jython |
| `../scripts/FindCombatMath.py` | Search for combat math patterns | Jython |

## External Data Files (on Desktop, not in repo)

| File | Content |
|------|---------|
| `~/Desktop/rebellion-strings.txt` | 205 combat-related strings from REBEXE.EXE |
| `~/Desktop/rebellion-combat-xrefs.txt` | String → function cross-references |
| `~/Desktop/rebellion-combat-region.txt` | All functions in 0x4f0000-0x540000 (84KB) |
| `~/Desktop/rebellion-callers.txt` | Caller trace (confirmed virtual dispatch) |
