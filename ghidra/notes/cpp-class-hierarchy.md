---
title: "REBEXE.EXE — C++ Class Hierarchy (Reconstructed)"
description: "Reconstructed C++ class hierarchy from Ghidra decompilation"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# REBEXE.EXE — C++ Class Hierarchy (Reconstructed)

Generated from Ghidra decompilation of REBEXE.EXE (Star Wars Rebellion 1998, LucasArts).
Cross-referenced with: annotated-functions.md, combat-formulas.md, side-numbering.md, story-missions.md, character-roster.md, death-star-strategic.md.

The game uses the **Notification/Observer pattern** with pure C++ virtual dispatch throughout. Every
mutable field on every game object is modified only through validated setters that check entity
validity, range-validate the new value, write the field, broadcast to both side-observers, then
fire a vtable change-event. No setter bypasses this chain.

---

## 1. Vtable Map

Vtable offsets are byte offsets from `*(void**)this` (i.e., `*this` dereferences to the vtable
array; `*this + N` is the pointer at slot N). All are confirmed from direct `(**(code **)(*this + N))`
call sites in decompiled code.

### GameObject Base Vtable Slots

These slots appear on all game objects; they are the base interface.

```
+0x004  GetFamilyId()                   — entity class identifier byte (id >> 0x18)
+0x00c  GetSideKey()                    — side, game numbering (1 Alliance, 2 Empire; side-numbering.md)
+0x034  GetSpeed()                      — hyperspace speed for transit (FUN_0055d8c0; ship FUN_00500820, fighter FUN_00502f80, default FUN_004f63f0).
+0x038  IsActive()                      — reads status_flags bit0
```

### Flag-Word Handler Slots (MissionMgr, CharacterMgr)

Each bit of a manager's `+0x58` flag word has a setter (`FUN_0053a640` with one mask), a
broadcaster and a handler slot (`story-missions.md`). The MissionMgr's:

```
+0x1c0  bit 0  LukeDagobahRequired   (setter FUN_00543af0)
+0x1c4  bit 2  LukeDagobah           (setter FUN_00543b60)
+0x1c8  bit 5  LukePalace            (setter FUN_00543bd0)
+0x1d0  bit 7  LeiaPalace            (setter FUN_00543cb0)
+0x1d4  bit 8  ChewbaccaPalace       (setter FUN_00543d20)
+0x1f4         called by the bit-2 broadcaster FUN_0054a1d0 with (bit 2, view)
```

The CharacterMgr's `+0x58` bit 0 is the Emperor-at-Coruscant flag: `FUN_005617b0` computes it
and `FUN_0055f650` writes it and calls slot `+0x1d8` (`death-star-strategic.md`).

### CapitalShip / FighterSquadron / Facility Vtable Slots

Hull, shield, and weapon change-event slots. Used by the validate-write-notify setter chain.

```
+0x244  GetMaxSquadSize()               — FighterSquadron only; returns max for range check;
                                           error string "Invalid_SquadSizeDamage_value_"
+0x248  GetMaxHullValue()               — CapitalShip/facility; returns max for range check;
                                           error string "Invalid_HullValueDamage_value_"
+0x260  NotifySquadChanged(old, new, ctx)  — FighterSquadron change event (FUN_005032c0)
+0x29c  NotifyHullChanged(old, new, ctx)   — CapitalShip hull change event (FUN_00501490)
+0x2a0  NotifyShieldChanged(old, new, ctx) — shield recharge rate change (FUN_00501510)
+0x2a4  NotifyWeaponChanged(old, new, ctx) — weapon recharge rate change (FUN_005015a0)
```

### Character / Unit Vtable Slots

Used by character stat setters; distinct from the ship slots above.

```
+0x238  NotifyLoyaltyChanged(old, new, ctx)         — base loyalty setter (FUN_005341a0)
+0x318  NotifyEnhancedLoyaltyChanged(old, new, ctx) — EnhancedLoyalty mission bonus (FUN_004ee030)
+0x330  NotifyCommandKindChanged(old, new, ctx)     — officer rank +0x96 change (FUN_004ee350)
+0x338  NotifyHyperdriveChanged(old, new, ctx)      — MissionHyperdriveModifier (FUN_004ee470)
```

