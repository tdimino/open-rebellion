# REBEXE.EXE — C++ Class Hierarchy (Reconstructed)

Generated from Ghidra decompilation of REBEXE.EXE (Star Wars Rebellion 1998, LucasArts).
Cross-referenced with: annotated-functions.md, combat-formulas.md, space-combat.md.

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

These slots appear on all combat entity objects; they are the base interface.

```
+0x004  GetFamilyId()                   — entity class identifier byte (id >> 0x18)
+0x00c  GetSideKey()                    — which combat side (1=attacker, 2=defender)
+0x034  CanBombard()                    — boolean, whether unit can perform orbital bombardment
+0x038  IsActive()                      — reads status_flags bit0
```

### CapitalShip / CombatEntity Vtable Slots

Hull, shield, and weapon change-event slots. Used by the validate-write-notify setter chain.

```
+0x1c4  ResolveWeaponFire(side, ctx)    — phase 3: per-side weapon fire resolver
                                           (FUN_00543b60 → gate on combat_phase_flags bit0 & ~bit6)
+0x1c8  ResolveShieldAbsorb(side, ctx)  — phase 4: per-side shield absorption resolver
                                           (FUN_00543bd0 → gate on combat_phase_flags bit5/0x20)
+0x1d0  ResolveHullDamage(side, ctx)    — phase 5: per-side hull damage resolver
                                           (FUN_00543cb0 → gate on combat_phase_flags bit7/0x80)
+0x1d4  ResolveFighterEngage(side, ctx) — phase 6: per-side fighter engagement resolver
                                           (FUN_00543d20 → gate on combat_phase_flags bit8/0x100)
+0x1d8  ResolveShieldAlt(side, ctx)     — alt shield path for special units (family 0x71-0x72,
                                           capability_flags bit7 set); called from FUN_00544130
+0x1f4  ApplyWeaponDamage(side, ctx)    — notification handler called by FUN_0054a1d0;
                                           also reached as vtable+500 decimal (0x1f4)
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

Used by character stat setters; distinct from the ship combat slots above.

```
+0x238  NotifyLoyaltyChanged(old, new, ctx)         — base loyalty setter (FUN_005341a0)
+0x318  NotifyEnhancedLoyaltyChanged(old, new, ctx) — EnhancedLoyalty mission bonus (FUN_004ee030)
+0x330  NotifyCombatStrengthChanged(old, new, ctx)  — regiment/troop strength change (FUN_004ee350)
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
│    GetFamilyId(), GetSideKey(), IsActive(), CanBombard()
│    (and many more base slots not yet traced)
│
├── CNotifyObject (adds observer plumbing)
│   │  +0x2c  void* combat_data_ptr     — ptr to associated combat data block
│   │  +0x50  uint  status_flags        — bit0=active, bit3=fighter_combat_eligible,
│   │                                     bit12=special_disabled
│   │  +0x58  uint  combat_phase_flags  — space combat phase state machine bitfield
│   │  +0x78  byte  capability_flags    — bit4=special_victory_eligible,
│   │                                     bit6=disabled_a, bit7=alt_shield_path / disabled_b
│   │  +0xac  byte  alive_flag          — bit0=alive/combat-eligible
│   │  +0xb0  byte  combat_ready_flags  — bit1=combat_ready_for_fleet_eval
│   │
│   ├── CCombatUnit (adds the combat phase vtable block)
│   │   │  Virtual (new in CCombatUnit):
│   │   │    +0x1c4 ResolveWeaponFire
│   │   │    +0x1c8 ResolveShieldAbsorb
│   │   │    +0x1d0 ResolveHullDamage
│   │   │    +0x1d4 ResolveFighterEngage
│   │   │    +0x1d8 ResolveShieldAlt
│   │   │    +0x1f4 ApplyWeaponDamage
│   │   │
│   │   ├── CCapitalShip
│   │   │      Family bytes: 0x30-0x33, 0x35-0x38, 0x3a-0x3b
│   │   │      +0x60  int   hull_current             — current hull damage value
│   │   │      +0x64  uint  shield_weapon_packed      — bits 0-3: shield_recharge_allocated
│   │   │                                               bits 4-7: weapon_recharge_allocated
│   │   │                                               bits 8-31: unknown (preserved)
│   │   │      +0x96  short regiment_strength         — (ships only: used for unit strength too)
│   │   │      Virtual (override from CCombatUnit):
│   │   │        +0x248 GetMaxHullValue()             → error "Invalid_HullValueDamage_value_"
│   │   │        +0x29c NotifyHullChanged(old,new,ctx)
│   │   │        +0x2a0 NotifyShieldChanged(old,new,ctx)
│   │   │        +0x2a4 NotifyWeaponChanged(old,new,ctx)
│   │   │      Vtable address: PTR_LAB_0066c2d0  (set in FUN_005a7500 constructor at *param_1)
│   │   │      Event IDs: 0x1c0 (hull damage), 0x1c1 (shield recharge), 0x1c2 (weapon recharge)
│   │   │
│   │   ├── CDeathStar
│   │   │      Family byte: 0x34
│   │   │      Inherits CCapitalShip layout; special-cased in combat result (FUN_005445d0)
│   │   │      Family 0x73-0x74: "special combat entity" — triggers FUN_00534640 victory path
│   │   │      Fire condition: !(alive_flag & 1) AND (status_flags & 1)
│   │   │      faction_control field: bits 6-7 at +0x24 of faction object (0x80 = Empire)
│   │   │      Tactical message: cmSHIP_POST_DS_STATUS, DEATHSTAR_UPDATE, DEATHSTAR_FIRE,
│   │   │                         DEATHSTAR_WITHDRAW
│   │   │
│   │   └── CFighterSquadron
│   │          Family bytes: 0x71-0x72 (shield absorb special path), 0x30-0x3b range
│   │          +0x60  int   squad_size_current        — shares offset with hull_current
│   │          Virtual (override):
│   │            +0x244 GetMaxSquadSize()             → error "Invalid_SquadSizeDamage_value_"
│   │            +0x260 NotifySquadChanged(old,new,ctx)
│   │            +0x1d8 ResolveShieldAlt (triggered by capability_flags bit7)
│   │          Event ID: 0x1a0 (squad size damage)
│   │          Combat eligible via dead state: status_flags bit3 overrides alive_flag==0
│   │
│   ├── CTroopRegiment
│   │      Family bytes: 0x14-0x1b
│   │      +0x96  short regiment_strength             — ground combat strength (16-bit)
│   │      Virtual:
│   │        +0x330 NotifyCombatStrengthChanged(old,new,ctx)
│   │      Ground combat primitive: FUN_004ee350 (set_regiment_strength)
│   │      Blockade event: 0x340 (TroopRegDestroyedRunningBlockade)
│   │
│   ├── CSpecialForces  (sub-type of troop family)
│   │      Family bytes: 0x14-0x1b (overlaps troops — distinguished by mission_id bitmask)
│   │      Layout: same as CTroopRegiment at combat layer
│   │
│   └── CFacility
│          Family bytes: 0x1c-0x1f (defense/manufacturing/production)
│          +0x60  int  hull_current                   — facility integrity
│          Virtual:
│            +0x248 GetMaxHullValue()  (same slot as CCapitalShip — shared base)
│            +0x29c NotifyHullChanged(old,new,ctx)
│
├── CCharacter (separate branch — no CCombatUnit)
│   │  +0x66  short loyalty_base              — base loyalty skill (0-100)
│   │  +0x8a  short enhanced_loyalty          — EnhancedLoyalty mission bonus (0-100, short-safe)
│   │  +0x9a  short hyperdrive_mod            — MissionHyperdriveModifier (Han Solo bonus)
│   │  Force fields (from FUN_0058a3f0):
│   │    entity[9] >> 6 & 3                   — force_potential_tier (2-bit, 0-3)
│   │    entity[0x23]  (short)                — force_experience
│   │    entity+0x8c   (short)                — force_potential_raw
│   │    entity[0x1e] & 1                     — is_unknown_jedi flag
│   │  Virtual:
│   │    +0x238 NotifyLoyaltyChanged(old,new,ctx)
│   │    +0x318 NotifyEnhancedLoyaltyChanged(old,new,ctx)
│   │    +0x330 NotifyCombatStrengthChanged(old,new,ctx)  (reused on Character for combat skill)
│   │    +0x338 NotifyHyperdriveChanged(old,new,ctx)
│   │  Event IDs: 0x1e1 (Force), 0x1e5 (ForceTraining)
│   │
│   ├── CMajorCharacter
│   │      Family bytes: 0x08-0x0b
│   │      is_unable_to_betray flag (Luke, Vader cannot switch sides)
│   │      Plot-gated: DarthToLukeFinalBattle, DarthToEmperorFinalBattle, LukeFinalBattleReady
│   │
│   └── CMinorCharacter
│          Family bytes: 0x0c-0x0f
│          Stats: base+variance at game-start
│
├── CStarSystem
│      Family bytes: 0x90 (explored), 0x92 (unexplored)
│      DatId target example: 0x90000109 (Alderaan or current Death Star target)
│      Fields from FUN_0049eea0:
│        +0x18  HWND   window_handle
│        +0x30  uint   display_id
│        +0x114 void*  battle_info_ptr
│
├── CFleet
│      Fleet battle entry: FUN_004ff7a0 (FleetBattleNotif)
│      Contains: CCapitalShip + CFighterSquadron list + CCharacter list
│
└── CMission
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
| `PTR_FUN_0066bdd0` | CShipGroupHandler (message router) | FUN_005a7500, slots 0x262-0x274: 7× `param_1[N] = &PTR_FUN_0066bdd0` |
| `PTR_FUN_0066bdb8` | CNavPointHandler | FUN_005a7500, slot 0x277: `param_1[0x277] = &PTR_FUN_0066bdb8` |
| `PTR_LAB_0066c390` | CShipEventHandler | FUN_005a7500, slot 0x291: `param_1[0x291] = &PTR_LAB_0066c390` |
| `PTR_FUN_0066c380` | CShipEventHandler variant B | FUN_005a7500, slot 0x394 |
| `PTR_FUN_0066c370` | CShipEventHandler variant C | FUN_005a7500, slot 0x397 |

