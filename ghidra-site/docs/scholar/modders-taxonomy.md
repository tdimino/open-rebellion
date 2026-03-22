---
title: "Modder's Function Taxonomy—Star Wars Rebellion (REBEXE.EXE)"
description: "Categorized function taxonomy of 22,741 REBEXE.EXE functions for modders"
category: "ghidra"
created: 2026-03-15
updated: 2026-03-16
---

# Modder's Function Taxonomy—Star Wars Rebellion (REBEXE.EXE)

*Generated from Ghidra decompilation of REBEXE.EXE (2.8MB, 22,741 functions). Cross-referenced with annotated-functions.md, combat-formulas.md, space-combat.md, ground-combat.md, bombardment.md, and rebellion-all-functions.txt.*

*All game logic lives in REBEXE.EXE. STRATEGY.DLL and TACTICAL.DLL are resource-only (sprites, art).*

---

## How to Use This Document

Each section maps known function addresses to a game system, explains which GNPRTB/SDPRTB parameters control that system, and lists what a total conversion modder must change. Function addresses link to decompiled `.c` files in `ghidra/notes/` where available.

**GNPRTB quick reference**: 213-entry master balance table (GNPRTB.DAT). Each entry has 8 values: Development, Alliance Easy/Med/Hard, Empire Easy/Med/Hard, Multiplayer. Only ~29% of parameters are documented; the remainder are RE targets.

**SDPRTB quick reference**: 35-entry per-faction modifier table (SDPRTB.DAT). Same 8-value layout.

---

## 1. Unit Stats & Balance

*Functions that set, validate, and notify changes to hull, shields, weapon recharge, squadron size, and troop strength. These are the arithmetic heart of every combat outcome.*

### Function Table

| Address | Size | Purpose | Decompiled File |
|---------|------|---------|----------------|
| FUN_00501490 |—| **Hull damage validator/setter**—validates `new_hull ∈ [0, max_hull]`, writes to `entity+0x60`, fires `CapShipHullValueDamageNotif` | `FUN_00501490_hull_damage.c` |
| FUN_00501510 |—| **Shield recharge rate setter**—writes 4-bit value to bits 0-3 of packed field at `entity+0x64`, fires `ShieldRechargeRateCHAllocated` | `FUN_00501510_shield_recharge.c` |
| FUN_005015a0 |—| **Weapon recharge rate setter**—writes 4-bit value to bits 4-7 of packed field at `entity+0x64`, fires `WeaponRechargeRateCHAllocated` | `FUN_005015a0_weapon_recharge.c` |
| FUN_005032c0 |—| **Squad size damage setter**—validates `new_size ∈ [0, max_squad_size]`, writes to `entity+0x60`, fires `FightSquadSquadSizeDamageNotif` | `FUN_005032c0_squad_size.c` |
| FUN_004ee350 | 30 | **Regiment/troop strength setter**—`entity+0x96` (short), non-negative, fires vtable `+0x330` | `FUN_004ee350_unit_combat.c` |
| FUN_00509cc0 | 117 | **SystemEnergy validator**—`Invalid SystemEnergy value!` error |—|
| FUN_00509d40 | 117 | **SystemEnergyAllocated validator** |—|
| FUN_00509dc0 | 117 | **SystemRawMaterial validator** |—|
| FUN_00509e40 | 117 | **SystemRawMaterialAllocated validator** |—|
| FUN_00509f40 | 117 | **ProductionModifier validator** (system-level) |—|
| FUN_0055a7a0 | 117 | **ProductionModifier validator** (facility-level) |—|
| FUN_0053fc90 |—| **Range validator**—`validate_range(value, min, max)`. Universal range-check primitive used by all setters |—|
| FUN_0053e0f0 |—| **Secondary clamp**—`clamp(value, 0, 0x7fff)`. Prevents short overflow. Applied after range check in loyalty and enhanced loyalty |—|
| FUN_0053a000 |—| **Entity validity check**—entry guard for all setter functions |—|
| FUN_0053a010 |—| **Entity validity check (notification path)**—used by notification dispatchers |—|

### Entity Object Layout (Offset Map)

```
+0x50  uint   status_flags        bit0=active, bit3=fighter_combat_eligible, bit12=special_disabled
+0x58  uint   combat_phase_flags  Space combat state machine (see §2)
+0x60  int    hull_current        Capital ships: current hull. Fighters: current squad size.
+0x64  uint   shield_weapon_packed  bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated
+0x66  short  loyalty_base        Character base loyalty 0-100
+0x78  byte   capability_flags    bit4=special_entity, bit6/7=disabled, bit7=alt_shield_path
+0x8a  short  enhanced_loyalty    Mission bonus loyalty 0-100 (distinct from base)
+0x96  short  regiment_strength   Troop/ship regiment strength. 0 = destroyed/non-combat.
+0x9a  short  hyperdrive_modifier Han Solo fleet speed bonus
+0xac  byte   alive_flag          bit0=alive/combat-eligible
+0xb0  byte   combat_ready_flags  bit1=combat_ready_for_fleet_eval
```

### GNPRTB Parameters

GNPRTB parameters control balance at a table level rather than per-entity. Specific indexes for combat damage scaling are not yet mapped, but the bombardment divisor is confirmed:

| GNPRTB Global | Purpose | Notes |
|---------------|---------|-------|
| `DAT_006bb6e8` | Bombardment damage divisor | Scales Euclidean power ratio to final damage. Identified from FUN_0055d8c0. |
| `DAT_00661a88` | Difficulty modifier table | Applied after GNPRTB divisor via FUN_0053e190. Index derived from `entity+0x24 bits 4-5` (0-3 = Dev/Easy/Med/Hard) |

### What to Change for a Total Conversion

**Yuuzhan Vong / KOTOR / Thrawn era stat overrides:**
- Capital ship hull, shields, weapon recharge: patch the DAT fields in CAPSHPSD.DAT (`hull`, `shields_strength`, `shields_recharge`, weapon arc values). The setter functions read per-entity maxima from vtable `+0x248` (hull) and `+0x244` (squad size)—these vtable-resolved values come from the DAT record.
- The open-rebellion mod system patches these at the `world/` layer via RFC 7396 JSON overlay. No code change needed for stat rebalancing: create a `capital_ships.json` with `{ "id": N, "hull": X }` entries.
- **New weapon types**: vtable slot `+0x1c4` dispatches weapon fire by type. Adding a new weapon type requires a new vtable handler and a new combat phase flag bit.
- **Shield/weapon recharge range**: both are 4-bit (0-15 max). Expanding the packed field at `+0x64` to allow higher values requires patching FUN_00501510, FUN_005015a0, and their `get_max_*` callers.
- **Troop strength beyond short max (32767)**: regiment strength at `+0x96` is a signed `short`. Expanding to `int` requires patching FUN_004ee350 and all readers of `+0x96`.

---

## 2. Combat Resolution

*The full auto-resolve pipeline: space combat, ground combat, bombardment, and Death Star. Every battle between fleets or armies passes through FUN_00514a60.*

### Master Orchestrator

