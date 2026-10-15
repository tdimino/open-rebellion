---
title: "Ghidra Reverse Engineering"
description: "RE status and methodology for REBEXE.EXE decompilation (22,741 functions)"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-14
tags: [ghidra, reverse-engineering, rebexe, gnprtb]
---

# Ghidra Reverse Engineering

REBEXE.EXE (2.8MB) contains the game logic. STRATEGY.DLL is primarily a
resource container. Simulation-focused reverse engineering is substantial, but
interface reverse engineering is still in progress. The text export contains
4,934 canonical `FUN_????????.c` files, of which 2,790 are zero-byte
placeholders. Query the saved Ghidra project when an interface target is empty.
See the [interface reverse-engineering ledger](../docs/qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md)
for the active UI evidence queue.

## RE Status: Simulation Substantial, Interface In Progress

| What | Status |
|------|--------|
| Space battles | One resolver: the tactical engine `FUN_0040a050`, played or headless (`ghidra/notes/simulate-runs-tactical-engine.md`) |
| Bombardment, Destroy System | Orders `0x220`-`0x223`, gate `FUN_00582440`, run `FUN_0058d800` (`ghidra/notes/bombardment.md`) |
| Planetary assault | Open (`or-q-ground-resolution-nms`, `or-q-regiment-combat-eaxn`) |
| GNPRTB parameters | **111 mapped**: 34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side) |
| Game object layout | 10+ field offsets confirmed across all entity types |
| C++ class hierarchy | Reconstructed by DAT family (`ghidra/notes/cpp-class-hierarchy.md`) |
| Entity family bytes | The DAT header ranges (`ghidra/notes/record-registry.md`) |
| Scripted events | 15+ story events mapped, 50 event IDs |
| Modder documentation | 4 scholar documents ready |
| Strategic shell geometry and routing | Exact apertures, rail rectangles, controls, state paint, and modeless-window routing recovered |
| Advisor frame and action pipeline | Partial. Custom type-302 frames and SPT/BIN/FDT mappings still require decoding |
| Tactical and multiplayer interface | Partial. Event/resource vocabulary is mapped; composition and live behavior remain open |

## Setup

| Component | Location |
|-----------|----------|
| Ghidra install | `~/ghidra/` |
| Launch | `ghidra` (alias in ~/.zshrc) |
| Java | Temurin JDK 25.0.2 |
| Project | `open-rebellion/ghidra/Open Rebellion Ghidra.gpr` |
| GhidraMCP plugin | LaurieWired v11.3.2. REST API on `:8080` (caps at 99 results) |
| Bridge script | `~/ghidra/GhidraMCP/bridge_mcp_ghidra.py` (bethington v4.3.0 script, old plugin JAR) |
| pyghidra-mcp | Config fixed: `--project-path`, `--force-analysis`, `--wait-for-analysis` |

**Note**: the GhidraMCP rows above are historical. Current work runs Ghidra
12.1.4 (Homebrew) headless and read-only with the Java scripts in `ghidra/scripts/`
(Workflow, below). The `.py` scripts need PyGhidra, which this install lacks,
so headless runs refuse them. The project imports `REBEXE.EXE` as
`x86:LE:32:default` with compiler spec `windows` (compiler `visualstudio`);
any other tool pointed at the binary must use the same, not `borlandcpp`.

## Target Files

| File | Size | Functions | Decompiled | Content |
|------|------|-----------|------------|---------|
| **REBEXE.EXE** | 2.8MB | 22,741 discovered entries | **4,934 canonical export targets; 2,144 non-empty** | Main executable logic. The saved project is authoritative for empty exports. |
| COMMON.DLL | 2.9MB | Not yet counted | Partial | Shared code, templates, bitmaps, sounds, and multiplayer resources |
| STRATEGY.DLL | 29MB | 43 analyzed CRT entries | N/A | Primarily strategic resources |
| TACTICAL.DLL | 7.8MB | Not yet counted | Partial | Tactical resources plus loader/event paths reached from REBEXE |
| TEXTSTRA.DLL | 150KB | N/A | N/A | Strings parsed via pelite |

## Key Findings

