# AI production: the modules, the system record and the goal token

The production modules of the side AI (`ai-order-emission.md`, "Production",
summarises them). Part F covers the system record `FUN_00433e40`, the goal
token and the facility modules 3, 1 and 2; part U the unit modules 11, 10,
7, 5 and 6. Every rule cites the decompile it was read from
(`ghidra/notes/FUN_*.c`). UNSURE marks a meaning the decompile does not
settle; MISSING marks a callee with no decompile.

# Part F: facilities, the system record and the goal token

UNSURE marks anything the decompile does not settle. MISSING marks a callee with no decompile.

Notation: `m+` module object, `c+` side AI context (`m+0x2c`), `sys+` per-system AI record (`FUN_00433e40`),
`rec+` sector record (`FUN_004321e0`/`FUN_00432350`), `obj+` the game system object (`FUN_004f3220(side, id)`).
Sides: 1 Alliance, 2 Empire (`sys+0x1c`, `m+0x30`).

## F0. Primitives every module uses

### F0.1 Object id families (id >> 24)

| Range | Meaning | Source |
|---|---|---|
| `0x80`-`0x8f` | sector | `FUN_00419b80` walks up parent ids until it is in this range |
| `0x90`-`0x97` | star system | every "valid site" test (`FUN_004d9e70`, `FUN_004d8ad0` ...) |
| `0xa0`-`0xa1` | construction-yard producer ("production manager") | `FUN_004d9e70`, `FUN_004d9fb0`, `FUN_004dc9f0` test `0x9f < f < 0xa2` |
| `0xa2`-`0xa3`, `0xa4`-`0xa5` | shipyard / training producers | only by sibling notes; code here only tests `0xa0`-`0xa1` |
| family numbers `0x20`..`0x2d` | objects at a system | `FUN_0052cab0` `0x28` shipyards, `FUN_0052c740` `0x29` training, `FUN_0052c420` `0x2a` construction yards, `FUN_00527550` `0x22`, `FUN_005272e0` `0x23`, `FUN_005278c0` `0x24`, `FUN_0051c270` `0x25`, `FUN_0052d610` `0x2c` mines, `FUN_0052d1f0` `0x2d` refineries, `FUN_00526a60` `0x20`-`0x21` HQ |
| `0x90000109` | Coruscant | `FUN_004d76f0`, `FUN_00433e40` (low 24 bits `0x109`) |
| class ids | Mine `0x2c000001`, Refinery `0x2d000002` | literals in `FUN_004d6e70`, `FUN_004db2f0`, `FUN_004e25c0` |

`FUN_00509670(sysObj, k)`: first object of family `[0xa0,0xb0)` at the system whose vtable `+0x1ec()` equals k. k=1 construction-yard
producer (proved by the `0xa0`-`0xa1` test on its result in `FUN_004d9e70`), k=0 shipyard, k=2 training (the builder pairs them with
`sys+0x58`, `sys+0x5c`, `sys+0x60`). The producer id becomes the order's team. Producer `+0x58 == 0` is the "idle" test (`FUN_00433e40`);
what `+0x58` counts is UNSURE (queue length is the likely meaning).

### F0.2 Orders emitted by modules 3, 1, 2

| Code | Name | Fields | Built by |
|---|---|---|---|
| `0x200` | Scrap | `+0x20` side; list of object ids (`FUN_004f57b0`); no quantity | `FUN_004d7330`, `FUN_004d8520`, `FUN_004d8fe0`, `FUN_004d9b00`, `FUN_004d95c0`, `FUN_004d7cf0`, `FUN_004db820`, `FUN_004dbe70`, `FUN_004dc650`, `FUN_004e2940`, `FUN_004e31b0`, `FUN_004e3a40`, `FUN_004e4020` |
| `0x214` | Destination | `+0x20` side; team list = [producer id]; vtable `+0x2c(site id)`; needs site family `0x90`-`0x97` and producer family `0xa0`-`0xa1` | `FUN_004d9e70`, `FUN_004dc9f0`, `FUN_004e4300` |
| `0x210` | Build facility | `+0x20` side; team = [producer]; `+0x44` class id; `+0x48` quantity; needs producer family `0xa0`-`0xa1` | `FUN_004d9fb0`, `FUN_004dcb30` (also needs class id high byte != 0), `FUN_004e4440` |

A Destination and its Build are two separate steps and therefore two scheduler rounds (the Destination step sets `m+0x20 = 1` and `m+0x34++`).
Order execution and refusal (`FUN_005385f0`, error `0xf3`) are summarised in `ai-order-emission.md`.

### F0.3 Rank-sum selection (`FUN_00435790` + `FUN_004357b0`)

`m+0x40` (and `m+0x68`/`m+0x70` in other modules) is a holder of sorted lists. Each query function returns a list of
(id, key) nodes sorted by the key with comparator `FUN_0041c250`: node a goes before b when a.key < b.key if the list's direction
(`+0x1c`) is 1, a.key > b.key if it is 2, equal keys by `rand(10) < 5`. `FUN_00435790(holder, list)` appends the list; `FUN_0041c230`
numbers the nodes 1..n. `FUN_004357b0(holder)` keeps only ids present in every list, sums the 1-based positions across the lists and returns the
id with the largest sum (first in the first list's order wins a tie), removing it from the first list. So **direction 1 favours the largest
key, direction 2 the smallest key**, and several lists are a Borda count. `FUN_005f58b0(holder)` clears the lists.
Ids are 0 (invalid) when nothing survives; callers test `id >> 24` against the ranges above.

### F0.4 Query functions

| Function | Arguments | Filters | Key |
|---|---|---|---|
| `FUN_00419a00(c, S1, S2, S3, N1, N2, N3, key, dir)` | sector query | over `c+0x44` sector records: `rec+0x24 & S1 == S1` and `& N1 == 0`, `rec+0x28` with S2/N2, `rec+0x2c` with S3/N3 | `rec+0x70 + 4*key` |
| `FUN_00419b80(c, startId, S1, S2, S3, N1, N2, N3, dir)` | nearest sectors | same filters, over the sector of `startId` (parent walk) | distance from that sector (`FUN_0041b7b0` on `rec+0x40`) |
| `FUN_0041a340(c, sectorId, A, B, C, D, E, F, key, dir)` | systems of one sector (10 slots at `rec+0x48`) | `sys+0x28 & A == A`, `sys+0x2c & B == B`, `sys+0x30 & C == C`; `sys+0x28 & D == 0`, `sys+0x2c & E == 0`, `sys+0x30 & F == 0` (`FUN_00431830`) | `sys+0x48 + 4*key` |
| `FUN_0041a400(c, sectorId, nearId, A, B, C, D, E, F, dir)` | MISSING (no decompile) | arguments read as the same A..F as `FUN_0041a340`; presumably systems of the sector ranked by distance to `nearId` | UNSURE |
| `FUN_0041bd90` | MISSING | returns a sector record for an id | |

Consequently `sys+0x30 & 1` is "own", `sys+0x28 & 3 == 0` is "not under uprising and not blockaded" (see 1.2).

### F0.5 Class pick `FUN_0041b230(c, outClassId, type, mask, key, dir)` (thunk to `FUN_0042e3c0`)

- Candidates: the side's currently producible classes of production family `type` (`FUN_0052e580`: list `side+0x9c` for `0x28`, `+0xa0` for `0x29`,
  `+0xa4` for `0x2a`); keep those whose AI-class-node flags (`node+0x18`) contain every bit of `mask`.
- `key == 0` picks a random candidate. Otherwise rank-sum selection (0.3) over the node key (node vtable `+4(key)`); dir 1 largest, dir 2 smallest.
- Writes the winner's class id to `outClassId` (zero id on no candidates) and returns the winner's maintenance (`node+0x20`, = class `+0x4c`, `FUN_0053b870`);
  returns 0 when nothing qualifies. `FUN_0041b270` (`FUN_0042e720`) returns the whole ranked list instead (not used by modules 3, 1, 2).
- Node flags: facilities (`FUN_00473090`, family = class `+0x2c`): `0x4000` family `0x28`, `0x8000` `0x29`, `0x10000` `0x2a`.
  Defences (`FUN_00473290`): `0x20000` family `0x24`, `0x40000` family `0x25`, `0x80000` families `0x22` and `0x23`.
- Node keys: facility (`FUN_00473060`) `0x4000` -> node `+0x30` = class `+0x5c` (DEFFACSD file `0x34`), `1` -> maintenance, `4` -> refined cost (`+0x1c`).
  Defence (`FUN_00473250`) `0x10000` -> node `+0x34` = class `+0x60` (file `0x38`, shield_strength), `0x40000` -> node `+0x30` = class `+0x5c`
  (file `0x34`, attack_strength), `1` maintenance, `4` refined cost.
- Production family of a class is class `+0x30` (file `0x08`): type `0x2a` selects everything built by construction yards (yards, defences, mine, refinery).
  The family number used in the node ctors is class `+0x2c`, the record's own `family_id` (file `0x10`): the
  capital-ship ctor `FUN_004739b0` gives node flag `0x80` to `0x18`..`0x1b`, which in CAPSHPSD is the Death Star alone
  (record 136; `death-star-strategic.md`), and that node never gets the combat bit 1.

## F1. `FUN_00433e40`: the per-system AI record (`sys+`)