| Address | Size | Purpose |
|---------|------|---------|
| FUN_00514a60 | 1622 | **System battle orchestrator**—entry for ALL combat at a star system. Calls space, ground, and bombardment sub-systems. |
| FUN_0040a700 | 1246 | **Battle state machine**—18-state switch on `STRAT_INIT → STRAT_BATTLE_FINISH`. Entry for player-initiated battles. Strings: `STRAT_ALL_BATTLE_START`, `STRAT_BATTLE_CHOOSE_TYPE`, `STRAT_BATTLE_CONVERT_TO_TACT`, `STRAT_BATTLE_CONVERT_TO_TACT_ABSTRACT`, etc. |

Two combat resolution modes: `CONVERT_TO_TACT` (2D tactical view, player-controlled) and `CONVERT_TO_TACT_ABSTRACT` (auto-resolve).

### Space Combat—7-Phase Auto-Resolve Pipeline

**Entry**: FUN_00514a60 → FUN_005457f0 → FUN_00549910

| Phase | Address | Lines | Purpose | Phase Flag |
|-------|---------|-------|---------|------------|
| 1—Init | FUN_005442f0 |—| Initialize combat state, check eligibility (alive + active + capability_flags clear + not special_disabled) |—|
| 2—Fleet Eval | FUN_00544da0 | 96 | Pre-combat fleet evaluation. Asymmetric (single pass, not bilateral). Skipped if `combat_phase_flags & 0x2000`. | `0x2000` skip |
| 3—Weapon Fire | FUN_00544030 |—| Weapon fire. **Attacker (side=1) fires first, defender (side=0) second.** Gate: `flags & 0x01 AND NOT flags & 0x40`. Calls FUN_00543b60 per side via vtable `+0x1c4`. | `0x01 / 0x40` |
| 4—Shield Absorb | FUN_00544130 | 83 | Shield absorption. Standard path for most ships; alt path for family 0x71-0x72 (fighter types) via vtable `+0x1d8`. Gate: `flags & 0x40`. | `0x40` |
| 5—Hull Damage | FUN_005443f0 | 54 | Hull damage application. Same bilateral pattern. Gate: `flags & 0x40`. | `0x40` |
| 6—Fighter Engage | FUN_005444e0 | 53 | Fighter squadron engagement. Final active phase. Gate: `flags & 0x40`. | `0x40` |
| 7—Result | FUN_005445d0 | 175 | Determine winner: gather 5 stat types, count survivors via `alive_flag & 1` at `+0xac`, fighter exception via `status_flags & 8`. Family 0x73/0x74 triggers special victory path (FUN_00534640). |—|
| 8—Post-Combat | FUN_00544a20 | 86 | Post-combat cleanup. Gate: `flags & 0x1000`. | `0x1000` |

**Per-side resolvers (called within phases):**

| Address | Purpose |
|---------|---------|
| FUN_00543b60 | Weapon fire resolver—sets phase flag `0x04`, calls vtable `+0x1c4` |
| FUN_00543bd0 | Shield absorption resolver—vtable `+0x1c8` |
| FUN_00543cb0 | Hull damage application—vtable `+0x1d0` |
| FUN_00543d20 | Fighter engagement—vtable `+0x1d4` |
| FUN_00543d90 | Victory determinator—sets flag `0x200` (victory bit), vtable `+0x1d8` |
| FUN_00534640 | Special entity victory (family 0x73)—sets flag `0x08` in `+0x78`, vtable `+0x258` |

**Stat getters (return both sides' objects + battle reference):**

| Address | Stat Type |
|---------|-----------|
| FUN_005434b0 | Weapon stats |
| FUN_00543550 | Hull stats |
| FUN_005435f0 | Special/init stats |
| FUN_00543690 | Fighter stats |
| FUN_00543760 | 5th stat (unknown) |
| FUN_00543800 | 6th/post-combat stat |

**Notification handlers:**

| Address | Fires After |
|---------|------------|
| FUN_0054a1d0 | Weapon damage applied |
| FUN_0054a260 | Shield absorption applied |
| FUN_0054a380 | Hull damage applied |
| FUN_0054a410 | Fighter damage applied |
| FUN_0054a4a0 | Victory determined |

**Difficulty modifier**: `entity+0x24 bits 4-5` (2-bit, 0-3). Applied via FUN_004fd600 inside FUN_0054a1d0.

**Space combat entity validator**: FUN_005438a0. Accepts exactly 6 type codes: `0x31000241`, `0x32000242`, `0x33000243`, `0x34000280`, `0x35000281`, `0x38000343`. The `0x34` prefix confirms Death Star is space-combat eligible.

**Space combat tactical constructor**: FUN_005a7500 (4856 bytes). Registers message handlers for the 2D tactical view: `SHIP_ADD/REMOVE/ABSTRACT_DESTROY`, all `SHIP_FIRE*` and `SHIP_TAKE_*_HIT` events, `SHIPGROUP_*`, `NAV_*`, `cmSHIP_POST_DS_STATUS`. This is the entry point for the player-controlled tactical battle mode.

### Ground Combat

**Entry**: FUN_00514a60 → FUN_00560d50

| Address | Lines | Purpose |
|---------|-------|---------|
| FUN_00560d50 | 232 | **Ground combat main**—dispatches on entity family. Ships (0x30-0x3b) delegate to space path; Death Star (0x34) calls FUN_005617b0; troops (0x14-0x1b) iterate all units at the system. **No friendly fire**: same-faction units skip. Uses `FUN_004f25c0(system, type=3)` to iterate. |
| FUN_004ee350 | 30 | **Per-unit resolution**—sets regiment strength (`+0x96`) to new value, fires vtable `+0x330`. Current implementation: set to 0 (destroy). Actual damage formula is upstream. |
| FUN_005617b0 | 68 | **Death Star planet destruction**—family 0x34 only. Checks `alive_flag & 1` (target alive) AND `status_flags & 1` (DS operational). Faction control check: `+0x24 & 0xc0 == 0x80` = Empire control. Calls FUN_0055f650 (superlaser effect). Hardcoded DatId `0x90000109` likely references the targeted system. |

**Ground combat entity type ranges (DatId family byte):**

| Range | Type |
|-------|------|
| 0x08-0x0f | Characters |
| 0x14-0x1b | Troops / Special Forces (combat-eligible ground units) |
| 0x30-0x3b | Capital Ships (delegated to space path) |
| 0x34 | Death Star (special handler) |

### Bombardment

**Entry**: FUN_00514a60 → FUN_00556430

```
FUN_00556430 (78 lines)—main bombardment handler
  → checks vtable+0x38 AND NOT this[0x14] & 0x800 (bombardment allowed)
  → FUN_00555540  (fleet strength getter for each side)
  → FUN_004025b0  (extract faction IDs—skip if same faction)
  → FUN_00555d30  (bombardment setup)
    → FUN_00555b30  (calculation)
      → FUN_00509620  (extract short[2] combat stats from entity+0x2c+0x4c)
      → FUN_0055d8c0  (damage formula)
        → FUN_0055d860  (power ratio = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²))
        → / DAT_006bb6e8  (GNPRTB divisor)
        → FUN_0053e190  (difficulty modifier)
  → FUN_004f7390  (apply damage to system)
```

**Bombardment formula (confirmed)**:
```
raw_power = sqrt((attacker_bombardment - defender_defense)² + (atk[1] - def[1])²)
damage = apply_difficulty_modifier(raw_power / GNPRTB_DIVISOR)
if damage == 0: damage = 1   // minimum 1 damage always dealt
```

`this[0x14] & 0x800` = bombardment blocked flag (already under bombardment or protected).

**Decompiled files**: `FUN_00556430_bombardment.c`, `FUN_00555d30.c`, `FUN_00555b30.c`, `FUN_0055d8c0.c`, `FUN_0055d860.c`, `FUN_0053e190.c`, `FUN_00509620_combat_stats.c`.

### GNPRTB Parameters Affecting Combat

| GNPRTB / Global | Controls | Notes |
|-----------------|---------|-------|
| `DAT_006bb6e8` | Bombardment damage divisor | Higher value = less bombardment damage |
| `DAT_00661a88` | Difficulty modifier table | Indexed by 2-bit difficulty level |
| Unconfirmed | Space combat weapon scaling | Parameters for vtable `+0x1c4` dispatch not yet traced |
| Unconfirmed | Shield absorption multiplier | Phase 4 internal calculation not yet decoded |
| Unconfirmed | Fighter attrition rates | Phase 6 internal not yet decoded |

### What to Change for a Total Conversion

- **New combat phase** (e.g., Force-power phase for KOTOR mod): add a new bit to `combat_phase_flags` at `+0x58`, add a new phase function following the same bilateral pattern as phases 3-6, insert it in FUN_00549910's phase sequence.
- **Alter victory conditions** (e.g., first to destroy X% of fleet wins): patch FUN_005445d0. The current logic counts fleet survivors via `alive_flag & 1`. Replace with custom threshold logic.
- **Change attacker-first initiative**: swap `side=1` and `side=0` in FUN_00544030. For simultaneous fire, call both sides before applying any results.
- **New Death Star variant** (Yuuzhan Vong worldship): FUN_005617b0 is the superlaser handler. Assign a new family byte, add a check in FUN_00560d50, write a new handler following the same pattern. The hardcoded `0x90000109` target ID needs to be replaced with a runtime lookup.
- **Bombardment formula change**: replace FUN_0055d860 (Euclidean distance) with your formula. The `short[2]` stat array from FUN_00509620 can be extended to more dimensions if needed.
- **Disable auto-resolve** (force tactical only): patch FUN_00549910 to return immediately; this forces `CONVERT_TO_TACT` path always.

---

## 3. GNPRTB Parameter System

*34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side) = 111 total parameter bindings. GNPRTB.DAT has 213 entries × 8 difficulty values = 1,704 i32 scalars. SDPRTB.DAT has 35 entries × 76 bytes each.*

