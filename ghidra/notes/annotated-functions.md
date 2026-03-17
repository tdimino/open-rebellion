---
title: "REBEXE.EXE — Annotated Function Reference"
description: "Cross-referenced annotated function catalog from Ghidra decompilation"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# REBEXE.EXE — Annotated Function Reference

Generated from Ghidra decompilation of REBEXE.EXE (Star Wars Rebellion 1998, LucasArts).
Cross-referenced with: combat-formulas.md, space-combat.md, ground-combat.md, bombardment.md.

Entity family byte quick reference (from `id >> 0x18`):
- `0x08-0x0f` — Characters (major + minor)
- `0x14-0x1b` — Troops / Special Forces
- `0x30-0x3b` — Capital Ships + Fighters
- `0x34` — Death Star (special ship)
- `0x71` — Fighter squadron type (shield absorb alt-path, exact match only — NOT a range)
- `0x73-0x74` — Special combat entity (combat result special path, see FUN_005445d0)
- `0x90-0x98` — Star Systems

---

## Struct Layout: Entity Base Object

All combat entities share a common object layout. Fields identified across multiple functions:

| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x00` | `void*` | vtable | C++ vtable pointer |
| `+0x50` | `uint` | status_flags | Bitmask: bit0=active, bit3=fighter_combat_eligible, bit12=special_disabled |
| `+0x58` | `uint` | combat_phase_flags | Space combat bitfield (see space-combat.md) |
| `+0x60` | `int` | hull_current | Current hull / squad size (int for ships, int for squadrons) |
| `+0x66` | `short` | loyalty_current | Character loyalty (0-100) |
| `+0x78` | `byte` | capability_flags | Bit6+7=disabled, bit4=special_entity_flag, bit7=alt_shield_path |
| `+0x8a` | `short` | enhanced_loyalty | EnhancedLoyalty bonus value (0-100 clamped, also 0-0x7fff raw) |
| `+0x96` | `short` | regiment_strength | Troop/ship regiment current strength |
| `+0x9a` | `short` | hyperdrive_modifier | Mission hyperdrive speed bonus (Han Solo) |
| `+0xac` | `byte` | alive_flag | bit0=alive/combat-eligible |
| `+0xb0` | `byte` | combat_ready_flags | bit1=combat_ready_for_fleet_eval |

### Character Struct Additional Fields

| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x66` | `short` | loyalty | Base loyalty skill (0-100) |
| `+0x8a` | `short` | enhanced_loyalty | EnhancedLoyalty mission bonus |
| `+0x9a` | `short` | hyperdrive_mod | MissionHyperdriveModifier (Han Solo) |

### Capital Ship Struct Additional Fields

| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x60` | `int` | hull_damage | Current hull damage value |
| `+0x64` | `uint` | shield_weapon_packed | Packed field: bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated |

Note: offset `+0x64` decimal is `0x40` hex = 64 decimal. The code uses `*(uint*)((int)this + 100)` which is decimal 100 = 0x64.

---

## Struct Layout: Space Combat State Object (`this` in combat functions)

| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x00` | `void*` | vtable | vtable pointer |
| `+0x24` | `uint` | difficulty_packed | bits 4-5 = difficulty level (0-3) |
| `+0x50` | `uint` | fleet_status_flags | bit0=active, bit12=special_disable |
| `+0x58` | `uint` | combat_phase_flags | Phase state machine (see space-combat.md) |
| `+0x1c4` | `code*` | vtable_weapon_fire | vtable slot: weapon fire phase handler |
| `+0x1c8` | `code*` | vtable_shield_absorb | vtable slot: shield absorption handler |
| `+0x1d0` | `code*` | vtable_hull_damage | vtable slot: hull damage application |
| `+0x1d4` | `code*` | vtable_fighter_engage | vtable slot: fighter engagement |
| `+0x1d8` | `code*` | vtable_who_won | vtable slot: victory determination |

---

## FUN_0053a000 — Entity Validity Check

**Purpose**: Validates that `this` is a live, valid entity object. Returns 0 if the entity is in an invalid/dead state. Used as the entry guard in virtually every setter function.

**Called by**: All validate/setter functions before doing any work.

**Pattern**: `bVar2 = FUN_0053a000((int)this); iVar3 = CONCAT31(extraout_var, bVar2); if (iVar3 != 0) { ... }`

The `CONCAT31` + `extraout_var` pattern is Ghidra's way of expressing x86 `movsx`/sign-extension of a bool into a 32-bit int for comparison against zero.

---

## FUN_0053a010 — Entity Validity Check (Notification Path)

**Purpose**: Same as FUN_0053a000 but used by notification-dispatch functions (the ones that fire events without a range-check step). Slightly different signature (likely checks a different validity bit).

**Called by**: All `*_notif` functions (hull damage notif, force notif, etc.)

---

## FUN_0053fc90 — Range Validator

**Purpose**: `validate_range(value, min, max)` — checks that a value falls within [min, max]. Returns 0 if out of range.

**Signature**: `int FUN_0053fc90(int value, int min, int max)`

**Used by**: All validate functions for their range check step.

**Key calls**:
- `FUN_0053fc90(param_1, 0, 100)` — validates 0-100 range (loyalty, force percentages)
- `FUN_0053fc90(param_1, 0, iVar3)` — validates 0 to entity-defined max (hull, squad size)
- `FUN_0053fc90(param_1, 0, param_1)` — validates non-negative (troop strength, hyperdrive)

---

## FUN_0053e0f0 — Secondary Clamp

**Purpose**: Secondary clamp `clamp(value, min, 0x7fff)`. Applied after `FUN_0053fc90` in loyalty and enhanced loyalty functions to ensure the raw value never exceeds `0x7fff` (32767), likely for the underlying `short` storage.

**Called by**: FUN_004ee030 (enhanced loyalty), FUN_005341a0 (loyalty validate)

---

## FUN_00539fd0 — Side Observer Getter

**Purpose**: `get_side_observer(this, side)` — returns the notification observer object for a given combat side (1 = attacker, 2 = defender). Used before every notification dispatch.

**Signature**: `void* FUN_00539fd0(void* this, int side)`

**Pattern seen in all setter functions**:
```c
pvVar4 = FUN_00539fd0(this, 1);  // get attacker observer
FUN_005023b0(pvVar4, context);   // dispatch notification to attacker
pvVar4 = FUN_00539fd0(this, 2);  // get defender observer
FUN_005023b0(pvVar4, context);   // dispatch notification to defender
```

---

## FUN_00501490 — Hull Damage Validator / Setter

**File**: Likely `capship.cpp` or `combat.cpp`
**Called by**: Hull damage phase (FUN_005443f0 → FUN_00543cb0 → vtable chain)
**Calls**:
- FUN_0053a000 (validity check)
- vtable `*this+0x248` (get max hull — returns string for "Invalid_HullValueDamage_value_")
- FUN_0053fc90 (range check: 0 to max_hull)
- FUN_00539fd0 (get side observers)
- FUN_005023b0 (dispatch hull damage notification to each side)
- vtable `*this+0x29c` (fire damage change event: old_hull, new_hull, context)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x60` | `int` | hull_current | Read old value, write new value |
| vtable `+0x248` | `code*` | get_max_hull | Returns the maximum hull value for range check |
| vtable `+0x29c` | `code*` | on_hull_changed | Event: (old_value, new_value, context) |

### Pseudocode (annotated)
```c
int set_hull_damage(void* ship, int new_hull, void* context) {
    if (!is_valid(ship)) return 0;

    int max_hull = vtable[ship, 0x248]("Invalid_HullValueDamage_value_");
    int valid = range_check(new_hull, 0, max_hull);

    if (valid && ship->hull_current != new_hull) {
        int old_hull = ship->hull_current;
        ship->hull_current = new_hull;

        notify_side(get_observer(ship, 1), context);  // attacker
        notify_side(get_observer(ship, 2), context);  // defender

        vtable[ship, 0x29c](old_hull, ship->hull_current, context);
    }
    return valid;
}
```

### Game Rules Extracted
- Hull value has a per-entity maximum (read from vtable, not a global constant)
- Hull damage of 0 is valid (dead ship = 0 hull)
- Both sides are notified on every hull change
- The change is only applied and broadcast if `new_hull != current_hull` (no-op guard)
- Error string `"Invalid_HullValueDamage_value_"` confirms this is the hull damage setter

---

## FUN_00501510 — Shield Recharge Rate Validator / Setter

**File**: Likely `capship.cpp`
**Called by**: Combat init, shield phase setup
**Calls**:
- FUN_0053a000 (validity check)
- FUN_00500670 (get max shield recharge — returns the 4-bit max, likely 15)
- FUN_0053fc90 (range check: 0 to max)
- FUN_00539fd0 (get side observers)
- FUN_00502430 (dispatch shield recharge notification)
- vtable `*this+0x2a0` (fire shield recharge change event)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x64` (decimal 100) | `uint` | shield_weapon_packed | Bits 0-3 = shield_recharge_allocated |

