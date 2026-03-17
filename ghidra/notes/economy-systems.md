---
title: "Economy & Star System Reference — REBEXE.EXE"
description: "Economy functions and star system management from Ghidra decompilation"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# Economy & Star System Reference — REBEXE.EXE

Generated from Ghidra decompilation of REBEXE.EXE (Star Wars Rebellion 1998, LucasArts).
Cross-referenced with: annotated-functions.md, FUN_0050e820_economy.c, FUN_00509*.c, FUN_00511*.c, FUN_00512*.c.

---

## 1. Resource System

### CStarSystem Resource Fields (Confirmed Offsets)

All four resource fields live on the `CStarSystem` object. The setter/getter pattern is identical for all four: `FUN_0053a000` validity check → `FUN_0053fc90` range validate → write field → notify two observer sides via `FUN_00539fd0(this, 1)` and `FUN_00539fd0(this, 2)` → vtable dispatch with old/new values.

| Offset (hex) | Offset (dec) | Field | Validator / Setter | Notif ID | Valid Range |
|---|---|---|---|---|---|
| `+0x5c` | 92 | `system_energy` | `FUN_00509cc0` | `0x141` (321) | `[0, param_1]` (non-negative, self-bounded) |
| `+0x60` | 96 | `system_energy_allocated` | `FUN_00509d40` | `0x142` (322) | `[0, param_1]` (non-negative, self-bounded) |
| `+0x64` / `100` | 100 | `system_raw_material` | `FUN_00509dc0` | `0x143` (323) | `[0, param_1]` (non-negative, self-bounded) |
| `+0x68` | 104 | `system_raw_material_allocated` | `FUN_00509e40` | `0x144` (324) | `[0, param_1]` (non-negative, self-bounded) |

**Note on offset 100**: Ghidra decodes `this + 100` as decimal (not hex). `100 decimal = 0x64 hex`. This is the `system_raw_material` field—do not confuse with the combat object `shield_weapon_packed` field (also at +0x64 on `CCombatUnit`). The two offsets share the same address only because `CStarSystem` and `CCombatUnit` are separate C++ classes within the `CNotifyObject` hierarchy.

### Additional System Stat Setters (FUN_00509ec0 – FUN_00509fc0)

These follow the identical setter pattern with different fields and ranges:

| Function | Offset | Field (inferred) | Valid Range | vtable slot |
|---|---|---|---|---|
| `FUN_00509ec0` | `+0x6c` | `production_modifier` | `[-100, 100]` (signed) | `+0x1e4` |
| `FUN_00509f40` | `+0x70` | `smuggling_percent` or `troop_withdraw_percent` | `[0, param_1]` | `+0x1e8` |
| `FUN_00509fc0` | `+0x74` | `troop_reg_surplus` or `troop_reg_required` | `[0, 100]` | `+0x1ec` |
| `FUN_00509c30` | `+0x58` | `base_loyalty` (system) | `[0, 100]` | `+0x1d0` |

`FUN_00509c30` validates an additional precondition: `*(uint*)(this + 0x88) & 1` must be set—a flag indicating the system is populated/colonized. Loyalty changes are gated on population.

### Notif Strings

Error strings embedded in the binary confirm field names precisely:

| Field | Error String |
|---|---|
| `system_energy` | `"Invalid SystemEnergy value!"` |
| `system_energy_allocated` | `"Invalid SystemEnergyAllocated value!"` |
| `system_raw_material` | `"Invalid SystemRawMaterial value!"` |
| `system_raw_material_allocated` | `"Invalid SystemRawMaterialAllocated value!"` |

### How Energy and Materials Flow

**FUN_0050e820** is the production eligibility checker (1,658 bytes). It orchestrates the full production readiness check for a system object. Decompiled logic:

1. **Initial validity**: calls `FUN_004f9700(param_1)` — system object validity check.
2. **Allegiance gate** (`param_1[0x14] & 8` == 0): calls three resource validators:
   - `FUN_0055bba0(param_1, faction_side, 4)` — energy validator
   - `FUN_0055b820(param_1, faction_side, 4)` — raw material validator
   - `FUN_0055b480(param_1, faction_side, 4)` — third resource validator (maintenance/refined)
   - All three must return 1 for production to proceed.