### Parameter Table Loader

| Address | Size | Purpose |
|---------|------|---------|
| FUN_00569280 | 91 | **DAT format dispatcher**—recognizes `IntTableEntry`, `SideParamTableEntry`, `SeedFamilyTableEntry`, `SeedTableEntry`, `GeneralParamTableEntry`. Entry point for loading all parameter tables at startup. |

### Confirmed Global Bindings

| Address | GNPRTB Role | Value Notes |
|---------|-------------|-------------|
| `DAT_006bb6e8` | Bombardment divisor (combat GNPRTB) | Higher = weaker bombardment |
| `DAT_00661a88` | Difficulty modifier table pointer | 4-entry array (Dev/Easy/Med/Hard), applied in FUN_0053e190 |
| `DAT_006be3b8` | Game state root object pointer | All game state accessible from here |
| `DAT_0066e660` | Config data | References in multiple utility functions |

### Mission Probability Tables (*MSTB files)

Each `*MSTB.DAT` file defines a skill-threshold → probability curve for one mission type (IntTableEntry format: `threshold: i32`, `value: u32` probability 0-100).

| File | Mission |
|------|---------|
| DIPLMSTB.DAT | Diplomacy |
| ESPIMSTB.DAT | Espionage |
| ASSNMSTB.DAT | Assassination |
| INCTMSTB.DAT | Incite uprising |
| DSSBMSTB.DAT | Death Star sabotage |
| ABDCMSTB.DAT | Abduction |
| RCRTMSTB.DAT | Recruitment |
| RESCMSTB.DAT | Rescue |
| SBTGMSTB.DAT | Sabotage |
| SUBDMSTB.DAT | Subdue uprising |
| ESCAPETB.DAT | Escape probability |
| FDECOYTB.DAT | Fleet decoy success |
| FOILTB.DAT | Mission foil probability |
| INFORMTB.DAT | Informant probability |
| CSCRHTTB.DAT | Covert search probability |
| UPRIS1TB.DAT | Uprising start |
| UPRIS2TB.DAT | Uprising end |
| RLEVADTB.DAT | Rebel evasion probability |
| RESRCTB.DAT | Resource threshold |
| TDECOYTB.DAT | Troop decoy success |

### What to Change for a Total Conversion

**No code change required for most balance tuning:**
- Edit GNPRTB.DAT values directly (or via the open-rebellion JSON mod overlay for `general_params.json`) to adjust combat recovery, travel time, diplomacy shifts, uprising thresholds, etc.
- Edit `*MSTB.DAT` entries to change mission success probability curves.
- SDPRTB.DAT adjusts per-faction modifiers—useful for asymmetric faction design (Thrawn Empire vs Remnant, etc.).

**Requires code change:**
- Adding a 214th GNPRTB parameter: the parameter loader FUN_00569280 must be patched to accept the higher count, and all array accesses updated.
- Changing difficulty levels from 4 to more values: `entity+0x24 bits 4-5` is 2-bit (0-3 only). Expanding to 3 bits requires patching FUN_004fd600 and all callers.

---

## 4. Mission System

*Mission types, dispatch, probability formulas, lifecycle management. All 25 mission definitions live in MISSNSD.DAT.*

### Core Mission Functions

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0050d5a0 | 3050 | **Mission dispatch**—13-case switch on mission type. Sets entity family filters for each mission type's valid target pool. The largest function in the 0x4f0000-0x540000 combat region. |
| FUN_00536800 | 120 | **Mission role notification**—`RoleMissionKeyNotif` / `Mission` |
| FUN_004fc080 | 89 | **Mission destruction outcome**—handles `DestroyedSabotage` (type 6), `DestroyedAssassination` (type 7), `Autoscrap` (type 0x15 = 21) |
| FUN_004fc3a0 | 16 | **Damage application**—`GameObjDamagedNotif` / `Damaged` |

**Mission dispatch filter ranges** (from FUN_0050d5a0 case analysis):

| Case | Type | Entity Family Ranges Used |
|------|------|--------------------------|
| 1 | Diplomacy / Recruitment | Fleet filter |
| 2 | Special Forces | Unit filter |
| 3 | Type 3 | 0x10-0x28, two filters |
| 4 | Type 4 | 0xa0-0xb0, 0x28-0x30 |
| 5 | Type 5 | 0x08-0x10, fleet 1-255 |
| 6 | **Sabotage** | 0x08-0x10, fleet 1-255, side filter, `special_flag=true` |
| 7 | **Assassination** | 0x08-0x10, fleet 1-255, side + troop filters |
| 8 | Type 8 | 4 filters: 0x20-0x30, 0x10-0x20, 0xa0-0xb0, 0x08-0x10 |
| 9 | Type 9 | 0x30-0x40 |
| 10 | Type 10 | Three fleet ranges (1-255) |
| 11 | Type 11 | 0x10-0x40 + 0xa0-0xb0 |
| 12 | Type 12 | 0x20-0x30 |

