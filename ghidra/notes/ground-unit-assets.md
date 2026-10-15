---
title: "Ground Units and Facilities: Art, Build Lists and Family Gates"
description: "How a troop, special-force or facility type gets its build, row, overview and encyclopedia art from its TEXTSTRA id, the eight facility sprites, the build-list rules, the factory registry's exact family bytes, and the literal ids in finders and the AI"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [troops, special-forces, facilities, art, build-selection, finders, mod-surface]
---

# Ground units and facilities

Answers task-graph node `or-q-mod-groundunits-j6tv` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-groundunits`; the facility sprite switch, the factory registry and the Troop Finder ids were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code.

## Answer

1. **Per-type art is computed from the record's TEXTSTRA id, not stored.** The class record's low 16 bits at `+0x30` are the DAT `text_stra_dll_id` (loader `FUN_00584c10`: `+0x2c` family dword, `+0x30` and `+0x32` two u16s; record `+0x18` is the tree key, the DAT record id). Every per-type picture uses `id = (class+0x30) & 0xfff`: the build-selection image is GOKRES bitmap `id` (`FUN_00437880`); the manufacturing band product is GOKRES `id` (`FUN_00457c90`); every status, defense-window and battle-summary row is GOKRES `id + 0x4000`, or plain `id` for the status portrait (`FUN_0042c3b0`); the encyclopedia entry is keyed `id + 0x1000` and read from ENCYTEXT as RCDATA `id + 0x1000` plus an ENCYBMAP/EDATA bitmap (`FUN_00422620`, `FUN_0045fa60`). All shipped records have all three: GOKRES `id` and `id + 0x4000` exist for every record of the five files, and ENCYTEXT `id + 0x1000` exists for all 33 records of the five files (table below).
2. **The sector window has no per-type art.** Its four overlay icons are chosen by kind and side only (`FUN_0045ca80`, STRATEGY 10771..10790); troop, special-force and defense objects feed the bottom-left icon, manufacturing and production facilities the top-left one (`FUN_0045b770` dispatch; `FUN_0045cdc0`, `FUN_0045ce80`). The only per-record art in the System window is the facility list picture: a **four-state sprite chosen by the record id** (1..6 for MANFACSD, 1..2 for PROFACSD) from six constants in `FUN_00458fe0`. An id past those has no picture.
3. **What decides "which yard builds it" is DAT, but the producer families are hard-coded.** A class is offered when its `production_family`/`next_production_family` pair (`class+0x24..+0x28`) covers the producer family, its refined cost is non-zero, its side flag is set and its research order is at or below the side's level (`FUN_00567880`). The producer family itself is a literal pair of switches to `0x28` (ships), `0x29` (troops and special forces), `0x2a` (facilities) (`FUN_00537ff0`, `FUN_0052e580`, and the gate `0x28..0x2b` in `FUN_00567880`). Shipped troops and special forces have `production_family 0x29`; facilities `0x2a`.
4. **Object creation is gated by an exact family byte.** The factory registry (`FUN_0051ebf0` into the table at `DAT_006b90f8`, `FUN_00540540`) holds one entry per family byte: `0x10`, `0x14`, `0x18`, `0x1c`, `0x3c`, `0x20`, `0x22`..`0x25`, `0x28`, `0x29`, `0x2a`, `0x2c`, `0x2d`; `FUN_0053f100` (create) and `FUN_0053a1e0` (save load) look it up with `id >> 24` and nothing else. The DAT header's family range (for example `0x10..0x14`) only selects which table holds the class (`FUN_0058ec90`). So a record whose own `family_id` is `0x11`, `0x2b`, `0x26`, `0x3d` and so on loads as a class but cannot be instantiated.
5. **The family gates that matter are ranges on the object's family byte**, in about 60 functions (sweep below). Troops are `0x10..0x13`, special forces `0x3c..0x3f`, defense `0x22..0x27`, yards `0x28/0x29/0x2a` one function each, mines `0x2c`, refineries `0x2d`. Three behaviours key on **one family byte inside DEFFACSD or on literal DatIds** instead of ranges: the blockade-withdrawal modifier counts only family `0x22` (`FUN_00527550`, `[0x22,0x23)`, the KDY-150 record, but any DEFFACSD record whose `family_id` is `0x22` counts), the Troop Finder counts exactly ten literal troop DatIds (`FUN_0046d8d0`), and special forces carry ids 1..9 literals in the AI, the Personnel Finder and the missions.
6. **`FUN_00560d50` is not ground combat.** It is the character manager's hook run by the object-move dispatcher `FUN_00514a60`; its `0x14..0x1b` and `0x30..0x3b` gates are capital ships and characters, and the per-object write `FUN_004ee350` sets a **character's `CommandKind`** (`+0x96`, `FUN_004f1f80` notifies event `0x1e2`; error string "Invalid CommandKind value"). It has no gate on `0x10..0x13` or `0x3c..0x3f` regiments. `ground-combat.md`, `modders-taxonomy.md` and `docs/mechanics/ground-combat.md` read the family bytes as "troops 0x14-0x1b / capital ships 0x30-0x3b"; the DAT headers and the class-profile builder `FUN_0042dd00` show the opposite (`CAPSHPSD` family `0x14`/`0x18`, `MJCHARSD` `0x30..0x35`, `MNCHARSD` `0x38`). The regiment-side "ground combat art" is only the battle-summary roster tab (`FUN_004714e0`, tab `0x67`, family `[0x10,0x14)`), which draws the generic row picture from `FUN_0042c3b0`.
7. **Both sides.** Art and gates are the same for both sides except where a literal list is per side: the Troop Finder's five columns (Alliance `0x10000002, 1, 5, 3, 4`; Empire `0x10000008, 0x0a, 7, 6, 9`), the Personnel Finder's special-force columns, the seeding of the two Army regiments `0x10000002`/`0x10000008` (`FUN_0051ab20`), and the side flags `is_alliance`/`is_empire` (`class+0x40`/`+0x44`) read by the build list (`FUN_00567880`) and by ownership (`FUN_004f27d0`, `facility-ownership.md`). A record's art is not side-switched: the side is in the TEXTSTRA id ranges (Alliance `0x2440..`, Empire `0x2480..`, by convention, not by code).

## Per-type assets: where each one comes from

`id` below is `text_stra_dll_id & 0xfff`. `ord` is the DAT record id (the class tree key at `class+0x18`, `FUN_005f5500`).

| Surface | Troops `0x10` (TROOPSD) | Special forces `0x3c` (SPECFCSD) | Defense `0x22..0x25` (DEFFACSD) | Yards `0x28/29/2a` (MANFACSD) | Mines `0x2c`, refineries `0x2d` (PROFACSD) | Source |
|---|---|---|---|---|---|---|
| Sector-window quadrant icon | none per type; generic kind 8 icon by side | same | same (kind 8) | none per type; generic kind 4 icon | same (kind 4) | `FUN_0045ca80` (read), dispatch `FUN_0045b770` (read) |
| System-window facility list picture (type 9) | not listed (managers only) | not listed | not listed (Defenses window instead) | by `ord`: 1 `0x232e`, 2 `0x2336`, 3 `0x233e`, 4 `0x2332`, 5 `0x233a`, 6 `0x2342`; state +0 built, +1 building (+2 when system side is 1), +3 en route; none for `ord` > 6 | by `ord`: 1 `0x2329` (mine), 2 `0x2346` (refinery); none for `ord` > 2 | `FUN_00458fe0` via `FUN_00539f50` (`*(class+0x18)`); `FUN_00454160` (read) |
| Page tab / page label | n/a | n/a | n/a | by family byte: `0x28` 10327, `0x29` 10330, `0x2a` 10333 | `0x2c` 10321, `0x2d` 10324 | `FUN_00454160`, `FUN_004568a0` (read) |
| Manufacturing overview band product mini | GOKRES `id` | GOKRES `id` | n/a | GOKRES `id` | n/a | `FUN_00457c90` (read) |
| Build-selection image (210 by 261 window, 195 by 63 box) | GOKRES `id`, bitmap resource module `param_1[0x53]` | GOKRES `id` | GOKRES `id` | GOKRES `id` | GOKRES `id` | `FUN_00437880` (read) |
| Build-selection list membership | `production_family 0x29`, side flags, cost non-zero, research order <= level | same (SPECFCSD record 9 has cost 0: never listed) | `production_family 0x2a` | `production_family 0x2a` | `production_family 0x2a` | `FUN_00567880`, `FUN_0052e580`, `FUN_00537ff0` (read) |
| Status-window portrait | GOKRES `id` | GOKRES `id` | GOKRES `id` | GOKRES `id` | GOKRES `id` | `FUN_00443130` calling `FUN_0042c3b0(.., 1, 1)` (read) |
| Defenses / battle-summary / fleet-window row picture | GOKRES `id + 0x4000`, plus overlay | same | same | same | same | `FUN_004a9ab0`, `FUN_0042c3b0(.., 0, 1)` (read) |
| Row overlay while the object is moving (bit `+0x50` `0x10` set, `0x08` clear) | STRATEGY `0x2cfb` (`+1` selected) except ids `0x10000002` and `0x10000008` (literals) | `0x2ced` (+`0x13` selected) except ids `0x3c000003` and `0x3c000005` (literals) | `0x2cf5` (+1) | `0x2cf1` (+1) | `0x2cf1` (+1) | `FUN_0042c3b0` (read) |
| Finder row | **ten literal DatIds, five columns per side** (see below); a regiment of another id is not counted | Personnel Finder row, four columns per side, ids `0x3c000001..8` | none | none | none | `FUN_0046d8d0` (read); `FUN_00465540` (role from `map-03`, hyp: not re-read) |
| Encyclopedia entry | list key `id + 0x1000`, one entry per key (dedupe `FUN_0060a860`); category 0x74 `[0x10,0x14)`; text ENCYTEXT RCDATA `id + 0x1000`; bitmap ENCYBMAP/EDATA by the same key | key as left; category 0x75 `[0x30,0x40)` (with characters) | category 0x72 `[0x20,0x30)` | category 0x72 `[0x20,0x30)` | category 0x72 `[0x20,0x30)` | `FUN_00422620`, `FUN_0045f100`, `FUN_0045fa60` (read) |
| Ground-combat art | none found; battle-summary roster rows only | none found | none found | n/a | n/a | `FUN_004714e0` (role from `map-03`), `FUN_0046f360` |
| Name | TEXTSTRA `text_stra_dll_id` (`FUN_00583c40`) | same | same | same | same | `FUN_00437880` (read) |

Shipped text ids and the resources they select (all present in the shipped DLLs; read directly):

| File | Records (`ord`: name, text id, `id`) | GOKRES `id` | GOKRES `id+0x4000` | ENCYTEXT `id+0x1000` |
|---|---|---|---|---|
| TROOPSD | 1 Alliance Fleet 9280/0x440, 2 Alliance Army 9281/0x441, 3 Sullustan 9282/0x442, 4 Wookiee 9283/0x443, 5 Mon Calamari 9284/0x444; 6 Stormtrooper 9344/0x480, 7 Imperial Fleet 9345/0x481, 8 Imperial Army 9346/0x482, 9 War Droid 9347/0x483, 10 Dark Trooper 9348/0x484 | present for 10/10 | 10/10 | 10/10 |
| SPECFCSD | 1 Guerrillas 9536/0x540 .. 4 Bothan Spies 9539; 5 Probe Droid 9600/0x580 .. 9 Bounty Hunters 9604 | 9/9 | 9/9 | 9/9 |
| DEFFACSD | 1 KDY-150 8704/0x200, 2 LNR I 8705, 3 GenCore I 8706, 4 Death Star Shield 8832/0x280, 5 LNR II 8707, 6 GenCore II 8708 | 6/6 | 6/6 | 6/6 |
| MANFACSD | 1..3 Orbital Shipyard, Training, Construction 8448..8450/0x100.., 4..6 Advanced 8451..8453 | 6/6 | 6/6 | 6/6 |
| PROFACSD | 1 Mine 8193/0x001, 2 Refinery 8194/0x002 | 2/2 | 2/2 | 2/2 |

The shipped ids sit in disjoint 12-bit windows (`0x001`, `0x100`, `0x200`, `0x280`, `0x440`, `0x480`, `0x540`, `0x580`). GOKRES also holds `id + 0x1000` for TROOPSD ord 2 and 8 and SPECFCSD ord 3 and 5 only (hyp: the +0x1000 pictures are the Army regiments' and the probes' own variants; not traced).

## Family-range gates

Every gate is on the **object's family byte** (`DatId >> 24`, or the class's `+0x2c` family dword, or the vtable `+4` type code, which equals the same byte). "Exact" means the range is one byte wide. `read` = read in the decompile named; `note` = taken from the owner's note or a wiki map page and not re-read here.

### The five functions the question named

| Function | Address | Gates | Effect | Read? |
|---|---|---|---|---|
| Manufacturing window constructor | `FUN_00452fc0` | none on families; window type 9, 226 by 304, pages `0x67..0x6c` | the family selection is by page: `0x68` `0x28`, `0x69` `0x29`, `0x6a` `0x2a`, `0x6b` `0x2d`, `0x6c` `0x2c`; counts by `FUN_0052c8c0` `[0x28,0x29)`, `FUN_0052c5a0` `[0x29,0x2a)`, `FUN_0052c270` `[0x2a,0x2b)`, `FUN_0052d270` `[0x2d,0x2e)`, `FUN_0052d690` `[0x2c,0x2d)` | read (`FUN_00452fc0` has no gate; counters read) |
| Build-selection candidates | `FUN_00537ff0` then `FUN_0052e580` then `FUN_00567880` | producer family: type 0 to `0x28`, type 1 to `0x2a`, type 2 to `0x29` (`FUN_00537ff0`); `FUN_0052e580` accepts exactly `0x28`, `0x29`, `0x2a` (research-level slots `+0x9c`, `+0xa0`, `+0xa4`); `FUN_00567880` accepts a family in `[0x28,0x2b]` and side `1..2`, then filters classes by the pair `class+0x24..+0x28` | the build list is DAT-driven inside those three producer families | read |
| Troop Finder | `FUN_0046ce40` (window), `FUN_0046ea10` (list), `FUN_0046d8d0` (row) | window: none. List: walk of systems `FUN_004f31b0` (`0x90..0x97`) and iterator `FUN_00504cc0` / `FUN_00504c40` on **`[0x10,0x14)`** (side filter by tab). Row: five literal DatIds per tab side | a system is listed if it holds any `0x10..0x13` regiment; its five columns count only the literal ids | read |
| "Ground combat" | `FUN_00560d50` | `0x30..0x3b` (characters), `0x34` exact (Emperor, `FUN_005617b0`, the seat-of-power handler), `0x14..0x1b` (capital ships), `0x08..0x0f` (fleets, side test) | resets `CommandKind` (`+0x96`) of characters moved onto or off ships; **no regiment gate** | read; naming `hyp:` (settled by the `CommandKind` notify `FUN_004f1f80` and `map-10`) |
| Encyclopedia collection | `FUN_00422620` | `[0x14,0x20)`, `[0x28,0x30)`, `[0x22,0x28)`, `[0x10,0x14)`, `[0x30,0x40)`, `[0x08,0x10)`, and `[0x50,0x80)` when the mission's `+0x5c` is 0 | the object is listed if its family is in any of those (so fighters `0x1c..0x1f` are in with the ships); **`0x20` and `0x21` (the HQ), `0x40..0x4f` and `0x80+` are out**; one entry per key `id + 0x1000` | read |

The encyclopedia's category buttons (`FUN_0045f100`, command: range): `0x6f` all, `0x70` `[0x90,0x98)`, `0x71` `[0x14,0x20)`, `0x72` `[0x20,0x30)`, `0x73` `[0x40,0x80)`, `0x74` `[0x10,0x14)` (troops), `0x75` `[0x30,0x40)` (characters and special forces). Read.

### Blockade withdrawal

| Function | Address | Gate | Effect | Read? |
|---|---|---|---|---|
| System child hook | `FUN_00508660` (system vtable `+0xd0`) | by the child's type code: `0x20..0x2f` (`FUN_0050b760`), `0x20..0x21` HQ (`FUN_0055a0f0`, `FUN_0050c9f0`), **`0x22` exact** (`FUN_0050b310`), `0x2c..0x2f` (`FUN_0050c560`), **`0x28` exact** (manager 0, `FUN_0050bb70`, `FUN_0050c180`), **`0x29` exact** (manager 2, `FUN_0050c1b0`), **`0x2a` exact** (manager 1, `FUN_0050c1e0`), `0x08..0x0f`, `0x1c..0x1f`, **`0x10..0x13`** (`FUN_0050b610`, `FUN_0050ba90`, `FUN_0050b500`, and on add `FUN_0050c540`) | a regiment arriving gets the system's withdraw percent copied into its `+0x60`; a family-`0x22` facility recomputes the percent | read |
| Withdraw percent | `FUN_0050b310` | iterator `FUN_00527550` = family `[0x22,0x23)`; if the blockaded system has none active, `100 - ships*GNPRTB(7684) - fighters*GNPRTB(7685)` | the KDY-150 role is the family byte `0x22`, so any DEFFACSD record with `family_id` 0x22 acts as a KDY | read (`FUN_00527550`), rest `note` (`blockade-troop-withdrawal.md`) |
| Regiment roll | `FUN_00504990` and `FUN_00504960` (regiment vtable `0x0065e438`) | none (virtual dispatch from the class) | `FUN_0053e2f0(+0x60)`, event `0x340` | note |

### Other sweeps (every function that tests a range touching these families)

Found by scanning all 14,560 decompiles for `X < lo || hi < X` and `lo-1 < X && X < hi+1` tests on a family byte. Groups, with the function addresses:

| Range | What it means | Functions |
|---|---|---|
| `0x10..0x13` troops | Status filler, finders, sector window add/update, defenses window, agent survey, blockade, system invariant, AI profile | `FUN_00422620`, `FUN_00429440`, `FUN_0042c3b0`, `FUN_00437f80`, `FUN_00438980`, `FUN_00443130` (status filler `FUN_00445280`), `FUN_0045b3e0`, `FUN_0045b770`, `FUN_00496f50`, `FUN_00497940`, `FUN_004981c0`, `FUN_0049d3b0`, `FUN_004a2d60`, `FUN_004a2e70`, `FUN_004a73c0`, `FUN_004a7b40`, `FUN_004a7f50`, `FUN_004aa4d0`, `FUN_00502020` (garrison aggregator), `FUN_00508660`, `FUN_00508da0`, `FUN_0050e820` (invariant), `FUN_0053d430`, `FUN_0042dd00` (profile builder, on `class+0x2c`) |
| `0x20..0x21` HQ | HQ-only | `FUN_00419260`, `FUN_00443130`, `FUN_0045b3e0`, `FUN_004981c0`, `FUN_004f3330`, `FUN_00508660`, `FUN_005140c0`, `FUN_00514510`, `FUN_00561470` (seat of power) |
| `0x22..0x27` defense | not a producer; add/update windows | `FUN_00422620`, `FUN_00429440`, `FUN_0042c3b0`, `FUN_00443130`, `FUN_0045b3e0`, `FUN_0045b770`, `FUN_00497940`, `FUN_004981c0`, `FUN_00526fd0` (iterator `[0x22,0x28)`) |
| `0x22`, `0x23`, `0x24`, `0x25` single | Defenses-window pages | `FUN_004a7b40`, `FUN_004a7f50`, `FUN_004a90d0`, `FUN_004aa4d0`, `FUN_00508660` (`0x22`) |
| `0x28`, `0x29`, `0x2a` single, `0x2c`, `0x2d` | System window pages and managers | `FUN_00454160`, `FUN_00454b30`, `FUN_00458d20`, `FUN_00508660`, `FUN_00516360` (event `0x315`), `FUN_0052fff0` (mines `0x2c`, refineries `0x2d`), `FUN_0052eb60` (`0x28..0x2b`, manager by `vtable+0x204`) |
| `0x28..0x2b` / `0x2c..0x2f` / `0x28..0x2f` | picture bases, sector icon, AI | `FUN_00458fe0`, `FUN_00497940`, `FUN_004981c0`, `FUN_00545240`, `FUN_0045cdc0` (helper `FUN_0053b6e0` `[0x28,0x30)`), `FUN_0058ce70` |
| `0x3c..0x3f` special forces | status, finders, AI class bits, Bounty Hunters | `FUN_00401d20`, `FUN_0042c3b0`, `FUN_00437f80`, `FUN_00438980`, `FUN_00443130` (filler `FUN_00445780`), `FUN_00488de0`, `FUN_00495bf0`, `FUN_00497940`, `FUN_004d0610`, `FUN_004f3af0`, `FUN_005425e0`, `FUN_00582cd0`, `FUN_00582e90` |
| `0x10..0x3f` broad | maintenance cost row, galaxy overview, removal and change events, deploy event, production completion | `FUN_0044a620`, `FUN_0044a7a0`, `FUN_005140c0`, `FUN_00514510`, `FUN_00514890`, `FUN_005150c0`, `FUN_00528d30`, `FUN_00545240`, `FUN_00545540`, `FUN_00545670`, `FUN_0057f540` |

The sweep lists where a range test exists, not what each one does; the table of the previous section and the "Mod surface" section only claim the effect of the functions marked `read`. The other functions are in the list so that a mod does not miss one.

### Literal DatIds (the other kind of gate)

| Literal | Where | Meaning | Read? |
|---|---|---|---|
| `0x10000001..0x1000000a` | `FUN_0046d8d0` | the Troop Finder's ten column ids (five per side, in the order above) | read |
| `0x10000002`, `0x10000008` | `FUN_0042c3b0` | Army regiments get no moving overlay | read |
| `0x10000002`, `0x10000008` | `FUN_0051ab20` | low-support garrisons are seeded with these | note (`starting-forces-seeding.md`), read |
| `0x10000006` | `FUN_005091f0` | counts Stormtroopers for the uprising score | note, read |
| `0x10000004`, `0x10000005` | `FUN_004c7650` | the AI's garrison phase requests Wookiee and Mon Calamari regiments | read |
| `0x3c000001..8` | `FUN_00402e80`, `FUN_00472cc0` | the AI class bit of a special force: 1/5 bit `0x10000000`, 2/6 `0x20000000`, 3/7 `0x40000000`, 4/8 `0x80000000`; no bit for other ids | note, read |
| `0x3c000001..8` | `FUN_00465540` | the Personnel Finder's special-force row | note (role), hyp: not re-read |
| `0x3c000003`, `0x3c000005` | `FUN_0042c3b0` | probes: no moving overlay | read |
| `0x3c000009` | `FUN_00575ae0`, `FUN_00575cc0`, `FUN_00545e90`, `FUN_004955b0`, `FUN_00497940` | Bounty Hunters: the Bounty mission accepts only this id; destroyed with reason `0x14` | note |
| `0x25000004` | `FUN_004197b0` | the Death Star Shield tech (AI flag) | note |
| `0x2c000001`, `0x2d000002` | `FUN_00559850`, `FUN_00559a60`, `FUN_004e25c0`, `FUN_004d6e70`, `FUN_004db2f0` | the mine and refinery ids used by seeding and the AI | note, read (`FUN_00559850`) |

## Functions

| Function | Address | Role |
|---|---|---|
| `FUN_00452fc0` | `0x00452fc0` | Manufacturing window (type 9) constructor; no family gate |
| `FUN_004568a0` | `0x004568a0` | page selector `0x67..0x6c` |
| `FUN_00454160` | `0x00454160` | System-window object-added hook: page by family `0x28/29/2a/2c/2d`, tab label art |
| `FUN_00458fe0` | `0x00458fe0` | facility picture by record id (six bases), state offsets |
| `FUN_00539f50` | `0x00539f50` | returns `*(class+0x18)`, the record id |
| `FUN_005f5500` | `0x005f5500` | tree lookup by key at `node+0x18`; the class record is the node |
| `FUN_00567790` / `FUN_0058ec90` | `0x00567790` / `0x0058ec90` | class lookup: family byte selects the table by range, `id & 0xffffff` the record |
| `FUN_00567880` | `0x00567880` | candidate classes by producer family, side, cost, research order |
| `FUN_0052e580` | `0x0052e580` | per-producer-family list; levels `+0x9c/+0xa0/+0xa4` |
| `FUN_00537ff0` | `0x00537ff0` | producer type to family `0x28/0x2a/0x29` |
| `FUN_00437880` | `0x00437880` | Build Selection list; GOKRES `id` picture |
| `FUN_00457c90` | `0x00457c90` | overview band product picture |
| `FUN_0042c3b0` | `0x0042c3b0` | row picture factory: GOKRES `id + 0x4000`, overlays, literal exemptions |
| `FUN_00443130` | `0x00443130` | Status window: portrait and filler dispatch by family |
| `FUN_0046ce40` | `0x0046ce40` | Troop Finder window; side art; no family gate |
| `FUN_0046ea10` | `0x0046ea10` | Troop Finder list: systems with `[0x10,0x14)` regiments |
| `FUN_0046d8d0` | `0x0046d8d0` | Troop Finder row: five literal DatIds per side |
| `FUN_00504cc0` / `FUN_00504c40` | `0x00504cc0` / `0x00504c40` | iterators over `[0x10,0x14)` |
| `FUN_00422620` | `0x00422620` | Encyclopedia collection and its family ranges |
| `FUN_0045f100` / `FUN_0045fa60` | `0x0045f100` / `0x0045fa60` | category filter; topic text and bitmap keys |
| `FUN_0045ca80`, `FUN_0045cdc0`, `FUN_0045ce80`, `FUN_0045b770`, `FUN_0045b3e0` | `0x0045ca80` etc. | sector-window overlay art and family dispatch |
| `FUN_00560d50`, `FUN_004ee350`, `FUN_004f25c0`, `FUN_00514a60`, `FUN_00504f70`, `FUN_00505190`, `FUN_005617b0` | `0x00560d50` etc. | the character `CommandKind` hook, not regiment combat |
| `FUN_0051ebf0`, `FUN_00540540`, `FUN_00540500`, `FUN_0053f100`, `FUN_0053a1e0` | `0x0051ebf0` etc. | factory registry: exact family byte to object factory |
| `FUN_00508660`, `FUN_0050b310`, `FUN_00527550`, `FUN_00526fd0`, `FUN_0050c540` | `0x00508660` etc. | blockade withdrawal, producer-manager hooks |
| `FUN_0052c8c0`, `FUN_0052c5a0`, `FUN_0052c270`, `FUN_0052d270`, `FUN_0052d690`, `FUN_0052cab0`, `FUN_0053b6e0` | `0x0052c8c0` etc. | one-family iterators `0x28`, `0x29`, `0x2a`, `0x2d`, `0x2c`, `0x28`, `[0x28,0x30)` |
| `FUN_0042dd00`, `FUN_00402e80`, `FUN_00472cc0` | `0x0042dd00` etc. | AI class-profile builder; special-force class bits |
| `FUN_004714e0`, `FUN_0046f360` | `0x004714e0`, `0x0046f360` | Battle Summary roster tabs (`hyp:` window type 8) |

## Evidence

- **The text id is the art key.** `FUN_00584c10` reads four words and two u16s into `class+0x20..+0x32`; `FUN_00437880` line 99 calls `FUN_005fbd20(param_1[0x53], class+0x30 & 0xfff, 2)`; `FUN_00457c90` line 54 does the same on the producer's class; `FUN_0042c3b0` forms `(uVar3 & 0xfff) + 0x4000` (and `+ 0x1000`, `+ 0x5000`, `+ 0x6000`, `+ 0x7000` for overlay and mission variants); `FUN_00422620` stores key `(+0x30 & 0xfff) + 0x1000` and skips an existing key (`FUN_0060a860`). Resource existence was checked in the shipped DLLs (a small resource-directory reader: GOKRES 580 bitmaps, ENCYTEXT 348 RCDATA, STRATEGY 1042 bitmaps).
- **Record id selects the facility sprite.** `FUN_00458fe0` disassembly (`0x00459061..0x00459112`): `CALL 0x00539f50` then `DEC EAX; CMP EAX,5; JA`; jump table to the six constants; mines/refineries compare 1 and 2. `FUN_00539f50` returns `*(*(obj+0x2c)+0x18)`; `FUN_00567790` gets the class with `FUN_005f5500(id & 0xffffff)`, which compares `node+0x18`. For an `ord` past the table the base stays 0 and the picture is `0..3`: STRATEGY has no bitmap below 900 (checked), so the cell is blank.
- **Factory registry.** `FUN_0051ebf0` makes 58 calls to `FUN_00540540`, capacity `0x3d` (`FUN_00540540`: `DAT_006b90f0 + 1 < 0x3d`); lookup `FUN_00540500` compares the key for equality. `FUN_0053f100` and `FUN_0053a1e0` call it with `id >> 24`.
- **Class table range.** DAT headers: TROOPSD family `0x10` high `0x14`, SPECFCSD `0x3c`/`0x40`, DEFFACSD `0x22`/`0x28`, MANFACSD `0x28`/`0x2c`, PROFACSD `0x2c`/`0x30` (JSON `family_id`, `field4`); `FUN_0058ec90` tests `lo <= family < hi`.
- **Producer pair.** `FUN_00567880` compares `*pair <= param_3 < pair[1]` with `pair = (class+0x24, class+0x28)` from `FUN_00567a70`; the DAT values are TROOPSD and SPECFCSD `0x29, 0x2a`, DEFFACSD/MANFACSD/PROFACSD `0x2a, 0x2b`. Cost test `FUN_0053b860 != 0` (`class+0x48`), side test `+0x40`/`+0x44`, research order `+0x50` against `param_4` with the three flags `(1,1,0)` from `FUN_0052e580`.
- **No regiment gate in `FUN_00560d50`.** All 230 lines read: gates `0x30..0x3b`, `0x34`, `0x14..0x1b`, `0x08..0x0f`; its children iterator `FUN_004f25c0` builds a `[0x30,0x3c)` filter (`local_14 = 0x30, local_10 = 0x3c`); `FUN_004ee350` writes `+0x96` and fires vtable `+0x330`, the notify path of `FUN_004f1f80`; `map-10` names `+0x96` as the Character's `CommandKind`.
- **Troop Finder literals.** `FUN_0046d8d0` lines 74..111 (ids stored to the column array, five per side); the row loop (`for 5`) counts only ids equal to a column; rows draw at `x = 0xc2 + 0x1c * column`.

## Open questions

- **`hyp:` ground-combat art.** No per-type picture was found for a regiment-versus-regiment battle; the strategic game resolves it without a regiment screen. The resolution function itself (where attack `class+0x64`, defense `+0x68`, bombardment defense `+0x60` are used) is not in the five functions of this packet; start from the status filler `FUN_00445280` (getters `+0x1d8..+0x1e0`) and event `0x150` (`FUN_00512370`, `FUN_00496f50`).
- Whether the `GOKRES id + 0x1000` bitmaps (TROOPSD ord 2 and 8, SPECFCSD ord 3 and 5) are used for a variant in `FUN_0042c3b0`'s side path or are dead.
- What ENCYBMAP resource a record's topic bitmap uses (`FUN_0045f970`); the DLL holds 31 string tables, the bitmaps are in `EData/`.
- `FUN_00465540` (Personnel Finder special-force row) and `FUN_00497940`/`FUN_00545e90` were taken from the map pages, not re-read.
- What the missing text or bitmap does: the encyclopedia lists a record whose ENCYTEXT resource is absent (`FindResourceA` null path skips the text); the bitmap loader `FUN_0045f970` was not read.

## Mod surface

What a mod adding a regiment, special force or facility must do in REBEXE, by content type. "DAT" items need no executable change.

**New regiment (TROOPSD record 11 and up, for example clone troopers or battle droids).**

1. DAT: `family_id` must stay `0x10` (the object factory exists only for that byte, `FUN_0051ebf0`); `production_family 0x29`, `next_production_family 0x2a`; the side flag; a non-zero `refined_material_cost`; `research_order` at or below the side's training level; a `text_stra_dll_id` whose low 12 bits are unused by every other record.
2. Resources, no code: GOKRES bitmap `id` and `id + 0x4000` (and the 195 by 63 picture the build selection scales), TEXTSTRA name, ENCYTEXT RCDATA `id + 0x1000`, an ENCYBMAP/EDATA bitmap. Without them the build list shows a blank picture and the encyclopedia an empty topic.
3. Executable patch only if the type must be counted in the Troop Finder: its row has five fixed columns and **ten literal ids** (`FUN_0046d8d0`, array of 5 at `local_bc`, loop `iVar6 < 5`, stride `0x1c`, start `0xc2`). A sixth regiment type is invisible in the finder (the system is still listed). Replacing a column id means changing one literal in each side's branch; adding a column means widening the row (the strip is 330 pixels with 28-pixel columns from x 194).
4. Behaviour literals to review: `0x10000002`/`0x10000008` (Army: seeding `FUN_0051ab20`, overlay `FUN_0042c3b0`), `0x10000006` (uprising Stormtrooper count, `FUN_005091f0`), `0x10000004`/`0x10000005` (AI garrison requests, `FUN_004c7650`). The rest of the AI reads classes through the family range (`FUN_0042dd00`) and picks them up.

**New special force (SPECFCSD record 10 and up).** DAT as above with family `0x3c`; refined cost non-zero or it is never offered. Executable: the AI class bit is set for ids 1..8 only (`FUN_00402e80`); the Personnel Finder row has fixed ids 1..8 (`FUN_00465540`, hyp); Bounty Hunters is a literal (`0x3c000009`: Bounty mission validation `FUN_00575ae0`, `FUN_00575cc0`, destroy rule `FUN_00545e90`); ids 3 and 5 are exempt from the moving overlay (`FUN_0042c3b0`). A mod that adds a tenth id must extend each literal list it wants it in. Its `mission_id` (SPECFCSD) is a bitmask, hyp: bit meaning not traced here.

**New facility (DEFFACSD, MANFACSD, PROFACSD record N+1).**

- *Yards and training or construction facilities.* `family_id` must be exactly `0x28`, `0x29` or `0x2a`: the iterators, the System window pages, the production managers and the factory are one family byte each (`FUN_0052c8c0`, `FUN_0052c5a0`, `FUN_0052c270`, `FUN_00508660`). A new producer family such as `0x2b` has no factory, no page and no manager. The record id selects its 69 by 40 sprite; **ids 7 and up have none**, so extending `FUN_00458fe0` means a switch case plus four consecutive STRATEGY bitmaps (built, building, building on side 1's systems, en route: base, +1, +2, +3, as the six shipped bases do). The `processing_rate` DAT field (`class+0x5c`) sets production speed, so a droid foundry that builds faster is a DAT edit.
- *Mines and refineries.* Families `0x2c`/`0x2d` exact (pages `0x6c`/`0x6b`); picture by id 1 and 2 only; seeding and the AI name `0x2c000001` and `0x2d000002` by literal.
- *Defense.* Family `0x22` (KDY role), `0x23`, `0x24`, `0x25` select the Defenses-window page and behaviour (`FUN_004aa4d0`), and any record with family `0x22` counts as a KDY in the blockade rule (`FUN_00527550`). Families `0x26` and `0x27` pass the range gates but have no factory.
- Encyclopedia: automatic for any record in a gated range if the text and bitmap exist.

**Generic limits (for the page that sums them up).** DAT: costs, research order, side flags, production pair, record count (header). Table in the executable: the factory registry `DAT_006b90f8` (58 of `0x3d` slots used), the six sprite bases of `FUN_00458fe0` (jump table at `0x00459190`), the finder's five columns. Literal or family check in code: everything in the gate and literal tables above. Quantity input is capped at `0x19` characters and clamped to 1..255 (`FUN_00438c30`, note); displayed costs and times cap at 9999 (`FUN_00438dd0`, `FUN_00438f30`, note). (The `+0x96` short that `modders-taxonomy.md` calls regiment strength is the character's `CommandKind`; the regiment strength field was not traced here.)

**Proposed follow-up questions** (each settled by the executable):

1. Where is regiment-against-regiment ground combat resolved, and which class fields (`+0x60`, `+0x64`, `+0x68`) does it read? Start from `FUN_00496f50`, `FUN_00512370`, `FUN_00445280`.
2. Does `FUN_0046d8d0`'s column bitmap (28 pixels wide, five columns) draw per-type icons that sit in the STRATEGY finder backgrounds (10335, 10336, 10584, 10588)? Settle by reading the background bitmaps; then a mod can recolour a column without code.
3. What does each of the five `FUN_0042c3b0` overlay ids (`0x2cf1`, `0x2cf5`, `0x2cfb`, `0x2ced`, `0x2d32/34`) show, and is `+0x50` bit `0x200` the "on mission" bit? Start from `FUN_0042c3b0` and `object-state-flags.md`.
4. How are DAT class tables registered (the list `thunk_FUN_005f5060` read by `FUN_0058ec90`): is there a cap on tables or records per table? Start from `FUN_00584150`, `FUN_005841a0`.
5. Which regiment overrides (`FUN_00504960`, `FUN_00504990`) run for an object whose family byte is `0x11..0x13`, if a mod forced such a record? Probably none (no factory), which the registry reading predicts but does not prove.
6. What do the 31 ENCYBMAP string tables hold: the encyclopedia topic captions or the picture names? Start from `FUN_0045f970`.