3. **Manufacturing facility check**: `FUN_0053fc90(param_1[0x18], 0, param_1[0x17])` — validates the manufacturing facility count/type pair at offsets `+0x17` (count) and `+0x18` (type).
4. **Production facility check**: `FUN_0053fc90(param_1[0x1a], 0, param_1[0x19])` — validates production facility pair at offsets `+0x19` / `+0x1a`.
5. **Entity type filter** (the large nested-range check): iterates the system's entity collection via `FUN_0052bed0` (get next entity from iterator). For each entity, checks `DatId >> 24` against a multi-range list of valid production-eligible family bytes: `0x20-0x2f` (troops/special forces), `0x30-0x3f` (ships), `0x1c-0x1f`, `0x08-0x0f` (characters), `0xa0-0xaf`, `0x40-0x7f`, `0x10-0x13`.
6. **Flight count check**: `FUN_005093e0(param_1, &local_108)` — gets the number of in-flight entities. Production is blocked if flight count >= 2.
7. **Unit readiness check**: four separate entity iterators enumerate capital ships (`FUN_00539d70`), special forces (`FUN_00558480`), characters (`FUN_0052c170`), and troops (`FUN_00536da0`). For each unit:
   - If `entity+0x50 >> 3 & 1` (fighter combat eligible flag) is set and `entity+0x50 >> 0xe & 1` is also set → skip.
   - Otherwise: `((byte)(entity[0x24]) ^ (byte)(param_1[9])) & 0xc0 == 0` — faction XOR check. Both must belong to the same faction (same control bits `0xc0`). Unit at offset `+0x78 >> 7 & 1` (special entity flag) provides an alternate path for character types.
8. **Fleet active check**: iterates fleets via `FUN_004ffe70(local_4c, param_1, 3)`. For each fleet: `entity+0x50 & 1` (active bit) must be set, then `param_1[0x22] >> 5 & 1` (a system flag) must equal `entity+0x58 >> 5 & 1` (fleet combat_phase_flags bit 5).

**Production flow summary**: Energy and raw materials are consumed by manufacturing facilities at a rate determined by the item being built and facility `processing_rate`. The validators in step 2 gate whether sufficient resources exist. `system_energy_allocated` and `system_raw_material_allocated` track how much of the production pool is committed to current build queues—preventing over-commitment.

---

## 2. Star System Control

### Control Kind Enum

Five distinct `ControlKind` states fire notifications. Based on notif string names and event IDs:

| ControlKind | Notif Function | Notif String | Event ID | Notes |
|---|---|---|---|---|
| `BattleWon` | `FUN_00511ec0` | `SystemControlKindBattleWonNotif` | none registered | Ownership transfer after combat win |
| `BattleWithdrew` | `FUN_00511f00` | `SystemControlKindBattleWithdrewNotif` | none registered | Attacker withdrew from combat |
| `Uprising` | `FUN_00511f40` | `SystemControlKindUprisingNotif` | `0x151` (337) | Loyalty-triggered control change |
| `BattleReady` | `FUN_00511f90` | `SystemControlKindBattleReadyNotif` | none registered | Forces assembled, awaiting combat |
| `BattlePending` | `FUN_005124c0` | `SystemBattlePendingNotif` | none registered | Battle scheduled but not yet begun |

All five use the same `FUN_0053a010` validity check pattern. `BattleWon` and `BattleWithdrew` and `BattleReady` use `FUN_004f88d0` (3-param notif dispatcher with old/new values and context). `Uprising` additionally registers event `0x151`, indicating it is a scripted-event-system trigger, not just a display notification. `BattlePending` and the three that follow use `FUN_004f8880` (1-entity param notif dispatcher).

**Faction control bits**: Extracted from `entity+0x24 bits 6-7` (`(byte >> 6) & 3`). This 2-bit value is the `faction_side` parameter passed to energy/resource validators. Values: 0 = neutral, 1 = Alliance, 2 = Empire, 3 = contested (inferred).

**Ownership transfer**: `param_1[9]` contains faction bits. The production eligibility check in `FUN_0050e820` uses `(entity[0x24] ^ param_1[9]) & 0xc0 == 0` to match units to the controlling faction—ownership is determined by matching the high 2 bits of the control byte at `+0x24`.

### LoyaltyCausedCurrentControlKind