### Mission Lifecycle Notifications

| Address | Notif String | Event ID |
|---------|-------------|----------|
| FUN_00524fc0 | MissionUserIDNotif / UserID |—|
| FUN_00524fc0-00525120 | Target, Objective, Leader, OriginLocation |—|
| FUN_00525560 | ReadyForNextPhaseNotif |—|
| FUN_005255a0 | ImpliedTeamNotif |—|
| FUN_005255e0 | MandatoryNotif |—|
| FUN_00536880 | MissionSeedKeyNotif / MissionSeed |—|
| FUN_005368c0 | ParentAtMissionCompletionKeyNotif |—|
| FUN_00536900 | LocationAtMissionCompletionKeyNotif |—|
| FUN_00536980 | MovingBetweenMissionsNotif |—|
| FUN_005369c0 | MissionRemoveRequestNotif |—|
| FUN_00536a00 | MissionResignRequestNotif |—|
| FUN_00536a40 | CanResignFromMissionNotif |—|
| FUN_00536b00 | OnMissionNotif / OnMission |—|
| FUN_00536b40 | OnHiddenMissionNotif |—|
| FUN_00536b80 | OnMandatoryMissionNotif |—|

### Mission Type Codes (confirmed)

| Code | Mission |
|------|---------|
| 6 | Sabotage |
| 7 | Assassination |
| 0x15 (21) | Autoscrap |

### Probability Formula (from rebellion2 reverse engineering)

```
prob = clamp(a·score² + b·score + c, min%, max%)
```

Coefficients (a, b, c, min, max) are mission-type-specific and read from `*MSTB.DAT` files at mission execution time. The quadratic formula is implemented in `rebellion-core/src/missions.rs`.

### GNPRTB Parameters

Unconfirmed indexes, but GNPRTB likely controls: mission duration multipliers, success probability bonuses/penalties per difficulty, reward/penalty magnitudes (loyalty shifts, damage amounts).

### What to Change for a Total Conversion

- **New mission type** (e.g., Yuuzhan Vong "Voxyn strike team"): add a new case to FUN_0050d5a0, create a new `*MSTB.DAT`-style table for its probability curve, assign a new MISSNSD.DAT entry with appropriate `special_force_eligibility` bitmask.
- **Change success rates without code changes**: edit the threshold/value pairs in the relevant `*MSTB.DAT` file. The probability formula reads these at runtime.
- **New story mission**: add a new `GameEvent` in the `EventState` system (`rebellion-core/src/events.rs`) with `EventCondition::TickReached` or `CharacterAtSystem` and the appropriate `EventAction`. No REBEXE.EXE patching required for open-rebellion.

---

## 5. Character & Force System

*Skill enhancement, Jedi training, Force detection, loyalty, betrayal, and special character abilities.*

### Loyalty System

| Address | Purpose | Notif / Event ID |
|---------|---------|-----------------|
| FUN_005341a0 | **Base loyalty setter**—`entity+0x66` (short), 0-100, vtable `+0x238` | SystemLoyaltyNotif / Loyalty |
| FUN_004ee030 | **Enhanced loyalty setter** (mission bonus)—`entity+0x8a` (short), 0-100 clamped + 0x7fff clamp, vtable `+0x318` | EnhancedLoyaltyNotif |
| FUN_00535380 | **Loyalty notification dispatcher** | LoyaltyNotif |

Two distinct loyalty fields:
- `+0x66`: base loyalty (diplomacy missions, betrayal mechanics)
- `+0x8a`: mission-bonus loyalty (EnhancedLoyalty special ability)

### Skill Enhancement System

| Address | Purpose | Notif |
|---------|---------|-------|
|—| Enhanced diplomacy | `CharacterEnhancedDiplomacyNotif` / `EnhancedDiplomacy` |
|—| Enhanced espionage | `CharacterEnhancedEspionageNotif` / `EnhancedEspionage` |
|—| Enhanced combat | `CharacterEnhancedCombatNotif` / `EnhancedCombat` |
| FUN_004ee030 | Enhanced loyalty | `CharacterEnhancedLoyaltyNotif` / `EnhancedLoyalty` |
| FUN_004ee470 | Han Solo hyperdrive modifier | `MissionHyperdriveModifierNotif` / `MissionHyperdriveModifier` |

**Hyperdrive modifier** (`+0x9a`, short): applied to Han Solo's fleet movement speed. Non-negative only (no upper bound). Consumed in `rebellion-core/src/movement.rs` when calculating `ticks_per_hop`.

### Jedi / Force System

| Address | Size | Purpose | Event ID |
|---------|------|---------|----------|
| FUN_004f1e00 | 73 | **Force attribute change notif**—`CharacterForceNotif` / `Force` | 0x1e1 (481) |
| FUN_004f1e50 | 73 | **Force experience notif**—`CharacterForceExperienceNotif` / `ForceExperience` |—|
| FUN_004f1ea0 | 73 | **Force training notif**—`CharacterForceTrainingNotif` / `ForceTraining` | 0x1e5 (485) |
| FUN_004f20e0 | 73 | **Force user discovered notif**—`CharacterForceUserDiscoveredKeyNotif` |—|
| FUN_004f2240 | 51 | **ForceAware notif**—`CharacterForceAwareNotif` |—|
| FUN_004f2280 | 51 | **ForcePotential notif**—`CharacterForcePotentialNotif` |—|
| FUN_004f2300 | 51 | **DiscoveringForceUser notif**—`CharacterDiscoveringForceUserNotif` |—|
| FUN_0058a3f0 | 306 | **Force user detection**—`force-detect` string. Calculates detectability from 2-bit Force tier + experience + detector's potential |—|

**Force detection mechanic** (from FUN_0058a3f0 decompilation):

```
potential_tier = target[9] >> 6 & 3        // 2-bit: 0=Low, 1=Med, 2=High, 3=Very High
force_xp       = (short)target[0x23]       // Force experience accumulator
target_pot     = *(short*)(detector+0x8c)  // Detector's Force potential

detected = FUN_0055e4d0(potential_tier, force_xp, target_pot, ...)
if detected AND !(target[0x1e] & 1):        // is_jedi_but_not_discovered
    detection_context->detection_pending = 1
```

Three conditions all required: `FUN_0055e4d0` (stat check), `FUN_0055ff60` (secondary), `FUN_0058a530` (finalize).

**Force object layout:**

| Offset | Field | Notes |
|--------|-------|-------|
| `[9] >> 6 & 3` | force_potential_tier | 2-bit, 0-3 |
| `[0x23]` as short | force_experience | Accumulates through training/use |
| `+0x8c` short | force_potential_raw | Detector's Force potential |
| `+0x1e & 1` | is_undiscovered_jedi | Set before Force User Discovered event |
| `detection_context+0x4c` | detection_pending | Set when undiscovered Jedi is found |

### What to Change for a Total Conversion

