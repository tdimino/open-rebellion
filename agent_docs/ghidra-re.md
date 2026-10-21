---
title: "Ghidra Reverse Engineering"
description: "RE status and methodology for REBEXE.EXE decompilation (22,741 functions)"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-19
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
| Planetary assault | Order `0x234`, enable `FUN_005814d0`, run `FUN_00563390` -> `FUN_0058c120` (`ghidra/notes/ground-assault.md`) |
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
| Ghidra | 12.1.4, Homebrew; `/opt/homebrew/opt/ghidra/libexec/support/analyzeHeadless` |
| Project | `open-rebellion/ghidra/Open Rebellion Ghidra.gpr` |
| Scripts | `ghidra/scripts/*.java` (headless, read-only; Workflow, below) |

The `.py` scripts need PyGhidra, which this install lacks, so headless runs
refuse them. The project imports `REBEXE.EXE` as `x86:LE:32:default` with
compiler spec `windows` (compiler `visualstudio`); any other tool pointed at
the binary must use the same, not `borlandcpp`.

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
- **FUN_0053e450**: 34 general bindings (28 base + 6 per-side, IDs 0x0a00-0x0a21)—travel, skills, economy
- **FUN_0055cb60**: 77 parameters from ID 0x1400
- `DAT_006bb6e8` = parameter 0x1400 = the transit distance divisor (5; `ghidra/notes/build-delivery.md`)
- `DAT_00661a88` = the percentage base (100)
- Runtime struct: 68 bytes (vtable + base fields + 8 i32 difficulty values at offset 36)

### AI and Mission Functions

The AI functions' roles (`FUN_0052e970`, `FUN_00506ea0`, `FUN_00508250` and
others) are in `ghidra/notes/ai-behavior-analysis.md`. Mission rolls are in
`ghidra/notes/decoy-roll.md` and `mission-lifecycle.md`; community `sub_`
addresses come from another REBEXE build (`ghidra/notes/community-address-remap.md`).

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
Ghidra never defined, by address). For a range: `DecompileRange.java`
(every function, `index.tsv` and `gaps.tsv`), then `RecoverGaps.java` (every
function hidden in those gaps). For evidence tables: `CallSiteArgs.java`
(the arguments pushed at every call to a function, such as a registry),
`FindDatIds.java` (every literal record id), `FamilyChecks.java` (every
`SHR reg,0x18` family test and what it compares) and `FindString.java` (a
text's address and the functions using it). `ghidra/` is ignored, so add a new
script with `git add -f`, as a cited `.c` file is. The Python scripts below
need PyGhidra, so headless runs here refuse them:

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

When another agent holds the project lock (`ghidra/Open Rebellion Ghidra.lock`;
headless runs fail with "Unable to lock project!"), clone the project into a
scratch directory, run on the clone read-only, then delete the clone. Never
remove another agent's lock file:

```bash
cp -c -R "ghidra/Open Rebellion Ghidra.rep" "ghidra/Open Rebellion Ghidra.gpr" SCRATCH/
/opt/homebrew/opt/ghidra/libexec/support/analyzeHeadless SCRATCH \
  "Open Rebellion Ghidra" -process REBEXE.EXE -readOnly -noanalysis \
  -scriptPath "$PWD/ghidra/scripts" -postScript …
```

### Porting an original flow

A plan's account of how the original behaves is a hypothesis until a trace
confirms it. In F-019 the planned drag entry was a Move, the "refusal
strings" were advisor reactions, and the dialog's targets were systems only.

1. Read the manual first ([manual-cross-check.md](manual-cross-check.md): the
   Markdown transcription for search, the PDF for figures).
2. Trace the window procedure and the hub it uses, for both sides.
3. Write the note in `ghidra/notes/` with addresses, field offsets and the
   cited `.c` files, then plan the port from the note. A note states current
   facts: when one is wrong, rewrite it to say what the code does, with no
   "corrected", "an earlier version said" or "not X" prose (git holds the
   history), and put an unsettled claim under the note's open questions.
4. Mark what the port leaves out with `port:`, and name the finding that owns
   it in the audit ledger.
