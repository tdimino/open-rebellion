---
title: "Ship Type Assets: Meshes, Portraits, Icons, Encyclopedia Art and Voice"
description: "How a capital ship or fighter type gets its tactical mesh, HUD portrait, fighter art, strategic icon, encyclopedia bitmap and voice: the 38-entry ordinal registry in REBEXE versus DAT arithmetic on text_stra_dll_id, the family bands, and what a mod adding a ship type changes"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [ships, fighters, meshes, icons, encyclopedia, voice, tactical, mod-surface]
---

# Ship type assets

Answers task-graph node `or-q-mod-ships-dc1b` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-ships`; the ordinal registry and its lookup were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code.

## Answer

**Families.**

1. Ships are families `0x14`, `0x18` and `0x1c`; `0x30`-`0x3b` is personnel. The DAT headers give the families: `CAPSHPSD.DAT` header `(1, 30 records, 0x14, 0x1c)`, `FIGHTSD.DAT` `(1, 8, 0x1c, 0x20)`, `MJCHARSD` `0x30-0x38`, `MNCHARSD` `0x38-0x3c`, `MISSNSD` `0x40-0x80` (read from the files). Capital ships are family `0x14` (29 records, ids 64-78 Alliance, 128-135 and 137-142 Empire) plus family `0x18` (the single Death Star, id 136); fighters are `0x1c` (ids 1-8). The Encyclopedia category table in `FUN_00422620` agrees: ship category `[0x14,0x20)`, with `[0x30,0x40)` the Personnel category.
2. `0x71`-`0x74` are mission ids (MISSNSD records 0x43-0x46 and 0x82 carry `+0x14` words 0x71-0x73 in the file, and `annotated-functions.md` names `0x71` Dagobah, `0x72` Vacation and Sabbatical, `0x73` Pickup). `FUN_00544130` and `FUN_005445d0` test the family of the *current mission or task id* of the named character objects Luke/Leia/Vader/Chewbacca (`FUN_00507010`, `FUN_00506fb0`, `FUN_00507070`, `FUN_005066a0` build the ids `0x32000242`, `0x31000241`, `0x35000281`, `0x38000343`), so they are character rules. They never read a ship type. A new ship record does not touch vtable `+0x1d8` or `FUN_00534640` (read: `FUN_005445d0` lines calling `FUN_005434b0..FUN_00543760`, `FUN_005439b0`; `FUN_00534640` only sets a phase bit `8` on `this+0x78`).

**Per-type tactical assets come from a table indexed by an *ordinal* and from per-asset literals.** The pivot is a 38-entry registry built by `FUN_00597610` (called once through `FUN_00597530`, singleton `DAT_006bc248`, 0x6b8 bytes): ordinal = position, key = `DatId & 0xfff`. Every tactical lookup goes `DatId -> FUN_00599c40 -> ordinal -> table or switch`. Nothing computes a mesh from the family byte; the strategic icon and encyclopedia art compute from the DAT's own resource word.

| # | Asset or rule | Source | Function and address | Side branch |
|---|---|---|---|---|
| 1 | Class key | DAT field: record word 5 of CAPSHPSD/FIGHTSD (`text_stra_dll_id`, e.g. `0x22740`), low 12 bits `0x740` | runtime object `+0x30`; `FUN_004f63d0` returns `+0x30 & 0xfff` (vtable `+0x30`) | none: the Alliance and Empire records differ only by value (`0x740..0x74e`, `0x780..0x78e`, fighters `0x640..0x643`, `0x680..0x683`) |
| 2 | DatId to tactical ordinal | **table in the executable**: 256 slots at registry `+0x268` (`param_1[0x9a..0xbf]`), 38 filled by literal stores in `FUN_00597610`; linear search compares `DatId & 0xfff` | `FUN_00599c40` (0x00599c40): **returns 0 when not found**, not an error | none |
| 3 | Capital-ship mesh family (type 301, LOD ids base, +1, +2) | **table in the executable**: 29 immediate stores at tactical-manager `+0xa50..+0xac0`, indexed by ordinal (`2010 + 10*n` for ordinals 0-14, `2510 + 10*(n-15)` for 15-28) | `FUN_005ab650` (0x005ab650) writes it; read as `*(short*)(mgr + ordinal*4 + 0xa50)` in `FUN_005ab650` and `FUN_005ab0e0`; load `FUN_005c1100` -> `FUN_005c1f10` -> `FUN_005d26c0` (ids `base`, `base+1`, `base+2` = close, medium, far) | Alliance ordinals 0-14, Empire 15-28 by registry order; the mesh itself has no side switch. `FUN_005d26c0` picks the texture flag `0x80 or 0x40` from the passed side (`param_1[0xf] != 1`) |
| 4 | Capital-ship textures | type 303 resources paired with the mesh by the loader (`SDESTI52.BMP` for 2560 per the port's lookup) | `FUN_005d26c0` (hyp: texture name is inside the type-301 `.x` file; not traced here) | none |
| 5 | Fighter close sprite (type 303) | **literal switch** on ordinal type id `0x1d..0x24`: Alliance `4000/4004, 4010/4014, 4020/4024, 4030/4034`, Empire `4100/4104 .. 4130/4134` | `FUN_005ab650` stores to `+0xac4..+0xae0` (0x005ab7e6 onward); `FUN_005ab650` second switch and `FUN_005ab0e0` repeat the same literals (`case 0x1d: 4000`, `0x1e: 0xfaa`, ...). Default for an unknown type is `0x541` | which of the two columns a group uses depends on `piVar[0x5a] == mgr[0x9ec]` (player side or not), and on `FUN_00595c40()` (`mgr+0x9ec`, local side) for the 0x1d-0x20 versus 0x21-0x24 block |
| 6 | Fighter far sprite and indicator | arithmetic: close id +5; `4200 + id % 10` | `FUN_005d43c0` | as 5 |
| 7 | Selected-capital-ship HUD portrait | arithmetic on the ordinal: `ordinal + 0x7d1` (2001..2029) | `FUN_005e4870` (0x005e4870), consumed by `FUN_005e4b40` and `FUN_005e7b10` | none |
| 8 | Selected-fighter panel art | literal chain `0x1d..0x24 -> 0x7ee..0x7f5` (2030-2037) | `FUN_005e16a0` (0x005e16a0, `FUN_00595c40()` selects the Alliance or Empire block of four) | Alliance 0x1d-0x20 / Empire 0x21-0x24 |
| 9 | Death Star mesh | **literal**: `0x1392` (5010), +10 when manager object `+0x74` is set; ordinal not used | `FUN_005ab650` (`param_1+0x8d4`, `+0xae4`) | Empire-only record (id 136) |
| 10 | Strategic fleet, list and battle-alert icon | DAT field + arithmetic: `class_key = text_stra_dll_id & 0xfff`; GOKRES bitmap `class_key` or miniature `class_key + 0x4000` (`+0x5000` for a second side form) | `FUN_0042c3b0` (0x0042c3b0, general branch `param_4==0 ? key+0x4000 : key`); `FUN_0044aa20`, `FUN_004a1590` (`+0x4000`, `+0x5000` by side) | `FUN_004a1590` picks `0x4000` vs `0x5000` by the viewer-side test `iVar8 == 1`; `FUN_0042c3b0` by `object+0x24 >> 6 & 3` for its missions/characters branches (hyp: the ship branch reads no side bit) |
| 11 | Encyclopedia entry list | family range test: ships `[0x14,0x20)`, produced from the live object list with filters `[0x14,0x20) [0x28,0x30) [0x22,0x28) [0x10,0x14) [0x30,0x40) [8,0x10) [0x50,0x80)` and `[0x90,0x98)` | `FUN_00422620` (0x00422620): inserts one entry keyed `(+0x30 & 0xfff) + 0x1000` for every object whose family passes | none: ranges include both sides' ships |
| 12 | Encyclopedia art (ENCYBMAP / EDATA) | DAT field + arithmetic: ENCYBMAP key `class_key + 0x1000`; for the non-mission, non-character branch the stored key is used directly | `FUN_00422620` stores key as the entry id; `FUN_0045fa60` (0x0045fa60) uses it unchanged for ships (`+0x1000` Alliance, `+0x2000` Empire only on its character/mission branch) | ship entries share the key across sides; the Empire shift applies to characters/missions only (read in `FUN_0045fa60`) |
| 13 | Encyclopedia category button for ships | literal ranges | `FUN_0045d400` / `FUN_0045ddc0` (`0x72`: `[0x20,0x30)` is facilities; `0x71`: `[0x14,0x20)` ships) per `encyclopedia-window.md` | Alliance and Empire use different bitmaps for the same command |
| 14 | Voice lines | **not per type**. Events (`0x20..0x13c`) map to WAVE ids in a 0x13d-entry table filled by literal stores; the module is chosen by faction | `FUN_005bae60` (table built in constructor, event `+0x1c`), `FUN_005ba980` (queue, `< 0x20` is a sound effect, otherwise voice), `FUN_005bad50` (resolve to `0x51c+`) | Alliance `VOICEFXA.DLL` 14001.., Empire `VOICEFXE.DLL` 15001.. (`docs/reference/asset-library/tactical-voice-map.json`); variants are by task force 1-8 and fighter colour red/blue/green/gold, never by ship class |
| 15 | Class-specific sound/effect scale | **literal ordinal checks** `0x0b, 0x0e, 0x0d, 0x16, 0x19` (CC9600, Dauntless, Hydra, Assault Transport, Destroyer II) | `FUN_005b1f60` (0x005b1f60) passes scale `0.75`/mode 2 to `FUN_005c12d0`, else `1.0`/mode 3 (hyp: an explosion or effect size; the callee `FUN_005c12d0` is not read) | none |
| 16 | Per-ordinal motion variant | arithmetic `(ordinal + object +0x3b) & 3` | `FUN_005a6b40`, `FUN_005a6d60` (hyp: one of four launch or arrival animations) | none |
| 17 | Fighter group slots | **fixed array size**: four colour groups per side (type ids `0x1d..0x20` or `0x21..0x24`, `- 0x1d < 4` and `- 0x21 < 4`) | `FUN_005ab650` (`iVar5 - 0x1dU < 4`, `iVar5 - 0x21U < 4`), `FUN_005ae460`, `FUN_005adfa0`, `FUN_005ab0e0` | Alliance block 0x1d-0x20, Empire block 0x21-0x24; the sixth ordinal after eight fighters is the Death Star |
| 18 | Family dispatch for a placed object | family range tests via vtable `+0x4` | `FUN_00508660` (0x00508660) ranges `[0x20,0x30) 0x20-0x21 0x22 0x2c-0x2f 0x28 0x29 0x2a [8,0x10) [0x1c,0x20) [0x10,0x14)`: it handles characters, troops and facilities. **Ships (family `0x14-0x1b`) are not among its branches** | none |
| 19 | Family band table | 9 ranges `[0x14,0x18) [0x18,0x1c) [0x1c,0x20) [0x20,0x30) [0x2c,0x2d) [0x30,0x40) [0x10,0x14) [8,0x10) [0x40,0x80)` | `FUN_0054f5b0` (0x0054f5b0, builds table at `DAT_006bb2f0`, 8 bytes per band) used by the order-validity code `FUN_00550200`, `FUN_005504d0`, `FUN_00550700` | `[0x14,0x18)` capital ships, `[0x18,0x1c)` Death Star-class, `[0x1c,0x20)` fighters; a new record inside these bands needs no change |
| 20 | Buildable ship list | family `0x28` manager: record flags `+0x40`/`+0x44` (the DAT `is_alliance`/`is_empire` words) and `+0x48` cost, family range `[0x28,0x2c)` | `FUN_0052e580` -> `FUN_00567880` (walks the class tables) | `param_2 == 1` tests `+0x40`, `== 2` tests `+0x44`; a new record needs only its DAT flags |

**Mesh and icon cost, in short.** Mesh, HUD portrait and fighter art are all keyed by *ordinal*, and ordinal comes from a table that exists only in the executable. The strategic icon and encyclopedia art are keyed by the DAT's own `text_stra_dll_id`, so a new record with a new low-12-bit key and matching GOKRES / ENCYBMAP resources needs no code change for those two.

## Functions

| Function | Address | Role |
|---|---|---|
| `FUN_00597610` | 0x00597610 | constructs the 38-entry ship registry: 38 names, 38 `DatId & 0xfff` keys at `+0x268` |
| `FUN_00597530` | 0x00597530 | creates the registry singleton `DAT_006bc248` (0x6b8 bytes) |
| `FUN_00599c40` | 0x00599c40 | key to ordinal over 0x100 slots, 0 on a miss |
| `FUN_005975d0` | 0x005975d0 | key to second short from a list (used for a side or type lookup, hyp) |
| `FUN_00595f60` | 0x00595f60 | builds a capital-ship battle object; calls `FUN_00599c40` then `FUN_005b05c0` |
| `FUN_005964c0` | 0x005964c0 | same for fighters (`FUN_005b9c60`) |
| `FUN_005ab650` | 0x005ab650 | battle setup: mesh bases `+0xa50..`, fighter bases `+0xac4..`, Death Star `0x1392` |
| `FUN_005ab0e0` | 0x005ab0e0 | rebuilds fighter and ship sprites from the same literals |
| `FUN_005c1100`, `FUN_005c1f10`, `FUN_005d26c0` | 0x005c1100 ... | load the type-301 mesh family `base..base+2` and bind textures |
| `FUN_005e4870` | 0x005e4870 | HUD portrait id `ordinal + 0x7d1` |
| `FUN_005e16a0` | 0x005e16a0 | fighter panel art 2030-2037 |
| `FUN_005b1f60` | 0x005b1f60 | class-specific effect scale on ordinals 11, 13, 14, 22, 25 |
| `FUN_0042c3b0` | 0x0042c3b0 | GOKRES class bitmap or miniature from `+0x30 & 0xfff` |
| `FUN_00422620` | 0x00422620 | Encyclopedia collection and family filters |
| `FUN_0045fa60` | 0x0045fa60 | Encyclopedia topic art and text |
| `FUN_00508660` | 0x00508660 | placement dispatcher by family (no ship branch) |
| `FUN_005445d0`, `FUN_00544130`, `FUN_00534640` | 0x005445d0 ... | story checks on the named characters (DarthPickup, LukePalace) and the resign request; `0x71` and `0x73` are the Dagobah and Pickup mission families |
| `FUN_0054f5b0` | 0x0054f5b0 | nine family bands |
| `FUN_005bae60`, `FUN_005ba980`, `FUN_005bad50` | 0x005bae60 ... | voice event table and queue |

## Evidence

- **The registry** ([tables/ordtable.tsv](tables/ordtable.tsv)). `FUN_00597610` stores 38 names (`CALAMARI` .. `DREADNAUGHT_A`, 14 more Alliance, `STRIKE_CRUISER` .. `DREADNAUGHT_E`, `XWING`.. `ASSAULT_GUNSHIP`, `DEATH_STAR`) and writes `param_1[0x9a..0xbf] = 0x740..0x74e, 0x780..0x78e, 0x642, 0x643, 0x640, 0x641, 0x680..0x683, 0x788`. The registry order is not DAT order: ordinals 11-14 are DAT ids 77, 75, 76, 78 and the Empire's DatId 136 (Death Star) is ordinal 37 (read). Join to the DATs: [ordtable.tsv] (38 rows: ordinal, name, DAT kind, record id, family, side, class key, GOKRES base `+0x4000`, ENCYBMAP key `+0x1000`, mesh or sprite base, HUD id).
- **Mesh bases.** `FUN_005ab650` stores `0x7da, 0x7e4 .. 0x866` to `+0xa50..+0xa88` (ordinals 0-14, 2010-2150) and `0x9ce .. 0xa46` to `+0xa8c..+0xabc` (ordinals 15-28, 2510-2630), then `0xa50` to `+0xac0` (2640, the 29th). The consumer `*(short *)(mgr + ordinal*4 + 0xa50)` is at `FUN_005ab650` and `FUN_005ab0e0`. `FUN_005ac6c0`/`FUN_005acaf0`/`FUN_005acf80` save and load `0x100` such slots, so the on-disk array is fixed at 256 entries even though 29 are used.
- **Fighter literals.** `FUN_005ab650` `case 0x1d: 4000; 0x1e: 0xfaa; 0x1f: 0xfb4; 0x20: 0xfbe; 0x21: 0x1004; 0x22: 0x100e; 0x23: 0x1018; 0x24: 0x1022` for the player side, and `0xfa4, 0xfae, 0xfb8, 0xfc2, 0x1008, 0x1012, 0x101c, 0x1026` otherwise; unknown type goes to `0x541`.
- **Voice.** `FUN_005bae60` sets 0x13d (317) event slots; the first 0x32c8 (13000) block is weapon and effect WAVEs, the rest voice ids. `FUN_005ba980` treats events `< 0x20` as sound effects (flag 6) and others as voices (flag 0xe). No ship type reaches this table.
- **DAT headers.** `CAPSHPSD`: 30 records of 200 bytes; word 4 = family (`0x14` x 29, `0x18` for id 136), word 5 = class word (`0x22740..0x2274e`, `0x22780..0x2278e`), words 6 and 7 = Alliance and Empire flags. `FIGHTSD`: 8 records of 168 bytes, family `0x1c`, class words `0x22640..0x22643`, `0x22680..0x22683`.
- **Death Star.** `0x1392` plus 10 when `+0x74` is set (`FUN_005ab650`): see `tactical-death-star-path.md`.
- **Not read.** The type-303 texture names, `FUN_005c12d0`, the later branches of `FUN_0042c3b0` that serve missions and other families, `FUN_005e7b10` (the panel `0x16` refresh), what `FUN_005975d0`'s returned short feeds, and the order `0x201` path for a new ship family beyond `FUN_00550200`.

## Mod surface

What a mod that adds a capital ship or fighter would have to change in REBEXE (all are executable edits, not DAT edits):

1. **Registry `FUN_00597610`**: add a name and a `DatId & 0xfff` key. Without it, `FUN_00599c40` returns ordinal 0 and the ship draws as the Mon Calamari Cruiser (hyp: an unfound type for a fighter hits the `0x541` default). The key table has 256 slots, 38 used, so the capacity is there but the 38 names live in `param_1 + 0x28` for 0x26 slots (`FUN_00618c80(param_1+0x28, 0xc, 0x26, ..)`): a 39th name overruns that array (read: element count `0x26`).
2. **Mesh table `FUN_005ab650` (and the identical literals in `FUN_005ab0e0`)**: a new capital ship needs a new immediate store at `+0xa50 + 4*ordinal`; the 256-entry array has room, but the code is immediates, so it needs a patch or a hook. Each ordinal also needs three consecutive type-301 resources `base..base+2` in TACTICAL.DLL.
3. **HUD portrait `FUN_005e4870`** (`ordinal + 0x7d1`): a new capital ship needs bitmap `2001 + ordinal` in TACTICAL.DLL; ordinals after 28 collide with the fighter art block 2030-2037 (read: ordinals 29-36 are the fighters).
4. **Fighters**: the eight fighters occupy type ids `0x1d..0x24` in four-colour blocks. A ninth fighter needs new switch cases in `FUN_005ab650` (two switches), `FUN_005ab0e0`, `FUN_005e16a0` (panel art 2030-2037, `0x1d..0x24` chain) and a new fixed-size group array (`- 0x1dU < 4`, `- 0x21U < 4` in `FUN_005ab650`, `FUN_005ae460`).
5. **Death Star-like special hulls**: the one record uses family `0x18` and `0x1392`; a second needs the literal and the separate battle object (`tactical-death-star-path.md`).
6. **Class effect scale** in `FUN_005b1f60` (ordinals `0x0b, 0x0e, 0x0d, 0x16, 0x19`): optional.
7. **Strategic icon and encyclopedia**: no executable change if the new record keeps its class word in the existing families (`0x14-0x1f`) and its GOKRES `key`, `key + 0x4000` and ENCYBMAP `key + 0x1000` resources exist (`FUN_0042c3b0`, `FUN_00422620`). A new family outside `[0x14,0x20)` would also change the category filters in `FUN_00422620`, `FUN_0045f100` and the buttons in `FUN_0045ddc0`, and the band table `FUN_0054f5b0`.
8. **Voice**: no change; the voice table is per event, not per class.
9. **Family rules**: none for ships in `FUN_00508660`, `FUN_005445d0`, `FUN_00544130`, `FUN_00534640` (hyp: the ship rules live in the `[0x14,0x1c)` band tests of `FUN_00560d50` and the fleet code, which are not re-read here; the scan [familybands.tsv] lists 47 functions that test `[0x14,0x1c)`, 13 that test `[0x18,0x1c)` and 26 that test `[0x1c,0x20)`).

Proposed follow-up questions:

- What exactly does `FUN_005c12d0` do with the ordinal checks in `FUN_005b1f60` (start: `FUN_005b1f60`, `FUN_005c12d0`)?
- Where does a type-301 `.x` file name its texture, and does the loader accept a new base without a table entry (start: `FUN_005d26c0`, `FUN_005c1f10`)?
- Which of the 47 `[0x14,0x1c)` testers would break if capital ships were given families beyond `0x1b`, and which need the Death Star band `0x18` (start: [tables/familybands.tsv](tables/familybands.tsv))?
- Does the registry names array (`param_1+0x28`, 0x26 slots) bound the 39th type, and what reads the names (start: `FUN_00597610`, `FUN_005975d0`, `FUN_005a02c0`)?

## Open questions

- The mesh table is written by immediate stores in a function that also runs the battle setup. Whether a loader patch can substitute a data table without moving code is untested.
- `hyp:` the ENCYBMAP key for the Empire's ship entries equals the Alliance rule; only the character and mission branch of `FUN_0045fa60` was read adding `0x2000`.