**Function**: `FUN_00512500`
**Notif**: `SystemLoyaltyCausedCurrentControlKindNotif` / `LoyaltyCausedCurrentControlKind`
**Pattern**: `FUN_004f8880` — 1-entity param, no event ID registered.

This fires when system loyalty has drifted far enough to flip control. The function does not contain the threshold logic itself—it is the notification endpoint called by the loyalty system after thresholds are evaluated elsewhere. The implication: there is a periodic loyalty evaluation function (not yet decompiled) that monitors `system_base_loyalty` and triggers this notif when control changes.

### BattlePendingCausedCurrentBlockade

**Function**: `FUN_00512540`
File is currently empty (placeholder). The symbol appears in `rebellion-all-functions.txt`:
```
0x00512540  51  FUN_00512540 | BattlePendingCausedCurrentBlockade | (SystemBattlePendingCausedCurrentBlockade)
```
**Notif**: `SystemBattlePendingCausedCurrentBlockade`
**Pattern**: Same 51-byte size as other 1-entity notif dispatchers — confirmed `FUN_004f8880` pattern.

Interpretation: When a `BattlePending` state is set at a system (hostile fleet present, battle queued), it simultaneously triggers a blockade. This is the mechanistic link between fleet arrival and manufacturing shutdown.

### BlockadeAndBattlePendingManagementRequired

**Function**: `FUN_005126c0`
**Notif**: `SystemBlockadeAndBattlePendingManagementRequiredNotif` / `BlockadeAndBattlePendingManagementRequired`
**Pattern**: `FUN_004f8880` — 1-entity param.

Fires when both a blockade and a pending battle are active simultaneously. This is the combined-state notification: it tells the game manager that both conditions need handling in the same tick—i.e., manufacturing is blocked AND combat must be resolved before normal operations resume.

---

## 3. Uprising Mechanics

### UprisingIncident Trigger

**Function**: `FUN_00512580`
**Notif**: `SystemUprisingIncidentNotif` / `UprisingIncident`
**Event ID**: `0x152` (338)
**Pattern**: `FUN_0053fdd0(0x152, this, param_1, param_2)` — 1-extra-param event registration.

An "incident" is a precursor event to a full uprising. It fires when threshold conditions are met (low loyalty) but before the uprising is confirmed. The `FUN_0053fdd0` registration with event ID `0x152` means this is visible to the scripted event system—an `EventCondition` of type `EventFired(0x152)` can chain off it.

### Uprising Start (UPRIS1TB)

**UPRIS1TB.DAT**: 3 `IntTableEntry` records (16B each). Layout: `{ id: u32, field2: u32, threshold: i32, value: u32 }`.

- `threshold` (i32, signed) = loyalty delta relative to neutral. Negative = below average loyalty.
- `value` (u32) = probability of uprising starting (0–100).
- 3 entries means 3 loyalty thresholds, each mapping to a start probability.
- Interpretation: at very low loyalty (threshold deeply negative), the start probability approaches 100. At threshold = 0 (neutral loyalty), probability is minimal or zero.

**Function**: `FUN_005121e0`
**Notif**: `SystemUprisingNotif` / `Uprising`
**Event ID**: `0x14b` (331)
**Pattern**: `FUN_0053fdd0(0x14b, this, param_1, param_2)` — fires when an uprising begins.

### Uprising End / Subdue (UPRIS2TB)

**UPRIS2TB.DAT**: 4 `IntTableEntry` records. Same layout as UPRIS1TB but with 4 thresholds.

- `threshold` = loyalty level at time of subdue attempt.
- `value` = probability of successfully subduing the uprising.
- 4 entries: more granularity than start—subduing is harder at lower loyalty, easier if loyalty has recovered.
- Used by the Subdue Uprising mission (`SUBDMSTB.DAT`, 13 entries).

### System Loyalty Thresholds

From `FUN_00509c30` (base loyalty setter for systems):
- Valid range: `[0, 100]` (unsigned byte semantics despite being stored as int).
- Write gate: `*(uint*)(this + 0x88) & 1` must be set — system must be populated.
- Offset `+0x58` = system base loyalty field.

The 2-bit faction control field (`entity+0x24 bits 6-7`) is separate from the 0–100 loyalty value. Loyalty drives probability of uprising; when an uprising succeeds, `ControlKindUprising` fires, which may flip the control bits.

---

## 4. Blockade Mechanics