---

## 2. Class Hierarchy (Reconstructed)

The hierarchy is inferred from:
- Which vtable slots a given setter accesses (different slots → different leaf type)
- Field offset overlap (hull_current and squad_size_current share +0x60 → polymorphic field)
- Family byte ranges from entity ID high-byte (`id >> 0x18`)
- Error strings (hull vs squad have distinct "Invalid_*_value_" literals at separate vtable slots)
- Notification strings (CapShip vs FightSquad vs TroopReg vs Character prefixes)

```
CRebObject (base)
│  +0x00  void* vtable
│  Virtual:
│    GetFamilyId(), GetSideKey(), IsActive(), GetSpeed()
│    (and many more base slots not yet traced)
│
├── CNotifyObject (adds observer plumbing)
│   │  +0x24  uint  side bits 6-7 (1 Alliance, 2 Empire); bits 4-5 a side selector for FUN_004fd600
│   │  +0x2c  void* the object's DAT record (a system's record +0x4c holds its x/y, FUN_00509620)
│   │  +0x50  uint  status_flags        — bit0=active
│   │  +0x58  uint  flag word, per class (MissionMgr story flags, CharacterMgr seat of power,
│   │                                     fleet order bits; see section 4)
│   │
│   ├── CCapitalShip
│   │      Family bytes: 0x14-0x1b (CAPSHPSD 0x14; the Death Star is CAPSHPSD record 136, family 0x18)
│   │      Factories: 0x14 FUN_004fffe0 (vtable 0x0065d650), 0x18 FUN_005002a0 (vtable 0x0065d930)
│   │      +0x60  int   hull_current             — current hull damage value
│   │      +0x64  uint  shield_weapon_packed      — bits 0-3: shield damage counter
│   │                                               bits 4-7: weapon damage counter
│   │                                               bits 8-23: four more counters (ship-damage-repair.md)
│   │      Virtual:
│   │        +0x248 GetMaxHullValue()             → error "Invalid_HullValueDamage_value_"
│   │        +0x26c / +0x270  fighter / regiment capacity (FUN_00502020)
│   │        +0x29c NotifyHullChanged(old,new,ctx)
│   │        +0x2a0 NotifyShieldChanged(old,new,ctx)
│   │        +0x2a4 NotifyWeaponChanged(old,new,ctx)
│   │      Event IDs: 0x1c0 (hull damage), 0x1c1 (shield recharge), 0x1c2 (weapon recharge)
│   │      The Death Star is this class with family 0x18; every rule keys on [0x18,0x1c)
│   │      (death-star-strategic.md). Tactical messages: cmSHIP_POST_DS_STATUS,
│   │      DEATHSTAR_UPDATE, DEATHSTAR_FIRE, DEATHSTAR_WITHDRAW
│   │
│   ├── CFighterSquadron
│   │      Family bytes: 0x1c-0x1f (FIGHTSD 0x1c)
│   │      +0x60  int   squad_size_current        — shares offset with hull_current
│   │      Virtual (override):
│   │        +0x244 GetMaxSquadSize()             → error "Invalid_SquadSizeDamage_value_"
│   │        +0x260 NotifySquadChanged(old,new,ctx)
│   │      Event ID: 0x1a0 (squad size damage)
│   │
│   ├── CTroopRegiment
│   │      Family bytes: 0x10-0x13 (TROOPSD 0x10)
│   │      Blockade event: 0x340 (TroopRegDestroyedRunningBlockade)
│   │
│   ├── CSpecialForces
│   │      Family bytes: 0x3c-0x3f (SPECFCSD 0x3c)
│   │
│   └── CFacility
│          Family bytes: 0x20-0x2f — Alliance HQ 0x20-0x21; defence 0x22-0x25 (KDY-150, LNR,
│          GenCore, Death Star shield); shipyard 0x28, training 0x29, construction yard 0x2a;
│          mine 0x2c, refinery 0x2d (facility-order-lists.md)
│          +0x60  int  hull_current                   — facility integrity
│          Virtual:
│            +0x248 GetMaxHullValue()  (same slot as CCapitalShip — shared base)
│            +0x29c NotifyHullChanged(old,new,ctx)
│
├── CCharacter (separate branch)
│   │  +0x66  short loyalty_base              — base loyalty skill (0-100)
│   │  +0x78  byte  mission flags             — bit4 on a mission it may resign; bits 6/7 adrift /
│   │                                           on a mission (story-missions.md)
│   │  +0x8a  short enhanced_loyalty          — EnhancedLoyalty mission bonus (0-100, short-safe)
│   │  +0x8c  short Force                     — read by detection FUN_0058a3f0 (force-users.md)
│   │  +0x96  short command_kind              — officer rank: 0 none, 1 Commander, 2 Admiral,
│   │                                           3 General (setter FUN_004ee350, character-roster.md)
│   │  +0x9a  short hyperdrive_mod            — MissionHyperdriveModifier (Han Solo bonus)
│   │  +0xac  byte  prisoner                  — bit0 (ai-mission-planning.md, victory-conditions.md)
│   │  +0xb0  byte  story flags               — Luke, Han, Leia (story-missions.md)
│   │  A character is free (FUN_005439e0) when +0xac bit0 is clear, +0x50 bit0 set,
│   │  +0x50 bit 0x1000 clear and +0x78 bits 6-7 clear.
│   │  Virtual:
│   │    +0x238 NotifyLoyaltyChanged(old,new,ctx)
│   │    +0x318 NotifyEnhancedLoyaltyChanged(old,new,ctx)
│   │    +0x330 NotifyCommandKindChanged(old,new,ctx)
│   │    +0x338 NotifyHyperdriveChanged(old,new,ctx)
│   │  Event IDs: 0x1e1 (Force), 0x1e5 (ForceTraining)
│   │
│   ├── CMajorCharacter
│   │      Family bytes: 0x30-0x35 (MJCHARSD; 0x34 is Palpatine 0x34000280)
│   │      is_unable_to_betray flag (Luke, Vader cannot switch sides)
│   │      Plot-gated: DarthToLukeFinalBattle, DarthToEmperorFinalBattle, LukeFinalBattleReady
│   │
│   └── CMinorCharacter
│          Family byte: 0x38 (MNCHARSD)
│          Stats: base+variance at game-start
│
├── CStarSystem
│      Family bytes: 0x90 (core, vtable 0x663698), 0x92 (rim, vtable 0x664098)
│      DatId example: 0x90000109 (Coruscant)
│      Fields from FUN_0049eea0:
│        +0x18  HWND   window_handle
│        +0x30  uint   display_id
│        +0x114 void*  battle_info_ptr
│
├── CFleet
│      Family bytes: 0x08-0x0f
│      Fleet battle entry: FUN_004ff7a0 (FleetBattleNotif)
│      Contains capital ships; a ship carries fighters, regiments, characters and special
│      forces of its side (FUN_00502020)
│      +0x58 bit 6: the fleet is in the current bombard order (bombardment.md)
│
└── CMission
       Family bytes: 0x41-0x73
       Mission type codes (from FUN_004fc080 switch):
         6  = Sabotage
         7  = Assassination
         21 = Autoscrap (0x15)
       Base duration, max officers, target_flags at entity layer
```

