---
title: "Modder's Function Taxonomy — Star Wars Rebellion (REBEXE.EXE)"
description: "Categorized function taxonomy of 22,741 REBEXE.EXE functions for modders"
category: "ghidra"
created: 2026-03-15
updated: 2026-10-14
---

# Modder's Function Taxonomy — Star Wars Rebellion (REBEXE.EXE)

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
| FUN_00501490 | — | **Hull damage validator/setter** — validates `new_hull ∈ [0, max_hull]`, writes to `entity+0x60`, fires `CapShipHullValueDamageNotif` | `FUN_00501490.c` |
| FUN_00501510 | — | **Shield recharge rate setter** — writes 4-bit value to bits 0-3 of packed field at `entity+0x64`, fires `ShieldRechargeRateCHAllocated` | `FUN_00501510.c` |
| FUN_005015a0 | — | **Weapon recharge rate setter** — writes 4-bit value to bits 4-7 of packed field at `entity+0x64`, fires `WeaponRechargeRateCHAllocated` | `FUN_005015a0.c` |
| FUN_005032c0 | — | **Squad size damage setter** — validates `new_size ∈ [0, max_squad_size]`, writes to `entity+0x60`, fires `FightSquadSquadSizeDamageNotif` | `FUN_005032c0.c` |
| FUN_004ee350 | 30 | **Character CommandKind setter** — `entity+0x96` (short, non-negative: 0 none, 1 Commander, 2 Admiral, 3 General), notifies event `0x1e2` and fires vtable `+0x330` | `FUN_004ee350.c` |
| FUN_00509cc0 | 117 | **SystemEnergy validator** — `Invalid SystemEnergy value!` error | — |
| FUN_00509d40 | 117 | **SystemEnergyAllocated validator** | — |
| FUN_00509dc0 | 117 | **SystemRawMaterial validator** | — |
| FUN_00509e40 | 117 | **SystemRawMaterialAllocated validator** | — |
| FUN_00509f40 | 117 | **ProductionModifier validator** (system-level) | — |
| FUN_0055a7a0 | 117 | **ProductionModifier validator** (facility-level) | — |
| FUN_0053fc90 | — | **Range validator** — `validate_range(value, min, max)`. Universal range-check primitive used by all setters | — |
| FUN_0053e0f0 | — | **Secondary clamp** — `clamp(value, 0, 0x7fff)`. Prevents short overflow. Applied after range check in loyalty and enhanced loyalty | — |
| FUN_0053a000 | — | **Entity validity check** — entry guard for all setter functions | — |
| FUN_0053a010 | — | **Entity validity check (notification path)** — used by notification dispatchers | — |

### Entity Object Layout (Offset Map)

```
+0x24  uint   owner_bits          bits 6-7 = side (1 Alliance, 2 Empire)
+0x50  uint   status_flags        bit0=active, bit2=completed, bit3=destroyed, bit4=en route, bit5=en route active, bit6=existing, bit11=autorouting, bit12=autoscrap request
+0x60  int    hull_current        Capital ships: current hull. Fighters: current squad size.
+0x64  uint   shield_weapon_packed  bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated
+0x66  short  loyalty_base        Character base loyalty 0-100
+0x78  uint   role_flags          characters: bit0=decoy, bit4=may resign its mission, bit6=adrift, bit7=on a mission
+0x8a  short  enhanced_loyalty    Mission bonus loyalty 0-100 (distinct from base)
+0x96  short  command_kind        Characters: officer rank, 0 none, 1 Commander, 2 Admiral, 3 General
+0x9a  short  hyperdrive_modifier Han Solo fleet speed bonus
+0xac  byte   character_flags     bit0=prisoner, bit4=Force aware, bit5=Force sensitive, bit7=discoverer
```

`+0x50` bits are named in `build-delivery.md`, the character fields in `character-roster.md`, `story-missions.md` and `entity-system.md`. `+0x58` is the MissionMgr story-flag word (Appendix C) and, on a fleet, bit 6 marks the fleet's bombard order (`bombardment.md`).

### GNPRTB Parameters

GNPRTB parameters control balance at a table level rather than per-entity. Bombardment's parameters are in §2; two globals recur in the arithmetic helpers:

| GNPRTB Global | Purpose | Notes |
|---------------|---------|-------|
| `DAT_006bb6e8` | GNPRTB `0x1400`, transit distance divisor | 5 in every column; `FUN_0055d8c0` divides the system distance by it (§2, Transit Time) |
| `DAT_00661a88` | Percentage base | 100; `FUN_0053e190(x, s)` = `s * x / 100` through `FUN_0053e170` |

### What to Change for a Total Conversion

**Yuuzhan Vong / KOTOR / Thrawn era stat overrides:**
- Capital ship hull, shields, weapon recharge: patch the DAT fields in CAPSHPSD.DAT (`hull`, `shields_strength`, `shields_recharge`, weapon arc values). The setter functions read per-entity maxima from vtable `+0x248` (hull) and `+0x244` (squad size) — these vtable-resolved values come from the DAT record.
- The open-rebellion mod system patches these at the `world/` layer via RFC 7396 JSON overlay. No code change needed for stat rebalancing: create a `capital_ships.json` with `{ "id": N, "hull": X }` entries.
- **New weapon types**: vtable slot `+0x1c4` dispatches weapon fire by type. Adding a new weapon type requires a new vtable handler and a new combat phase flag bit.
- **Shield/weapon recharge range**: both are 4-bit (0-15 max). Expanding the packed field at `+0x64` to allow higher values requires patching FUN_00501510, FUN_005015a0, and their `get_max_*` callers.

---

## 2. Combat Resolution

*Space battles run the tactical engine, played or simulated. Bombardment, Destroy System and planetary assault are fleet orders. The object-move dispatcher `FUN_00514a60` and the MissionMgr story chain `FUN_00549910` apply no combat damage.*

### Battle Entry

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0040a700 | 1246 | **Battle state machine** — 18-state switch on `STRAT_INIT → STRAT_BATTLE_FINISH`. Entry for player-initiated battles. Strings: `STRAT_ALL_BATTLE_START`, `STRAT_BATTLE_CHOOSE_TYPE`, `STRAT_BATTLE_CONVERT_TO_TACT`, `STRAT_BATTLE_CONVERT_TO_TACT_ABSTRACT`, etc. |
| FUN_005a7500 | 4856 bytes | **Space combat tactical constructor** — registers message handlers for the 2D tactical view: `SHIP_ADD/REMOVE/ABSTRACT_DESTROY`, all `SHIP_FIRE*` and `SHIP_TAKE_*_HIT` events, `SHIPGROUP_*`, `NAV_*`, `cmSHIP_POST_DS_STATUS`. |
| FUN_005ae9c0 | — | **Headless tactical step** — ranks opponents, assigns targets and applies each group's damage rate once a second; the 1200-second stalemate rule ends every battle. |