Rebuilt only when `sys+0x24 != 0` (dirty flag; it is cleared first). `FUN_00433e30` sets `sys+0x24 = 1` and marks the sector record (`FUN_004321d0`). The modules call it after editing a role bit; the object-change handlers call it for the system an object belongs to: `FUN_00418a60` / `FUN_00418db0` / `FUN_00419260` / `FUN_00419630` → `FUN_00431170` (the system of a changed object), `FUN_0041b110` (a fleet's system, from `FUN_00430220`), `FUN_004311d0` (every record, from `FUN_00418240` / `FUN_00418270`), `FUN_00433dc0`, `FUN_00433e10`. No record field holds a battle's outcome.
Sector aggregation then folds the record into `rec+` (`FUN_004321e0` -> `FUN_00432350`).

### F1.1 Reset and early exits

1. `sys+0x48`..`sys+0x137` (0x3c dwords) are zeroed.
2. `sys+0x28 &= 0x3e00000` (persistent: module role bits `0x200000`, `0x400000`, `0x800000`, `0x1000000`, `0x2000000`), `sys+0x30 &= 0x35ff0000`, `sys+0x2c &= 0x60000000`.
3. `sys+0x20` (owner) = `obj+0x24 >> 6 & 3`. Own owner value = `sys+0x1c`; enemy = the other side.
4. If `obj+0x50 & 8`: `sys+0x30 |= 0x10` and `FUN_004fcdd0(&sys+0x28)` (MISSING; read as clear, which would drop the role bits too). UNSURE.
5. `sys+0x30 |= 0x40` when `sys id >> 24` in `0x92`..`0x93`.
6. Ownership: owner == own -> `sys+0x30 |= 0x21`; owner == enemy -> `|= 0x24`; else `|= 2`.
7. If `obj+0x88 & 2 == 0` the record is finished with `sys+0x2c |= 0x80` (nothing else is computed). Otherwise `sys+0x2c |= 0x8000000`.
   `obj+0x88` bit 0 = populated, bit 2 = uprising, bit 5 (`0x20`) = blockade (`economy-systems.md`, `blockade-bit.md`, `mission-lifecycle.md`); bit 1 = visible to the side is UNSURE.

### F1.2 `sys+0x28` bits (A)

| Bit | Set when | Source |
|---|---|---|
| `0x1` | `obj+0x88 & 4` (uprising), only in the populated branch | `FUN_00433e40` |
| `0x2` | `obj+0x88 & 0x20` (blockade) (also `sys+0x44 = FUN_004fcf00()` when `sys+0x30 & 6`, UNSURE meaning) | |
| `0x20` | own and `sys+0x84 > 0` | `sys+0x84 = obj+0x5c - obj+0x60` |
| `0x40` | own, `sys+0x84 > 0` and `sys+0x80 > 0` (also sets `0x20`) | `sys+0x80 = obj+0x64 - obj+0x68` |
| `0x80` | `0x100` branch with `sys+0x80 > 0` (set together with `0x100`): a free raw-material node, so a mine can go here | see `0x100` |
| `0x100` | (a) unpopulated own system with `A & 0x20`; (b) populated own system with no role (`0x3e00000`), `A & 3 == 0`, and `[sys+0x74 + sys+0x70 + sys+0x54 + sys+0x50 + sys+0x64 (+1 if sys+0x30 & 0x80000000)] < sys+0x4c`: a free energy slot, so a refinery can go here | |
| `0x200` | any operational shipyard (family `0x28`, `obj+0x50 & 1`), `sys+0x58 > 0`, `A & 3 == 0`, and its producer `FUN_00509670(obj,0)` exists with `+0x58 == 0` (idle) | |
| `0x400` | same for training facilities (`0x29`, `sys+0x5c`, producer kind 2) | |
| `0x800` | same for construction yards (`0x2a`, `sys+0x60`, producer kind 1): "yard ready" | |
| `0x1000` | any operational shipyard, training facility or construction yard (set while scanning each list) | |
| `0x2000` | regiment shortage: `sys+0x90 < sys+0xa0` (own, populated); `sys+0x9c = sys+0xa0 - sys+0x90` | only when `A & 2 == 0` or `sys+0x30 & 0x30000 == 0` |
| `0x10000` | regiment surplus: `sys+0x90 > sys+0xa0`; `sys+0x94 = sys+0x90 - sys+0xa0` | |
| `0x20000` | defender shortage: `sys+0x8c < sys+0xa0`; `sys+0x9c = sys+0xa0 - sys+0x8c` | |
| `0x4000` | defence deficit: `sys+0xdc < 0` after both counts are at or below target | |
| `0x8000` | shield-count shortage: `sys+0x64 - sys+0x6c < 0` | |
| `0x40000` | fighter shortage: `sys+0xe0 < 0` | |
| `0x100000` | `sys+0x11c < 0` | |
| `0x80000` | own and at least one operational squadron | |
| `0x10000000` | own special system (`A & 0x83e00000 != 0`) with a regiment whose class flag `0x1000` is set and `node+0x34 >= class-table+0x24` | regiment loop |
| `0x20000000` | own plain system with a regiment lacking flag `0x1000`; or special system with flag `0x2000` and `node+0x34 >= class-table+0x24` | regiment loop |
| `0x4000000` | own, `A & 3 == 0`, `A & 0x3e00000 == 0` (unassigned system); not populated systems never get it | |
| `0x40000000` | Empire (`sys+0x1c == 2`) in the Coruscant branch, `c+4 & 0x1000000` (class `0x25000004` producible), `c+4 & 4` (own Death Star ship) and `sys+0x70 == 0`: wants a Death Star shield | UNSURE only for the `c+4` bit names |
| `0x80000000` | id low 24 bits `0x109`, or `FUN_00526a60(obj, owner, 3) != 0` (an HQ object of the owner at the system) | |
| `0x200000`, `0x400000` | persistent role bits: shipyard site, training site (set by modules 11/10/7 and 5/6; `FUN_004cafa0` sets `0x200000`) | |
| `0x800000` | persistent role: module 1 yard system (`FUN_004daf10`, `FUN_004db510`, `FUN_004dcc40`) | |
| `0x1000000` | persistent role: module 2 yard system (`FUN_004e3530`, `FUN_004e3810`, `FUN_004e4540`) | |
| `0x2000000` | persistent role: module 3 yard system (`FUN_004d8ad0`, `FUN_004d8db0`, `FUN_004da0d0`) | |

Role sets: `0x3e00000` = all five roles, `0x3800000` = the three construction-yard roles, `0x83e00000` also includes the HQ bit.

### F1.3 Numeric fields

| Offset | Meaning | Source |
|---|---|---|
| `+0x18` id, `+0x1c` side, `+0x20` owner side, `+0x24` dirty, `+0x3c` sector record, `+0x40` class table (node lookup by id, `FUN_0042e190`; side class table `c+0x138`), `+0x44` UNSURE | | |
| `+0x48` | popularity: 100 for any own system, then `FUN_00507270(obj, side)` for populated (side 1 `obj+0x58`, else `100 - obj+0x58`) | |
| `+0x4c` | `obj+0x5c`, system energy (slot capacity) | economy-systems.md |
| `+0x50` / `+0x54` | mines (family `0x2c`) / refineries (`0x2d`) of the owner (`FUN_0052d370`, `FUN_0052cf40`) | |
| `+0x58` / `+0x5c` / `+0x60` | counts of shipyards / training facilities / construction yards (own owner) | |
| `+0x64` / `+0x68` | count of family `0x24` / sum of their node `+0x34` (shield strength) for own operational ones | |
| `+0x6c` | target family-`0x24` count (see 1.4) | |
| `+0x70` | count of family `0x25`; own operational one sets `sys+0x30 |= 0x80` | |
| `+0x74` / `+0x78` | count of families `0x22`+`0x23` / sum of node `+0x30` (attack) for own operational ones | |
| `+0x7c` | target family-`0x22`/`0x23` count | |
| `+0x80` / `+0x84` | `obj+0x64 - obj+0x68` / `obj+0x5c - obj+0x60` | |
| `+0x88` / `+0x8c` / `+0x90` | regiments present / regiments counted as defenders (see `0x20000000` rule) / operational regiments | |
| `+0x94` / `+0x98` / `+0x9c` / `+0xa0` | regiment surplus / need (`+= (0x4f - sys+0x48)/10` capped at 6 while `sys+0x48 < 0x46`, doubled when uprising) / shortage / target (`c+0x37c` plain, `c+0x380` role, `c+0x384` capital; at least `+0x98`) | |
| `+0xa4` / `+0xa8` / `+0xac` | squadrons present / counted (own, plain or `node+0x34 >= class-table+0x28`) / operational | |
| `+0xb0`,`+0xb4`,`+0xb8` | sums of `FUN_00473610/620/630` over operational squadrons | |
| `+0xbc`,`+0xe4`,`+0xe8`,`+0xec`,`+0xf4` | sums over foreign fleet records at the system (fleet `+0xa4`,`+0xb4`,`+0xb8`,`+0x60`,`+0x70`) | |
| `+0xc0` / `+0xc4` / `+0xc8` | maintenance of regiments / of defences (families `0x22`-`0x25`) / of squadrons (class node `+0x20`) -> `rec+0xfc/+0x100/+0x104` -> `c+0x1d8/+0x1dc/+0x1e0` | |
| `+0xcc` | maintenance of yards of all three kinds; `+0xd0` shipyards, `+0xd4` training, `+0xd8` construction yards | |
| `+0xdc` | defence balance: with `a = sys+0x64 - sys+0x6c`, `b = sys+0x74 - sys+0x7c`: both <= 0 gives `a+b`; `a > 0, b <= 0` gives `a`; `a <= 0, b > 0` gives `b`; both > 0 gives `a+b`. Negative is a deficit | |
| `+0xe0` / `+0x11c` | fighters `+0xa8` / `+0xac` minus target (`c+0x388` plain, `c+0x38c` role, `c+0x390` capital); plain system with no regiments counted uses `+0xa4` | |
| `+0xf8`..`+0x110` | character counts by flag (`FUN_00536da0` loop) | UNSURE meanings |
| `+0x114` | free slots = `sys+0x4c - ([role counts: sys+0x58 if A&0x200000] + [sys+0x5c if A&0x400000] + [sys+0x60 if A&0x3800000]) - max(sys+0x64, sys+0x6c) - max(sys+0x74, sys+0x7c) - (1 if sys+0x30 & 0x80000000)` | the `0x4000`-class pick caps |
| `+0x118` | enemy threat: neutral `sys+0x74 + sys+0x98`; enemy `sys+0x88 + sys+0x98 + sys+0x74` | |

### F1.4 Defence and regiment targets (the numbers modules 3 and 6 chase)

`c+0x364`..`c+0x390` start at 3, 3, 2, 1, 1, 0, 0, 3, 9, 0, 6, 12 (`FUN_0041bd20`), adjusted by `FUN_00418240` (+1) and `FUN_00418270` (-1).
For an own populated system (`FUN_00433e40` tail):

| System | `sys+0x6c` (family `0x24` target) | `sys+0x7c` (`0x22`/`0x23` target) |
|---|---|---|
| plain (no role, not Coruscant), only when `sys+0x98 > 0` | `c+0x374` (1) if `c+0x378 + c+0x374 < sys+0x4c` | `c+0x378` (0) in that case |
| role system (`A & 0x3e00000`) | `c+0x36c` (2) and `c+0x370` (1) if `c+0x370 + c+0x36c < sys+0x4c`; else `c+0x374` (1), batteries 0, if `c+0x374 < sys+0x4c` | |
| Coruscant (`sys+0x30 & 0x40000000`) | `c+0x368` (3), batteries `c+0x364` (3); Empire reserves 1 slot (`iVar8 = 1`); if `sys+0x4c < reserve + 2` shields = `sys+0x4c - reserve`, batteries 0; if `sys+0x4c < reserve + c+0x364 + c+0x368`, shields `c+0x368`, batteries `sys+0x4c - c+0x368 - reserve` | |

### F1.5 `sys+0x2c` and `sys+0x30` bits (completeness; semantics UNSURE unless stated)

`sys+0x30` (C): `0x1` own, `0x2` neutral, `0x4` enemy, `0x8` populated (`obj+0x88 & 1` branch), `0x10` `obj+0x50 & 8`, `0x20` owned by a side, `0x40` system id family `0x92`-`0x93`,
`0x80` operational family-`0x25` object present (any owner), `0x100` `sys+0x64 > 1`, `0x200` own... foreign fleet class flag present (fleet record `+0x38 & 0x4000` with `&1 == 0`),
`0x400` such a fleet with `&0x10`, `0x800` (enemy branch) installations present but `sys+0x74 == 0`, `0x1000` fleet `+0x38 & 0x2000000` or character flags `0x7000000`, `0x3000` with `0x2000000`,
`0x4000` / `0x8000` regiment node flags `0x1000` / `0x2000`, `0x8000000` id low 24 bits `0x121`, `0x40000000` Coruscant, `0x80000000` HQ; persistent `0x10000000` = module rotation marker (1.6).
`sys+0x2c` (B): `0x1` own (end), `0x2` `c+0xa4 > 0` and not `sys+0x30 & 0x8000000`, `0x4`/`0x8`/`0x10` yard-role systems with idle yards and a free `c+4` bit and `sys+0x90 > 0`,
`0x20` `A & 1` without `sys+0x30 & 0x200`, `0x40` `sys+0x2c & 0x10000000` without those, `0x80` not visible or not own, `0x100` enemy with `sys+0x48 > 0x32`,
`0x200` own or enemy end, `0x400`,`0x1000`,`0x4000`,`0x4000000` character flags, `0x800`,`0x2800`,`0x8000`,`0x800000`,`0x1000000` fleet flags, `0x10000`..`0x400000` enemy-branch summaries
(`0x10000` mines or refineries, `0x20000` yards, `0x40000` `sys+0x68 > 0`, `0x80000` `sys+0x30 & 0x80`, `0x100000` `sys+0x78 > 0`, `0x200000` `sys+0x90 > 0`, `0x400000` `sys+0xac > 0`),
`0x2000000` HQ, `0x8000000` visible, `0x10000000` `sys+0x48 < 0x46`.

### F1.6 Sector record fields the module queries use (`FUN_00432350`, own-system branch unless noted)

| `rec+0x24` bit | Set when a system in the sector has |
|---|---|
| `0x8` | own and not blockaded |
| `0x20` / `0x40` / `0x80` | own, `A & 3 == 0`, `sys+0x30 & 0x8000000` clear, `sys+0x114 > 0`, and no shipyard role / no training role / no yard role |
| `0x100` / `0x200` / `0x400` / `0x800` | own, `A & 3 == 0` and `A & 0x80` / `0x100` / `0x20` / `0x40` |
| `0x1000` | `A & 0x800000` and `A & 0x800` and `A & 3 == 0` (ready module-1 yard) |
| `0x2000` | `A & 0x800` and no yard role |
| `0x4000` | `sys+0xdc < 0` and any role bit; `0x8000` `sys+0xdc < 0`; `0x10000` `sys+0xdc > 0` |
| `0x20000` | not HQ, `sys+0x74 > 0` and `sys+0x64 > 0` |

Key slots (`rec+0x70 + 4*k`): `3` `rec+0x7c` own systems, `4` `rec+0x80` enemy systems, `7` `rec+0x8c` mean popularity (sum / own count), `8` `rec+0x90` sum of `sys+0x4c`,
`9` `rec+0x94` sum of `sys+0x84`, `10` `rec+0x98` sum of `sys+0x80`, `0x11` `rec+0xb4` shipyards at systems with no shipyard role, `0x13` `rec+0xbc` training at no training role,
`0x14` `rec+0xc0` yards at systems with a yard role, `0x15` `rec+0xc4` yards at systems with no yard role, `0x1c` `rec+0xe0` systems with `sys+0xdc < 0`, `0x1d` `rec+0xe4` with `sys+0xdc > 0`.
System key slots (`sys+0x48 + 4*k`): `0` popularity, `4` shipyards, `5` training, `6` yards `sys+0x60`, `7` `sys+0x64`, `0xb` `sys+0x74`, `0xe` `sys+0x80`, `0xf` `sys+0x84`, `0x25` `sys+0xdc`, `0x33` `sys+0x114`.

## F2. Goal token (`FUN_004199b0`, `FUN_004178f0`)

`c+8` pending bits, `c+0xc` token (start `0x80`), `c+0x10` seed (start `0x40`).

- `c+8 == 0`: token = seed, seed >>= 1, wrapping 0 -> `0x80`. The token therefore keeps cycling `0x40,0x20,...,1,0x80` when nobody is pending.
- `c+8 != 0`: token >>= 1 (wrapping to `0x80` when it reaches 0) until `c+8 & token != 0`, then that bit is cleared. The new holder's pending bit is consumed.
- Bits: module 3 `0x1`, module 2 `0x40`, module 1 `0x80`. A module builds a facility only when `c+0xc` equals its bit, and rotates the token itself right after it picks a base (module 1: before the site pick; modules 2, 3: after the producer pick succeeds).
- Each facility pass first clears its own pending bit (`FUN_004d7060` case 4 `& ~1`, `FUN_004db2f0` case 1 `& ~0x80`, `FUN_004e2400` case 7 `& ~0x40`).
- Module 1 also tests `c+4 & 0x40` (set by `FUN_004daf10` when no yard site exists; clear point UNSURE).

## F3. Module 3: defences, mines and refineries (`FUN_004d6d30`, ctor `FUN_004d6b20`, weight `m+0x24 = 9`)

Fields: `m+0x1c` arm state (must be 1, else `m+0x38 = m+0x3c = 0`, done), `m+0x20` done, `m+0x34` order counter, `m+0x38` phase, `m+0x3c` step,
`m+0x40` rank holder, `m+0x50` producer id, `m+0x54` defence/mine target system, `m+0x58` new-yard site, `m+0x5c` class id, `m+0x60` quantity,
`m+0x64` yards cap, `m+0x68` site free slots, `m+0x6c` role list (systems with `0x2000000`), `m+0x74` used list.

Phase dispatch (`FUN_004d6d30`): `0x3ec` -> `FUN_004d7060` (new construction yards), `0x3ed` -> `FUN_004d6da0` (surplus scrap), `0x3ee` -> `FUN_004d6e70` (defences, mine, refinery);
any other phase -> `0x3ed`, step 0. Cycle: `0x3ed` -> `0x3ee` -> `0x3ec` -> `0x3ed`, each phase ending with `m+0x20 = 1` only when it emitted an order or finished its pass
(see tables).

### F3.1 Budgets

| Function | Formula |
|---|---|
| `FUN_004da4d0` (defence unit budget) | `(m+0x24 * c+0x184 / 100) * 90 / 100 - c+0x1dc`; 0 when it exceeds `c+0x184 - c+0x188` |
| `FUN_004da530` (yard budget) | `(m+0x24 * c+0x184 / 100) * 10 / 100 - sum(sys+0xd8 over m+0x6c)`; 0 when it exceeds spare |

### F3.2 Phase `0x3ed` (`FUN_004d6da0`)

| Step | Action | Next |
|---|---|---|
| 0 | | 1 |
| 1 | `FUN_004d71e0`: sector `FUN_00419a00(c, rec.24 & 0x10000, key 0x1d, dir 1)` (sector with the most surplus systems), then `FUN_0041a340(c, sector, C=1, key 0x25, dir 1)` -> `m+0x54` = own system with the largest `sys+0xdc`; returns 1 only if that `sys+0xdc > 0` | 1 -> step 2, else phase `0x3ee`, step 0 (no done) |
| 2 | `FUN_004d7330` Scrap at `m+0x54`: surplus shields `a = sys+0x64 - sys+0x6c` take the first `a` objects of family `0x24`; surplus batteries `b = sys+0x74 - sys+0x7c` then take up to `b` of family `0x22`, then of `0x23` (list order) | phase `0x3ee`, step 0, done (returns the Scrap order, or null if the lists are empty and then no Scrap) |
| 3, 7 | step 3 `FUN_004d76a0` (MISSING) decides; step 7 = `FUN_004d8520` over-budget scrap (see 3.5); nothing in the shown code sets step 3 or 7 in this phase. UNSURE who does | step 7 -> phase `0x3ec`, done |

### F3.3 Phase `0x3ee`: defence build (`FUN_004d6e70`)

| Step | Function | Condition and action | Next |
|---|---|---|---|
| 0 | | | 4 |
| 4 | `FUN_004d7ac0` | walk `m+0x6c`: a system with no record is dropped; own system that lacks `0x2000000` loses the role (bit cleared, dirty, removed); first own system with `0x2000000`, `sys+0x60 > 0`, `A & 0x800`, `A & 3 == 0` is chosen; `m+0x50 = FUN_00509670(obj,1)` (the producer id). None -> `m+0x50` invalid | found -> 1; else step 0, phase `0x3ec` (no done) |
| 1 | `FUN_004d76f0` | choose the target `m+0x54`, see 3.4; returns 1 when a defence is needed | 1 -> 6, 0 -> 9 |
| 6 | `FUN_004d7cf0` | Scrap to free a slot at `m+0x54` (3.6); order -> done, `m+0x34++` | 5 |
| 5 | `FUN_004d8460` | `m+0x60 = 0`; budget `FUN_004da4d0`; `<= 0` -> 9 (or 7 if `< 0`); else `sys = rec(m+0x54)`; needs `sys+0x84 > 0`; class pick (3.7); if class id high byte != 0, cost != 0 and cost <= budget then `m+0x60 = 1` -> `0xb`; otherwise 9 (a system without free slots also gives 9: the stale `iVar4` is the module pointer and never <= budget) | `0xb`, 9 or 7 |
| 7 | `FUN_004d8520` | scrap one defence (3.5); no order -> 9; order -> end of pass | 9 or end |
| 9 | | `c+0x1d4 < c+0x1d0` (refineries < mines): class `0x2d000002`, site `FUN_004d99d0`; otherwise class `0x2c000001`, site `FUN_004d98a0`. Found -> `m+0x60 = 1`, step 10; not found -> step 0, phase `0x3ec` | 10 |
| 10 | `FUN_004d9b00` | Scrap every shipyard (`0x28`), training facility (`0x29`) and construction yard (`0x2a`) object at `m+0x54` (all of them; purpose UNSURE); order -> done, `m+0x34++` | `0xb` |
| `0xb` | `FUN_004d9e70(m, m+0x54)` | Destination: team = `m+0x50`, target `m+0x54`; order -> step `0xc`, done, `m+0x34++` | `0xc` (no order: end of pass) |
| `0xc` | `FUN_004d9fb0` | Build `0x210`: side `m+0x30`, team `m+0x50`, class `m+0x5c`, quantity `m+0x60` | end of pass |

End of pass: step 0, phase `0x3ec`, done. A budget of exactly 0 still reaches step 9: no budget test guards the mine or refinery, only the executor's `0xf3` refusal.

### F3.4 Target choice `FUN_004d76f0` (step 1)

1. `m+0x54 = 0x90000109`. Let C be Coruscant's record. If C is not own (`sys+0x30 & 1 == 0`) or `A & 2`: `m+0x54` invalid.
   Else if `C.A & 0x40000000 == 0`: need = 1 when `C.sys+0x64 < c+0x36c` (default 2); else (Death Star shield wanted) need = 1.
2. If need == 0: `m+0x54` from two sector lists intersected: `FUN_00419a00(c, rec.24 & 0x4000, key 0x1c, dir 1)` and `FUN_00419b80(c, m+0x50, rec.24 & 0x4000, dir 2)`; then
   `FUN_0041a340(c, sector, A=0x4000, C=1, D=0x4000003, key 0x25, dir 2)`: the own, unblocked, role-bearing, assigned system with the largest deficit.
3. If `m+0x54` is still not a system: reset to Coruscant; need = 1 only when C is own, `C.sys+0xdc < 0`, `C.A & 2 == 0`; otherwise clear.
4. If still invalid: `FUN_00419b80(c, m+0x50, rec.24 & 0x8000, dir 2)` then `FUN_0041a340(c, sector, A=0x4000, C=1, D=3, key 0x25, dir 2)`.
5. If `m+0x54` is a system: need = 1 when its `sys+0xdc < 0`. If it is not a system: when `FUN_004da4d0 > 0` and `c+0x374 < 2`, `FUN_00418240(c, 4)` raises the plain-system shield target `c+0x374` by 1.

### F3.5 Scrap over budget `FUN_004d8520`

Selection of a system into `m+0x54` (first success wins): (1) sector with `rec.24 & 0x10000`, key `0x1d` dir 1, then own system with largest `sys+0xdc`;
(2) sectors with `rec.24 & 0x200e0`, key 3 dir 1, systems own, `D=0x3e00000`, `F=0x40000000`, keys 7 and `0xb` dir 1 (rank sum); (3) sectors `rec.24 & 0x20000`, key 3, same systems with `D=0`.
Then it scraps the first object of family `0x22`; if none the first of `0x23`; if none the first of `0x24` (one object). Order `0x200`.

### F3.6 Make-room Scrap `FUN_004d7cf0` (step 6) and `FUN_004d8fe0` (yard pass step 6)

Both scrap, at their target system T: (a) all shipyards unless `T.A & 0x200000`; (b) all training facilities unless `T.A & 0x400000`; (c) in `FUN_004d7cf0` all construction yards unless `T.A & 0x3800000`
(`FUN_004d8fe0` instead scraps all family-`0x25` objects). If nothing was queued and `T.sys+0x84 == 0`: one refinery (first of `0x2d`) when `c+0x1d0 < c+0x1d4` or `T.sys+0x50 == 0`; if still nothing, one mine
(first of `0x2c`) when `c+0x1d4 <= c+0x1d0` or `T.sys+0x54 == 0`. `FUN_004d7cf0` with a system that already has free slots may return no order. Order `0x200`.

### F3.7 Defence class (`FUN_004d8460`)

`FUN_0041b230(c, &m+0x5c, 0x2a, mask, key, 1)`:

| Condition on `sys = rec(m+0x54)` | mask | key | Meaning |
|---|---|---|---|
| `A & 0x40000000` | `0x40000` (family `0x25`) | `0x10000` (shield strength) | Death Star shield |
| else `A & 0x8000 == 0` | `0x80000` (`0x22`, `0x23`) | `0x40000` (attack strength) | strongest weapon defence |
| else | `0x20000` (`0x24`) | `0x10000` | strongest shield-count defence |

Direction 1: the largest key. The returned maintenance must be `<= budget`; the build quantity is always 1.

### F3.8 Phase `0x3ec`: more construction yards (`FUN_004d7060`)

| Step | Function | Action | Next |
|---|---|---|---|
| 0 / other | | | 1 |
| 1 | `FUN_004d8ad0` | site `m+0x58`: first, over `m+0x6c`, the own system with `A & 3 == 0`, `sys+0x114 > 0`, and the fewest yards (`sys+0x60 < best`, start 1000); `m+0x68 = sys+0x114`. If none: sectors `rec.24 & 0x80` ranked by key `0x15` dir 1 (plus key 4 dir 2 when `m+0x6c` is non-empty), then systems `FUN_0041a340(c, sector, C=1, D=0x3800003, F=0x40000000, key 6 dir 1)` (plus key `0x33` dir 1 when the list is non-empty); the winner is appended to `m+0x6c` and gets `A |= 0x2000000`, dirty. Returns 1 if found | found -> 4, else end of pass (no goal setter) |
| 4 | `FUN_004d8db0` | `c+8 &= ~1`. If `c+0xc == 1` and `FUN_004d8db0` succeeds: `FUN_00419b80(c, m+0x58, rec.24 & 0x1000, dir 2)` -> sector, then `FUN_0041a400(c, sector, m+0x58, A=0x800800, C=1, D=3, dir 2)` -> producer base; `m+0x64 = sys+0x60`, minus 1 when above 1; `m+0x50 = FUN_00509670(obj,1)`; site `m+0x58` appended to `m+0x6c` with `0x2000000`. On success rotate the token (`FUN_004199b0`) | success -> 6; else `0xd` |
| 6 | `FUN_004d8fe0` | make-room Scrap at `m+0x58` (3.6); order -> done, `m+0x34++` | 5 |
| 5 | `FUN_004d9520` | `m+0x60 = 0`; budget `FUN_004da530`; `< 0` -> 8; `<= 0` -> end; else class `FUN_0041b230(c, &m+0x5c, 0x2a, 0x10000, 0x4000, 2)` (the yard class with the smallest `+0x5c`), `m+0x60 = budget / maintenance`; cap by `m+0x64`, then by `min(sys+0x84 at m+0x58, m+0x68)`; `> 0` -> `0xb` | `0xb`, 8 or end |
| 8 | `FUN_004d95c0` | retire a yard: over `m+0x6c`, drop systems with `sys+0x60 < 1` (clear the role); skip systems without `A & 0x1000`; a system with exactly one yard is chosen only when the list has more than one system (and is removed, role cleared); a system with 2 or more yards is chosen at once; Scrap the first construction yard there | end |
| `0xb` | `FUN_004d9e70(m, m+0x58)` | Destination: team `m+0x50`, target `m+0x58`; order -> step `0xc`, done | `0xc` |
| `0xc` | `FUN_004d9fb0` | Build `0x210`: team `m+0x50`, class `m+0x5c`, quantity `m+0x60` | end |
| `0xd` | `FUN_004da0d0` then `FUN_004da610` | goal setter and rotation (section 6) | end |

End of pass: step 0, phase `0x3ed`, done.

## F4. Module 1: construction yards (`FUN_004dabe0`, ctor `FUN_004da950`, weight 4)

Fields: `m+0x24` weight, `m+0x2c` c, `m+0x40` site, `m+0x44` producer id, `m+0x48` yard class id, `m+0x4c` quantity, `m+0x50` site free slots, `m+0x54` producer-system yard count,
`m+0x58` role list (`0x800000`), `m+0x60` used list, `m+0x68` rank holder. Budget `FUN_004dd050`: `m+0x24 * c+0x184 / 100 - sum(sys+0xd8 over m+0x58)`, 0 when above spare
(there is no 100 percent factor beyond the `/100`; the share is the whole budget).

Phase 0 (`FUN_004dabe0`):
1. `m+0x1c != 1` -> done.
2. `FUN_004daca0`: scan `m+0x58`: null system -> drop; own, `A & 0x800000`, not ready (`A & 0x800 == 0`) and it is the only entry with `sys+0x60 == 0` -> role cleared, dropped; ready (`0x800`) and `A & 3 == 0` -> `m+0x54 = sys+0x60`, `m+0x44 = FUN_00509670(obj,1)` and return 1. Found -> phase `0x3ec`.
3. Else `FUN_004daf10` (only when `m+0x58` is empty): rank sectors `rec.24 & 0x2000` by key 4 dir 2, key `0x15` dir 1, key 8 dir 1 (sum), systems `A & 0x800`, own, `D=0x3800000`, keys 6, 0, `0x33` dir 1; fallback sector `rec.2c & 4` by key `0x14` dir 1 and own system by key 6; if still none `c+4 |= 0x40`. A winner gets `A = (A & 0xfc7fffff) | 0x800000`, dirty, appended to `m+0x58`; if that system has `A & 0x800`, producer and `m+0x54` are set and the function returns 1 -> phase `0x3ec`, step 0.
4. Else: if `c+0xc != 0x80` or `c+4 & 0x40`, run the goal setter `FUN_004dcc40` (section 6); then `FUN_004dd120` (rotation); phase 0, done.

Phase `0x3ec` (`FUN_004db2f0`):

| Step | Function | Action | Next |
|---|---|---|---|
| 0 | | | 1 |
| 1 | | `c+8 &= ~0x80`; `c+0xc != 0x80` -> step 10; else rotate the token | 2 or 10 |
| 2 | `FUN_004db510` | site `m+0x40`: own, `A & 3 == 0`, `sys+0x114 > 0`, fewest yards in `m+0x58` (`m+0x50 = sys+0x114`); else sectors `rec.24 & 0x80` (key 4 dir 2, key `0x15` dir 1, rank sum), systems `C=1, D=0x3800003, F=0x40000000`, keys 6, 0, `0x33` dir 1; appended to `m+0x58` with `0x800000`. None -> phase 0, done | 3 |
| 3 | `FUN_004db820` | make-room Scrap at `m+0x40` (as 3.6 with `FUN_004d8fe0`); order -> done, `m+0x34++` | 4 |
| 4 | `FUN_004dbd60` | `m+0x4c = 0`; site must be a system; budget `FUN_004dd050`; `> 0`: yard class `FUN_0041b230(c, &m+0x48, 0x2a, 0x10000, 0x4000, 2)`, `m+0x4c = budget / maintenance`; cap by `min(sys+0x84 of m+0x40, m+0x50)`, then by `m+0x54` (no minus 1); `> 0` -> 8. Whenever `m+0x4c == 0` the function returns 6 | 8, 6; 0 ends the pass |
| 5 | `FUN_004dbe70` | retire a yard (as `FUN_004d95c0`); Scrap | end |
| 6 | | `c+0x1d4 < c+0x1d0`: class `0x2d000002`, site `FUN_004dc3f0`; else `0x2c000001`, site `FUN_004dc150`. No site -> end. Found: `m+0x4c = 1` | 7 |
| 7 | `FUN_004dc650` | Scrap at `m+0x40` if it has no role (`A & 0x3e00000 == 0`): shipyards, training facilities, then one refinery or mine (3.6 rules); order -> done, `m+0x34++` | 8 |
| 8 | `FUN_004dc9f0` | Destination: team `m+0x44`, target `m+0x40`; order -> step 9, done | 9 |
| 9 | `FUN_004dcb30` | Build `0x210`: team `m+0x44`, class `m+0x48`, quantity `m+0x4c` | end |
| 10 | | `FUN_004dcc40` goal setter | end |

Decompile note (UNSURE, verify): in `FUN_004dbd60` a budget below 0 sets the result 5 (retire a yard) but `m+0x4c` was zeroed at entry, so the final `if (m+0x4c == 0) result = 6` overwrites it. Step 5 is therefore unreachable as decompiled.
Mine site `FUN_004dc150`: sectors `rec.24 & 0x100` by keys 7, 10, 9 dir 1; systems `A & 0x80` by keys `0xe`, 0, `0xf` dir 1; fallback nearest sector to `m+0x44` with `rec.24 & 0x800` and `FUN_0041a400(..., A=0x40)`.
Refinery site `FUN_004dc3f0`: sectors `rec.24 & 0x200` by keys 7, 9; systems `A & 0x100`, `C=1`, `D=0x3e00003` by keys `0xf`, 0; fallback nearest sector `rec.24 & 0x400` and `FUN_0041a400(..., A=0x20)`.

## F5. Module 2: deficit relief (`FUN_004e2120`, ctor `FUN_004e1e50`, weight 3)

Fields: `m+0x40` mine site, `m+0x44` refinery site, `m+0x48` relief or yard site, `m+0x4c` producer id, `m+0x50` class id, `m+0x54` quantity, `m+0x58` yards cap, `m+0x5c` site free slots, `m+0x60` role list (`0x1000000`), `m+0x68` used list, `m+0x70` rank holder.
Budget `FUN_004e4950`: `m+0x24 * c+0x184 / 100 - sum(sys+0xd8 over m+0x60)`, 0 when above spare.

Phase 0: `m+0x1c != 1` -> done. `FUN_004e21d0` scans `m+0x60` (own, `A & 0x1000000`, `sys+0x60 > 0`, `A & 0x800`, `A & 3 == 0`; stale entries lose the bit) and sets `m+0x4c` to that system's producer -> returns 1.
Return 1: phase `0x3eb` when `c+0x1d4 <= c+0x1d0` (refinery), else `0x3ea` (mine); return 0: phase `0x3ec`.

Phase `0x3ea`/`0x3eb` (`FUN_004e25c0`):

| Step | Function | Action | Next |
|---|---|---|---|
| 1 | `FUN_004e2760` | only when `c+0x184 < c+0x188`: sectors `rec.24 & 0xe0` by keys `0x15`, `0x11`, `0x13` dir 1, systems `C=1, D=0x3e00000` by keys 6, 4, 5 dir 1 -> `m+0x48`; returns 0 when found, else 1 | found -> 2; else 3 |
| 2 | `FUN_004e2940` | Scrap one facility at `m+0x48`: the first shipyard, else the first training facility, else the first construction yard; order -> done, `m+0x34++` | 3 |
| 3 | `FUN_004e2c70` (mine) or `FUN_004e2f10` (refinery) | keep `m+0x40`/`m+0x44` if it is still `A & 3 == 0` and `A & 0x80` (mine) / `A & 0x100` (refinery); else nearest sector to `m+0x4c` with `rec.24 & 0x100` / `0x200`, then `FUN_0041a400(..., A=0x80 / 0x100, D=3)`; fallback `rec.24 & 0x800` / `0x400` with `A=0x40 / 0x20`. None -> reset, done | 4 |
| 4 | `FUN_004e31b0` | Scrap at the site: all shipyards unless `A & 0x200000`, all training unless `0x400000`, all construction yards unless `0x3800000`; sets step 8 whether or not an order resulted | 8 |
| 8 | `FUN_004e4300` | Destination: team `m+0x4c`, target the site | 9 |
| 9 | | `m+0x50 = 0x2d000002` (refinery) or `0x2c000001` (mine), `m+0x54 = 1`, `FUN_004e4440` Build `0x210`; end (phase 0, done) | |

No budget test guards this build either; only the executor refusal applies. The deficit gate lives in step 1's relief Scrap, not in the build; whether an arm predicate gates the whole module on `c+0x184 < c+0x188` is not visible in the decompiles (vtable `0x0065c838`). UNSURE.

Phase `0x3ec` (`FUN_004e2400`): steps 1, 2 as above; 3 `FUN_004e3530` new yard site `m+0x48` (same query as module 3's `FUN_004d8ad0` over `m+0x60`, `m+0x5c = sys+0x114`, role `0x1000000`; sectors by keys `0x15` and 4); no site -> end;
7 `c+8 &= ~0x40`, token `0x40` and `FUN_004e3810` (`rec.24 & 0x1000` sector nearest to `m+0x48`, system `A=0x800800, C=1, D=3`; `m+0x58 = sys+0x60` minus 1 above 1; `m+0x4c` producer; site appended to `m+0x60`) -> rotate, step 4, else 10;
4 `FUN_004e3a40` make-room Scrap at `m+0x48` (as `FUN_004d8fe0`); 5 `FUN_004e3f80` (class `0x2a`/`0x10000`/`0x4000`/dir 2 -> `m+0x50`, `m+0x54 = budget / maintenance`, cap `m+0x58`, cap `min(sys+0x84, m+0x5c)`; `< 0` -> 6, `> 0` -> 8); 6 `FUN_004e4020` retire a yard; 8 Destination to `m+0x48`; 9 Build `m+0x4c`/`m+0x50`/`m+0x54`; 10 `FUN_004e4540` goal setter and `FUN_004e4a20` rotation.

## F6. Goal setters (`FUN_004da0d0` module 3, `FUN_004dcc40` module 1, `FUN_004e4540` module 2) and rotation

Shared algorithm (budget = the module's facility budget):
1. Clear the site (`m+0x58` for module 3, `m+0x40` module 1, `m+0x48` module 2).
2. If budget >= 1: take the sector `FUN_00419a00(c, rec.24 & 0x80, key 0x15, dir 1)` (the sector with the most yards at unassigned systems); only that top sector is examined, and only if `rec+0xc4 > 0`.
   Rank its systems `FUN_0041a340(c, sector, C=1, D=0x3800003, F=0x40000000, key 6, dir 1)`; the first with `sys+0x60 > 0` and `sys+0xd8 <= budget` becomes the site.
3. Found: append it to the role list and set the module's role bit (`0x2000000`, `0x800000`, `0x1000000`), dirty.
4. Not found (also reached when budget < 1): if the role list has fewer than 2 entries and its head does not have `sys+0x60 > 0`, set the class `FUN_0041b230(c, &class, 0x2a, 0x10000, 0x4000, 2)` and, when its maintenance `<= budget`, `c+8 |= bit` (module 3 `1`, module 1 `0x80`, module 2 `0x40`).
   A class pick that fails returns 0, so with budget 0 the bit would be set. UNSURE whether this is intended.
Rotation (`FUN_004da610` module 3, `FUN_004dd120` module 1, `FUN_004e4a20` module 2): picks the role system with the most yards that has `sys+0x90 > 0` and is not in the new list, clears `sys+0x30 & 0x10000000` on the previous pick, sets it on the new one and swaps the used list. Consumers of the marker are `rec+0x188` and `rec.2c & 0x10000000` (`FUN_00432350`); what they do with it is untraced.

`FUN_004cafa0` (module 11's producer base, the "yards - 1" cap): nearest sector to `m+0x58` with `rec.24 & 0x1000`, then `FUN_0041a340(c, sector, A=0x800800, C=1, D=3, key 6, dir 1)`; `m+0x70 = sys+0x60`, minus 1 when above 1; `m+0x50 = FUN_00509670(obj,1)`; the system gets `0x200000` and is appended to `m+0x78`.
Modules 3 and 2 subtract 1 from the producer's yards (`FUN_004d8db0`, `FUN_004e3810`); module 1 does not (`FUN_004daca0`, `FUN_004daf10`).

## F7. Order summary

| Module | Phase | Orders | Quantity | Destination |
|---|---|---|---|---|
| 3 | `0x3ee` defence | Destination then Build `0x210` class from 3.7 | 1 | `m+0x54` |
| 3 | `0x3ee` | Mine `0x2c000001` or Refinery `0x2d000002` | 1 | `m+0x54` (nearest sector with `rec.24 & 0x100` / `0x200`) |
| 3 | `0x3ec` | yard class (`0x2a`/`0x10000`/`0x4000`, dir 2) | `budget / maint`, caps `m+0x64`, `sys+0x84`, `m+0x68` | `m+0x58` |
| 3 | `0x3ed`, `0x3ee` | Scrap surplus / over-budget / make-room | per lists | |
| 1 | `0x3ec` | yard, or mine/refinery | `m+0x4c` | `m+0x40` |
| 2 | `0x3ea`/`0x3eb`, `0x3ec` | mine or refinery, or yard; Scrap relief | 1 / `m+0x54` | site |

## F8. Open

- MISSING decompiles: `FUN_0041a400`, `FUN_00433e30`, `FUN_004d76a0`, `FUN_0041bd90`, `FUN_004fcdd0`, `FUN_004fcf00`, `FUN_004ecc70`, `FUN_00418240` internals beyond "+1", the arm predicate of module 2.
- Producer `+0x58` meaning; `c+4` bit names `0x4`, `0x40`, `0x1000000`; `c+0xa4`.
- Who sets module 3 steps 3 and 7 in phase `0x3ed`; whether `FUN_004dbd60` step 5 is truly dead.
- Why `FUN_004d9b00` scraps every yard at a mine site; whether module 2 is gated on the deficit outside `FUN_004e2760`.
- The `0x1000000` role bit in `c+...` for the Death Star shield flag only exists for Coruscant, so module 3's shield branch fires only there.

# Part U: units

Offsets: `m+` module object, `c+` side AI context (FUN_004182a0 / FUN_00418500), `sys+` system AI record (FUN_00433e40), `sec+` sector record (FUN_004321e0/00432350), `rec+` regional record in `c+0xd8` (FUN_00476f90 family). "id hi" = top byte of a 32-bit id.

## Budgets, class picks, queries and orders

- Budget (all unit and facility budgets): `share = m+0x24 * c+0x184 / 100; b = share * PCT / 100 - spent; if (b > c+0x184 - c+0x188) b = 0` (integer divisions, NOT min). FUN_004cc220 (m11 90), FUN_004cec20 (m10 80), FUN_004d66a0 (m7 80), FUN_004d1d90 (m5 70), FUN_004e19c0 (m6 80, minus c+0x250 minus c+0x1d8). Scrap tests: FUN_004caad0 (m11 `<0` at 90), FUN_004cd2f0 (m10 `<0` at 80), FUN_004d47b0 (m7 `<0` at **90**, not 80), FUN_004d0570 (m5 = FUN_004d1d90 `<0`). Spent counters: m11 c+0x24c, m10 c+0x254, m7 c+0x1e0, m5 c+0x2c4, m6 c+0x250+c+0x1d8 (writers FUN_0041ba80 / FUN_0041b7e0 / FUN_0041bc10, section 4).
- Facility ("0x3ec") budgets: m11 FUN_004cc280 = 10% − Σ`sys+0xd8` over m+0x78 (note: construction-yard maintenance, yet the site test below uses shipyard `sys+0xd0`; quirk kept as read). m10 FUN_004cec80 20% − Σ`sys+0xd0` over m+0x74. m7 FUN_004d6700 20% − Σ`sys+0xd0` over m+0x6c. m5 FUN_004d1df0 30% − Σ`sys+0xd4` over m+0x78. m6 FUN_004e1a20 20% − Σ`sys+0xd4` over m+0x7c. Same `> c+0x184−c+0x188 → 0` clamp.
- `FUN_0041b230(c,&out,type,mask,key,dir)` → FUN_0042e3c0: candidates = side's available classes of production family `type` (0x28 shipyard products = ships+fighters, 0x29 training products = regiments+special forces, 0x2a construction products = facilities; FUN_0052e580) whose class node (`c+0x138` table, FUN_0042e190) has `node+0x18 & mask == mask`. Key 0 → random pick. Else sort by node key (virtual slot +4 of the node, key code in `*param`), dir 1 = ascending list, dir 2 = descending, equal keys coin-flip (`FUN_0041cd80(10) < 5`, FUN_0041c250), rank 1 = head, **best = highest rank** (FUN_004357b0). So **dir 1 → largest key wins, dir 2 → smallest key wins**. Return value = winner's maintenance (`node+0x20`), `out` = winner id. `FUN_0041b270` (FUN_0042e720) returns the whole sorted list; successive FUN_004357b0 calls pop best, next best, ...
- FUN_004357b0 with several merged lists (FUN_00435790 called more than once before picking): only entries present in **every** list survive, score = sum of ranks, highest wins, earliest on tie (`<` strict). Used by FUN_004cacc0 (two sector lists, two system lists).
- Query helpers (all return sorted lists of ids; pop with FUN_004357b0):
  - FUN_00419a00(c, req24, req28, req2c, excl24, excl28, excl2c, keySlot, dir): sector list `c+0x44`; flags `sec+0x24/0x28/0x2c`; key = `sec + 0x70 + 4*slot`.
  - FUN_00419b80(c, startId, req24, req28, req2c, excl24, excl28, excl2c, dir): same filter, key = distance from startId's sector (FUN_0041b7b0 on `sec+0x40`); start id is walked up by `+0x1c` until it is a sector id (hi 0x80–0x8f). dir 2 = nearest first.
  - FUN_0041a340(c, sectorId, req28, req2c, req30, excl28, excl2c, excl30, keySlot, dir): systems of that sector (10-slot array at `sec+0x48`), flags `sys+0x28/0x2c/0x30`, key = `sys + 0x48 + 4*slot` (slot 0x15 → sys+0x9c, 0x26 → sys+0xe0, 4 → sys+0x58, 5 → sys+0x5c, 0x33 → sys+0x114).
  - FUN_0041a4c0(c, req38, excl38, keySlot, dir): own fleets `c+0x78`, flag word `fleet+0x38`, key = `fleet + 0x50 + 4*slot` (slot 0x17 → fleet+0xac free hangar slots, 9 → fleet+0x74).
  - FUN_0041b0d0 → FUN_00403460(c+0x8c characters, req30, req34, excl30, excl34, keySlot, min, max, dir): key `char + 0x40 + 4*slot` (slot 9 → char+0x64 = maintenance).
- Order construction (all production orders): side at order+0x20; team list = the **producer facility object** (id hi 0xa0–0xa1 construction yard, 0xa2–0xa3 shipyard, 0xa4–0xa5 training facility; checked in every order builder); class id at order+0x44; quantity at order+0x48. Destination 0x214: team = producer, vtable slot +0x2c(target id); target id hi 1..0xfe in module 10 (FUN_004ce350), 0x90–0x97 (system) in modules 7 and 6 (FUN_004d5dd0, FUN_004e0a30). Scrap 0x200 and Retire 0x242 carry only the team list (units). Build 0x211 ship/fighter (FUN_004cba90, FUN_004ce490, FUN_004d5f10), 0x212 regiment/special force (FUN_004d15f0, FUN_004e0b70), 0x210 facility (FUN_004cbcf0).
- Producer facility lookup `FUN_00509670(systemObj, kind)`: first child facility whose vtable `+0x1ec` returns `kind` (0 shipyard, 1 construction yard, 2 training facility). Id passed on is the facility id (not the system).
- Module step machine: `m+0x38` phase, `m+0x3c` step, `m+0x20` done flag, `m+0x34` order counter. All five modules return immediately (phase 0, step 0, done=1) when `FUN_005f4960(m+0x1c) != 1` (m+0x1c semantics UNSURE: an enabled flag). Unknown phase → set first phase, step 0. One step per call; an emitted Destination sets done and `m+0x34 += 1`.
- Quirk shared by the count code: `if (cost*count != budget && budget <= cost*count) count = budget/cost`; count may become 0 and the step ends without an order. If no class was found (id hi 0) the count is left unclamped (m10 FUN_004cd230, m7 FUN_004d46e0, m5 FUN_004d0590, m6 FUN_004df9a0).

## U1. Class nodes: node offset ← class (DAT) field

In-memory class record offset = file offset + 0x28 for file offsets ≥ 0x18. The five leading fields are stored elsewhere: node ctors read the family byte at `+0x2c` (FUN_004739b0 `0x18..0x1b`, FUN_00473090 `0x28/0x29/0x2a`), which equals file `family_id` (0x10) — i.e. leading fields sit at file+0x1c (UNSURE: inferred; `field2` is 1 in every CAPSHPSD row so +0x2c cannot be `field2`; Death Star row 136 has `family_id` 0x18). Common to all nodes (FUN_004736c0, 004739b0, 004734d0, 00472cc0, 00473090): `+0x1c` = `FUN_0053b860` = class `+0x48` (file 0x20 refined_material_cost), `+0x20` = `FUN_0053b870` = class `+0x4c` (file 0x24 maintenance_cost), `+0x24` = class `+0x50` (file 0x28 research_order), `+0x28` side: class `+0x44` (is_empire) == 0 → 1 (Alliance), else 2 (facility nodes: +0x44==0 → 1; elif +0x40 (is_alliance)==0 → 2; else 0 both). Node `+0x18` flags, `+0x1c` also key 4.

### Capital ship node (FUN_004739b0 ctor, FUN_00473930 key fn; CAPSHPSD, tools/dat-dumper types/capital_ships.rs)
Weapon matrix `W(arc,type) = class + 0x74 + 12*arc + 4*type` (FUN_0053bb80), arc 0 fore 1 aft 2 port 3 starboard, type 0 turbolaser 1 ion 2 laser (= file 0x4c + 12*arc + 4*type).

| node | key code (FUN_00473930, first match top-down: 0x200,0x80000,0x400,0x800,0x80,0x100,1,2,4,0x10000) | value | class field (file off) |
|---|---|---|---|
| +0x30 | 0x200 attack score | `weapon_recharge * A[best] / (T+I+L)[best]` where `A[a] = 100*(T+I) + 100*L/6`; best arc = max A, ties fore>aft>port>starboard order of the chain in FUN_004739b0; stays 0 when all A<=0 and for Death Star (skipped) | weapon_recharge_rate +0xdc (0xb4) |
| +0x34 | 0x400 | `maneuverability * weapon_recharge * B[best] / (L+T)[best]`, `B[a]= 10*L + 10*T/2` | maneuverability +0x68 (0x40) |
| +0x38 | 0x800 | `bombardment_modifier != 0` (0/1) | +0xd4 (0xac) |
| +0x3c | 0x10000 | `shield_recharge_rate * 100` | +0xe0 (0xb8) |
| +0x40 | (not a key) | hull | +0xc0 (0x98) |
| +0x44 | 0x80 | fighter_capacity | +0xe4 (0xbc) |
| +0x48 | 0x100 | troop_capacity | +0xe8 (0xc0) |
| +0x4c | 0x2 | detection | +0x5c (0x34) |
| +0x50 / +0x54 | – | weapon_recharge / shield_recharge | +0xdc / +0xe0 |
| +0x58 | 0x80000 | `node+0x30 * 10 / node+0x20` (score per maintenance) | derived |
| +0x5c | – | `node+0x30 * 10 / node+0x1c` | derived |
| +0x1c / +0x20 / key 4 / key 1 | 4 / 1 | refined cost / maintenance | +0x48 / +0x4c |

Flags (`+0x18`): `0x80` Death Star (`+0x2c` in 0x18..0x1b; then skips every other flag except hull/troop/fighter/bombard ones and `+0x30`/`+0x34` stay 0); non-DS: `0x100000` always; `0x400` gravity_well_projector (+0xcc, file 0xa4) > 0; `0x1` not 0x400 and any arc has T+I > 0; `0x2` not 0x400, troop_capacity == 0, any arc has L > 0 (tested after the T/I loop, FUN_004739b0 lines on `iVar6*10`); troop_capacity>0 → `0x200`, plus `0x4` when old flags have none of `0x403` and have `0x100000`; `0x8` when troop-capable and `0x1`; fighter_capacity>0 → `0x100`; bombard>0 → `0x800`; hull `<1100` (0x44c) `0x10`, `<2000` `0x20`, else `0x40` (DS too).
Computed values for the shipped CAPSHPSD (computed from the formulas above; not game-verified): e.g. Alliance `0x4b` (MC-80-class heavy, maint 175): flags 0x100b49, +0x30 8000, +0x34 800, +0x58 457; Empire `0x86` (maint 170): 0x100b49, 7000, 350, 411; Empire `0x88` Death Star row (id 136 in file order, maint 600): flags 0xbc0, +0x30 0, +0x44 24, +0x48 18, key 0x10000 = 10000. Gravity-well ships: Alliance `0x4a` flags 0x100c20, Empire `0x82` 0x100420.

### Fighter node (FUN_004736c0 ctor, FUN_00473640 keys; FIGHTSD, types/fighters.rs: …overall_attack 0x94, torpedoes 0x98, torpedoes_range 0x9c, squadron_size 0xa0, bombardment_defense 0xa4)
| node | key | value | class field |
|---|---|---|---|
| +0x30 | 0x200 | `(100*I + 100*L/6) * squadron / (L+I)` (fore arc only; set only if numerator>0) | squadron_size +0xc8 (0xa0), ion/laser fore W(0,1), W(0,2) |
| +0x34 | 0x400 | `x = maneuver*squadron*10` (laser term cancels, needs laser_fore>0); `x + x*shield_strength/8` (trunc toward 0) | maneuverability +0x68 (0x40), shield_strength +0x60 (0x38) |
| +0x38 | 0x10000 | squadron*25/10 | +0xc8 |
| +0x3c | 0x20000 | maneuverability | +0x68 |
| +0x40 | 0x800 | bombardment_defense | +0xcc (0xa4) |
| +0x44 | 0x2 | detection | +0x5c (0x34) |
| +0x48 | 0x8000 | torpedoes * squadron | +0xc0 (0x98) * +0xc8 |
| +0x4c | – | squadron_size | +0xc8 |
Flags `0x200000` only. Keys 1 maintenance, 4 refined cost. (`+0x30/+0x34` are only written conditionally; ctor relies on zeroed memory — UNSURE.) Shipped values: Alliance fighter 0x1: +0x34 1560, 0x2: +0x34 765, +0x48 144.

### Regiment node (FUN_004734d0 ctor via FUN_00473400, FUN_00473490 keys; TROOPSD: detection 0x34, bombardment_defense 0x38, attack_strength 0x3c, defense_strength 0x40)
`+0x30` = class `+0x64` (attack), `+0x34` = `+0x68` (defense), `+0x38` = `+0x60` (bombardment_defense), `+0x3c` = `+0x5c` (detection). Flags: `0x1000` when defense <= attack, `0x2000` when attack <= defense (both when equal). Keys: 0x1000 → +0x30, 0x2000 → +0x34, 1 maintenance, 2 → +0x3c, 4 refined cost.

### Special-force node (FUN_00472cc0; SPECFCSD, skill pairs base/var from file 0x30)
`+0x30` = class `+0x58` (file 0x30 diplomacy_base), `+0x34` = `+0x60` (0x38 espionage_base), `+0x38` = `+0x80` (0x58 combat_base), `+0x3c` = `+0x88` (0x60 leadership_base). Flags by class id (`FUN_00402e40`): `0x3c000001` or `…05` → `0x1000000`; `…02`/`…06` → `0x2000000`; `…03`/`…07` → `0x4000000`; `…04`/`…08` → `0x8000000`. Key function for this node: MISSING (module 5 only ever uses key 1 = maintenance via the generic slot; not decompiled here). Shipped SPECFCSD maintenance is 1 for ids 1–8 (so the "dearest" rule is a random pick among the available classes of that bit), id 9 maintenance 0.

### Facility node (FUN_00473090 ctor, FUN_00473060 keys; MANFACSD/PROFACSD records: processing_rate file 0x34)
`+0x30` = class `+0x5c` = processing_rate (key `0x4000`; key 1 maintenance, key 4 cost). Flags by `+0x2c`: family 0x28 → `0x4000` (shipyard), 0x29 → `0x8000` (training facility), 0x2a → `0x10000` (construction yard). Used by the facility goal check `FUN_0041b230(c, out, 0x2a, 0x4000|0x8000, 0x4000, 2)` = cheapest processing rate.

## U2. Goal bits (facility token)
`c+8` pending bits (start 0), `c+0xc` token (start 0x80), `c+0x10` seed (0x40) (FUN_004178f0). Each module's end-of-0x3ec step clears nothing; step "8/5/9" does `c+8 &= ~bit`, then `if c+0xc == bit && <site/producer ok>` → FUN_004199b0 (rotate token to next pending bit, clear that bit) and build; else skip to the end step. FUN_004199b0: `c+8 == 0` → token = seed, seed >>= 1 (0 → 0x80); else token >>= 1 (0 → 0x80) until `token & c+8`, then clear it from `c+8`. Bits (set by the module's end step): 0x2 m11 FUN_004cbe10, 0x10 m10 FUN_004ce810, 0x8 m7 FUN_004d6290, 0x4 m5 FUN_004d1970, 0x20 m6 FUN_004e15b0, 0x1 m3, 0x80 m1, 0x40 m2. The token comparisons inside each module are literal: m11 `c+0xc == 2` (FUN_004c8d20 step 8), m10 `== 0x10` (FUN_004ccb50 step 3), m7 `== 8` (FUN_004d3350 step 5), m5 `== 4` (FUN_004d0010 step 9), m6 `== 0x20` (FUN_004dda00 step 5).

## U3. System / sector flag glossary (only what these modules test)
`sys+0x28` (FUN_00433e40): 0x1/0x2 (tested together as `&3`, "not blocked/contested": UNSURE exact meaning; set from system flag `+0x88 & 4`), 0x20 `sys+0x84>0` free slots, 0x40 also `sys+0x80>0`, 0x100/0x180 free-slot variants, 0x200 own shipyard exists and `FUN_00509670(sys,0)+0x58 == 0` (idle), 0x400 same for training facility (kind 2), 0x800 same for construction yard (kind 1), 0x1000 any own production facility, **0x200000 = system is in module 11/10/7's shipyard list**, **0x400000 = in module 5/6's training list**, 0x800000/0x1000000/0x2000000 construction-yard registrations (modules 3/1/2), 0x3e00000 = any registration (facility system), 0x2000 regiment shortage (`sys+0x9c = sys+0xa0 − sys+0x90` when sys+0x90 <= sys+0xa0), 0x20000 regiment shortage vs `sys+0x8c`, 0x40000 `sys+0xe0 < 0` (fighter shortage), 0x100000 `sys+0x11c < 0`, 0x4000000 own, `&3==0`, no registration, 0x80000000 = HQ/Coruscant (`id&0xffffff == 0x109`) or FUN_00526a60 hit. `sys+0x30`: 1 own, 2 neutral, 4 enemy, 0x10000000 module-11 rotation mark (FUN_004cc360), 0x40000000. System counts: `sys+0x58` shipyards, `+0x5c` training facilities, `+0x60` construction yards, `+0x84` free slots (class slots − built), `+0x114` free-slot figure (`+0x4c − max(regiment target…)…`), maintenance sums `+0xd0` shipyards / `+0xd4` training / `+0xd8` construction yards, `+0xa4` fighters at system, `+0xa8` adequate fighters (all when no registration, else fighter node `+0x34 >= c+0x160`), `+0xac` own fighters, `+0xe0 = +0xa8 − target`, `+0x11c = +0xac − target` (target = c+0x388 plain / c+0x38c when `&0x3e00000` / c+0x390 when `&0x80000000`; when `sys+0x8c < 1` and plain: `+0xe0 = +0xa4`), `+0x9c` regiment shortage, `+0xa0` regiment target (c+0x37c plain / c+0x380 registered / c+0x384 HQ, then max with `+0x98`).
Sector `sec+0x24` (FUN_00432350; `sys+0x48+4i` = `param_1[i]`): 0x20 / 0x40 / 0x80 = an own `&3==0` system with free slots (`+0x114>0`) lacking the 0x200000 / 0x400000 / 0x3800000 registration (site candidates for a new shipyard / training / construction facility), 0x1000 own system with 0x800000 and 0x800 flags, 0x800000 own registered system with `sys+0x2000`, 0x1000000 a system with `sys+0xe0 < 0` (any), **0x4000000 = such a system that is registered (`&0x3e00000`)**, 0x40000000 a system with 0x80000000 that is own/neutral, 0x80000000 same but enemy. `sec+0x2c`: 0x400000 / 0x800000 etc.: see FUN_00432350 (module 6 selectors use sector `+0x24` bits 0x800000 FUN_004e1060, 0x400000 FUN_004e0ef0, 0x40000 FUN_004e1430, `+0x24 & 0x40000000` FUN_004e11a0 only).

## U4. Context `c+` fields: reader modules and writers

| field | meaning (as read) | writer | read by |
|---|---|---|---|
| c+4 bit 4 | own Death Star ship (fleet flag 0x10, ship node flag 0x80) | FUN_0041ba80 | m11 request |
| c+4 bit 0x1000000 | Interdictor/shield class available (class 0x1400004a Alliance / 0x14000082 Empire in FUN_004198a0; 0x25000004 in FUN_004197b0) | those two | m11 counter for bit 0x800 |
| c+4 bits 0x800000 / 0x400000 | posture engine (module 8) "fresh pick": 0x800000 when R<1.5 or G!=0, 0x400000 mode 5 | FUN_004e5440 side | m6 (raise on 0x800000, lower on 0x400000) |
| c+8 / c+0xc / c+0x10 | goal bits | modules, FUN_004199b0 | all |
| c+0x14 / c+0x18 | shared producer-system cache (m5/m6) | m6 writes 0x14, m5 writes 0x18 (each clears its own at start, falls back to the other's) | FUN_004d01a0, FUN_004df160 |
| c+0x1c / 0x20 / 0x24 | cache m7 / m10 / m11 | FUN_004d3ff0 (clears 0x1c, writes 0x1c), FUN_004cccc0 (0x20; falls back 0x1c then 0x24), FUN_004ca300 (0x24; falls back 0x1c) | those three |
| c+0x44 / 0x2c / 0x78 / 0x8c | sector list / system list / own fleets / characters | – | all |
| c+0xd8 | regional records (posture engine) | module 8 | m10, m11 |
| c+0x138 | class table object; `+0x18` research level, `+0x28..0x3c` = c+0x160..0x174 best-class figures (FUN_0042e210/FUN_0042e960) | FUN_004197b0/004198a0 chain | fighter adequacy (sys+0xa8), m11 |
| c+0x16c | = c+0x138+0x34 = FUN_0042e960(table, side, level, mask 0x100000, key 0x80): best fighter capacity of any available non-DS capital ship class | FUN_0042e210 | m11 carrier request |
| c+0x184 / 0x188 | maintenance capacity / load | FUN_00418970 / 00418990 | budgets |
| c+0x1d0 / 0x1d4 | own mines / refineries | FUN_0041b7e0 | m3/m1/m2 |
| c+0x1d8 / 0x1dc / 0x1e0 | regiment / defence / fighter maintenance in systems | FUN_0041b7e0 (sec+0xfc/0x100/0x104) | m6, m7 |
| c+0x20c | max over enemy-system sectors of sec+0x134 = max `sys+0x118` | FUN_0041b7e0 | m11 troop request |
| c+0x24c / 0x250 / 0x254 | maintenance capital ships / regiments aboard / fighters aboard (Σ fleet+0x88/+0x8c/+0x90 over own fleets) | FUN_0041ba80 (zeroed with 0x24c..0x2c3) | m11, m6, m10 |
| c+0x258 / 0x25c / 0x260 / 0x264 | Σ fleet+0xb4 / +0x7c / +0xbc / +0x94 over own fleets; **c+0x260 = troop capacity on `0x4`-flag (transport) ships** | FUN_0041ba80 | m11 troop request |
| c+0x268 / 0x26c / 0x27c | Σ over own fleets with `fleet+0x38 & 4` of: fleet+0x98 (# ships class-flag 1), fleet+0x9c (# class-flag 2), fleet+0x78 (fighter capacity) | FUN_0041ba80, FUN_00433780 | m11 |
| c+0x2c4 | special-force maintenance (Σ char+0x24 of own chars, FUN_0041bc10) | FUN_0041bc10 | m5 |
| c+0x2ec/0x2f0/0x2f4/0x2f8 | # own special forces per class bit k=0..3 with char `+0x34` bits 0x10000000<<k and 0x800000 (non-major chars, `char+0x30 & 0x20000000 == 0`) | FUN_0041bc10 | FUN_004d0480 (offset `c+0x184+code*4`, code 0x5a..0x5d) |
| c+0x2fc | max over sectors of `sec+0x140` = max `sys+0xbc` (enemy-stack figure; UNSURE, = Σ fleet+0xa4 of enemy fleets). **Zeroed by the character census (0x2c4..0x313)**, only refreshed by the sector census, so it is often 0 → threshold 8 | FUN_0041b7e0, zero in FUN_00418500 case 5 | FUN_004d0480 |
| c+0x364..0x390 | targets idx 0..11 = 3,3,2,1,1,0,0,3,9,0,6,12 (FUN_0041bd20); idx 6,7,8 regiment (plain, registered, HQ), 9,10,11 fighters; `FUN_00418240(c,i)` +1, `FUN_00418270(c,i)` −1 | m7, m6 | FUN_00433e40 |
| c+0x16c vs c+0x20c etc. census flags | census runs in FUN_00418500 case 5 (flags 0x20000000 sectors, 0x40000000 fleets, 0x10000000 characters) | – | – |

## U5. Module 11 — capital ships (FUN_004c8b30, weight 30)
Phases: `0x3f8` (FUN_004c8ba0: step 2 FUN_004c8e90 moves damaged ships to a repair system, step 3 FUN_004c94c0 splits a flagged fleet, step 4 FUN_004c9cb0 moves capital ships to a sector system not yet listed in `c+0xd8`; step 2 is read in `ship-damage-repair.md`) → `0x3f9` build → `0x3ec` facilities → `0x3f8`. Any other phase → `0x3f8`.
**Phase 0x3f9 (FUN_004c8c20)**: step default → 8. 8: FUN_004ca300 producer; found → 5 else → 0xd. 5: FUN_004ca5e0 request, → 9. 9: FUN_004ca910 → returns 0 (phase `0x3ec`, done), 10 (scrap), 0xf (build). 0xd: FUN_004caad0; `<0` → 10 else step 0, phase `0x3ec` (not done). 10: FUN_004cab20 Scrap, then step 0, phase `0x3ec`, done. 0xf: FUN_004cba90 Build, step 0, phase `0x3ec`, done.
- Producer FUN_004ca300: clear `c+0x24`; walk `m+0x78` systems: id hi must be 0x90–0x97 and record exist else drop from list; if not own (`sys+0x30&1`) or `sys+0x28 & 0x200000` clear: clear the flag, FUN_00433e30, drop. First system with `sys+0x58 > 0 && sys+0x28 & 0x200 && &3==0` → `c+0x24 = it`. If none, `m+0x50 = c+0x1c` (m7's cache) accepted if own, 0x200000, `+0x58>0`, 0x200, `&3==0`. Else fail. Success: producer = FUN_00509670(FUN_004f3220(side, sysid), 0) (shipyard object id → `m+0x50`).
- Request FUN_004ca5e0: `m+0x60` class mask, `m+0x64` key, `m+0x68` blocker. Counts over `c+0xd8` records using `rec+0x64` bits with the blocker clear in `m+0x28`: n1 (bit 1, blocker 0x1000000), n80 (0x80, 0x2000000), n4 (4, 0x4000000), n8 (8, 0x8000000), n10 (0x10, 0x10000000), n800 (0x800, 0x40000000, only when `c+4 & 0x1000000`). Decision (first match): n4>=1 → (0x100004, key 0x100, blk 0x4000000); else n1>=1 and >= each other count → (0x100001, 0x80000, 0x1000000); else n80>=1 and >= n4,n10,n8,n800 → (0x100002, 0x400, 0x2000000); else n10>=1 and >= n8,n800 → (0x100800, 0x800, 0x10000000); else n8>=1 and >= n800 → (0x100102, 0x80, 0x8000000); else n800>=1 → (0x100400, 0x10000, 0x40000000). If still unset, in order: [blocker 0x4000000 clear and `c+0x260 < c+0x20c`] → (0x100004, 0x100, 0x4000000); else [`m+0x30==2` (Empire), `|m+0x78| > 1`, `c+4 & 4` clear, blocker 0x20000000 clear] → (0x80, 0x200, 0x20000000); else [blocker 0x1000000 clear, `c+0x268 < 1`] → (0x100001, 0x80000, 0x1000000); else [0x2000000 clear, `c+0x26c < 1`] → (0x100002, 0x400, 0x2000000); else [0x8000000 clear, `c+0x27c <= c+0x16c`] → (0x100102, 0x80, 0x8000000). Nothing → mask 0 key 0 (UNSURE: FUN_004ca910 then queries `FUN_0041b270(c,0x28,0,0,1)` = random affordable shipyard class, ships or fighters).
- Class choice FUN_004ca910: `m+0x28 &= 0x80ffffff` (clears all blockers), `m+6c=0`, budget FUN_004cc220; `<0` → return 10; `>0` → ranked list FUN_0041b270(c,0x28,mask,key,dir 1) popped until the class table node `c+0x138[id].+0x20 (maintenance) <= budget`; then `m+0x5c = class`, `m+0x6c = 1`, return 0xf (also clears `c+0x24`). None fits → `m+0x28 |= blocker` (skips that request next pass), return 0. Quantity is always 1. The Death Star row requires a budget of at least 600, unreachable below capacity 2,223 (`death-star-product.md`).
- Order: Build 0x211, team = producer (`m+0x50`, id hi 0xa2/0xa3), class `m+0x5c`, quantity `m+0x6c` = 1, no Destination (FUN_004cba90).
- Scrap (FUN_004cab20): only when 90% − c+0x24c < 0. Fleet chosen by FUN_0041a4c0(c, 5, 0x402, 0xe, 2): own (`fleet+0x38` `0x1`) fleets that no regional record or task has adopted (`0x4`, `FUN_00432e40`), neither en route (`0x2`) nor holding an unfinished ship (`0x400`, `FUN_00433780.c:35-37`), key `fleet+0x88` (the sum of its ships' `+0x70`, `FUN_00433780.c:107`), smallest first (dir 2). Then FUN_0041ac80 -> FUN_00433450(fleet, 0x24000, 0x2800, 0xb, 2): its ships whose `+0x24` holds own (`0x4000`) and free (`0x20000`, no record holds it, `FUN_00484230`) and neither en route (`0x800`) nor unfinished (`0x2000`), key `+0x44 + 4 × 0xb` = `+0x70`, smallest. Id hi must be 0x14–0x1b (capital ship), so a free Death Star (`0x18`) in an unadopted fleet qualifies. Order Scrap 0x200 of that one ship. A fleet held by a record or task is never a candidate. hyp: ship `+0x70` is its class maintenance (the census reads `fleet+0x88` as `c+0x24c`).
**Phase 0x3ec (FUN_004c8d20)**: default → 7. 7: FUN_004cacc0 find a site system; found → 8. 8: `c+8 &= ~2`; `c+0xc == 2 && FUN_004cafa0` (construction-yard producer in the site's region) → FUN_004199b0, → 0xb; else → 0x10. 0xb: FUN_004cb1d0 Scrap order for facilities already at the site (see below); → 9. 9: FUN_004cb710; returns 0 (phase `0x3f8`, done), 0xc (budget <0: FUN_004cb7b0 scrap a facility), 0xe (Destination). 0xe: FUN_004cbbb0 Destination 0x214 (team = construction yard, target = site `m+0x58`) → 0xf. 0xf: FUN_004cbcf0 Build 0x210 (class `m+0x5c`, quantity `m+0x6c`). 0x10: FUN_004cbe10 + FUN_004cc360 (goal bit / list maintenance) then step 0, phase `0x3f8`.
- FUN_004cacc0 (site): among `m+0x78` systems own, `&3==0`, `sys+0x114 > 0`, lowest `sys+0x58 < 1000` → `m+0x58`, `m+0x74 = sys+0x114`. Otherwise: sectors FUN_00419a00(c, 0x20, key 0x11 (sec+0xb4), dir 1) (+ merge with key slot 4 dir 2 when m+0x78 non-empty), then systems FUN_0041a340(c, sector, 0,0, req30 1, excl28 0x200003, excl30 0x40000000, key slot 4 dir 1) (+ merge with slot 0x33 dir 1 when non-empty); the system is added to `m+0x78`, `sys+0x28 |= 0x200000`.
- FUN_004cafa0: nearest sector with `sec+0x24 & 0x1000` to the site (FUN_00419b80 dir 2), then its system with `sys+0x28 ⊇ 0x800800` by slot 6 (nearest/largest, dir 1): its construction yard FUN_00509670(obj,1) = producer; `m+0x70 = sys+0x60` (−1 if >1); registers the system in `m+0x78`.
- FUN_004cb710 count: facility budget FUN_004cc280 (>0): `best = FUN_0041b230(c,&m+0x5c,0x2a,mask 0x4000,key 0x4000,dir 2)` (cheapest processing rate); `count = budget/maint`, `min(count, m+0x70)`, then `min(count, sys+0x84, m+0x74)` for the site record → 0xe. Budget <0 → 0xc.
- Goal bit (FUN_004cbe10): facility budget `b`; if `b >= 1` scan sectors (FUN_00419a00(c,0x20,…,0x11,1)) with `sec+0xb4 > 0` and in each the systems (FUN_0041a340(c,id,0,0,1,0x200003,0,0x40000000,4,1)): first with `sys+0x58 > 0 && sys+0xd0 <= b` → candidate `m+0x58`, appended to `m+0x78` with `sys+0x28 |= 0x200000`. No candidate: if `|m+0x78| < 2` and the first system of `m+0x78` has no shipyard (`sys+0x58 <= 0`, or list empty) and `FUN_0041b230(c,&m+0x5c,0x2a,0x4000,0x4000,2) <= b` → `c+8 |= 2`. FUN_004cc360 rotates the 0x10000000 mark in `sys+0x30` across `m+0x78` systems by shipyard count (UNSURE role; FUN_004ca300 does not test it).

## U6. Module 10 — fighters for fleets (FUN_004cc9c0, weight 8)
Phases `0x3f2` (units), `0x3ec` (facilities, FUN_004ccb50); `0x3f2 → 0x3ec → 0x3f2`; unknown phase → `0x3f2`.
**0x3f2 (FUN_004cca20)**: default → 3. 3: FUN_004cccc0 producer; ok → 2 else 8. 2: FUN_004cd040 target; ok → 4 else 8. 4: FUN_004cd230 → 0 (phase `0x3ec`, done), 5 (scrap), 9 (Destination). 8: FUN_004cd2f0 over-budget? no → step 0, phase `0x3ec`; yes → 5. 5: FUN_004cd310 Scrap then step 0, phase `0x3ec`, done. 9: FUN_004ce350 Destination (done, `m+0x34++`) → 10. 10: FUN_004ce490 Build, step 0, phase `0x3ec`, done.
- Producer FUN_004cccc0: as FUN_004ca300 over `m+0x74`, clearing `c+0x20`, caches `c+0x1c` then `c+0x24`; producer `m+0x58` = shipyard object (kind 0).
- Target FUN_004cd040: `m+0x68 = 0`; start at record id `m+0x44` in `c+0xd8` (else the head); each record via FUN_00476f90: system `rec+0x40` (hi 0x90–0x97), `sys+0x28 & 2 == 0`; `need = Σ fleet+0xac (free hangar slots) of the record's own fleets (fleets with fleet+0x30 == record id, flag 1; key slot 0x17) − sys+0xa4` → `rec+0x9c`; need > 0 → target `m+0x5c = rec+0x40` (system), count `m+0x68 = rec+0x9c`, cursor `m+0x40 = rec id`, `m+0x44 = next rec`. No record: `FUN_0041a4c0(c, 0x100005, 0xc02, 0x17, 2)` (own fleets with flags ⊇ 0x100005, excluding 0xc02, fewest free hangar slots first); fleet id hi 8–0xf and in `c+0x78` → target = that fleet id, count 1.
- Class FUN_004cd230: budget FUN_004cec20; `<0` → return 5; `>0`: key = `0x8000` when `rand(10) > 5` (4 of 10) else `0x400`; `FUN_0041b230(c,&m+0x64,0x28,mask 0x200000,key,dir 1)` (largest torpedo load, or largest fighter `+0x34`); count `m+0x68` cut to `budget/maint` if `maint*count >= budget` (see quirk), then `min(count, FUN_00528040(producerObj,0))` (count of the producer's slots whose `+0x60&1 == 0`; UNSURE exact meaning); count>0 → clear `c+0x20`, return 9.
- Orders: Destination 0x214 team = producer (`m+0x58`, hi 0xa2/0xa3), target `m+0x5c`; then Build 0x211 team = producer, class `m+0x64`, quantity `m+0x68` (FUN_004ce350, FUN_004ce490). Destination needs the target id hi in 1..0xfe.
- Scrap FUN_004cd310 (80% − c+0x254 < 0): fleet FUN_0041a4c0(c,5,2,9,1) = own fleets `⊇5`, not `2`, key `fleet+0x74` largest; ship FUN_0041ac80(c, fleet, 0x24000, 0x800, 6, 1) (largest slot 6 among ships ⊇0x24000 not 0x800); id hi 0x14–0x1b; among that ship's fighter squadrons (list FUN_005039d0) the one with the smallest vtable `+500` value; Scrap 0x200 of that squadron.
- Facility step FUN_004ce810 (bit 0x10, token `== 0x10`, list `m+0x74`, sector flag 0x20 key 0x11, systems excl 0x200003 key slot 4, test `sys+0x58>0 && sys+0xd0<=b`, class `FUN_0041b230(…,0x2a,0x4000,0x4000,2)`). Sub-steps FUN_004cd580 (site), 004cd860 (producer), 004cdfd0 (count, same shape as FUN_004cb710), 004cda90/004ce5b0/004ce6f0 (Destination/Build) and 004ce070 (scrap): not re-read here — read the structure from module 11.

## U7. Module 7 — fighters for systems (FUN_004d3100, weight 17)
Phases `0x3f1` (move/scrap surplus, FUN_004d3170: 004d34c0, 004d3610, 004d3a90, 004d3db0 — not read), `0x3f2` (FUN_004d3220), `0x3ec` (FUN_004d3350); cycle `0x3f1 → 0x3f2 → 0x3ec → 0x3f1`.
**0x3f2**: default → 5. 5: FUN_004d3ff0 producer; ok → 2 else 10. 2: FUN_004d4370 target; ok → 6 else 10. 6: FUN_004d46e0 → 0 (phase `0x3ec`, done), 7 (budget <0), 0xb. 10: FUN_004d47b0 false → step 0, phase `0x3ec`; true → 7. 7: FUN_004d4800 (move/scrap) then step 0, phase `0x3ec`, done. 0xb: FUN_004d5dd0 Destination (done) → 0xc. 0xc: FUN_004d5f10 Build, step 0, phase `0x3ec`, done.
- Producer FUN_004d3ff0: `m+0x64 = 0`; walk `m+0x6c`; same filter as module 11 (own, 0x200000, `+0x58>0`, 0x200, `&3==0`), `m+0x64 = sys+0x58` (**shipyards at the producer system**), caches: writes `c+0x1c`, falls back `c+0x20` then `c+0x24`. Producer `m+0x50` = FUN_00509670(…,0).
- Target FUN_004d4370 (start id `m+0x50`, result `m+0x54`): (1) sector FUN_00419b80(…, req24 0x4000000, dir 2) = nearest sector with a registered system short of fighters; systems FUN_0041a340(c,sec,req28 0x40000,0,req30 1,excl28 0x4000003,0,0,slot 0x26,dir 2) = own systems with `sys+0xe0 < 0`, not plain, most negative first. (2) if none: sector `req24 0x40000000`, same systems with excl28 `3`. (3) if none: sector `req24 0x1000000`, systems excl `3`. Found and `sys+0xe0 < 0` → return 1. None: if FUN_004d66a0 > 0 then `FUN_00418240(c,9)`, `(c,10)`, `(c,11)` (fighter targets +1 each) and return 0.
- Class FUN_004d46e0: budget FUN_004d66a0; `<0` → 7; `>0`: `m+0x60 = −sys+0xe0` of the target; plain system (`sys+0x28 & 0x83e00000 == 0`) key 1 dir 2 = cheapest fighter; else key `0x400` dir 1 = best `+0x34`; mask `0x200000`; count cut to `budget/maint` (quirk); then `min(count, m+0x64)`; count>0 → clear `c+0x1c`, return 0xb.
- Orders: Destination 0x214 (team = producer hi 0xa2/0xa3, target `m+0x54` system hi 0x90–0x97), Build 0x211 class `m+0x5c`, quantity `m+0x60`.
- FUN_004d4800 (over budget, 90% − c+0x1e0 < 0): pick a surplus fighter squadron: sectors `req24 0x2000000` key slot 0x22 dir 1, systems slot 0x35 excl30 0x40000, smallest vtable `+500` squadron; else sectors `0x8000000` (slot 0x19) etc.; then **Move 0x201** of it to a system short of fighters (FUN_0041a600 queries `0x100009`/`0x100005`) else **Scrap 0x200** (FUN_004f5cd0(0x200)). Details of FUN_0041a600/FUN_00419e90 MISSING from this read.
- Facility FUN_004d6290 (bit 8, token `== 8`, list `m+0x6c`, class mask 0x4000, same shape as 004cbe10).

## U8. Module 5 — special forces (FUN_004cfed0, weight 4)
Phases `0x3f4` (FUN_004cff20), `0x3ec` (FUN_004d0010); `0x3f4 → 0x3ec → 0x3f4`.
**0x3f4**: default → 9. 9: FUN_004d01a0 producer; ok → 1 else 6. 1: FUN_004d0480; found → 5 else 6. 5: FUN_004d0590 → 0 (phase `0x3ec`, done), 2 (retire), 0xb (build). 6: FUN_004d0570 true → 2, else falls through to step 0, phase `0x3ec`, not done. 2: FUN_004d0610 Retire. 0xb: FUN_004d15f0 Build.
- Producer FUN_004d01a0: clear `c+0x18`; walk `m+0x78`; own, `sys+0x28 & 0x400000`, `sys+0x5c > 0`, `0x400`, `&3==0`; falls back to `c+0x14`; producer = FUN_00509670(…,2) (training facility, id hi 0xa4–0xa5) → `m+0x60`.
- Need FUN_004d0480: cycles `m+0x40` code 0x5a→0x5b→0x5c→0x5d→0x5a (0 → 0x5a); per code `m+0x50` = class mask 0x1000000/0x2000000/0x4000000/0x8000000, `m+0x54 = 1` (dir), `m+0x48` = 1 when counter `c+0x184 + code*4` (c+0x2ec/0x2f0/0x2f4/0x2f8) `< max(c+0x2fc, 8)`, else 0; stops at the first code with count 1 or after a full circle (so the cycle position persists; a full circle with no need leaves `m+0x48 = 0` and returns found=false).
- Class FUN_004d0590: budget FUN_004d1d90; `<0` → 2; `>0`: `FUN_0041b230(c,&m+0x64,0x29,mask m+0x50,key 1,dir 1)` (dearest; all shipped = random among that bit's classes); count 1 cut by `budget/maint` quirk; count>0 → clear `c+0x18`, return 0xb. Build 0x212: team `m+0x60` (hi 0xa4–0xa5), class `m+0x64`, quantity `m+0x48`. **No Destination**.
- Retire FUN_004d0610 (70% − c+0x2c4 < 0): FUN_0041b0d0(c, 0x802000, 0, 0xa0, 0, 9, 0, 1000, 1) = characters with `char+0x30 ⊇ 0x802000` excluding `0xa0`, key `char+0x64` (maintenance) 0..1000, largest first; id hi must be 0x3c–0x3f; else the same with `0x2000` only. Order 0x242 (Retire) of that one. Char flag meanings UNSURE.
- Facility FUN_004d1970 (bit 4, token `== 4`, list `m+0x78`): `b = FUN_004d1df0`; `|list| < 2`; sectors `FUN_00419a00(c,0x40,…,0x13)` with `sec+0xbc > 0`, systems FUN_0041a340(c,id,0,0,1,0x400003,0,0x40000000,5,1) with `sys+0x5c > 0 && sys+0xd4 <= b`, mark `sys+0x28 |= 0x400000`; bit set when `FUN_0041b230(c,&m+0x64,0x2a,0x8000,0x4000,2) <= b` and no usable first system (`sys+0x5c <= 0`).

## U9. Module 6 — regiments (FUN_004dd7a0, weight 22)
Phases `0x3ef` (FUN_004dd840: 004ddb70, 004de130, 004de6f0, 004deba0 — move/surplus, not read), `0x3f0` (FUN_004dd8d0), `0x3ec` (FUN_004dda00): `0x3ef → 0x3f0 → 0x3ec → 0x3ef`.
**0x3f0**: default → 5. 5: FUN_004df160 producer (list `m+0x7c`, flag 0x400000, `sys+0x5c>0`, 0x400, `&3==0`, writes `c+0x14`, falls back `c+0x18`, kind 2 training facility → `m+0x58`... note `m+0x64 = sys+0x5c`); ok → 6 else 0xb. 6: FUN_004df440 need; ok → 7 else 0xb. 7: FUN_004df9a0 → 0 (phase `0x3ec`, done), 8, 0xc. 0xb: FUN_004dfa70 (zeroes `m+0x60`, returns 0) → step 0, phase `0x3ec`, not done; **the Scrap step 8 is only reached from 7 when budget <0**. 8: FUN_004dfa80 Scrap. 0xc: FUN_004e0a30 Destination (target `m+0x54`) → 0xd. 0xd: FUN_004e0b70 Build 0x212 (class `m+0x5c`, quantity `m+0x60`).
- Need FUN_004df440: clears `m+0x60/6c/70/74` and `m+0x28 &= ~0xf`; rotates `m+0x78` (0 → 1 and **no selector run that call**; 1..4); selector order by `m+0x78`: 1: A(004e1060), B(004e0ef0), D(004e1430), E(004e12e0), C(004e11a0); 2: B, D, D, E, C, A; 3: E, C, A, B, D; 4: C, A, B, D, E; each later selector runs only while `m+0x54` is not a system id. Result `m+0x54` target system, `m+0x60` count, `m+0x6c` mask, `m+0x70` key, `m+0x74` dir. Selectors (start id `m+0x50`): A: nearest sector `req24 0x800000`, systems `req28 0x2000, req30 1, excl28 0x4000003`, key slot 0x15 (`sys+0x9c` shortage) dir 2 → mask 0x2000, key 0x2000, dir 1, count `sys+0x9c`; fail sets `m+0x28 |= 1`. B: sector `0x400000`, systems req28 0x20000 excl 3 slot 0x15 dir 2; if the system is registered (`&0x83e00000`) → mask 0x2000 key 0x2000 dir 1, else key 1 dir 2 (cheapest); count `sys+0x9c`; fail `|= 8`. C: sector `FUN_00419a00(c,0x40000000,…,key slot 0x1e dir 2)`, systems req28 0x2000 excl 3 slot 0x15 dir 2, mask 0x2000 key 0x2000 dir 1; fail `|= 2`. D: sector `0x40000`, systems req28 0x2000 excl 3 slot 0x15 dir 2, mask 0x2000 key 1 dir 2; fail `|= 4`. E: FUN_004e12e0 walks the parent chain of `m+0x50` to the first system id and targets that (attack): count `m+0x64`, mask 0x1000, key 0x1000, dir 1. After selection: if `(m+0x28 & 0xf) == 0xf` (all four failed) and `c+4 & 0x800000` and FUN_004e19c0 > 0: `c+0x384 < 12` → `FUN_00418240(c,8)`, `c+0x380 < 6` → `(c,7)`, `c+0x37c < 4` → `(c,6)`. The step returns found when `m+0x54` is a system with a record.
- Class FUN_004df9a0: budget FUN_004e19c0; `<0` → 8; `>0`: `FUN_0041b230(c,&m+0x5c,0x29,mask m+0x6c,key m+0x70,dir m+0x74)`; count `m+0x60` cut to `budget/maint` (quirk), `min(count, m+0x64 = sys+0x5c)`; count>0 → clear `c+0x14`, return 0xc. Afterwards, whenever `c+4 & 0x400000` and budget < 1: `c+0x384 > 9` → `FUN_00418270(c,8)`, `c+0x380 > 3` → `(c,7)`, `c+0x37c > 0` → `(c,6)`.
- Orders: Destination 0x214 team = producer (hi 0xa4–0xa5, id check 0xa4..0xa5), target = system `m+0x54`; Build 0x212 class `m+0x5c`, quantity `m+0x60`.
- Scrap FUN_004dfa80: sector `FUN_00419a00(c,0x80000,…,0x1f,2)`, system FUN_0041a340(c,id,0x10000,0,1,0,0,0,0x13,2); the own regiment with the smallest vtable `+0x1e0` value in that system; Scrap 0x200.
- Facility FUN_004e15b0 (bit 0x20, token `== 0x20`, list `m+0x7c`, same as module 5 with `sys+0x5c`/`+0xd4`).

## U10. Open
1. Leading class-record field placement (inferred, section 1); ship record `param_1[i]` meanings in FUN_00433780 (only fields 5, 7, 8, 10 read as fighters-aboard, fighter capacity, troops-aboard, troop capacity).
2. `rec+0x64` need bits (module 8 posture engine) that drive module 11; only the writer FUN_00477e80 was read: bits `0x1 ← rec+0x6c<0`, `0x2 ← +0x70<0`, `0x4 ← +0x74<0`, `0x8 ← +0x78<0`, `0x10 ← +0x80<0`, `0x20 ← +0x88`, `0x40 ← +0x8c`, `0x80 ← +0x90<0`, `0x800 ← +0x84<0` (each a shortfall after FUN_004adaf0). FUN_00477cb0 maps `+0x6c` Σ fleet+0xb4, `+0x70` fleet+0xb8, `+0x74` fleet+0x84, `+0x78` fleet+0x78, `+0x80` fleet+0x68, `+0x88` fleet+0x7c, `+0x8c` fleet+0x70, `+0x90` fleet+0x9c.
3. Meaning of `sys+0x28 & 3`, fleet `+0x38` bits 1/4, character flag bits (`+0x30` 0x2000/0x800000/0x20/0x80) and `m+0x1c`.
4. MISSING decompiles: special-force node key function (only key 1 used), FUN_004cd350 (referenced name, absent), FUN_0041a600, FUN_00419e90, module 10/7/6 sub-steps listed as "not read", and the surplus-move phases `0x3f8/0x3f1/0x3ef`.
5. The mask-0 fallback in FUN_004ca910. (`FUN_00528040(producer, 0)` counts the producer's usable yards of its kind and side whose `+0x60` bit 0 is clear: `research.md`, "Passive points"; `sys+0x88` bit 10 is that count being non-zero for the shipyard producer: `ship-damage-repair.md`.)
6. Module 1 scraps the unlisted shipyards of its first site on its first pass; no starting state exempts them. The selector is one predicate: `FUN_004db820` (step 3) takes every usable shipyard (iterator mode 1, `object-state-flags.md`) at `m+0x40` when `sys+0x28` lacks `0x200000`, every training facility when it lacks `0x400000`, then every family-`0x25` object, and returns Scrap `0x200`. The role bits of `sys+0x28` start at 0 (`FUN_00433b10.c:24-28`; `FUN_00433e40` keeps only the persisted role bits, line 75) and the shipyard role has these writers only: `FUN_004cacc0`, `FUN_004cafa0`, `FUN_004cbe10` (module 11), `FUN_004cd580`, `FUN_004cd860`, `FUN_004ce810` (module 10), `FUN_004d5000`, `FUN_004d52e0`, `FUN_004d6290` (module 7). The scheduler steps the modules in ascending kind: `FUN_005f4f10` inserts each module into a tree keyed by its kind (`+0x18`, set by `FUN_004ec940` from the kind `FUN_004bea20` passes; comparator `FUN_005f5560.c`), `FUN_004be980` starts at the leftmost node (`FUN_005f5060.c`) and follows the in-order links, so module 1 runs before modules 2, 3, 5, 6, 7, 10 and 11. Every module is armed unconditionally (`FUN_004cc990`, vtable slot `+0x10`). First round, `c+8 = 0`, token `c+0xc = 0x80` (`FUN_004178f0.c:61-63`): phase 0 `FUN_004daf10` lists the system with a ready yard (`A & 0x800`, no yard role), step 0 to 1, step 1 rotates the token, step 2 `FUN_004db510` takes the listed system while `sys+0x114 > 0`, step 3 scraps. `sys+0x114` subtracts only role-listed facilities (F1.3), so an unlisted shipyard never makes a system look full and a capacity test cannot exempt it. The site pick of modules 11, 10 and 7 (`FUN_004cacc0`) ranks sectors by shipyards at unlisted systems (`rec+0xb4`, largest first) and so lists existing shipyards, but it runs after module 1. Scrap acceptance: the menu enable is own side and not en route (`facility-order-lists.md`); hyp: the order validity (`FUN_0051ff30` → `FUN_00553960`, per-object slot `+0x1c` of the facility) has no further test. An original save confirms it: on day 2 the Empire's three manufacturing objects at the mine site (two shipyards and a training facility) are destroyed records, while its mines and refineries there are live and a new mine is queued for that site (step 7, `FUN_004dc650`, at `m+0x40` when the site has no role). One pass scraps at the yard site (step 3) and again at the mine site (step 7); each is its own step and order. Module 1 never retires a construction yard: `FUN_004dbd60` zeroes `m+0x4c` (line 14), so a negative budget's result 5 (lines 26-29) is overwritten with 6 (lines 56-60) and `FUN_004dbe70` is not reached. port: in the headless seed-42 galaxy each side's module 1 scraps three shipyards at its yard site on day 1 (Empire at Uvena, Alliance at Balmorra) and builds a construction yard there.
