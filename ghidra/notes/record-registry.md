---
title: "Record Registry: DAT Loading, Factory Registries and Why Appended Records Fail"
description: "How REBEXE loads every entity DAT (one generic loop over the header count into heap trees), the record-factory and family-to-class registries that bound what can exist, the producer bands and research slots, the special-force classes chosen by literal id, and the order in which an appended record fails"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [dat, records, loader, registry, families, research, special-forces, mod-surface]
---

# Record registry

Answers task-graph node `or-q-mod-registry-zrfv` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-registry`; the record loop `FUN_0058e6b0`, both registries (`FUN_00584b60` with its 17 rows in `FUN_00569300`, `FUN_00540540` with its 58 rows in `FUN_0051ebf0`), the exact-pair match `FUN_00584ac0`, the special-force class bits `FUN_00402e80` and the research slots `FUN_0052e580` were reread here. The page's assets hold the family list, the record factory sizes and 99 hard-coded record ids (`assets/map/mod-registry/`). `hyp:` marks anything not traced through code. Galaxy limits are `galaxy-limits.md`; ship and fighter art is `ship-type-assets.md`.

## Summary

The loader has no per-category limit: one generic routine reads the header count and allocates one heap record per entry into a tree keyed by id. An appended record fails on something else: (1) a header count that was not raised, (2) a header range that is not one of the 17 registered pairs, (3) a family byte with no class among the 58 registered, (4) the producer bands and three research slots, (5) an encyclopedia key already taken, and (6) code that names ids by literal.

## The load chain

1. New-game stage 3 (`FUN_005136d0`; the load path `FUN_00513a50` too) → `FUN_00567430` creates the class database `DAT_006bb7a4` (`FUN_005673d0`, list root `FUN_0058ec40`) and calls `FUN_005674e0`. A failed load destroys it (`FUN_005674c0`).
2. `FUN_005674e0` calls `FUN_00567d90` for 17 files (RCDATA names `0x6a4`..`0x6b0`, `0x6b2`..`0x6b5`; `0x6b1` is absent), then six research-tier checks (`FUN_00567c90`, sides 1 and 2 × families `0x28`..`0x2a`). `BASICSD.DAT` is never read.
3. `FUN_00567d90` builds a table (`FUN_0058eb60`), loads it through `FUN_005952a0` (`GDATA\` + name), copies the record names (`FUN_005f2fc0`) and appends the table to the database (`FUN_005f5440`).
4. **The record loop** `FUN_0058e6b0` (slot 0 of vtable `0x0066a958`; save `FUN_0058e770`, text load `FUN_0058e870`, text save `FUN_0058e800`): it reads the header (`FUN_00583530`, then three u32 **count, family lo, family hi**; lo and hi go to the table `+8` / `+0xc`), then loops `count` times: `FUN_00584ac0(lo, hi)` returns a new empty record from the record-factory registry, the record reads itself (vtable `+8`, for example `FUN_005901f0` for a system), and `FUN_0058ead0` → `FUN_005f5440` → `FUN_005f4f10` inserts it by its id at `+0x18`. Records are heap allocations, so a table has no capacity. There is no end-of-file test. A reader returning 0 ends the loop early and the table keeps what it read; `FUN_005952a0` always returns 1, so a short file does not stop the start.
5. A record's DatId is `(record +0x2c) << 24 | (record +0x18 & 0xffffff)` (`FUN_00402e40`): the family byte is the record's own word, not the table's. Lookup (`FUN_0051cab0` → `FUN_00567790`) takes the table whose header range contains the family (`FUN_0058ec90`) and then the id in its tree (`FUN_005f5500`). Nothing checks that a record's family lies in its own table's range.
6. Duplicate ids: the tree inserts an equal key to the right (`FUN_005f4f10`, comparator `FUN_005f5560`) and lookup stops at the first equal id, so the first record answers DatId lookups while in-order walks (encyclopedia, build lists) list both.

## Fixed structures

| Structure | Where | Capacity | Used | Kind |
|---|---|---|---|---|
| Record-factory registry `(lo, hi, ctor)` | `FUN_00584b60` (count `DAT_006bbb48`, entries `0x006bbb50`, 12 bytes) | `count + 1 < 0x3d` (60) | 17, one per shipped header pair (`FUN_00569300`) | table (code) |
| Exact pair match | `FUN_00584ac0` | | the 17 pairs; no match returns null and the loop reads nothing for that record (hyp: the stream desynchronises) | table (code) |
| Family-to-class registry `(family, ctor)` | `FUN_00540540` (count `DAT_006b90f0`, entries `0x006b90f8`, 8 bytes), read by `FUN_00540500` | 60 | 58 (`FUN_0051ebf0`) | table (code) |
| Object creation gate | `FUN_0053f100` (called by `FUN_004f7d50`) | | family non-zero, class record found, constructor registered | literal / table |
| Param-table type registry (IntTable, SideParam, SeedFamily, Seed, GeneralParam) | `FUN_0058f870` | `count + 1 < 10` | 5 (`FUN_00569280`) | table (code) |
| Side research slots | side object `+0x9c` (producer `0x28`), `+0xa0` (`0x29`), `+0xa4` (`0x2a`); `FUN_0052e580` returns 0 for any other family | 3 | 3 | object layout |
| Side object to side | `FUN_00530a20`: id `0xf` → 1, `0x10` → 2, else 0 | 2 | 2 | literal |

Every family a shipped record carries is registered except FLEETSD record 5, whose family word is `0x00`, so that record cannot become an object.

## Special-force classes by literal id

`FUN_00402e80` sets a class bit in the AI record's capability word `+0x34` from the id: `0x3c000001` / `0x3c000005` → `0x10000000`, `2` / `6` → `0x20000000`, `3` / `7` → `0x40000000`, `4` / `8` → `0x80000000`. `FUN_00418500` case 5 zeroes the census block `ctx +0x2c4` (0x14 dwords) and `FUN_0041bc10` recounts it per side: `+0x2dc`..`+0x2e8` count special forces per class bit, `+0x2ec`..`+0x2f8` those that also carry capability bit `0x800000`. `FUN_004d0480` round-robins the four class codes `0x5a`..`0x5d` against `max(ctx +0x2fc, 8)` and sets the request word `0x1000000 << k` (`ai-order-emission.md`, task class 5). Ids 1..4 are Alliance and 5..9 Empire, so each class pairs one force from each side; special force 9 (`0x3c000009`) and any appended special force carry no class bit and are never counted (hyp: the class round robin therefore never trains them).

## Why an appended record fails, in order

1. **Header count.** Bytes after the last counted record are never read; raise header word 2.
2. **Header range.** Words 3 and 4 must stay one of the 17 registered pairs, and the record's family must lie inside them.
3. **Family byte.** It must be one of the 58: ships `0x14` or `0x18`, fighters `0x1c`, troops `0x10`, special forces `0x3c`, minor characters `0x38`, systems `0x90` / `0x92`, sectors `0x80`, defence `0x22`..`0x25`, manufacturing `0x28`..`0x2a`, production `0x2c` / `0x2d`, the six major families `0x30`..`0x35`, and the 22 mission families. A seventh major (`0x36`, `0x37`), a new mission kind (`0x45`..`0x4f`, `0x59`..`0x60`, `0x66`..`0x68`) or a new facility kind (`0x26`, `0x27`, `0x2b`, `0x2e`, `0x2f`) cannot be created. A second record in an existing family needs no code.
4. **Build lists and research.** `FUN_00567880` lists a class for a side only when its production range `[+0x24, +0x28)` holds the producer family `0x28`..`0x2a`, its side flag (`+0x40` Alliance, `+0x44` Empire) is set and its refined cost `+0x48` is non-zero; it is buildable when `research_order` is at or below the side's level. In the shipped data every `research_order` of 1 or more is unique per side and producer family, and the research code (`FUN_00567a90`, `FUN_00567ba0`) takes the first record at a tier, so a second record at a tier above 0 is buildable but is never the one researched or priced. A cost of 0 hides the record. hyp: the six load-time tier checks only gate each other and cannot fail the load.
5. **Encyclopedia.** `FUN_00422620` lists a class only in the family bands `[0x14,0x20)`, `[0x28,0x30)`, `[0x22,0x28)`, `[0x10,0x14)`, `[0x30,0x40)`, `[8,0x10)`, `[0x50,0x80)` with the hidden flag `+0x5c` clear, keys it `(record +0x30 & 0xfff) + 0x1000`, and skips a key already present (`FUN_0060a860`). A new record needs a text word with new low 12 bits and ENCYTEXT / ENCYBMAP entries for it.
6. **Art and strings.** The name is the TEXTSTRA id; GOKRES, ENCYBMAP and EDATA use the same low 12 bits; tactical meshes use the ordinal registry (`ship-type-assets.md`).
7. **Ids named in code.** 77 literals in code are DAT ids ([tables/hardcodedids.tsv](tables/hardcodedids.tsv), which also lists 22 literals no shipped DAT holds): every TROOPSD and SPECFCSD id, the six majors, eight minors (`0x38000341`, `343`, `344`, `380`..`383`, `38c`), one mission id for 21 of the 22 mission families, the five ABODESD ids and the UNIQUESD ids `0xf2000002`, `0xf3000003`, `0xf8000008`, `0xf9000009`, `0xfa00000a` that new-game stage 9 (`FUN_00519910`) creates one by one (the stage fails when any is missing). The AI names one ship class per side (`FUN_004198a0`: Alliance `0x1400004a`, Empire `0x14000082`). An appended record is inert on those paths.

## Per file

| File | Header (count, lo, hi), record bytes | Record class | Binding limit |
|---|---|---|---|
| CAPSHPSD | 30, `0x14`-`0x1c`, 200 | `FUN_00591d30`, 0xf0 | families `0x14`, `0x18`; producer `0x28`; tactical ordinals (`ship-type-assets.md`) |
| FIGHTSD | 8, `0x1c`-`0x20`, 168 | `FUN_005917b0`, 0xd0 | family `0x1c`; four colour groups per side (`ship-type-assets.md`) |
| TROOPSD | 10, `0x10`-`0x14`, 68 | `FUN_0058ff10`, 0x6c | family `0x10`; producer `0x29`; all ten ids named in code |
| SPECFCSD | 9, `0x3c`-`0x40`, 116 | `FUN_005904c0`, 0x9c | family `0x3c`; AI classes by literal id |
| MJCHARSD | 6, `0x30`-`0x38`, 148 | `FUN_005915f0`, 0xbc | one family per major; `0x36`, `0x37` unregistered (`character-roster.md`) |
| MNCHARSD | 54, `0x38`-`0x3c`, 148 | `FUN_005912b0`, 0xbc | family `0x38`; placed only by seed tables or recruitment |
| MISSNSD | 25, `0x40`-`0x80`, 112 | `FUN_00590b40`, 0x98 | family = mission kind, 22 registered; a second record of a kind is allowed |
| DEFFACSD | 6, `0x22`-`0x28`, 60 | `FUN_00591aa0`, 0x64 | families `0x22`..`0x25`; build-queue rows by exact family (`FUN_00454160`) |
| MANFACSD | 6, `0x28`-`0x2c`, 56 | `FUN_005913f0`, 0x60 | three research slots; queue tabs `0x68`..`0x6a` |
| PROFACSD | 2, `0x2c`-`0x30`, 56 | `FUN_00590980`, 0x60 | families `0x2c`, `0x2d` |
| ALLFACSD | 1, `0x20`-`0x22`, 52 | `FUN_00592260`, 0x5c | family `0x20` |
| SYSTEMSD | 200, `0x90`-`0x98`, 44 | `FUN_00590180`, 0x54 | `galaxy-limits.md` |
| SECTORSD | 20, `0x80`-`0x90`, 36 | `FUN_005906d0`, 0x4c | created only when `+0x44` is at or below the galaxy size (`FUN_00519c20`) |
| FLEETSD, UNIQUESD, ABODESD, MANMGRSD | 2, 6, 5, 3 | `FUN_0058fdb0`, 0x40 | stage 9 names the UNIQUESD and ABODESD records |
| BASICSD | 16, `0x98`-`0xfb`, 24 | not loaded | |

## Mod surface

1. **More records in an existing family (DAT only):** append the record, raise header word 2, keep its family inside the range and among the 58, use a new id, a text word with new low 12 bits, a non-zero refined cost, the side flag, and a `research_order` of 0 or a new tier.
2. **A new family:** a row in `FUN_0051ebf0` (two free slots) and a class, plus every band test that must see it (`FUN_00422620`, `FUN_0050d5a0`, `FUN_0054f5b0`); a producer family also needs a research slot, a build-queue tab and a sprite (`FUN_0052e580`, `FUN_00454160`, `FUN_00454b30`).
3. **A new file or header range:** a row in `FUN_00569300` (43 free), a record class, and a loader call plus RCDATA name in `FUN_005674e0`.
4. **A special force the AI uses, or a fifth class:** `FUN_00402e80`, the counters at `ctx +0x2dc`..`+0x2f8`, and the switch in `FUN_004d0480`.
5. **More than three producer families:** the side object's slots and `FUN_0052e580`, `FUN_00567880`, `FUN_00567c90`.

The port's mod loader rejects records with new ids (`crates/rebellion-data/src/mods.rs`, `agent_docs/mod-surface.md`); allowing them is an architecture change.


## Tables

- [tables/recordfactories.tsv](tables/recordfactories.tsv): each entity DAT's header count and range, file and memory record sizes, record factory and RCDATA id.
- [tables/familyregistry.tsv](tables/familyregistry.tsv): the 58 family-to-class registrations (`FUN_0051ebf0`) with their constructors and the DAT that ships each family.
- [tables/hardcodedids.tsv](tables/hardcodedids.tsv): every DatId literal in code, its family, whether a shipped DAT holds it, and the functions that name it.

## Functions

| Function | Role |
|---|---|
| [FUN_00567430](FUN_00567430.c), [FUN_005674e0](FUN_005674e0.c), [FUN_00567d90](FUN_00567d90.c) | Database creation, 17-file load, one table |
| [FUN_0058eb60](FUN_0058eb60.c), [FUN_005952a0](FUN_005952a0.c) | Table object; file open and load |
| [FUN_0058e6b0](FUN_0058e6b0.c), [FUN_0058e770](FUN_0058e770.c) | Record loop (load, save) |
| [FUN_00584ac0](FUN_00584ac0.c), [FUN_00584b60](FUN_00584b60.c), [FUN_00569300](FUN_00569300.c) | Record-factory lookup, registration, the 17 rows |
| [FUN_005901f0](FUN_005901f0.c), [FUN_00584c10](FUN_00584c10.c) | A record reader (system) and the common prefix |
| [FUN_005f5440](FUN_005f5440.c), [FUN_005f4f10](FUN_005f4f10.c), [FUN_005f5500](FUN_005f5500.c) | Tree insert and lookup |
| [FUN_00402e40](FUN_00402e40.c), [FUN_0051cab0](FUN_0051cab0.c), [FUN_00567790](FUN_00567790.c), [FUN_0058ec90](FUN_0058ec90.c) | DatId of a record; lookup by DatId |
| [FUN_0053f100](FUN_0053f100.c), [FUN_00540500](FUN_00540500.c), [FUN_00540540](FUN_00540540.c), [FUN_0051ebf0](FUN_0051ebf0.c) | Object creation and the family registry |
| [FUN_00567880](FUN_00567880.c), [FUN_00567a90](FUN_00567a90.c), [FUN_00567ba0](FUN_00567ba0.c), [FUN_00567c90](FUN_00567c90.c), [FUN_0052e580](FUN_0052e580.c), [FUN_00530a20](FUN_00530a20.c) | Build lists, research tiers, side slots |
| [FUN_00454160](FUN_00454160.c), [FUN_00454b30](FUN_00454b30.c) | Build-queue rows by producer family |
| [FUN_00422620](FUN_00422620.c), [FUN_0051caf0](FUN_0051caf0.c) | Encyclopedia collection |
| [FUN_00402e80](FUN_00402e80.c), [FUN_00418500](FUN_00418500.c), [FUN_0041bc10](FUN_0041bc10.c), [FUN_004d0480](FUN_004d0480.c) | Special-force classes and the AI census |
| [FUN_00519910](FUN_00519910.c), [FUN_00519c20](FUN_00519c20.c), [FUN_004198a0](FUN_004198a0.c) | Stage 9 named records, sectors by size, AI-named ship classes |

## Open questions

- What capability bit `0x800000` means and what the request word `0x1000000 << k` is matched against (`FUN_004d01a0`, `FUN_004d15f0`, `FUN_00402720`).
- Whether a header count larger than the data reads zeros or fails (`FUN_005f4d90`).
- Whether a second record at one research tier shows on the research screen (`FUN_0051cb90` callers).
- Where appended special forces and minor characters get placed in play (`FUN_0051a280`, `FUN_0051bc70`).
- Whether a saved class id above `0xff` survives (`FUN_0051cec0`, `FUN_005688d0`, `FUN_00568560`).
