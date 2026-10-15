---
title: "REBEXE.EXE — Entity System Reference"
description: "Entity system architecture from Ghidra decompilation with cross-references"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# REBEXE.EXE — Entity System Reference

Generated from Ghidra decompilation of REBEXE.EXE (Star Wars Rebellion 1998, LucasArts).
Cross-referenced with: cpp-class-hierarchy.md, annotated-functions.md, modders-taxonomy.md, combat-formulas.md.

The entity system is built on a single **Notification/Observer pattern** used uniformly across every
mutable field of every game object. Every setter in the codebase follows an identical chain:
validate → range-check → write → bilateral observer notify → vtable change-event. No setter bypasses
this chain. The same design holds for pure notification dispatchers (no write step): they guard with
`FUN_0053a010` instead of `FUN_0053a000` and skip the range-check.

---

## 1. Character System

All character state setters and notification dispatchers in the `0x4ee030–0x4f2300` range.

### 1.1 Setter Chain (write-path functions)

These functions validate, write the field, dispatch bilateral observer notifications, and fire a
vtable change-event. They use `FUN_0053a000` (strict validity guard) followed by `FUN_0053fc90`
(range check) before touching any field.

#### EnhancedLoyalty — `FUN_004ee030`

| Property | Value |
|----------|-------|
| Address | `0x004ee030` |
| Size | 87 bytes |
| Notif string | `CharacterEnhancedLoyaltyNotif` / `EnhancedLoyalty` |
| Field | `+0x8a` (short, 0–100, also clamped to 0x7fff) |
| Vtable event | `+0x318` `NotifyEnhancedLoyaltyChanged(old, new, ctx)` |
| Observer dispatch | `FUN_004f0190` (bilateral, sides 1 and 2) |
| Validity guard | `FUN_0053a000` |
| Range check | `FUN_0053fc90(value, 0, 100)` then `FUN_0053e0f0(value, 0, 0x7fff)` |

This is the **mission-bonus loyalty** field, distinct from the base loyalty at `+0x66`. The
secondary `0x7fff` clamp guards against short overflow. Vtable slot `+0x318` differs from base
loyalty's `+0x238` — they are separate event channels.

```c
int set_enhanced_loyalty(void* character, int new_loyalty, void* context) {
    if (!FUN_0053a000(character)) return 0;
    int valid = FUN_0053fc90(new_loyalty, 0, 100);
    FUN_0053e0f0(new_loyalty, 0, 0x7fff);
    if (valid && *(short*)(character+0x8a) != new_loyalty) {
        short old = *(short*)(character+0x8a);
        *(short*)(character+0x8a) = (short)new_loyalty;
        FUN_004f0190(FUN_00539fd0(character, 1), context);
        FUN_004f0190(FUN_00539fd0(character, 2), context);
        (**(code**)(*(int*)character + 0x318))(old, *(short*)(character+0x8a), context);
    }
    return valid;
}
```

#### CommandKind — `FUN_004ee350`

| Property | Value |
|----------|-------|
| Address | `0x004ee350` |
| Size | ~150 bytes |
| Notif string | `CharacterCommandKindNotif` (error `Invalid CommandKind value`) |
| Field | `+0x96` (short): 0 none, 1 Commander, 2 Admiral, 3 General |
| Vtable event | `+0x330` `NotifyCommandKindChanged(old, new, ctx)` |
| Observer dispatch | `FUN_004f04f0` (bilateral) |
| Validity guard | `FUN_0053a000` |
| Range check | `FUN_0053fc90(value, 0, value)` |

The character's officer rank. The command orders `0x260`-`0x263` write it; a move onto or
off a ship clears it (`FUN_00560d50`), and so does injury, capture or loss (`FUN_004eeed0`). The range check passes any value, so the
gate is `FUN_004ed750` (`character-roster.md`).

#### HyperdriveModifier — `FUN_004ee470`

| Property | Value |
|----------|-------|
| Address | `0x004ee470` |
| Size | 129 bytes |
| Notif string | `MissionHyperdriveModifierNotif` / `MissionHyperdriveModifier` |
| Field | `+0x9a` (short, non-negative, no upper bound) |
| Vtable event | `+0x338` `NotifyHyperdriveChanged(old, new, ctx)` |
| Observer dispatch | `FUN_004f0610` (bilateral) |
| Validity guard | `FUN_0053a000` |
| Range check | `FUN_0053fc90(value, 0, value)` — non-negative only |

The Han Solo speed bonus. Applied when calculating `ticks_per_hop` in the movement system. Short
storage at `+0x9a` gives a max of 32767 — effectively unbounded for gameplay purposes. The
`MissionHyperdriveModifierNotif` notification string is the game's canonical name for this effect.

**Cross-reference (`rebellion-core/src/movement.rs`)**: When building `ticks_per_hop`, check
`character.hyperdrive_modifier` (field `+0x9a`) for any character assigned as admiral. Subtract
from the base hop duration.

