---
title: "Strategic Death Star: Family 0x18, Destroy System, the Cascade and a Second Superweapon"
description: "How the strategic Death Star is identified (CAPSHPSD family 0x18), built, moved, fired and sabotaged, what destroying a system does to fleets, troops, facilities and characters, what victory and the AI read, and what a second superweapon runs through or hits"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [death-star, superweapon, destroy-system, bombardment, capital-ships, mod-surface]
---

# Strategic Death Star

Answers task-graph node `or-q-mod-superweapons-awzg` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-superweapons`; the Emperor check `FUN_005617b0` / `FUN_005070d0`, the `[0x18,0x1c)` iterator `FUN_005003d0`, the kill-phase family test `FUN_0058d440`, the per-system cap `FUN_0040f340`, the fleet's destroy slot (vtable `0x0065d438` `+0xb4` → `FUN_00524fb0`) and the tactical side test `FUN_005afe40` were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code. The tactical battle is `tactical-death-star-path.md`; the order's phases are `bombardment.md`.

## Summary

1. The Death Star is an ordinary capital ship whose CAPSHPSD class record (136) has `family_id` 24 (`0x18`) instead of 20 (`0x14`). Every rule keys on the family range `[0x18,0x1c)`; no function names record 136.
2. Build, move, split, join, blockade, hyperdrive and sabotage run the ordinary ship code from DAT fields. Which side may build it is the record's `is_alliance` / `is_empire` flags.
3. Fire is order `0x223` (Destroy System). Its gate and its kill phase are family-range scans; nothing tests side, shield, charge or the target's owner.
4. A new family-`0x18` record passes every test, for either side, but meets a cap of one family-`0x18..0x1b` ship per system (literal 1), a single tactical Death Star slot, and a tactical attacker test for side 1. Families `0x19`..`0x1b` pass the range tests but have no object factory.
5. Family `0x34` is the Emperor's: `FUN_005617b0` tests that Palpatine (`0x34000280`, `FUN_005070d0`) is free, active and at Empire-held Coruscant (`0x90000109`) and writes bit 0 of his `+0x58` through `FUN_0055f650` (`story-encounters.md`, row 7a).

## Identification

| Rule | Function | Kind |
|---|---|---|
| CAPSHPSD record 136: `family_id` 24, TEXTSTRA 10120, `is_empire` 1, `is_alliance` 0; the other 29 records are family 20 | `CAPSHPSD.DAT` | DAT |
| Capital-ship iterators over literal ranges: `[0x14,0x18)` (`FUN_005001b0`), `[0x18,0x1c)` (`FUN_005003d0`, ctor `FUN_00500450`), `[0x14,0x1c)` (`FUN_00502db0`) | | literal |
| First Death Star in the game `FUN_005061f0`; "is this id a Death Star" `FUN_00506300` / `FUN_00506260` | | literal |
| The object factory registry (`FUN_0051ebf0`, `FUN_00540540`, lookup `FUN_00540500`, used by `FUN_0053f100`, 58 families) holds capital-ship factories for `0x14` (`FUN_004fffe0`, vtable `0x0065d650`) and `0x18` (`FUN_005002a0`, vtable `0x0065d930`) only | | table (code) |
| The two ship vtables differ only in the destructor, slot 1 and the clone slot (57); the class is a type tag | | literal |
| Class lookup: CAPSHPSD covers `[0x14,0x1c)`; index = low 24 bits (136 → `0x18000088`) | `FUN_00567790`, `FUN_0058ec90` | table |
| A fleet "has a Death Star" when a child is in `[0x18,0x1c)` (`FUN_005003d0`, `FUN_00582440`, `FUN_00507ab0`, `FUN_0040b0e0`). The AI record flag `0x10` (`+0x38`, read by `FUN_004b1070`) comes from object `+0x50` bit 7 through `FUN_00433780`, not from the family | | literal |

## Construction

- A shipyard (production family `0x28`) lists a class when its production range `[40,41)` holds `0x28`, the side flag matches (side 1 `is_alliance`, side 2 `is_empire`), `research_order` is at or below the side's level and the refined cost is non-zero (`FUN_0052e580` → `FUN_0051cb20` → `FUN_00567880`; side key `FUN_00530a20`). The Death Star's `research_order` is 0 and its cost 584, so every Empire shipyard lists it from the first day. No research, date or story gate.
- Build order `0x211` (`FUN_005385f0`, class vtable `0x006611a0` slot 20): product family `0x10`..`0x3f` (`FUN_0052e970`; error `0xf3` when maintenance exceeds spare capacity, the Death Star's maintenance is 600); a code-built range table puts `[0x18,0x1c)` in its own category; a family outside all nine categories gives error `0x19` (`FUN_00551a20`, `FUN_005504d0`, `FUN_0054f5b0`).
- **One per system.** System slot `+0x80` (`FUN_00507ab0`; vtables `0x0065e640`, `0x00663698`, `0x00664098`) counts `[0x18,0x1c)` ships over the system's fleets and reports room `FUN_0040f340() - count`; `FUN_0040f340` returns the literal 1. The order-leg builder `FUN_00550700` calls it (via `FUN_00552d10`, `FUN_005513a0`, `FUN_00551270`). The count has no side filter (hyp: `FUN_00513050` leaves `+0x14` at 0); which refusal a second ship raises is not read.
- Build time, completion and delivery are the ordinary path (`build-delivery.md`); no family-`0x18` branch.
- Not seeded: no CMUN table holds record 136, though the seeder's capital-ship test is `[0x14,0x1c)` (`FUN_0051aa50`, `FUN_0051b1f0`, `FUN_0051bf30`). No story or mission event creates it.

## Movement

- Ship speed is the class hyperdrive (80) minus the damage term, else `hyperdrive_if_damaged` (140), else GNPRTB 1 (100) (`FUN_00500820`, `FUN_005011f0`, `FUN_00501230`); the fleet takes its slowest ship (`FUN_004fd900`, through `FUN_00502db0`). Travel time is distance over speed scaled by GNPRTB `0x1400` (`FUN_0055d8c0`). A move between systems with speed 0 is refused 1/`0x18` (`FUN_00555920`).
- Split, join, Create Fleet and the other ordinary orders use `[0x14,0x1c)`; the Death Star counts as one ship in the blockade formula (`FUN_0055a020`).
- A fleet may move to and stay at a destroyed system; regiment landings and mission team `0x241` are refused there (`FUN_00537180`, `FUN_005073d0`).

## Fire: Destroy System (`0x223`)

- **Gate** `FUN_00582440`: each team fleet needs a bombard-strength sum (slot `+0x1dc`) above 0 and, for `0x223`, a usable ship in `[0x18,0x1c)`; one passing fleet is enough. No side, shield, charge, operational or target test.
- **Menu:** `0x223` is in every fleet's order list, always listed (`FUN_0040f340`) and enabled only when `FUN_00581e80` → `FUN_00553770` and `FUN_00582440` pass (`FUN_004ff8e0`, `FUN_0050f0b0`, `FUN_005827e0`).
- **Target:** the only refusal is an already destroyed system (1/`0x22`, `FUN_00507f10`, `FUN_00507fc0`). Own, neutral, enemy, the Alliance HQ, Coruscant and the last system are all legal.
- **Kill phase:** `FUN_0058d990` walks the fleets with order bit `+0x58` bit 6 (`FUN_0058ca70`); its callback `FUN_0058d440` reports a ship in `[0x18,0x1c)`; `FUN_0058ce70` then destroys the system through slot `+0xac` with reason byte `0x10` when the system's holder is the bombarder, else `0xf`. The defence phase `FUN_0058d770` still runs afterwards.

## What destroying a system does

`FUN_005080d0` → `FUN_004f8660` → `FUN_004f71d0` stores the reason at `+0x40` and calls slot `+0x100` (`FUN_004fb200` → `FUN_004f7560`), which sets `+0x50` bit 3. Hook `+0x11c` (`FUN_00510950` → `FUN_004fb520`) calls slot `+0xb4(1, ctx)` on every child of the system and raises event `0x319` (`FUN_005650f0`: both sides observe the wreck).

| Child | Slot `+0xb4` | Result |
|---|---|---|
| Fleets (vtable `0x0065d438`) | `FUN_00524fb0`: returns 1, does nothing | **survive**, any side, with their ships, fighters, regiments and characters; the Death Star's own fleet survives |
| Regiments, capital ships, fighters, the HQ, defence and production facilities | `FUN_004f8710` | destroyed with the system's reason (`FUN_004f7ed0`) when completed and not in transit; an uncompleted one gets the autoscrap request (`+0x50` bit `0x1000`, `FUN_004f78e0`) |
| Special forces | `FUN_00503e20` | destroyed unless in transit; autoscrap request when not on a mission |
| Characters directly at the system, not on a mission | `FUN_004ef2e0` | turn adrift (role from `FUN_00534790`); not killed |

- **Character roll** in `FUN_0058d990`: only minor characters (`FUN_0056f3d0`, families `0x38`..`0x3b`) not in transit are rolled against the bombardment probability (`FUN_0055e9e0`, GNPRTB `0x606`); a hit runs slot `+0x2f0(1, 10)` (`FUN_004ef6d0`, injury) and slot `+0xac(10)`, which destroys the character. Major characters are not rolled.
- Population, owner and resources are not written by the chain; the owner stays (hyp: `FUN_0050b760` / `FUN_0050a3c0` later clear the populated bit).
- Readers of the destroyed bit: victory, the move, landing and bombard refusals (`FUN_00537180`, `FUN_005073d0`, `FUN_00553410`, `FUN_00507f10`), mission end rules 2 and 4 (`FUN_00523450`, `FUN_00592600`), the blockade side flip (`FUN_00515f40`, `FUN_00561280`) and the reason counters (`FUN_00508c60`: reasons `0xf` and `0xe` feed the blockade loss counters).
- Support shifts follow `support-shifts.md` (destroyed system -20 for the bombarder's side, galaxy-wide core -10 and rim -2, halved toward the Alliance when system `+0x88` bit 11 is set).
- **Bit 11, "a Death Star is in the sector":** `FUN_005178b0` marks every system in the sector of each usable `[0x18,0x1c)` object (`FUN_005061f0`, `FUN_00518690`, setter `FUN_0050a890`); `FUN_00514a60` reruns it when such an object changes container, `FUN_00514510` on event `0x301`. The bit is side-neutral; its effects branch by side (`support-shifts.md`, `uprising-incident.md`).

## Victory

Victory reads the Death Star only through the AD bit (Alliance HQ missing or destroyed) and the CD bit (Coruscant destroyed), both set by the cascade (`victory-conditions.md`, `game-end-path.md`). `FUN_005180e0` and the ending code test no side and no family `0x18`. Losing the Death Star does not end the game.

## Death Star Shield (facility family `0x25`)

DEFFACSD id 4: family 37, Empire only, cost 25, maintenance 20, `research_order` 3, built at the construction yard (`0x2a`). Only its family byte reads it: the "Death Star Shields:" counter and icons (`FUN_0051c0d0`, `FUN_004a7f50`, `FUN_005276d0`), AI facility sweeps (`FUN_004ebe40` and others), and the tactical hand-off `FUN_0040bcd0` → `FUN_00595ef0` → `FUN_005af4e0` (tactical Death Star `+0x70`, read by `FUN_005ada10` case 1). Bombardment, Destroy System and auto-resolve ignore it (`FUN_0058df60` subtracts only the `0x24` generators); nothing blocks the order.

## Tactical presence

`FUN_0040b0e0` exports a system's forces to a battle: `[0x14,0x18)` ships as full records (`FUN_0040b8a0`), `[0x18,0x1c)` as short records (`FUN_0040bcd0` → `FUN_00595ef0`: family, index, not-completed flag, shield flag, remaining hull), plus an uncompleted family-`0x18` product from the system's production manager (`FUN_00509670`, `FUN_0040d760`, `FUN_00506260`); `FUN_0040c780` imports the outcome. The tactical Death Star is one singleton (`FUN_00596bd0`); attackers of it must carry side `+0x168 == 1` (`FUN_005afe40`, `FUN_005c0b60`), and the trench-run result removes Alliance participants (`FUN_005d04e0`). hyp: `+0x74` of the tactical Death Star is "not completed" (sprite 5020 against 5010).

## AI

- **Build:** the AI ship builders (`FUN_004cd230`, `FUN_004d46e0`) take the player's class list (`FUN_0041b230` → `FUN_0042e3c0` → `FUN_0052e580`) and filter by AI class flag `0x200000`, type and budget (`FUN_0042e960`); no family-`0x18` exclusion. Whether the Death Star passes the filter is not traced.
- **Fire:** the posture emitter `FUN_004b1070` issues `0x223` (`FUN_004b1330`) when a need bit is set, the fleet record flag `+0x38 & 0x10` is set and the target record `+0x110` is above 0; the gate `FUN_00582440` then requires the ship. Where `0x10` and `+0x110` come from is not traced, so whether the AI ever fires is open.
- **Sabotage:** the DS Sabotage planner `FUN_00482180` / `FUN_00482330` branches on `[0x18,0x1c)` (`FUN_00482560`).

## Sabotage

Mission family `0x6a` (DSSBMSTB, chance `FUN_0055c8d0` = (espionage + combat) / 2): `FUN_005744c0` accepts only targets in `[0x18,0x1c)` (refusal `0x40/0x29`); ordinary Sabotage `FUN_0056a110` refuses them (`0x40/0x28`). Success runs the target's slot `+0xac(6)` (`FUN_005746e0`): one success destroys the Death Star. Both sides can run it.

## Save

A saved Death Star is the generic capital-ship record (`capital-ship-save-layout.md`); no serializer branches on family `0x18`. A destroyed system carries bit 3 and the reason byte.

## A second superweapon

| Variant | Runs | Binds |
|---|---|---|
| New CAPSHPSD record, family `0x18`, Empire | gate, kill phase, cascade, victory, sabotage, build list, movement, tactical export, bit 11, AI hooks | one `[0x18,0x1c)` ship per system, shared with the Death Star (`FUN_00507ab0`); one tactical slot per battle; Destroy System is the only effect |
| Same, Alliance (`is_alliance` 1) | the same code | the tactical attacker literal `== 1` (`FUN_005afe40`, `FUN_005c0b60`); bit 11 effects assume an Empire weapon |
| Families `0x19`..`0x1b` | range checks | no object factory: creation fails |
| Families `0x14`..`0x17` | build list, movement | fail the Destroy System gate; no bit 11, no sabotage |
| A weapon that acts on a system without destroying it | build, move | needs a new order beside `0x223` |

## Mod surface

1. **Data only:** a CAPSHPSD record with family 24 and either side's flag builds, moves and fires Destroy System as the Death Star does.
2. **Per-system cap:** the literal 1 in `FUN_0040f340` is shared by other slots; patch the call in `FUN_00507ab0`.
3. **A different effect** needs a weapon test in `FUN_0058d440` and a new branch instead of `FUN_0058ce70`.
4. **An Alliance weapon in battle** needs the side literals in `FUN_005afe40`, `FUN_005c0b60`, `FUN_005d04e0` and the singleton `FUN_00596bd0`.
5. **More families** need factories for `0x19`..`0x1b` in `FUN_0051ebf0`; otherwise tell weapons apart by class index.
6. **Tuning from data:** support shifts (GNPRTB/SDPRTB), the `0x606` character roll, DSSBMSTB, the DEFFACSD shield record.

## Functions

| Function | Role |
|---|---|
| [FUN_005003d0](FUN_005003d0.c), [FUN_005001b0](FUN_005001b0.c), [FUN_00502db0](FUN_00502db0.c) | Ship iterators `[0x18,0x1c)`, `[0x14,0x18)`, `[0x14,0x1c)` |
| [FUN_005061f0](FUN_005061f0.c), [FUN_00506300](FUN_00506300.c), [FUN_00506260](FUN_00506260.c) | First / is-a Death Star |
| [FUN_0051ebf0](FUN_0051ebf0.c), [FUN_00540540](FUN_00540540.c), [FUN_00540500](FUN_00540500.c), [FUN_0053f100](FUN_0053f100.c) | Object factory registry |
| [FUN_0052e580](FUN_0052e580.c), [FUN_0051cb20](FUN_0051cb20.c), [FUN_00567880](FUN_00567880.c), [FUN_00530a20](FUN_00530a20.c) | Shipyard class list and side filter |
| [FUN_005385f0](FUN_005385f0.c), [FUN_0052e970](FUN_0052e970.c), [FUN_00551a20](FUN_00551a20.c), [FUN_005504d0](FUN_005504d0.c), [FUN_0054f5b0](FUN_0054f5b0.c) | Build order checks and category table |
| [FUN_00507ab0](FUN_00507ab0.c), [FUN_00550700](FUN_00550700.c), [FUN_00552d10](FUN_00552d10.c), [FUN_0040f340](FUN_0040f340.c) | One per system |
| [FUN_00500820](FUN_00500820.c), [FUN_004fd900](FUN_004fd900.c), [FUN_00555920](FUN_00555920.c), [FUN_0055d8c0](FUN_0055d8c0.c) | Speed, hyperdrive, travel |
| [FUN_00582440](FUN_00582440.c), [FUN_0058d990](FUN_0058d990.c), [FUN_0058d440](FUN_0058d440.c), [FUN_0058ca70](FUN_0058ca70.c), [FUN_0058ce70](FUN_0058ce70.c), [FUN_0058d770](FUN_0058d770.c) | Destroy System gate and kill phase |
| [FUN_00507f10](FUN_00507f10.c), [FUN_005073d0](FUN_005073d0.c), [FUN_00537180](FUN_00537180.c) | Target and move refusals |
| [FUN_005080d0](FUN_005080d0.c), [FUN_004f71d0](FUN_004f71d0.c), [FUN_004f7560](FUN_004f7560.c), [FUN_00510950](FUN_00510950.c), [FUN_004fb520](FUN_004fb520.c), [FUN_004f8710](FUN_004f8710.c), [FUN_00524fb0](FUN_00524fb0.c), [FUN_004ef2e0](FUN_004ef2e0.c), [FUN_00503e20](FUN_00503e20.c), [FUN_004ef6d0](FUN_004ef6d0.c) | Destroy cascade |
| [FUN_005178b0](FUN_005178b0.c), [FUN_0050a890](FUN_0050a890.c), [FUN_00514a60](FUN_00514a60.c), [FUN_00518690](FUN_00518690.c) | Bit 11 |
| [FUN_005744c0](FUN_005744c0.c), [FUN_0056a110](FUN_0056a110.c), [FUN_005746e0](FUN_005746e0.c), [FUN_0055c8d0](FUN_0055c8d0.c) | Death Star sabotage |
| [FUN_0051c0d0](FUN_0051c0d0.c), [FUN_0040bcd0](FUN_0040bcd0.c), [FUN_0040b0e0](FUN_0040b0e0.c), [FUN_0040c780](FUN_0040c780.c), [FUN_005ada10](FUN_005ada10.c) | Shield facility, battle export and import |
| [FUN_005afe40](FUN_005afe40.c), [FUN_005c0b60](FUN_005c0b60.c), [FUN_005d04e0](FUN_005d04e0.c), [FUN_00596bd0](FUN_00596bd0.c) | Tactical side literals and singleton |
| [FUN_004b1070](FUN_004b1070.c), [FUN_00433780](FUN_00433780.c), [FUN_0042e960](FUN_0042e960.c), [FUN_00482560](FUN_00482560.c), [FUN_004739b0](FUN_004739b0.c) | AI |
| [FUN_005617b0](FUN_005617b0.c), [FUN_005070d0](FUN_005070d0.c), [FUN_0055f650](FUN_0055f650.c), [FUN_00560d50](FUN_00560d50.c) | Emperor-at-Coruscant flag (family `0x34`) |

## Open questions

- The refusal a second family-`0x18` ship at one system raises (`FUN_00550bd0`, `FUN_0054fd90`).
- Where the AI's fleet flag `0x10` and target `+0x110` come from, and whether the AI class flag `0x200000` admits the Death Star.
- What clears a destroyed system's populated bit (`FUN_0050b760`, `FUN_0050a3c0`, `FUN_00508660`).
- Two tactical Death Stars in one battle (`FUN_00596bd0`, `FUN_005a9330`).