- **New Force powers** (KOTOR mod—Force Lightning, Battle Meditation): add new notification strings following the `CharacterForce*Notif` pattern, add new vtable slots, implement new skill effects.
- **Different Jedi training mechanics** (no Dagobah, multiple teachers): FUN_0056fc70 (DagobahCompleted, event 0x210) and FUN_00571590 (Teacher key notif) define the training chain. Patch or replace the Dagobah prerequisite check in the mission manager.
- **More Force tiers** (0-3 is 2-bit—only 4 levels): expanding to 3 bits (0-7) requires patching FUN_0058a3f0's `>> 6 & 3` extraction to `>> 5 & 7` and updating FUN_0055e4d0's tier comparisons.
- **Betrayal mechanic**: `not betray` string at `0x006ab3dc` confirms a betrayal check function exists. The `is_unable_to_betray` flag in MJCHARSD.DAT/MNCHARSD.DAT is the DAT-layer gate. FUN_005341a0 handles loyalty changes that could trigger betrayal.

---

## 6. Faction & Victory

*Victory conditions, faction control, recruitment, uprising mechanics.*

### Victory Conditions

| Address | Size | Purpose | Notif |
|---------|------|---------|-------|
| FUN_00532e00 |—| **Construction yard research order notif**—`SideConstructionYardRdOrderNotif` (event 0x127). Note: originally mislabeled as victory; the actual victory condition function is separate. |—|
| FUN_00532f40 | 65 | **Recruitment done notif**—`SideRecruitmentDoneNotif` / `RecruitmentDone` (event 0x12c) |—|
|—|—| **Victory condition check**—referenced by `SideVictoryConditionsNotif` / `VictoryConditions` string at `0x006aa764` |—|
| FUN_0054ba00 | 65 | **Final battle trigger**—`MissionMgrFinalBattleNotif` / `FinalBattle` |—|
| FUN_0054b9c0 | 51 | **FinalBattleReady notif** |—|

**Victory sound events**: `VICTORY` at `0x006ab5f8`, `VICTORY_II` at `0x006ab5b0`.

### System Control

| Address | Purpose | Notif | Event ID |
|---------|---------|-------|----------|
| FUN_00511ec0 | System control change—battle won | `ControlKindBattleWonNotif` |—|
| FUN_00511f40 | System control change—uprising | `SystemControlKindUprisingNotif` / `ControlKindUprising` |—|
| FUN_005121e0 | Uprising handler | `SystemUprisingNotif` / `Uprising` |—|
| FUN_005122d0 | Blockade system handler | `SystemBlockadeNotif` / `Blockade` | 0x14e (334) |
| FUN_00512280 | System battle notification | `SystemBattleNotif` / `Battle` | 0x14d (333) |
| FUN_00512440 | Unit fast repair | `SystemCombatUnitFastRepairNotif` / `CombatUnitFastRepair` |—|
| FUN_00512580 | Uprising incident | `SystemUprisingIncidentNotif` / `UprisingIncident` | 0x152 (338) |
| FUN_00512540 | Battle pending caused blockade | `BattlePendingCausedCurrentBlockade` |—|
| FUN_005126c0 | Blockade + battle pending management required | `BlockadeAndBattlePendingManagementRequired` |—|

**Loyalty-driven control**: `LoyaltyCausedCurrentControlKind` notif string confirms that loyalty thresholds directly trigger faction control changes. Thresholds are likely in UPRIS1TB.DAT / UPRIS2TB.DAT.

**System state notification strings** (from rebellion-strings.txt):
- `SystemLoyaltyNotif` / `Loyalty`
- `ControlKindBattleWon`, `ControlKindBattleWithdrew`, `ControlKindUprising`, `ControlKindBattleReady`
- `BattlePending`, `LoyaltyCausedCurrentControlKind`

### GNPRTB Parameters

UPRIS1TB.DAT (3 entries) and UPRIS2TB.DAT (4 entries) control uprising start/end thresholds at the IntTableEntry level. GNPRTB likely has additional parameters for loyalty decay rates, diplomacy shift magnitudes, and uprising suppression costs.

### What to Change for a Total Conversion

- **New victory condition** (e.g., control 80% of systems instead of capturing HQ): patch the `VictoryConditions` handler. The `SideVictoryConditionsNotif` string is the entry point.
- **Multiple factions** (Thrawn era—Empire, New Republic, Remnant, Hutts): the game uses a 2-faction architecture hardcoded throughout. Adding factions requires: new faction IDs in BASICSD.DAT, patching all same/opposite-faction checks, new per-faction UI elements. Major effort.
- **Alliance victory sounds**: `VICTORY` vs `VICTORY_II` suggests two different victory fanfares. Replace these audio resources.
- **Uprising thresholds**: edit UPRIS1TB.DAT and UPRIS2TB.DAT threshold/value pairs directly.

---

## 7. Fleet & Movement

*Fleet composition, hyperspace transit, blockades, and troop blockade interception.*

### Fleet Events

| Address | Size | Purpose | Notif |
|---------|------|---------|-------|
| FUN_004ff7a0 | 65 | **Fleet battle trigger** | `FleetBattleNotif` / `Battle` |
| FUN_004ff7f0 | 65 | **Fleet blockade handler** | `FleetBlockadeNotif` / `Blockade` |
| FUN_004ff840 | 65 | **Fleet bombardment handler** | `FleetBombardNotif` / `Bombard` |
| FUN_004ff890 | 65 | **Fleet assault handler** | `FleetAssaultNotif` / `Assault` |
| FUN_00504a00 | 231 | **Troop destruction during blockade**—`TroopRegDestroyedRunningBlockade` (event 0x340) |—|
| FUN_00504470 | 118 | **Troop withdrawal percent setter**—`TroopReg.cpp: Invalid WithdrawPercent value!` error |—|
| FUN_00504af0 | 56 | **Troop withdrawal percent notif**—`TroopRegWithdrawPercentNotif` |—|
| FUN_00511e70 | 73 | **TroopRegWithdrawPercent system notif** | `SystemTroopRegWithdrawPercentNotif` |
| FUN_004ee470 | 129 | **Han Solo hyperdrive modifier**—`MissionHyperdriveModifier` notif; stored at `+0x9a` (short) |—|

### Fleet Composition Function

| Address | Size | Purpose |
|---------|------|---------|
| FUN_00502020 | 897 | **Garrison/fleet strength aggregator**—iterates entities by DatId family_id byte. Largest aggregator function in the notification region. Family ranges: 0x10-0x14=troops, 0x1c-0x20=facilities, 0x30-0x40=ships. |
| FUN_00555540 |—| **Fleet strength getter** (for bombardment side) |

### Movement System (rebellion-core)

Implemented in `rebellion-core/src/movement.rs` (453 LOC, 11 tests):

```
ticks_per_hop = BASE_TICKS_PER_HOP(30) / slowest_hyperdrive_rating
clamped to MIN_TICKS_PER_HOP(5)
Fleets without capital ships: DEFAULT_TICKS_PER_HOP(20)
```

Han Solo modifier: `character.hyperdrive_mod` (`entity+0x9a`) is applied to the fleet's speed calculation when Han is assigned.

### What to Change for a Total Conversion