Every space battle, windowed or simulated from the Battle Alert, runs one tactical session; the only hull writers are the tactical import (`FUN_00501490` ← `FUN_00501ae0` / `FUN_00501b70` ← `FUN_0040cfc0` / `FUN_0040d170` ← `FUN_0040c780`). See `simulate-runs-tactical-engine.md`, `tactical-engagement-ai.md` and `tactical-weapon-loop.md`.

### Object Moves (`FUN_00514a60`)

`FUN_00514a60(obj, from, to, ctx)` is the fire slot of event `0x320` (transit start; only caller `FUN_0057ba50`, slot 8 of vtable `0x006697d8`). In order it runs:

1. the MissionMgr hook `FUN_005457f0`, which runs the story chain when `obj` is one of the six story characters (below);
2. the character manager's hook `FUN_00560d50` (below);
3. while `obj +0x50` bit 2 (completed) is clear and `+0x38` names a manager, that manager's `FUN_00529d20`;
4. transit start `FUN_00556430` (below);
5. the old and new containers' slot `+0xc4` when `obj +0x50` bit 6 (existing) is set and slot `+0xc8` when bit 0 (active) is set.

`build-delivery.md` traces the event and the transit start.

### Story-Flag Checks (`FUN_005457f0` → `FUN_00549910`)

`FUN_005457f0` calls `FUN_00549910` only when `FUN_005438a0(obj)` is true. `FUN_005438a0` is true for exactly six character ids: Leia Organa `0x31000241`, Luke Skywalker `0x32000242`, Han Solo `0x33000243`, Emperor Palpatine `0x34000280`, Darth Vader `0x35000281` and Chewbacca `0x38000343` (MJCHARSD/MNCHARSD with TEXTSTRA names).

`FUN_00549910` runs ten checks in a fixed order and ANDs the results. Each check sets or pulses one bit of the MissionMgr story-flag word `+0x58` (Appendix C), and each bit's handler starts a story mission or move:

| Check | Bit | Story event |
|---|---|---|
| FUN_005442f0 | 6 | Han a prisoner at Jabba's Palace (`0x98000481`) |
| FUN_00544da0 | 12 | Final battle ready |
| FUN_00544030 | 2 | Luke to Dagobah (mission `0x71000043`) |
| FUN_00544130 | 5 | Luke to the Palace; when Luke's current mission is family `0x71` (Dagobah) in state 8, calls that mission's slot `+0x1d8` |
| FUN_005443f0 | 7 | Leia to the Palace |
| FUN_005444e0 | 8 | Chewbacca to the Palace |
| FUN_005445d0 | 9 | Vader's Pickup (mission `0x73000082`); reads Vader's current mission family `0x73` and asks him to resign another mission through `FUN_00534640` |
| FUN_00544a20 | 10 | Vader to Luke |
| FUN_00544be0 | 11 | Vader to the Emperor |
| FUN_00544fc0 | 13 | Final battle (event `0x220`) |

`FUN_005434b0` (Luke), `FUN_00543550` (Leia), `FUN_005435f0` (Han), `FUN_00543690` (Chewbacca), `FUN_00543760` (Vader) and `FUN_00543800` (Palpatine) are fixed-id accessors used by the checks. A character is free (`FUN_005439e0`) when `+0xac` bit 0 (prisoner) is clear, `+0x50` bit 0 (active) is set, `+0x50` bit `0x1000` is clear and `+0x78` bits 6 and 7 (adrift, on a mission) are clear. See `story-missions.md` and `story-encounters.md`.

`FUN_0054a1d0`, a bit broadcaster in `FUN_0054a140`-`FUN_0054a770`, passes `+0x24 >> 4 & 3` to `FUN_004fd600`, which returns the Alliance (1) or Empire (2) view; no difficulty modifier is read there.

### Character Command Hook (`FUN_00560d50`)

| Address | Lines | Purpose |
|---------|-------|---------|
| FUN_00560d50 | 230 | **Character manager's move hook.** Gates on the moved object's family. A moved capital ship (`0x14`-`0x1b`) clears the CommandKind of every character aboard (`FUN_004f25c0`, filter `[0x30,0x3c)`) through `FUN_004ee350(…, 0, …)`. A moved character (`0x30`-`0x3b`) keeps a non-zero CommandKind only when it moves between two capital ships whose containers are the same fleet (`0x08`-`0x0f`); otherwise it is cleared. Family `0x34` exactly (the Emperor) also calls `FUN_005617b0`. |
| FUN_004ee350 | 30 | **CommandKind setter** — writes character `+0x96` (short, non-negative: 0 none, 1 Commander, 2 Admiral, 3 General), notifies both side views and fires vtable `+0x330` (`FUN_004f1f80`, `CharacterCommandKindNotif`, event `0x1e2`). |
| FUN_005617b0 | 68 | **Emperor-at-Coruscant flag.** Palpatine (`FUN_005070d0`, `0x34000280`) not a prisoner (`+0xac` bit 0 clear), active (`+0x50` bit 0) and contained by Coruscant `0x90000109` with the Empire's side bits (`+0x24 & 0xc0 == 0x80`) → `FUN_0055f650(1)`, else `FUN_0055f650(0)`. The flag raises event `0x230` (`SeatOfPower`, "Emperor Arrives at Coruscant"). |

The hook has no gate on regiments (`0x10`-`0x13`) or special forces (`0x3c`-`0x3f`). Officer rank is `character-roster.md`; the family gates are `ground-unit-assets.md`.

### Planetary Assault

Order `0x234` (Planetary Assault) is resolved in `0x0058c120`-`0x0058c8b0` (`rebexe-map.md`). Where regiment-versus-regiment combat is resolved, and which class fields (attack `+0x64`, defense `+0x68`, bombardment defense `+0x60`) it reads, is open: task-graph nodes `or-q-ground-resolution-nms` and `or-q-regiment-combat-eaxn`.

### Bombardment

Orders `0x220` Target Military Facilities, `0x221` Target Civilian Facilities, `0x222` General Bombardment and `0x223` Destroy System (`bombardment.md`):