---

## 3. Known Vtable Pointers

Vtable address constants observed in decompiled code, mapped to class types.

| Vtable Address | Class | Source |
|---------------|-------|--------|
| `PTR_LAB_0066c2d0` | CTacticalBattleManager (or CShipGroup root) | FUN_005a7500, line 108: `*param_1 = &PTR_LAB_0066c2d0` |
| `PTR_FUN_0066bdd0` | Object list keyed by id | FUN_005a7500, slots 0x262-0x274: 7× `param_1[N] = &PTR_FUN_0066bdd0`, 12 bytes apart. Slot 2 `FUN_005f5560` orders nodes by `+0x18` id; slot 3 `FUN_005c6000` inserts only when `FUN_005ca4b0` finds no node with that id; dtor `FUN_005c5ff0` → list base `FUN_005f5300` |
| `PTR_FUN_0066bdb8` | Object list keyed by id | FUN_005a7500, slot 0x277: `param_1[0x277] = &PTR_FUN_0066bdb8`. Slot 4 `FUN_005c5d80` inserts only when `FUN_005c61d0` finds no node with the same `+0x18` id; dtor `FUN_005c5b60` → `FUN_005c6190` |
| `PTR_LAB_0066c390` | CShipEventHandler | FUN_005a7500, slot 0x291: `param_1[0x291] = &PTR_LAB_0066c390` |
| `PTR_FUN_0066c380` | CShipEventHandler variant B | FUN_005a7500, slot 0x394 |
| `PTR_FUN_0066c370` | CShipEventHandler variant C | FUN_005a7500, slot 0x397 |