- **New movement mechanic** (hyperspace lanes / Thrawn-era gravity wells): REBEXE.EXE's blockade system (`FleetBlockadeNotif`, `FUN_004ff7f0`) handles gravity well interdiction. In open-rebellion, add a new `MovementOrder` variant and constraint in `movement.rs`.
- **Interdictor cruisers**: the blockade event chain (`TASKFORCE_NEW` at `0x006aba08`, `FleetBlockadeNotif`) is the hook. Add the Interdictor's gravity well projector as a blocking condition in `MovementSystem::advance`.
- **Speed model changes**: edit `BASE_TICKS_PER_HOP` and `MIN_TICKS_PER_HOP` constants in `movement.rs`. No REBEXE.EXE work needed for open-rebellion.
- **Troop transport interception**: FUN_00504a00 handles `TroopRegDestroyedRunningBlockade` (event 0x340). Changing interception probability requires patching the caller that invokes this function.

---

## 8. Economy & Production

*Manufacturing queues, resource management, research orders.*

### Economy Functions

| Address | Size | Purpose | Notif |
|---------|------|---------|-------|
| FUN_0050e820 | 1658 | **System energy/material allocator**—`Invalid SystemEnergyAllocated value!`, `Invalid SystemRawMaterialAllocated value!`. Largest economy function. |—|
| FUN_00509cc0 | 117 | **SystemEnergy validator** | `SystemSystemEnergyNotif` / `SystemEnergy` |
| FUN_00509d40 | 117 | **SystemEnergyAllocated validator** | `SystemSystemEnergyAllocatedNotif` |
| FUN_00509dc0 | 117 | **SystemRawMaterial validator** | `SystemSystemRawMaterialNotif` / `SystemRawMaterial` |
| FUN_00509e40 | 117 | **SystemRawMaterialAllocated validator** | `SystemSystemRawMaterialAllocatedNotif` |
| FUN_00509f40 | 117 | **ProductionModifier validator** (system-level) | `SystemProductionModifierNotif` |
| FUN_0055a7a0 | 117 | **ProductionModifier validator** (facility-level) | `ProcFacilProductionModifierNotif` |
| FUN_00511c90-00511dd0 | 73 each | **Economy notification dispatchers** (5 functions)—`SystemEnergy`, `SystemEnergyAllocated`, `SystemRawMaterial`, `SystemRawMaterialAllocated`, `ProductionModifier` |—|
| FUN_0055b200 | 56 | **ProcFacilProductionModifier notif** | `ProcFacilProductionModifierNotif` |
| FUN_00532e00 |—| **ConstructionYardRdOrder**—research order from construction yard (event 0x127) |—|

### Production System (rebellion-core)

Implemented in `rebellion-core/src/manufacturing.rs` (520 LOC, 13 tests):
- Per-system `VecDeque<QueueItem>` production queues
- `ticks_remaining` decremented each tick; overflow propagates to next item
- `BuildableKind`: CapitalShip/Fighter/Troop/DefenseFacility/ManufacturingFacility/ProductionFacility
- `CompletionEvent` fired when item completes

**DAT-layer production fields** (all entities):
- `refined_material_cost`: resource cost
- `maintenance_cost`: ongoing cost
- `research_order` + `research_difficulty`: tech tree position
- `production_family` / `next_production_family`: linked list through tech tree
- `processing_rate`: manufacturing facility build speed multiplier

### GNPRTB Parameters

RESRCTB.DAT (4 entries) defines resource thresholds. GNPRTB likely contains production rate modifiers, research cost multipliers, and maintenance cost scaling per difficulty.

### What to Change for a Total Conversion

- **New tech tree** (KOTOR-era weapons and ships): CAPSHPSD.DAT `research_order` and `research_difficulty` fields define the tree structure. The `production_family` / `next_production_family` linked list determines unlock chains. Edit via JSON mod overlay.
- **New resource types** (e.g., Cortosis ore for Yuuzhan Vong resistance): `SystemRawMaterial` / `SystemRawMaterialAllocated` are the resource primitives. Adding a second resource type requires new notification strings, new DAT fields, and patching FUN_0050e820.
- **Production facility specialization**: `ProcFacilProductionModifier` (FUN_0055b200) is a per-facility speed modifier. Add facility-type-specific modifiers here.

---

## 9. Scripted Events

*Luke/Vader saga, Jabba's Palace, Bounty Hunters, Dagobah training—hardcoded story event chains.*

### Luke/Vader Saga

| Address | Size | Purpose | Notif | Event ID |
|---------|------|---------|-------|----------|
| FUN_0054b770 | 51 | Luke Dagobah prerequisite | `MissionMgrLukeDagobahRequiredNotif` |—|
| FUN_0054b7b0 | 65 | Luke Dagobah mission active | `MissionMgrLukeDagobahNotif` / `LukeDagobah` | 0x221 (545) |
| FUN_00575320 | 56 | First Dagobah training day | `DagobahMissionFirstTrainingDayNotif` / `FirstTrainingDay` |—|
| FUN_0056fc70 |—| Dagobah training completed | `LukeDagobahCompletedNotif` / `DagobahCompleted` | 0x210 (528) |
| FUN_00571590 | 56 | Jedi training teacher key | `MissionJediTrainingTeacherKeyNotif` / `Teacher` |—|
| FUN_0054b900 | 51 | Darth Vader pickup | `MissionMgrDarthPickupNotif` / `DarthPickup` |—|
| FUN_0054b940 | 51 | Darth to Luke final battle | `MissionMgrDarthToLukeFinalBattleNotif` / `DarthToLukeFinalBattle` |—|
| FUN_0054b980 | 51 | Darth to Emperor final battle | `MissionMgrDarthToEmperorFinalBattleNotif` / `DarthToEmperorFinalBattle` |—|
| FUN_0054b9c0 | 51 | Final battle ready | `MissionMgrFinalBattleReadyNotif` / `FinalBattleReady` |—|
| FUN_0054ba00 | 65 | Final battle trigger | `MissionMgrFinalBattleNotif` / `FinalBattle` |—|
| FUN_0054ba50 | 51 | Luke final battle ready | `LukeFinalBattleReadyNotif` |—|

**Event ID ordering**: DagobahCompleted (0x210 = 528) < BountyAttack (0x212 = 530) < LukeDagobah (0x221 = 545). Completion events have lower IDs than requirement events—the chain is sequenced by ID.

### Jabba's Palace

| Address | Purpose | Notif |
|---------|---------|-------|
| FUN_0054b800 | Luke captured at Palace | `MissionMgrLukePalaceNotif` |
| FUN_0054b840 | Han captured at Palace | `MissionMgrHanCapturedAtPalaceNotif` |
| FUN_0054b880 | Leia captured at Palace | `MissionMgrLeiaPalaceNotif` |
| FUN_0054b8c0 | Chewbacca captured at Palace | `MissionMgrChewbaccaPalaceNotif` |

### Bounty Hunters

| Address | Size | Purpose | Notif | Event ID |
|---------|------|---------|-------|----------|
| FUN_0054ba50 | 51 | Bounty hunters activated | `MissionMgrBountyHuntersActiveNotif` |—|
| FUN_00572b40 |—| Han bounty attack | `HanBountyAttackNotif` / `BountyAttack` | 0x212 (530) |