| Address | Purpose |
|---------|---------|
| FUN_00582440 | Gate: every team fleet needs a non-zero slot-`+0x1dc` sum; Destroy System also needs a ship of family `0x18`-`0x1b` |
| FUN_0058d800 | Setup per order (military, civilian, energy and raw material, collateral, Destroy System), then `FUN_0058d770` |
| FUN_0058d990 | Ship phase: with Destroy System and a family-`0x18` ship, destroys the system (`FUN_0058ce70`, cause `0x10` own / `0xf` enemy) and rolls each minor character at GNPRTB `0x606` |
| FUN_0058dc10 | Defence phase: each KDY-150 (`0x22`) then LNR (`0x23`) fires at a random bombarding ship |
| FUN_0058df60 | Effect phase: hits = slot-`+0x1dc` sum × (Admiral leadership / GNPRTB `0x601` + 1), minus shield generators (family `0x24`); each hit rolls a random target against its value |
| FUN_0058e320 | Finalize: support shift GNPRTB 7705 (−20) for the bombarding side, sector or galaxy shifts, clear the bombard bits |

The system's holder is the only target side; nothing branches by side except the own/enemy cause.

### Transit Time

`FUN_00556430(obj, from, to, ctx)` starts travel when `from` and `to` lie in different systems (`FUN_00555540` returns an object's system):

```
FUN_00556430 → FUN_00555d30 → FUN_00555b30 → FUN_0055d8c0(coords(a), coords(b), speed)
  coords:  FUN_00509620 — SYSTEMSD x_position / y_position (record +0x2c, then +0x4c)
  d      = isqrt((b.x - a.x)² + (b.y - a.y)²)          // FUN_0055d860, FUN_0053e1d0
  ticks  = ((d / GNPRTB[0x1400]) * speed) / 100          // DAT_006bb6e8, FUN_0053e190
  ticks  = max(ticks, 1) when d > 0
```

GNPRTB `0x1400` (5120) is 5 in every column; `speed` is a time percentage (100 default, larger is slower). The object carries its own transit state (`+0x50` bits 5 and 11, destination key `+0x3c`, arrival tick `+0x44`, pending event `0x387`). See `build-delivery.md`.

**Decompiled files**: `FUN_00556430.c`, `FUN_00555d30.c`, `FUN_00555b30.c`, `FUN_0055d8c0.c`, `FUN_0055d860.c`, `FUN_0053e190.c`, `FUN_00509620.c`.

### Death Star

The Death Star is CAPSHPSD record 136 with `family_id` `0x18`; every other capital ship is `0x14`. Rules key on the range `[0x18,0x1c)`: Destroy System's gate and kill phase, the per-system cap (`FUN_00507ab0` against `FUN_0040f340`, literal 1), sector bit 11 (`FUN_005178b0`). Destroying a system destroys its regiments, ships, fighters and facilities; fleets survive with their contents (slot `+0xb4` `FUN_00524fb0`); characters at the system turn adrift. See `death-star-strategic.md`.

### GNPRTB Parameters Affecting Combat

| GNPRTB / Global | Controls | Notes |
|-----------------|---------|-------|
| `DAT_006bb6e8` (GNPRTB `0x1400`) | Transit distance divisor | 5 in every column (`FUN_0055cb60`) |
| `DAT_00661a88` | Percentage base of `FUN_0053e170` | 100 (`blockade-troop-withdrawal.md`) |
| GNPRTB `0x600`, `0x601` | Leadership divisors for bombardment defence and hits | 40 |
| GNPRTB `0x602`, `0x603` | Bombardment hit draw | 1..10 |
| GNPRTB `0x604`, `0x605` | Energy and raw-material target value | 9 |
| GNPRTB `0x606` | Character hit chance under Destroy System | 50 |
| GNPRTB 7705 | Support shift for the bombarding side | −20 |

Tactical battle parameters are in `tactical-weapon-loop.md` and `tactical-engagement-ai.md`.

### What to Change for a Total Conversion

- **Space combat rules** (e.g., a Force-power effect for a KOTOR mod): the tactical engine — the headless step `FUN_005ae9c0` and the weapon loop (`tactical-weapon-loop.md`).
- **A second superweapon** (Yuuzhan Vong worldship): a CAPSHPSD record with `family_id` `0x18` passes every strategic test for either side, from DAT alone. It meets the one-per-system cap (`FUN_0040f340`, literal 1), the single tactical Death Star slot and the tactical side-1 attacker test; families `0x19`-`0x1b` pass the range tests but have no object factory (`death-star-strategic.md`).
- **Bombardment balance**: GNPRTB `0x600`-`0x606` and 7705 (DAT); the per-hit roll and target values are code in `FUN_0058df60` (`bombardment.md`).
- **Travel speed**: GNPRTB `0x1400` and the class hyperdrive fields (DAT).
- **A new story beat**: a new bit, setter and handler slot on the MissionMgr and a new check in the `FUN_00549910` chain (`story-encounters.md`). The Emperor's seat of power `0x90000109` is a literal in `FUN_005617b0`.

---

## 3. GNPRTB Parameter System

*34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side) = 111 total parameter bindings. GNPRTB.DAT has 213 entries × 8 difficulty values = 1,704 i32 scalars. SDPRTB.DAT has 35 entries × 76 bytes each.*

### Parameter Table Loader

| Address | Size | Purpose |
|---------|------|---------|
| FUN_00569280 | 91 | **DAT format dispatcher** — recognizes `IntTableEntry`, `SideParamTableEntry`, `SeedFamilyTableEntry`, `SeedTableEntry`, `GeneralParamTableEntry`. Entry point for loading all parameter tables at startup. |

### Confirmed Global Bindings

| Address | GNPRTB Role | Value Notes |
|---------|-------------|-------------|
| `DAT_006bb6e8` | GNPRTB `0x1400`, transit distance divisor | 5 in every column (`FUN_0055cb60`) |
| `DAT_00661a88` | Percentage base | 100, used by `FUN_0053e170` / `FUN_0053e190` |
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
| UPRIS1TB.DAT | Uprising incident outcome (first code) |
| UPRIS2TB.DAT | Uprising incident outcome (second code) |
| RLEVADTB.DAT | Rebel evasion probability |
| RESRCTB.DAT | Resource threshold |
| TDECOYTB.DAT | Troop decoy success |

### What to Change for a Total Conversion

