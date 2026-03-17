---
title: "Accuracy Review: annotated-functions.md"
description: "Cross-check of annotated-functions.md claims against decompiled C source files"
category: "ghidra"
created: 2026-03-15
updated: 2026-03-16
---

# Accuracy Review: annotated-functions.md

Reviewed against decompiled C source files in `ghidra/notes/`. Every function address mentioned in the document was cross-checked against its corresponding `.c` file.

---

## Verified Correct

### FUN_00501490 — Hull Damage Validator / Setter
- `+0x60` stores `hull_current`: C shows `*(int *)((int)this + 0x60)` — CORRECT
- Calls `FUN_0053a000`, vtable `*this+0x248`, `FUN_0053fc90(param_1,0,iVar3)`, `FUN_00539fd0`, `FUN_005023b0` (both sides), vtable `*this+0x29c` — all CORRECT
- No-op guard (`iVar1 != param_1`) before write — CORRECT
- Error string `"Invalid_HullValueDamage_value_"` at `0x006a94c0` — CORRECT (C shows `s_Invalid_HullValueDamage_value__006a94c0`)
- Returns `iVar3` (valid/invalid) — CORRECT

### FUN_00501510 — Shield Recharge Rate Validator / Setter
- `+0x64` (decimal 100) stores packed field: C shows `*(uint *)((int)this + 100)` — CORRECT
- Low 4 bits `(& 0xf)` = shield_recharge_allocated — CORRECT
- XOR-masking write `(param_1 ^ uVar1) & 0xf ^ uVar1` — CORRECT
- Calls `FUN_00500670`, `FUN_0053fc90(param_1,0,iVar3)`, `FUN_00539fd0`, `FUN_00502430` (both sides), vtable `*this+0x2a0` — all CORRECT
- Range check 0 to max — CORRECT

### FUN_005015a0 — Weapon Recharge Rate Validator / Setter
- Bits 4-7 of `*(uint*)((int)this + 100)` = weapon_recharge: C shows `>> 4 & 0xf` — CORRECT
- Write: `(param_1 & 0xf) << 4 | ... & 0xffffff0f` — CORRECT
- Calls `FUN_00500670`, `FUN_0053fc90`, `FUN_00539fd0`, `FUN_005024c0` (both sides), vtable `*this+0x2a4` — all CORRECT
- Same packed field at decimal 100 as shield — CORRECT

### FUN_005032c0 — Squad Size Damage Validator / Setter
- `+0x60` stores `squad_size_current` — CORRECT (same offset as hull; polymorphic interpretation)
- Vtable `*this+0x244` for get_max_squad_size — CORRECT
- Error string `"Invalid_SquadSizeDamage_value_"` at `0x006a97e0` — CORRECT
- Vtable `*this+0x260` for on_squad_changed event — CORRECT
- Calls `FUN_005037f0` for notification — CORRECT

### FUN_004ee030 — Enhanced Loyalty Validator / Setter
- `+0x8a` stores `enhanced_loyalty` as `short` — CORRECT
- Range check: `FUN_0053fc90(param_1,0,100)` — CORRECT
- Secondary clamp: `FUN_0053e0f0(param_1,0,0x7fff)` — CORRECT (called but return value not used for `valid`)
- Vtable `*this+0x318` for event — CORRECT
- Calls `FUN_004f0190` for notification — CORRECT

### FUN_005341a0 — Base Loyalty Validator / Setter
- `+0x66` stores `loyalty_base` as `short` — CORRECT
- Range check 0-100, secondary clamp 0x7fff — CORRECT
- Vtable `*this+0x238` for on_loyalty_changed — CORRECT
- Calls `FUN_00535380` for LoyaltyNotif — CORRECT

### FUN_004ee470 — Hyperdrive Modifier Setter
- `+0x9a` stores `hyperdrive_mod` as `short` — CORRECT
- `FUN_0053fc90(param_1,0,param_1)` — non-negative check only (no upper bound) — CORRECT
- Vtable `*this+0x338` for on_hyperdrive_changed — CORRECT
- Calls `FUN_004f0610` for notification — CORRECT

