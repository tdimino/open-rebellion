---
title: "Accuracy Review: annotated-functions.md"
description: "Cross-check of annotated-functions.md claims against decompiled C source files"
category: "ghidra"
created: 2026-03-15
updated: 2026-10-14
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

### FUN_004ee350 — Setter for Short +0x96
- `+0x96` is a `short` written on a master copy — CORRECT
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

### FUN_0049eea0 — Battle Overlay Window Content
- Calls `FUN_004067d0()` (the TEXTSTRA edition flag) to choose the label — CORRECT
- `*(HWND*)((int)param_1 + 0x18)` for window_handle — CORRECT
- `*(int*)((int)param_1 + 0x30)` for display_id — CORRECT
- `*(int*)((int)param_1 + 0x114)` for the owner, the galaxy view — CORRECT
- `*(int*)(owner + 0x9c) == 1` for the owner's side — CORRECT
- Color: `0xff` if side==1 else `0xff00` — CORRECT
- Viewport offsets `+0xcc, +0xd0, +0xd4, +0xd8` — CORRECT
- `"Battle in Progress..."` string at `0x006a879c` — CORRECT
- GDI calls: `GetDC`, `CreateCompatibleDC`, `ReleaseDC`, `SelectObject`, `DeleteDC` — all CORRECT

### FUN_005617b0 — Palpatine at Coruscant Flag
- Calls `FUN_005070d0` (Emperor `0x34000280`) and `FUN_004f6b50` (his location) — CORRECT
- `*(byte*)((int)this_00 + 0xac) & 1 == 0` (not a prisoner) — CORRECT
- `*(byte*)((int)this_00 + 0x50) & 1 != 0` (active) — CORRECT
- `local_14 = 0x90000109` (Coruscant) — CORRECT
- `((byte)*(undefined4*)((int)local_18 + 0x24) & 0xc0) == 0x80` owner check — CORRECT
- Calls `FUN_0055f650` to write bit 0 of `this+0x58` — CORRECT

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

### FUN_00509620 — System Coordinates
- Reads `*(this+0x2c) + 0x4c`, 0 when `+0x2c` is null — CORRECT
- Callers `FUN_00555b30` and `FUN_00459e30` (`FUN_00555b30.c:108-109`, `FUN_00459e30.c:274`) — CORRECT

### FUN_005457f0 — Story Hook (Event `0x320`)
- Returns `1` when `FUN_005438a0` is false — CORRECT
- Calls `FUN_00549910(this, param_4)` otherwise — CORRECT
- Called by `FUN_00514a60` (`FUN_00514a60.c:43`) — CORRECT

### FUN_005438a0 — Named-Character Check
- Exactly 6 entity codes: `0x32000242, 0x31000241, 0x33000243, 0x38000343, 0x35000281, 0x34000280` — CORRECT

### FUN_00549910 — Story Check Chain
- Calls `FUN_005442f0`, `FUN_00544da0`, `FUN_00544030`, `FUN_00544130`, `FUN_005443f0`, `FUN_005444e0`, `FUN_005445d0`, `FUN_00544a20`, `FUN_00544be0`, `FUN_00544fc0` in that order and ANDs the results — CORRECT
- Setter masks and slots: `FUN_00543b60` (`4`, `+0x1c4`), `FUN_00543bd0` (`0x20`, `+0x1c8`), `FUN_00543c40` (`0x40`, `+0x1cc`), `FUN_00543cb0` (`0x80`, `+0x1d0`), `FUN_00543d20` (`0x100`, `+0x1d4`), `FUN_00543d90` (`0x200`, `+0x1d8`), `FUN_00543e00` (`0x400`, `+0x1dc`), `FUN_00543e70` (`0x800`, `+0x1e0`), `FUN_00543ee0` (`0x1000`, `+0x1e4`), `FUN_00543f50` (`0x2000`, `+0x1e8`) — CORRECT
- `FUN_005442f0` and `FUN_00544da0` pass their condition to the setter (held); the others call it with 1 then 0 (pulse), and `FUN_00544fc0` calls `FUN_00543f50(this, 1)` then `FUN_00544da0` — CORRECT