**No code change required for most balance tuning:**
- Edit GNPRTB.DAT values directly (or via the open-rebellion JSON mod overlay for `general_params.json`) to adjust combat recovery, travel time, diplomacy shifts, uprising thresholds, etc.
- Edit `*MSTB.DAT` entries to change mission success probability curves.
- SDPRTB.DAT adjusts per-faction modifiers — useful for asymmetric faction design (Thrawn Empire vs Remnant, etc.).

**Requires code change:**
- Adding a 214th GNPRTB parameter: the parameter loader FUN_00569280 must be patched to accept the higher count, and all array accesses updated.
- Changing difficulty levels from 4 to more values: `entity+0x24 bits 4-5` is 2-bit (0-3 only). Expanding to 3 bits requires patching FUN_004fd600 and all callers.

---

## 4. Mission System

*Mission types, dispatch, probability formulas, lifecycle management. All 25 mission definitions live in MISSNSD.DAT.*

### Core Mission Functions

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0050d5a0 | 3050 | **Observation level** — copies a system's state into one side's view at level 0-12 (`observation-levels.md`); espionage and mission ends call it |
| FUN_00536800 | 120 | **Mission role notification** — `RoleMissionKeyNotif` / `Mission` |
| FUN_004fc080 | 89 | **Mission destruction outcome** — handles `DestroyedSabotage` (type 6), `DestroyedAssassination` (type 7), `Autoscrap` (type 0x15 = 21) |
| FUN_004fc3a0 | 16 | **Damage application** — `GameObjDamagedNotif` / `Damaged` |

### Mission Lifecycle Notifications

| Address | Notif String | Event ID |
|---------|-------------|----------|
| FUN_00524fc0 | MissionUserIDNotif / UserID | — |
| FUN_00524fc0-00525120 | Target, Objective, Leader, OriginLocation | — |
| FUN_00525560 | ReadyForNextPhaseNotif | — |
| FUN_005255a0 | ImpliedTeamNotif | — |
| FUN_005255e0 | MandatoryNotif | — |
| FUN_00536880 | MissionSeedKeyNotif / MissionSeed | — |
| FUN_005368c0 | ParentAtMissionCompletionKeyNotif | — |
| FUN_00536900 | LocationAtMissionCompletionKeyNotif | — |
| FUN_00536980 | MovingBetweenMissionsNotif | — |
| FUN_005369c0 | MissionRemoveRequestNotif | — |
| FUN_00536a00 | MissionResignRequestNotif | — |
| FUN_00536a40 | CanResignFromMissionNotif | — |
| FUN_00536b00 | OnMissionNotif / OnMission | — |
| FUN_00536b40 | OnHiddenMissionNotif | — |
| FUN_00536b80 | OnMandatoryMissionNotif | — |

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

- **New mission type** (e.g., Yuuzhan Vong "Voxyn strike team"): a second MISSNSD.DAT record in one of the 22 registered mission families needs no code; a new mission family needs a family-to-class row in `FUN_0051ebf0` (two free slots) and a class (`record-registry.md`). Then create a new `*MSTB.DAT`-style table for its probability curve, and give the record with appropriate `special_force_eligibility` bitmask.
- **Change success rates without code changes**: edit the threshold/value pairs in the relevant `*MSTB.DAT` file. The probability formula reads these at runtime.
- **New story mission**: add a new `GameEvent` in the `EventState` system (`rebellion-core/src/events.rs`) with `EventCondition::TickReached` or `CharacterAtSystem` and the appropriate `EventAction`. No REBEXE.EXE patching required for open-rebellion.

---

## 5. Character & Force System

*Skill enhancement, Jedi training, Force detection, loyalty, betrayal, and special character abilities.*

### Loyalty System

| Address | Purpose | Notif / Event ID |
|---------|---------|-----------------|
| FUN_005341a0 | **Base loyalty setter** — `entity+0x66` (short), 0-100, vtable `+0x238` | SystemLoyaltyNotif / Loyalty |
| FUN_004ee030 | **Enhanced loyalty setter** (mission bonus) — `entity+0x8a` (short), 0-100 clamped + 0x7fff clamp, vtable `+0x318` | EnhancedLoyaltyNotif |
| FUN_00535380 | **Loyalty notification dispatcher** | LoyaltyNotif |

Two distinct loyalty fields:
- `+0x66`: base loyalty (diplomacy missions, betrayal mechanics)
- `+0x8a`: mission-bonus loyalty (EnhancedLoyalty special ability)

### Skill Enhancement System

| Address | Purpose | Notif |
|---------|---------|-------|
| — | Enhanced diplomacy | `CharacterEnhancedDiplomacyNotif` / `EnhancedDiplomacy` |
| — | Enhanced espionage | `CharacterEnhancedEspionageNotif` / `EnhancedEspionage` |
| — | Enhanced combat | `CharacterEnhancedCombatNotif` / `EnhancedCombat` |
| FUN_004ee030 | Enhanced loyalty | `CharacterEnhancedLoyaltyNotif` / `EnhancedLoyalty` |
| FUN_004ee470 | Han Solo hyperdrive modifier | `MissionHyperdriveModifierNotif` / `MissionHyperdriveModifier` |

**Hyperdrive modifier** (`+0x9a`, short): applied to Han Solo's fleet movement speed. Non-negative only (no upper bound). Consumed in `rebellion-core/src/movement.rs` when calculating `ticks_per_hop`.

### Jedi / Force System

| Address | Size | Purpose | Event ID |
|---------|------|---------|----------|
| FUN_004f1e00 | 73 | **Force attribute change notif** — `CharacterForceNotif` / `Force` | 0x1e1 (481) |
| FUN_004f1e50 | 73 | **Force experience notif** — `CharacterForceExperienceNotif` / `ForceExperience` | — |
| FUN_004f1ea0 | 73 | **Force training notif** — `CharacterForceTrainingNotif` / `ForceTraining` | 0x1e5 (485) |
| FUN_004f20e0 | 73 | **Force user discovered notif** — `CharacterForceUserDiscoveredKeyNotif` | — |
| FUN_004f2240 | 51 | **ForceAware notif** — `CharacterForceAwareNotif` | — |
| FUN_004f2280 | 51 | **ForcePotential notif** — `CharacterForcePotentialNotif` | — |
| FUN_004f2300 | 51 | **DiscoveringForceUser notif** — `CharacterDiscoveringForceUserNotif` | — |
| FUN_0058a3f0 | 306 | **Force user detection** — `force-detect` string. Calculates detectability from 2-bit Force tier + experience + detector's potential | — |

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