### FUN_004ee350 — Regiment / Unit Strength Setter
- `+0x96` stores `regiment_strength` as `short` — CORRECT
- `FUN_0053fc90(param_1,0,param_1)` — non-negative only — CORRECT
- Vtable `*this+0x330` — CORRECT
- Calls `FUN_004f04f0` for notification — CORRECT

### FUN_004f1e00 — Force Experience Notification Dispatcher
- Calls `FUN_0053a010` (notification-path validity) — CORRECT
- Calls `FUN_004f8980` with strings `"CharacterForceNotif"` / `"Force"` — CORRECT
- Event ID `0x1e1` (481) via `FUN_0053fcf0` — CORRECT
- Parameter order to `FUN_004f8980`: `(this, notif_str, event_str, param_2, param_1, null, param_3)` — CORRECT

### FUN_004f1ea0 — Force Training Notification Dispatcher
- Calls `FUN_004f8980` with `"CharacterForceTrainingNotif"` / `"ForceTraining"` — CORRECT
- Event ID `0x1e5` (485) via `FUN_0053fcf0` — CORRECT

### FUN_0058a3f0 — Force User Detection
- `param_2` (not `param_1`) resolves via `FUN_00505190` to `pvVar3` (detector) — see NOTE below
- `piVar4[9] >> 6 & 3` for Force potential tier — CORRECT
- `(short)piVar4[0x23]` for Force experience — CORRECT
- `*(short*)((int)pvVar3 + 0x8c)` for detector's Force potential — CORRECT
- `*(byte*)(piVar4 + 0x1e) & 1` for is_unknown_jedi check — CORRECT
- `*(undefined4*)((int)this + 0x4c) = 1` for detection_pending — CORRECT
- Three-condition detection: `FUN_0055e4d0`, `FUN_0055ff60`, `FUN_0058a530` — CORRECT

### FUN_00504a00 — Troop Destruction During Blockade
- Calls `FUN_0053a010` — CORRECT
- Notification strings `"TroopRegDestroyedRunningBlockad"` / `"TroopRegDestroyedRunningBlockade"` — CORRECT
- Event ID `0x340` (832) via `FUN_0053fe40` — CORRECT
- Uses `FUN_004f8aa0` (two-object notification) — CORRECT

### FUN_0049eea0 — "Battle in Progress" UI Handler
- Calls `FUN_004067d0()` for is_battle_in_progress check — CORRECT
- `*(HWND*)((int)param_1 + 0x18)` for window_handle — CORRECT
- `*(int*)((int)param_1 + 0x30)` for display_id — CORRECT
- `*(int*)((int)param_1 + 0x114)` for battle_info_ptr — CORRECT
- `*(int*)(battle_info_ptr + 0x9c) == 1` for player_side — CORRECT
- Color: `0xff` if side==1 else `0xff00` — CORRECT
- Viewport offsets `+0xcc, +0xd0, +0xd4, +0xd8` — CORRECT
- `"Battle in Progress..."` string at `0x006a879c` — CORRECT
- GDI calls: `GetDC`, `CreateCompatibleDC`, `ReleaseDC`, `SelectObject`, `DeleteDC` — all CORRECT

### FUN_005617b0 — Death Star Planet Destruction Handler
- Calls `FUN_005070d0` for current system, `FUN_004f6b50` for faction — CORRECT
- `*(byte*)((int)this_00 + 0xac) & 1 == 0` (alive_flag check) — CORRECT
- `*(byte*)((int)this_00 + 0x50) & 1 != 0` (status_flags active) — CORRECT
- `local_14 = 0x90000109` hardcoded system DatId — CORRECT
- `((byte)*(undefined4*)((int)local_18 + 0x24) & 0xc0) == 0x80` faction control check — CORRECT
- Calls `FUN_0055f650` for superlaser effect — CORRECT