### What to Change for a Total Conversion

**Replacing the OT saga with Thrawn Trilogy / NJO / KOTOR events:**

The story events are implemented as notification handlers in the `MissionMgr*Notif` family. Each is a small function (51-65 bytes) that fires a specific event ID. In open-rebellion, these map to `EventSystem` (`rebellion-core/src/events.rs`) entries with `EventCondition::TickReached` or `CharacterAtSystem`.

- **Replace Dagobah with Wayland** (Thrawn mod): replace the `LukeDagobah*` event chain with new events triggered by a character reaching Wayland (system DatId lookup).
- **New capture/rescue events**: follow the Palace pattern—each event is a `MissionMgr*Notif` handler triggering event ID registration. Create new event IDs starting above 0x370 (880, the current maximum).
- **Remove OT events entirely**: set `enabled: false` on the corresponding `GameEvent` entries, or stub out the `MissionMgr*Notif` handlers.
- **New final battle**: `FinalBattle` (0x221) and `FinalBattleReady` are the hooks. The `DEATHSTAR_FIRE` sound event (`0x006ab854`) and `EMPIRE_START_TURN`/`EMPIRE_END_TURN` strings confirm there are turn-structured events beyond the mission system.

---

## 10. UI & Rendering

*Strategy view, tactical view, encyclopedia, galaxy map. Note: STRATEGY.DLL and TACTICAL.DLL are resource-only—all UI code lives in REBEXE.EXE.*

### Strategy View

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0049eea0 |—| **"Battle in Progress" overlay**—Win32 GDI rendering; side-dependent color: `0xff` (Alliance blue) vs `0xff00` (Empire red) based on `battle_info_ptr+0x9c`. |
| FUN_0040a700 | 1246 | **Battle state machine**—18-state switch for strategy view battle flow |
| FUN_00421c70 | 1490 | **DLL loader**—loads `strategy.dll`, `textcomm.dll` |
| FUN_00414a80 | 1033 | **Main window handler**—loads `textcomm.dll`, `common.dll`; handles `REBAPP_UI_PHASE_COCKPIT_SCREEN` |

**Ship name strings function**: FUN_00597610 (9220 bytes) references all ship type strings: `CALAMARI`, `BULK_CRUISER`, `ASSAULT_FRIGATE`, `NEBULON_B`, `ESCORT_A`, `CORVETTE`, `MEDIUM_TRANSPORT`, `BULK_TRANSPORT`, `GUNBOAT`, `DREADNAUGHT_A`, plus 40 more. This is likely the fleet display or tactical setup function.

### Tactical View

| Address | Size | Purpose |
|---------|------|---------|
| FUN_005a7500 | 4856 | **Tactical ship object constructor**—registers all tactical battle message handlers. Full handler list: `SHIP_ADD/REMOVE/ABSTRACT_DESTROY`, `cmSHIP_POST_DS_STATUS`, `NAV_*`, `SHIPGROUP_MOVE/NEWMISSION/NEWFORMATION/NEWTARGETLIST`, all `SHIP_FIRE*` and `SHIP_TAKE_*_HIT` events, `SHIP_SHIELD_HIT/FIX`, `SHIP_WEAPON_HIT/DOWN/FIX`, `SHIP_ION_DAMAGE`. |
| FUN_0059a8d0 | 393 | **Tactical view loader**—loads `texttact.dll`, `tactical.dll`, `VoiceFxA.dll`, `VoiceFxE.dll` |
| FUN_005d9eb0 | 321 | **D3DRM camera setup**—`Failed to create the camera's frame`, viewport creation |
| FUN_005da080 | 160 | **D3DRM render loop**—`Clearing viewport failed`, `Rendering scene failed` |

**Death Star tactical event**: `cmSHIP_POST_DS_STATUS`—Death Star status update for the tactical view.

**Tactical message strings** (from rebellion-strings.txt):
- `SHIP_FIRELASERCANNON`, `SHIP_FIRETURBOLASER`, `SHIP_FIREIONCANNON`, `SHIP_FIRETORPEDO`
- `SHIP_TAKE_LASER_HIT`, `SHIP_TAKE_TURBO_HIT`, `SHIP_TAKE_TORPEDO_HIT`, `SHIP_ION_DAMAGE`
- `SHIP_SHIELD_HIT`, `SHIP_SHIELD_FIX`, `SHIP_WEAPON_HIT`, `SHIP_WEAPON_DOWN`, `SHIP_WEAPON_FIX`
- `TASKFORCE_NEW`, `DEATHSTAR_FIRE`, `EMPIRE_START_TURN`, `EMPIRE_END_TURN`

### Encyclopedia

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0045d400 | 893 | **Encyclopedia DLL loader**—loads `encytext.dll`, `encybmap.dll` |
| FUN_00466350 | 351 | **Encyclopedia bitmap loader**—loads `encybmap.dll` |
| FUN_0052d920 | 2894 | **Unit description/display**—references TEXTSTRA string IDs. Second-largest function after the ship-name function. Likely renders entity descriptions for the encyclopedia panel. |

In open-rebellion, the encyclopedia is implemented in `crates/rebellion-render/src/encyclopedia.rs` with 4 tabs and BMP texture cache from `data/base/EData/`. HD-upscaled PNGs go in `data/hd/EData/`.

### Multiplayer / Network

| Address | Size | Purpose |
|---------|------|---------|
| FUN_005f9860 | 1601 | **CommMgr::ReceiveIncomingCommPacket**—DirectPlay receive handler; `CM_PLAYER_LEFT_UNEXCUSED`, `DPSYS_DELETEPLAYER/ADDPLAYER` |
| FUN_005f7ea0 | 1361 | **CommMgr::Flush**—DirectPlay send; error handling |
| FUN_005f8600 | 316 | **CommMgr::CleanUp**—session teardown |
| FUN_005fa400 | 307 | **CommMgr::PingOpponents**—latency check |
| FUN_0040ec60 | 824 | **MultiPlayerWindow**—multiplayer UI, cancel/quit session |

**Network architecture**: DirectPlay (DirectX 6 era). Not relevant for open-rebellion (uses Rust-native networking or local-only).

### What to Change for a Total Conversion

- **New galaxy map layout**: in open-rebellion, the galaxy map is `crates/rebellion-render/src/lib.rs` (`draw_galaxy_map`). No REBEXE.EXE work needed—the map is fully re-implemented.
- **New tactical view elements**: the 2D tactical view (FUN_005a7500) registers message handlers. New weapon types need new `SHIP_FIRE*` message strings and corresponding handler registration. The Direct3D Retained Mode (D3DRM) renderer at FUN_005d9eb0 is Win32-specific and irrelevant for open-rebellion.
- **New ship names in encyclopedia**: replace FUN_00597610's string set, or—in open-rebellion—edit `data/encyclopedia.json` entries directly.
- **Custom UI panels**: open-rebellion's egui panels (`crates/rebellion-render/src/panels/`) are fully modifiable. No REBEXE.EXE dependency.

---

## Appendix A: Entity Family Byte Reference

