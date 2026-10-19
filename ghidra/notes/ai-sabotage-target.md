---
title: "The AI's Sabotage and Death Star Sabotage Planners: Target, Staging, Team and Decoys"
description: "How the strategic AI picks a Sabotage target system and object (the six posture slot-12 queries and FUN_004ae480's random draw), the staging system, the team and decoys, the Death Star Sabotage planner and why the shipped AI never requests it"
category: "ghidra"
created: 2026-10-17
updated: 2026-10-17
tags: [ai, missions, sabotage, death-star, posture, planners]
---

# The AI's Sabotage planners

Sides are the game's: 1 Alliance, 2 Empire (`side-switches.md`). `hyp:` marks anything not traced through code. No function on these paths reads a GNPRTB parameter (no `DAT_006xxxxx` or `DAT_007xxxxx` load in the files cited); every threshold is a literal.

## 1. Two planner machines

Ordinary Sabotage (`0x69000012`, constructor `FUN_0047d380`, vtable `0x0065a670`) runs the regional state machine `FUN_004bda60` (vtable slot 11, `FUN_0047d380.c`, `FUN_004bda60.c:11-65`):

| State | Action |
|-------|--------|
| 2 | `FUN_004bdbc0`: the target. Success to 3, failure to 9 (`FUN_004be4c0`, give up) |
| 3 | `FUN_004bdcd0`: the staging system (+0x38), failure to 9 |
| 4 | `FUN_0047b110`: drop members that fail `FUN_0047ba40` |
| 5 / 6 | vtable `+0x40` team `FUN_0047d520`, `+0x44` decoys `FUN_0047d720` |
| 7 | `FUN_004be230`: Move `0x201` for members off the staging system, flag 8 when all stand there |
| 8 | `FUN_0047b360`: order `0x240` naming the object in planner `+0x30` |

DS Sabotage (`0x6a000041`, vtable `0x0065a958`) runs the main machine `FUN_004bced0` (`ai-mission-planning.md`): team `+0x48` `FUN_00482180`, decoys `+0x4c` `FUN_00482330`, target `+0x50` `FUN_00482560`, then the location step `FUN_004bd0a0`.

Planner fields: `+0x30` target object id, `+0x34` target system, `+0x38` staging system, `+0x70` regional record id (Sabotage), `+0x68` AI context, `+0x64` side record pool.

## 2. Ordinary Sabotage target

**Request.** Posture slot 6 `FUN_004b7150.c:25-35` recommends `0x69` when no earlier kind took the primary slot (Recon `0x54`, Espionage `0x52`), the regional record's `+0xac` counter is 0 and the sector record's `+0x2c` bit `0x8` is set (a system of the sector held by the other side, `FUN_00432350.c:24`). `FUN_004d2690` creates the planner from the record's `+0xbc`/`+0xc0` and stores the record id at planner `+0x70`. `FUN_004e8290` has no `0x69` entry (`FUN_004e8290.c:90-99` is the only `0x6a`).

**Target (`FUN_004bdbc0`).** It clears `+0x34`/`+0x30`, looks the regional record up (`FUN_005f5500`, context `+0xd8`) and calls `FUN_00476e80(record, kind & 0xffffff, &+0x34, &+0x30)` (`FUN_004bdbc0.c:21-26`). The switch sends kind `0x12` (Sabotage) to the record's posture object (`+0x5c`) slot `+0x30` (`FUN_00476e80.c:6-8`), whose output order is (system, object). Success needs `+0x30` family byte in `1..0xfe` and `+0x34` family in `0x90..0x97` (`FUN_004bdbc0.c:30-31`).

**System.** Slot 12 is overridden per posture kind (vtables `0x0065c108` kind 5, `0x0065c160` kind 3, `0x0065c1b8` kind 6, `0x0065c210` kind 4, `0x0065c268` kind 2, `0x0065c2c0` kind 1): `FUN_004b1f80`, `FUN_004b3f80`, `FUN_004b5540`, `FUN_004b7b10`, `FUN_004b99b0`, `FUN_004bab30`. Each runs only when the record's sector (`+0x10`->`+0x34`) has family `0x80..0x8f`, then tries a fixed sequence of queries, stopping at the first that yields a system. A query is `FUN_0041a340(ctx, sector, 0, req2c, req30, 0, 0, 0, key 0x1d, order 2)` (`FUN_0041a340.c`, `FUN_00431830.c`): the sector record's ten systems whose `sys+0x2c` holds `req2c` and `sys+0x30` holds `req30`, sorted by system-record slot `0x1d` = `sys+0xbc`, order 2 (smallest ranks best, `ai-production.md` "dir 2"), ties by coin flips (`FUN_0041c250`, `FUN_0041cd80(10) < 5`), picked by `FUN_004357b0`. No filter tests ownership directly: the `+0x2c` bits 0x10000..0x400000 exist only on systems held by the other side (`FUN_00433e40.c:604-672`).