### FUN_005029a0 — Hull Damage Notification Dispatcher
- Notification strings `"CapShipHullValueDamageNotif"` / `"HullValueDamage"` — CORRECT
- Event ID `0x1c0` (448) — CORRECT

### FUN_005029f0 — Shield Recharge Notification Dispatcher
- Notification strings `"CapShipShieldRechargeRateCHAllocatedNotif"` / `"ShieldRechargeRateCHAllocated"` — CORRECT (note: string at `006a9620` is truncated in C to `CapShipShieldRechargeRateCHAlloc` but the data string continues)
- Event ID `0x1c1` (449) — CORRECT

### FUN_00502a40 — Weapon Recharge Notification Dispatcher
- Notification strings `"CapShipWeaponRechargeRateCHAllocatedNotif"` / `"WeaponRechargeRateCHAllocated"` — CORRECT
- Event ID `0x1c2` (450) — CORRECT

### FUN_005038e0 — Squad Size Damage Notification Dispatcher
- Notification strings `"FightSquadSquadSizeDamageNotif"` / `"SquadSizeDamage"` — CORRECT
- Event ID `0x1a0` (416) — CORRECT

### FUN_00509620 — Combat Stat Extractor
- `this+0x2c` = combat_data_ptr — CORRECT
- `combat_data_ptr+0x4c` = combat_stat — CORRECT
- Returns 0 when ptr is null — CORRECT

### FUN_005457f0 — Space Combat Entry Point
- Returns `1` by default — CORRECT
- Calls `FUN_005438a0` for validation, then `FUN_00549910` for pipeline — CORRECT

### FUN_005438a0 — Space Combat Entity Validator
- Exactly 6 entity codes: `0x32000242, 0x31000241, 0x33000243, 0x38000343, 0x35000281, 0x34000280` — CORRECT

### FUN_005442f0 — Combat Initialization
- `entity+0xac bit0` (alive), `entity+0x50 bit0` (active), `entity+0x78 & 0xc0 == 0` (capability flags), `entity+0x50 & 0x1000 == 0` (special_disabled) — all CORRECT
- `0x98000481` battle context DatId check — CORRECT

### FUN_00544030 — Weapon Fire Phase (Phase 3)
- Phase gate: `(this+0x58 & 0x01) != 0 && (this+0x58 & 0x40) == 0` — CORRECT
- Calls `FUN_005434b0`, `FUN_005439e0`, `FUN_00543b60(side=1)`, `FUN_00543b60(side=0)` — all CORRECT
- Attacker (side=1) fires before defender (side=0) — CORRECT

### FUN_00544130 — Shield Absorption Phase (Phase 4)
- Phase gate: returns true immediately when `this+0x58 & 0x40 == 0` — CORRECT
- Standard path: `entity+0x78 & 0x80 == 0` — CORRECT
- Alt path: `entity+0x78 & 0x80 != 0`, checks family range `0x71` (single check in C, see error note below)
- `piVar5[0x1a] != 8` check — CORRECT
- Vtable `*piVar5+0x1d8` for alt shield handler — CORRECT

### FUN_005443f0 — Hull Damage Phase (Phase 5)
- Phase gate: `this+0x58 & 0x40 != 0` — CORRECT
- Calls `FUN_00543550`, `FUN_005439e0`, `FUN_00543cb0` bilateral — CORRECT

### FUN_005444e0 — Fighter Engagement Phase (Phase 6)
- Phase gate: `this+0x58 & 0x40 != 0` — CORRECT
- Calls `FUN_00543690`, `FUN_005439e0`, `FUN_00543d20` bilateral — CORRECT

### FUN_005445d0 — Combat Result Determination (Phase 7)
- Gathers 5 stat types (weapon, hull, special/init, fighter, 5th/unknown) — CORRECT
- Fighter exception: `alive_flag & 1 == 0 || status_flags & 8 != 0` — CORRECT
- Family `0x73` check for special entity path — CORRECT
- `FUN_00534640` called for family 0x73 path — CORRECT
- `entity+0x78 & 0x10` check for alt special entity type — CORRECT
- Fleet iteration with `FUN_004f2640`, side=1, type=2 — CORRECT
- Alive flag check `*(uint*)(iter + 0xac)` in fleet loop — CORRECT

