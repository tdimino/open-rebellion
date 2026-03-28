# Source Tree Cross-Reference Report (2026-03-27)

## Dataset

Community RE source tree from `rebexe-disassembly-source-trees.zip` (14,884 files). A significant upgrade over the prior 13,036-function disassembly — this dataset adds semantic function names (1,000+), C++ class hierarchy, and typed event records.

| Directory | Files | Description |
|-----------|-------|-------------|
| `functions/` | 13,036 | Decompiled C functions with semantic names |
| `classes/` | 216 | C++ class hierarchy (EventRecordBase → 93 EventRecord types) |
| `ptr_tables/` | 1,530 | Virtual method pointer tables (62 named entity types) |
| `dat_tables/` | 102 | Static data/lookup tables |

Also available online at `rebexe.biocrypto.com/disassembly/` (auth: chewbacca/chewbacca).

## Key Findings

### 1. Economy Tick Pipeline (FUN_005073d0)

The per-system economy tick is an **18-function strategic state rebuild**, not a narrow "support drift + collection rate" calculation. Our economy.rs previously implemented 3 of 17 functions (~17% parity). After Phase 3b: 16 of 17 (~94%).

| # | Function | Purpose | Status |
|---|----------|---------|--------|
| 1 | `FUN_00509ed0_adjust_energy` | Cap energy output | ✅ |
| 2 | `FUN_00509ef0` (facility cap) | Prune excess facilities | ✅ |
| 3 | `FUN_0050a220_adjust_mines` | Cap mine output | ✅ |
| 4 | `FUN_0050a3a0_adjust_popular_support` | Support drift | ✅ (fixed) |
| 5 | `FUN_0050a430_adjust_collection_rate` | Collection rate | ✅ |
| 6 | `FUN_0050a480_adjust_for_kdy` | KDY production modifier | ✅ |
| 7 | `FUN_0050a780_system_join_side` | Troop-based control | ✅ |
| 8 | `FUN_0050a710_adjust_garrison_requirement` | Garrison requirement | ✅ (fixed) |
| 9 | `FUN_0050a670` (troop surplus) | Troops - garrison | ✅ |
| 10 | `FUN_0050ac00_set_troops` | Total troop count | ✅ |
| 11 | `FUN_0050ace0_set_orbital_shipyards` | Shipyard presence | ✅ |
| 12-14 | `FUN_0050add0/af70/b4c0` | Fleet posture (3 passes) | ✅ |
| 15 | `FUN_0050aa50_adjust_for_fleet_fighters` | Fighter posture | ✅ |
| 16 | `FUN_00575590_do_nothing_return_1` | No-op | N/A |
| 17 | `FUN_0050a970` | Incident state | ✅ |
| 18 | `FUN_0050ac70` | Uprising visibility | ✅ |

### 2. Critical Bugs Found

- **Swapped event IDs**: EVT_NATURAL_DISASTER was 0x155 (should be 0x154), EVT_RESOURCE_DISCOVERY was 0x154 (should be 0x155). Confirmed from net_notify functions.
- **Wrong notification model**: 8 random-per-tick notification events replaced with state-transition-driven incidents from the economy tick.

### 3. GNPRTB Parameters Traced

17 total indices used by the economy pipeline. All values confirmed from GNPRTB.DAT:

| Index | Value | Purpose |
|-------|-------|---------|
| 7680 | 2 | Empire troop mult / garrison halving |
| 7682 | 2 | Uprising garrison doubler |
| 7684 | 5 | KDY capship penalty |
| 7685 | 2 | KDY fighter penalty |
| 7686 | 10 | Fleet influence on support drift |
| 7687 | 5 | Fighter influence on support drift |
| 7688 | 2 | Troop influence on support drift |
| 7691 | 0 | Empire-controlled transition delta |
| 7693 | 1 | Support favors controller delta |
| 7694 | 30 | Maintenance check rate (controlled) |
| 7695 | -1 | Support opposes controller delta |
| 7696 | 30 | Maintenance check rate (neutral) |
| 7697 | -1 | Controlled systems delta |
| 7732 | 40 | Drift threshold |
| 7733-7737 | 25/30/50/20/75 | Drift bracket values |
| 7760 | 60 | Energy control threshold |
| 7761 | 60 | Garrison threshold |
| 7762 | -10 | Garrison divisor |
| 7763 | 100 | Collection rate base |

### 4. Event Record Types (93)

The source tree reveals a typed event record hierarchy:

- **System events** (26): blockade, energy, loyalty, battle, bombardment, production, incidents
- **Character events** (10): force, training, injury, commanding, traitor
- **GameObject events** (16): completed, deployed, created, destroyed (5 variants)
- **Fleet/combat events** (7): assault, battle, blockade, bombard, fighter damage
- **Mission/role events** (10): mission report, Dagobah, bounty, outcomes
- **Research/manufacturing** (12): 3 order + 3 done per facility type, 4 mfg manager
- **Capital ship events** (7): hull, shield, weapon, tractor, speed, hyperdrive
- **Side events** (5): maintenance, recruitment, victory, Final Battle

### 5. Net Notify Functions (179)

179 named `net_notify_*` functions confirmed (was estimated at 178). These map 1:1 to multiplayer protocol message types for Phase 5.

### 6. Named Entity Types (62)

62 semantic entity classes identified via ptr_tables: Fleet, Troops, Fighters, CapitalShip, SpecForces, Shipyard, Mine, Refinery, 23 mission types, 3 named characters (Han, Luke, Vader), and management classes.