### Blockade Notification

**Function**: `FUN_005122d0`
**Notif**: `SystemBlockadeNotif` / `Blockade`
**Event ID**: `0x14e` (334)
**Pattern**: `FUN_0053fdd0(0x14e, this, param_1, param_2)` — fires when blockade state changes.

Also registered under `FleetBlockadeNotif` at `FUN_004ff7f0` (65 bytes), with the same `Blockade` event string at `006a947c`. This dual registration confirms blockade is a fleet-level event that propagates to the system level.

### Manufacturing Blocked During Blockade

From `FUN_0050e820` (production eligibility), step 8: the fleet active check reads `entity+0x50 & 1` (fleet active flag). When a hostile fleet is blockading, this flag state combined with the faction mismatch check (`entity+0x58 >> 5 & 1` vs `param_1[0x22] >> 5 & 1`) returns false, causing `uVar2 = 0` and aborting the production loop at `LAB_0050edcc`. Manufacturing halts for the blockaded system's controlling faction.

### TroopRegDestroyedRunningBlockade

**Function**: `FUN_00504a00`
**Notif**: `TroopRegDestroyedRunningBlockadeNotif` / `TroopRegDestroyedRunningBlockade`
**Event ID**: `0x340` (832)
**Pattern**: `FUN_004f8aa0` — 2-entity ref params + `FUN_0053fe40(0x340, ...)`.

```c
// Annotated pseudocode
int destroy_troop_in_blockade(CStarSystem* system, int troop_entity_id, int battle_ctx) {
    if (!is_valid_for_notification(system)) return 0;
    void* troop_transport = get_troop_transport(system);  // FUN_004f6b70
    if (!troop_transport || troop_entity_id == 0) return 0;
    void* troop_ptr = resolve_ref(troop_entity_id);       // FUN_004ece30
    dispatch_notification(system,
        "TroopRegDestroyedRunningBlockad",  // truncated
        "TroopRegDestroyedRunningBlockade",
        troop_transport, troop_ptr, null, battle_ctx);
    return register_event(0x340, system, &troop_transport, &troop_ptr, battle_ctx);
}
```

**Game rules**:
- Troop regiments being transported through a blockaded system are destroyed, not captured.
- The function fires once per regiment caught in transit.
- `FUN_004f6b70` retrieves the troop transport association on the system object—transport ships count as the "container" for the regiment during transit.
- Event `0x340` is visible to the scripted event system, enabling story events that react to blockade casualties.

### Blockade Resolution

Blockade ends when the hostile fleet is destroyed, withdraws, or the system changes hands. The `SystemBlockadeNotif` fires on state change (both enter and exit blockade). `BattlePendingCausedCurrentBlockade` fires on entry; blockade exit likely fires with a `param_1 = 0` to `FUN_005122d0` (false state).

---

## 5. Combat Unit Repair

### CombatUnitFastRepair

**Function**: `FUN_00512440`
**Notif**: `SystemCombatUnitFastRepairNotif` / `CombatUnitFastRepair`
**Pattern**: `FUN_004f8880` — 1-entity param, no event ID registered.

This is a state toggle notification: it fires when a combat unit at a system transitions into or out of "fast repair" mode. No event ID means it is not visible to the scripted event system—it is a UI/production feedback notification only.

### CombatUnitUnderRepair

Not found as a separate notif function. `CombatUnitFastRepair` is likely the only repair-state notif, covering both entry into and exit from fast-repair. The distinction between "fast" and normal repair is:
- **Normal repair**: happens automatically each turn at any system the unit is stationed at.
- **Fast repair**: triggered when specific GNPRTB conditions are met, accelerating recovery.

### Repair Rate and GNPRTB Parameters

`FUN_00511300` (FUN_00511300.c) is the production/repair eligibility checker. It calls:
- `FUN_0050c0b0`, `FUN_0050bbb0`, `FUN_0050c180`, `FUN_0050c1b0`, `FUN_0050c1e0` — five sequential checks, all must return non-zero.
- `FUN_0050b310` — sixth check.
- If all six pass AND `param_1 != 0` (repair mode active): additional manufacturing capacity checks via `FUN_0050ce20(this, 0, 1/2/3)` and `FUN_0050ce50(this, 0, 1/2/3)`.
- Final gate: `*(byte*)(this + 0x88) & 0x20` — a specific flag at offset `+0x88`, bit 5. This is likely the "has shipyard" or "repair dock present" flag.
- If shipyard present, `FUN_00509890` gets the GNPRTB repair parameter and `FUN_0050ce80` applies it.