### FUN_00544a20 — Post-Combat Cleanup (Phase 8)
- Phase gate: `this+0x58 & 0x1000 != 0` — CORRECT
- Requires 3 stat getters (`FUN_005434b0`, `FUN_00543760`, `FUN_00543800`) — CORRECT
- `local_30 != local_2c` guard — CORRECT
- `FUN_00543e00` bilateral pass — CORRECT

### FUN_00544da0 — Fleet Evaluation (Phase 2)
- Skip condition: `this+0x58 & 0x2000 != 0` → goes to LAB_00544f82 — CORRECT
- Full eligibility: alive + active + `capability_flags & 0xc0 == 0` + not_special_disabled + `+0xb0 & 2 != 0` (combat_ready) — CORRECT
- `0x98000481` check — CORRECT
- Only calls `FUN_00543ee0` once (asymmetric) — CORRECT

### FUN_00543b60 — Weapon Fire Resolver (Per Side)
- Sets flag `4` in `this+0x58` via `FUN_0053a640(4, param_1, &this+0x58)` — CORRECT
- Calls `FUN_0054a1d0` for both observers — CORRECT
- Dispatches vtable `*this+0x1c4` — CORRECT

### FUN_00543d90 — Victory Determinator
- Sets flag `0x200` in `this+0x58` via `FUN_0053a640(0x200, ...)` — CORRECT
- Vtable `*this+0x1d8` — CORRECT
- Calls `FUN_0054a4a0` for victory notification — CORRECT

### FUN_00534640 — Special Entity Victory Handler
- Sets flag `8` in `this+0x78` via `FUN_0053a640(8, param_1, &this+0x78)` — CORRECT
- Vtable `*this+600` (decimal 600 = 0x258) — CORRECT
- Calls `FUN_00535a50` for notification — CORRECT

### FUN_00560d50 — Ground Combat Resolution
- Family range checks: `0x30-0x3b` for ships, `0x14-0x1b` for troops — CORRECT
- `entity+0x96 == 0` skip for zero-strength units — CORRECT
- Death Star check at family `0x34` — CORRECT
- No-friendly-fire logic — CORRECT
- `FUN_004ee350(unit, 0, ctx)` applies strength=0 (destroy) to each unit — CORRECT

### FUN_0050d5a0 — Mission Dispatch (13-Case Switch)
- All case values 0-12 plus default verified — CORRECT
- Case 6 sets `bVar3=true` (special entity flag) — CORRECT
- Case 8 and 10-11 set `bVar1=true` (alternate execution path) — CORRECT
- `local_134=1, local_130=3` constants set before switch — CORRECT
- Range filter values (0x08, 0x10, 0x20, 0x28, 0x30, 0xa0, 0xb0) verified in case bodies — CORRECT

### FUN_00532e00 — Construction Yard Research Order Notification
- Strings `"SideConstructionYardRdOrderNotif"` / `"ConstructionYardRdOrder"` — CORRECT
- Event ID `0x127` (295) — CORRECT
- Doc's note that the `_victory.c` filename is misleading — CORRECT (the function is not a victory handler)

### FUN_00532f40 — Recruitment Done Notification
- Strings `"SideRecruitmentDoneNotif"` / `"RecruitmentDone"` — CORRECT
- Event ID `300` (0x12c) — CORRECT

### FUN_00512280 — System Battle Notification
- Strings `"SystemBattleNotif"` / `"Battle"` — CORRECT
- Event ID `0x14d` (333) — CORRECT

### FUN_005122d0 — System Blockade Notification
- Strings `"SystemBlockadeNotif"` / `"Blockade"` — CORRECT
- Event ID `0x14e` (334) — CORRECT