**Note**: `FUN_005a7500` is the **tactical battle manager/controller constructor** — it allocates and initializes the object
that handles the 2D tactical battle view. It installs all tactical-layer message handlers (SHIP_ADD,
SHIP_REMOVE, SHIP_FIRELASERCANNON, SHIP_TAKE_LASER_HIT, DEATHSTAR_FIRE, etc.) and sets the root
vtable pointer to `PTR_LAB_0066c2d0`. `FUN_00585640` is a factory: it allocates
0x44 bytes and constructs the object with `FUN_005855f0`.

---

## 4. Object Field Layout

Consolidated struct layout for the shared base. Fields confirmed across multiple decompiled functions.

### CNotifyObject Base

| Offset | Type | Field | Source Functions | Notes |
|--------|------|-------|-----------------|-------|
| `+0x00` | `void*` | vtable | all setters | C++ vtable pointer |
| `+0x20` | `void*` | views | FUN_00539fd0 → FUN_005844e0 | copies 0-2 at `+0x24` + 4n of the pointed block (`side-numbering.md`) |
| `+0x24` | `uint` | owner and view | FUN_004f8c60, FUN_0054a1d0 | bits 6-7 owner (1 Alliance, 2 Empire, 3 Neutral); bits 4-5 the copy (0 master, 1 Alliance view, 2 Empire view) |
| `+0x2c` | `void*` | DAT record | FUN_00509620 | a system's record `+0x4c` holds its SYSTEMSD x/y (`build-delivery.md`) |
| `+0x50` | `uint` | status_flags | many | bit0=active |
| `+0x58` | `uint` | flag word | see below | meaning per class |
| `+0x60` | `int`  | hull_current / squad_size_current | FUN_00501490, FUN_005032c0 | Polymorphic: hull for ships and facilities, size for squadrons |
| `+0x64` | `uint` | shield_weapon_packed | FUN_00501510, FUN_005015a0 | bits 0-3: shield damage counter; bits 4-7: weapon damage counter; bits 8-23: four more (`ship-damage-repair.md`) |

### The `+0x58` Flag Word

| Class | Bits | Source |
|-------|------|--------|
| MissionMgr | story flags 0-14 (LukeDagobahRequired … BountyHuntersActive) | `story-missions.md`, setters `FUN_00543af0`-`FUN_00543fc0` |
| CharacterMgr | bit 0 = Palpatine free, active and at Empire-held Coruscant | `FUN_005617b0`, `FUN_0055f650` |
| CFleet | bit 6 = in the current bombard order | `bombardment.md` |

### Character-Specific Fields (CCharacter branch)