**Repair-related GNPRTB parameters** (IDs in the 0x0a00 range, general parameter table):
- From `FUN_0053f9c0(0x305, ...)` and `FUN_0053fa60(0x386, ...)` called at the end of `FUN_00511300`: parameter `0x305` (773 decimal) and `0x386` (902 decimal) are repair or production rate modifiers. These are beyond the 26 documented general GNPRTB parameters—they belong to a higher ID range.

**Which units can be repaired**:
- Capital ships and fighters (family `0x30-0x3f`) — confirmed by the space combat section of the entity type filter.
- Death Star (family `0x34`) — included in the space-eligible set and the production range check.
- Troops (`0x14-0x1b`) cannot fast-repair (no shipyard path), only ground recovery.

---

## 6. System Notification Architecture

### Complete System Notif Handler Registry

All `CStarSystem` notification functions, with addresses, sizes, event IDs, and patterns:

#### Resource / Economy Setters (Pattern A: Validate + Set + Notify)

| Address | Size | Field | Offset | Notif String | Event ID | vtable slot |
|---|---|---|---|---|---|---|
| `FUN_00509c30` | — | `system_base_loyalty` | `+0x58` | — | — | `+0x1d0` |
| `FUN_00509cc0` | 117 | `system_energy` | `+0x5c` | `SystemSystemEnergyNotif` | `0x141` (321) | `+0x1d4` |
| `FUN_00509d40` | 117 | `system_energy_allocated` | `+0x60` | `SystemSystemEnergyAllocatedNotif` | `0x142` (322) | `+0x1d8` |
| `FUN_00509dc0` | 117 | `system_raw_material` | `+0x64` | `SystemSystemRawMaterialNotif` | `0x143` (323) | `+0x1dc` |
| `FUN_00509e40` | 117 | `system_raw_material_allocated` | `+0x68` | `SystemSystemRawMaterialAllocatedNotif` | `0x144` (324) | `+0x1e0` |
| `FUN_00509ec0` | — | `production_modifier` | `+0x6c` | — | — | `+0x1e4` |
| `FUN_00509f40` | — | field at `+0x70` | `+0x70` | — | — | `+0x1e8` |
| `FUN_00509fc0` | — | field at `+0x74` (0–100) | `+0x74` | — | — | `+0x1ec` |

#### Economy / Production Notif Dispatchers (Pattern B: Notify Only, 2-param)

These use `FUN_004f8980` (reversed param order: p2, p1) + `FUN_0053fcf0` (2-extra-param event registrar):

| Address | Size | Notif String | Event ID |
|---|---|---|---|
| `FUN_00511c90` | 73 | `SystemSystemEnergyNotif` / `SystemEnergy` | `0x141` (321) |
| `FUN_00511ce0` | 73 | `SystemSystemEnergyAllocatedNotif` / `SystemEnergyAllocated` | `0x142` (322) |
| `FUN_00511d30` | 73 | `SystemSystemRawMaterialNotif` / `SystemRawMaterial` | `0x143` (323) |
| `FUN_00511d80` | 73 | `SystemSystemRawMaterialAllocatedNotif` / `SystemRawMaterialAllocated` | `0x144` (324) |
| `FUN_00511dd0` | 73 | `SystemProductionModifierNotif` / `ProductionModifier` | *(not yet read)* |
| `FUN_00511e20` | 73 | `SystemSmugglingPercentNotif` / `SmugglingPercent` | *(not yet read)* |
| `FUN_00511e70` | 73 | `SystemTroopRegWithdrawPercentNotif` / `TroopRegWithdrawPercent` | *(not yet read)* |
| `FUN_00511fd0` | 73 | `SystemTroopRegSurplusNotif` / `TroopRegSurplus` | `0x148` (328) |
| `FUN_00512020` | 73 | `SystemTroopRegRequiredNotif` / `TroopRegRequired` | `0x156` (342) |

#### Control Kind Dispatchers (Pattern B, 3-param with old/new)

Use `FUN_004f88d0` (p2, p1, context with old/new value semantics):