### Pseudocode (annotated)
```c
int set_shield_recharge(void* ship, uint new_rate, void* context) {
    if (!is_valid(ship)) return 0;

    int max_rate = get_max_shield_recharge();  // FUN_00500670, likely returns 15
    int valid = range_check(new_rate, 0, max_rate);

    if (valid) {
        uint old_rate = ship->shield_weapon_packed & 0xf;  // low 4 bits
        if (old_rate != new_rate) {
            // Write only the low 4 bits, preserving high bits
            ship->shield_weapon_packed = (new_rate ^ old_rate) & 0xf ^ ship->shield_weapon_packed;

            notify_side(get_observer(ship, 1), context);
            notify_side(get_observer(ship, 2), context);

            vtable[ship, 0x2a0](old_rate, ship->shield_weapon_packed & 0xf, context);
        }
    }
    return valid;
}
```

### Game Rules Extracted
- Shield recharge rate is stored in the **low 4 bits** of the packed field at offset +0x64 (decimal 100)
- Max value is 4 bits = 15 max. This matches the in-game 0-15 power allocation system
- The XOR-masking write `(new ^ old) & 0xf ^ packed` is a textbook read-modify-write for bitfields
- Shield recharge allocation and weapon recharge allocation coexist in the same 32-bit word

---

## FUN_005015a0 — Weapon Recharge Rate Validator / Setter

**File**: Likely `capship.cpp`
**Called by**: Combat init, weapon phase setup
**Calls**:
- FUN_0053a000 (validity)
- FUN_00500670 (get max weapon recharge)
- FUN_0053fc90 (range check)
- FUN_00539fd0 (observers)
- FUN_005024c0 (dispatch weapon recharge notification)
- vtable `*this+0x2a4` (fire weapon recharge change event)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x64` (decimal 100) | `uint` | shield_weapon_packed | Bits 4-7 = weapon_recharge_allocated |

### Pseudocode (annotated)
```c
int set_weapon_recharge(void* ship, uint new_rate, void* context) {
    if (!is_valid(ship)) return 0;

    int max_rate = get_max_weapon_recharge();
    int valid = range_check(new_rate, 0, max_rate);

    if (valid) {
        uint old_rate = (ship->shield_weapon_packed >> 4) & 0xf;  // bits 4-7
        if (old_rate != new_rate) {
            // Write bits 4-7, preserve all others
            ship->shield_weapon_packed = (new_rate & 0xf) << 4 | (ship->shield_weapon_packed & 0xffffff0f);

            notify_side(get_observer(ship, 1), context);
            notify_side(get_observer(ship, 2), context);

            vtable[ship, 0x2a4](old_rate, (ship->shield_weapon_packed >> 4) & 0xf, context);
        }
    }
    return valid;
}
```

### Game Rules Extracted
- Weapon recharge rate lives in **bits 4-7** of the same packed field as shield recharge
- Same 4-bit (0-15) range as shield recharge
- Shield and weapon share a single 32-bit word: `[......WW|WWSSSS]` (high bits unknown)
- vtable slot +0x2a4 vs +0x2a0 for shield: these are distinct event types

### Cross-Reference: Packed Field Structure at +0x64
```
Bit layout of *(uint*)((int)ship + 100):
  bits  0- 3: shield_recharge_allocated   (FUN_00501510)
  bits  4- 7: weapon_recharge_allocated   (FUN_005015a0)
  bits  8-31: unknown (preserved by both)
```

---

## FUN_005032c0 — Squad Size Damage Validator / Setter

**File**: Likely `fighter.cpp` or `squadron.cpp`
**Called by**: Fighter combat phase (FUN_005444e0 → FUN_00543d20 chain)
**Calls**:
- FUN_0053a000 (validity)
- vtable `*this+0x244` (get max squad size — same pattern as hull, returns string "Invalid_SquadSizeDamage_value_")
- FUN_0053fc90 (range check)
- FUN_00539fd0 (observers)
- FUN_005037f0 (dispatch squad size damage notification)
- vtable `*this+0x260` (fire squad size change event)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x60` | `int` | squad_size_current | Shares offset with hull_current — different object type |
| vtable `+0x244` | `code*` | get_max_squad_size | Returns max size for range check |
| vtable `+0x260` | `code*` | on_squad_changed | Event: (old_size, new_size, context) |

### Pseudocode (annotated)
```c
int set_squad_damage(void* squadron, int new_size, void* context) {
    if (!is_valid(squadron)) return 0;

    int max_size = vtable[squadron, 0x244]("Invalid_SquadSizeDamage_value_");
    int valid = range_check(new_size, 0, max_size);

    if (valid && squadron->squad_size_current != new_size) {
        int old_size = squadron->squad_size_current;
        squadron->squad_size_current = new_size;

        notify_side(get_observer(squadron, 1), context);
        notify_side(get_observer(squadron, 2), context);

        vtable[squadron, 0x260](old_size, squadron->squad_size_current, context);
    }
    return valid;
}
```

### Game Rules Extracted
- Fighter squadrons use offset `+0x60` for their "size" (same offset as hull in capital ships)
- Max squad size is entity-defined (per class in FIGHTSD.DAT, read from vtable)
- Squad size 0 = squadron destroyed
- vtable offset `+0x260` vs `+0x29c` for hull: distinguishable by vtable slot
- The error string `"Invalid_SquadSizeDamage_value_"` is at `0x006a97e0`

### Cross-Reference with FUN_00501490 (Hull)
Both hull and squad use `+0x60` for current value — this confirms hull (ships) and size (squadrons) are the same field, polymorphically interpreted by object type.

---

## FUN_004ee030 — Enhanced Loyalty Validator / Setter

**File**: Likely `character.cpp`
**Called by**: EnhancedLoyalty mission outcome handler
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053fc90 (range check: 0 to 100)
- FUN_0053e0f0 (secondary clamp: 0 to 0x7fff)
- FUN_00539fd0 (observers)
- FUN_004f0190 (dispatch EnhancedLoyalty notification)
- vtable `*this+0x318` (fire loyalty change event)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x8a` | `short` | enhanced_loyalty | 16-bit signed, 0-100 clamped, also raw-clamped to 0x7fff |
| vtable `+0x318` | `code*` | on_enhanced_loyalty_changed | Event: (old_value, new_value, context) |

### Pseudocode (annotated)
```c
int set_enhanced_loyalty(void* character, int new_loyalty, void* context) {
    if (!is_valid(character)) return 0;

    int valid = range_check(new_loyalty, 0, 100);
    clamp(new_loyalty, 0, 0x7fff);  // secondary safety clamp for short storage

    if (valid && character->enhanced_loyalty != new_loyalty) {
        short old_loyalty = character->enhanced_loyalty;
        character->enhanced_loyalty = (short)new_loyalty;

        notify_side(get_observer(character, 1), context);
        notify_side(get_observer(character, 2), context);

        vtable[character, 0x318](old_loyalty, character->enhanced_loyalty, context);
    }
    return valid;
}
```

### Game Rules Extracted
- EnhancedLoyalty is stored as a `short` (16-bit) at offset `+0x8a`
- Valid range: 0-100 (percentage)
- Secondary 0x7fff clamp guards against short overflow (belt-and-suspenders)
- This is the **mission bonus loyalty** field, not the base loyalty — distinct from FUN_005341a0
- vtable slot +0x318 distinguishes this from the base loyalty notifier at +0x238

---

## FUN_005341a0 — Base Loyalty Validator / Setter

**File**: Likely `character.cpp`
**Called by**: Loyalty change events (diplomacy missions, betrayal)
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053fc90 (range check: 0 to 100)
- FUN_0053e0f0 (secondary clamp: 0 to 0x7fff)
- FUN_00539fd0 (observers)
- FUN_00535380 (dispatch LoyaltyNotif)
- vtable `*this+0x238` (fire loyalty change event)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x66` | `short` | loyalty_base | Base loyalty skill, 0-100 |
| vtable `+0x238` | `code*` | on_loyalty_changed | Event: (old_value, new_value, context) |

### Game Rules Extracted
- Base loyalty at `+0x66` vs enhanced loyalty at `+0x8a` — two separate fields
- Both range-validated 0-100 with the same `0x7fff` secondary clamp
- vtable `+0x238` vs `+0x318`: base vs enhanced use different event slots