| Offset | Type | Field | Notes |
|--------|------|-------|-------|
| `+0x50` | `uint` | status | bit 0 active, bit 3 destroyed, bit `0x1000` must be clear to be free (`story-missions.md`) |
| `+0x66` | `short` | loyalty_base | 0-100, validated + 0x7fff clamped (FUN_005341a0) |
| `+0x78` | `byte` | mission flags | bit 0 read by detection `FUN_0058a3f0`; bit 4 on a mission it may resign; bits 6/7 adrift / on a mission |
| `+0x8a` | `short` | enhanced_loyalty | EnhancedLoyalty mission bonus (FUN_004ee030) |
| `+0x8c` | `short` | Force | both characters' `+0x8c` feed detection `FUN_0055e4d0` (`force-users.md`) |
| `+0x8e` | `short` | Force experience | `story-missions.md` |
| `+0x96` | `short` | command_kind | officer rank 0-3 (FUN_004ee350, `character-roster.md`) |
| `+0x9a` | `short` | hyperdrive_mod | MissionHyperdriveModifier (FUN_004ee470) |
| `+0xac` | `byte` | prisoner | bit 0 (`ai-mission-planning.md`, `victory-conditions.md`) |
| `+0xb0` | `byte` | story flags | Luke, Han, Leia (`story-missions.md`) |

### Tactical Battle Object (FUN_005a7500 — CTacticalBattleManager)

This 0x3a8-word object (initialized in FUN_005a7500) is the tactical view controller.

| Offset (words) | Field | Notes |
|---------------|-------|-------|
| `param_1[0]` | vtable | set to `&PTR_LAB_0066c2d0` |
| `param_1[4]` | message_router_a | FUN_00618c80(param_1+4, 0xc, 0x80, ...) — 0x80-slot message router |
| `param_1[0x184]` | message_router_b | FUN_00618c80(param_1+0x184, 0xc, 0x26, ...) — 0x26-slot router |
| `param_1[0x1f6]–[0x215]` | float4[4] | ship position/state blocks (init to 0,0,0,-1.0f) |
| `param_1[0x22b]` | `= 3` | init constant |
| `param_1[0x22c]` | `= 5` | init constant |
| `param_1[0x22d]` | `= 1` | init constant |
| `param_1[0x230]` | `= 1` | active flag |
| `param_1[0x231]` | `= 0` | phase/state |
| `param_1[0x232]` | `= 0` | phase/state |
| `param_1[0x236]` | `= 0` | cleanup state |
| `param_1[0x21f]` | `= 3` | init constant |
| `param_1[0x3a1]` | `= 0x13d` (317) | timer or counter |
| `param_1[0x3a5]` | `= 0` | state |
| `param_1[0x3a6]` | `= 0x13d` (317) | timer or counter |
| `param_1[0x3a7]` | `= 1` | active flag |
| `param_1[0x262]–[0x274]` (7 slots) | seven embedded object lists | each vtable set to `&PTR_FUN_0066bdd0` |
| `param_1[0x277]` | embedded object list | vtable set to `&PTR_FUN_0066bdb8` |

Message slots registered (in constructor order):

