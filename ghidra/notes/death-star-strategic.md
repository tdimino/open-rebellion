---
title: "Strategic Death Star: Family 0x18, Destroy System, the Cascade and a Second Superweapon"
description: "How the strategic Death Star is identified (CAPSHPSD family 0x18), built, moved, fired and sabotaged, what destroying a system does to fleets, troops, facilities, characters, missions and builds, what victory and the AI read, what a second superweapon runs through or hits, and the port"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-17
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
| A fleet "has a Death Star" when a child is in `[0x18,0x1c)` (`FUN_005003d0`, `FUN_00582440`, `FUN_00507ab0`, `FUN_0040b0e0`). The AI fleet record flag `0x10` (`+0x38`, read by `FUN_004b1070`) comes from a child ship record whose class flag word has bit `0x80`, which `FUN_004739b0` sets for families `0x18`..`0x1b` (AI section) | | literal |

## Construction

- A shipyard (production family `0x28`) lists a class when its production range `[40,41)` holds `0x28`, the side flag matches (side 1 `is_alliance`, side 2 `is_empire`), `research_order` is at or below the side's level and the refined cost is non-zero (`FUN_0052e580` → `FUN_0051cb20` → `FUN_00567880`; side key `FUN_00530a20`). The Death Star's `research_order` is 0 and its cost 584, so every Empire shipyard lists it from the first day. No research, date or story gate.
- Build order `0x211` (`FUN_005385f0`, class vtable `0x006611a0` slot 20): product family `0x10`..`0x3f` (`FUN_0052e970`; error `0xf3` when maintenance exceeds spare capacity, the Death Star's maintenance is 600); a code-built range table puts `[0x18,0x1c)` in its own category; a family outside all nine categories gives error `0x19` (`FUN_00551a20`, `FUN_005504d0`, `FUN_0054f5b0`).
- **One per system.** System slot `+0x80` (`FUN_00507ab0`; vtables `0x0065e640`, `0x00663698`, `0x00664098`) answers a room query for a product family range inside `[0x18,0x1c]`: it counts `[0x18,0x1c)` ships over the system's fleets and reports room `FUN_0040f340() - count` in its third output, and writes the status pair `{0x90, 3}` as the refusal that room overdraft carries. `FUN_0040f340` returns the literal 1 (a stub shared by 230 other call sites and data slots). The count has no side filter (`FUN_00513050` sets `+0x14` to 0, `FUN_005131d0` skips the side test) and uses mode 3 (`+0x50` bit 6, existing: created and not destroyed), so a product counts from its order, and a Death Star in an enemy fleet at the system also blocks.
- **Where it binds.** The room is consumed by the order-leg builder, not by a menu predicate: `FUN_00552d10` / `FUN_005513a0` / `FUN_00551270` fetch it, and the placers `FUN_00551630`, `FUN_00551770`, `FUN_00551900`, `FUN_00551a20`, `FUN_00551b50`, `FUN_00551c80`, `FUN_00551dc0`, `FUN_00551ee0`, `FUN_00552000` run it through `FUN_00552dd0` → `FUN_00550eb0` / `FUN_00550fb0` → `FUN_00550bd0`, a loop over nine request categories whose combiner `FUN_0054fd90` (over `FUN_0054fca0`: room plus or minus the request, negative room copies the stored status) fails the leg when a category goes negative. So a Build (`0x211`, `FUN_00551a20`'s category), a Move of a fleet carrying a second hull (`FUN_00551630`) and the retreat chain's candidate test (`FUN_005571c0` → `FUN_00551630`) all meet it at the destination system. The refusal pair is `{0x90, 3}`; it has no text; the Empire advisor `FUN_004c1550` maps it to table slot `+0x2c`, and the Alliance `FUN_004c3750` has no `{0x90,3}` case and plays the generic reaction.
- Build time, completion and delivery are the ordinary path (`build-delivery.md`); no family-`0x18` branch. The ship is created at order time inside the destination system's spare fleet and completes in place (`death-star-product.md`).
- Not seeded: no CMUN table holds record 136, though the seeder's capital-ship test is `[0x14,0x1c)` (`FUN_0051aa50`, `FUN_0051b1f0`, `FUN_0051bf30`). No story or mission event creates it.

## Movement

- Ship speed is the class hyperdrive (80) minus the damage term, else `hyperdrive_if_damaged` (140), else GNPRTB 1 (100) (`FUN_00500820`, `FUN_005011f0`, `FUN_00501230`); the fleet takes its slowest ship (`FUN_004fd900`, through `FUN_00502db0`). Travel time is distance over speed scaled by GNPRTB `0x1400` (`FUN_0055d8c0`). A move between systems with speed 0 is refused 1/`0x18` (`FUN_00555920`).
- Split, join, Create Fleet and the other ordinary orders use `[0x14,0x1c)`; the Death Star counts as one ship in the blockade formula (`FUN_0055a020`).
- A fleet may move to and stay at a destroyed system; regiment landings and mission team `0x241` are refused there (`FUN_00537180`, `FUN_005073d0`).

## Fire: Destroy System (`0x223`)

- **Gate** `FUN_00582440`: each team fleet needs a bombard-strength sum (slot `+0x1dc`) above 0 and, for `0x223`, a usable ship in `[0x18,0x1c)`; one passing fleet is enough, and an inactive fleet (`+0x50` bit 0 clear) is skipped. No side, shield, ion or charge test.
- **Menu:** `0x223` is in every fleet's order list, always listed (`FUN_0040f340`) and enabled only when `FUN_00581e80` → `FUN_00553770` and `FUN_00582440` pass (`FUN_004ff8e0`, `FUN_0050f0b0`, `FUN_005827e0`).
- **Target:** the only refusal is an already destroyed system (1/`0x22`, `FUN_00507f10`, `FUN_00507fc0`). Own, neutral, enemy, the Alliance HQ, Coruscant and the last system are all legal.
- **Kill phase:** `FUN_0058d990` walks the usable fleets with order bit `+0x58` bit 6 and their usable ships (mode 1, `FUN_0058ca70.c:26-34`); its callback `FUN_0058d440` reports a ship in `[0x18,0x1c)`; `FUN_0058ce70` then destroys the system through slot `+0xac` with reason byte `0x10` when the system's holder is the bombarder, else `0xf`. The defence phase `FUN_0058d770` still runs afterwards.

## What destroying a system does

Order of effects, from system slot `+0xac` (`FUN_005080d0`; views `FUN_0050ff50`, then `FUN_004f8660` → `FUN_004f71d0`):

1. `FUN_004f71d0` stores the reason at `+0x40` and calls slot `+0x100` (`FUN_004fb200` → `FUN_004f7560`), which sets `+0x50` bit 3 and calls hook `+0x11c`.
2. Hook `+0x11c` (`FUN_00510950`) runs `FUN_004fb520`. Its first call is slot `+0x88` (`FUN_004f7bc0`: existing = created and not destroyed, so 0), which clears `+0x50` bit 6 through `FUN_004f76b0`. That change calls hook `+0x128` (`FUN_005109a0`): event `0x300` (`FUN_004fb810`), then `FUN_0050b760`, which clears the populated bit (below).
3. `FUN_004fb520` calls slot `+0xb4(1, ctx)` on every child of the system (`FUN_004fcd00`, mode 3). A block guarded by an argument the decompile shows as `unaff_retaddr` then clears the system's own `+0x50` bits `0x20`, `0x800` and `0x1000` (`FUN_004f7640`, `FUN_004f7870`, `FUN_004f78e0` with 0).
4. `FUN_00510950` raises event `0x319` (`FUN_005650f0`: observation level 11 for both sides, so each side's view of the system takes the destroyed state).

| Child | Slot `+0xb4` | Result |
|---|---|---|
| Fleets (vtable `0x0065d438`) | `FUN_00524fb0`: returns 1, does nothing | **survive**, any side, with their ships, fighters, regiments and characters; the Death Star's own fleet survives |
| Regiments, capital ships and fighters held by the system, the HQ, defence and production facilities | `FUN_004f8710` | completed (`+0x50` bit 2) and not en route (bit 5, `0x20`): destroyed with the system's reason (`FUN_004f7ed0`). Uncompleted or en route, and not destroyed: the autoscrap request (`+0x50` bit 12, `FUN_004f78e0(1)`), consumed by event `0x307` (below) |
| Special forces | `FUN_00503e20` | not en route and not destroyed: destroyed with the system's reason. En route: left alone. The autoscrap branch of `FUN_00503e20` runs only for argument 0; the cascade passes 1 |
| Characters at the system | `FUN_004ef2e0` | on a mission (`+0x78` bit 7) or destroyed: untouched. Container reason byte `0x14`: autoscrap request. Otherwise adrift (`+0x78` bit 6) |

- **Character roll** in `FUN_0058d990`: only minor characters (`FUN_0056f3d0`, families `0x38`..`0x3b`) that are direct children of the system, not aboard a fleet, and exist (`+0x50` bit 6, iterator mode 3) and are not in transit (`+0x50` bit 5 clear) are rolled against the bombardment probability (`FUN_0055e9e0`, GNPRTB `0x606`); a hit runs slot `+0x2f0(1, 10)` (`FUN_004ef6d0`, injury) and slot `+0xac(10)`, which always destroys the character (bit 6 already holds). Major characters are not rolled. A destroyed system refuses the order (1/`0x22`, `FUN_00507f10`), so the roll cannot run there again.
- Resources are not written by the chain. The holder goes neutral through hook
  `+0x20c` once the populated bit clears: `FUN_0050b610` rewrites the control
  word `+0x84` (bits 2-3 take 3, bits 4-5 follow the system's usable regiments,
  `FUN_0050b610.c:26-70`), `FUN_0050a300` stores it and event `0x31b` carries
  the old and new words (`FUN_00510d70.c:27`), whose holder change writes the
  owner bits (`FUN_004f6f40`, `FUN_00510d70.c:19`). The shift record is
  `support-shifts.md`, "Destroy System's kill"; the system's regiments
  choose it.
- Readers of the destroyed bit: victory, the move, landing and bombard refusals (`FUN_00537180`, `FUN_005073d0`, `FUN_00553410`, `FUN_00507f10`), mission end rules 2 and 4 (`FUN_00523450`, `FUN_00592600`), the blockade side flip (`FUN_00515f40`, `FUN_00561280`) and the reason counters (`FUN_00508c60`: reasons `0xf` and `0xe` feed the blockade loss counters).
- **Interface readers of the destroyed bit** (`+0x50` bit 3 of the player's view of the system, `FUN_004f2d10` / `FUN_004f3220`; event `0x319` writes both side views at once). Three places draw the state, and each keys on that bit:
  - *Galaxy view marker* `FUN_0042b330` (`UpdateSystemMarker`; marker items in list `+0x484`, display mode `+0xc8` = the GID command id, `0x11` Popular Support). A destroyed system draws the white cross, the same art as an unexplored system: in every display mode except `0x20`, `0x21`, `0x22`, `0x43`, `0x44`, `0x92`, `0x93` and `0x80`, the destroyed test at `0x0042b4eb`-`0x0042b4f3` reaches the exit at `0x0042b4f5`, which sets marker state `+0x3c` bit `0x80` and clears the mode art (`FUN_0060bd20(item, 0, ...)`, `0x0042c1c3`); the marker paint `FUN_0042cde0` (`0x0042cf1c`-`0x0042cf39`) draws view sprite `+0x2c4`, STRATEGY 10158, for a marker with bit `0x80`. The same exit takes a system whose view `+0x88` bit 1 is clear (set, in mode `0x17`; `0x0042b4d0`-`0x0042b4e9`). By command id: modes `0x22` and `0x44` skip the exit and draw normal art for a destroyed or unexplored system alike; modes `0x20`, `0x21`, `0x43`, `0x92` and `0x93` bypass the destroyed test (destroyed: normal art; unexplored: white cross); mode `0x80` returns before any marker update (`0x0042b330.c:71`); every other mode draws a destroyed system as the white cross. No mode leaves a destroyed system with no marker. In the original, destroyed Yaga Minor shows 10158 under Popular Support (days 240-280) and a large green marker for the two Imperial fleets in orbit under Idle Fleets (day 315).
  - *Sector window planet item* `FUN_0045bbb0`, called from `FUN_0045b770`: status `0x40`, STRATEGY 10240 (the asteroid field), name colour `0x2ffff00`, and no HQ overlay, flame or status bars (`sector-window-hit-test.md`).
  - *The sector window's three per-system child controls*, keys `(id << 6) | 1`, `| 2` and `| 3`: the Alliance and Empire popularity bars (`FUN_0045c240`, `FUN_0045c450`) and the bar `FUN_0045baf0`. Each tests the bit and, when set, calls its control's slot `+8` (the hide that pairs with the slot `+4` show of command 5 in the other branch), so a destroyed system carries none of the three (`FUN_0045c240.c:28,76`, `FUN_0045c450.c:28,76`, `FUN_0045baf0.c:12,27`).
  - Everything else in the interface reads contents: the quadrant icons (`FUN_0045ccc0`, `FUN_0045cdc0`, `FUN_0045ce80`, `FUN_0045d090`, `FUN_00459e30`) show by what the system holds, so the surviving fleets keep their icons; the System Defenses and Missions windows title themselves with the system name (`FUN_004f62d0`, `sector-quadrants.md`) and the mission target picture is the ordinary planet picture (`FUN_0045c970(FUN_00509610)`); `FUN_0046a9c0`, `FUN_004a10a0` and `FUN_0045fa60` carry no test of the bit. Scans of `0x00401000`-`0x00600000` for `+0x50` tests of mask `0x8` add three more readers of a system's view: the message filer `FUN_00488de0` (case `0x14f`, the system's Bombard notification with value 0, files class `0xf` and class 0, no message, when the view is destroyed; `FUN_00488de0.c:176-188`) and the advisor scripts `FUN_004c19f0` and `FUN_004c3bb0` (`.c:328`, which set a step deadline on a destroyed view; hyp). The object-level readers in that range are the facility lists (`FUN_004568a0`), the Personnel Finder row (`FUN_00465bb0`) and the battle-result rows (`FUN_0049b720`).
  - *Text.* TEXTSTRA 6451 "Destroyed" is a column header of the battle-result tables (`FUN_0044c630`, `FUN_0046f360`, beside `0x1932`); 20501 "Planet Destroyed" and 20511 "Rebel HQ Destroyed" are message-type labels (`bombardment.md`) with no code immediate. No system window carries a "destroyed" string.
- Support shifts follow `support-shifts.md`: galaxy-wide core -10 and rim -2 for the bombarder's side, halved toward the Alliance when system `+0x88` bit 11 is set. The destroyed system's own -20 (GNPRTB 7705) is nil, since `FUN_0050c9f0` skips a system whose populated bit the cascade has cleared.
- **Bit 11, "a Death Star is in the sector":** `FUN_005178b0` marks every system in the sector of each usable `[0x18,0x1c)` object (`FUN_005061f0`, `FUN_00518690`, setter `FUN_0050a890`); `FUN_00514a60` reruns it when such an object changes container, `FUN_00514510` on event `0x301`. The bit is side-neutral; its effects branch by side (`support-shifts.md`, `uprising-incident.md`).

### The populated bit (system `+0x88` bit 0), bits 1 and 3, reason `0xe`

- **Clearing.** `FUN_0050b760` (called by `FUN_005109a0` while the game runs, `DAT_006b90e0`, and by `FUN_00508660` when a child in families `0x20`..`0x2f` changes) has two branches: an unpopulated, undestroyed system takes the populated bit from whether it holds any child in `[0x20,0x30)` (`FUN_00539d70`, `FUN_005131b0`); a destroyed system gets `FUN_0050a3c0(this, 0)`. `FUN_0050a3c0` is the setter (`FUN_0053a640(1, v, +0x88)`, both side views `FUN_0050fbf0`, hook `+0x20c`). The existing-bit change in step 2 reaches it, so the bit is clear once the cascade has run.
- **Hook `+0x20c`** (`FUN_00510e30`, with the bit now 0): `FUN_0050b610` (the `+0x84` state word takes 3 for an unpopulated system), `FUN_0050b5a0` (the derived value `FUN_0050a290` takes 0 for a system that is unowned or unpopulated), `FUN_0050ac10(1)` (`+0x88` bit `0x100000`), `FUN_0050b890`. Their field meanings are not read.
- **Readers that follow the clear:** mission rule 7, below (container not populated), the blockade rule (`FUN_0050b8e0`: a populated system or one whose side is not 3 can be blockaded), regiment landing (`FUN_005073d0`), the destroyed-system planet picture (`sector-window-hit-test.md`).
- **Bit 3 (`+0x88` `0x8`) and bit 1 (`0x2`)** are set together by system slot `+0xbc` (`FUN_005081f0` → `FUN_0050a430` bit 1, view `FUN_0050fc80`, hook `+0x210`; `FUN_0050a510` bit 3, view `FUN_0050fda0`, hook `+0x218`; `FUN_0051ebb0` returns 1). Callers: the Populate methods (`FUN_00566d40` for core systems, a second one at `0x0056a573` for the other classes). `FUN_00566d40` ends with `FUN_0050a510(0)`, and `FUN_00566de0` (the facility seeder) clears it as well. While bit 3 is set, `FUN_0050b890` (called from hook `+0x20c`) rewrites it to "not (populated and side not 3)". Bit 3 is a setup guard that the seeders clear, and the cascade neither sets nor reads it. Bit 1 stays set after setup; readers: mission creation (`FUN_00592600`, column 12: a system container needs bit 1, else `0x40`/`0x11`) and the landing test (`FUN_005073d0`: a side-3 system with bit 1 set and unpopulated). hyp: bit 1 means "seeded".
- **Reason `0xe`** is written by three callers of slot `+0xac(0xe)` in the `0x0058b000` range (`FUN_0058c580`, `FUN_0058bec0`, `FUN_0058c3c0`; hyp: ground-combat kills, `support-shifts.md`). `FUN_005081f0` carries no reason.

### Adrift characters

- **Setter.** `FUN_004ef2e0` → `FUN_00534790(1, ctx)`, the Adrift role setter (bit 6 of the role byte `+0x78`). It runs only while `FUN_0053a000` holds (`FUN_0053a000.c:4`: `+0x24 & 0x30 == 0`; bits 4-5 number the object's copy, 0 the master and 1 and 2 the side views, so the guard holds for every master object, which is what the cascade's child walk visits; every setter opens with the same test and copies its write to the two views), writes `FUN_0053a640(0x40, v, +0x78)`, copies the bit into both side views (`FUN_00535c00`, whose slot `+0x2bc` is the trace logger `FUN_00536ac0`, `RoleAdriftNotif` → `FUN_004f8880`, gate `FUN_006158b0` returns 0) and calls slot `+0x264` (`FUN_00536460`, entry `0x0065ccd4` of the character vtable (`FUN_004f1440`, an undefined thunk whose call at `0x004f144e` is `FUN_00536460`) and `0x0065e3c4` of the special-force one), which posts event `0x30e` through `FUN_0053f9c0(0x30e, character, new bit, ctx)` on every change of the bit, set and clear alike. The event class is built by `FUN_00541110` (registered in `FUN_0051ef80`, vtable `0x00661b18`, `0x54` bytes, priority `0x200` from `FUN_0048b450`). The setter returns the `FUN_0053a000` truth (1 for a live object) and raises no message, window or sound.
- **Consumer.** The sweep `FUN_00548e90(location)` collects the characters with `+0x78` bit 6 held by the location and its fleets and ships, moves each into the system (slot `+0xa8(system id)`), and, unless the location is a system whose battle bit (`+0x88` `0x1000`) is set, runs `FUN_00549230` until no group remains. Its triggers: (a) event `0x304` with flag 0 (factory `FUN_00563c80`, execute `FUN_00563dd0` → `FUN_00515dc0` → `FUN_00545950`), raised by hook `+0x23c` (`FUN_00511780`) whenever the system's battle bit changes (`FUN_0050a900`, called by the blockade rule `FUN_0050b8e0`); a flag of 1 only records `FUN_00506e10 +0x70 = 1`; (b) a character's autoscrap request (event `0x307`, `FUN_00515480` branch 2 → `FUN_00545df0` → `FUN_00548da0`, which sweeps the character's own location); (c) event `0x30e`, posted by the setter above: execute `FUN_00541270` (`+0x50` is the new bit) → `FUN_00515970` (a live object of families `0x30`..`0x3f`; the mission manager is root `+0xb8`, `FUN_00506e40`) → `FUN_00545df0(manager, id, bit, ctx)`: a bit of 1 runs `FUN_00548da0` (the character holds `+0x50` bit 12 or `+0x78` bit 6, so its location is swept), then for either value the story-flag chain `FUN_005438a0` / `FUN_00549910` (the six named characters only).
- **When a Destroy System sweeps its adrift characters.** `FUN_004f7560` sets the system's destroyed bit before hook `+0x11c` runs the cascade, so each character `FUN_004ef2e0` flags already sits in a destroyed container, and each flag posts event `0x30e`. The event queue serves them in the same frame step as the order (`blockade-bit.md`, "The event queue and the order of work"), ahead of event `0x311` (priority `0x400`), so the destroyed system is swept the day of the kill:
  1. Battle bit clear: the first `0x30e` sweeps every flagged character at the system and `FUN_00549230` dispatches the groups (next bullet). The later `0x30e` events find the flags cleared, and each clear of the bit posts event `0x30e` with bit 0, which runs only the story chain.
  2. Battle bit set (fleets of both sides present): the sweep gathers the characters into the system and skips the dispatch. The cascade's recomputes (hook `+0x20c`, `FUN_00510e30` → `FUN_0050ac10(1)` → event `0x311` → `FUN_0050b8e0`, and the holder's change, side-bits hook `+0xf0`, `FUN_00510820`) leave the bit set: the destroyed system's holder is neutral, side 3 (`FUN_0050b610`: bits 2-3 are 3 once the populated bit is clear, and `FUN_0055a080` never gives 0), and `FUN_0050b8e0` treats side 3 as held, writing the battle bit while both sides are present. The characters stay at the system, still flagged, until a side's fleets leave and the bit clears (event `0x304` with flag 0), whose sweep dispatches the group.
  The cascade writes no `+0x88` `0x1000` directly.
- **What the sweep does** (`FUN_00549230`, `FUN_0054c7d0`). A flagged character (`+0x50` bit 12 or `+0x78` bit 6) not on a mission, whose container is destroyed or of another side (`+0x24` bits 6-7), is grouped by (side, adrift flag, destination); a character holding a captor id (`FUN_004ed270`, `+0x54` → `+0x30`) is skipped and stays flagged. Every other flagged character loses both flags (`FUN_004f78e0(0)`, `FUN_00534790(0)`), so one at its own side's intact system stays there unflagged (`FUN_0054c7d0.c:79-80,142-155`). The destination is `FUN_00543040`: the origin `FUN_005431e0` (the character's location when that is a system, else `+0x74` when that is a system (the system of the container it joined its mission from, `mission-return.md`), else Yavin `0x92000121` for side 1 and Coruscant `0x90000109` for the others) goes to the retreat chain (`FUN_005545e0`, candidate test `FUN_005571c0` with container acceptance `FUN_00553410`, which refuses a destroyed system with `0x22`); a character whose mission key is in family `0x65` (Bounty) takes Jabba's Palace `0x98000481` instead. Each member's request bit and adrift bit are cleared. A group with a non-captive member (slot `+0x1d4` is the prisoner bit, `FUN_004edc70`) starts the hidden mission Autorouting `0x43000003` (`FUN_00547740`, adrift flag 0) or Adrift `0x44000004` (`FUN_00547880`, adrift flag 1) through `FUN_00547ba0`, whose target may be a system `0x90`..`0x97` or an abode `0x98`..`0x9f`; a group of captives with a destination goes through `FUN_00549850`, and a group with no destination goes character by character through `FUN_00549580` (both in "Jail abodes" below). The Adrift mission's mode word is `0x4103` (`FUN_00576a20`); both missions are hidden in MISSNSD (TEXTSTRA 11267 "Autorouting", 11268 "Adrift"). The sweep scans the direct character children of the location and the characters aboard its fleets and ships; a location that is not a system holding the battle bit is dispatched at once, so an abode location (family `0x98`) is swept like a system (`FUN_00548da0.c:22-33`, `FUN_00548e90.c:122-141`; `jail.md`, section 5). A special force (family `0x3c`..`0x3f`) in the scan skips the captor test, which covers families `0x30`..`0x3b` only (`FUN_0054c7d0.c:58-74`), so it joins a group as a free member. At `FUN_00549580` its capture slot `+0x20c` (special-force vtable `0x0065e160`) is `FUN_00503eb0`, which calls `FUN_00524fb0` and then slot `+0xac(9)`: a special force with no destination is destroyed with reason 9 and never reaches a jail (its prisoner slot `+0x1d4` is `FUN_006158b0`, 0, and slot `+0x218` is `FUN_0051ebb0`, a no-op; `FUN_00549580.c:56-60,96-99`).
- **Readers of the bit.** A scan of `0x00401000`-`0x00600000` for byte tests of `+0x78` (`TEST` and `AND` with `0x40` or `0xc0`, `SHR 6`) finds the sweep (`FUN_00548da0`, `FUN_00548e90`, `FUN_0054c7d0`), the setter's view copy (`FUN_00535c00`) and five predicates: `FUN_005439b0` (a character is free: not a prisoner, active, no `+0x50` `0x1000`, not adrift; `FUN_005439e0` adds "not on a mission" and serves the story checks `FUN_00544030`, `FUN_005443f0`, `FUN_005444e0`, `FUN_00544da0`), `FUN_005442f0` and `FUN_00544da0` (mask `0xc0`, adrift or on a mission, `story-missions.md`), `FUN_00534fa0` (placed: not adrift, in a container that exists and is intact; reached by `FUN_004efd30` and, through two thunks, by the special-force slot `+0x20`, entry `0x00503d70`) and `FUN_00536070` (slot `+0xf4`, called through `FUN_004f11c0` for the majors: the location test of the object pass is skipped for an adrift character). The iterators (`FUN_004f6b90` modes 1-4: active, en route, existing, all) and the list builders read none of these.
- **Display.** No interface code reads the bit, so an adrift character carries no icon, text or message. The System Defenses window lists it with the system's other personnel (the Personnel tab and quadrant icon filter on the visible-mission test only, `sector-quadrants.md`), and Autorouting and Adrift are hidden missions (`FUN_00520b70`), so the Defenses window keeps listing a character on one (`personnel-finder.md`). The Personnel Finder row (`FUN_00465bb0`) builds its state word from four tests: captured (`+0xac` bit 0, TEXTSTRA `0x8746`), on a mission (container family `0x50`..`0x7f`, `0x8745`; the hidden missions `0x43` and `0x44` sit below it), injured (`+0x94`, `0x8747`) and en route (`+0x50` bit 4, `0x8744`), so a travelling adrift character reads "Enroute".

### Jail abodes (family `0x98`)

- **Class.** `FUN_00519910` (new-game stage 9) creates the five ABODESD records through `FUN_004f7d50(root, id, owner, ...)`: `0x98000440` Alliance Jail (owner 1, root `+0xcc`), `0x98000441` Dagobah (3, `+0xd0`), `0x98000442` Hermitage (3, `+0xd4`), `0x98000480` Empire Jail (2, `+0xd8`) and `0x98000481` Jabba's Palace (2, `+0xdc`); each ABODESD record's name field gives its TEXTSTRA name: 11600 Alliance Jail, 11601 Dagobah, 11602 Hermitage, 11616 Empire Jail, 11617 Jabba's Palace (DAT). Family `0x98` maps to factory `FUN_00577530` (`FUN_0051ebf0.c:5`) → constructor `FUN_00577050` → vtable `0x00669260`, `0x58` bytes on the container base `FUN_004f5ea0`: the character-group class that also holds a mission's team (`rebexe-map.md`). Its slot `+0x20` (`FUN_005775a0`) accepts a group whose children are all families `0x30`..`0x3f`, off a mission (`+0x78` bit 7 clear) and of the group's own side (`+0x24` bits 6-7); the room query at slot `+0x78` (`FUN_005773b0`) takes only family ranges inside `[0x30, 0x40]`, and slot `+0x74` (`FUN_00577270`) passes order `0x240` for those families into `FUN_00553410`. The root has a getter for the Palace alone (`FUN_00506ed0`); the jails are reached by id.
- **Entry.** The ids `0x98000440` and `0x98000480` appear in code at `FUN_00519910` (creation), `FUN_00513a50` (root fields) and `FUN_00549580` alone. `FUN_00549580` takes a group to which the retreat chain gave no destination. For each member that exists, a free one is captured by slot `+0x20c` with a null captor (`FUN_004ef480` → `FUN_004ee3e0(·, 0)` writes the status short `+0x98` = 0) and a prisoner takes slot `+0x218` (`FUN_004ef5b0` → `FUN_004ee3e0(·, 2)`, status 2). Every status change runs slot `+0x334` (`FUN_004f18e0`): the prisoner bit `+0xac` bit 0 takes `status == 0` (`FUN_004eef70` → `FUN_004ee800`), the captor clears, the autoscrap request sets (`FUN_004f78e0(1)`), the remove request `+0x78` bit 2 sets when the character holds a mission key (`FUN_005345d0`) and event `0x30a` posts (`FUN_0057c8b0`; victory reads it for a major character, `victory-conditions.md`). The prisoner-bit hook (`FUN_004f1a80` → `FUN_004eec60`) gives a captured member its captor's side bits, the opposite of its native side, and gives a freed one its own back. `FUN_00543040` then runs again on the member's new side: a destination starts the hidden Autorouting there, a prisoner aboard as one. With no destination again, a member freed in this pass is captured again, and the abode is chosen from its **current** side bits (1 → `0x98000440`, else `0x98000480`), so a stranded Alliance character takes the Empire Jail; the request bit clears and `FUN_00547740` starts the hidden Autorouting to it with the prisoner flag (mission `+0xa4` bit 2, `FUN_00522210`; no reader in `0x00520000`-`0x005a0000`). The trip is 1 day: `FUN_00555b30` writes 1 tick between two keys that are not both systems, and `FUN_00556390` reparents the member into the abode at once (`jail.md`).
- **Captive group with a destination** (`FUN_00549850`): slot `+0x218` on each member that is a prisoner and exists, status 2, prisoner bit clear. The status handler raises the autoscrap request again, so the next sweep (event `0x307`, `FUN_00515480`) sends the freed characters as ordinary ones. That sweep is queued (priority `0x300`, "The autoscrap request"), so it runs after `FUN_00549850` and the whole dispatch return, with every freed character's request bit already set. The first `0x307` handler to run sweeps the whole location (`FUN_00548da0` → `FUN_00548e90`), so the freed characters at one location form one group; each later `0x307` event of the batch finds its character's bits clear and does nothing (`FUN_00548da0.c:22`).
- **Statuses.** Slots `+0x20c`, `+0x214`, `+0x218` and `+0x210` write `+0x98` = 0, 1, 2 and 3 (`FUN_004ef480`, `FUN_004ef570`, `FUN_004ef5b0`, `FUN_004ef510`; `FUN_004ef510` also stores the captor key through `FUN_004ef190`, as `FUN_004ef480` does). Callers: capture by Abduction and by an exposed member's defeat and by the sweep (0), the uprising's freed prisoner (1, `uprising-incident.md`), the sweep's captives and Luke after the Force test (2) and the Rescue and Palace releases (3). Only status 0 is a prisoner.
- **Held, shown, counted.** An abode occupant is a child of a family-`0x98` object, so no system list holds it. The Personnel Finder lists it as `<name> - <abode name>` alone: the family-`0x98` branch of `FUN_00465bb0` (`0x00465fed`; `FUN_004f62d0` of the container) skips the state and rank words, and its Display action handles fleet and system parents only (`FUN_00429440`, `personnel-finder.md`). Its prisoner bit still counts for the victory flags and the story checks (a prisoner counts whoever holds it, `victory-conditions.md`). No code names the jail ids as a source. An occupant leaves by escape: its prisoner bit arms timer `0x38a`, whose roll frees it (`jail.md`, "Escape"). A Rescue may legally target it, but neither the player's windows nor the AI planner reach it (`jail.md`, "Rescue targeting"); the other releases (slots `+0x210`, `+0x214`, `+0x218` by the Palace mission, an uprising or a sweep) name only the Palace abode (`0x98000481`, `FUN_0056df70`, `FUN_0056d4f0`).

### The autoscrap request (`+0x50` bit 12)

- **Set by a control change.** A system's holder change calls slot `+0xb0` on each character there (`FUN_004ef260`), which releases a prisoner and sets this bit when the container's side differs from the character's; the event `0x307` sweep then sends it to its own side's nearest system (`ground-assault.md`, section 4, "Characters").

- **Consumer: event `0x307`.** `FUN_004f78e0` sets the bit (both views `FUN_004faa10`, slot `+0x13c`); slot `+0x13c` is `FUN_004fba70` (or a fleet's `FUN_004ff690`), which raises event `0x307` (`FUN_0057b0a0` factory, handler `FUN_0057b1f0` → `FUN_00515480(obj, flag, ctx)`). Event `0x304` is the system battle-bit event above; its separate notification-name use is "DestroyedAutoscrap", the reason-`0x15` destroy (`FUN_004fc080`).
- **When.** Event `0x307` is queued: `FUN_004f78e0` raises it through `FUN_0053f9c0` → `FUN_0053fcd0` → `FUN_004fd390` → the callback `DAT_006b2af8` (`FUN_004fd010`, stored by `FUN_004fcde0`) → `FUN_0051d8f0`, which inserts it into event queue `+0xb0` (`blockade-bit.md`, "The event queue and the order of work"). Its class (`FUN_0057b0a0`, vtable `0x00669748`) has priority `0x300` (`FUN_0057b260`), so within the same frame step it runs after the queued events of priority `0x10` (mission validator `0x300`, holder change `0x31b`), `0x100` (support shift `0x373`) and `0x200` (adrift sweep `0x30e`), and before the blockade recompute `0x311` (`0x400`). One event per flagged child; a child destroyed before its event runs is skipped (`FUN_00515480.c:38-41`). `FUN_00515480` for a non-character: relocation through the candidate chain (`FUN_005545e0`); a refused status gives `+0xac(2)` (destroyed with reason 2); an accepted destination goes through `FUN_00556390` (en route bit, reparent), with reason 5 for a zero-speed object bound for another system. A flagged unit at a destroyed system finds its container refused (`FUN_00553410`, `0x22`) at stage 1, so it moves to the hint (the origin system for an uncompleted product, the transit origin for an en route unit) or the nearest accepting system of its side, else it is destroyed with reason 2.
- **Refunds and messages.** The half-cost refund (`FUN_00530270`) reads reasons `0x14`..`0x16` on a completed object, so reasons 2 and 5 refund nothing. The generic destroyed event `0x302` follows a destroy (`FUN_004fc080`); the text of reasons 2 and 5 is not traced. A relocated unit raises no message in the code read.

### Missions, builds and arrivals at a destroyed system

- **Mission creation.** `FUN_005073d0` refuses a destroyed target system with `1`/`0x22` for regiment landings (`0x204` over `[0x10,0x14)`) and for mission teams (`0x241`); it exempts fleet moves (`0x204` over `[0x08,0x10)`) and character moves (`0x240`). `FUN_00553410` returns `0x22` for any destroyed container.
- **Running missions** (`mission-lifecycle.md`): the existing-bit clear in step 2 raises event `0x300` with flag 0 (`FUN_004fb810`). The event is queued with priority `0x10` (`blockade-bit.md`, "The event queue and the order of work"), so the walk below runs after the whole synchronous cascade (step 3's child walk, the managers' product destruction) and before the adrift sweeps (`0x30e`, `0x200`) and the autoscrap relocations (`0x307`, `0x300`). Its fire slot `FUN_0057a730` → `FUN_005140c0` → `FUN_00545240` walks every mission. A mission whose container (`+0x74`) or target (`FUN_00520c70`, `FUN_00520cb0`) is the destroyed system runs the validator `FUN_00522480` at once, and a mission whose container is the system, with no end code and past its landing (phase above 6, `FUN_00520ae0`), then takes end code 7 through slot `+0x1dc` whatever its columns. Before the landing the validator's rules decide: rule 2 (column 11) ends with code 7 (`0x40`/`0x12`) when the container is destroyed; rule 4 (column 14) with code 6 (`0x40`/`0x21`) when the target is destroyed; rule 7 (column 13) with code `0xd` (`0x40`/`0x16`) when the container system is unpopulated. From the shipped columns: Diplomacy, Recruitment, Incite and Subdue end by rules 2, 4 and 7, Research by 2 and 4, Rescue, Abduction, Assassination, Sabotage and DS Sabotage by 4, Espionage and Reconnaissance by none.
- **Builds.** Queued products are real objects created at order time in the destination container (`death-star-product.md`, section 1). Their order is kept by the production manager (family `0xa0`..`0xaf`) in the key list at `+0x54 → +0x18`. Each system holds three managers, direct children created completed by the system initializer (`FUN_0050e200.c:24-38`: `0xa2000006` ships, `0xa4000007` troops, `0xa0000005` facilities). A completed shipyard or construction facility at the system is destroyed with it. What happens to a product depends on whose manager it belongs to and where it sits:
  1. *A product of a manager at the destroyed system* (any kind, any destination). The manager is a completed child, so `FUN_004f8710` destroys it with the system's reason (`FUN_004f7ed0`). The clear of its existing bit runs hook `+0x128` (`FUN_0052b7d0`, vtable `0x0065fbf8` slot 74), which calls `FUN_0052a2c0`: slot `+0xac(0x16)` on every product key in the queue, then `FUN_004f6f40(manager, 3)`. Each product is destroyed at once, inside the child walk, with reason `0x16`. The refund `FUN_00530270` pays a completed object only, so an unfinished product refunds nothing. The products' own autoscrap requests find them destroyed and do nothing (`FUN_00515480.c:38-41`).
  2. *A product of another manager whose container is the destroyed system.* A capital ship product sits in the destination's spare fleet (`death-star-product.md`, section 1), a fleet child, and fleets are not walked (`FUN_00524fb0`), so it is untouched and stays queued. A squadron, regiment or facility product is a direct child of the system: uncompleted and not destroyed, it takes the autoscrap request and runs through the chain with the hint `H` = the producer's system when that differs (`system-squadrons-combat.md`, section 3b, step 4). It keeps its queue slot: `FUN_00528890` accepts a queue entry that is created, not completed, destroyed or en route and whose `+0x38` key (`FUN_0048a640`) equals the manager's own id (`FUN_00528890.c:13-34`), and `FUN_004f6fd0` rewrites the container but not `+0x38`. The manager therefore completes it at its own pace, and the completion event (`FUN_0052bee0` → `FUN_004f74f0` → hook `+0x118` `FUN_004f2b10` → `FUN_004fb4b0.c:18`, event `0x308` → `FUN_005150c0.c:90-93` → `FUN_00556430(product, +0x38, own location)`) sends it from the yard to its new container. An uncompleted product is not deployed, so its relocation is a plain reparent with no transit (`FUN_00556390.c:12-15`). A special force product (`0x3c`): `FUN_00503e20` with argument 1 destroys it when it is not en route and not destroyed, with no completion test.
  3. *A completed product travelling to the system.* A capital ship is a child of the spare fleet and is untouched. A special force en route is left alone (`FUN_00503e20`). A squadron, regiment or facility en route takes the autoscrap request and relocates as in "A relocated unit's pending arrival", below.
- **Arrivals.** Fleets may move to and stay at a destroyed system (`FUN_00537180` refuses a destroyed destination only when it is not a system `0x90..0x97`); fleets already en route keep their legs, and the cascade cancels no order. Regiments en route are redirected by the request path; special forces have no leg of their own ("Arrival at a destroyed system", below). Characters on a Move mission arrive.
- **Blockade.** The cascade does not touch the blockade bit. The rule `FUN_0050b8e0` reruns on presence changes; a destroyed system of side 1 or 2 stays blockadable (the unpopulated test applies to side 3 only).

### Arrival at a destroyed system

Citations are `file:line` in this folder. Nothing in the leg start, the arrival event or the arrival handlers tests the destination's destroyed bit (`+0x50` bit 3), so no unit is lost, held or redirected because its leg ends at a destroyed system.

- **Leg start accepts a destroyed destination.** `FUN_00556430` tests the unit's slot `+0x38`, the autorouting bit 11 (`FUN_00556430.c:29-30`), that both keys resolve to systems that differ (`:31-51`), and the route builder `FUN_00555d30` → `FUN_00555b30` (the tick count from the two systems' coordinates only, `FUN_00555b30.c:80-112`). No destroyed test appears. Then it sets bit 5, stores the new place and `clock + ticks` (`:59-69`). code read
- **Arrival changes nothing about the destination.** Timer `0x387` → `FUN_0057bf80` → `FUN_004f7fc0` clears bit 11 → event `0x306` → `FUN_00515a50`. A non-character takes `FUN_00556620` (second leg when `+0x3c` differs from the location, `FUN_00515a50.c:51`, `FUN_00556620.c:31`), then `FUN_004f7640(unit, 0)` clears bit 5 (`FUN_00515a50.c:63`). The unit is already a child of its destination container; the arrival moves, destroys and flags nothing. code read
- **Capital ship** (a delivery leaving a yard). Deploy event `0x308` runs `FUN_00556430(ship, yard key at +0x38, own location)` for every family outside `0x30..0x3f` (`FUN_005150c0.c:90-102`), where the ship's location is the system of the spare fleet that holds it. The cascade leaves that fleet and its ships alone (fleets are not walked), so the leg to the destroyed system starts, ends on its day, and the ship stays in the fleet at the destroyed system, not en route, not destroyed. The post-leg check `FUN_00556710` tests the ship's own container, the fleet, which is intact (`FUN_005150c0.c:103-107`; `system-squadrons-combat.md`, `FUN_00556710` stage 1). code read for the leg and arrival, inference for the fleet staying intact (the cascade table above)
- **Special force** (a product delivery). No leg exists: `FUN_005150c0` sends families `0x30..0x3f` to `FUN_00545180` (`FUN_005150c0.c:76`), which raises the deployment notification `FUN_00547180` and, when bit 5 is clear, runs `FUN_00556710` (`FUN_00545180.c:23-27`); it never calls `FUN_00556430`, so a special force has no transit and no arrival. At a destroyed container that check calls slot `+0xb4` of the special-force table `0x0065e160`, `FUN_00503e20(0)`, which sets the autoscrap request (`FUN_004f78e0(1)`, `FUN_00503e20.c:12-15`) and does not destroy it. The request raises event `0x307` → `FUN_00515480` → `FUN_00545df0` → the adrift sweep, which sends it to the nearest accepting system of its side, or destroys it with reason 9 when no destination exists (`FUN_00549580.c:56-60`). An uncompleted special force product at the cascade is destroyed at once (`FUN_00503e20(1)`, reason of the system); a completed one not en route likewise. code read
- **Regiment and squadron in transit.** The cascade flags them (event `0x307`) and relocates them with their leg kept; they arrive on the original day, then run the second leg from the destroyed system (next section). A leg that starts after the cascade toward the destroyed system, with no relocation, ends with the unit sitting in the destroyed system, completed, bit 5 clear. Regiment landings at a destroyed system are refused at order time (`FUN_005073d0`, `1`/`0x22`), so only a fleet's cargo reaches one, and cargo has no leg of its own. A later presence check `FUN_00556710` stage 1 (battle recheck `FUN_00516ef0.c:129`) destroys a completed unit not en route in a destroyed container through slot `+0xb4` `FUN_004f8710` (`FUN_004f8710.c:11-12`). code read for the arrival, inference for the later destruction needing a battle at that system
- **Character in transit** (Move mission). Order creation refuses a destroyed target (`1`/`0x22`, `move-order.md`, "Route refusals"). A Move already under way when the system is destroyed: the cascade's mission walk runs the validator, which ends it with code 6 (target destroyed, column 14) or 7 (container destroyed, column 11) the same day. The member keeps bit 11 and its leg (`FUN_00556430.c:30` returns at once for bit 11), so the return path (`mission-return.md`, sections 2 and 3: the member's container `D` is refused with `0x22`, the stored origin container `+0x70` is accepted) reparents it toward its origin without cancelling the timer; it arrives at the destroyed system on the original day, `FUN_00522280` then runs `FUN_00556620` for each en-route member (`FUN_00522280.c:79-89`), which starts the leg from the destroyed system to the origin. inference (chain of code-read steps joined by one assumption: that the return placement reparents the member before the arrival event); needs VM
- **Port rule.** `FUN_004fc080` (`FUN_004fc080.c:45-54`) raises the destroyed-on-arrival notification for an object that is itself destroyed while en route, and tests no destination. The original loses no delivery at a destroyed destination; only a destination that no longer exists loses one. code read

### A relocated unit's pending arrival

A unit that travels by itself carries its leg in its own fields (`build-delivery.md`, "Travel"): autorouting bit 11, destination key `+0x3c`, arrival tick `+0x44` and a pending timer `0x387`; it is already a child of its destination. The autoscrap relocation of such a unit (a regiment, squadron or facility product en route to the destroyed system):

- `FUN_00556390.c:17-21` leaves the in-transit bit alone while bit 11 is set, then reparents the unit into the new container (slot `+0xa8`). `FUN_00514a60` → `FUN_00556430` returns at once for a unit with bit 11 set (`FUN_00556430.c:30`). Nothing cancels the timer (`FUN_004fba10` runs only at arrival) or rewrites `+0x3c` and `+0x44`: the unit keeps its original arrival day, and its destination key still names the destroyed system.
- On that day `FUN_0057bf80` → `FUN_004f7fc0` clears bit 11, whose hook `FUN_004fba10` raises event `0x306` → `FUN_00515a50` → `FUN_00556620`. That function reads the unit's location (slot `+0xc`) and `+0x3c`; when the two keys differ it starts the next leg `FUN_00556430(unit, +0x3c as from, location as to)` (`FUN_00556620.c:27-31`; the push order at `0x005566a0`-`0x005566a8` puts the `+0x3c` copy second and the location third). That leg sets bit 5, arms `0x387` with `FUN_0055d8c0(coords(destroyed system), coords(new place), speed)` ticks, stores the new place's system at `+0x3c` and `clock + ticks` at `+0x44`. When both ends resolve to one system no leg starts (`FUN_00556430.c:40-51`).

So the unit travels on to the destroyed system on its original schedule and then makes a second trip from there to the relocation target. An uncompleted product has no deployment and no bit 11: no pending leg, no later trip (`FUN_00556390.c:12-15`). Confidence: code read; not run in the VM. VM check: a regiment or squadron in transit to a system that then takes Destroy System; observe its row at the relocation target on the original arrival day (still "Enroute") and its arrival day there (original day plus the distance from the destroyed system).

### The HQ in transit

- `FUN_00520b60` returns a class record's `+0x58`; the HQ's slot `+0x1c0` is a thunk to it (vtable `0x0065f280` entry `0x00558050`), so the HQ's bombardment value is ALLFACSD record `0x20000001` field `+0x58` (`bombardment.md`). `FUN_00526860` runs `FUN_00526c00`, which builds the HQ order list `DAT_006b8ff8` (orders `0x201`, `0x202`, `0x204`; no Scrap), then `FUN_00520600` on it; it is not capture code. The capture case is `game-end-path.md`, "An HQ in transit".
- An HQ (family `0x20`, completed) that is en route (`+0x50` bit 5) at Destroy System takes the second branch of `FUN_004f8710`: not destroyed, autoscrap request set. `FUN_004f8680` (control change) has the same shape, so an HQ in transit survives both and is relocated by `FUN_00515480`; the HQ class serves only the Alliance (`facility-ownership.md`), so the chain's acceptance (`FUN_005571c0`) offers it Alliance systems. The in-transit HQ is a child of its destination (`move-order.md`), so a destination destroyed under it sends it to its transit origin or the nearest accepting Alliance system, else it is destroyed with reason 2. hyp: reason 2 on the HQ raises the same removal reactions as a cascade destroy (`game-end-path.md`).

- **Which sides are told.** Event `0x319` copies the destroyed state into both sides' views (`FUN_005650f0`, `observation-levels.md`). The Death Star movie and message come from the FleetBombard event `0x182` (`FUN_004950d0`, `movies.md`), which looks the fleet up in the receiving side's own view (`FUN_004f36a0(side, fleet)`): a side receives it when its view holds the fleet. hyp: the defender receives it only while it observes the Death Star's fleet.

## In the original (VM)

`empire-huge-deathstar-built.sav`, played on: Fleets 1 and 6 (Death Star 1)
moved from Coruscant to Alliance-held Yaga Minor (no shield, two Alliance
regiments, five civilian facilities), read from saves taken before and after
each order.

- With the Death Star in orbit, Destroy System is enabled in the Planetary
  Bombardment submenu beside the other three.
- General Bombardment first: both Alliance regiments and four of the five
  civilian facilities died; the alert "Orbital bombardment of Yaga Minor"
  opened at once.
- Destroy System on day 240 played the superlaser movie at once ("The Death
  Star has destroyed the Alliance system Yaga Minor."). After it: the system
  is an asteroid field, both Imperial fleets survive in orbit with their two
  capital ships, six fighter squadrons and five regiments, the last civilian
  facility is gone, and the system's holder reads neutral. Support moved as
  `support-shifts.md` records (the core shifted, the rim did not).
- The Empire's third top-bar counter read 309 before the order and −122
  after it; what it counts here is unread.

## Victory

Victory reads the Death Star only through the AD bit (Alliance HQ missing or destroyed) and the CD bit (Coruscant destroyed), both set by the cascade (`victory-conditions.md`, `game-end-path.md`). `FUN_005180e0` and the ending code test no side and no family `0x18`. Losing the Death Star does not end the game.

## Death Star Shield (facility family `0x25`)

DEFFACSD id 4: family 37, Empire only, cost 25, maintenance 20, `research_order` 3, built at the construction yard (`0x2a`). Only its family byte reads it: the "Death Star Shields:" counter and icons (`FUN_0051c0d0`, `FUN_004a7f50`, `FUN_005276d0`), AI facility sweeps (`FUN_004ebe40` and others), and the tactical hand-off `FUN_0040bcd0` → `FUN_00595ef0` → `FUN_005af4e0` (tactical Death Star `+0x70`, read by `FUN_005ada10` case 1). Bombardment, Destroy System and auto-resolve ignore it (`FUN_0058df60` subtracts only the `0x24` generators); nothing blocks the order.

## Tactical presence

`FUN_0040b0e0` exports a system's forces to a battle: `[0x14,0x18)` ships as full records (`FUN_0040b8a0`), `[0x18,0x1c)` as short records (`FUN_0040bcd0` → `FUN_00595ef0`: family, index, not-completed flag, shield flag, remaining hull), plus an uncompleted family-`0x18` product from the system's production manager (`FUN_00509670`, `FUN_0040d760`, `FUN_00506260`); `FUN_0040c780` imports the outcome. The tactical Death Star is one singleton (`FUN_00596bd0`); Empire (side 1) squadrons dock into its hangar (`FUN_005afe40`, `FUN_005c0b60`), and the trench-run result removes Alliance participants (`FUN_005d04e0`). `+0x74` of the tactical Death Star is "not completed" (ship `+0x50` bit 2 clear, `FUN_0040bcd0`) (sprite 5020 against 5010).

## AI

- **Class flags.** The AI's capital-ship class node (`FUN_004739b0`) sets flag `0x80` for families `0x18`..`0x1b` and skips the ordinary flags, so CAPSHPSD record 136 carries `0xbc0` (`0x80`, `0x100`, `0x200`, `0x800`, `0x40`) and no `0x100000`; every other capital ship carries `0x100000`, and fighters carry `0x200000` (`ai-production.md`, class-node flags). The fighter builders (`FUN_004cd230`, `FUN_004d46e0`) filter on mask `0x200000` through `FUN_0042e960` (class nodes of the requested type whose flag word meets the mask and whose cost fits the budget; keys and picks in `FUN_0042e210`, `FUN_0042e330`), so the Death Star is outside them. The ordinary capital-ship requests use masks built on `0x100000`, outside as well.
- **Build.** Module 11's request table has one row for the hull: side 2, more than one system in the module's list, no own Death Star (`c+4` bit 4 clear), mask `0x80`, key `0x200`, blocker `0x20000000` (`ai-order-emission.md`, module 11; `FUN_004ca5e0`). Mask `0x80` selects the Death Star's class node. `c+4` bit 4 is set by the fleet census when an own fleet record holds flag `0x10` (`FUN_0041ba80`, `ai-production.md`). Budget = (30·c+0x184/100)·90/100 − c+0x24c, 0 if above c+0x184 − c+0x188 (`FUN_004cc220`); `FUN_004ca910` accepts a class only if its maintenance (600) is at most the budget, which needs capacity of at least 2,223 plus 3.7 times the fleet maintenance (`death-star-product.md`).
- **Fleet flag `0x10`.** `FUN_00433620` rebuilds a fleet record and folds each child ship record into it with `FUN_00433780`, which sets fleet `+0x38` bit `0x10` when the child's class flag word (`+0x24`) has bit `0x80`: a fleet record carries `0x10` exactly when a Death Star hull is aboard. hyp: the child word is the class node's `+0x18`; the copy is in `FUN_00484630`.
- **System record `+0x110`.** The system census `FUN_00433e40` increments it for a non-own system whose record `+0x28` carries bit 31, and for each enemy character record with flags `0x20000000`, `0x40000000` and bit 3 set. Record `+0x28` bit 31 is set when the system id's low 24 bits are `0x109` (Coruscant) or the system holds a family `0x20`..`0x21` object of the record's side view (`FUN_00526a60`, the HQ). So the target needs to be Coruscant, an HQ system, or hold a flagged leader.
- **Fire.** The posture emitter `FUN_004b1070` acts on an arrived fleet whose record `+0x38` has need bit `0x20000000` or `0x40000000`: with fleet bit `0x10` and target `+0x110 >= 1` it issues `0x223` (`FUN_004b1330`) in place of an assault or bombardment. Need bit `0x20000000` is set by the assault planners (`FUN_004b6510`, `FUN_004b8ab0`) after the assault test `FUN_004ae240` accepts the fleet and target (system flag `0x20` and enough troop capacity); `0x40000000` is the bombardment need. The gate `FUN_00582440` then asks only for the hull.
- **Verdict.** By code the Empire AI can build the hull (module 11, row above) and fire it at Coruscant-class or HQ targets, so `or-q-ai-death-star-use-z9ow` reads yes: traced for each link except the budget and the planners reaching a fleet that carries the hull. Open: whether any shipped scenario run does it.
- **Sabotage:** the DS Sabotage planner `FUN_00482180` / `FUN_00482330` branches on `[0x18,0x1c)` (`FUN_00482560`). The shipped AI never requests Death Star Sabotage: `FUN_004e8290` requests `0x6a` only for side 1 when ctx+4 bit 0x4 (an own Death Star hull, `FUN_0041ba80`) is set (`death-star-product.md`).

## Sabotage

Mission family `0x6a` (DSSBMSTB, chance `FUN_0055c8d0` = (espionage + combat) / 2): `FUN_005744c0` accepts only targets in `[0x18,0x1c)` (refusal `0x40/0x29`); ordinary Sabotage `FUN_0056a110` refuses them (`0x40/0x28`). Success runs the target's slot `+0xac(6)` (`FUN_005746e0`): one success destroys the Death Star. Both sides can run it. `FUN_00593500` requires family `[0x10,0x40)` and the target's `+0x50` bit 2 (completed), so an uncompleted product cannot be targeted; the target slot holds the ship (`death-star-product.md`).

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

## Port

- **Identity.** `CapitalShipClass::family` is the CAPSHPSD family and
  `is_death_star` tests `0x18`-`0x1b`; `Give Death Star` adds a ship of that
  class. `Fleet::has_death_star` asks the fleet's live ships for family
  `[0x18,0x1c)` (`FUN_005003d0`); bit 11 (`refresh_sectors`), the AI, the
  tactical station and Destroy System all read it, and a station destroyed
  in battle kills its ship (`tactical::apply`).
- **Fire.** Order `0x223` runs through the gate and `integrator::resolve_strike`
  (`bombardment.md`, Port). Its Kill branch, in order:
  1. the system's own regiments and squadrons as losses with reason `0xf`
     (`0x10` at the bombarder's own system) into the blockade counters
     (`support_shift::system_unit_losses`);
  2. the populated bit cleared and the holder set neutral, then
     `support_shift::control_changed` (the HQ's removal with reason 3 and the
     change's shift; VM Yaga Minor read neutral and its sector "took 1
     back", `support-shifts.md`);
  3. `support_shift::remove_headquarters` for an HQ at a system already
     neutral;
  4. `integrator::destroy_system`: `is_destroyed`, the cascade
     (`death_star::cleanup_destroyed_system`), then the mission validator at
     once (`MissionState::validate_destroyed`: rules 2, 4 and 7, code 7 past
     the landing; `port:` a mission not yet past its creation steps is left to
     its next wake);
  5. the character roll (`bombardment::roll_characters`, GNPRTB `0x606`),
     the finalize and the flags;
  6. when the cascade set an adrift flag, the adrift sweep (Sweep, below).
- **Movies.** A Destroy System run plays movie `0x6f` (MDATA.101, System
  Destroyed) to each side `strike_orders::recipients` tells
  (`strike_orders::strike_movies`; caption body `0x70c9` with the holder's
  adjective, `0x70ca` for a neutral system), and a sabotaged Death Star plays
  movie `0x72` (MDATA.104, Death Star Sabotaged) to both sides
  (`strike_orders::sabotage_movies`; caption body `0x71be` with the system's
  name): kind-6 messages, the body as the caption, with no Message Index row
  and no advice (`movies.md`).
- **Cascade.** Fleets of every side survive with their contents. The
  system's completed regiments, squadrons, facilities and HQ die; special
  forces not en route die, and en-route ones are left to arrive. Characters
  at the system, off a mission and not aboard a fleet go adrift
  (`Character::adrift`, saved), with no message. The autoscrap request runs
  at once for the system's uncompleted products (hint: the system),
  deliveries bound for it (hint: their origin) and the regiments and
  squadrons travelling to its surface: each is placed through the retreat
  chain (`post_battle::request_place`), else destroyed with reason 2 and no
  refund. `CascadeReport` lists each. No move is cancelled and the blockade
  is untouched; fleets may enter a destroyed system and stay, and regiment
  landings there are refused (`troop_transport.rs`). `hyp:` a relocated
  product leaves its queue. `port:` the children are taken per kind, not in
  container order; a relocated delivery leaves its origin again on the day
  of the kill; Coruscant's `is_headquarters` is the Empire seat, not an
  ALLFACSD object, and stays. The sector window draws the destroyed system as
  the asteroid field with no status bars (`sector-window-hit-test.md`).
- **Sweep.** `death_star::sweep_adrift` (`FUN_00548e90`) runs at a system
  on each change of its battle bit (`BlockadeState::battle`, `+0x88`
  `0x1000`: side 1 and side 2 both present at a system with a side,
  `BlockadeSystem::battle_bit`; event `0x304`), through
  `death_star::sweep_battle_changes` after `BlockadeSystem::advance` in the
  headless and native steps alike (`agent_docs/simulation.md`), with the
  new bit. It also runs after a Destroy System, following the character
  roll, when the cascade set an adrift flag (event `0x30e`) or the kill
  changed the battle bit (`integrator::resolve_strike`, which the headless
  and native strikes share): with no fleets of both sides in orbit the
  system's adrift characters leave the day of the kill; with both, the
  neutral holder (side 3) keeps the bit set and they wait for it to clear
  ("When a Destroy System sweeps its adrift characters"). Adrift characters
  aboard the system's fleets move into the system. With the bit clear,
  each pass (`sweep_groups`, `FUN_00549230`) takes the adrift characters
  off a mission at the system: one with a captor keeps its flag and stays;
  every other loses the flag, and one whose system is destroyed or whose
  side differs from the system's side bits (`bombardment::holder`, the
  earlier holder through a stalemate) joins a group by side and
  retreat-chain destination. A group with a
  free member starts a hidden trip with every member, prisoners aboard
  (`MissionState::send_adrift`); a group of prisoners is freed and goes
  round once more as ordinary characters; a group with no destination goes
  member by member through `death_star::strand` (`FUN_00549580`): a free
  member is captured with no captor and a prisoner is freed, and the
  retreat chain runs again on the member's new side, so a destination
  starts its hidden trip there, a prisoner aboard as one; with none,
  `death_star::hold_in_jail` holds it in the jail of its side bits
  (`Abode::jail_of`), the other side's: a prisoner with no captor, out of
  every system, fleet and fleet roster. An occupant is
  `Character::held_abode`: a prisoner with no captor in no system or
  fleet; every placement and change of captivity clears the abode
  (`Character::leave_abode`). The Personnel Finder row of an occupant is
  its name and its jail's name with no state word, and its Display opens
  nothing. `MissionSystem::escape_step` rolls an occupant like every other
  prisoner, its guards the captor side's free characters in the same jail
  (`escape_input`); victory reads the prisoner bit alone
  (`victory-conditions.md`, Port). `port:` the trips run as the Move kind;
  the port has no Adrift or Autorouting kind. `port:` the 1-day trip to a
  jail (`FUN_00555b30`) is not run: the member sits in the abode from the
  sweep on, as `FUN_00556390` reparents it at once, with its en route bit
  clear, which no reader of an occupant sees. `port:` the port has no Bounty mission, so Jabba's
  Palace is never a destination. `hyp:` the prisoners a pass frees form one
  group on the next.
- **Telemetry.** `death_star_fired` carries the cascade's counts, the adrift
  characters and the autoscrapped objects; `character_killed` each rolled
  character; `mission_dispatched` each trip the sweep started.
- **Victory.** `victory-conditions.md`, Port.

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
| [FUN_004fb810](FUN_004fb810.c), [FUN_004f7bc0](FUN_004f7bc0.c), [FUN_004f76b0](FUN_004f76b0.c), [FUN_005109a0](FUN_005109a0.c), [FUN_0050b760](FUN_0050b760.c), [FUN_0050a3c0](FUN_0050a3c0.c), [FUN_00510e30](FUN_00510e30.c), [FUN_0050b890](FUN_0050b890.c), [FUN_00508660](FUN_00508660.c) | Existing bit, event `0x300`, populated bit and its hook |
| [FUN_005081f0](FUN_005081f0.c), [FUN_0050a430](FUN_0050a430.c), [FUN_0050a510](FUN_0050a510.c), [FUN_00566d40](FUN_00566d40.c) | System `+0x88` bits 1 and 3 |
| [FUN_00534790](FUN_00534790.c), [FUN_00536ac0](FUN_00536ac0.c), [FUN_004f8880](FUN_004f8880.c), [FUN_006158b0](FUN_006158b0.c), [FUN_00535c00](FUN_00535c00.c) | Adrift setter and its notifier |
| [FUN_00511780](FUN_00511780.c), [FUN_0050a900](FUN_0050a900.c), [FUN_00563c80](FUN_00563c80.c), [FUN_00563dd0](FUN_00563dd0.c), [FUN_00515dc0](FUN_00515dc0.c), [FUN_00545950](FUN_00545950.c), [FUN_00548e90](FUN_00548e90.c), [FUN_00548da0](FUN_00548da0.c), [FUN_00549230](FUN_00549230.c), [FUN_0054c7d0](FUN_0054c7d0.c), [FUN_00549580](FUN_00549580.c), [FUN_00549850](FUN_00549850.c), [FUN_00543040](FUN_00543040.c), [FUN_005431e0](FUN_005431e0.c), [FUN_00547740](FUN_00547740.c), [FUN_00547880](FUN_00547880.c), [FUN_00576a20](FUN_00576a20.c) | Adrift sweep (event `0x304`) |
| [FUN_004f78e0](FUN_004f78e0.c), [FUN_004faa10](FUN_004faa10.c), [FUN_004fba70](FUN_004fba70.c), [FUN_0057b1f0](FUN_0057b1f0.c), [FUN_00515480](FUN_00515480.c), [FUN_00545df0](FUN_00545df0.c), [FUN_00515970](FUN_00515970.c), [FUN_00541270](FUN_00541270.c), [FUN_005545e0](FUN_005545e0.c), [FUN_005571c0](FUN_005571c0.c), [FUN_00553410](FUN_00553410.c) | Autoscrap request (event `0x307`) and relocation |
| [FUN_00523450](FUN_00523450.c), [FUN_00592600](FUN_00592600.c), [FUN_00545240](FUN_00545240.c), [FUN_005140c0](FUN_005140c0.c), [FUN_0057a730](FUN_0057a730.c) | Missions at a destroyed system |
| [FUN_00520b60](FUN_00520b60.c), [FUN_00526860](FUN_00526860.c), [FUN_00526c00](FUN_00526c00.c), [FUN_004f7640](FUN_004f7640.c) | HQ in transit |
| [FUN_00550bd0](FUN_00550bd0.c), [FUN_0054fd90](FUN_0054fd90.c), [FUN_0054fca0](FUN_0054fca0.c), [FUN_00550eb0](FUN_00550eb0.c), [FUN_00550fb0](FUN_00550fb0.c), [FUN_00552dd0](FUN_00552dd0.c) | Room algebra behind the one-per-system refusal |
| [FUN_00433620](FUN_00433620.c), [FUN_00433e40](FUN_00433e40.c), [FUN_00526a60](FUN_00526a60.c), [FUN_004b6510](FUN_004b6510.c), [FUN_004b8ab0](FUN_004b8ab0.c), [FUN_0042e210](FUN_0042e210.c) | AI fleet flag `0x10`, `+0x110` census, need bits |
| [FUN_005617b0](FUN_005617b0.c), [FUN_005070d0](FUN_005070d0.c), [FUN_0055f650](FUN_0055f650.c), [FUN_00560d50](FUN_00560d50.c) | Emperor-at-Coruscant flag (family `0x34`) |
| [FUN_0042b330](FUN_0042b330.c), [FUN_0045bbb0](FUN_0045bbb0.c), [FUN_0045b770](FUN_0045b770.c), [FUN_0045baf0](FUN_0045baf0.c), [FUN_0045c240](FUN_0045c240.c), [FUN_0045c450](FUN_0045c450.c), [FUN_0060bd20](FUN_0060bd20.c) | The destroyed system on screen: galaxy marker, sector planet item, child bars |
| [FUN_00488de0](FUN_00488de0.c), [FUN_004c19f0](FUN_004c19f0.c), [FUN_004c3bb0](FUN_004c3bb0.c), [FUN_004568a0](FUN_004568a0.c), [FUN_00465bb0](FUN_00465bb0.c), [FUN_0049b720](FUN_0049b720.c), [FUN_0044c630](FUN_0044c630.c), [FUN_0046f360](FUN_0046f360.c), [FUN_0045c970](FUN_0045c970.c) | Other readers of the destroyed bit and the "Destroyed" text |
| [FUN_00536460](FUN_00536460.c), [FUN_00541110](FUN_00541110.c), [FUN_0053f9c0](FUN_0053f9c0.c), [FUN_0053f950](FUN_0053f950.c), [FUN_0048b450](FUN_0048b450.c), [FUN_00545df0](FUN_00545df0.c), [FUN_0050ac10](FUN_0050ac10.c), [FUN_00510820](FUN_00510820.c) | Event `0x30e`, its raiser and the blockade-rule recompute |
| [FUN_005439b0](FUN_005439b0.c), [FUN_005439e0](FUN_005439e0.c), [FUN_005442f0](FUN_005442f0.c), [FUN_00544da0](FUN_00544da0.c), [FUN_00534fa0](FUN_00534fa0.c), [FUN_004efd30](FUN_004efd30.c), [FUN_00536070](FUN_00536070.c), [FUN_004f11c0](FUN_004f11c0.c), [FUN_00549910](FUN_00549910.c) | Readers of the adrift bit |
| [FUN_00519910](FUN_00519910.c), [FUN_00513a50](FUN_00513a50.c), [FUN_00577530](FUN_00577530.c), [FUN_00577050](FUN_00577050.c), [FUN_005775a0](FUN_005775a0.c), [FUN_005773b0](FUN_005773b0.c), [FUN_00577270](FUN_00577270.c), [FUN_00506ed0](FUN_00506ed0.c) | Jail abodes: creation and class |
| [FUN_004ef480](FUN_004ef480.c), [FUN_004ef510](FUN_004ef510.c), [FUN_004ef570](FUN_004ef570.c), [FUN_004ef5b0](FUN_004ef5b0.c), [FUN_004ee3e0](FUN_004ee3e0.c), [FUN_004ef190](FUN_004ef190.c), [FUN_004f18e0](FUN_004f18e0.c), [FUN_004eef70](FUN_004eef70.c), [FUN_004ee800](FUN_004ee800.c), [FUN_004edc70](FUN_004edc70.c), [FUN_004ed270](FUN_004ed270.c), [FUN_005345d0](FUN_005345d0.c), [FUN_0057c8b0](FUN_0057c8b0.c) | Character status, capture and release slots |
| [FUN_00547ba0](FUN_00547ba0.c), [FUN_00522210](FUN_00522210.c), [FUN_00586c80](FUN_00586c80.c), [FUN_00429440](FUN_00429440.c), [FUN_0056df70](FUN_0056df70.c), [FUN_0056d4f0](FUN_0056d4f0.c) | Autorouting start, Rescue target, Display, Palace |

## Open questions

- The advisor text of the `{0x90, 3}` room refusal for a second family-`0x18` ship.
- A VM check of the same-day sweep (read from code, "When a Destroy System sweeps its adrift characters"): Destroy System at a system holding a minor character and no fleet of the other side, then read the character's location in saves taken the day of the kill and the next; a hidden trip to another system of its side confirms it. With fleets of both sides in orbit the trip should start when the battle bit clears.
- A VM run of a jail occupant: its escape and where it lands, and the AI Rescue planner's target construction (`jail.md`, "Open").
- A VM run of the product cascade (code read, "Builds"): queue a regiment and a ship at a yard system, destroy it, and read the products (both destroyed with their manager); queue a regiment at yard A with the Destination at system B, destroy B, and watch the regiment (relocated, then completed at A's pace and shipped to its new container); and a regiment in transit to a destroyed system ("A relocated unit's pending arrival").
- Whether the planners ever hand the Death Star an assault target (a Ghidra read of the module-11 budget and `FUN_004ae240`'s caller chain, or an AI-vs-AI soak).
- Two tactical Death Stars in one battle (`FUN_00596bd0`, `FUN_005a9330`).