| Range | Type | Notes |
|-------|------|-------|
| 0x08-0x0f | Characters (major + minor) | 6 major, 54 minor in base game |
| 0x10-0x13 | Troops (general) | Non-combat-eligible sub-range |
| 0x14-0x1b | Troops / Special Forces | Combat-eligible ground units |
| 0x1c-0x1f | Facilities | Defense, manufacturing, production |
| 0x30-0x3b | Capital Ships + Fighters | All space combat units |
| 0x34 | Death Star | Special handler (FUN_005617b0) |
| 0x71-0x72 | Fighter squadron types | Alt shield absorption path |
| 0x73-0x74 | Special combat entities | Special victory path (FUN_00534640) |
| 0x90-0x98 | Star Systems | 0x90=explored, 0x92=unexplored |
| 0x98 | Battle context | `0x98000481` = space battle context DatId |

---

## Appendix B: Event ID Registry

| ID (hex) | ID (dec) | Event | Source |
|---------|---------|-------|--------|
| 0x127 | 295 | ConstructionYardRdOrder | FUN_00532e00 |
| 0x12c | 300 | RecruitmentDone | FUN_00532f40 |
| 0x14d | 333 | Battle | FUN_00512280 |
| 0x14e | 334 | Blockade | FUN_005122d0 |
| 0x152 | 338 | UprisingIncident | FUN_00512580 |
| 0x1a0 | 416 | SquadSizeDamage | FUN_005038e0 |
| 0x1c0 | 448 | HullValueDamage | FUN_005029a0 |
| 0x1c1 | 449 | ShieldRechargeRateCHAllocated | FUN_005029f0 |
| 0x1c2 | 450 | WeaponRechargeRateCHAllocated | FUN_00502a40 |
| 0x1e1 | 481 | CharacterForce | FUN_004f1e00 |
| 0x1e5 | 485 | CharacterForceTraining | FUN_004f1ea0 |
| 0x210 | 528 | DagobahCompleted | FUN_0056fc70 |
| 0x212 | 530 | BountyAttack | FUN_00572b40 |
| 0x221 | 545 | LukeDagobah | FUN_0054b7b0 |
| 0x340 | 832 | TroopRegDestroyedRunningBlockade | FUN_00504a00 |
| 0x370 | 880 | MissionEspionageExtraSystem | FUN_005738f0 |

---

## Appendix C: Combat Phase Flag Register

Located at `entity+0x58` (space combat state object):

| Bit | Mask | Meaning |
|-----|------|---------|
| 0 | 0x0001 | Space combat active |
| 1 | 0x0002 | Weapon fire phase (attacker side) |
| 2 | 0x0004 | Weapon fire type code |
| 5 | 0x0020 | Shield absorb type code |
| 6 | 0x0040 | Combat phases enabled (shields/hull/fighters enabled after weapon fire) |
| 7 | 0x0080 | Hull damage type code |
| 8 | 0x0100 | Fighter engagement type code |
| 9 | 0x0200 | Victory flag (set when a side wins) |
| 12 | 0x1000 | Post-combat phase trigger |
| 13 | 0x2000 | Fleet eval skip flag |

---

## Appendix D: Notification Architecture

All 100+ notification functions follow one of two patterns.

**Pattern A—Validate + Set + Notify** (setter functions, e.g., FUN_00501490):
```
FUN_0053a000 (entity validity)
→ vtable dispatch (get per-entity max value)
→ FUN_0053fc90 (range check: [min, max])
→ write new value to entity field
→ FUN_00539fd0 × 2 (get side 1 and side 2 observers)
→ FUN_005023b0 / FUN_004f04f0 (dispatch typed notification)
→ vtable dispatch (fire change event with old_value, new_value, context)
```

**Pattern B—Notify Only** (event dispatchers, e.g., FUN_004f1e00):
```
FUN_0053a010 (validity—notification path)
→ FUN_004f8980 / FUN_004f8880 / FUN_004f8aa0 (dispatch notification)
→ FUN_0053fcf0 / FUN_0053fdd0 / FUN_0053fe40 (register event with numeric ID)
```

The three notification dispatchers differ by parameter count:
- `FUN_004f8880(this, notif_str, event_str, param1, null, ctx)`—1 entity param
- `FUN_004f8980(this, notif_str, event_str, param2, param1, null, ctx)`—2 entity params, reversed order
- `FUN_004f8aa0(this, notif_str, event_str, ref2, ref1, null, ctx)`—2 entity reference params

---

## Appendix E: Cross-Reference Index

### By System → Functions

| System | Key Functions |
|--------|---------------|
| Hull damage | FUN_00501490, FUN_005029a0, FUN_005443f0, FUN_00543cb0 |
| Shield recharge | FUN_00501510, FUN_005029f0, FUN_00544130, FUN_00543bd0 |
| Weapon recharge | FUN_005015a0, FUN_00502a40 |
| Fighter squadrons | FUN_005032c0, FUN_005038e0, FUN_005444e0, FUN_00543d20 |
| Ground combat | FUN_00560d50, FUN_004ee350, FUN_005617b0 |
| Bombardment | FUN_00556430, FUN_00555b30, FUN_0055d8c0, FUN_0055d860 |
| Space combat pipeline | FUN_005457f0, FUN_00549910, FUN_005442f0-FUN_00544a20 |
| Loyalty | FUN_005341a0, FUN_004ee030 |
| Force/Jedi | FUN_0058a3f0, FUN_004f1e00, FUN_004f1ea0, FUN_004f20e0 |
| Mission dispatch | FUN_0050d5a0, FUN_004fc080, FUN_004fc3a0 |
| Mission scripted events | FUN_0054b770–FUN_0054ba50, FUN_00572b40 |
| Fleet movement | FUN_004ff7a0–FUN_004ff890, FUN_00504a00 |
| Economy | FUN_0050e820, FUN_00509cc0–FUN_00509f40, FUN_00511c90–FUN_00511dd0 |
| System control | FUN_00511ec0, FUN_00511f40, FUN_005121e0–FUN_005126c0 |
| Victory | FUN_00543d90, FUN_00534640, FUN_0054ba00 |
| UI—Strategy | FUN_0049eea0, FUN_0040a700 |
| UI—Tactical | FUN_005a7500, FUN_005d9eb0 |
| UI—Encyclopedia | FUN_0045d400, FUN_0052d920 |
| GNPRTB loading | FUN_00569280 |

### By DAT File → Functions Consuming It

| DAT File | Used By |
|----------|---------|
| CAPSHPSD.DAT | FUN_00501490 (hull max via vtable), FUN_005438a0 (type codes) |
| GNPRTB.DAT | FUN_0055d8c0 (bombardment divisor via DAT_006bb6e8) |
| `*MSTB.DAT` files | Mission probability lookup (traced via FUN_0050d5a0 dispatch chain) |
| UPRIS1TB.DAT / UPRIS2TB.DAT | FUN_005121e0, FUN_00511f40 (uprising threshold consumers) |
| MISSNSD.DAT | FUN_0050d5a0, FUN_004fc080 (mission type switch) |
| MJCHARSD / MNCHARSD | FUN_005341a0 (loyalty), FUN_004ee030 (enhanced loyalty), FUN_004ee470 (hyperdrive), FUN_0058a3f0 (Force) |