`sys+0xbc` (`ai-production.md` F1.3): the sum of fleet `+0xa4` over foreign active fleets at the system (`FUN_00433e40.c:237-238`, fleet field `FUN_00433780.c:103`) plus, at an enemy system, its operational regiments `+0x90` and squadrons `+0xac` (`FUN_00433e40.c:639-641`). The AI sabotages the least defended qualifying system.

Query bits (`sys+0x2c`, enemy systems unless noted): `0x40000` shield strength `+0x68 > 0` (`:646`), `0x80000` an active Death Star shield (`:649`), `0x100000` battery attack `+0x78 > 0` (`:652`), `0x200000` regiments `+0x90 > 0` (`:655`), `0x400000` squadrons `+0xac > 0` (`:658`), `0x20000` yards (`:669`), `0x10000` mines or refineries (`:672`), `0x800000` a foreign fleet with an active ship (any owner of the system, `:240`), `0x2000000` headquarters (`:152`). `sys+0x30`: `0x100` more than one shield generator (`FUN_00433e40.c:461`), `0x200` foreign fleet (`:239`), `0x80000000` headquarters (`:151`).

Sequences (mask: extra `+0x30` requirement), each the query list in file order:

| Kind | Sequence | File |
|------|----------|------|
| 5 | `0x40000` (S `0x100`), `0x80000`, `0x100000`, `0x40000`, `0x200000`, `0x400000`, `0x800000` (S `0x200`) | `FUN_004b1f80.c:33-180` |
| 3 | as kind 5 with `0x20000` between `0x200000` and `0x400000` | `FUN_004b3f80.c` |
| 6 | `0x40000` (S `0x100`), `0x2000000` (S `0x80000000`), `0x80000`, `0x100000`, `0x40000`, `0x200000`, `0x400000`, `0x800000` (S `0x200`) | `FUN_004b5540.c:33-198` |
| 4 | as kind 5 without `0x20000`; first copies planner-side `+0x5c` when it is a non-own system with `sys+0x30 & 0x100`, then the first query overwrites it (hyp: dead) | `FUN_004b7b10.c:33-189` |
| 2 | `0x400000`, `0x800000` (S `0x200`), `0x20000` | `FUN_004b99b0.c:39-83` |
| 1 | `0x2000000` (S `0x80000000`), `0x20000`, `0x10000`, `0x200000`, `0x800000` (S `0x200`), `0x400000` | `FUN_004bab30.c` |

If no query yields a system, `FUN_004ae480` is skipped and `+0x30` stays null: the planner ends in state 9.

**Object (`FUN_004ae480(posture, &system, &object, mask)`).** The mask is that of the query that found the system. It collects the system's usable children (mode 1: `+0x50` bit 0, so completed and not en route, `FUN_00513050.c`, `object-state-flags.md`; no side filter) by kind, appending in this order (`FUN_004ae480.c`):

| Mask | Objects | Lines |
|------|---------|-------|
| `0x2000000` | family `0x20` headquarters | 60-93 |
| `0x40000` | `0x24` shield generators | 94-128 |
| `0x80000` | `0x25` Death Star shields | 129-162 |
| `0x100000` | `0x23` then `0x22` batteries | 163-230 |
| `0x200000` | `0x10..0x13` regiments | 231-265 |
| `0x400000` | `0x1c..0x1f` squadrons | 266-300 |
| `0x800000` | every ship of each non-own fleet (list `fleet+0x48`, no completion test, `:316`) | 301-348 |
| `0x20000` | `0x29` training, `0x28` shipyards, then `0x2a` construction yards unless `ctx+4 & 0x40` | 349-451 |
| `0x10000` | all `0x2d` refineries, then the first `0x2c` mine only (`if`, not `while`) | 452-512 |

Then one draw, `FUN_0041cd80(count)` uniform on `[0, count)`, and a walk of that many links from the list head; the object id goes to `+0x30` (`FUN_004ae480.c:517-533`). Special forces and characters are never offered.

**Legality at the order.** `FUN_00593500` requires target family `[0x10,0x40)` and `+0x50` bit 2 (`FUN_00593500.c:25-58`); `FUN_0056a110` refuses `[0x18,0x1c)` (`{0x40,0x28}`) and characters (`{0x40,0x25}`) (`death-star-product.md` §4). hyp: a ship branch can draw a Death Star or an uncompleted ship in a usable fleet, and the order refuses it.

