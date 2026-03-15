# REBEXE.EXE Reverse Engineering Notes

## Binary Overview

| File | Size | .text | Functions | Content |
|------|------|-------|-----------|---------|
| **REBEXE.EXE** | 2.8MB | 2.4MB (0x401000-0x6573ff) | **22,741** | Main game executable — ALL game logic |
| COMMON.DLL | 2.9MB | TBD | TBD | Shared library (MFC/Win32 wrappers) |
| STRATEGY.DLL | 29MB | 9KB only | 43 (CRT) | **Resource-only** — strategy view sprites/art |
| TACTICAL.DLL | 7.8MB | TBD | TBD | Likely resource-only (tactical view sprites/art) |

**Key finding**: STRATEGY.DLL has almost zero code (9KB of CRT boilerplate, 29MB of .rsrc). All combat formulas, GNPRTB lookups, and game logic live in REBEXE.EXE.

## Ghidra Project

- **Project**: Open Rebellion Ghidra
- **Location**: `/Users/tomdimino/Desktop/Programming/open-rebellion/ghidra/`
- **Plugin**: GhidraMCP (LaurieWired v11.3.2) — REST API on `:8080`
- **Note**: Old plugin caps `/methods` and `/strings` at **99 results**. Use Jython scripts for full access.
- **Bethington fork** (v4.3.0, 176 endpoints) downloaded but requires Ghidra 12.0.3 (we have 11.3.2)
- **pyghidra-mcp**: Config fixed (`--project-path` instead of positional arg, `--force-analysis`, `--wait-for-analysis`)

## Architecture Discovery

The game uses a **Notification/Observer pattern** with virtual dispatch:
- Combat functions are called via vtable, not direct `CALL` instructions
- Xref caller trace returned zero direct callers for all combat setters
- String references (`*Notif` suffixes) are registered as notification handlers

### Battle State Machine
```
STRAT_INIT → STRAT → STRAT_ALL_BATTLE_START → STRAT_BATTLE_START
  → STRAT_BATTLE_CHOOSE_TYPE → STRAT_BATTLE_CONVERT_TO_TACT
  → STRAT_BATTLE_CONVERT_TO_TACT_ABSTRACT → STRAT_BATTLE_FINISH
  → STRAT_BATTLE_CHOOSE_DONE → STRAT_BATTLE_WAIT_FOR_DONE_CHOICE
  → STRAT_ALL_BATTLE_FINISH
```

Two combat paths: `CONVERT_TO_TACT` (tactical 2D view) and `CONVERT_TO_TACT_ABSTRACT` (auto-resolve).

## Confirmed Functions

### Combat Core (Capital Ships)

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_005029a0 | HullValueDamage | Hull damage setter | 16 |
| FUN_005029f0 | ShieldRechargeRateCHAllocated | Shield recharge rate setter | 16 |
| FUN_00502a40 | WeaponRechargeRateCHAllocated | Weapon recharge rate setter | 16 |
| FUN_005038e0 | SquadSizeDamage | Fighter squadron damage | 16 |
| FUN_00502020 | (near setters) | **Garrison/fleet strength aggregator** — iterates entities by DatId family_id | 897 |

### Fleet Events

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_004ff7a0 | FleetBattleNotif | Fleet battle trigger | 15 |
| FUN_004ff7f0 | FleetBlockadeNotif, Blockade | Blockade handler | 16 |
| FUN_004ff840 | FleetBombardNotif | Bombardment handler | 15 |

### System Control

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_00511ec0 | ControlKindBattleWon | System control change — battle won | 15 |
| FUN_00511f40 | ControlKindUprising | System control change — uprising | 16 |
| FUN_005121e0 | Uprising | Uprising handler | 16 |
| FUN_005122d0 | Blockade | Blockade system handler | — |
| FUN_00512440 | CombatUnitFastRepair | Unit repair system | 15 |

### Mission Destruction

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_004fc080 | DestroyedSabotage, DestroyedAssassination | Mission outcome handler | 89 |
| FUN_004fc3a0 | Damaged | Damage application | 16 |

**Mission type codes** (from FUN_004fc080 switch):
- `6` = Sabotage
- `7` = Assassination
- `0x15` (21) = Autoscrap

### Battle Orchestration

| Address | String | Purpose | Size |
|---------|--------|---------|------|
| FUN_0040a700 | STRAT_BATTLE_START | Battle state machine entry (switch on 18 states) | 182 |
| FUN_00532e00 | (near VictoryConditions at 0x533019) | Victory condition check | — |
| FUN_0054ba00 | FinalBattle | Luke vs Vader final battle | 16 |
| FUN_00536740 | BaseCombat | Base combat skill reference | 15 |

### Tactical Combat (FUN_005a7500, 572 lines)