**Note**: `FUN_005a7500` is the **tactical ship constructor** — it allocates and initializes the object
that handles the 2D tactical battle view. It installs all tactical-layer message handlers (SHIP_ADD,
SHIP_REMOVE, SHIP_FIRELASERCANNON, SHIP_TAKE_LASER_HIT, DEATHSTAR_FIRE, etc.) and sets the root
vtable pointer to `PTR_LAB_0066c2d0`. The `FUN_00585640` constructor (GNPRTB parser) allocates
0x44 bytes and calls `FUN_005855f0` — this is the GNPRTB parameter table reader, not a game object.

---

## 4. Object Field Layout

Consolidated struct layout for the shared base. Fields confirmed across multiple decompiled functions.

### CNotifyObject / CCombatUnit Base (all combat entities)

| Offset | Type | Field | Source Functions | Notes |
|--------|------|-------|-----------------|-------|
| `+0x00` | `void*` | vtable | all setters | C++ vtable pointer |
| `+0x24` | `uint` | difficulty_packed | FUN_0054a1d0 | bits 4-5 = difficulty level 0-3 (space combat) |
| `+0x2c` | `void*` | combat_data_ptr | FUN_00509620 | → combat data block; `+0x4c` within = bombardment stat |
| `+0x4c` | `uint` | detection_pending | FUN_0058a3f0 | set to 1 when undiscovered Jedi detected (context obj) |
| `+0x50` | `uint` | status_flags | many | bit0=active, bit3=fighter_combat_eligible, bit12=special_disabled |
| `+0x58` | `uint` | combat_phase_flags | FUN_00543b60-d20, space-combat.md | Phase state machine — see bitfield table below |
| `+0x60` | `int`  | hull_current / squad_size_current | FUN_00501490, FUN_005032c0 | Polymorphic: hull for ships, size for squadrons |
| `+0x64` | `uint` | shield_weapon_packed | FUN_00501510, FUN_005015a0 | bits 0-3: shield_recharge_alloc; bits 4-7: weapon_recharge_alloc |
| `+0x66` | `short` | loyalty_base | FUN_005341a0 | Character only: base loyalty skill (0-100) |
| `+0x78` | `byte` | capability_flags | FUN_005442f0, FUN_00544130, FUN_005445d0 | bit4=special_victory_eligible, bit6=disabled_a, bit7=alt_shield_path/disabled_b |
| `+0x8a` | `short` | enhanced_loyalty | FUN_004ee030 | Character only: EnhancedLoyalty mission bonus (0-100) |
| `+0x96` | `short` | regiment_strength | FUN_004ee350 | Troop/ship regiment current strength |
| `+0x9a` | `short` | hyperdrive_mod | FUN_004ee470 | Character only: MissionHyperdriveModifier (Han Solo) |
| `+0xac` | `byte` | alive_flag | FUN_005445d0, FUN_005617b0 | bit0=alive/combat-eligible; 0=dead |
| `+0xb0` | `byte` | combat_ready_flags | annotated-functions.md | bit1=combat_ready_for_fleet_eval |