## 2a. The defence figure: unit counts, not firepower

**Ship record `+0x88` (fleet `+0xa4`) is a unit count.** `FUN_00484630` zeroes the ship record's `+0x44..+0xbb` (`:41-45`) and, for a ship that exists and is not destroyed (`obj+0x50` bit 3 clear, `:57`), adds 1 to `+0x88` for the hull (`:109`). It then walks the ship's carried fighter squadrons (`FUN_005039d0`, `:110-170`) and regiments (`FUN_00504c40`, `:171-205`). A carried object adds 1 to `+0x88` (and 1 to `+0x5c` squadrons-active, `+0x68` regiments-active) only when it is completed (`obj+0x50` bit 2 set) and not en-route-active (bit 5 clear) (`:126`, `:142-161`; `:186-197`). Regiments add `+0x88` under the same two tests (`:187-196`). No class field feeds it: not attack, turbolaser, hull or shield (those go to `+0x50`, `+0x7c`, `+0x90`, `+0x44`; `ship-damage-repair.md`, "Ship record scaling"). So `+0x88 = 1 + active squadrons + active regiments aboard`, per hull, and character passengers and special forces add nothing.

`FUN_00433780` folds it into the fleet: `fleet+0xa4 += ship[0x11]` (`:103`, ship record `+0x44 + 0x44`), inside the block skipped when the ship carries flag `0x2000` (not completed, `FUN_00484630.c:64-68`; `FUN_00433780.c:90`). Ships in transit (flag `0x800`) still count. `FUN_00433620.c:53-60` calls both per ship of the fleet.

**Which fleets feed `sys+0xbc` (`FUN_00433e40.c:195-240`).** The loop `FUN_004ffe70(system, mode 3)` walks the system object's direct child fleets (types `8..0x10`, `FUN_004ffe70.c:9-10`; mode 3 is `+0x50` bit 6, existing, `object-state-flags.md`), with no side filter. Per fleet record it adds `fleet+0xa4` to `sys+0xbc` only when fleet flag `0x1` is clear and `0x4000` is set (`:229-238`):

| Fleet flag | Meaning | Source |
|------------|---------|--------|
| `0x1` | fleet owner side equals the AI's side (clear: the other side's fleet, foreign) | `FUN_00433620.c:30-38,40-42` |
| `0x4000` | at least one ship record without `0x800` (en route) and without `0x2000` (not completed): an active ship | `FUN_00433780.c:18-26`, `FUN_00484630.c:61-68` |

Only fleets standing at the system count: the loop is the system's own child list, and no other statement writes `sys+0xbc` (`:237`, `:639`). A fleet whose only ships are in transit or incomplete has `0x4000` clear and contributes 0 even though its ships carry `+0xa4`. hyp: whether a fleet in hyperspace is still a child of its origin system is not traced here.

At a system the other side holds (`sys+0x30 & 4`), `sys+0xbc` adds `sys+0x90` (operational regiments: regiment children with `obj+0x50` bit 0, `:195-204`) and `sys+0xac` (squadrons, `FUN_00433e40.c:564-583`) (`:639-641`), one unit each; the neutral branch (`:610-624`) adds nothing.

**Rust.** `CapitalShipClass` (`world/mod.rs:336`) has no field for it, and none is needed. `ShipRecord` (`ai_records.rs:153`) holds `squadrons_active` and `regiments_active` (set at `:572`, `:583`); the hull term is the constant 1. `FleetRecord` (`:186`) sums them (`:649-652`) but has no `+0xa4` and no `+0x88`. Missing: a ship `units` figure `1 + squadrons_active + regiments_active` and a fleet `units` summed in the `ship::INCOMPLETE == 0` block (`:645`), then `SystemRecord` `+0xbc` over non-own fleets with an active ship (`fleet::ACTIVE`, `0x4000`: a ship with neither `EN_ROUTE` nor `INCOMPLETE`; it is a different bit from `fleet::READY` `0x8000`, which also needs no damage). Port differences to check: the original excludes carried objects that are en route-active (bit 5), the port counts all.

## 3. Staging, team, decoys

**Staging (`FUN_004bdcd0`).** Planner `+0x38` is an own system in the target sector (`+0x74`) with `sys+0x30 & 1` and `sys+0x28 & 2` clear, through `FUN_0041a460` (random key), else `FUN_0041b2b0` over the nearest other sector (`FUN_004bdcd0.c:184-221`). The machine runs it before the team, so the nearness query `FUN_00403750` (squared distance to `+0x38`, order 2) joins the team and decoy selections of a Sabotage planner.

