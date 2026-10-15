---
title: "Galaxy Limits: What Fixes 200 Systems and 20 Sectors"
description: "The nearness table as the one literal 200, system and sector family windows, the galaxy-size filter at new game, the galaxy view frame, the save layout, and what raising the system count would touch"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [galaxy, systems, sectors, nearness, galaxy-size, save, mod-surface]
---

# Galaxy limits

Answers task-graph node `or-q-mod-galaxy-w665` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-galaxy`; the nearness table, the galaxy-size filter and the system creation gate were reread here. Basis: `read` = the decompile was read; `hyp:` = inferred, with what would settle it.

## Answer

Short version: **one hard capacity (the nearness table), one hard id window (system ids 100-299), and a hard-coded family range (0x90-0x97 systems, 0x80-0x8f sectors) repeated in about 300 functions.** Everything else reads the DAT or walks a list. No sector path read here holds a literal 20, and the only 200 in code is the nearness table.

1. **The only literal 200 in the executable's logic is the nearness table** (`FUN_005562e0` → `FUN_0058b060`). Allocation `0x27424` = 4 + 200 × `0x324`; row stride `0x324` = 200 pointers + 4 spare bytes; per-system row at `(id - 100) * 0x324`; a 400-dword stack buffer, a literal `200` sort count and copy count, and `-100` as the id base. None of it reads the DAT count. A grep of all 10,983 decompiles for `0x324`, `0x27424`, `0x9d08`, `0x13a0c` and `200`/`0xc8` as a count finds these four functions plus `FUN_0058afd0`/`FUN_0058af80` and nothing else ([Evidence](#evidence)).
2. **System ids are a window 100-299 by construction, not by a check.** The table's row and buffer index is `id - 100` with no bounds test; ids outside 100-299 write outside the stack buffer (`FUN_0058b060`) and read outside the table (`FUN_0058afd0`). The DAT ids are exactly 100..299.
3. **Families are literals.** Systems are DatId family `0x90..0x97` (half-open `[0x90, 0x98)`), sectors `0x80..0x8f`. The DAT headers carry the same ranges (SYSTEMSD header words `1, 200, 0x90, 0x98`; SECTORSD `1, 20, 0x80, 0x90`) but code never reads them for these checks: 271 functions have an inline `< 0x90 || 0x97 <` test and 211 set the pair `0x90`/`0x98` for a family iterator; 64 and 50 do the same for sectors. A fourth family `0x98..0x9f` is tested apart from systems (`FUN_00418db0` line 160, `FUN_0043dd30` line 69), so systems cannot grow past `0x97`.
4. **The galaxy-size filter is traced: `FUN_00519c20`** (step `0xa` of the new-game pipeline, `FUN_005136d0` case `0xa`). It walks the SECTORSD records (`FUN_0051cad0(0x80)`, next at `+0x10`) and creates a sector object (`FUN_004f7d50`) only when the record's `+0x44` is `<=` the chosen size. `+0x44` is the record's file offset `0x1c` (`galaxy_size`) plus the in-memory shift `0x28`. The size is the game root's `+0x5c`, validated `1..3` as a literal at `FUN_00409fe0`. Systems follow their sector: `FUN_0051bc70` creates a system only if its sector object exists (`FUN_00506a40` on key `0x80000000 | record +0x40`). So the 100/150/200 system counts come from the DAT's `galaxy_size` values, not from a literal. A mod can add sectors of size 1-3 freely; a sector with size 4 is never created, because the chosen size is validated to 1..3 (item 5 shows the same range in the budget).
5. **Galaxy size 1..3 is also a literal in the starting-force budget:** `FUN_0055d670` (Alliance, side 1) and `FUN_0055d6d0` (Empire, side 2) return 0 for `size < 1 || 3 < size`, else pick one of three SDPRTB values per side (`DAT_006bb664/6f4/69c`, `DAT_006bb600/610/654`).
6. **`FUN_00519d00` is step `0xb` of the new-game pipeline: it counts the systems whose sector exists and splits that count into side buckets by SDPRTB `0x1e00`/`0x1e01` percentages (`FUN_005597e0`), with Yavin (`0x92000121`) and Coruscant (`0x90000109`) handled first. Its counts come from list sizes (`FUN_004f4290`), so it follows the DAT and the filter; it holds no 200.
7. **Families `0xa0`-`0xa5` are per-system objects; sectors are `0x80`-`0x8f`.** Sectors are `0x80..0x8f` (`FUN_005069d0` iterates `[0x80, 0x90)`; SECTORSD `family_id` 128). `FUN_00537180` tests an object's family in `[0xa0, 0xa2)`, then a system in `[0x90, 0x98)`; the saves hold about one `0xa0`, one `0xa2` and one `0xa4` record per system (98 in a 99-system save, 199 in a 200-system one). hyp: per-system child objects (the side views `FUN_004f3220` resolves); settle by reading `FUN_004f3220` and the `0xa0` object's constructor.
8. **The galaxy view is a fixed 0-1023 frame.** `DAT_00658bd8`/`DAT_00658bda` are two u16 `0x3ff` in `.rdata`; the transform multiplies by `607.0`/`437.0` (f64 at `0x658be0`/`0x658be8`) over `1024`. DAT coordinates are packed shorts, so a system outside 0-1023 draws off the starfield.
9. **The save has no per-system array.** The save is an object graph; systems are records (families `0x90`/`0x92`). A 99-system save is 410,763 bytes, 200-system ones 823-855 KB. The nearness table is not in the save: its builder runs only in the new-game pipeline and the load path (`FUN_00513a50`) sets step `0x1b` and does not run it. hyp: after a load `manager +0x68` is null and its readers skip the table (`FUN_00554150` checks it); settle by loading a save and breaking on `FUN_00554150`.

## Limits

"Reads DAT?" says whether the limit follows the DAT record count or header. Addresses are the function entry unless a line is given.

| # | Limit | Where | Literal | Kind | Reads DAT? | Raising it touches |
|---|---|---|---|---|---|---|
| 1 | Nearness-table allocation | `FUN_005562e0` | `0x27424` | literal in code | no | the `FUN_00618b70(0x27424)` size, and `FUN_0058af80`'s zero-fill count `0x9d08` dwords (= `(0x27424-4)/4`) |
| 2 | Row stride | `FUN_0058b060` (writes), `FUN_0058afd0` (reads) | `0x324` (= 200 × 4 + 4), and `-0x13a0c` (= 4 − 100 × `0x324`) | literal in code | no | both functions; the 4 spare bytes per row are unused (read) |
| 3 | Row/column count | `FUN_0058b060` | sort `FUN_0061a0c0(buf, 200, 8, FUN_0058b140)`, copy loop `200` | literal in code | no | both loops. A smaller galaxy is safe: absent systems are null pairs and `FUN_0058b140` sorts nulls last, so every row is nearest-first then nulls (read) |
| 4 | Stack buffer | `FUN_0058b060` | `undefined4 local_640[400]` (200 pairs of pointer, distance) | literal frame size | no | the function's stack frame; a larger galaxy writes past it |
| 5 | Id base | `FUN_0058b060` (`obj +0x18 - 100`), `FUN_0058afd0` (`& 0xffffff`, `-0x13a0c`) | `100` | literal in code | no (the DAT ids are 100..299) | the `-100`s and the `0x13a0c`; no bounds check anywhere |
| 6 | Nearness-table consumers | `FUN_00554150` → `FUN_00554e80`, `FUN_00554f80`, `FUN_005550e0`, `FUN_005551e0`, `FUN_005552e0` | none of their own | reads the table | follows table | only `FUN_005545e0` calls them (retreat/withdraw system choice; callers `FUN_00516730`, `FUN_00515480`, `FUN_00543040`). A table row longer than 200 needs no consumer change; they stop at the first null |
| 7 | System family range | `FUN_00505ef0` / `FUN_0051a5d0` (`[0x90, 0x98)` list heads), 271 inline tests, 211 `0x90`/`0x98` iterator pairs | `0x90`, `0x97`, `0x98` | literal in code (and the DAT header repeats it) | no | every one of those sites; adding a family past `0x97` needs a code patch at each |
| 8 | Sector family range | `FUN_005069d0` (`[0x80, 0x90)`), 64 inline tests, 50 iterator pairs | `0x80`, `0x8f`, `0x90` | literal in code (and the DAT header) | no | each site |
| 9 | Table lookup by family range | `FUN_0051cad0` → `FUN_005677d0` → `FUN_0058ec90` | none; matches `[table +0x28, +0x2c)` | DAT header range | yes | nothing: a table is found by the range its header declares |
| 10 | DAT table load | `FUN_00567430` → `FUN_005674e0` (RCDATA names) | none | DAT | yes (the record loop `FUN_0058e6b0` runs the header count; `record-registry.md`) | nothing in code; file name and id are an RCDATA entry |
| 11 | Galaxy-size filter | `FUN_00519c20` (test `record +0x44 <= size`) | none for the count | DAT field `galaxy_size` | yes (per record) | nothing; add records with sizes 1-3 |
| 12 | Galaxy size range | `FUN_00409fe0`; `FUN_0055d670`, `FUN_0055d6d0` | `1`, `3` | literal in code | no | the three tests and the SDPRTB columns that `FUN_0055d670`/`FUN_0055d6d0` pick |
| 13 | Systems in sector | none found | — | DAT (`sector_id`, record `+0x40`) | yes | nothing; "10 per sector" is data only |
| 14 | New-game system step batch | `FUN_0051a280` (`0x27 < local_10`) | `0x28` = 40 | literal in code | no | hyp: a batch size per call, not a cap (the step does not advance until its done flag is set; `FUN_005136d0` case `0xc`); settle by running a 200-system start and counting calls |
| 15 | Galaxy view frame | `FUN_00425d00` (transform), `FUN_00429ce0` (half-width test) | `DAT_00658bd8` = `DAT_00658bda` = `0x3ff`, `607.0`, `437.0` | table in the executable (`.rdata`) | no | the `.rdata` words and the two f64s; coordinates are 16-bit |
| 16 | System icon | `FUN_00425d00` | rect `0x11` square | literal in code | no | overlap only |
| 17 | Window key | `FUN_00429ce0` (`(ushort)(id << 6) \| 1`), `FUN_0045aac0` (`(key & 0x3ff) << 6 \| type`) | `0x3ff`, `6` | literal in code | no | system ids that differ by a multiple of 1024 give the same window key; ids 100-1023 are safe |
| 18 | Save | `FUN_0051cec0` (root), `FUN_00513a50` (graph) | none | object graph | per object | no per-system array to resize; keep ids unique |
| 19 | Starting-force percentages | `FUN_005597e0`, SDPRTB `0x1e00`/`0x1e01` | none | DAT (SDPRTB), per side | yes | nothing |

### By side

- **Nearness table, galaxy-size filter, family ranges, window key:** no side branch; the same code serves both sides.
- **Galaxy view frame** (`FUN_00425d00`): branches on the view's side (`+0x9c == 1`): Alliance adds `(0x15, 0x19)` = (21, 25) to each scaled coordinate, Empire `(0x54, 0x1b)` = (84, 27). Same 0-1023 frame and 607 × 437 scale.
- **Sector window columns** (`FUN_00429ce0`): Alliance `x = 60`/`300`, `y = 35`; Empire `x = 120`/`365`, `y = 40`, by `sector-window-placement.md` (not re-read here); the half-width test is `DAT_00658bd8 / 2` for both.
- **Starting forces** (`FUN_0051b1f0`): side 1 `FUN_0055d670`, side 2 `FUN_0055d6d0`, each literal `1..3`.
- **System allocation** (`FUN_00519d00`): `FUN_005597e0(side, flag)` returns the SDPRTB value for side 1 or 2 and the remainder for side 3 (`DAT_00661a88` = 100 minus the four).
- **Retreat readers of the table** filter on the side bits: `((obj +0x24 ^ other +0x24) & 0xc0) == 0` (`FUN_00554e80`).

## Functions

| Function | Address | Role |
|---|---|---|
| `FUN_005562e0` | `0x005562e0` | Builds the nearness table: frees `+0x68`, allocs `0x27424`, constructs (`FUN_0058af80`), fills (`FUN_0058b060`); only caller `FUN_0051a530` |
| `FUN_0058b060` | `0x0058b060` | For each system object, sorts all systems by distance into row `(id-100)*0x324` |
| `FUN_0058af80` | `0x0058af80` | Table constructor: vtable `0x66a894`, zero-fills `0x9d08` dwords |
| `FUN_0058afd0` | `0x0058afd0` | Row lookup: family `0x90..0x97` test, `(id&0xffffff)*0x324 - 0x13a0c` |
| `FUN_0058b140` | `0x0058b140` | Sort comparator: nulls last, then ascending distance |
| `FUN_0055d860` | `0x0055d860` | Squared-distance isqrt on two packed short coordinates |
| `FUN_00554150` | `0x00554150` | `if manager +0x68: FUN_0058afd0` |
| `FUN_00554e80`, `FUN_00554f80`, `FUN_005550e0`, `FUN_005551e0`, `FUN_005552e0` | | The five readers (walk a row until the first null) |
| `FUN_005545e0` | `0x005545e0` | Chains the retreat candidate passes; callers `FUN_00516730`, `FUN_00515480`, `FUN_00543040` |
| `FUN_005136d0` | `0x005136d0` | New-game step dispatcher (`DAT_006b2bb4` = step 2..0x19); cases `0xa` = sectors, `0xb` = `FUN_00519d00`, `0xc` = `FUN_0051a280`, `0xd` = nearness table, `0x12` = starting forces |
| `FUN_00519c20` | `0x00519c20` | Galaxy-size filter: creates sector objects with `record +0x44 <= size` |
| `FUN_00519d00` | `0x00519d00` | System allocation into buckets by SDPRTB percentages (see Answer 6) |
| `FUN_0051a280` | `0x0051a280` | Creates systems batch by batch via `FUN_0051bc70` (40 per call) |
| `FUN_0051bc70` | `0x0051bc70` | Creates one system (`FUN_004f7d50`) only if its sector object exists |
| `FUN_00505ef0`, `FUN_0051a5d0` | | List head / next over objects of family `[0x90, 0x98)` |
| `FUN_005069d0` | `0x005069d0` | List head over objects of family `[0x80, 0x90)` |
| `FUN_0051cad0` → `FUN_005677d0` → `FUN_0058ec90` | | DAT record list by family range `[+0x28, +0x2c)` |
| `FUN_00567430` → `FUN_005674e0` | | Loads every DAT table at start (RCDATA names) |
| `FUN_00409fe0` | `0x00409fe0` | New game: rejects galaxy size outside `1..3` |
| `FUN_0055d670`, `FUN_0055d6d0` | | Starting-force budget by size and side |
| `FUN_005597e0` | `0x005597e0` | SDPRTB `0x1e00`/`0x1e01` by side and flag |
| `FUN_00425d00` | `0x00425d00` | Galaxy view layout: 0-1023 scaled to 607 × 437, offsets by side |
| `FUN_00429ce0`, `FUN_0045aac0` | | Sector-window key `(id << 6) | type` |
| `FUN_00537180` | `0x00537180` | Per-system object loop; families `[0xa0, 0xa2)` then `[0x90, 0x98)` |
| `FUN_00513a50`, `FUN_0051cec0` | | Game load / root save; step `0x1b` after load |

## Evidence

- Commands (all in `/workspace`, no build, no game run): `analyzeHeadless … -postScript DecompileTargets.java` on the functions above; `DecompileRange.java 0x00401000 0x00658000` (10,983 functions) as a grep corpus; `DecompileVtable.java 0x0066a894` and `0x0066a96c`; `CreateAndDecompileTargets.java 0058b140`; `RefsToRange.java` (a small script, not committed) over the RCDATA file-name block `0x6c2860-0x6c2c20`.
- Greps over the corpus: `0x324`, `0x27424`, `0x9d08`, `0x13a0c` → `FUN_005562e0`, `FUN_0058b060`, `FUN_0058af80`, `FUN_0058afd0` (plus unrelated vtable offsets in `FUN_004ee1d0`, `FUN_005bae60`); `200`/`0xc8` as a count → only `FUN_0058b060` among galaxy code (the rest are window sizes and object offsets); `-100`/`0x64` → `FUN_0058b060` (and `FUN_00596340`, unrelated).
- DAT facts: SYSTEMSD header `(1, 200, 144, 152)`, 200 records of 44 bytes, ids 100..299, `family_id` 144 (60 records) or 146 (140), `sector_id` 20..39, 10 per sector, x 118-791, y 103-839. SECTORSD header `(1, 20, 128, 144)`, 20 records of 36 bytes, ids 20..39, all family 128, `galaxy_size` 1 (10 sectors), 2 (5), 3 (5). In-memory record offset = file offset + `0x28`: SYSTEMSD `sector_id` (file `0x18`) is read at `+0x40` (`FUN_00519d00`, `FUN_0051bc70`), SECTORSD `galaxy_size` (file `0x1c`) at `+0x44` (`FUN_00519c20`), system coordinates (file `0x24`) at `+0x4c`.
- Saves (`tools/original-capture/saves`, records with a family word `0x90`/`0x92` after a key that is 0 or the record's own offset): `empire-day2-start.sav` 29 + 70 = 99 system records, 98 each of `0xa0`/`0xa2`/`0xa4`; `empire-sixteen-tie-day415.sav` (Huge) 62 + 140, 199 each. The count follows the galaxy, not a fixed layout.
- Not traced: whether anything else sizes itself to 200 through a heap allocation by count (a grep cannot find an allocation sized from the DAT count), and the load path's handling of manager `+0x68`.

## Mod surface

What a mod adding systems or sectors would have to change in `REBEXE.EXE` (besides the DATs):

1. **More than 200 systems:** patch `FUN_005562e0` (alloc), `FUN_0058af80` (zero-fill), `FUN_0058b060` (stride, `-100`, 200 × 2 loops, sort count, the 1600-byte stack buffer) and `FUN_0058afd0` (stride, `0x13a0c`). The stack buffer needs a new frame size or a heap buffer: a code cave, not a constant.
2. **Ids outside 100-299:** same functions; also keep ids distinct modulo 1024 for the sector-window key (`FUN_00429ce0`).
3. **Fewer or other sectors:** no code change for 1-3 `galaxy_size` values. A size-4 tier needs `FUN_00409fe0`, `FUN_0055d670`, `FUN_0055d6d0` and an SDPRTB column.
4. **Systems in family `0x98+` or sectors beyond `0x8f`:** the 271 + 211 + 64 + 50 family-range sites. Reuse `0x90`-`0x97` ids instead (8 families, each ids up to `0xffffff`).
5. **Coordinates outside 0-1023:** `.rdata` `0x658bd8`/`0x658bda`, the starfield scale in `FUN_00425d00`, and the sector-window half test.
6. **A new galaxy without code changes:** keep ids 100-299 (reuse or reassign), keep every system in a sector that the chosen size creates, and keep coordinates in 0-1023: 200 slots, any layout, any grouping.

### Proposed follow-up questions

- Does a loaded save have manager `+0x68` null, and what do the retreat readers do then? (`FUN_00554150`, `FUN_005545e0`, `FUN_00513a50`).
- What are families `0xa0`-`0xa5` (one per system in the saves)? (`FUN_00537180`, `FUN_004f3220`, `FUN_0044acd0`).
- How does `FUN_00519d00` use the buckets afterwards, and are Yavin/Coruscant forced there? (`FUN_0051a280`, `FUN_005597e0`, SDPRTB `0x1e00`/`0x1e01`).
- Is `0x28` in `FUN_0051a280` a batch size? Count calls on a 200-system start.
- What reads the SYFCCRTB/SYFCRMTB/SCTRANTB tables named in the RCDATA block (not shipped in the data folder)? (`FUN_005674e0`, `FUN_0058b420`).

## Open questions

- The tail of `FUN_00519d00` (`FUN_005f54d0`/`FUN_004f4270`, a list-pick by random index) is read as bucket assignment; the bucket meaning (Alliance/Empire × two flags and a neutral remainder) is `hyp:`. SDPRTB `0x1e00` (id 30) holds 10-40 per side and difficulty column, `0x1e01` (id 31) 0-10 (side 1 is 0 in every column).
- Whether the saves' header word 3 (2, 1, 3, 4 in the four sample saves) is the galaxy size: not settled, and it does not follow the system counts above.
- The sector-window column positions are from `sector-window-placement.md`; `FUN_00429ce0`'s side branch was not reread.