#### BaseLoyalty — `FUN_005341a0`

| Property | Value |
|----------|-------|
| Address | `0x005341a0` |
| Notif string | `SystemLoyaltyNotif` / `Loyalty` |
| Field | `+0x66` (short, 0–100) |
| Vtable event | `+0x238` `NotifyLoyaltyChanged(old, new, ctx)` |
| Observer dispatch | `FUN_00535380` (bilateral) |
| Validity guard | `FUN_0053a000` |
| Range check | `FUN_0053fc90(value, 0, 100)` then `FUN_0053e0f0(value, 0, 0x7fff)` |

Base loyalty drives diplomacy missions, betrayal mechanics, and system control shifts. Two
distinct loyalty fields exist:

```
+0x66  base_loyalty       — FUN_005341a0, vtable +0x238
+0x8a  enhanced_loyalty   — FUN_004ee030, vtable +0x318
```

**Betrayal mechanic**: The `not betray` string at `0x006ab3dc` confirms a betrayal check function.
The `is_unable_to_betray` flag in MJCHARSD.DAT / MNCHARSD.DAT is the DAT-layer gate for major
characters (Luke, Vader cannot switch sides). `FUN_005341a0` is the code-layer entry point for
loyalty changes that could trigger betrayal — any loyalty drop below a threshold (presumably
from the UPRIS1TB.DAT / UPRIS2TB.DAT tables) invokes the betrayal check path.

---

### 1.2 Enhanced Skill Notification Dispatchers (notification-only, no write step)

These functions use `FUN_0053a010` (notification-path validity guard) and dispatch via
`FUN_004f8980` without writing a field directly. They are called by the mission outcome system
after the outcome has already been committed. Each fires a named notification so observers can
react to the skill boost.

All of these follow the identical pattern:
```c
int skill_notif(void* character, undefined4 new_val, undefined4 old_val, int ctx) {
    if (!FUN_0053a010(character)) return 0;
    FUN_004f8980(character, "Character*Notif", "SkillName", old_val, new_val, null, ctx);
    return valid;
}
```

| Address | Notif String | Event Name | Data Address |
|---------|-------------|------------|--------------|
| `FUN_004f1c00` | `CharacterEnhancedDiplomacyNotif` | `EnhancedDiplomacy` | `0x006a88e0` |
| `FUN_004f1c40` | `CharacterEnhancedEspionageNotif` | `EnhancedEspionage` | `0x006a8918` |
| `FUN_004f1d40` | `CharacterEnhancedCombatNotif` | `EnhancedCombat` | `0x006a8a10` |
| `FUN_004ee030` | `CharacterEnhancedLoyaltyNotif` | `EnhancedLoyalty` | (field writer — §1.1) |
| `FUN_004f2000` | `MissionHyperdriveModifierNotif` | `MissionHyperdriveModifier` | `0x006a8ba8` |

**Note**: EnhancedShipDesign, EnhancedTroopTraining, EnhancedFacilityDesign, and
EnhancedLeadership notif dispatchers exist in the binary (confirmed by the `BaseShipyardRD`,
`BaseTrainingFacilRD`, `BaseConstructionYardRD`, `BaseLeadership` notif strings in the `0x00536xxx`
range) but have not been individually decompiled. They follow the same `FUN_004f8980` pattern.

---

### 1.3 Force / Jedi System

All Force-related dispatchers use `FUN_0053a010` (notification path). None of them write fields
directly — the actual Force value modification happens upstream (in the Jedi training mission
manager). The dispatchers are the observer-broadcast step.

| Address | Size | Notif String | Event Name | Event ID |
|---------|------|-------------|------------|----------|
| `FUN_004f1e00` | 73 | `CharacterForceNotif` | `Force` | `0x1e1` (481) |
| `FUN_004f1e50` | 73 | `CharacterForceExperienceNotif` | `ForceExperience` | — |
| `FUN_004f1ea0` | 73 | `CharacterForceTrainingNotif` | `ForceTraining` | `0x1e5` (485) |
| `FUN_004f20e0` | 73 | `CharacterForceUserDiscoveredKeyNotif` | `ForceUserDiscovered` | `0x362` (866) |
| `FUN_004f2240` | 51 | `CharacterForceAwareNotif` | `ForceAware` | — |
| `FUN_004f2280` | 51 | `CharacterForcePotentialNotif` | `ForcePotential` | — |
| `FUN_004f2300` | 51 | `CharacterDiscoveringForceUserNotif` | (see binary) | — |

**ForceUserDiscovered** (`FUN_004f20e0`) is special: it dispatches via `FUN_004f8aa0` (two
entity-ref params) and registers event `0x362`. This is the entry point for the "Jedi discovered
by the other side" story moment.

**Force fields (CCharacter)**, from `force-users.md`:

| Expression | Type | Field | Notes |
|------------|------|-------|-------|
| `+0x8c` (`entity[0x23]`) | `short` | Force | `+0x8e + +0x90` (`FUN_004eee40`), clamped 0..0x7fff |
| `+0x8e` | `short` | base Force | DAT `jedi_level_base + rand(variance)` once bits 4 and 5 are set (`FUN_004eee60`) |
| `+0x90` | `short` | training bonus | Jedi Training gains (`FUN_004ee1d0`) |
| `+0xac` bit 5 | bit | sensitive | rolled at creation against DAT `jedi_probability` |
| `+0xac` bit 4 | bit | aware | DAT `is_known_jedi`, or discovery |
| `+0xac` bit 7 | bit | discoverer | DAT `is_jedi_trainer` and Force >= GNPRTB 2580 |
| `entity[9] >> 6 & 3` | 2-bit | owner side | 1 Alliance, 2 Empire, 3 Neutral (`side-numbering.md`) |
| `entity[0x1e] & 1` | bit | decoy | mission decoy flag `+0x78` bit 0 (`decoy-roll.md`) |
| `context+0x4c` | `uint` | detected | set to 1 on a detection hit |

**Force detection** (`FUN_0058a3f0`, 306 bytes): `FUN_0055e4d0(owner, a, b,
&hit, &chance)` ignores its first argument; when both characters' Force is
non-zero, `chance = Force(a) + Force(b) + GNPRTB 3587` (-100) percent. On a
hit the manager's `+0x4c` is set unless the first character is a decoy, and
`FUN_0055ff60` pairs and reveals both when the root's `+0xc0` object exists.

---

### 1.4 Mission Role Assignment

These dispatchers manage whether a character is currently on a mission and what their mission
status is. All use `FUN_0053a010` and `FUN_004f8880` (single entity-param pattern).

| Address | Notif String | Event Name | Notes |
|---------|-------------|------------|-------|
| `FUN_00536b00` | `RoleOnMissionNotif` | `OnMission` | Character has been assigned to a mission |
| `FUN_00536b40` | `OnHiddenMissionNotif` | `OnHiddenMission` | Mission is concealed from opponent |
| `FUN_00536b80` | `RoleOnMandatoryMissionNotif` | `OnMandatoryMission` | Story-driven forced assignment |
| `FUN_00536a40` | `RoleCanResignFromMissionNotif` | `CanResignFromMission` | Character resignation eligibility |
| `FUN_00536a00` | `RoleMissionResignRequestNotif` | `MissionResignRequest` | Player requested resignation |
| `FUN_00536880` | `RoleMissionSeedKeyNotif` | `MissionSeed` | Starting mission (game initialization) |

`OnHiddenMission` marks missions invisible to the opposing side's intelligence. The `OnMandatory`
flag is set by story events and prevents player reassignment — Luke's training on Dagobah is
the canonical example.

**Base skill role notifications** (from the `0x005366xx` range — confirmed in binary):

| Address | Notif String | Event Name |
|---------|-------------|------------|
| `FUN_00536600` | `RoleBaseDiplomacyNotif` | `BaseDiplomacy` |
| `FUN_00536640` | `RoleBaseEspionageNotif` | `BaseEspionage` |
| `FUN_00536680` | `RoleBaseShipyardRDNotif` | `BaseShipyardRD` |
| `FUN_005366c0` | `RoleBaseTrainingFacilRDNotif` | `BaseTrainingFacilRD` |
| `FUN_00536700` | `RoleBaseConstructionYardRDNotif` | `BaseConstructionYardRD` |
| `FUN_00536740` | `RoleBaseCombatNotif` | `BaseCombat` |
| `FUN_00536780` | `RoleBaseLeadershipNotif` | `BaseLeadership` |
| `FUN_005367c0` | `RoleBaseLoyaltyNotif` | `BaseLoyalty` |

These broadcast the character's base skill assignment when a character accepts a role (admiral,
commander, general). The base skill values are read from MJCHARSD.DAT / MNCHARSD.DAT at load time.

---

## 2. Game Object Base

### 2.1 CRebObject → CNotifyObject Inheritance

All game entities inherit from this two-level base. `CRebObject` provides the vtable root and
entity identity. `CNotifyObject` adds observer plumbing and all mutable field infrastructure.