### GNPRTB Parameter System
Two binding tables map GNPRTB parameter IDs to global data addresses:
- **FUN_0053e450**: 34 general bindings (28 base + 6 per-side, IDs 0x0a00-0x0a21) — travel, skills, economy
- **FUN_0055cb60**: 77 parameters from ID 0x1400
- `DAT_006bb6e8` = parameter 0x1400 = the transit distance divisor (5; `ghidra/notes/build-delivery.md`)
- `DAT_00661a88` = the percentage base (100)
- Runtime struct: 68 bytes (vtable + base fields + 8 i32 difficulty values at offset 36)

### Game Object Layout

Field offsets by class, with their setters, are in
`ghidra/notes/entity-system.md` (§2) and `ghidra/notes/cpp-class-hierarchy.md`.
Common misreadings: on a character `+0x96` is the CommandKind (officer rank)
and `+0xac` bit 0 the prisoner bit; `+0x58` means something different in
each class (MissionMgr story flags, the Emperor flag, a fleet's bombard bit);
`+0x24 >> 4 & 3` numbers the object's copy (master, Alliance view, Empire
view), not a difficulty.

### Interface Hubs

The original builds its interface from a few shared mechanisms. Porting one
makes every feature on it cheap, so trace the hub before the feature:

| Hub | Functions | Serves | Note |
|-----|-----------|--------|------|
| Order objects | `FUN_0051f8f0` factory table, `FUN_0051d990` order list | Every right-click order: Move, Mission, Retire, Command | `ghidra/notes/object-popup-menu.md` |
| Game Menu Window | `FUN_00442860`, `FUN_00442a80`, `FUN_004424c0` | The speed menu and the object pop-up menu | `ghidra/notes/object-popup-menu.md` |
| Galaxy view modes | `FUN_00422ce0` (`+0xc0` mode, `+0xc4` order) | Targeting, move drops, list drags | `ghidra/notes/object-popup-menu.md` |
| Advisor reactions | Side classes at `+0xc0`: vtables `0x0065c4c8` and `0x0065c4a0`; the 88-slot `+0x168` table | Refusals and about 45 game events (`FUN_004c44b0`) | `ghidra/notes/mission-dialog.md`, "Refusal" |

The two sides often run separate classes with different slot numbers, as the
advisors do. Trace and test both.

### Entity Family Bytes (DatId >> 24)
| Range | Type |
|-------|------|
| 0x08-0x0f | Fleets |
| 0x10-0x13 | Regiments |
| 0x14-0x1b | Capital ships (0x18 the Death Star, CAPSHPSD record 136) |
| 0x1c-0x1f | Fighter squadrons |
| 0x20-0x2f | Facilities |
| 0x30-0x35 | Major characters (0x34000280 Emperor Palpatine, 0x35000281 Darth Vader) |
| 0x38 | Minor characters |
| 0x3c | Special forces |
| 0x41-0x73 | Missions (0x71 Dagobah, 0x73 Pickup) |
| 0x80 | Sectors |
| 0x90, 0x92 | Star systems |

The class tree is in `ghidra/notes/cpp-class-hierarchy.md`.

## Scholar Documents

Read these when implementing or modding:

| Document | When to Read |
|----------|-------------|
| `ghidra/notes/rust-implementation-guide.md` | Before mapping original fields and families to Rust |
| `ghidra/notes/modders-taxonomy.md` | When designing mod system or total conversion support |
| `ghidra/notes/cpp-class-hierarchy.md` | When mapping C++ objects to Rust structs |
| `ghidra/notes/annotated-functions.md` | When you need exact field offsets or game rules |
| `ghidra/notes/COMBAT-SUMMARY.md` | Where each kind of combat is resolved |

## Ghidra Scripts

The Java scripts run headless (Workflow, below): `DecompileTargets.java`
(functions with callers and callees), `DecompileVtable.java` (a vtable's
slots), `FindOperandText.java` (instructions whose text matches, such as a
field write `+ 0xc0],`), `DumpReferences.java`, `DumpInstructions.java`,
`DumpPointerTable.java`, `DumpMemory.java`, `FindScalarUses.java`,
`DecompileCallersOf.java` (every function calling an import, such as
`DrawTextA`), `FindIndirectCalls.java`, `FindScalarReferences.java`,
`DumpFloatsAt.java`, and `CreateAndDecompileTargets.java` (a function
Ghidra never defined, by address). `ghidra/` is ignored, so add a new
script with `git add -f`, as a cited `.c` file is. The historical Jython
scripts below need PyGhidra:

| Script | Purpose |
|--------|---------|
| FindAllFunctions.py | x86 prologue scanner (found 22,741 functions) |
| DumpStrings.py | Keyword string search → ~/Desktop/rebellion-strings.txt |
| DumpCombatXrefs.py | String → function xref tracer |
| DumpCallers.py | Direct caller finder (confirmed virtual dispatch) |
| DumpCombatRegion.py | Function listing in combat area |
| FindCombatMath.py | Combat math pattern search |
| DumpAllGameFunctions.py | Exhaustive 4,938-function catalog with string references |
| DumpGNPRTBXrefs.py | GNPRTB parameter → consuming function tracer |

## 2026-03-23 GhidraMCP Session — 23 AI Functions Decoded

Decompiled via `curl -X POST http://127.0.0.1:8080/decompile -d "FUN_ADDR"` with REBEXE.EXE loaded in CodeBrowser.

### Key Findings

| Function | Lines | Finding |
|----------|-------|---------|
| `FUN_0052e970` | 53 | **Not a scoring function** — binary capacity check. Checks if entity (family 0x10-0x3f) fits deployment budget at `this+0x58 - this+0x5c`. Our 4-factor model is strictly superior. |
| `FUN_00506ea0` | 13 | Faction-specific evaluator pointer: Alliance at `DAT_006b2bb0+0xc4`, Empire at `+0xc8`. Different deployment budgets per faction. |
| `FUN_004927c0` | 2098 | Master turn processing. The **event 0x1f0** day-tick reading is refuted; see `ghidra/notes/timer-scheduler.md`. |
| `FUN_00520580` | 9 | 2-field struct setter (`*(this) = cmd; *(this+4) = param`). Not a transit calculator. |
| `FUN_0053b870` | 7 | Entity capacity reader: returns `*(entity + 0x4c)`. |
| `FUN_00508250` | 139 | **All 18 validator sub-functions decoded.** 2 are no-ops (FUN_0051ebb0 always returns 1). 4 match our existing checks. 12 are new capacity/composition/status checks. |

### Mission Probability Formulas (from TheArchitect2018 wiki)

Superseded. F-019 deleted `compute_table_input()`; the per-member roll and
its inputs are in `ghidra/notes/decoy-roll.md` and `mission-lifecycle.md`.
The wiki's `sub_` addresses come from a different REBEXE build
(`ghidra/notes/community-address-remap.md`). Historical list:
- Diplomacy: `(enemy_pop - our_pop) + diplomacy_rating` (sub_55ae50)
- Recruitment: `leadership - resistance` (sub_55aed0)
- Subdue: `(enemy_pop - our_pop) + diplomacy` (sub_55af50)
- DS Sabotage: `(espionage + combat) / 2` (sub_55b0a0)
- Escape: `((p3 + p2) - p4) - p5` + RNG roll (sub_55cfb0)

## Workflow

Run the saved project headless and read-only, from the repository root:

```bash
/opt/homebrew/opt/ghidra/libexec/support/analyzeHeadless "$PWD/ghidra" \
  "Open Rebellion Ghidra" -process REBEXE.EXE -readOnly -noanalysis \
  -scriptPath "$PWD/ghidra/scripts" \
  -postScript DecompileTargets.java OUTPUT_DIR FUN_00487c90 FUN_004861b0
```

Several `-postScript` pairs may follow one another. Keep scratch output
outside the repository; copy each `.c` a note cites into `ghidra/notes/` and
stage it with `git add -f` (`ghidra/` is ignored).

### Porting an original flow

A plan's account of how the original behaves is a hypothesis until a trace
confirms it. F-019 corrected three such accounts: the planned drag entry was a
Move, the "refusal strings" were advisor reactions, and the dialog's targets
were systems only.

1. Read the manual first
   (`docs/reference/campaign-history/archive/star-wars-rebellion-manual.pdf`). One sentence
   on p. 100 settled F-019's entry flow.
2. Trace the window procedure and the hub it uses, for both sides.
3. Write the note in `ghidra/notes/` with addresses, field offsets and the
   cited `.c` files, then plan the port from the note.
4. Mark what the port leaves out with `port:`, and name the finding that owns
   it in the audit ledger.
