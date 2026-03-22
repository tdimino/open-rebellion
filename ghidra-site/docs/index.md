---
title: "Ghidra Reverse Engineering—REBEXE.EXE"
description: "Exhaustive decompilation of Star Wars Rebellion (1998)—5,127 functions, 7 scholar docs, 8 scripts"
category: ghidra
created: 2026-03-13
updated: 2026-03-22
tags: [ghidra, reverse-engineering, rebexe, decompilation, combat, ai, gnprtb]
---

<div class="tanit-logo" style="text-align:center; margin-bottom:1em;">
  <img class="dark-only" src="/open-rebellion/assets/tanit-dark.png" alt="Symbol of Tanit" style="height:80px; width:auto;">
  <img class="light-only" src="/open-rebellion/assets/tanit-light.png" alt="Symbol of Tanit" width="80">
</div>

# Ghidra Reverse Engineering—REBEXE.EXE

Exhaustive decompilation of Star Wars Rebellion (1998, LucasArts) for the Open Rebellion reimplementation.

## What We Decompiled

| Target | Size | Total Functions | Decompiled | Coverage |
|--------|------|-----------------|------------|----------|
| **REBEXE.EXE** | 2.8 MB | 22,741 | **5,127** | Every function >100 bytes |
| COMMON.DLL | 2.9 MB |—| 0 | Imported, not analyzed |
| STRATEGY.DLL | 29 MB | 43 (CRT only) | N/A | Resource-only—no game logic |

**Key finding:** All game logic lives in REBEXE.EXE. STRATEGY.DLL is a resource container (BMPs, strings, data tables). COMMON.DLL handles DirectPlay networking.

## Scholar Documents (7 analysis docs, 6,049 lines)