- **New Force powers** (KOTOR mod — Force Lightning, Battle Meditation): add new notification strings following the `CharacterForce*Notif` pattern, add new vtable slots, implement new skill effects.
- **Different Jedi training mechanics** (no Dagobah, multiple teachers): FUN_0056fc70 (DagobahCompleted, event 0x210) and FUN_00571590 (Teacher key notif) define the training chain. Patch or replace the Dagobah prerequisite check in the mission manager.
- **More Force tiers** (0-3 is 2-bit — only 4 levels): expanding to 3 bits (0-7) requires patching FUN_0058a3f0's `>> 6 & 3` extraction to `>> 5 & 7` and updating FUN_0055e4d0's tier comparisons.
- **Betrayal mechanic**: `not betray` string at `0x006ab3dc` confirms a betrayal check function exists. The `is_unable_to_betray` flag in MJCHARSD.DAT/MNCHARSD.DAT is the DAT-layer gate. FUN_005341a0 handles loyalty changes that could trigger betrayal.

---

## 6. Faction & Victory

*Victory conditions, faction control, recruitment, uprising mechanics.*

### Victory Conditions

| Address | Size | Purpose | Notif |
|---------|------|---------|-------|
| FUN_00532e00 | — | **Construction yard research order notif** — `SideConstructionYardRdOrderNotif` (event 0x127). The victory flags are FUN_005180e0 (`victory-conditions.md`). | — |
| FUN_00532f40 | 65 | **Recruitment done notif** — `SideRecruitmentDoneNotif` / `RecruitmentDone` (event 0x12c) | — |
| — | — | **Victory condition check** — referenced by `SideVictoryConditionsNotif` / `VictoryConditions` string at `0x006aa764` | — |
| FUN_0054ba00 | 65 | **Final battle trigger** — `MissionMgrFinalBattleNotif` / `FinalBattle` | — |
| FUN_0054b9c0 | 51 | **FinalBattleReady notif** | — |

**Victory sound events**: `VICTORY` at `0x006ab5f8`, `VICTORY_II` at `0x006ab5b0`.

### System Control

| Address | Purpose | Notif | Event ID |
|---------|---------|-------|----------|
| FUN_00511ec0 | System control change — battle won | `ControlKindBattleWonNotif` | — |
| FUN_00511f40 | System control change — uprising | `SystemControlKindUprisingNotif` / `ControlKindUprising` | — |
| FUN_005121e0 | Uprising handler | `SystemUprisingNotif` / `Uprising` | — |
| FUN_005122d0 | Blockade system handler | `SystemBlockadeNotif` / `Blockade` | 0x14e (334) |
| FUN_00512280 | System battle notification | `SystemBattleNotif` / `Battle` | 0x14d (333) |
| FUN_00512440 | Unit fast repair | `SystemCombatUnitFastRepairNotif` / `CombatUnitFastRepair` | — |
| FUN_00512580 | Uprising incident | `SystemUprisingIncidentNotif` / `UprisingIncident` | 0x152 (338) |
| FUN_00512540 | Battle pending caused blockade | `BattlePendingCausedCurrentBlockade` | — |
| FUN_005126c0 | Blockade + battle pending management required | `BlockadeAndBattlePendingManagementRequired` | — |

**Loyalty-driven control**: `LoyaltyCausedCurrentControlKind` notif string confirms that loyalty thresholds directly trigger faction control changes. UPRIS1TB and UPRIS2TB are read by `FUN_00559ce0`, which the uprising incident (`FUN_0050d030`) uses to pick outcome codes; see `ghidra/notes/uprising-incident.md`.

**System state notification strings** (from rebellion-strings.txt):
- `SystemLoyaltyNotif` / `Loyalty`
- `ControlKindBattleWon`, `ControlKindBattleWithdrew`, `ControlKindUprising`, `ControlKindBattleReady`
- `BattlePending`, `LoyaltyCausedCurrentControlKind`

### GNPRTB Parameters

UPRIS1TB.DAT (3 entries) and UPRIS2TB.DAT (4 entries) map an uprising-incident score to outcome codes 0-5, read by `FUN_00559ce0` (see `ghidra/notes/uprising-incident.md`). FUN_005121e0 and FUN_00511f40 fire uprising notifications (events 0x14b and 0x151) and do not read them.

### What to Change for a Total Conversion

- **New victory condition** (e.g., control 80% of systems instead of capturing HQ): patch the `VictoryConditions` handler. The `SideVictoryConditionsNotif` string is the entry point.
- **Multiple factions** (Thrawn era — Empire, New Republic, Remnant, Hutts): the game uses a 2-faction architecture hardcoded throughout. Adding factions requires: new faction IDs in BASICSD.DAT, patching all same/opposite-faction checks, new per-faction UI elements. Major effort.
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
| FUN_00504a00 | 231 | **Troop destruction during blockade** — `TroopRegDestroyedRunningBlockade` (event 0x340) | — |
| FUN_00504470 | 118 | **Troop withdrawal percent setter** — `TroopReg.cpp: Invalid WithdrawPercent value!` error | — |
| FUN_00504af0 | 56 | **Troop withdrawal percent notif** — `TroopRegWithdrawPercentNotif` | — |
| FUN_00511e70 | 73 | **TroopRegWithdrawPercent system notif** | `SystemTroopRegWithdrawPercentNotif` |
| FUN_004ee470 | 129 | **Han Solo hyperdrive modifier** — `MissionHyperdriveModifier` notif; stored at `+0x9a` (short) | — |

### Capital-Ship Check