### combat_phase_flags Bitfield (at +0x58)

| Bit | Mask | Meaning | Gate Phase |
|-----|------|---------|-----------|
| 0 | `0x0001` | Space combat active | Phase 3 weapon fire entry |
| 1 | `0x0002` | Weapon fire (attacker side) | — |
| 2 | `0x0004` | Weapon fire type code | Phase 3 type |
| 5 | `0x0020` | Shield absorb type code | Phase 4 type |
| 6 | `0x0040` | Combat phases enabled (shields, hull, fighters) | Phases 4-6 gate |
| 7 | `0x0080` | Hull damage type code | Phase 5 type |
| 8 | `0x0100` | Fighter engagement type code | Phase 6 type |
| 12 | `0x1000` | Post-combat flag | Phase 8 gate |

### Character-Specific Fields (CCharacter branch)

| Offset | Type | Field | Notes |
|--------|------|-------|-------|
| `+0x66` | `short` | loyalty_base | 0-100, validated + 0x7fff clamped |
| `+0x8a` | `short` | enhanced_loyalty | EnhancedLoyalty mission bonus |
| `+0x8c` | `short` | force_potential_raw | Detector's Force sensitivity (FUN_0058a3f0) |
| `+0x9a` | `short` | hyperdrive_mod | MissionHyperdriveModifier |
| `entity[9] >> 6 & 3` | 2 bits | force_potential_tier | 0-3 (None/Low/Medium/High/Very High) |
| `entity[0x23]` | `short` | force_experience | Influences Jedi detectability |
| `entity[0x1e] & 1` | bit | is_unknown_jedi | 0 = undiscovered Jedi |

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
| `param_1[0x262]–[0x274]` (7 slots) | CShipGroupHandler vtables | each set to `&PTR_FUN_0066bdd0` |
| `param_1[0x277]` | CNavPointHandler vtable | set to `&PTR_FUN_0066bdb8` |

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
| `FUN_0053a000` | `bool(int this)` | Entity validity guard — entry check in all setters |
| `FUN_0053a010` | `bool(int this)` | Entity validity guard — notification path variant |
| `FUN_0053fc90` | `int(value, min, max)` | Range validator: returns 0 if out of [min, max] |
| `FUN_0053e0f0` | `int(value, min, max)` | Secondary clamp to [0, 0x7fff] for short storage |
| `FUN_00539fd0` | `void*(this, side)` | Side observer getter: side=1 (attacker), side=2 (defender) |
| `FUN_0053a640` | `int(type_mask, side, flags*)` | Phase gate evaluator — checks combat_phase_flags bitmask |
| `FUN_0053a610` | `int(type_mask, flags, flags*)` | Alternate phase gate (notification side) |
| `FUN_005404d0` | `int(this, entity_id_at+8)` | Entity ID lookup from param |
| `FUN_004fd3d0` | `int()` | Difficulty check — returns non-zero if combat enabled |
| `FUN_004fd600` | `int(ctx, difficulty_bits)` | Apply difficulty modifier (extracts bits 4-5 of +0x24) |
| `FUN_00618b70` | `void*(size)` | Heap allocator (new) — called in GNPRTB parser constructor |