| Address | Size | Notif String | Event ID |
|---|---|---|---|
| `FUN_00511ec0` | 56 | `SystemControlKindBattleWonNotif` / `ControlKindBattleWon` | — |
| `FUN_00511f00` | 56 | `SystemControlKindBattleWithdrewNotif` / `ControlKindBattleWithdrew` | — |
| `FUN_00511f40` | 73 | `SystemControlKindUprisingNotif` / `ControlKindUprising` | `0x151` (337) |
| `FUN_00511f90` | 56 | `SystemControlKindBattleReadyNotif` / `ControlKindBattleReady` | — |

#### State / Boolean Notif Dispatchers (Pattern B, 1-param)

Use `FUN_004f8880` (param1, null, context):

| Address | Size | Notif String | Event ID |
|---|---|---|---|
| `FUN_00512140` | 65 | `SystemPopulatedNotif` / `Populated` | `0x149` (329) |
| `FUN_00512190` | 65 | `SystemExploredNotif` / `Explored` | `0x14a` (330) |
| `FUN_005121e0` | 65 | `SystemUprisingNotif` / `Uprising` | `0x14b` (331) |
| `FUN_00512230` | 65 | `SystemNeverBeenControlledNotif` / `NeverBeenControlled` | `0x14c` (332) |
| `FUN_00512280` | 65 | `SystemBattleNotif` / `Battle` | `0x14d` (333) |
| `FUN_005122d0` | 65 | `SystemBlockadeNotif` / `Blockade` | `0x14e` (334) |
| `FUN_00512580` | 65 | `SystemUprisingIncidentNotif` / `UprisingIncident` | `0x152` (338) |
| `FUN_005125d0` | 65 | `SystemInformantIncidentNotif` / `InformantIncident` | `0x153` (339) |
| `FUN_00512620` | 65 | `SystemDisasterIncidentNotif` / `DisasterIncident` | `0x154` (340) |
| `FUN_00512670` | 65 | `SystemResourceIncidentNotif` / `ResourceIncident` | `0x155` (341) |

#### Combat / Assault Dispatchers (Pattern B, with faction param)

Use `FUN_004f8880` + `FUN_0053fcf0` with faction side extracted from `this+0x24 >> 6 & 3`:

| Address | Size | Notif String | Event ID |
|---|---|---|---|
| `FUN_00512320` | 75 | `SystemBombardNotif` / `Bombard` | `0x14f` (335) |
| `FUN_00512370` | 75 | `SystemAssaultNotif` / `Assault` | `0x150` (336) |

#### Garrison / Status Dispatchers (Pattern B, 1-param, no event ID)

| Address | Size | Notif String |
|---|---|---|
| `FUN_005123c0` | 51 | `SystemGarrisonedNotif` / `Garrisoned` |
| `FUN_00512400` | 51 | `SystemSuppressingNotif` / `Suppressing` |
| `FUN_00512440` | 51 | `SystemCombatUnitFastRepairNotif` / `CombatUnitFastRepair` |
| `FUN_00512480` | 51 | `SystemDeathStarNearbyNotif` / `DeathStarNearby` |
| `FUN_005124c0` | 51 | `SystemBattlePendingNotif` / `BattlePending` |
| `FUN_00512500` | 51 | `SystemLoyaltyCausedCurrentControlKindNotif` / `LoyaltyCausedCurrentControlKind` |
| `FUN_00512540` | 51 | `SystemBattlePendingCausedCurrentBlockadeNotif` / `BattlePendingCausedCurrentBlockade` |
| `FUN_005126c0` | 51 | `SystemBlockadeAndBattlePendingManagementRequiredNotif` / `BlockadeAndBattlePendingManagementRequired` |

#### Troop / Blockade Combat

| Address | Size | Notif String | Event ID |
|---|---|---|---|
| `FUN_00504a00` | 231 | `TroopRegDestroyedRunningBlockadeNotif` | `0x340` (832) |

### Complete Event ID Map (System Events)

Extends the existing table in `annotated-functions.md`:

| ID (hex) | ID (dec) | Event | Source Function |
|---|---|---|---|
| `0x141` | 321 | `SystemEnergy` | `FUN_00509cc0` / `FUN_00511c90` |
| `0x142` | 322 | `SystemEnergyAllocated` | `FUN_00509d40` / `FUN_00511ce0` |
| `0x143` | 323 | `SystemRawMaterial` | `FUN_00509dc0` / `FUN_00511d30` |
| `0x144` | 324 | `SystemRawMaterialAllocated` | `FUN_00509e40` / `FUN_00511d80` |
| `0x148` | 328 | `TroopRegSurplus` | `FUN_00511fd0` |
| `0x149` | 329 | `Populated` | `FUN_00512140` |
| `0x14a` | 330 | `Explored` | `FUN_00512190` |
| `0x14b` | 331 | `Uprising` | `FUN_005121e0` |
| `0x14c` | 332 | `NeverBeenControlled` | `FUN_00512230` |
| `0x14d` | 333 | `Battle` | `FUN_00512280` |
| `0x14e` | 334 | `Blockade` | `FUN_005122d0` |
| `0x14f` | 335 | `Bombard` | `FUN_00512320` |
| `0x150` | 336 | `Assault` | `FUN_00512370` |
| `0x151` | 337 | `ControlKindUprising` | `FUN_00511f40` |
| `0x152` | 338 | `UprisingIncident` | `FUN_00512580` |
| `0x153` | 339 | `InformantIncident` | `FUN_005125d0` |
| `0x154` | 340 | `DisasterIncident` | `FUN_00512620` |
| `0x155` | 341 | `ResourceIncident` | `FUN_00512670` |
| `0x156` | 342 | `TroopRegRequired` | `FUN_00512020` |
| `0x340` | 832 | `TroopRegDestroyedRunningBlockade` | `FUN_00504a00` |

IDs `0x145`, `0x146`, `0x147` are not yet identified (likely `ProductionModifier`, `SmugglingPercent`, `TroopRegWithdrawPercent` notif IDs — the functions `FUN_00511dd0`, `FUN_00511e20`, `FUN_00511e70` need their decompilations read to confirm).

### How System State Changes Propagate

The notification architecture follows a strict two-path pattern:

**Path 1 — Field mutation** (setter functions, e.g. `FUN_00509cc0`):
```
Caller sets new value
→ FUN_0053a000(this)              // validity: is object alive/initialized?
→ FUN_0053fc90(new_val, min, max) // range validate
→ write this+offset = new_val
→ FUN_00539fd0(this, 1)           // get observer list: faction side 1
→ per-type dispatch (e.g. FUN_0050f410) with side-1 observer
→ FUN_00539fd0(this, 2)           // get observer list: faction side 2
→ per-type dispatch with side-2 observer
→ vtable[slot](old_val, new_val, context) // fire change event
```

**Path 2 — Pure notification** (event dispatchers, e.g. `FUN_00511ec0`):
```
Game logic calls notif function directly
→ FUN_0053a010(this)              // validity: notification path check
→ FUN_004f8880/8980/8aa0(...)     // dispatch notification to subscribers
→ FUN_0053fcf0/fdd0/fe40(id, ...) // register with scripted event system
```

The scripted event system (`EventSystem::advance` in rebellion-core) listens on the event IDs registered by Path 2. An `EventCondition::EventFired(id)` can trigger any `EventAction` when a system state changes.

### CStarSystem Object Layout (Economy Fields)

Consolidated from all setter functions:

| Offset | Decimal | Type | Field |
|---|---|---|---|
| `+0x58` | 88 | `int` | `base_loyalty` (0–100, requires populated flag) |
| `+0x5c` | 92 | `int` | `system_energy` (≥ 0) |
| `+0x60` | 96 | `int` | `system_energy_allocated` (≥ 0) |
| `+0x64` | 100 | `int` | `system_raw_material` (≥ 0) |
| `+0x68` | 104 | `int` | `system_raw_material_allocated` (≥ 0) |
| `+0x6c` | 108 | `int` | `production_modifier` (–100 to +100) |
| `+0x70` | 112 | `int` | field (likely `smuggling_percent` or `troop_withdraw_percent`) |
| `+0x74` | 116 | `int` | field (likely `troop_reg_surplus` or `base_loyalty_modifier`, 0–100) |
| `+0x88` bit 0 | — | `bit` | `is_populated` (gate for loyalty changes) |
| `+0x88` bit 5 | — | `bit` | `has_repair_dock` / `has_shipyard` (gate for fast repair) |

**vtable entries for CStarSystem setters** (inferred from sequential layout):