| Address | Size | Purpose |
|---------|------|---------|
| FUN_00502020 | 897 | **Capital-ship consistency check** — every carried object (characters and special forces `0x30`-`0x3f`, fighters `0x1c`-`0x1f`, troops `0x10`-`0x13`) shares the ship's side bits; hull `+0x60` lies in `[0, max]` (slot `+0x248`); carried fighters fit slot `+0x26c` and troops slot `+0x270`. |
| FUN_00555540 | — | **Containing system** of an object (transit start, §2) |

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
| FUN_0050e820 | 1658 | **System energy/material allocator** — `Invalid SystemEnergyAllocated value!`, `Invalid SystemRawMaterialAllocated value!`. Largest economy function. | — |
| FUN_00509cc0 | 117 | **SystemEnergy validator** | `SystemSystemEnergyNotif` / `SystemEnergy` |
| FUN_00509d40 | 117 | **SystemEnergyAllocated validator** | `SystemSystemEnergyAllocatedNotif` |
| FUN_00509dc0 | 117 | **SystemRawMaterial validator** | `SystemSystemRawMaterialNotif` / `SystemRawMaterial` |
| FUN_00509e40 | 117 | **SystemRawMaterialAllocated validator** | `SystemSystemRawMaterialAllocatedNotif` |
| FUN_00509f40 | 117 | **ProductionModifier validator** (system-level) | `SystemProductionModifierNotif` |
| FUN_0055a7a0 | 117 | **ProductionModifier validator** (facility-level) | `ProcFacilProductionModifierNotif` |
| FUN_00511c90-00511dd0 | 73 each | **Economy notification dispatchers** (5 functions) — `SystemEnergy`, `SystemEnergyAllocated`, `SystemRawMaterial`, `SystemRawMaterialAllocated`, `ProductionModifier` | — |
| FUN_0055b200 | 56 | **ProcFacilProductionModifier notif** | `ProcFacilProductionModifierNotif` |
| FUN_00532e00 | — | **ConstructionYardRdOrder** — research order from construction yard (event 0x127) | — |

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

*Luke/Vader saga, Jabba's Palace, Bounty Hunters, Dagobah training — hardcoded story event chains.*

### Luke/Vader Saga

| Address | Size | Purpose | Notif | Event ID |
|---------|------|---------|-------|----------|
| FUN_0054b770 | 51 | Luke Dagobah prerequisite | `MissionMgrLukeDagobahRequiredNotif` | — |
| FUN_0054b7b0 | 65 | Luke Dagobah mission active | `MissionMgrLukeDagobahNotif` / `LukeDagobah` | 0x221 (545) |
| FUN_00575320 | 56 | First Dagobah training day | `DagobahMissionFirstTrainingDayNotif` / `FirstTrainingDay` | — |
| FUN_0056fc70 | — | Dagobah training completed | `LukeDagobahCompletedNotif` / `DagobahCompleted` | 0x210 (528) |
| FUN_00571590 | 56 | Jedi training teacher key | `MissionJediTrainingTeacherKeyNotif` / `Teacher` | — |
| FUN_0054b900 | 51 | Darth Vader pickup | `MissionMgrDarthPickupNotif` / `DarthPickup` | — |
| FUN_0054b940 | 51 | Darth to Luke final battle | `MissionMgrDarthToLukeFinalBattleNotif` / `DarthToLukeFinalBattle` | — |
| FUN_0054b980 | 51 | Darth to Emperor final battle | `MissionMgrDarthToEmperorFinalBattleNotif` / `DarthToEmperorFinalBattle` | — |
| FUN_0054b9c0 | 51 | Final battle ready | `MissionMgrFinalBattleReadyNotif` / `FinalBattleReady` | — |
| FUN_0054ba00 | 65 | Final battle trigger | `MissionMgrFinalBattleNotif` / `FinalBattle` | — |
| FUN_0054ba50 | 51 | Luke final battle ready | `LukeFinalBattleReadyNotif` | — |

**Event ID ordering**: DagobahCompleted (0x210 = 528) < BountyAttack (0x212 = 530) < LukeDagobah (0x221 = 545). Completion events have lower IDs than requirement events — the chain is sequenced by ID.

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
| FUN_0054ba50 | 51 | Bounty hunters activated | `MissionMgrBountyHuntersActiveNotif` | — |
| FUN_00572b40 | — | Han bounty attack | `HanBountyAttackNotif` / `BountyAttack` | 0x212 (530) |

### What to Change for a Total Conversion

**Replacing the OT saga with Thrawn Trilogy / NJO / KOTOR events:**

The story events are implemented as notification handlers in the `MissionMgr*Notif` family. Each is a small function (51-65 bytes) that fires a specific event ID. In open-rebellion, these map to `EventSystem` (`rebellion-core/src/events.rs`) entries with `EventCondition::TickReached` or `CharacterAtSystem`.

- **Replace Dagobah with Wayland** (Thrawn mod): replace the `LukeDagobah*` event chain with new events triggered by a character reaching Wayland (system DatId lookup).
- **New capture/rescue events**: follow the Palace pattern — each event is a `MissionMgr*Notif` handler triggering event ID registration. Create new event IDs starting above 0x370 (880, the current maximum).
- **Remove OT events entirely**: set `enabled: false` on the corresponding `GameEvent` entries, or stub out the `MissionMgr*Notif` handlers.
- **New final battle**: `FinalBattle` (event `0x220`, story bit 13) and `FinalBattleReady` (bit 12) are the hooks. The `DEATHSTAR_FIRE` sound event (`0x006ab854`) and `EMPIRE_START_TURN`/`EMPIRE_END_TURN` strings confirm there are turn-structured events beyond the mission system.

---

## 10. UI & Rendering

*Strategy view, tactical view, encyclopedia, galaxy map. Note: STRATEGY.DLL and TACTICAL.DLL are resource-only — all UI code lives in REBEXE.EXE.*

### Strategy View

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0049eea0 | — | **Battle overlay window content** — label text from TEXTSTRA `0x1a00` or the literal "Battle in Progress..." by the text-edition flag (`FUN_004067d0`); colour COLORREF `0xff` (red, Alliance) or `0xff00` (green, Empire) by the owner's `+0x9c`. |
| FUN_0040a700 | 1246 | **Battle state machine** — 18-state switch for strategy view battle flow |
| FUN_00421c70 | 1490 | **DLL loader** — loads `strategy.dll`, `textcomm.dll` |
| FUN_00414a80 | 1033 | **Main window handler** — loads `textcomm.dll`, `common.dll`; handles `REBAPP_UI_PHASE_COCKPIT_SCREEN` |

**Ship name strings function**: FUN_00597610 (9220 bytes) references all ship type strings: `CALAMARI`, `BULK_CRUISER`, `ASSAULT_FRIGATE`, `NEBULON_B`, `ESCORT_A`, `CORVETTE`, `MEDIUM_TRANSPORT`, `BULK_TRANSPORT`, `GUNBOAT`, `DREADNAUGHT_A`, plus 40 more. This is likely the fleet display or tactical setup function.

### Tactical View