### The story checks
- `FUN_005442f0`: Han (`FUN_005435f0`) with `+0xac` bit 0 set, `+0x50` bit 0 set, `+0x78 & 0xc0 == 0`, `+0x50 & 0x1000 == 0`, location `0x98000481` → 1 — CORRECT
- `FUN_00544da0`: skipped when `this+0x58 & 0x2000`; Luke a prisoner, active, `+0x78 & 0xc0 == 0`, `+0x50 & 0x1000 == 0`, location not `0x98000481`, `+0xb0 & 2`; Vader and the Emperor pass `FUN_005439e0` — CORRECT
- `FUN_00544030`: gate `this+0x58 & 1` set and `& 0x40` clear; Luke (`FUN_005434b0`) passes `FUN_005439e0` — CORRECT
- `FUN_00544130`: gate `this+0x58 & 0x40`; Luke passes `FUN_005439b0`; when `+0x78 & 0x80` is set, his mission's family must equal `0x71` and its `+0x68` (`piVar5[0x1a]`) equal 8, then slot `+0x1d8` — CORRECT
- `FUN_005443f0`, `FUN_005444e0`: gate `this+0x58 & 0x40`; Leia (`FUN_00543550`) and Chewbacca (`FUN_00543690`) pass `FUN_005439e0` — CORRECT
- `FUN_005445d0`: Luke, Leia, Han `+0xac` bit 0; Chewbacca `+0xac` bit 0 or `+0x50` bit 3; a member of `FUN_00506ed0` (root `+0xdc`, Jabba's Palace) with `+0xac` bit 0 (`FUN_004f2640`, `FUN_0052bed0`); Vader passes `FUN_005439b0`; on a mission, family `0x73` leaves the bit alone, else `+0x78 & 0x10` leads to `FUN_00534640(Vader, 1)` — CORRECT
- `FUN_00544a20`: gate `this+0x58 & 0x1000`; Vader's location key differs from Luke's (`local_30 != local_2c`) — CORRECT
- `FUN_00544be0`, `FUN_00544fc0`: gate `this+0x58 & 0x1000`; Vader with Luke and the Emperor elsewhere, or all three together — CORRECT

### FUN_005434b0 — Luke and His Location
- `FUN_00507010` builds id `0x32000242`; `FUN_004f6b50` gives the location; `FUN_004025b0` / `FUN_004f26d0` its key — CORRECT
- `FUN_00543550` (`FUN_00506fb0`) and `FUN_00543690` (`0x38000343`, `FUN_005066a0`) share the shape — CORRECT

### FUN_00534640 — Resign Request
- Sets flag `8` in the character's `+0x78` via `FUN_0053a640(8, param_1, &this+0x78)` — CORRECT
- Vtable `*this+600` (decimal 600 = 0x258) — CORRECT
- Calls `FUN_00535a50` for both side views — CORRECT

### FUN_00560d50 — Moved Object: Clear Character Field +0x96
- Character branch `0x30-0x3b`, capital ship branch `0x14-0x1b` — CORRECT
- Family `0x34` (Emperor Palpatine) runs `FUN_005617b0` — CORRECT
- Non-zero `+0x96` is cleared with `FUN_004ee350(object, 0, ctx)`, except between two capital ships in the same fleet (`0x08-0x0f`) — CORRECT

### FUN_0050d5a0 — Observation Level (13-Case Switch)
- All case values 0-12 plus default verified — CORRECT
- Case 6 sets `bVar3=true` — CORRECT
- Case 8 and 10-11 set `bVar1=true` — CORRECT
- `local_134=1, local_130=3` constants set before switch — CORRECT
- Range filter values (0x08, 0x10, 0x20, 0x28, 0x30, 0xa0, 0xb0) verified in case bodies — CORRECT

### FUN_00532e00 — Construction Yard Research Order Notification
- Strings `"SideConstructionYardRdOrderNotif"` / `"ConstructionYardRdOrder"` — CORRECT
- Event ID `0x127` (295) — CORRECT
- Not a victory handler; the victory flags are `FUN_005180e0` — CORRECT

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

None open. Every section above matches its `.c` file.

---

## Decompiles

Every function the document cites has its `.c` file in `ghidra/notes/`,
including `FUN_0053a000`, `FUN_0053a010`, `FUN_0053e0f0`, `FUN_00539fd0`,
`FUN_00500670`, `FUN_0053a640`, `FUN_0055e4d0` and `FUN_00549910`.

---

## Notes

1. **FUN_0050d5a0**: case 3 copies two family ranges, `0x10-0x20` and `0x22-0x28` (`local_74 = 0x22; local_70 = 0x28`).

2. **FUN_00532f40**: registers its event with `FUN_0053fdd0(300, ...)`, the one-parameter registrar, where the force dispatchers use `FUN_0053fcf0`.

3. **The `0x98000481` DatId** is Jabba's Palace (ABODESD). `FUN_005442f0` requires Han to be there; `FUN_00544da0` requires Luke not to be.