### Cross-Reference: Loyalty System Offsets
```
+0x66 — base loyalty         (FUN_005341a0, vtable +0x238)
+0x8a — enhanced loyalty     (FUN_004ee030, vtable +0x318)
```

---

## FUN_004ee470 — Hyperdrive Modifier Setter (Han Solo Speed Bonus)

**File**: Likely `character.cpp`
**Notif string**: `MissionHyperdriveModifier`
**Called by**: Mission outcome when Han Solo completes a mission with hyperdrive bonus
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053fc90 (range check: 0 to param_1 — self-bounded, non-negative check only)
- FUN_00539fd0 (observers)
- FUN_004f0610 (dispatch HyperdriveModifier notification)
- vtable `*this+0x338` (fire hyperdrive modifier change event)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x9a` | `short` | hyperdrive_mod | Mission speed modifier, 16-bit signed |
| vtable `+0x338` | `code*` | on_hyperdrive_changed | Event: (old_value, new_value, context) |

### Pseudocode (annotated)
```c
int set_hyperdrive_modifier(void* character, int new_mod, void* context) {
    if (!is_valid(character)) return 0;

    int valid = range_check(new_mod, 0, new_mod);  // non-negative only (0 = min = max = new_mod)

    if (valid && character->hyperdrive_mod != new_mod) {
        short old_mod = character->hyperdrive_mod;
        character->hyperdrive_mod = (short)new_mod;

        notify_side(get_observer(character, 1), context);
        notify_side(get_observer(character, 2), context);

        vtable[character, 0x338](old_mod, character->hyperdrive_mod, context);
    }
    return valid;
}
```

### Game Rules Extracted
- `FUN_0053fc90(param_1, 0, param_1)`: passing value as both value and max — this is a **non-negative check only**, not a 0-100 clamp. The modifier has no upper bound other than short max.
- Stored as `short` at `+0x9a` — max 32767
- This is the `MissionHyperdriveModifier` bonus, applied to Han Solo's fleet movement speed
- vtable `+0x338` is specific to hyperdrive changes
- In `rebellion-core/movement.rs`: this value should be consulted when calculating `ticks_per_hop` for fleets containing Han Solo

---

## FUN_004ee350 — Regiment / Unit Strength Setter (Ground Combat Core)

**File**: Likely `troop.cpp` or `unit.cpp`
**Called by**: FUN_00560d50 (ground combat), direct during troop damage application
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053fc90 (range check: 0 to param_1 — non-negative)
- FUN_00539fd0 (observers)
- FUN_004f04f0 (dispatch strength change notification)
- vtable `*this+0x330` (fire strength change — fires HullValueDamage/SquadSizeDamage per unit type)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x96` | `short` | regiment_strength | Current regiment strength (16-bit) |
| vtable `+0x330` | `code*` | on_strength_changed | Dispatches damage notification |

### Pseudocode (annotated)
```c
int set_regiment_strength(void* unit, int new_strength, void* context) {
    if (!is_valid(unit)) return 0;

    int valid = range_check(new_strength, 0, new_strength);  // non-negative only

    if (valid && unit->regiment_strength != new_strength) {
        short old_strength = unit->regiment_strength;
        unit->regiment_strength = (short)new_strength;

        notify_side(get_observer(unit, 1), context);
        notify_side(get_observer(unit, 2), context);

        vtable[unit, 0x330](old_strength, unit->regiment_strength, context);
    }
    return valid;
}
```

### Game Rules Extracted
- Regiment strength is a `short` at `+0x96`, 0 = destroyed
- No upper bound enforced here — the caller is responsible for providing valid max values
- vtable `+0x330` fires the unit-type-appropriate notification (troops get different notification than ships)
- This function is the ground combat damage application primitive — every troop hit passes through here

---

## FUN_004f1e00 — Force Experience Notification Dispatcher

**File**: Likely `character.cpp` or `force.cpp`
**Notif string**: `CharacterForceNotif` / `Force`
**Called by**: Force-related events (Jedi training, Force use)
**Calls**:
- FUN_0053a010 (validity — notification path)
- FUN_004f8980 (dispatch notification: CharacterForceNotif, "Force", param_2, param_1, null, param_3)
- FUN_0053fcf0 (register event 0x1e1 = 481 decimal)

### Pseudocode (annotated)
```c
void fire_force_notification(void* character, uint4 new_value, uint4 old_value, int context) {
    if (!is_valid_for_notification(character)) return;

    dispatch_notification(character,
        "CharacterForceNotif",   // notification type
        "Force",                  // event name
        old_value, new_value,
        null, context);
    register_event(0x1e1, character, new_value, old_value, context);
}
```

### Game Rules Extracted
- Event ID `0x1e1` (481) = Force experience change event
- Uses FUN_004f8980 (the two-entity notification pattern: char + param)
- This is a pure notification dispatcher — the actual Force value modification happens upstream

---

## FUN_004f1ea0 — Force Training Notification Dispatcher

**File**: Likely `character.cpp` or `force.cpp`
**Notif string**: `CharacterForceTrainingNotif` / `ForceTraining`
**Called by**: Jedi training mission completion
**Calls**:
- FUN_0053a010 (validity)
- FUN_004f8980 (dispatch: CharacterForceTrainingNotif, "ForceTraining", param_2, param_1, null, param_3)
- FUN_0053fcf0 (register event 0x1e5 = 485 decimal)

### Game Rules Extracted
- Event ID `0x1e5` (485) = Force training event (distinct from 0x1e1 Force experience)
- Training and experience are separate event channels — the game tracks them independently
- Both use FUN_004f8980 but with different notification strings

### Event ID Map (Force System)
| Event ID | Hex | Notification String | Meaning |
|----------|-----|--------------------|----|
| 481 | 0x1e1 | CharacterForceNotif | Force attribute change |
| 485 | 0x1e5 | CharacterForceTrainingNotif | Training event |

---

## FUN_0058a3f0 — Force User Detection

**File**: Likely `force.cpp` or `mission.cpp`
**Called by**: Mission system when checking for Force-sensitive characters
**Calls**:
- FUN_00505190 (resolve entity pointer from ID)
- FUN_0055e4d0 (Force detection calculation)
- FUN_005897b0 (unknown post-detection call)
- FUN_00506e80 (check: is Jedi-trainable?)
- FUN_0055ff60 (secondary detection check)
- FUN_0058a530 (finalize detection result)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `piVar4[9] >> 6 & 3` | bits | force_potential_tier | 2-bit Force potential level (0-3) |
| `piVar4[0x23]` (short) | `short` | force_experience | Current Force experience |
| `pvVar3+0x8c` (short) | `short` | force_potential_raw | Raw Force potential of target character |
| `*(byte*)(piVar4+0x1e) & 1` | bit | is_unknown_jedi | Character is a Jedi but not yet discovered |
| `this+0x4c` | `uint` | detection_pending | Set to 1 when undiscovered Jedi is detected |

### Pseudocode (annotated)
```c
int detect_force_user(void* detection_context,
                      uint* detector_id, uint* target_id, int mission_ctx) {
    void* detector = resolve_entity(detector_id);
    int* target    = resolve_entity(target_id);

    if (!detector || !target) return 0;

    int potential_tier = target[9] >> 6 & 3;          // 2-bit Force potential level
    short force_xp     = (short)target[0x23];          // Force experience
    short target_pot   = *(short*)((char*)detector + 0x8c);  // detector's Force potential

    int detected = FUN_0055e4d0(potential_tier, force_xp, target_pot, &out1, &out2);
    if (!detected) return 0;

    FUN_005897b0();  // post-detection cleanup

    if (out1 != 0 && detector && target) {
        if (!(target[0x1e] & 1)) {
            // Target is Jedi but unknown — flag for discovery
            detection_context->detection_pending = 1;
        }

        bool jedi_trainable = is_jedi_trainable();
        bool secondary_check = FUN_0055ff60(target, detector, mission_ctx);

        if (FUN_0058a530(context, detector_id, target_id) && jedi_trainable && secondary_check) {
            return 1;
        }
    }
    return 0;
}
```

### Game Rules Extracted
- Force detection uses a **2-bit tier** extracted from `target[9] >> 6 & 3` — four Force potential levels (0-3), matching the game's "Low/Medium/High/Very High" Force sensitivity tiers
- Force experience (`target[0x23]` as `short`) influences detectability — higher-experience Jedi are easier to detect
- An undiscovered Jedi (`!(target[0x1e] & 1)`) triggers a `detection_pending` flag on the context, initiating the discovery event chain
- Detection requires three conditions: `FUN_0055e4d0` (stat check), `FUN_0055ff60` (secondary check), and `FUN_0058a530` (finalization)

