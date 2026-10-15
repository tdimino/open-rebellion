---
title: "REBEXE.EXE — Annotated Function Reference"
description: "Cross-referenced annotated function catalog from Ghidra decompilation"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# REBEXE.EXE — Annotated Function Reference

Generated from Ghidra decompilation of REBEXE.EXE (Star Wars Rebellion 1998, LucasArts).
Cross-referenced with: combat-formulas.md, story-missions.md, story-encounters.md, bombardment.md.

Entity family byte quick reference (from `id >> 0x18`; the `family_id` fields of the DAT files):
- `0x08-0x0f` — Fleets
- `0x10-0x13` — Troops
- `0x14-0x1f` — Ships (capital ships `0x14-0x1b`; CAPSHPSD's Death Star record is family `0x18`)
- `0x20-0x2f` — Facilities
- `0x30-0x3b` — Characters (Mon Mothma `0x30000240`, Han Solo `0x33000243`)
- `0x3c-0x3f` — Special forces
- `0x40-0x7f` — Missions (MISSNSD; `0x71` Dagobah, `0x72` Vacation and Sabbatical, `0x73` Pickup)
- `0x80-0x8f` — Sectors
- `0x90-0x97` — Star systems

Six major characters have their own family: `0x30` Mon Mothma, `0x31` Leia
Organa, `0x32` Luke Skywalker, `0x33` Han Solo, `0x34` Emperor Palpatine,
`0x35` Darth Vader (MJCHARSD, TEXTSTRA 10304-10307, 10368-10369). Minor
characters are family `0x38`.

---

## Struct Layout: Entity Base Object

All combat entities share a common object layout. Fields identified across multiple functions:

| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x00` | `void*` | vtable | C++ vtable pointer |
| `+0x50` | `uint` | status_flags | bit0=active; bit3 (Chewbacca, read by `FUN_005445d0`); bit `0x1000` must be clear for a free character (`FUN_005439b0`, `FUN_005439e0`) |
| `+0x60` | `int` | hull_current | Current hull / squad size (int for ships, int for squadrons) |
| `+0x66` | `short` | loyalty_current | Character loyalty (0-100) |
| `+0x78` | `byte` | mission_flags | On characters: bit 3 resign request (`FUN_00534640`), bit 4 may resign, bit 6 adrift, bit 7 on a mission (`story-missions.md`) |
| `+0x8a` | `short` | enhanced_loyalty | EnhancedLoyalty bonus value (0-100 clamped, also 0-0x7fff raw) |
| `+0x96` | `short` | command_kind | On characters: 0 none, 1 Commander, 2 Admiral, 3 General; set by `FUN_004ee350`, cleared when they or their capital ship move (`FUN_00560d50`) |
| `+0x9a` | `short` | hyperdrive_modifier | Mission hyperdrive speed bonus (Han Solo) |
| `+0xac` | `uint` | prisoner | On characters, bit 0 set while held prisoner (`ai-mission-planning.md`, `victory-conditions.md`) |
| `+0xb0` | `byte` | story_flags | Per story character; Luke's bit 1 gates FinalBattleReady (`story-missions.md`) |

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

## Struct Layout: MissionMgr (`this` in the story checks)

| Offset | Type | Field Name | Notes |
|--------|------|-----------|-------|
| `+0x00` | `void*` | vtable | vtable pointer |
| `+0x58` | `uint` | story_flags | Story-flag word, bits 0-14 (`story-missions.md`) |
| `+0x1c0`-`+0x1ec` | `code*` | vtable slots | One handler per story bit, called by that bit's setter |

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

**Purpose**: `get_side_view(this, side)` — returns side `side`'s view of the object (1 Alliance, 2 Empire) through `this+0x20` and `FUN_005844e0`, or 0 when `+0x20` is null. Every setter updates both views before its change slot.

**Signature**: `void* FUN_00539fd0(void* this, int side)`

**Pattern seen in all setter functions**:
```c
pvVar4 = FUN_00539fd0(this, 1);  // Alliance view
FUN_005023b0(pvVar4, context);   // update it
pvVar4 = FUN_00539fd0(this, 2);  // Empire view
FUN_005023b0(pvVar4, context);   // update it
```

---

## FUN_00501490 — Hull Damage Validator / Setter

**File**: Likely `capship.cpp` or `combat.cpp`
**Called by**: the tactical write-back (`FUN_0040cfc0` → `FUN_00501ae0` / `FUN_00501b70`; `simulate-runs-tactical-engine.md`)
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

        notify_side(get_observer(ship, 1), context);  // Alliance view
        notify_side(get_observer(ship, 2), context);  // Empire view

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
**Called by**: the tactical write-back `FUN_0040cfc0`
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
**Called by**: the tactical write-back `FUN_0040cfc0`
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
**Called by**: `FUN_00503370` (adds to the squad count)
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

## FUN_004ee350 — Setter for Short +0x96

`FUN_004ee350(object, value, ctx)`. On a master copy (`FUN_0053a000`) whose
new value passes the range check (`FUN_0053fc90`, `0..value`), and when the
value differs, it stores it at `+0x96`, updates side views 1 and 2
(`FUN_00539fd0`, `FUN_004f04f0`) and calls slot `+0x330(old, new, ctx)`.

`FUN_00560d50` calls it with 0 on characters (families `0x30-0x3b`) when they
or the capital ship carrying them move.

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

## FUN_0049eea0 — Battle Overlay Window Content

**Class**: slot 14 of vtable `0x0065bcb8`, the content builder of a 380x200
window created through `FUN_00606380`; `+0x114` is its owner, the galaxy view.
**Calls**: `FUN_004067d0` (returns `DAT_006b0df8`, the TEXTSTRA edition flag
that `data0.reb` sets in `FUN_00406600`; the "Attached: " switch in
`FUN_0044a2e0` reads the same flag), `FUN_006037f0`, `FUN_00601880`,
`FUN_005fbd20`, `FUN_006075b0`, `FUN_00606980`; GDI `GetDC`,
`CreateCompatibleDC`, `SelectObject`, `DeleteDC`.

### What it builds
- A text label: when `FUN_004067d0()` is 0 the text is TEXTSTRA id
  `CONCAT22(DAT_0065d424, 0x1a00)`; otherwise the literal
  "Battle in Progress..." at `0x006a879c`.
- STRATEGY bitmap `0x2a11` (10769) composed into a compatible bitmap.
- The label colour from the owner's side `owner +0x9c`: 1 (Alliance) gives
  COLORREF `0xff` (red), anything else (Empire) gives `0xff00` (green).
- Placement from the owner's `+0xcc`-`+0xd8` rectangle (`FUN_00606980`).

It renders only; it reads no battle state and changes no game state.

---

## FUN_005617b0 — Palpatine at Coruscant Flag

`FUN_005617b0(this, ctx)`, run by `FUN_00560d50` when the moved character is
family `0x34`. It finds Emperor Palpatine (`FUN_005070d0` looks up
`0x34000280`) and his location (`FUN_004f6b50`). The flag is 1 when his
`+0xac` bit 0 is clear, his `+0x50` bit 0 is set, the location is Coruscant
(`0x90000109`) and its owner bits (`+0x24 & 0xc0`) are `0x80`; otherwise 0.
`FUN_0055f650(this, flag, ctx)` writes it to bit 0 of `this+0x58`
(`FUN_0053a640`), updates side views 1 and 2 (`FUN_005622e0`) and calls slot
`+0x1d8(flag, ctx)`. The port names this flag the seat of power
(`crates/rebellion-core/src/combat.rs`).

---

## FUN_005029a0 — Hull Damage Notification Dispatcher

**File**: `capship.cpp`
**Notif string**: `CapShipHullValueDamageNotif` / `HullValueDamage`
**Called by**: the hull change path after `FUN_00501490` writes a new value
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

## FUN_00509620 — System Coordinates

`FUN_00509620(this, out)` writes the 4-byte value at `+0x4c` of the record
that `this+0x2c` points to, or 0 when `+0x2c` is null. On a star system
that value is the pair of shorts SYSTEMSD `x_position` / `y_position` (DAT
`0x24` / `0x26`).

Callers: `FUN_00555b30`, the transit-time step (`build-delivery.md`), and
the galaxy view's scaling `FUN_00459e30` (`sector-window-hit-test.md`).
Bombardment does not read it (`bombardment.md`).

---

## FUN_005457f0 — Story Hook (Event `0x320`)

`FUN_005457f0(this, object, a, b, ctx)` runs the story check chain
`FUN_00549910(this, ctx)` when `FUN_005438a0(object)` is true, and returns
1 otherwise. It is the event `0x320` hook (`story-encounters.md`);
`FUN_00514a60` calls it.

---

## FUN_005438a0 — Named-Character Check

**Called by**: the story hooks `FUN_00545240`, `FUN_00545540`, `FUN_005457f0`, `FUN_00545df0`, `FUN_00545e90`, `FUN_00546370`, `FUN_00546e30` (`story-encounters.md`)
**Purpose**: True if the given DatId is one of six named characters

### Accepted Character Ids
| Constant | Hex | Meaning |
|----------|-----|---------|
| `0x31000241` | — | Leia Organa (MJCHARSD, TEXTSTRA 10305) |
| `0x32000242` | — | Luke Skywalker (TEXTSTRA 10306) |
| `0x33000243` | — | Han Solo (TEXTSTRA 10307) |
| `0x35000281` | — | Darth Vader (TEXTSTRA 10369) |
| `0x34000280` | — | Emperor Palpatine (TEXTSTRA 10368) |
| `0x38000343` | — | Chewbacca (MNCHARSD) |

### Pseudocode (annotated)
```c
bool is_named_character(int* entity_id) {
    int id = *entity_id;
    return id == 0x32000242 || id == 0x31000241 || id == 0x33000243 ||
           id == 0x38000343 || id == 0x35000281 || id == 0x34000280;
}
```

### Game Rules Extracted
- Exactly six specific character DatIds pass: Leia, Luke, Han, the Emperor, Vader, and Chewbacca
- A change event on any of them runs the story check chain

---

## FUN_00549910 — Story Check Chain

The MissionMgr's story checks. `FUN_00549910(this, ctx)` calls ten checks
in a fixed order and ANDs their results; none short-circuits. Each check
reads where the story characters are and whether they are free, then sets
one bit of the story-flag word `this+0x58` through that bit's setter.
A **held** check writes its condition; a **pulse** check sets the bit and
clears it again, so the handler runs once. Handlers, timers and the
chains they drive: [story-missions.md](story-missions.md),
[story-encounters.md](story-encounters.md).

| Order | Check | Bit | Setter (mask, slot) | Condition |
|---|---|---|---|---|
| 1 | `FUN_005442f0` | 6 HanCapturedAtPalace, held | `FUN_00543c40` (`0x40`, `+0x1cc`) | Han (`FUN_005435f0`) a prisoner (`+0xac` bit 0), active (`+0x50` bit 0), `+0x78` bits 6-7 clear, `+0x50` bit `0x1000` clear, at Jabba's Palace `0x98000481` |
| 2 | `FUN_00544da0` | 12 FinalBattleReady, held | `FUN_00543ee0` (`0x1000`, `+0x1e4`) | bit 13 clear; Luke a prisoner, active, `+0x78` bits 6-7 clear, `+0x50` bit `0x1000` clear, not at Jabba's Palace, Luke `+0xb0` bit 1; Vader and the Emperor free (`FUN_005439e0`) |
| 3 | `FUN_00544030` | 2 LukeDagobah, pulse | `FUN_00543b60` (`4`, `+0x1c4`) | bit 0 set, bit 6 clear, Luke free |
| 4 | `FUN_00544130` | 5 LukePalace, pulse | `FUN_00543bd0` (`0x20`, `+0x1c8`) | bit 6 set; Luke passes `FUN_005439b0` and is not on a mission (`+0x78` bit 7). When Luke is on a Dagobah mission (family `0x71`) whose state `+0x68` is 8, the check calls that mission's slot `+0x1d8` instead |
| 5 | `FUN_005443f0` | 7 LeiaPalace, pulse | `FUN_00543cb0` (`0x80`, `+0x1d0`) | bit 6 set, Leia (`FUN_00543550`) free |
| 6 | `FUN_005444e0` | 8 ChewbaccaPalace, pulse | `FUN_00543d20` (`0x100`, `+0x1d4`) | bit 6 set, Chewbacca (`FUN_00543690`) free |
| 7 | `FUN_005445d0` | 9 DarthPickup, pulse | `FUN_00543d90` (`0x200`, `+0x1d8`) | Luke, Leia and Han prisoners; Chewbacca a prisoner or `+0x50` bit 3 set; a prisoner among the members of Jabba's Palace (`FUN_00506ed0`, root `+0xdc`; `FUN_004f2640`); Vader passes `FUN_005439b0` and is not on a mission. Vader on a Pickup mission (family `0x73`) leaves the bit alone; Vader on another mission with `+0x78` bit 4 set is asked to resign (`FUN_00534640`) |
| 8 | `FUN_00544a20` | 10 DarthToLukeFinalBattle, pulse | `FUN_00543e00` (`0x400`, `+0x1dc`) | bit 12 set; Luke, Vader and the Emperor found; Vader not where Luke is |
| 9 | `FUN_00544be0` | 11 DarthToEmperorFinalBattle, pulse | `FUN_00543e70` (`0x800`, `+0x1e0`) | bit 12 set; Vader where Luke is, the Emperor elsewhere |
| 10 | `FUN_00544fc0` | 13 FinalBattle | `FUN_00543f50` (`0x2000`, `+0x1e8`), then `FUN_00544da0` | bit 12 set; Luke, Vader and the Emperor in one place |

**Free** (`FUN_005439e0`): `+0xac` bit 0 clear, `+0x50` bit 0 set, `+0x50`
bit `0x1000` clear, `+0x78` bits 6 and 7 clear. `FUN_005439b0` is the same
test without bit 7, so a character on a mission passes it.

### The bit setters

`FUN_00543af0` (bit 0) and `FUN_00543b60`-`FUN_00543fc0` (bits 2 and 5-14)
share one shape:

```c
int set_story_bit(MissionMgr* this, int value, void* ctx) {
    if (!FUN_0053a000(this)) return 0;                       // master copy only
    if (FUN_0053a640(MASK, value, &this->story_flags)) {     // bit really changed
        BROADCAST(FUN_00539fd0(this, 1), ctx);               // FUN_0054a1d0 .. FUN_0054a770
        BROADCAST(FUN_00539fd0(this, 2), ctx);
        this->vtable[SLOT](value, ctx);                      // +0x1c0 .. +0x1ec
    }
    return 1;
}
```

### FUN_00534640 — Resign Request

`FUN_00534640(character, value, ctx)` writes bit 3 (`8`) of the character's
`+0x78` through `FUN_0053a640`, broadcasts to both side views
(`FUN_00535a50`) and calls slot `+0x258` (600). `FUN_005445d0` calls it
with 1 to have Vader leave his current mission for the Pickup.

---

## FUN_00560d50 — Moved Object: Clear Character Field +0x96

`FUN_00560d50(this, object, from, to, ctx)`, called by `FUN_00514a60`.
`object`, `from` and `to` are DatIds.

- **Character** (`object` family `0x30-0x3b`, resolved with `FUN_00505190`):
  when it is family `0x34` (Emperor Palpatine) it runs `FUN_005617b0`. When
  its short `+0x96` is non-zero, it clears it (`FUN_004ee350(character, 0,
  ctx)`) unless `from` and `to` are both capital ships (`0x14-0x1b`) whose
  containers (`FUN_00504dc0`, `FUN_004f6b70`) are the same fleet
  (`0x08-0x0f`).
- **Capital ship** (`object` family `0x14-0x1b`, resolved with
  `FUN_00504f70`): every member of type 3 aboard (`FUN_004f25c0`) whose
  `+0x96` is non-zero has it cleared the same way.
- Any other family returns true at once.

`FUN_004ee350(object, value, ctx)` is the setter of `+0x96`: on a master
copy, with the value in range (`FUN_0053fc90`), it stores the value, updates
both side views (`FUN_004f04f0`) and calls slot `+0x330(old, new, ctx)`.

`+0x96` is the character's CommandKind: 0 none, 1 Commander, 2 Admiral, 3 General (`character-roster.md`).

---

## FUN_0050d5a0 — Observation Level

`FUN_0050d5a0(system, level, side, ctx)` copies what `side` (1 or 2)
may know about `system` into that side's view, and the switch picks the level
(0-12). Each case sets DatId family ranges to copy and the system fields to
sync. Callers pass levels: informants draw one from INFORMTB (`FUN_0050d510`),
espionage passes 10 (`FUN_00573610`), the mission end slot 6
(`FUN_00592c80`), event `0x319` 11 (`FUN_005650f0`). The `FUN_0050fc80` /
`FUN_0050fbf0` / `FUN_0050f390` family are field syncs from the master copy. Full table and callers: `observation-levels.md`.

---

## FUN_00532e00 — Construction Yard Research Order Notification

**Notif string**: `SideConstructionYardRdOrderNotif` / `ConstructionYardRdOrder`
**Event ID**: `0x127` (295)

This function fires a **construction yard research order** notification. The victory flags are `FUN_005180e0` (`victory-conditions.md`).

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

## FUN_005434b0 — Luke and His Location

`FUN_005434b0(out_character, out_location, out_key)` looks up Luke
(`FUN_00507010`, id `0x32000242`), stores him in `out_character`, and when
he exists stores his location (`FUN_004f6b50`) in `out_location` and its
key (`FUN_004025b0`, `FUN_004f26d0`) in `out_key`. It returns true when
both are found.

`FUN_00543550` (Leia, `FUN_00506fb0`), `FUN_005435f0` (Han, `FUN_00506f50`),
`FUN_00543690` (Chewbacca, `FUN_005066a0` on `0x38000343`), `FUN_00543760`
(Vader, `FUN_00507070`) and `FUN_00543800` (the Emperor, `FUN_005070d0`)
are the same accessor for the other story characters
(`story-encounters.md`). Callers are the story checks above and the
story handlers (`FUN_00549a20`, `FUN_0054ac70`, `FUN_0054adf0`,
`FUN_0054b040`).

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
| `+0x50` bits | status_flags | FUN_005439b0, FUN_005439e0, FUN_005442f0, FUN_005445d0, FUN_00544da0, FUN_005617b0 |
| `+0x58` bits | MissionMgr story_flags | FUN_00549910 and its ten checks; setters FUN_00543af0-FUN_00543fc0 |
| `+0x60` int | hull_current / squad_size | FUN_00501490, FUN_005032c0 |
| `+0x64` (100) packed | shield+weapon recharge | FUN_00501510, FUN_005015a0 |
| `+0x66` short | base_loyalty | FUN_005341a0 |
| `+0x78` bits | mission_flags (characters) | FUN_005439b0, FUN_005439e0, FUN_005442f0, FUN_00544130, FUN_005445d0, FUN_00534640 |
| `+0x8a` short | enhanced_loyalty | FUN_004ee030 |
| `+0x96` short | character command_kind (officer rank), cleared on moves | FUN_004ee350, FUN_00560d50 |
| `+0x9a` short | hyperdrive_modifier | FUN_004ee470 |
| `+0xac` bit 0 | prisoner (characters) | FUN_005439b0, FUN_005439e0, FUN_005442f0, FUN_005445d0, FUN_00544da0, FUN_005617b0, FUN_005180e0 |
| `+0xb0` bit1 | Luke story flag | FUN_00544da0 |

---

## Cross-Reference: vtable Slots

| vtable Offset | Decimal | Function | Purpose |
|--------------|---------|----------|---------|
| `+0x238` | 568 | on_loyalty_changed | Base loyalty setter (FUN_005341a0) |
| `+0x244` | 580 | get_max_squad_size | Squadron max size (FUN_005032c0) |
| `+0x248` | 584 | get_max_hull | Ship max hull (FUN_00501490) |
| `+0x258` | 600 | on_resign_request | Character `+0x78` bit 3 change (FUN_00534640) |
| `+0x260` | 608 | on_squad_changed | Squad size event (FUN_005032c0) |
| `+0x29c` | 668 | on_hull_changed | Hull change event (FUN_00501490) |
| `+0x2a0` | 672 | on_shield_recharge_changed | Shield recharge event (FUN_00501510) |
| `+0x2a4` | 676 | on_weapon_recharge_changed | Weapon recharge event (FUN_005015a0) |
| `+0x318` | 792 | on_enhanced_loyalty_changed | Enhanced loyalty (FUN_004ee030) |
| `+0x330` | 816 | on_field_96_changed | Character `+0x96` change (FUN_004ee350) |
| `+0x338` | 824 | on_hyperdrive_changed | Hyperdrive modifier (FUN_004ee470) |
| `+0x1c0`-`+0x1ec` | 448-492 | MissionMgr story-bit handlers | One per bit, called by its setter (FUN_00543af0-FUN_00543fc0) |
| `+0x1d8` | 472 | mission slot | On the Dagobah mission object, called by FUN_00544130 in state 8 |

---

## Cross-Reference: Event IDs

| ID (hex) | ID (dec) | Event | Source Function |
|---------|---------|-------|----------------|
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

## Implementation Notes for Open Rebellion

### Story checks (rebellion-core)

The ten story checks are event-driven: any change event on one of the six
story characters (`FUN_005438a0`) runs the chain `FUN_00549910`. They read
prisoner state (`+0xac` bit 0), activity and mission flags, and locations;
they apply no combat damage. Space battles resolve in the tactical engine
(`simulate-runs-tactical-engine.md`).

**Key findings for character system:**

1. Loyalty: base at `+0x66`, enhanced bonus at `+0x8a` — both 0-100, stored as `short`
2. Hyperdrive modifier: `+0x9a` (short), non-negative, no upper bound — affects Han Solo fleet speed
3. Force fields: `piVar4[0x23]` as `short` = Force experience; `[9] >> 6 & 3` = 2-bit Force tier
4. Two distinct Jedi states: `+0x1e & 1` = is_jedi_but_not_discovered (triggers discovery event)

**Object identity:**
- `0x90000109` — Coruscant
- `0x98000481` — Jabba's Palace (ABODESD, family `0x98`; `story-missions.md`)