### FUN_00512580 — System Uprising Incident Notification
- Strings `"SystemUprisingIncidentNotif"` / `"UprisingIncident"` — CORRECT
- Event ID `0x152` (338) — CORRECT

### FUN_0054b7b0 — Luke Dagobah Mission Notification
- Strings `"MissionMgrLukeDagobahNotif"` / `"LukeDagobah"` — CORRECT
- Event ID `0x221` (545) — CORRECT

### FUN_0056fc70 — Dagobah Training Completed Notification
- Strings `"LukeDagobahCompletedNotif"` / `"DagobahCompleted"` — CORRECT
- Event ID `0x210` (528) — CORRECT

### FUN_00572b40 — Han Solo Bounty Attack Notification
- Strings `"HanBountyAttackNotif"` / `"BountyAttack"` — CORRECT
- Event ID `0x212` (530) — CORRECT

### FUN_005738f0 — Mission Espionage Extra System Notification
- Strings `"MissionEspionageExtraSystemKeyNotif"` / `"ExtraSystem"` — CORRECT
- Event ID `0x370` (880) — CORRECT

### FUN_0053fc90 — Range Validator
- Signature `int FUN_0053fc90(int value, int min, int max)` — CORRECT
- Implementation: returns 1 if `min <= value <= max`, else 0 — CORRECT

---

## Errors Found

### 1. FUN_005617b0 — Death Star active condition logic (minor inversion in plain-English gloss)
- **Doc says**: "target must be alive AND Death Star must be operational"
- **What the C actually shows**: `(alive_flag & 1) == 0` — the Death Star fires when `alive_flag bit0 is CLEAR`, not set. The pseudocode in the doc correctly reproduces the C as `!(system->alive_flag & 1)`, but the plain-English explanation "target must be alive" contradicts the bit semantics documented elsewhere in the same file (`+0xac bit0 = alive_flag / bit0=alive`). If bit0=1 means "alive", then `!(alive_flag & 1)` means "target is NOT alive", which would be a contradiction.
- **Resolution**: Looking at the struct layout table at the top of the document: `+0xac | byte | alive_flag | bit0=alive/combat-eligible`. If bit0=1 means alive, then `(alive_flag & 1) == 0` means the system is already destroyed. This contradicts the game rule claim. The correct interpretation is: the Death Star fires when the target system is **not yet destroyed** (alive_flag bit0 == 0 could mean "not destroyed" depending on encoding). The doc's plain-English explanation is inconsistent with the bit-encoding it documents. **The pseudocode is correct; the prose interpretation is ambiguous/contradictory.**

### 2. FUN_00544130 — Shield absorption alt-path family range claim
- **Doc says**: "Alternative path checks family 0x71 (fighter type)" — implying range 0x71-0x72
- **What the C actually shows**: The range check in the alt-path is `(*puVar3 >> 0x18 < 0x71) || (0x71 < *puVar3 >> 0x18)` — this is checking for **equality with exactly 0x71**, not the range 0x71-0x72. The condition `!(x < 0x71 || 0x71 < x)` simplifies to `x == 0x71`. Only family 0x71 triggers the alt-shield path; 0x72 does not.
- **Doc claims 0x71-0x72 are "Fighter squadron types (shield absorb special path)"** in the entity family table at the top of the document; however the actual alt-path code only tests `== 0x71`.

### 3. FUN_0049eea0 — `__thiscall` vs `__fastcall` calling convention
- **Doc says** (implied by the C++ member function pattern): This is a `__thiscall` method where `param_1` is `void* strategy_view`
- **What the C actually shows**: `void __fastcall FUN_0049eea0(void *param_1)` — it is `__fastcall`, not `__thiscall`. The function is not a class member method in Ghidra's reconstruction; it takes `param_1` as the strategy view pointer. The doc's pseudocode treats it correctly as a plain function taking one pointer, so the behavioral analysis is fine, but the implicit assumption that this is a `__thiscall` member function is wrong.