---

## FUN_00504a00 — Troop Destruction During Blockade

**File**: Likely `fleet.cpp` or `blockade.cpp`
**Notif string**: `TroopRegDestroyedRunningBlockade` / `TroopRegDestroyedRunningBlockad`
**Called by**: Blockade resolution when troops are caught in a hostile blockade
**Calls**:
- FUN_0053a010 (validity)
- FUN_004f6b70 (get troop transport association)
- FUN_004ece30 (reference counting / object management)
- FUN_004f8aa0 (dispatch TroopRegDestroyedRunningBlockade notification)
- FUN_0053fe40 (register event 0x340 = 832 decimal)
- FUN_00619730 (exception handler cleanup)

### Pseudocode (annotated)
```c
int destroy_troop_in_blockade(void* context, int troop_entity, int battle_ctx) {
    if (!is_valid_for_notification(context)) return 0;

    void* troop_ref = get_troop_ref(context);
    if (!troop_ref || troop_entity == 0) return 0;

    void* troop_ptr = resolve_ref(troop_entity);
    dispatch_notification(context,
        "TroopRegDestroyedRunningBlockad",
        "TroopRegDestroyedRunningBlockade",
        troop_ref, troop_ptr, null, battle_ctx);

    int result = register_event(0x340, context, &troop_ref, &troop_ptr, battle_ctx);
    return result;
}
```