| Message String | Slot Index (words from base) |
|---------------|------------------------------|
| SHIP_ADD | +0x4 |
| SHIP_REMOVE | +0x7 |
| SHIP_ABSTRACT_DESTROY | +0xa |
| SHIP_DESTROY | +0xd |
| cmSHIP_POST_DS_STATUS | +0x10 |
| NAV_ADD | +0x13 |
| NAV_REMOVE | +0x16 |
| NAV_PURGE | +0x19 |
| NAV_DELETE | +0x1c |
| SHIPGROUP_MOVE | +0x1f |
| SHIPGROUP_NEWMISSION | +0x22 |
| SHIPGROUP_NEWFORMATION | +0x25 |
| SHIPGROUP_NEWTARGETLIST | +0x28 |
| SHIP_FIRELASERCANNON | +0x2b |
| SHIP_FIRETURBOLASER | +0x2e |
| SHIP_FIREIONCANNON | +0x31 |
| SHIP_FIRETORPEDO | +0x34 |
| SHIP_TAKE_LASER_HIT | +0x37 |
| SHIP_TAKE_ION_HIT | +0x3a |
| SHIP_TAKE_TURBO_HIT | +0x3d |
| SHIP_TAKE_TORPEDO_HIT | +0x40 |
| SHIP_ION_DAMAGE | +0x46 |
| SHIP_LAUNCH | +0x4f |
| SHIP_RECOVER | +0x52 |
| SHIP_ENGINE_DOWN | +0x55 |
| SHIP_WEAPON_DOWN | +0x58 |
| SHIP_TRACTOR_DOWN | +0x5b |
| SHIP_SHIELD_HIT | +0x5e |
| SHIP_WEAPON_HIT | +0x61 |
| SHIP_TRACTOR_HIT | +0x64 |
| SHIP_ENGINE_HIT | +0x67 |
| SHIP_HYPERDRIVE_HIT | +0x6a |
| SHIP_SHIELD_FIX | +0x6d |
| SHIP_WEAPON_FIX | +0x70 |
| SHIP_TRACTOR_FIX | +0x73 |
| SHIP_ENGINE_FIX | +0x76 |
| SHIP_HYPERDRIVE_FIX | +0x79 |
| SHIP_TRACTOR_LOCK | +0x7c |
| SHIP_TRACTOR_UNLOCK | +0x7f |
| SHIP_GRAVITY_LOCK | +0x82 |
| SHIP_GRAVITY_UNLOCK | +0x85 |
| SHIP_SET_RECOVERY_SHIP | +0x88 |
| SHIP_HYPERSPACE | +0x8b |
| SHIP_WITHDRAW | +0x8e |
| SHIP_SCUTTLE | +0x91 |
| TASKFORCE_NEW | +0x94 |
| FIGHTERGROUP_NEW | +0x97 |
| SHIPGROUP_DELETE | +0x9a |
| SHIPGROUP_ADDSHIP | +0x9d |
| SHIPGROUP_ADDTARGET | +0xa0 |
| SHIPGROUP_REMOVETARGET | +0xa3 |
| SHIPGROUP_REPLACETARGETLIST | +0xa6 |
| SHIPGROUP_REMOVESHIP | +0xa9 |
| SHIPGROUP_ADDNAVPOINT | +0xac |
| SHIPGROUP_REMOVENAVPOINT | +0xaf |
| SHIPGROUP_REPLACENAVLIST | +0xb2 |
| SHIPGROUP_CHANGETARGET | +0xb5 |
| SHIPGROUP_TACTMISS_CHANGE_STATE | +0xb8 |
| FORMATION_ACCELERATE | +0xbe |
| CAPITALSHIP_UPDATE | +0xc1 |
| FIGHTERSQUADRON_UPDATE | +0xc4 |
| TACTCHAR_UPDATE | +0xc7 (199) |
| TACTICALRESULT_UPDATE | +0xca |
| DEATHSTAR_UPDATE | +0xcd |
| DEATHSTAR_FIRE | +0xd0 |
| DEATHSTAR_WITHDRAW | +0xd3 |
| ALLIANCE_START_TURN | +0xd6 |
| ALLIANCE_END_TURN | +0xd9 |
| EMPIRE_START_TURN | +0xdc |
| EMPIRE_END_TURN | +0xdf |
| GAME_OVER | +0xe2 |
| SYSTEM_TURN_COMPLETED | +0xe5 |
| SYSTEM_PAUSE | +0xe8 |
| SYSTEM_UNPAUSE | +0xeb |
| SYSTEM_SAVE | +0xee |
| SYSTEM_QUIT | +0xf1 |
| SYSTEM_WAIT_FOR_GAME_OVER | +0xf4 |
| SYSTEM_GAME_OVER | +0xf7 |
| SYSTEM_SYNCHRONIZE_BEGIN | +0xfa |
| SYSTEM_SYNCHRONIZE_END | +0xfd |
| SYSTEM_ABORT | +0x100 |

---

## 5. Event ID Registry