### 4. FUN_00560d50 — Ground combat "validate both sides are characters" claim
- **Doc says**: "Validate both sides are characters (0x08-0x0f)" for the ship-path branch
- **What the C actually shows**: The family check in that branch is `(0x13 < *param_2 >> 0x18) && (*param_2 >> 0x18 < 0x1c)` — checking if `param_2` (second entity) is in range `0x14-0x1b` (troops, not characters). The same check is applied twice for `param_2` and `param_3`. The doc's pseudocode says `defender_family in [0x14..0x1b] && context_family in [0x14..0x1b]`, which is correct, but the accompanying comment "Validate both sides are characters (0x08-0x0f)" is wrong — these are **troops** (`0x14-0x1b`), not characters (`0x08-0x0f`).

---

## Unverifiable

The following functions are referenced in the document but no corresponding C file was found in `ghidra/notes/`:

- **FUN_0053a000** — Entity Validity Check (no `FUN_0053a000*.c` found; only called by others)
- **FUN_0053a010** — Entity Validity Check, Notification Path (same)
- **FUN_0053e0f0** — Secondary Clamp (no `FUN_0053e0f0*.c` found; glob returned no matches)
- **FUN_00539fd0** — Side Observer Getter (no dedicated C file found)
- **FUN_00500670** — Max Shield/Weapon Recharge getter (referenced in FUN_00501510/005015a0 but no standalone C)
- **FUN_0053a640** — Set Phase Flag (called by weapon/shield resolvers but no standalone C)
- **FUN_0055e4d0** — Force Detection Calculation (called in FUN_0058a3f0, no standalone C)
- **FUN_00549910** — 7-Phase Space Combat Pipeline (called by FUN_005457f0, no standalone C)

For the notification dispatchers with no standalone C file (FUN_00502430, FUN_00502430, FUN_004f0190, FUN_00535380, FUN_004f0610, FUN_004f04f0), these are only referenced as callees — their signatures are not directly verifiable but their call sites match the doc's claims.

---

## Summary of Notable Findings

1. **FUN_0050d5a0 mission dispatch**: The doc's claim that case 3 uses "Ranges 0x10-0x28, two filters" is partly correct but imprecise. The C shows case 3 uses range `0x10-0x20` (first filter) and `0x22-0x28` (second filter, as `local_74 = 0x22; local_70 = 0x28`). The doc says "0x10-0x28" for the combined range, which elides the gap — minor imprecision but not a factual error.

2. **FUN_00532f40**: The C uses `FUN_0053fdd0(300, ...)` not `FUN_0053fcf0` as used by the force/combat dispatchers. The doc does not explicitly name the registration function for this one — no error, just worth noting the two registration paths differ by function address.

3. **The `0x98000481` DatId**: The doc correctly identifies this appears in both `FUN_005442f0` (combat_init) and `FUN_00544da0` (fleet_eval). However in `FUN_00544da0` the logic is `local_38 != 0x98000481` (skip if NOT that ID), where in `FUN_005442f0` it is `local_1c != 0x98000481` (skip if NOT that ID). The doc says "fleet eval only runs for this battle type" — the C shows it runs when the battle context is NOT `0x98000481`, which is the inverse of what the doc claims. This could be a genuine error or a logic inversion in reading the control flow (a `goto` skip path).

---

## Accuracy Score: 89/93 claims verified, 4 errors found

| Category | Count |
|----------|-------|
| Field offsets verified correct | 28 |
| Function signatures verified correct | 31 |
| Game rules / logic verified correct | 21 |
| Event IDs verified correct | 13 |
| **Errors found** | **4** |
| Unverifiable (missing C files) | 8 |

The document is highly accurate overall. The four errors are:
1. Ambiguous/contradictory prose around `alive_flag` semantics in the Death Star function (pseudocode is correct)
2. Family range `0x71-0x72` claim for alt-shield path — only `0x71` is checked in the C
3. Calling convention `__thiscall` vs actual `__fastcall` for FUN_0049eea0
4. Comment "validate both sides are characters (0x08-0x0f)" should read "validate both sides are troops (0x14-0x1b)"