### Game Rules Extracted
- Event ID `0x340` (832) = Troop regiment destroyed during blockade — specific event, not generic damage
- The notification string confirms this is **troop destruction during blockade transit** — troops being transported through a blockaded system are destroyed
- Uses FUN_004f8aa0 (the 2-entity `this` + two-object notification, vs FUN_004f8980's single-object)
- Two distinct notification strings (one slightly truncated) suggests the full name was trimmed in the binary resource

---

## FUN_0049eea0 — "Battle in Progress" UI Handler

**File**: Likely `ui.cpp` or `strategy_view.cpp`
**Called by**: Strategy view update loop when a battle is occurring
**Calls**:
- FUN_006037f0 (load font/resource, arg 7)
- FUN_00601880 (init display struct)
- FUN_004067d0 (check: is battle in progress?)
- FUN_006019a0 / FUN_00601990 (display text or overlay)
- Windows GDI: `GetDC`, `CreateCompatibleDC`, `ReleaseDC`, `SelectObject`, `DeleteDC`
- FUN_005fbd20 (create bitmap object)
- FUN_006075b0 (render to screen)
- FUN_00606980 (update viewport)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `param_1+0x18` | `HWND` | window_handle | The strategy view window |
| `param_1+0x30` | `uint` | display_id | Display/slot identifier |
| `param_1+0x114` | `void*` | battle_info_ptr | Pointer to current battle info struct |
| `battle_info_ptr+0x9c` | `int` | player_side | 1 = player is side 1, else side 2 |
| `battle_info_ptr+0xcc-0xd8` | `int[4]` | viewport_coords | x, y, w, h of the battle display area |
| `DAT_0065d424` | data | base_bitmap_id | Fallback bitmap when no battle |

### Pseudocode (annotated)
```c
void draw_battle_in_progress(void* strategy_view) {
    // Check battle state
    if (!is_battle_in_progress()) {
        // Show default/empty overlay
        display_bitmap(strategy_view, CONCAT22(DAT_0065d424, 0x1a00));
        return;
    }

    // Battle in progress — set up GDI rendering
    init_display_struct(display, "Battle in Progress...");

    HDC window_dc = GetDC(strategy_view->window_handle);
    HDC compat_dc = CreateCompatibleDC(window_dc);
    ReleaseDC(strategy_view->window_handle, window_dc);

    // Create bitmaps for battle display
    void* bitmap_a = create_bitmap(font_resource, 0x2a11, 10);
    void* bitmap_b = create_compatible_bitmap(bitmap_a, compat_dc);

    // Compose battle display
    compose_battle_display(bitmap_a, bitmap_b, 0, 0, 0, 0, null, 0);

    // Side-dependent color: 0xff (blue/Alliance) or 0xff00 (red/Empire)
    battle_info* bi = strategy_view->battle_info_ptr;
    int color = (bi->player_side == 1) ? 0xff : 0xff00;

    set_display_color(display, color, null_hwnd);
    render_to_dc(display, compat_dc);

    SelectObject(compat_dc, saved_obj);
    DeleteDC(compat_dc);

    // Blit to screen within viewport
    blit_to_screen(strategy_view, bitmap_b, 1);
    update_viewport(strategy_view,
        bi->viewport_x, bi->viewport_y,
        bi->viewport_w, bi->viewport_h);
}
```

### Game Rules Extracted
- `0xff` vs `0xff00` color selection based on `battle_info_ptr+0x9c == 1` — player side 1 = one color, side 2 = another (likely blue for Alliance, red for Empire)
- This is purely a UI/rendering function — no game logic. Does not affect combat outcomes.
- The `0x1a00` suffix in `CONCAT22(DAT_0065d424, 0x1a00)` is a resource ID component
- The "Battle in Progress" string at `0x006a879c` confirms this is the strategy view notification

---

## FUN_005617b0 — Death Star Planet Destruction Handler

**File**: Likely `deathstar.cpp` or `system.cpp`
**Called by**: FUN_00560d50 (ground/space combat) when family_id == 0x34
**Calls**:
- FUN_005070d0 (get current system object)
- FUN_004f6b50 (get system's parent / controlling faction)
- FUN_004025b0 (get entity ID)
- FUN_0055f650 (fire superlaser / apply Death Star effect)
- FUN_00619730 (exception cleanup)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `this_00+0xac` | `byte` | alive_flag | bit0: system target is alive |
| `this_00+0x50` | `uint` | status_flags | bit0: active; must be 1 to fire |
| `local_14 = 0x90000109` | `uint` | target_system_id | System DatId being targeted (family=0x90 = explored system) |
| `local_18+0x24` bits 6-7 | bits | faction_control | 0x80 = Empire control, checked via `& 0xc0 == 0x80` |

### Pseudocode (annotated)
```c
bool death_star_fire(void* death_star, void* context) {
    void* system = get_current_system();
    if (!system) return false;

    void* controlling_faction = null;
    bool has_faction = get_system_faction(system, &controlling_faction);
    if (!has_faction || !system) return false;
    if (!controlling_faction) return false;

    // Check: target is alive and Death Star is active
    bool ds_active = !(system->alive_flag & 1) && (system->status_flags & 1);

    if (ds_active) {
        uint target_id = 0x90000109;  // system DatId (family 0x90 = explored system, index 0x109)
        uint* faction_id = get_entity_id(controlling_faction);

        if (*faction_id == target_id &&
            (controlling_faction->faction_control & 0xc0) == 0x80) {
            // Empire controls this system — Death Star can fire
            fire_superlaser = true;
        }
    }

    int result = apply_death_star_effect(death_star, fire_superlaser, context);
    return result && has_faction;
}
```

### Game Rules Extracted
- The Death Star can only fire when: (a) target system is **not** already destroyed (`!(alive_flag & 1)`), AND (b) Death Star is active (`status_flags & 1`)
- `0x90000109` = a specific system DatId (family 0x90 = explored system, sequential index 0x109 = 265). This is likely **Alderaan** hardcoded as the canonical target, or the system currently targeted.
- Faction control check: `(faction_control & 0xc0) == 0x80` — the 0x80 bit in the high nibble of `+0x24` indicates Empire control
- The check `(alive_flag & 1) == 0` means target system's bit0 is clear (eligible for destruction). Note: bit0 semantics may differ between entity types — for combat units bit0=alive, but for Death Star targets bit0=0 may mean "not yet destroyed" (inverted encoding). The pseudocode `!(alive_flag & 1)` is what the C code does.
- `FUN_0055f650` is the actual superlaser effect applicator

---

## FUN_005029a0 — Hull Damage Notification Dispatcher

**File**: `capship.cpp`
**Notif string**: `CapShipHullValueDamageNotif` / `HullValueDamage`
**Called by**: Combat pipeline after hull damage is validated and applied
**Calls**:
- FUN_0053a010 (validity)
- FUN_004f8980 (dispatch: CapShipHullValueDamageNotif, HullValueDamage)
- FUN_0053fcf0 (register event 0x1c0 = 448 decimal)

### Game Rules Extracted
- Event ID `0x1c0` (448) = Capital ship hull damage event
- This is the **notification dispatcher** called after FUN_00501490 validates and writes the new hull value
- Pattern: validate → write → notify (FUN_00501490) vs just notify (FUN_005029a0). The latter is called by the change-event vtable

---

## FUN_005029f0 — Shield Recharge Rate Notification Dispatcher

**Notif string**: `CapShipShieldRechargeRateCHAllocatedNotif` / `ShieldRechargeRateCHAllocated`
**Event ID**: `0x1c1` (449)

---

## FUN_00502a40 — Weapon Recharge Rate Notification Dispatcher

**Notif string**: `CapShipWeaponRechargeRateCHAllocatedNotif` / `WeaponRechargeRateCHAllocated`
**Event ID**: `0x1c2` (450)

---

## FUN_005038e0 — Squad Size Damage Notification Dispatcher

**Notif string**: `FightSquadSquadSizeDamageNotif` / `SquadSizeDamage`
**Event ID**: `0x1a0` (416)

### Event ID Map (Combat Notifications)
| Event ID | Hex | Notification | Component |
|----------|-----|-------------|-----------|
| 448 | 0x1c0 | CapShipHullValueDamage | Capital ship hull |
| 449 | 0x1c1 | ShieldRechargeRateCHAllocated | Shield power allocation |
| 450 | 0x1c2 | WeaponRechargeRateCHAllocated | Weapon power allocation |
| 416 | 0x1a0 | SquadSizeDamage | Fighter squadron |

---

## FUN_00509620 — Combat Stat Extractor

**File**: Likely `unit.cpp` or `combatobj.cpp`
**Called by**: FUN_00555b30 (bombardment calculation) via FUN_00509620
**Purpose**: Extracts a 4-byte combat stat from an entity's associated combat data block

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `this+0x2c` | `void*` | combat_data_ptr | Pointer to associated combat data block |
| `combat_data_ptr+0x4c` | `uint4` | combat_stat | The stat value (bombardment attack/defense) |

### Pseudocode (annotated)
```c
void get_combat_stat(void* entity, uint4* out_stat) {
    if (entity->combat_data_ptr != null) {
        *out_stat = entity->combat_data_ptr->combat_stat;  // at +0x4c
    } else {
        *out_stat = 0;  // no combat data = zero stat
    }
}
```

### Game Rules Extracted
- Entities without a combat data block (`+0x2c == null`) contribute zero to combat calculations
- The stat at `+0x4c` within the combat data block is used for bombardment calculations (per bombardment.md)
- The `short[2]` that bombardment.md describes are likely packed into this `+0x4c` field

---

## FUN_005457f0 — Space Combat Entry Point

**File**: Likely `fleet.cpp` or `spacebattle.cpp`
**Called by**: System battle orchestrator (FUN_00514a60) for space combat
**Calls**:
- FUN_005438a0 (validate combat is space-eligible)
- FUN_00549910 (run the 7-phase space combat pipeline)

### Pseudocode (annotated)
```c
uint4 space_combat(void* battle_state, int* attacker, uint4 atk_id, uint4 def_id, void* context) {
    uint4 result = 1;  // default success
    if (validate_space_combat(attacker)) {
        result = run_space_combat_pipeline(battle_state, context);
    }
    return result;
}
```

### Game Rules Extracted
- Space combat is gated on `FUN_005438a0` which validates entity type codes
- If validation fails, returns 1 (success, no-op) — non-space entities gracefully skip

---

## FUN_005438a0 — Space Combat Entity Validator

**File**: Likely `spacebattle.cpp`
**Called by**: FUN_005457f0 (space combat entry)
**Purpose**: Checks if the given entity ID corresponds to a space-combat-eligible unit

### Validated Entity Type Codes
| Constant | Hex | Meaning |
|----------|-----|---------|
| `0x31000241` | — | Ship type 1 (likely Star Destroyer class) |
| `0x32000242` | — | Ship type 2 |
| `0x33000243` | — | Ship type 3 |
| `0x35000281` | — | Ship type 5 |
| `0x34000280` | — | Ship type 4 |
| `0x38000343` | — | Ship type 8 (likely Mon Calamari Star Cruiser) |

### Pseudocode (annotated)
```c
bool is_space_combat_eligible(int* entity_id) {
    int id = *entity_id;
    return id == 0x32000242 || id == 0x31000241 || id == 0x33000243 ||
           id == 0x38000343 || id == 0x35000281 || id == 0x34000280;
}
```

### Game Rules Extracted
- Exactly 6 entity type codes are space-combat eligible
- These appear to be specific DatId values, not family ranges — each is a specific unit type
- High bytes `0x31-0x38` align with the `0x30-0x3b` capital ship family range from ground-combat.md
- The `0x34` in `0x34000280` confirms the Death Star (family 0x34) is included in the space combat eligible set

---

## FUN_005442f0 — Combat Initialization

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 1 of the 7-phase space combat pipeline
**Called by**: FUN_005457f0 → FUN_00549910
**Calls**:
- FUN_005435f0 (get 3rd stat type — specials/bombers)
- FUN_004ece30/40 (reference counting)
- FUN_00543c40 (apply init result)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `entity+0xac` bit0 | bit | alive | Must be 1 |
| `entity+0x50` bit0 | bit | active | Must be 1 |
| `entity+0x78` bits 6-7 | bits | capability_flags | Must be 00 (unit not disabled) |
| `entity+0x50` bit12 | bit | special_disabled | Must be 0 |
| `0x98000481` | `uint` | space_battle_id | Specific battle context DatId |

### Pseudocode (annotated)
```c
bool combat_init(void* battle_state, void* context) {
    auto [entity, count, battle_id] = get_special_stats();
    if (!entity || !count) return valid;
    if (!resolve_ref(battle_id)) return valid;

    // Entity eligibility gate
    bool eligible = entity->alive &&
                    entity->active &&
                    (entity->capability_flags & 0xc0) == 0 &&
                    !(entity->special_disabled);

    int init_value = 0;
    if (eligible) {
        if (battle_id == 0x98000481) {  // specific battle context check
            init_value = 1;
        }
    }

    return apply_init(battle_state, init_value, context);
}
```

### Game Rules Extracted
- `0x98000481` is a specific DatId checked during init — likely the battle context object itself
- Four simultaneous conditions for combat eligibility: alive + active + not_disabled + not_special_disabled
- `capability_flags & 0xc0 == 0` means neither bit6 nor bit7 can be set (both represent disabled states)

---

## FUN_00544030 — Weapon Fire Phase (Space Combat Phase 3)

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 3 — Weapon Fire
**Called by**: FUN_00549910 (space combat pipeline)
**Calls**:
- FUN_005434b0 (get weapon stats for both sides)
- FUN_005439e0 (validate combat can proceed)
- FUN_00543b60 (resolve weapon fire — side 1, then side 0)

### Pseudocode (annotated)
```c
bool weapon_fire_phase(void* battle_state, void* context) {
    // Gate: bit0=active AND bit6=disabled
    if (!(battle_state->combat_phase_flags & 0x01) ||
         (battle_state->combat_phase_flags & 0x40)) {
        return true;  // skip, not applicable
    }

    auto [side1_entity, side2_entity, battle_ref] = get_weapon_stats();
    if (!side1_entity || !side2_entity) return valid;
    if (!resolve_ref(battle_ref)) return valid;

    if (validate_weapon_fire(side1_entity)) {
        fire_weapons(battle_state, side=1, context);  // attacker fires first
        fire_weapons(battle_state, side=0, context);  // defender fires second
    }
    return valid;
}
```

### Game Rules Extracted
- Phase only runs when `combat_phase_flags & 0x01 != 0` AND `combat_phase_flags & 0x40 == 0`
- Attacker (side=1) fires before defender (side=0) — initiative advantage
- If `FUN_005439e0` returns 0 (combat cannot proceed), weapon fire is skipped entirely

---

## FUN_00544130 — Shield Absorption Phase (Space Combat Phase 4)

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 4 — Shield Absorption
**Called by**: FUN_00549910
**Calls**:
- FUN_005434b0 (get weapon stats)
- FUN_005439b0 (validate shields exist)
- FUN_00543bd0 (apply shield absorption — both sides)
- FUN_0042d170 (get entity ID via temp buffer)
- FUN_00505970 (resolve entity pointer)
- vtable `piVar5+0x1d8` (special shield handler for non-standard units)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `entity+0x78` bit7 | bit | has_alt_shield | Triggers special shield path (0x80 check) |
| `0x71-0x72` | family range | fighter_shield_types | Families eligible for standard shield absorb |
| `piVar5[0x1a]` | int | unit_type_check | Must equal 8 for alt shield path |

### Game Rules Extracted
- Standard path: `entity->capability_flags & 0x80 == 0` — most ships
- Alternative path: `entity->capability_flags & 0x80 != 0` — triggers special handling
- Alternative path checks family 0x71 (fighter type) and calls vtable `+0x1d8`
- `piVar5[0x1a] == 8` — a specific unit type code for the alt shield path
- Phase gate: `battle_state->combat_phase_flags & 0x40 == 0` means "shields not yet processed"
- Returns `true` immediately if bit6 is NOT set in combat_phase_flags (shields not initialized)

---

## FUN_005443f0 — Hull Damage Application Phase (Space Combat Phase 5)

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 5 — Hull Damage
**Called by**: FUN_00549910
**Calls**:
- FUN_00543550 (get hull stats for both sides)
- FUN_005439e0 (validate)
- FUN_00543cb0 (apply hull damage — both sides)

### Pseudocode (annotated)
```c
bool hull_damage_phase(void* battle_state, void* context) {
    if (!(battle_state->combat_phase_flags & 0x40)) return true;  // skip

    auto [side1_hull, side2_hull, ref] = get_hull_stats();
    if (!side1_hull || !side2_hull || !resolve_ref(ref)) return valid;

    if (validate_hull_damage(side1_hull)) {
        apply_hull_damage(battle_state, side=1, context);
        apply_hull_damage(battle_state, side=0, context);
    }
    return valid;
}
```

### Game Rules Extracted
- Hull damage phase gated on same `0x40` flag as shield and fighter phases
- Same bilateral (both sides) resolution pattern as weapon fire

---

## FUN_005444e0 — Fighter Engagement Phase (Space Combat Phase 6)

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 6 — Fighter Engagement
**Called by**: FUN_00549910
**Calls**:
- FUN_00543690 (get fighter stats)
- FUN_005439e0 (validate)
- FUN_00543d20 (resolve fighter engagement)

### Game Rules Extracted
- Fighter engagement is the final active phase (phase 6 of 7)
- Same bilateral, gated-on-0x40 pattern as hull and shields

---

## FUN_005445d0 — Combat Result Determination

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 7 — Result
**Called by**: FUN_00549910
**Calls**:
- FUN_005434b0 (weapon stats)
- FUN_00543550 (hull stats)
- FUN_005435f0 (special/init stats)
- FUN_00543690 (fighter stats)
- FUN_00543760 (5th stat type — unknown)
- FUN_00543800 (6th stat getter — post-combat)
- FUN_004f2640 (iterate fleet units)
- FUN_0052bed0 / FUN_005130d0 (iterator advance)
- FUN_00543d90 (determine winner — side 1 then 0)
- FUN_00534640 (special entity victory path)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `entity+0xac` bit0 | bit | alive_flag | All 4 stat objects must show alive |
| `entity+0x50` bit3 | bit | fighter_combat_eligible | Fighters can be alive via this even when `alive_flag==0` |
| `0x73-0x74` | family | special_entity_types | Triggers FUN_00534640 special victory path |
| `entity+0x78` bit7 | bit | has_special_entity_flag | Triggers family 0x73 check |
| `entity+0x78` bit4 | bit | special_victory_eligible | Must be set for FUN_00534640 path |

### Pseudocode (annotated)
```c
int determine_combat_result(void* battle_state, void* context) {
    // Gather all 5 stat types
    auto [wpn_s1, wpn_s2, wpn_ref] = get_weapon_stats();
    auto [hul_s1, hul_s2, hul_ref] = get_hull_stats();
    auto [spc_s1, spc_s2, spc_ref] = get_special_stats();
    auto [fgt_s1, fgt_s2, fgt_ref] = get_fighter_stats();
    auto [unk_s1, unk_s2, unk_ref] = get_5th_stats();

    // All stat objects must be valid
    if (!all_valid) return 0;

    // Check alive status across all stat types
    bool all_alive = (wpn_s1->alive_flag & 1) &&
                     (hul_s1->alive_flag & 1) &&
                     (spc_s1->alive_flag & 1) &&
                     ((fgt_s1->alive_flag & 1) || (fgt_s1->status_flags & 8));  // fighter exception

    // Iterate remaining fleet units
    iterate_fleet(fleet_iter, side=1, type=2);
    while (has_next) {
        uint alive = fleet_unit->combat_phase_flags;  // check for survivors
        advance_iter();
    }

    if (has_survivors && all_alive) {
        if (!(entity->capability_flags & 0x80)) {
            // Standard victory determination
            determine_winner(battle_state, side=1, context);
            determine_winner(battle_state, side=0, context);
        } else {
            // Special entity path — check family 0x73
            uint family = entity_id >> 24;
            if (family == 0x73) {
                special_entity_victory(entity, side=1, context);  // FUN_00534640
            } else if (entity->capability_flags & 0x10) {
                // Another special entity type
                special_entity_victory_alt(entity, side=1, context);
            }
        }
    }
}
```

### Game Rules Extracted
- Five stat types are gathered before determining the winner (weapon, hull, special, fighter, unknown 5th)
- Fighter exception: dead fighters (`alive_flag & 1 == 0`) can still count as combat-eligible via `status_flags & 8`
- Family `0x73` triggers `FUN_00534640` — this is the **special entity victory** path (likely Death Star win condition or unique win state)
- Family `0x73` is distinct from `0x74` (also checked in alternate path at `local_30 = 0x74`)
- Fleet iteration uses `FUN_004f2640` with `side=1, type=2`
- The check `*(uint*)(iter + 0xac)` in the fleet loop reads the `alive_flag` field during iteration

---

## FUN_00544a20 — Post-Combat Cleanup

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 8 — Post-Combat
**Called by**: FUN_00549910 (final phase)
**Calls**:
- FUN_005434b0 (weapon stats)
- FUN_00543760 (5th stat type)
- FUN_00543800 (6th/cleanup stat type)
- FUN_00543e00 (apply post-combat — both sides)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `battle_state+0x58` bit12 | bit | post_combat_flag | `0x1000` — triggers post-combat |

### Game Rules Extracted
- Post-combat requires `combat_phase_flags & 0x1000 != 0` to run
- Requires 3 stat getters to return valid data
- `local_30 != local_2c` guard: the two stat identifiers must differ (not same entity)
- Final bilateral pass via `FUN_00543e00`

---

## FUN_00544da0 — Fleet Evaluation (Pre-Combat)

**File**: Likely `spacebattle.cpp`
**Phase**: Phase 2 — Fleet Evaluation
**Called by**: FUN_00549910
**Calls**:
- FUN_005434b0 (weapon stats)
- FUN_00543760 (5th stat type)
- FUN_00543800 (6th stat)
- FUN_005439e0 (validate both sides can engage)
- FUN_00543ee0 (apply fleet evaluation)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `entity+0xac` bit0 | bit | alive | Must be 1 for eval |
| `entity+0x50` bit0 | bit | active | Must be 1 |
| `entity+0x78` bits 6-7 | bits | capability_flags | Must be 00 |
| `entity+0x50` bit12 | bit | special_disable | Must be 0 |
| `entity+0xb0` bit1 | bit | combat_ready | Must be 1 |
| `entity+0x58` bit13 | bit | fleet_eval_skip | `0x2000` — skip eval if set |
| `0x98000481` | DatId | battle_context_id | Fleet eval only runs for this battle type |

### Game Rules Extracted
- Fleet evaluation is skipped if `combat_phase_flags & 0x2000` is set
- Full eligibility: alive + active + capability_flags_clear + not_special_disabled + **combat_ready (bit1 of +0xb0)**
- The `combat_ready` bit (`+0xb0 & 2`) is the key differentiator — units must be explicitly combat-ready
- `0x98000481` is checked again (same as in combat_init) — confirms this is the space battle context DatId
- The fleet eval only calls `FUN_00543ee0` once (not bilateral) — this is asymmetric

---

## FUN_00543b60 — Weapon Fire Resolver (Per Side)

**File**: Likely `spacebattle.cpp`
**Called by**: FUN_00544030 (weapon fire phase) for side=1 and side=0
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053a640 (set phase flag: `4` = weapon fire type code into `this+0x58`)
- FUN_00539fd0 (observers)
- FUN_0054a1d0 (weapon damage notification)
- vtable `*this+0x1c4` (dispatch weapon fire)

### Pseudocode (annotated)
```c
int resolve_weapon_fire(void* battle_state, int side, void* context) {
    if (!is_valid(battle_state)) return 0;

    int result = set_phase_flag(flag=4, side=side, &battle_state->combat_phase_flags);
    if (result != 0) {
        weapon_damage_notif(get_observer(battle_state, 1), context);
        weapon_damage_notif(get_observer(battle_state, 2), context);
        vtable[battle_state, 0x1c4](side, context);
    }
    return is_valid_result;
}
```

### Game Rules Extracted
- `FUN_0053a640(4, side, flags_ptr)` sets the weapon fire flag (`0x04`) in the combat phase bitfield
- vtable slot `+0x1c4` is the weapon fire handler — dispatches to the actual damage calculation
- Both observers are notified even when only one side fires in a given call

---

## FUN_00543d90 — Victory Determinator (Who Won)

**File**: Likely `spacebattle.cpp`
**Called by**: FUN_005445d0 (combat result) for side=1 and side=0
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053a640 (set phase flag: `0x200` = victory flag into `this+0x58`)
- FUN_00539fd0 (observers)
- FUN_0054a4a0 (victory notification)
- vtable `*this+0x1d8` (dispatch victory)

### Game Rules Extracted
- `0x200` (512) is the **victory flag** in combat_phase_flags — set when a side wins
- vtable slot `+0x1d8` handles victory processing
- Both sides receive victory notifications (one will be the winner, one the loser)

---

## FUN_00534640 — Special Entity Victory Handler

**File**: Likely `spacebattle.cpp` or `deathstar.cpp`
**Called by**: FUN_005445d0 (combat result, family 0x73 path)
**Calls**:
- FUN_0053a000 (validity)
- FUN_0053a640 (set phase flag: `8` = special victory into `this+0x78`)
- FUN_00539fd0 (observers)
- FUN_00535a50 (special victory notification)
- vtable `*this+600` = vtable `*this+0x258` (special victory dispatch)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `this+0x78` | `uint` | capability_flags | Flag `8` written here for special victory |
| vtable `+0x258` | `code*` | on_special_victory | Special victory event handler |

### Game Rules Extracted
- `FUN_0053a640(8, side, &this->capability_flags)` — writes flag value 8 into the capability_flags bitfield
- vtable offset 600 decimal = 0x258 — specific to special entity victory
- This path handles family 0x73 entities winning combat — likely the Death Star or a unique scenario win

---

## FUN_00560d50 — Ground Combat Resolution

**File**: Likely `groundcombat.cpp`
**Called by**: System battle orchestrator (FUN_00514a60)
**Calls**:
- FUN_00505190 (resolve ship entity from ID)
- FUN_00504f70 (get parent entity)
- FUN_004f25c0 (iterate all units at system, type=3)
- FUN_004ee350 (apply regiment strength change)
- FUN_005617b0 (Death Star handler for family 0x34)
- FUN_00504dc0 (get faction side for unit)
- FUN_004f6b70 (get character association)

### Key Offsets
| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `entity_id >> 24` | byte | family_byte | Entity class (0x30-0x3b=ship, 0x14-0x1b=troop, 0x08-0x0f=char) |
| `entity+0x96` | `short` | regiment_strength | Zero = non-combat unit |

### Pseudocode (annotated)
```c
bool ground_combat(void* battle, uint* attacker_id, uint* defender_id,
                   uint* context_id, void* combat_ctx) {
    uint family = *attacker_id >> 24;

    // Path 1: Ship — delegate to space combat
    if (family >= 0x30 && family <= 0x3b) {
        // Validate parent is also a ship
        int* parent = get_ship_entity(attacker_id);
        if (!parent) return false;

        uint parent_family = vtable[parent](family_getter);
        if (parent_family < 0x30 || parent_family > 0x3b) return false;

        // Death Star check
        uint special_family = vtable[parent](family_getter_2);
        if (special_family == 0x34) {
            death_star_fire(battle, combat_ctx);
        }

        // Regiment strength check
        if (parent->regiment_strength == 0) return success;  // no combat unit

        // Validate both sides are troops (0x14-0x1b)
        if (defender_family in [0x14..0x1b] && context_family in [0x14..0x1b]) {
            // Get faction sides — skip if same faction (no friendly fire)
            void* atk_side = get_faction(defender_id);
            void* def_side = get_faction(context_id);
            if (same_faction(atk_side, def_side)) skip_combat;
            apply_strength_change(parent, 0, combat_ctx);
        }
        return success;
    }

    // Path 2: Troop — ground combat
    if (family >= 0x14 && family <= 0x1b) {
        int* parent = get_parent_entity(attacker_id);
        if (!parent) return false;

        // Iterate all units at the system
        iterate_units(system, type=3);
        while (has_next) {
            void* unit = get_current();
            if (unit->regiment_strength != 0) {
                apply_strength_change(unit, 0, combat_ctx);  // apply damage
            }
            advance();
        }
        return success;
    }

    return true;  // unrecognized family — pass through
}
```

### Game Rules Extracted
- **No friendly fire**: `param_2 == param_1` (same faction ID) causes the combat call to be skipped
- **Regiment strength 0 = skip**: units with zero strength are non-combatants even during ground combat
- **Death Star triggers at family 0x34** — the family check is exact, not ranged
- Ground combat calls `FUN_004ee350(unit, 0, context)` — applying strength 0 (destroy) to each unit. The actual damage calculation must happen upstream, as this appears to be the kill step.
- Both parent validation checks confirm entity family before operating — strong type safety at runtime

---

## FUN_0050d5a0 — Mission Dispatch (13-Case Switch)

**File**: Likely `mission.cpp`
**Called by**: Mission execution system
**Purpose**: Dispatches mission-type-specific setup with appropriate filter criteria

### Mission Type Codes (switch cases)
| Case | Mission Type | Notes |
|------|-------------|-------|
| 0 | (no-op) | — |
| 1 | Diplomacy / Recruitment | Sets fleet filter |
| 2 | Special Forces Deployment | Sets unit filter |
| 3 | Mission type 3 | Ranges 0x10-0x28, two filters |
| 4 | Mission type 4 | Ranges 0xa0-0xb0, 0x28-0x30 |
| 5 | Mission type 5 | Range 0x08-0x10, fleet range 1-255 |
| 6 | Sabotage | Range 0x08-0x10, fleet 1-255, side filter, special_flag=true |
| 7 | Assassination | Range 0x08-0x10, fleet 1-255, side+troop filters |
| 8 | Mission type 8 | 4 separate range filters: 0x20-0x30, 0x10-0x20, 0xa0-0xb0, 0x08-0x10 + fleet/troop |
| 9 | Mission type 9 | Range 0x30-0x40 |
| 10 | Mission type 10 | Three fleet ranges: all 1-255 |
| 11 | Mission type 11 | Range 0x10-0x40 + 0xa0-0xb0 |
| 12 | Mission type 12 | Range 0x20-0x30 |
| default | Invalid | Sets bVar9=false |

### Game Rules Extracted
- Mission type 6 (Sabotage) is the only case that sets `bVar3=true` (special entity flag)
- Mission type 8 sets `bVar1=true` (uses a distinct execution path via vtable rather than FUN_0050fc80)
- Cases with `bVar2=true` use the standard `FUN_0050fc80/FUN_0050fbf0/FUN_004f9ca0` validation chain
- The range values (0x10, 0x20, 0x28, 0x30, 0xa0, 0xb0) are **entity type family ranges** used to filter eligible targets
- Post-switch: iterates over fleet units and sub-units to find matching targets in the filtered ranges
- `local_134=1, local_130=3` are constants set before the switch — likely min/max entity type specifiers

---

## FUN_00532e00 — Construction Yard Research Order Notification

**Notif string**: `SideConstructionYardRdOrderNotif` / `ConstructionYardRdOrder`
**Event ID**: `0x127` (295)

Note: The original `_victory.c` filename is misleading — this function fires a **construction yard research order** notification, not a victory condition. The victory condition function is elsewhere.

---

## FUN_00532f40 — Recruitment Done Notification

**Notif string**: `SideRecruitmentDoneNotif` / `RecruitmentDone`
**Event ID**: `300` (0x12c)
**Called by**: Recruitment mission completion handler

---

## FUN_00512280 — System Battle Notification

**Notif string**: `SystemBattleNotif` / `Battle`
**Event ID**: `0x14d` (333)
**Called by**: Battle orchestrator when a system battle begins

---

## FUN_005122d0 — System Blockade Notification

**Notif string**: `SystemBlockadeNotif` / `Blockade`
**Event ID**: `0x14e` (334)

---

## FUN_00512580 — System Uprising Incident Notification

**Notif string**: `SystemUprisingIncidentNotif` / `UprisingIncident`
**Event ID**: `0x152` (338)

---

## FUN_0054b7b0 — Luke Dagobah Mission Notification

**Notif string**: `MissionMgrLukeDagobahNotif` / `LukeDagobah`
**Event ID**: `0x221` (545)
**Called by**: Mission manager when Luke's Dagobah prerequisite is met

---

## FUN_0056fc70 — Dagobah Training Completed Notification

**Notif string**: `LukeDagobahCompletedNotif` / `DagobahCompleted`
**Event ID**: `0x210` (528)
**Called by**: Dagobah mission completion handler

### Game Rules Extracted
- Dagobah completed (0x210 = 528) has a lower event ID than Luke Dagobah required (0x221 = 545) — completion is a sub-event of the prerequisite chain

---

## FUN_00572b40 — Han Solo Bounty Attack Notification

**Notif string**: `HanBountyAttackNotif` / `BountyAttack`
**Event ID**: `0x212` (530)
**Called by**: Bounty hunter mission system when Han is attacked

---

## FUN_005738f0 — Mission Espionage Extra System Notification

**Notif string**: `MissionEspionageExtraSystemKeyNotif` / `ExtraSystem`
**Event ID**: `0x370` (880)
**Called by**: Espionage mission when an extra system is involved

---

## FUN_005434b0 — Weapon/Firepower Stat Getter

**File**: Likely `spacebattle.cpp`
**Called by**: Weapon fire phase (FUN_00544030), shield absorb, fleet eval, post-combat
**Purpose**: Returns pointers to both sides' weapon stat objects and resolves the battle reference

### Pseudocode (annotated)
```c
uint get_weapon_stats(void** out_entity, int* out_count, uint4* battle_ref) {
    *out_entity = null;
    *out_count = 0;
    init_ref(battle_ref);

    void* battle = get_current_battle();  // FUN_00507010
    *out_entity = battle;
    if (!battle) return 0;

    int* count_ptr = null;
    bool ok = get_weapon_count(battle, &count_ptr);
    if (ok) {
        uint* id = get_entity_id(count_ptr, &temp_id);
        pack_ref(battle_ref, id);
    }
    return ok;
}
```

### Game Rules Extracted
- `FUN_00507010` is the "get current battle context" function
- `FUN_004f6b50` gets the weapon count / secondary stat object
- `FUN_004025b0` extracts the entity ID
- `FUN_004f26d0` packs the ID into the reference

---

## Notification Architecture Summary

All notification-firing functions follow one of two patterns:

**Pattern A: Validate + Set + Notify** (setter functions)
```
FUN_0053a000 (validity check)
→ FUN_0053fc90 (range validate)
→ write new value
→ FUN_00539fd0 × 2 (get observers for side 1 and 2)
→ FUN_005023b0/FUN_004f04f0/etc (dispatch per-type notification)
→ vtable dispatch (fire change event with old/new values)
```

**Pattern B: Notify Only** (event dispatchers)
```
FUN_0053a010 (validity check — notification path)
→ FUN_004f8980/FUN_004f8880/FUN_004f8aa0 (dispatch notification)
→ FUN_0053fcf0/FUN_0053fdd0/FUN_0053fe40 (register event with numeric ID)
```

The three notification dispatchers differ in parameter count:
- `FUN_004f8880`: `(this, notif_str, event_str, param1, null, context)` — 1 entity param
- `FUN_004f8980`: `(this, notif_str, event_str, param2, param1, null, context)` — 2 entity params (reversed order)
- `FUN_004f8aa0`: `(this, notif_str, event_str, param2_ref, param1_ref, null, context)` — 2 entity ref params

The three event registrars differ in parameter count:
- `FUN_0053fcf0`: `(event_id, this, p1, p2, ctx)` — 2 extra params
- `FUN_0053fdd0`: `(event_id, this, p1, ctx)` — 1 extra param
- `FUN_0053fe40`: `(event_id, this, p1_ref, p2_ref, ctx)` — 2 ref params

---

## Cross-Reference: Object Offsets by Function

| Offset | Field | Functions |
|--------|-------|-----------|
| `+0x50` bits | status_flags | FUN_005442f0, FUN_00544da0, FUN_005617b0 |
| `+0x58` bits | combat_phase_flags | FUN_00544030, FUN_00544130, FUN_005443f0, FUN_005444e0, FUN_00544a20, FUN_00544da0, FUN_00543b60, FUN_00543d90 |
| `+0x60` int | hull_current / squad_size | FUN_00501490, FUN_005032c0 |
| `+0x64` (100) packed | shield+weapon recharge | FUN_00501510, FUN_005015a0 |
| `+0x66` short | base_loyalty | FUN_005341a0 |
| `+0x78` bits | capability_flags | FUN_005442f0, FUN_00544130, FUN_005445d0, FUN_005617b0, FUN_00534640 |
| `+0x8a` short | enhanced_loyalty | FUN_004ee030 |
| `+0x96` short | regiment_strength | FUN_004ee350, FUN_00560d50 |
| `+0x9a` short | hyperdrive_modifier | FUN_004ee470 |
| `+0xac` byte | alive_flag | FUN_005445d0, FUN_005617b0, FUN_005442f0 |
| `+0xb0` bit1 | combat_ready | FUN_00544da0 |

---

## Cross-Reference: vtable Slots

| vtable Offset | Decimal | Function | Purpose |
|--------------|---------|----------|---------|
| `+0x238` | 568 | on_loyalty_changed | Base loyalty setter (FUN_005341a0) |
| `+0x244` | 580 | get_max_squad_size | Squadron max size (FUN_005032c0) |
| `+0x248` | 584 | get_max_hull | Ship max hull (FUN_00501490) |
| `+0x258` | 600 | on_special_victory | Special entity win (FUN_00534640) |
| `+0x260` | 608 | on_squad_changed | Squad size event (FUN_005032c0) |
| `+0x29c` | 668 | on_hull_changed | Hull change event (FUN_00501490) |
| `+0x2a0` | 672 | on_shield_recharge_changed | Shield recharge event (FUN_00501510) |
| `+0x2a4` | 676 | on_weapon_recharge_changed | Weapon recharge event (FUN_005015a0) |
| `+0x318` | 792 | on_enhanced_loyalty_changed | Enhanced loyalty (FUN_004ee030) |
| `+0x330` | 816 | on_strength_changed | Regiment/troop strength (FUN_004ee350) |
| `+0x338` | 824 | on_hyperdrive_changed | Hyperdrive modifier (FUN_004ee470) |
| `+0x1c4` | 452 | weapon_fire_dispatch | Space combat weapon fire (FUN_00543b60) |
| `+0x1c8` | 456 | shield_absorb_dispatch | Shield absorb (space-combat.md) |
| `+0x1d0` | 464 | hull_damage_dispatch | Hull damage (space-combat.md) |
| `+0x1d4` | 468 | fighter_engage_dispatch | Fighter engagement (space-combat.md) |
| `+0x1d8` | 472 | victory_dispatch / alt_shield | Who won / alt shield path |

---

## Cross-Reference: Event IDs

| ID (hex) | ID (dec) | Event | Source Function |
|---------|---------|-------|----------------|
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

## Implementation Notes for Open Rebellion

### Combat System (rebellion-core)

**Key findings for implementing space combat auto-resolve:**

1. **Phase order is strict**: Init → Fleet Eval → Weapon Fire → Shield Absorb → Hull Damage → Fighter Engage → Result → Post-Combat
2. **Attacker fires first**: `FUN_00543b60(battle, side=1, ctx)` always before `side=0`
3. **Phase gates**: `combat_phase_flags & 0x01` for weapon fire; `combat_phase_flags & 0x40` for shields/hull/fighters; `combat_phase_flags & 0x1000` for post-combat
4. **Difficulty modifier**: extracted from `+0x24 bits 4-5` — 2-bit value (0-3) feeding into `FUN_004fd600`
5. **Victory detection**: iterates fleet survivors checking `alive_flag & 1` at `+0xac`, with fighter exception via `+0x50 & 8`

**Key findings for ground combat:**

1. No friendly fire: same-faction units do not fight
2. Regiment strength `+0x96` (short) — zero means destroyed
3. Troop damage = calling `FUN_004ee350(unit, 0, ctx)` — setting strength to 0
4. Death Star (family 0x34) gets its own handler called before normal ground combat

**Key findings for character system:**

1. Loyalty: base at `+0x66`, enhanced bonus at `+0x8a` — both 0-100, stored as `short`
2. Hyperdrive modifier: `+0x9a` (short), non-negative, no upper bound — affects Han Solo fleet speed
3. Force fields: `piVar4[0x23]` as `short` = Force experience; `[9] >> 6 & 3` = 2-bit Force tier
4. Two distinct Jedi states: `+0x1e & 1` = is_jedi_but_not_discovered (triggers discovery event)

**Object identity:**
- `0x90000109` — a specific explored system DatId (the Death Star target)
- `0x98000481` — the space battle context DatId (used as battle type identifier)
- `0x98` family prefix is consistent with system-related objects (0x90-0x98 range)