```
CRebObject (base)
  +0x00  void*  vtable
  Virtual: GetFamilyId(), GetSideKey(), IsActive(), CanBombard()

└── CNotifyObject
      +0x2c  void*  record             — the object's DAT record; a system's record +0x4c holds its x/y (FUN_00509620)
      +0x24  uint   owner and view     — bits 6-7 = owner (1 Alliance, 2 Empire, 3 Neutral); bits 4-5 = the copy
                                         (0 master, 1 Alliance view, 2 Empire view; side-numbering.md)
      +0x4c  uint   detection_pending  — Jedi detection flag
      +0x50  uint   status_flags       — bit0=active; on characters bit3=destroyed, bit12 must be clear to be free
      +0x58  uint   flag word          — per class: MissionMgr story flags bits 0-14 (story-missions.md),
                                         CharacterMgr bit 0 = Emperor at Coruscant (FUN_005617b0/FUN_0055f650),
                                         fleet bit 6 = in the bombard order (bombardment.md)
      +0x60  int    hull_current / squad_size_current  (polymorphic by type)
      +0x64  uint   shield_weapon_packed
      +0x66  short  loyalty_base        (CCharacter only)
      +0x78  byte   character mission flags — bit4=on a mission it may resign, bits 6/7=adrift / on a mission (story-missions.md)
      +0x8a  short  enhanced_loyalty    (CCharacter only)
      +0x8c  short  force_potential_raw (CCharacter only)
      +0x96  short  command_kind        (CCharacter: 0 none, 1 Commander, 2 Admiral, 3 General; setter FUN_004ee350, character-roster.md)
      +0x9a  short  hyperdrive_mod      (CCharacter only)
      +0xac  byte   prisoner            (CCharacter: bit0=prisoner; ai-mission-planning.md, victory-conditions.md)
      +0xb0  byte   story flags         (story characters: Luke, Han, Leia; story-missions.md)
```

### 2.2 Common Notification Pattern

Every field mutation follows this exact chain. The pattern is consistent across all 4,900
decompiled functions with no documented exceptions.

**Step 1**: `FUN_0053a000(this)` — true for the master copy (`+0x24 & 0x30` clear).
Used by setter functions that write fields.

**Step 2**: `FUN_0053fc90(value, min, max)` — range validate. Returns 0 if out of [min, max].

**Step 3**: `FUN_0053e0f0(value, min, 0x7fff)` — secondary short-storage clamp (loyalty fields
only). Belt-and-suspenders guard against short overflow.

**Step 4**: Read old value, write new value to field. Early-exit if `old == new`.

**Step 5**: `FUN_00539fd0(this, 1)` → dispatch to the Alliance's view copy.

**Step 6**: `FUN_00539fd0(this, 2)` → dispatch to the Empire's view copy.

**Step 7**: `(**(code**)(*this + vtable_slot))(old, new, context)` — fire vtable change-event.

**Notification-path variant** (`FUN_0053a010` instead of `FUN_0053a000`): Used by pure dispatchers
(no write step). It is true for a view copy (`+0x24 & 0x30` nonzero), the complement of
`FUN_0053a000`.

### 2.3 Object Lifecycle

**Creation**: Objects are heap-allocated via `FUN_00618b70` (wrapper over `new`). Constructors
set the vtable pointer and initialize all fields. The tactical battle manager (`FUN_005a7500`)
is the largest constructor observed — 4,856 bytes, allocates 0x3a8 words.

**Destruction variants** (from `FUN_004fc080`):

| Mission Type Code | Constant | Notif String | Event ID |
|-------------------|----------|-------------|----------|
| 0x15 (21) | `Autoscrap` | `GameObjDestroyedAutoscrapNotif` / `DestroyedAutoscrap` | `0x304` (772) |
| 6 | `Sabotage` | `GameObjDestroyedSabotageNotif` / `DestroyedSabotage` | `0x305` (773) |
| 7 | `Assassination` | `GameObjDestroyedAssassinationNotif` / `DestroyedAssassination` | `0x306` (774) |
| — | `OnArrival` | `GameObjDestroyedOnArrivalNotif` / `DestroyedOnArrival` | `0x303` (771) |
| — | `(generic)` | `GameObjDestroyedNotif` / `Destroyed` | `0x302` (770) |

The `OnArrival` variant is triggered when the object's `status_flags` bit5 is set and
`+0x40 & 0xff0000 != 0` — confirming the object was destroyed en route to a destination.

The mission type code is read from `+0x40 & 0xff` of the entity. The destroy function
(`FUN_004fc080`) fires the generic `Destroyed` event first, then conditionally fires the
specialized variant based on that byte. All destruction events use `FUN_0053fe40` to register
the event ID.

**Reference counting**: `FUN_004ece30` is the reference-counting helper called in destruction
paths — it manages object lifetime for objects referenced by multiple mission contexts.

### 2.4 Key Field Offsets (consolidated)