All confirmed event IDs from `FUN_0053fcf0` / `FUN_0053fe40` call sites.

| Event ID | Hex | Notification String | Class / Context |
|----------|-----|--------------------|-|
| 416 | 0x1a0 | FightSquadSquadSizeDamageNotif / SquadSizeDamage | CFighterSquadron |
| 448 | 0x1c0 | CapShipHullValueDamageNotif / HullValueDamage | CCapitalShip |
| 449 | 0x1c1 | ShieldRechargeRateCHAllocated | CCapitalShip |
| 450 | 0x1c2 | WeaponRechargeRateCHAllocated | CCapitalShip |
| 481 | 0x1e1 | CharacterForceNotif / Force | CCharacter |
| 485 | 0x1e5 | CharacterForceTrainingNotif / ForceTraining | CCharacter |
| 832 | 0x340 | TroopRegDestroyedRunningBlockade | CTroopRegiment |

---

## 6. Utility Functions

Key support functions used across all class setters.

| Address | Signature | Purpose |
|---------|-----------|---------|
| `FUN_0053a000` | `bool(int this)` | True for the master copy (`+0x24` bits 4-5 clear); entry check in all setters |
| `FUN_0053a010` | `bool(int this)` | True for a view copy; notification path |
| `FUN_0053fc90` | `int(value, min, max)` | Range validator: returns 0 if out of [min, max] |
| `FUN_0053e0f0` | `int(value, min, max)` | Secondary clamp to [0, 0x7fff] for short storage |
| `FUN_00539fd0` | `void*(this, n)` | View copy `n` of the object (`FUN_005844e0` on `+0x20`, n 0-2): 1 the Alliance's view, 2 the Empire's (`side-numbering.md`) |
| `FUN_0053a640` | `void(mask, on, flags*)` | Sets (`on` nonzero) or clears the `mask` bits of a flag word through `FUN_0053a610` |
| `FUN_0053a610` | `void(mask, value, flags*)` | Writes `value & mask` into the masked bits when they differ |
| `FUN_005404d0` | `int(object, side)` | Event side mask: matches when the low four bits equal `side`, or either is 3 (`side-numbering.md`) |
| `FUN_004fd3d0` | `int()` | Returns the callback at `DAT_006b2af0`, or 1 when none is set |
| `FUN_004fd600` | `int(ctx, n)` | `+0xc` for n = 1 (Alliance), `+0x10` for n = 2 (Empire), else 0 |
| `FUN_00618b70` | `void*(size)` | Heap allocator (new) — called in GNPRTB parser constructor |

---

## 7. Design Patterns Summary

**Setter-Notify-Event chain** (all field mutations):
1. `FUN_0053a000` — the object is the master copy
2. vtable call to get max value (for hull, squad)  **or**  `FUN_00500670` (for shield/weapon rate)
3. `FUN_0053fc90` — range validate
4. Write field only if `new_value != current_value`
5. `FUN_00539fd0(this, 1)` + notify dispatcher → the Alliance's view
6. `FUN_00539fd0(this, 2)` + notify dispatcher → the Empire's view
7. vtable change-event slot → fires the registered listener chain

**Flag-bit setter pattern** (manager flag words at `+0x58`):
- `FUN_0053a000` validity check
- `FUN_0053a640(mask, on, &flags)` — set or clear the bit
- On a change: the bit's broadcaster for view 1 and view 2
- Then the bit's handler slot (`+0x1c0`-`+0x1ec` on the MissionMgr; `story-missions.md`)

**Bilateral view dispatch**: every state change on the master copy is pushed to the Alliance's
view (`n=1`) and the Empire's view (`n=2`). The observer is retrieved via `FUN_00539fd0`
and is itself a polymorphic object whose dispatch method is a separate call (e.g., `FUN_005023b0`
for hull notifications, `FUN_00502430` for shield, `FUN_005024c0` for weapon, `FUN_005037f0` for
squad, `FUN_004f04f0` for CommandKind).