**Team (`FUN_0047d520`).** Up to 2 members (`FUN_0047d520.c:28`). Two `FUN_00403460` queries over the side pool, `(0x800002, caps 4, forbid 0x4f400051, 0, skill, 0x33, 10000, order 1)` with skill 5 (combat) and skill 1 (espionage), plus the nearness query (`:29-44`). Required: own side `0x2`, unassigned `0x800000`, Sabotage-capable (caps bit 2). Forbidden: leaders `0x40000000`, officer ranks and the reserved bit `0x0f000000`, mandatory mission `0x400000`, prisoner `0x40`, injured `0x10`, not recruited `0x1`. Ratings: combat >= 51 and espionage >= 51. Ranking sums list positions (`ai-mission-planning.md` "Picking"). Candidates are characters and special forces alike.

**Decoys (`FUN_0047d720`).** Special forces (`0x10800002`, caps 4, forbid `0x7400051`), espionage >= 51, order 1, plus nearness; the cap is `(sys+0xbc + 2) / 2` of the target system's record, read at planner `+0x34` through `FUN_00403d30(context +0x2c)` (`FUN_0047d720.c:36-53`). That record is the system AI record (`rec+0x30`, `rec+0x28` fields tested by `FUN_004bd0a0`), so `+0xbc` is the defence figure above.

## 4. Death Star Sabotage

**Target (`FUN_00482560`).** It clears `+0x30`/`+0x34` (`:36-37`), so its first branch (a target already in `[0x18,0x1c)`, `:43-69`) is dead code. It lists the side's AI fleet records that are not own and carry a Death Star: `FUN_0041a4c0(ctx, 0x10, 1, key 0x15, order 2)` (`:73`), pops them best-first into a list of their systems (`:76-110`), and takes the first system whose record has `rec+0xbc / 2 <= team size` (`FUN_004bd000`, members standing at `+0x38`) or `team size == planner +0x44()` = 4 (`FUN_004047d0`) (`:123-148`). The chosen system goes to `+0x34`; `FUN_00482910` requires `sys+0x2c & 0x1000000` (set only for Alliance records, `FUN_00433e40.c:244`), walks the system's usable fleets and takes the first ship of a non-own fleet with class flag `0x80` (`FUN_00482910.c`). `+0x30` non-null sets flag `0x10` (`:155-162`). hyp: a Death Star fleet's `sys+0xbc` normally exceeds 5, so the threshold blocks it.

**Team and decoys.** Team `FUN_00482180` (`FUN_004035d0`, flags `0x2`, caps `0x80`, forbid `0x47400051`, combat and espionage >= 51, 2 members, `FUN_00482180.c:30-35`). Decoys `FUN_00482330`: special forces `0x10800002`, caps `0x80`, forbid `0x7400051`, espionage >= 51, cap 4 (`FUN_00482330.c:27-40`). Only Alliance agents are legal for `0x6a` (`ai-mission-planning.md` legality table).

**Request.** `FUN_004e8290.c:90-99` writes `0x6a` only when `this+4 == 1` (Alliance), `ctx+0x334 < 1` and `ctx+4` bit `0x4` is set. `FUN_0041ba80.c:6,21-22` sets that bit only from an own fleet record with bit `0x10` (an own ship with class flag `0x80`); the foreign branch (`:28-37`) never does. Record 136 is Empire-only, so the Alliance flag is never set and the Empire fails the side test and the legality: the shipped AI never requests Death Star Sabotage.

## Cited decompiles

[FUN_00593500](FUN_00593500.c), [FUN_00482330](FUN_00482330.c), [FUN_00482560](FUN_00482560.c), [FUN_00482910](FUN_00482910.c), [FUN_004bda60](FUN_004bda60.c), [FUN_004bdbc0](FUN_004bdbc0.c), [FUN_004bdcd0](FUN_004bdcd0.c), [FUN_004be230](FUN_004be230.c), [FUN_00476e80](FUN_00476e80.c), [FUN_004b1f80](FUN_004b1f80.c), [FUN_004b3f80](FUN_004b3f80.c), [FUN_004b5540](FUN_004b5540.c), [FUN_004b7b10](FUN_004b7b10.c), [FUN_004b99b0](FUN_004b99b0.c), [FUN_004bab30](FUN_004bab30.c), [FUN_004ae480](FUN_004ae480.c), [FUN_0047d520](FUN_0047d520.c), [FUN_0047d720](FUN_0047d720.c), [FUN_004b7150](FUN_004b7150.c), [FUN_004e8290](FUN_004e8290.c), [FUN_0041ba80](FUN_0041ba80.c), [FUN_00433e40](FUN_00433e40.c).
