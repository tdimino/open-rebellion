---
title: "REBEXE.EXE Overview and Early Function Index"
description: "Binary overview, the battle session's states, the notification setters found from strings, the largest functions near the object classes, scripted-event strings, and where each topic is now covered"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# REBEXE.EXE Overview and Early Function Index

## Binary Overview

| File | Size | Content |
|------|------|---------|
| **REBEXE.EXE** | 2.8MB, .text `0x401000`-`0x6573ff` | All game logic |
| STRATEGY.DLL | 29MB | Resource module: 9KB of CRT code, the rest `.rsrc` (strategy art) |
| TACTICAL.DLL | 7.8MB | Resource module 6: tactical art and sounds (`audio-dispatch.md`) |
| COMMON.DLL | 2.9MB | Resource module 8: main-menu sounds and shared resources (`audio-dispatch.md`) |

Every formula, GNPRTB lookup and rule lives in REBEXE.EXE. The per-range
atlas of the executable, with function counts, is `rebexe-map.md`.

## Ghidra Project

The project is `ghidra/Open Rebellion Ghidra`, run headless and read-only;
scripts are in `ghidra/scripts/` and listed in `ghidra/README.md`. Workflow:
`agent_docs/agent-tooling.md`.

## The Battle Session

The battle session `FUN_0040a050` is a state machine on `this+8`
(`simulate-runs-tactical-engine.md`): it offers each battle to both sides,
runs the tactical engine windowed (state 7) or headless (state 8), imports
the outcome (`FUN_0040c780`, states `0x16`/`0x17`) and rechecks the system
(state 9, `FUN_0051ddf0`). The `STRAT_*` and `TACT_*` strings
(`STRAT_BATTLE_CONVERT_TO_TACT`, `STRAT_BATTLE_CONVERT_TO_TACT_ABSTRACT`
and the rest) are debug names: `FUN_0040a700` builds a `CString` from them
that `FUN_0040a050` discards (`text-sources.md`).
`CONVERT_TO_TACT_ABSTRACT` is the headless run of the same tactical engine;
there is no separate auto-resolve. Where each kind of combat is resolved:
`COMBAT-SUMMARY.md`.

## Notification Setters Found from Strings

Each is a short setter that fires the named notification; see
`entity-system.md` for the event ids.

### Capital ships and squadrons

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_005029a0 | HullValueDamage | Hull damage setter | 16 |
| FUN_005029f0 | ShieldRechargeRateCHAllocated | Shield allocation setter | 16 |
| FUN_00502a40 | WeaponRechargeRateCHAllocated | Weapon allocation setter | 16 |
| FUN_005038e0 | SquadSizeDamage | Squadron damage notification (setter `FUN_005032c0`, `system-squadrons-combat.md`) | 16 |
| FUN_00502020 | (near setters) | Capital-ship consistency check: carried objects share the ship's side, hull within range (`modders-taxonomy.md`) | 897 |

### Fleets

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_004ff7a0 | FleetBattleNotif | Event `0x180` | 15 |
| FUN_004ff7f0 | FleetBlockadeNotif, Blockade | Event `0x181` | 16 |
| FUN_004ff840 | FleetBombardNotif | Event `0x182`; the bombardment run is `FUN_0058d800` (`bombardment.md`) | 15 |

### Systems

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_00511ec0 | ControlKindBattleWon | Control change after a battle | 15 |
| FUN_00511f40 | ControlKindUprising | Control change by uprising | 16 |
| FUN_005121e0 | Uprising | Uprising notification (`economy-systems.md`) | 16 |
| FUN_005122d0 | Blockade | System blockade, event `0x14e` | — |
| FUN_00512440 | CombatUnitFastRepair | Fast-repair notification | 15 |

### Destruction

| Address | Notif String | Purpose | Size |
|---------|-------------|---------|------|
| FUN_004fc080 | Destroyed, DestroyedSabotage, DestroyedAssassination, DestroyedAutoscrap, DestroyedOnArrival | Destroy notification; the variant follows the reason byte `+0x40 & 0xff` (`entity-system.md`) | 89 |
| FUN_004fc3a0 | Damaged | `GameObjDamagedNotif` | 16 |

Reason bytes read by `FUN_004fc080`: `6` sabotage, `7` assassination,
`0x15` autoscrap; any other reason fires only the generic event.

### Other