| Offset | Type | Field | Entity Type |
|--------|------|-------|-------------|
| `+0x00` | `void*` | vtable | all |
| `+0x24` | `uint` | owner (bits 6-7); copy (bits 4-5: 0 master, 1 Alliance view, 2 Empire view) | all |
| `+0x2c` | `void*` | DAT record (a system's record `+0x4c` = x/y, `FUN_00509620`) | all |
| `+0x40` | `uint` | mission_type_byte (bits 0-7), arrival_flags (bits 16-23) | CMission |
| `+0x4c` | `uint` | detection_pending | detection context |
| `+0x50` | `uint` | status_flags | all |
| `+0x58` | `uint` | flag word (story flags, seat of power, fleet orders) | MissionMgr, CharacterMgr, CFleet |
| `+0x60` | `int` | hull_current / squad_size_current | CCapitalShip / CFighterSquadron |
| `+0x64` | `uint` | shield_weapon_packed | CCapitalShip |
| `+0x66` | `short` | loyalty_base | CCharacter |
| `+0x78` | `byte` | mission flags (bit 4 may resign, bits 6/7 adrift / on a mission) | CCharacter |
| `+0x80` | `uint` | entity_type_id | set during init (0x51–0x6a range) |
| `+0x8a` | `short` | enhanced_loyalty | CCharacter |
| `+0x8c` | `short` | force_potential_raw | CCharacter |
| `+0x94` | `short` | gnprtb_char_field (GNPRTB character stat) | CCharacter |
| `+0x96` | `short` | command_kind (officer rank) | CCharacter |
| `+0x9a` | `short` | hyperdrive_mod | CCharacter |
| `+0xac` | `byte` | prisoner (bit 0) | CCharacter |
| `+0xb0` | `byte` | story flags | Luke, Han, Leia |

---

## 3. Fleet System

### 3.1 Fleet Notification Dispatchers

All four fleet event dispatchers follow the same pattern: `FUN_0053a010` validity, then
`FUN_004f8880` (single entity-param), then `FUN_0053fdd0` to register the event.

| Address | Size | Notif String | Event Name | Event ID |
|---------|------|-------------|------------|----------|
| `FUN_004ff7a0` | 65 | `FleetBattleNotif` | `Battle` | `0x180` (384) |
| `FUN_004ff7f0` | 65 | `FleetBlockadeNotif` | `Blockade` | `0x181` (385) |
| `FUN_004ff840` | 65 | `FleetBombardNotif` | `Bombard` | `0x182` (386) |
| `FUN_004ff890` | 65 | `FleetAssaultNotif` | `Assault` | `0x183` (387) |

These four events are the core fleet-state transition broadcast points. Each fires from the
fleet object (`this`) with two integer parameters (param_1 = target system or battle context,
param_2 = secondary context). The event IDs form a contiguous block `0x180–0x183`.

**Troop destruction during blockade** (`FUN_00504a00`, 231 bytes):
- Notif string: `TroopRegDestroyedRunningBlockade`
- Event ID: `0x340` (832)
- Uses `FUN_004f8aa0` (two entity-ref params: troop + transport)
- Triggered when troops are caught in a hostile blockade during transit

**Troop withdrawal** (`FUN_00504af0`, 56 bytes):
- Notif string: `TroopRegWithdrawPercentNotif`
- Related: `FUN_00511e70` — `SystemTroopRegWithdrawPercentNotif`

### 3.2 Fleet Composition

| Address | Size | Purpose |
|---------|------|---------|
| `FUN_00502020` | 897 | Capital-ship contents check: every child must be a character or special force (`[0x30,0x40)`), a fighter squadron (`[0x1c,0x20)`) or a regiment (`[0x10,0x14)`) of the ship's side; fighters are counted against slot `+0x26c`, regiments against `+0x270` (`ai-behavior-analysis.md`). |
| `FUN_00555540` | — | The system an object is in, the container lookup of the transit chain (`build-delivery.md`) |
| `FUN_004f2640` | — | Fleet unit iterator (used in FUN_004ff7a0 callers) — params: `side=1, type=2` |

**CFleet contains**:
- Capital ships (families `0x14`-`0x1b`: CAPSHPSD `0x14`, the Death Star `0x18`)
- Fighter squadrons (families `0x1c`-`0x1f`, FIGHTSD `0x1c`)
- Characters (families `0x30`-`0x35` major, `0x38` minor); the fleet itself is family `0x08`-`0x0f`
- Entry point: `FUN_004ff7a0` (FleetBattleNotif) is the canonical start of the battle trigger chain

### 3.3 Fleet–Battle Integration

`FUN_004ff7a0` fires event `0x180` (FleetBattle). Every space battle then runs the tactical
engine: played in the battle window, simulated from the Battle Alert, or headless
(`FUN_0040a050` state 8; `simulate-runs-tactical-engine.md`).

`FUN_004ff8e0` (84 bytes) is a fleet system initialization function — it registers 10 message
handlers (event IDs `0x200–0x223` range + `0x234`) via `FUN_00520690`. This is the fleet
manager setup, run once at game load. All return 0 on failure.

---

## 4. Faction System

### 4.1 Side Resolution — `FUN_004f8c60`

**Address**: `0x004f8c60`
**Size**: 208 bytes
**Purpose**: Faction notification handler — resolves faction name, sets difficulty, builds
notification payload, fires via the notification log system.

The faction bits live at `*(uint*)(this + 0x24)`:

```
Bits 6-7 (>> 6 & 3):  own_faction_id   — 1=Alliance, 2=Empire, 3=Neutral, else UNKNOWN
Bits 4-5 (>> 4 & 3):  display_faction  — 1=Alliance, 2=Empire, else UNKNOWN
```

Faction string selection:
```c
uint faction_bits = *(uint*)(this + 0x24) >> 6 & 3;
switch (faction_bits) {
    case 1: local_1c = "Alliance";  break;
    case 2: local_1c = "Empire";    break;
    case 3: local_1c = "Neutral";   break;
    default: local_1c = "UNKNOWN!"; break;
}
```

String data addresses:
| String | Address |
|--------|---------|
| `"Alliance"` | `0x006a8f04` |
| `"Empire"` | `0x006a8f10` |
| `"Neutral"` | `0x006a8f1c` |
| `"UNKNOWN!"` | `0x006a8f28` |

The function also reads `param_6 + 0x14` as an integer for difficulty-related secondary
processing, and calls `FUN_004f62d0(this)` → `FUN_00583c40`, which returns the list head at
`+4` of its argument. The notification payload is assembled via `FUN_00616110` (string append) and
`FUN_00617480` (integer insert) into a log buffer at `DAT_006b2a70`.

**Cross-reference**: Bits 4-5 of the `0x24` packed field number the copy: 0 the master, 1 the
Alliance's view, 2 the Empire's (`side-numbering.md`). `FUN_0054a1d0` passes them to
`FUN_004fd600`, which returns `+0xc` for 1 and `+0x10` for 2 and 0 otherwise. No difficulty is
read there.

### 4.2 Victory Conditions

| Address | Purpose | Notif String | Event ID |
|---------|---------|-------------|----------|
| `FUN_00532f40` | Recruitment done | `SideRecruitmentDoneNotif` / `RecruitmentDone` | `0x12c` (300) |
| `FUN_0054ba00` | Final battle trigger | `MissionMgrFinalBattleNotif` / `FinalBattle` | — |
| `FUN_0054b9c0` | Final battle ready | `FinalBattleReadyNotif` | — |
| `FUN_00532f90` | Victory flags changed | `SideVictoryConditionsNotif` / `VictoryConditions` | `0x128` (296) |

**RecruitmentDone** fires when a side has completed all available character recruitment. It uses
`FUN_0053fdd0(300, this, param_2, param_3)` — this is event `0x12c`, the canonical
"recruitment phase complete" trigger.

`FUN_00532f90` is a side view's slot `+0x260`: it posts `0x128` when the view's copy of the
win flags (`+0xbc`) changes. The flags themselves come from `FUN_005180e0`, which tests the four
leaders, Coruscant and the Alliance HQ (`victory-conditions.md`).

The strings `VICTORY` (`0x006ab5f8`) and `VICTORY_II` (`0x006ab5b0`) are tactical ship class
names that `FUN_00597610` registers (Victory- and Victory II-class hulls), not audio. The
battle result music is tracks 12..15 (`audio-dispatch.md`).

### 4.3 System Control and Uprising

| Address | Notif String | Event ID |
|---------|-------------|----------|
| `FUN_00511ec0` | `ControlKindBattleWonNotif` | — |
| `FUN_00511f40` | `SystemControlKindUprisingNotif` / `ControlKindUprising` | — |
| `FUN_005121e0` | `SystemUprisingNotif` / `Uprising` | — |
| `FUN_005122d0` | `SystemBlockadeNotif` / `Blockade` | `0x14e` (334) |
| `FUN_00512280` | `SystemBattleNotif` / `Battle` | `0x14d` (333) |
| `FUN_00512580` | `SystemUprisingIncidentNotif` / `UprisingIncident` | `0x152` (338) |

`LoyaltyCausedCurrentControlKind` notif string confirms loyalty thresholds directly trigger
faction control changes. Thresholds are stored in UPRIS1TB.DAT (3 entries) and UPRIS2TB.DAT
(4 entries).

---

## 5. Notification Dispatcher Reference

Three distinct notification dispatch helper functions serve different entity parameter counts:

### `FUN_004f8880` — Single Entity Parameter
```c
// Signature (approximate):
void notify_1(void* this,
              char* notif_type_str, char* event_name_str,
              int param1, char* null_arg, int context);
```
Used by: FleetBattleNotif, FleetBlockadeNotif, FleetBombardNotif, FleetAssaultNotif,
OnMissionNotif, OnMandatoryMissionNotif, CanResignFromMissionNotif, ForceAwareNotif,
ForcePotentialNotif.

### `FUN_004f8980` — Two Entity Parameters (reversed order)
```c
// Signature (approximate):
void notify_2(void* this,
              char* notif_type_str, char* event_name_str,
              int param2, int param1, char* null_arg, int context);
```
**Note**: param order is reversed — the notification string lists new value first, then old
value. Confirmed in FUN_004f1e00 and FUN_004ee030 decompilations.

Used by: Force notifs, enhanced skill notifs, BaseDiplomacy/Espionage/Combat notifs.

### `FUN_004f8aa0` — Two Entity Reference Parameters
```c
// Signature (approximate):
void notify_2ref(void* this,
                 char* notif_type_str, char* event_name_str,
                 uint** param2_ref, uint** param1_ref, char* null_arg, int context);
```
Used by: TroopRegDestroyedRunningBlockade, ForceUserDiscovered, Encounter, Commanding.

### `FUN_004f8b80` — Mission Seed Key Variant
Used exclusively by `FUN_00536880` (RoleMissionSeedKeyNotif). Likely a specialized variant for
the initial mission seeding path.

---

## 6. Event ID Registry (Entity Range)

All confirmed event IDs from `FUN_0053fcf0` / `FUN_0053fe40` / `FUN_0053fdd0` call sites.

| Event ID | Hex | Notification / Event String | Class / Source |
|----------|-----|-----------------------------|----------------|
| 300 | 0x12c | RecruitmentDone | FUN_00532f40 |
| 333 | 0x14d | SystemBattle | FUN_00512280 |
| 334 | 0x14e | SystemBlockade | FUN_005122d0 |
| 338 | 0x152 | UprisingIncident | FUN_00512580 |
| 384 | 0x180 | FleetBattle | FUN_004ff7a0 |
| 385 | 0x181 | FleetBlockade | FUN_004ff7f0 |
| 386 | 0x182 | FleetBombard | FUN_004ff840 |
| 387 | 0x183 | FleetAssault | FUN_004ff890 |
| 416 | 0x1a0 | FightSquadSquadSizeDamage | CFighterSquadron |
| 448 | 0x1c0 | CapShipHullValueDamage | CCapitalShip |
| 449 | 0x1c1 | ShieldRechargeRateCHAllocated | CCapitalShip |
| 450 | 0x1c2 | WeaponRechargeRateCHAllocated | CCapitalShip |
| 481 | 0x1e1 | CharacterForce | FUN_004f1e00 |
| 485 | 0x1e5 | CharacterForceTraining | FUN_004f1ea0 |
| 770 | 0x302 | GameObjDestroyed (generic) | FUN_004fc080 |
| 771 | 0x303 | GameObjDestroyedOnArrival | FUN_004fc080 |
| 772 | 0x304 | GameObjDestroyedAutoscrap | FUN_004fc080 |
| 773 | 0x305 | GameObjDestroyedSabotage | FUN_004fc080 |
| 774 | 0x306 | GameObjDestroyedAssassination | FUN_004fc080 |
| 832 | 0x340 | TroopRegDestroyedRunningBlockade | FUN_00504a00 |
| 866 | 0x362 | CharacterForceUserDiscovered | FUN_004f20e0 |

---

## 7. Cross-Reference: Vtable Slots

Character vtable slots for all confirmed setters in the entity range.

| Vtable Offset | Decimal | Event | Source Function |
|--------------|---------|-------|-----------------|
| `+0x238` | 568 | NotifyLoyaltyChanged(old, new, ctx) | FUN_005341a0 |
| `+0x318` | 792 | NotifyEnhancedLoyaltyChanged(old, new, ctx) | FUN_004ee030 |
| `+0x330` | 816 | NotifyCommandKindChanged(old, new, ctx) | FUN_004ee350 |
| `+0x338` | 824 | NotifyHyperdriveChanged(old, new, ctx) | FUN_004ee470 |

Ship and squadron vtable slots (CCapitalShip, CFighterSquadron):

| Vtable Offset | Decimal | Event | Source Function |
|--------------|---------|-------|-----------------|
| `+0x260` | 608 | NotifySquadChanged(old, new, ctx) | FUN_005032c0 |
| `+0x29c` | 668 | NotifyHullChanged(old, new, ctx) | FUN_00501490 |
| `+0x2a0` | 672 | NotifyShieldChanged(old, new, ctx) | FUN_00501510 |
| `+0x2a4` | 676 | NotifyWeaponChanged(old, new, ctx) | FUN_005015a0 |

---

## 8. Utility Functions

| Address | Signature | Purpose |
|---------|-----------|---------|
| `FUN_0053a000` | `bool(int this)` | Master copy test (`+0x24 & 0x30` clear) — write-path setters |
| `FUN_0053a010` | `bool(int this)` | View copy test (`+0x24 & 0x30` set) — dispatchers |
| `FUN_0053fc90` | `int(value, min, max)` | Range validator: 0 if outside [min, max] |
| `FUN_0053e0f0` | `int(value, min, 0x7fff)` | Secondary clamp for short storage |
| `FUN_00539fd0` | `void*(this, n)` | View copy `n` (`FUN_005844e0` on `+0x20`, n 0-2): 1 Alliance, 2 Empire |
| `FUN_0053fcf0` | `void(type, this, p1, p2, ctx)` | Register event (notification path A) |
| `FUN_0053fe40` | `int(type, this, p1, p2, ctx)` | Register event (notification path B — returns result) |
| `FUN_0053fdd0` | `void(type, this, p1, p2)` | Register event (fleet notification path) |
| `FUN_004fd340` | `int()` | Current game day: calls the callback at `DAT_006b2b08` (`FUN_004fcf00`, day `+0x10`; `timer-scheduler.md`) |
| `FUN_004fd600` | `int(ctx, side)` | Side view: `+0xc` for side 1, `+0x10` for side 2, else 0 |
| `FUN_004f62d0` | `undefined*(int this)` | Get faction object from entity |
| `FUN_00583c40` | `int(int obj)` | Returns the list head at `+4` |
| `FUN_00618b70` | `void*(size)` | Heap allocator (new) |
| `FUN_004ece30` | `void*(void**)` | Reference counting / object lifetime helper |

---

## 9. Implementation Notes for rebellion-core

### Character world struct (`crates/rebellion-core/src/world/mod.rs`)

The following fields are confirmed by RE and must be present in the `Character` struct for
full game simulation parity:

```rust
pub struct Character {
    // Already present in current world model:
    pub dat_id: DatId,
    pub name: String,
    pub skills: [SkillPair; 8],
    pub jedi_probability: u8,
    pub jedi_level: SkillPair,
    pub is_known_jedi: bool,
    pub is_jedi_trainer: bool,
    pub is_unable_to_betray: bool,
    pub can_be_admiral: bool,
    pub can_be_commander: bool,
    pub can_be_general: bool,
    pub loyalty: SkillPair,

    // Required for entity system parity — not yet in world model:
    pub enhanced_loyalty: i16,       // +0x8a, mission bonus, 0-100 clamped
    pub hyperdrive_modifier: i16,    // +0x9a, Han Solo bonus, non-negative
    pub force: i16,                  // +0x8c = base + training
    pub force_base: i16,             // +0x8e
    pub force_training: i16,         // +0x90
    pub force_flags: u8,             // +0xac bits 4 aware, 5 sensitive, 7 discoverer
    pub on_mission: bool,            // OnMission flag
    pub on_hidden_mission: bool,     // OnHiddenMission flag
    pub on_mandatory_mission: bool,  // OnMandatoryMission flag
    pub can_resign: bool,            // CanResignFromMission flag
}
```

### Movement system (`crates/rebellion-core/src/movement.rs`)

The `hyperdrive_modifier` field (`+0x9a`) must be consulted when computing `ticks_per_hop`.
Subtract from `BASE_TICKS_PER_HOP` for fleets containing a character (admiral) with a non-zero
modifier:

```rust
let han_bonus = fleet.admiral()
    .and_then(|c| world.characters.get(c))
    .map(|c| c.hyperdrive_modifier as u32)
    .unwrap_or(0);
let ticks = (BASE_TICKS_PER_HOP.saturating_sub(han_bonus))
    .max(MIN_TICKS_PER_HOP);
```

### Event IDs for `rebellion-core/src/events.rs`

The following event IDs should be added to the `EventCondition::EventFired` variant's known
set for scripted event chaining:

```rust
pub const EVT_RECRUITMENT_DONE: u32    = 0x12c; // 300
pub const EVT_SYSTEM_BATTLE: u32       = 0x14d; // 333
pub const EVT_SYSTEM_BLOCKADE: u32     = 0x14e; // 334
pub const EVT_UPRISING_INCIDENT: u32   = 0x152; // 338
pub const EVT_FLEET_BATTLE: u32        = 0x180; // 384
pub const EVT_FLEET_BLOCKADE: u32      = 0x181; // 385
pub const EVT_FLEET_BOMBARD: u32       = 0x182; // 386
pub const EVT_FLEET_ASSAULT: u32       = 0x183; // 387
pub const EVT_CHARACTER_FORCE: u32     = 0x1e1; // 481
pub const EVT_CHARACTER_FORCE_TRAIN: u32 = 0x1e5; // 485
pub const EVT_GAME_OBJ_DESTROYED: u32  = 0x302; // 770
pub const EVT_DESTROYED_ON_ARRIVAL: u32 = 0x303; // 771
pub const EVT_DESTROYED_AUTOSCRAP: u32 = 0x304; // 772
pub const EVT_DESTROYED_SABOTAGE: u32  = 0x305; // 773
pub const EVT_DESTROYED_ASSASSINATION: u32 = 0x306; // 774
pub const EVT_TROOP_BLOCKADE_KILL: u32 = 0x340; // 832
pub const EVT_FORCE_USER_DISCOVERED: u32 = 0x362; // 866
```