| Address | Size | Purpose |
|---------|------|---------|
| FUN_005a7500 | 4856 | **Tactical ship object constructor** — registers all tactical battle message handlers. Full handler list: `SHIP_ADD/REMOVE/ABSTRACT_DESTROY`, `cmSHIP_POST_DS_STATUS`, `NAV_*`, `SHIPGROUP_MOVE/NEWMISSION/NEWFORMATION/NEWTARGETLIST`, all `SHIP_FIRE*` and `SHIP_TAKE_*_HIT` events, `SHIP_SHIELD_HIT/FIX`, `SHIP_WEAPON_HIT/DOWN/FIX`, `SHIP_ION_DAMAGE`. |
| FUN_0059a8d0 | 393 | **Tactical view loader** — loads `texttact.dll`, `tactical.dll`, `VoiceFxA.dll`, `VoiceFxE.dll` |
| FUN_005d9eb0 | 321 | **D3DRM camera setup** — `Failed to create the camera's frame`, viewport creation |
| FUN_005da080 | 160 | **D3DRM render loop** — `Clearing viewport failed`, `Rendering scene failed` |

**Death Star tactical event**: `cmSHIP_POST_DS_STATUS` — Death Star status update for the tactical view.

**Tactical message strings** (from rebellion-strings.txt):
- `SHIP_FIRELASERCANNON`, `SHIP_FIRETURBOLASER`, `SHIP_FIREIONCANNON`, `SHIP_FIRETORPEDO`
- `SHIP_TAKE_LASER_HIT`, `SHIP_TAKE_TURBO_HIT`, `SHIP_TAKE_TORPEDO_HIT`, `SHIP_ION_DAMAGE`
- `SHIP_SHIELD_HIT`, `SHIP_SHIELD_FIX`, `SHIP_WEAPON_HIT`, `SHIP_WEAPON_DOWN`, `SHIP_WEAPON_FIX`
- `TASKFORCE_NEW`, `DEATHSTAR_FIRE`, `EMPIRE_START_TURN`, `EMPIRE_END_TURN`

### Encyclopedia

| Address | Size | Purpose |
|---------|------|---------|
| FUN_0045d400 | 893 | **Encyclopedia DLL loader** — loads `encytext.dll`, `encybmap.dll` |
| FUN_00466350 | 351 | **Encyclopedia bitmap loader** — loads `encybmap.dll` |
| FUN_0052d920 | 2894 | **Unit description/display** — references TEXTSTRA string IDs. Second-largest function after the ship-name function. Likely renders entity descriptions for the encyclopedia panel. |

In open-rebellion, the encyclopedia is implemented in `crates/rebellion-render/src/encyclopedia.rs` with 4 tabs and BMP texture cache from `data/base/EData/`. HD-upscaled PNGs go in `data/hd/EData/`.

### Multiplayer / Network

| Address | Size | Purpose |
|---------|------|---------|
| FUN_005f9860 | 1601 | **CommMgr::ReceiveIncomingCommPacket** — DirectPlay receive handler; `CM_PLAYER_LEFT_UNEXCUSED`, `DPSYS_DELETEPLAYER/ADDPLAYER` |
| FUN_005f7ea0 | 1361 | **CommMgr::Flush** — DirectPlay send; error handling |
| FUN_005f8600 | 316 | **CommMgr::CleanUp** — session teardown |
| FUN_005fa400 | 307 | **CommMgr::PingOpponents** — latency check |
| FUN_0040ec60 | 824 | **MultiPlayerWindow** — multiplayer UI, cancel/quit session |

**Network architecture**: DirectPlay (DirectX 6 era). Not relevant for open-rebellion (uses Rust-native networking or local-only).

### What to Change for a Total Conversion

- **New galaxy map layout**: in open-rebellion, the galaxy map is `crates/rebellion-render/src/lib.rs` (`draw_galaxy_map`). No REBEXE.EXE work needed — the map is fully re-implemented.
- **New tactical view elements**: the 2D tactical view (FUN_005a7500) registers message handlers. New weapon types need new `SHIP_FIRE*` message strings and corresponding handler registration. The Direct3D Retained Mode (D3DRM) renderer at FUN_005d9eb0 is Win32-specific and irrelevant for open-rebellion.
- **New ship names in encyclopedia**: replace FUN_00597610's string set, or — in open-rebellion — edit `data/encyclopedia.json` entries directly.
- **Custom UI panels**: open-rebellion's egui panels (`crates/rebellion-render/src/panels/`) are fully modifiable. No REBEXE.EXE dependency.

---

## Appendix A: Entity Family Byte Reference

The family byte is `id >> 24`. Ranges are the DAT header ranges (`record-registry.md`); the right column lists the families the shipped records and the factory registry use.

| Range | Type | Families in use |
|-------|------|-----------------|
| `0x08`-`0x0f` | Fleets | executable-made; FLEETSD records |
| `0x10`-`0x13` | Troop regiments (TROOPSD) | `0x10` |
| `0x14`-`0x1b` | Capital ships (CAPSHPSD) | `0x14`; `0x18` the Death Star (record 136) |
| `0x1c`-`0x1f` | Fighter squadrons (FIGHTSD) | `0x1c` |
| `0x20`-`0x21` | Alliance HQ (ALLFACSD) | `0x20` |
| `0x22`-`0x27` | Defence facilities (DEFFACSD) | `0x22` KDY-150, `0x23` LNR, `0x24` shield generator, `0x25` Death Star shield |
| `0x28`-`0x2b` | Manufacturing facilities (MANFACSD) | `0x28` shipyard, `0x29` training facility, `0x2a` construction yard |
| `0x2c`-`0x2f` | Production facilities (PROFACSD) | `0x2c` mine, `0x2d` refinery |
| `0x30`-`0x37` | Major characters (MJCHARSD) | one family each, `0x30`-`0x35`: `0x31` Leia, `0x32` Luke, `0x33` Han, `0x34` Palpatine, `0x35` Vader |
| `0x38`-`0x3b` | Minor characters (MNCHARSD) | `0x38` (Chewbacca `0x38000343`) |
| `0x3c`-`0x3f` | Special forces (SPECFCSD) | `0x3c` |
| `0x40`-`0x7f` | Missions (MISSNSD) | family = mission kind, 22 registered: `0x41` Move, `0x64` Palace, `0x71` Dagobah, `0x73` Pickup, … |
| `0x80`-`0x8f` | Sectors (SECTORSD) | `0x80` |
| `0x90`-`0x97` | Star systems (SYSTEMSD) | `0x90`, `0x92` |
| `0x98` | Places (ABODESD) | `0x98000481` Jabba's Palace, `0x98000441` Dagobah, `0x98000440`/`0x98000480` the jails |

## Appendix B: Event ID Registry