| Address | String | Purpose | Size |
|---------|--------|---------|------|
| FUN_0040a700 | STRAT_BATTLE_START and others | Battle-state debug name (see above) | 182 |
| FUN_005180e0 | SideVictoryConditionsNotif (sent by FUN_00532f90) | Victory flags (`victory-conditions.md`) | 891 |
| FUN_0054ba00 | FinalBattle | `MissionMgrFinalBattleNotif` (`story-encounters.md`) | 16 |
| FUN_00536740 | BaseCombat | `RoleBaseCombatNotif` setter | 15 |

### Tactical battle manager (`FUN_005a7500`, 572 lines)

The tactical battle manager constructor registers the 2D battle's message
handlers:

- SHIP_ADD, SHIP_REMOVE, SHIP_ABSTRACT_DESTROY
- SHIP_FIRELASERCANNON, SHIP_FIRETURBOLASER, SHIP_FIREIONCANNON, SHIP_FIRETORPEDO
- SHIP_TAKE_LASER_HIT, SHIP_TAKE_TURBO_HIT, SHIP_TAKE_TORPEDO_HIT, SHIP_ION_DAMAGE
- SHIP_SHIELD_HIT, SHIP_SHIELD_FIX, SHIP_WEAPON_HIT, SHIP_WEAPON_DOWN
- SHIPGROUP_MOVE, SHIPGROUP_NEWMISSION, SHIPGROUP_NEWFORMATION, SHIPGROUP_NEWTARGETLIST
- NAV_ADD, NAV_REMOVE, NAV_PURGE, NAV_DELETE
- cmSHIP_POST_DS_STATUS (Death Star status)

The engine's rules are in the `tactical-*.md` notes.

## Largest Functions in `0x4f0000`-`0x540000`

| Address | Size | Purpose | Note |
|---------|------|---------|------|
| FUN_0050d5a0 | 3050B | Observation level 0-12 for one side | `observation-levels.md` |
| FUN_0052d920 | 2894B | References TEXTSTRA string ids; not traced | — |
| FUN_00537180 | 2591B | Per-system object loop: families `[0xa0,0xa2)`, then systems `[0x90,0x98)` | `galaxy-limits.md` |
| FUN_004f4b60 | 1689B | Mission command validator (slot `+0x50`) | `ai-mission-planning.md` |
| FUN_0050e820 | 1658B | System invariant check: members share the system's side | `facility-ownership.md` |
| FUN_005385f0 | 1654B | Build order `0x211`: product family and category checks | `death-star-strategic.md` |
| FUN_00514a60 | 1622B | Object-move dispatcher (event `0x320`) | `COMBAT-SUMMARY.md` |
| FUN_00508660 | 1523B | System child hook (slot `+0xd0`), by family | `ground-unit-assets.md`, `blockade-troop-withdrawal.md` |
| FUN_00516730 | 1440B | A battle's retreat options (`FUN_005545e0`) | `galaxy-limits.md`, `build-delivery.md` |
| FUN_00519d00 | 1399B | New-game step `0xb`: system allocation by side | `scenario-literals.md`, `galaxy-limits.md` |

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

The MissionMgr checks behind these strings are in `story-missions.md`,
`story-encounters.md` and `space-combat.md`.

## Open

- `FUN_0052d920`: what it builds from TEXTSTRA ids
  (`or-q-encyclopedia-topic-art-ddn8` covers the encyclopedia side).
- Families `0xa0`-`0xa5` walked by `FUN_00537180`
  (`or-q-families-a0-a5-8ctv`).

## Data Addresses

| Address | Type | Content |
|---------|------|---------|
| `0x006b2bb0` | Global pointer | Game root object (type `0xf1`) |
| `0x006be3b8` | Global pointer | GameOptions singleton (`rebexe-map.md`) |
| `0x0066e660` | Data | SEH scope table of the CRT entry (`entry.c`) |
| `0x0061a414` | Code | CRT exception handler installed by the entry (`entry.c`) |
| `0x0065d424` | Data | String-key group constant 2 (`text-sources.md`) |

## Community References

- **Metasharp** (`~/Desktop/Programming/StarWarsRebellionEditor.NET/`): GNPRTB.cs has 61/213 parameter names
- **rebellion2** (`~/Desktop/Programming/rebellion2/`): C# Unity remake — strategy layer only, no combat code
- **swrebellion.net**: Community forum with RebEd editor
- **Prima Strategy Guide**: Game mechanics documentation (approximate)