This is the **tactical ship object constructor** — registers message handlers for the 2D tactical battle view:
- SHIP_ADD, SHIP_REMOVE, SHIP_ABSTRACT_DESTROY
- SHIP_FIRELASERCANNON, SHIP_FIRETURBOLASER, SHIP_FIREIONCANNON, SHIP_FIRETORPEDO
- SHIP_TAKE_LASER_HIT, SHIP_TAKE_TURBO_HIT, SHIP_TAKE_TORPEDO_HIT, SHIP_ION_DAMAGE
- SHIP_SHIELD_HIT, SHIP_SHIELD_FIX, SHIP_WEAPON_HIT, SHIP_WEAPON_DOWN
- SHIPGROUP_MOVE, SHIPGROUP_NEWMISSION, SHIPGROUP_NEWFORMATION, SHIPGROUP_NEWTARGETLIST
- NAV_ADD, NAV_REMOVE, NAV_PURGE, NAV_DELETE
- cmSHIP_POST_DS_STATUS (Death Star status)

### Largest Functions in Combat Region (0x4f0000-0x540000)

| Address | Size | Likely Purpose |
|---------|------|---------------|
| FUN_0050d5a0 | 3050B | Mission dispatch (switch on 8+ mission types) |
| FUN_0052d920 | 2894B | Unit description/display (TEXTSTRA string IDs) |
| FUN_00537180 | 2591B | TBD — high priority to decompile |
| FUN_004f4b60 | 1689B | TBD |
| FUN_0050e820 | 1658B | TBD |
| FUN_005385f0 | 1654B | TBD |
| FUN_00514a60 | 1622B | TBD |
| FUN_00508660 | 1523B | TBD |
| FUN_00516730 | 1440B | TBD |
| FUN_00519d00 | 1399B | TBD |

## Scripted Events (from strings)

### Jedi/Force System
- CharacterForceNotif / Force
- CharacterForceExperienceNotif / ForceExperience
- CharacterForceTrainingNotif / ForceTraining
- CharacterForcePotentialNotif / ForcePotential
- CharacterForceAwareNotif / ForceAware
- CharacterForceUserDiscoveredKeyNotif / ForceUserDiscovered
- CharacterDiscoveringForceUserNotif / DiscoveringForceUser

### Luke/Vader Saga
- MissionMgrLukeDagobahRequiredNotif → DagobahMissionFirstTrainingDayNotif
- MissionMgrDarthPickupNotif → DarthToLukeFinalBattle → DarthToEmperorFinalBattle
- FinalBattleReady → FinalBattle
- LukeFinalBattleReadyNotif

### Jabba's Palace
- MissionMgrLukePalaceNotif
- MissionMgrHanCapturedAtPalaceNotif
- MissionMgrLeiaPalaceNotif
- MissionMgrChewbaccaPalaceNotif

### Bounty Hunters
- MissionMgrBountyHuntersActiveNotif
- HanBountyAttackNotif / BountyAttack

### Character Enhancements
- EnhancedDiplomacy, EnhancedEspionage, EnhancedCombat, EnhancedLoyalty
- MissionHyperdriveModifier (Han Solo speed bonus)

### Victory
- SideVictoryConditionsNotif / VictoryConditions
- VICTORY, VICTORY_II (sound events)

## Ghidra Scripts

| Script | Purpose |
|--------|---------|
| FindAllFunctions.py | Scan .text for x86 prologues, create functions |
| DumpStrings.py | Search all strings by keyword, write to file |
| DumpCombatXrefs.py | Trace combat string→function cross-references |
| DumpCallers.py | Find direct callers of combat functions |
| DumpCombatRegion.py | List all functions in 0x4f0000-0x540000 |
| FindCombatMath.py | Search for combat math patterns |

All scripts at: `ghidra/scripts/`
Run via Jython: `exec(open("...path...").read())`

## Next RE Steps

1. **Find auto-resolve function** — the `CONVERT_TO_TACT_ABSTRACT` path resolves combat without tactical view. Search for functions that aggregate fleet firepower and compare attacker vs defender
2. **Decompile FUN_00537180** (2591 bytes) — third-largest, purpose unknown
3. **Decompile FUN_00502020** further — appears to iterate entities by DatId family_id byte (0x10-0x14=troops, 0x1c-0x20=facilities, 0x30-0x40=ships)
4. **Find GNPRTB table access** — search for constants 213, 0xD5, or stride-32 array indexing patterns
5. **Decompile FUN_00532e00** — near VictoryConditions reference, likely victory logic
6. **Trace the bombardment path** — FUN_004ff840 → what function calculates bombardment damage?
7. **Import COMMON.DLL** analysis — may contain shared combat utilities

## Data Addresses

| Address | Type | Content |
|---------|------|---------|
| 0x006be3b8 | Global pointer | Game state root object |
| 0x0066e660 | Data | Config data |
| 0x0061a414 | Code label | Exception handler setup |
| 0x0065d424 | Data | Referenced by unit display (DAT_0065d424) |

## Community References

- **Metasharp** (`~/Desktop/Programming/StarWarsRebellionEditor.NET/`): GNPRTB.cs has 61/213 parameter names
- **rebellion2** (`~/Desktop/Programming/rebellion2/`): C# Unity remake — strategy layer only, NO combat code
- **swrebellion.net**: Community forum with RebEd editor
- **Prima Strategy Guide**: Game mechanics documentation (approximate)
