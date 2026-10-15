---
title: "RE Accuracy Review: Combat Scholar Documents"
description: "Spot-check of the offsets, vtable slots and event ids in cpp-class-hierarchy.md, rust-implementation-guide.md and modders-taxonomy.md against the decompiled C, with what each checked field and function is"
category: "ghidra"
created: 2026-03-15
updated: 2026-10-14
---

# RE Accuracy Review: Combat Scholar Documents

Scope: claims in `cpp-class-hierarchy.md`, `rust-implementation-guide.md`
and `modders-taxonomy.md`, checked against `ghidra/notes/FUN_*.c`. The
byte offsets, vtable slots and event ids hold; several of the meanings
those three notes give them do not. The tables below give the reading the
decompiles support.

## Offsets, slots and event ids that hold

| Claim | Source | Meaning |
|---|---|---|
| Vtable `+0x248` max hull, `+0x29c` hull changed | `FUN_00501490_hull_validate.c` | Capital-ship hull setter (`HullValueDamage`) |
| Vtable `+0x244` max squad size, `+0x260` squad changed | `FUN_005032c0_squad_validate.c` | Fighter squadron size setter (`SquadSizeDamage`) |
| Vtable `+0x2a0`, `+0x2a4`; `+0x64` bits 0-3 and 4-7 | `FUN_00501510_shield_validate.c`, `FUN_005015a0_weapon_validate.c` | Shield and weapon recharge allocation, written by XOR mask (`(v ^ old) & 0xf ^ old`) |
| `+0x60` int | hull and squad setters | Hull value / squad size |
| `+0x66` short, vtable `+0x238` | `FUN_005341a0_loyalty_validate.c` | Character base loyalty, checked to `[0, 100]` |
| `+0x8a` short, vtable `+0x318` | `FUN_004ee030_enhanced_loyalty.c` | Character enhanced loyalty |
| `+0x9a` short, vtable `+0x338` | `FUN_004ee470_hyperdrive_mod.c` | Character hyperdrive modifier |
| Event `0x1a0` | `FUN_005038e0.c` | `FightSquadSquadSizeDamageNotif` |
| Event `0x1c0` | `FUN_005029a0_hull_damage.c` | `CapShipHullValueDamageNotif` |
| Event `0x340` | `FUN_00504a00_troop_blockade_destroy.c` | `TroopRegDestroyedRunningBlockade` |
| Destroy causes `+0x40 & 0xff`: 6, 7, `0x15` | `FUN_004fc080_mission_destroy.c` | Sabotage (event `0x305`), Assassination (`0x306`), Autoscrap (`0x304`) |

## Fields and functions with their readings

| Item | Reading | Note |
|---|---|---|
| Character `+0x96` short, vtable `+0x330`, setter `FUN_004ee350` | CommandKind (0 none, 1 Commander, 2 Admiral, 3 General) | `character-roster.md`, `ground-combat.md` |
| `FUN_00560d50` | Character manager's move hook: clears CommandKind; families `0x30`..`0x3b` characters, `0x14`..`0x1b` capital ships, `0x08`..`0x0f` fleets | `ground-combat.md` |
| `FUN_005617b0` | Emperor-at-Coruscant flag: Palpatine (`0x34000280`) not a prisoner, active, at Empire-held Coruscant `0x90000109` | `ground-combat.md`, `death-star-strategic.md` |
| MissionMgr `+0x58` | Story-flag word, bits 0-14 | `space-combat.md`, `story-missions.md` |
| `FUN_00543b60`, `FUN_00543bd0`, `FUN_00543cb0`, `FUN_00543d20` | Setters of story bits 2, 5, 7, 8 (`FUN_0053a640` masks `4`, `0x20`, `0x80`, `0x100`); manager handler slots `+0x1c4`, `+0x1c8`, `+0x1d0`, `+0x1d4` | `story-encounters.md` |
| `FUN_005442f0`, `FUN_00544da0`, `FUN_00544030`, `FUN_00544130`, `FUN_005443f0`, `FUN_005444e0`, `FUN_005445d0`, `FUN_00544a20` | Story checks of the `FUN_00549910` chain (Han at the palace, final battle ready, Dagobah, Luke/Leia/Chewbacca to the palace, Pickup, Vader to Luke) | `space-combat.md` |
| `FUN_00544130` slot `+0x1d8` on `+0x78` bit 7 | Calls the Dagobah mission's slot when Luke is on it | `story-missions.md` |
| `FUN_0054a1d0` and `+0x24 >> 4 & 3` | Bit 2 broadcaster; `FUN_004fd600` picks the Alliance (1) or Empire (2) view | `space-combat.md` |
| Character `+0xac` bit 0 | Prisoner | `story-encounters.md` |
| Character `+0xb0` bit 1 with `0x98000481` | Luke's final-battle flag; `0x98000481` is Jabba's Palace | `story-missions.md` |
| `FUN_005445d0` family `0x73` | Pickup mission class (`0x73000082`) | `story-missions.md` |
| `FUN_00534640` | Sets character `+0x78` bit 3 (resign the current mission), vtable `+0x258` | `story-missions.md` |
| `FUN_00556430` | Travel start; `FUN_00555d30` → `FUN_00555b30` → `FUN_0055d8c0` give transit ticks from GNPRTB `0x1400` | `build-delivery.md` |
| `FUN_00509620`, `+0x2c` → `+0x4c` | The object's system coordinates (two packed shorts) | `build-delivery.md` |
| `FUN_005385f0` family `0x14`..`0x1b` | Capital ships | |
| `FUN_005a7500` | Tactical battle manager constructor (`__fastcall`, one object of `0x3a8` words with the message handlers) | `tactical-*.md` |

## Families

DatId families come from the DAT `family_id` fields: fleets `0x08`-`0x0f`,
regiments `0x10`, capital ships `0x14` and the Death Star `0x18`
(`CAPSHPSD`), fighters `0x1c`, facilities `0x22`-`0x2a`, major characters
`0x30`-`0x35` (`0x34` Palpatine), minor characters `0x38`, special forces
`0x3c`, missions `0x41`-`0x73`, sectors `0x80`, systems `0x90`/`0x92`.
Bombardment is `bombardment.md`; every space battle is the tactical engine
(`simulate-runs-tactical-engine.md`).