---

## 7. Design Patterns Summary

**Setter-Notify-Event chain** (all field mutations):
1. `FUN_0053a000` — entity live check
2. vtable call to get max value (for hull, squad)  **or**  `FUN_00500670` (for shield/weapon rate)
3. `FUN_0053fc90` — range validate
4. Write field only if `new_value != current_value`
5. `FUN_00539fd0(this, 1)` + notify dispatcher → side 1 observer
6. `FUN_00539fd0(this, 2)` + notify dispatcher → side 2 observer
7. vtable change-event slot → fires the registered listener chain

**Phase Gate pattern** (all 4 space combat phases):
- `FUN_0053a000` validity check
- `FUN_0053a640(phase_type_mask, side, &combat_phase_flags)` — gate evaluator
- If gate passes: call notify dispatcher for side 1 and side 2
- Then call vtable phase-handler slot

**Bilateral observer dispatch**: every state change notifies both `side=1` (attacker) and `side=2`
(defender) through their respective observer chains. The observer is retrieved via `FUN_00539fd0`
and is itself a polymorphic object whose dispatch method is a separate call (e.g., `FUN_005023b0`
for hull notifications, `FUN_00502430` for shield, `FUN_005024c0` for weapon, `FUN_005037f0` for
squad, `FUN_004f04f0` for regiment strength).