| ID (hex) | ID (dec) | Event | Source |
|---------|---------|-------|--------|
| 0x127 | 295 | ConstructionYardRdOrder | FUN_00532e00 |
| 0x128 | 296 | VictoryConditions | FUN_00532f90 |
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

## Appendix C: Story-Flag Word (MissionMgr `+0x58`)

Each bit has its own setter (`FUN_00543af0` bit 0, `FUN_00543b60` bit 2, `FUN_00543bd0`-`FUN_00543fc0` bits 5-14); a change runs both side views' broadcaster (`FUN_0054a140`-`FUN_0054a770`) and the manager's handler slot (`+0x1c0`-`+0x1ec`). Conditions and handlers: `story-missions.md`.

| Bit | Mask | Name | Set by |
|-----|------|------|--------|
| 0 | 0x0001 | LukeDagobahRequired | timer `0x392` |
| 2 | 0x0004 | LukeDagobah (event `0x221`) | FUN_00544030 |
| 5 | 0x0020 | LukePalace | FUN_00544130 |
| 6 | 0x0040 | HanCapturedAtPalace | FUN_005442f0 |
| 7 | 0x0080 | LeiaPalace | FUN_005443f0 |
| 8 | 0x0100 | ChewbaccaPalace | FUN_005444e0 |
| 9 | 0x0200 | DarthPickup | FUN_005445d0 |
| 10 | 0x0400 | DarthToLukeFinalBattle | FUN_00544a20 |
| 11 | 0x0800 | DarthToEmperorFinalBattle | FUN_00544be0 |
| 12 | 0x1000 | FinalBattleReady | FUN_00544da0 |
| 13 | 0x2000 | FinalBattle (event `0x220`) | FUN_00544fc0 |
| 14 | 0x4000 | BountyHuntersActive | FUN_00549d60 at the new game |

---

## Appendix D: Notification Architecture

All 100+ notification functions follow one of two patterns.

**Pattern A — Validate + Set + Notify** (setter functions, e.g., FUN_00501490):
```
FUN_0053a000 (entity validity)
→ vtable dispatch (get per-entity max value)
→ FUN_0053fc90 (range check: [min, max])
→ write new value to entity field
→ FUN_00539fd0 × 2 (get side 1 and side 2 observers)
→ FUN_005023b0 / FUN_004f04f0 (dispatch typed notification)
→ vtable dispatch (fire change event with old_value, new_value, context)
```

**Pattern B — Notify Only** (event dispatchers, e.g., FUN_004f1e00):
```
FUN_0053a010 (validity — notification path)
→ FUN_004f8980 / FUN_004f8880 / FUN_004f8aa0 (dispatch notification)
→ FUN_0053fcf0 / FUN_0053fdd0 / FUN_0053fe40 (register event with numeric ID)
```

The three notification dispatchers differ by parameter count:
- `FUN_004f8880(this, notif_str, event_str, param1, null, ctx)` — 1 entity param
- `FUN_004f8980(this, notif_str, event_str, param2, param1, null, ctx)` — 2 entity params, reversed order
- `FUN_004f8aa0(this, notif_str, event_str, ref2, ref1, null, ctx)` — 2 entity reference params

---

## Appendix E: Cross-Reference Index

### By System → Functions

| System | Key Functions |
|--------|---------------|
| Hull damage | FUN_00501490, FUN_005029a0, FUN_00501ae0, FUN_00501b70, FUN_0040cfc0 |
| Shield recharge | FUN_00501510, FUN_005029f0 |
| Weapon recharge | FUN_005015a0, FUN_00502a40 |
| Fighter squadrons | FUN_005032c0, FUN_005038e0 |
| Character CommandKind | FUN_00560d50, FUN_004ee350, FUN_004f1f80 |
| Emperor at Coruscant | FUN_005617b0, FUN_0055f650 |
| Bombardment | FUN_00582440, FUN_0058d800, FUN_0058d770, FUN_0058d990, FUN_0058dc10, FUN_0058df60, FUN_0058e320 |
| Transit time | FUN_00514a60, FUN_00556430, FUN_00555b30, FUN_0055d8c0, FUN_0055d860 |
| Space combat | FUN_0040a700, FUN_005a7500, FUN_005ae9c0 |
| Story-flag checks | FUN_005457f0, FUN_005438a0, FUN_00549910, FUN_005442f0-FUN_00544fc0 |
| Loyalty | FUN_005341a0, FUN_004ee030 |
| Force/Jedi | FUN_0058a3f0, FUN_004f1e00, FUN_004f1ea0, FUN_004f20e0 |
| Mission outcomes | FUN_004fc080, FUN_004fc3a0 |
| Mission scripted events | FUN_0054b770–FUN_0054ba50, FUN_00572b40 |
| Fleet movement | FUN_004ff7a0–FUN_004ff890, FUN_00504a00 |
| Economy | FUN_0050e820, FUN_00509cc0–FUN_00509f40, FUN_00511c90–FUN_00511dd0 |
| System control | FUN_00511ec0, FUN_00511f40, FUN_005121e0–FUN_005126c0 |
| Victory | FUN_005180e0 (`victory-conditions.md`) |
| UI — Strategy | FUN_0049eea0, FUN_0040a700 |
| UI — Tactical | FUN_005a7500, FUN_005d9eb0 |
| UI — Encyclopedia | FUN_0045d400, FUN_0052d920 |
| GNPRTB loading | FUN_00569280 |

### By DAT File → Functions Consuming It

| DAT File | Used By |
|----------|---------|
| CAPSHPSD.DAT | FUN_00501490 (hull max via vtable) |
| GNPRTB.DAT | FUN_0055d8c0 (transit divisor GNPRTB `0x1400` via DAT_006bb6e8); bombardment `FUN_0058dc10`, `FUN_0058df60`, `FUN_0058e320` (GNPRTB `0x600`-`0x606`, 7705) |
| `*MSTB.DAT` files | Mission probability lookup (`FUN_0053e240`; per-table consumers in `uprising-incident.md`) |
| UPRIS1TB.DAT / UPRIS2TB.DAT | FUN_00559ce0 (uprising incident outcome codes, called by FUN_0050d030) |
| MISSNSD.DAT | FUN_004fc080 (mission type switch) |
| MJCHARSD / MNCHARSD | FUN_005341a0 (loyalty), FUN_004ee030 (enhanced loyalty), FUN_004ee470 (hyperdrive), FUN_0058a3f0 (Force) |