| Document | Lines | What It Covers |
|----------|-------|----------------|
| `notes/annotated-functions.md` | 1,662 | Struct field layouts (+0x60 hull, +0x64 shield nibbles, +0x96 regiment strength, +0xac alive flag), renamed variables, 50 event IDs (0x127–0x370), game rules |
| `notes/modders-taxonomy.md` | 805 | 10 game systems mapped for total conversion mods (Yuuzhan Vong, Thrawn's Revenge, KOTOR). Per-system function addresses, GNPRTB parameters, mod guidance |
| `notes/rust-implementation-guide.md` | 1,267 | Maps decompiled C to Open Rebellion's stateless `advance()` pattern. `CombatSystem::resolve_space()`, `GnprtbParams`, `MstbTable`, 15 missing CapitalShipClass fields |
| `notes/cpp-class-hierarchy.md` | 445 | CRebObject → CNotifyObject → CCombatUnit hierarchy. 19 vtable slots, 6 vtable pointer constants, field layouts, setter-notify-event pattern |
| `notes/entity-system.md` | 668 | Characters (8 enhanced skills, Force/Jedi 6-tier, betrayal), game objects (5 destruction variants), fleets (4 events), factions (Alliance/Empire/Neutral bit encoding) |
| `notes/mission-event-cookbook.md` | 724 | 9+ mission types (FUN_0050d5a0 13-case switch), 4 story event chains (Dagobah, Vader's Palace, Bounty Hunters, Jabba), event ID registry |
| `notes/economy-systems.md` | 478 | Resources (energy/material at +0x5c–0x68), 5 ControlKind states, uprising tables (UPRIS1TB/2TB), blockade manufacturing halt, repair formulas |

## Combat Subsystem (3 dedicated docs)

| Document | Subsystem | Key Functions |
|----------|-----------|---------------|
| `notes/space-combat.md` | Space combat—7-phase auto-resolve pipeline | FUN_00549910, FUN_00544030, FUN_005443f0 |
| `notes/ground-combat.md` | Ground combat—troop iteration + per-unit resolution | FUN_00560d50, FUN_004ee350, FUN_005617b0 |
| `notes/bombardment.md` | Orbital bombardment—Euclidean distance formula | FUN_00556430, FUN_0055d8c0, FUN_0055d860 |

## AI Pipeline (6 core functions fully traced)

| # | Function | Purpose | Lines | Status in Open Rebellion |
|---|----------|---------|-------|--------------------------|
| 1 | FUN_00519d00 | Galaxy-wide system bucketing (7 categories) | 252 | DONE—evaluate_galaxy_state() + ratio scoring |
| 2 | FUN_00537180 | Primary fleet deployment (per-system iteration) | 381 | AUGMENTED—per-fleet scoring with 4 factors |
| 3 | FUN_005385f0 | Secondary deployment (redistribution) | 252 | DONE—aggression-scaled redistribution |
| 4 | FUN_00502020 | Garrison strength assessment | 897 | DONE—ships + troops + facilities |
| 5 | FUN_00508250 | Pre-dispatch validation (18 boolean checks) | ~200 | DONE—can_dispatch() with 6 checks |
| 6 | FUN_00520580 | Movement order issuance | ~300 | DONE—with already_moving dedup |

## Key Discoveries

- **111 GNPRTB parameters mapped**: 34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side). These are the "knobs" that control game difficulty and balance.
- **Bombardment formula**: `damage = sqrt((atk[0]-def[0])^2 + (atk[1]-def[1])^2) / GNPRTB[0x1400]`
- **Hull and squad_size share offset +0x60**—polymorphism by vtable, not separate structs
- **Shield/weapon recharge packed into 4-bit nibbles** at offset +0x64
- **C++ class hierarchy**: CRebObject → CNotifyObject → CCombatUnit/CCharacter/CStarSystem—all combat dispatched via vtable
- **AI is omniscient**—no fog-of-war check in FUN_00519d00 (galaxy evaluation)
- **Turn processing**: FUN_004927c0 (9K lines) is the master tick function—research dispatch, fleet orders, manufacturing, event processing

## Ghidra Scripts (8 Jython scripts)

| Script | Purpose |
|--------|---------|
| `FindAllFunctions.py` | x86 prologue scanner—finds all function entry points |
| `DumpAllGameFunctions.py` | Exhaustive catalog of 4,938 functions with referenced strings |
| `DumpStrings.py` | Keyword string search → file output |
| `DumpCombatXrefs.py` | String → function cross-reference tracer |
| `DumpCallers.py` | Direct caller finder for a target function |
| `DumpCombatRegion.py` | Function listing in combat address range |
| `FindCombatMath.py` | Pattern search for combat math operations |
| `DumpGNPRTBXrefs.py` | GNPRTB parameter → function tracer |

## Decompiled Functions by Game System

| System | Address Range | Functions | Key Entry Points |
|--------|--------------|-----------|-----------------|
| Game init / CRT | 0x401000–0x403e90 | ~100 | entry, __security_init_cookie |
| Galaxy map (GDI) | 0x422000–0x43a000 | ~50 | FUN_00422ce0 (11K), FUN_00433e40 (6K) |
| UI dialogs | 0x43a000–0x470000 | ~200 | FUN_0044c630 (6K), FUN_004665f0 (6K) |
| Turn processing | 0x490000–0x4a0000 | ~50 | FUN_004927c0 (9K master tick) |
| Characters | 0x4ee000–0x4f4000 | ~80 | Enhanced skills, Force system, loyalty |
| Missions | 0x4f4000–0x515000 | ~120 | FUN_0050d5a0 (13-case switch) |
| AI system | 0x515000–0x540000 | ~150 | 6-function pipeline (see table above) |
| Space combat | 0x540000–0x560000 | ~100 | 7-phase pipeline, CombatPhaseFlags |
| Ground combat | 0x560000–0x570000 | ~40 | Troop iteration, regiment damage |
| Economy | 0x570000–0x590000 | ~60 | Manufacturing, resources, blockade |
| Events | 0x590000–0x5a0000 | ~40 | Scripted chains, condition/action eval |
| DirectPlay (net) | 0x5a0000–0x5b0000 | ~30 | Session management, peer-to-peer |
| MSVC runtime | 0x5b0000+ | ~4,000 | CRT, STL, exception handling |

## Directory Structure

```
ghidra/
├── README.md                     ← you are here
├── INDEX.md                      # Detailed project index
├── Open Rebellion Ghidra.gpr     # Ghidra project file (not tracked)
├── Open Rebellion Ghidra.rep/    # Ghidra project data (not tracked)
├── notes/
│   ├── INDEX.md                  # Master index of all notes
│   ├── FUNCTION_INDEX.md         # 5,127-function catalog
│   ├── 7 analysis .md files      # Scholar documents (see table above)
│   ├── 3 combat .md files        # Combat subsystem docs
│   ├── 4 review .md files        # Internal review notes
│   └── ~5,127 .c files           # Decompiled C pseudocode
└── scripts/
    └── 8 .py files               # Ghidra Jython scripts
```

## How to Use

1. Open Ghidra: `ghidra` (alias)
2. Open project: `Open Rebellion Ghidra.gpr`
3. Double-click REBEXE.EXE in Program Trees
4. Navigate to function: Go → Address → `0x00519d00`
5. [GhidraMCP](https://github.com/LaurieWired/GhidraMCP) plugin (LaurieWired v11.3.2): `curl -X POST http://127.0.0.1:8080/decompile -d "FUN_ADDR"`
6. Jython scripts: Window → Jython → `exec(open("path/to/script.py").read())`

## Relationship to Open Rebellion

The decompiled code is the ground truth for the reimplementation. Every simulation system in `crates/rebellion-core/src/` is ported from specific decompiled functions. The `agent_docs/systems/ai-parity-tracker.md` maps each original function to our Rust implementation with DONE/AUGMENTED/PARTIAL/MISSING status.

The `ghidra/` directory is gitignored (binary project files + non-redistributable decompiled code). This README is the only tracked file. The tracked reference for agents is `agent_docs/ghidra-re.md`.