| vtable Offset | Decimal | Purpose |
|---|---|---|
| `+0x1d0` | 464 | `on_loyalty_changed` |
| `+0x1d4` | 468 | `on_energy_changed` |
| `+0x1d8` | 472 | `on_energy_allocated_changed` |
| `+0x1dc` | 476 | `on_raw_material_changed` |
| `+0x1e0` | 480 | `on_raw_material_allocated_changed` |
| `+0x1e4` | 484 | `on_production_modifier_changed` |
| `+0x1e8` | 488 | `on_field_0x70_changed` |
| `+0x1ec` | 492 | `on_field_0x74_changed` |

---

## Implementation Notes for Open Rebellion

### Economy System (rebellion-core)

1. **`CStarSystem` in Rust**: Add `energy: i32`, `energy_allocated: i32`, `raw_material: i32`, `raw_material_allocated: i32`, `production_modifier: i32`, `base_loyalty: i32` to `world::System`. All are non-negative except `production_modifier` (–100 to +100) and `base_loyalty` (0–100).
2. **Production gate**: Implement `FUN_0050e820` logic as `System::is_production_eligible(world) -> bool`. Key checks: (a) energy/material validators pass; (b) in-flight unit count < 2; (c) all garrisoned units belong to the controlling faction; (d) no active hostile fleet blockade.
3. **Blockade → manufacturing halt**: When a system's `is_blockaded` flag is true, `ManufacturingSystem::advance` should skip that system's queue.
4. **Loyalty polling**: The loyalty-to-control-flip path goes through a periodic evaluation function not yet decompiled. Safe assumption: evaluate every game tick; if `base_loyalty < threshold` (from UPRIS1TB), fire `ControlKindUprising`.

### Star System Control (rebellion-core)

1. Add `ControlKind` enum: `BattleWon`, `BattleWithdrew`, `Uprising`, `BattleReady`, `BattlePending`.
2. `LoyaltyCausedCurrentControlKind` is a derived state—not stored separately. Fire it when the periodic loyalty eval detects a threshold crossing.
3. `BattlePending` and `Blockade` are co-entailed: setting one should set the other via `BattlePendingCausedCurrentBlockade`.

### Uprising System (rebellion-core)

1. **UPRIS1TB** (3 entries): `uprising_start_probability(loyalty: i32) -> u32`. Linear interpolation between thresholds.
2. **UPRIS2TB** (4 entries): `uprising_subdue_probability(loyalty: i32) -> u32`. Used by Subdue Uprising mission.
3. Uprising fires `0x14b` (Uprising) + `0x151` (ControlKindUprising) — both must be registered to the `EventSystem`.
4. `UprisingIncident` (`0x152`) should fire as a precursor warning, 1–2 turns before a full uprising starts, allowing the player to respond.

### Event ID Registration

Add these to `EventState` initial conditions:

```rust
// System economy events
const EV_SYSTEM_ENERGY: u32 = 0x141;
const EV_SYSTEM_ENERGY_ALLOCATED: u32 = 0x142;
const EV_SYSTEM_RAW_MATERIAL: u32 = 0x143;
const EV_SYSTEM_RAW_MATERIAL_ALLOCATED: u32 = 0x144;
const EV_TROOP_REG_SURPLUS: u32 = 0x148;
// System state events
const EV_POPULATED: u32 = 0x149;
const EV_EXPLORED: u32 = 0x14a;
const EV_UPRISING: u32 = 0x14b;
const EV_NEVER_BEEN_CONTROLLED: u32 = 0x14c;
const EV_BATTLE: u32 = 0x14d;
const EV_BLOCKADE: u32 = 0x14e;
const EV_BOMBARD: u32 = 0x14f;
const EV_ASSAULT: u32 = 0x150;
const EV_CONTROL_KIND_UPRISING: u32 = 0x151;
const EV_UPRISING_INCIDENT: u32 = 0x152;
const EV_INFORMANT_INCIDENT: u32 = 0x153;
const EV_DISASTER_INCIDENT: u32 = 0x154;
const EV_RESOURCE_INCIDENT: u32 = 0x155;
const EV_TROOP_REG_REQUIRED: u32 = 0x156;
// Blockade combat
const EV_TROOP_REG_DESTROYED_RUNNING_BLOCKADE: u32 = 0x340;
```
