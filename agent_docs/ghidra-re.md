---
title: "Ghidra Reverse Engineering"
description: "RE status and methodology for REBEXE.EXE decompilation (22,741 functions)"
category: "agent-docs"
created: 2026-03-11
updated: 2026-03-16
tags: [ghidra, reverse-engineering, rebexe, gnprtb]
---

# Ghidra Reverse Engineering

REBEXE.EXE (2.8MB) contains ALL game logic — 22,741 functions, ~4,900 decompiled. STRATEGY.DLL is resource-only (29MB sprites, 9KB CRT). RE is **complete** for implementation purposes. See `ghidra/notes/` for the full corpus: 4 scholar documents (4,179 lines), 11 markdown docs, ~4,900 decompiled C files, 8 Jython scripts.

## RE Status: COMPLETE

| What | Status |
|------|--------|
| Combat call chain | Fully traced (orchestrator → 4 subsystems → per-unit) |
| Bombardment formula | **Decoded**: `sqrt(delta²) / GNPRTB[0x1400]` |
| Space combat pipeline | 7 phases mapped, vtable dispatch identified |
| Ground combat | Troop iteration at +0x96, per-unit via vtable +0x330 |
| GNPRTB parameters | **111 mapped**: 34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side) |
| Game object layout | 10+ field offsets confirmed across all entity types |
| C++ class hierarchy | Reconstructed: CRebObject → CNotifyObject → CCombatUnit |
| Entity type codes | 8 family byte ranges identified |
| Scripted events | 15+ story events mapped, 50 event IDs |
| Modder documentation | 4 scholar documents ready |

## Setup

| Component | Location |
|-----------|----------|
| Ghidra install | `~/ghidra/` |
| Launch | `ghidra` (alias in ~/.zshrc) |
| Java | Temurin JDK 25.0.2 |
| Project | `open-rebellion/ghidra/Open Rebellion Ghidra.gpr` |
| GhidraMCP plugin | LaurieWired v11.3.2 — REST API on `:8080` (caps at 99 results) |
| Bridge script | `~/ghidra/GhidraMCP/bridge_mcp_ghidra.py` (bethington v4.3.0 script, old plugin JAR) |
| pyghidra-mcp | Config fixed: `--project-path`, `--force-analysis`, `--wait-for-analysis` |

**Note**: Bethington GhidraMCP v4.3.0 JAR requires Ghidra 12.0.3 (we have 11.3.2). The bridge script is updated but the Java plugin is the old LaurieWired version. Use Jython scripts for unlimited function access.

## Target Files

| File | Size | Functions | Decompiled | Content |
|------|------|-----------|------------|---------|
| **REBEXE.EXE** | 2.8MB | 22,741 | **~4,900** | ALL game logic |
| COMMON.DLL | 2.9MB | TBD | 0 | MFC/Win32 shared library |
| STRATEGY.DLL | 29MB | 43 (CRT) | N/A | Resource-only (sprites) |
| TACTICAL.DLL | 7.8MB | TBD | 0 | Likely resource-only |
| TEXTSTRA.DLL | 150KB | — | — | Done — parsed via pelite |

## Key Findings

### GNPRTB Parameter System
Two binding tables map GNPRTB parameter IDs to global data addresses:
- **FUN_0053e450**: 34 general bindings (28 base + 6 per-side, IDs 0x0a00-0x0a21) — travel, skills, economy
- **FUN_0055cb60**: 77 combat parameters (IDs 0x1400-0x1445) — damage, shields, bombardment
- `DAT_006bb6e8` = parameter 0x1400 = bombardment base divisor
- `DAT_00661a88` = difficulty modifier table
- Runtime struct: 68 bytes (vtable + base fields + 8 i32 difficulty values at offset 36)

### Game Object Layout
| Offset | Type | Field | Notes |
|--------|------|-------|-------|
| +0x00 | void* | vtable | C++ vtable pointer |
| +0x50 | uint | status_flags | bit0=active, bit3=fighter_combat_eligible |
| +0x58 | uint | combat_phase_flags | Space combat bitfield |
| +0x60 | int | hull_current / squad_size | Polymorphic by vtable — same offset for ships and squadrons |
| +0x64 bits 0-3 | 4-bit | shield_recharge_rate | 0-15, XOR-masked read-modify-write |
| +0x64 bits 4-7 | 4-bit | weapon_recharge_rate | 0-15, same word as shield |
| +0x66 | short | base_loyalty | 0-100 |
| +0x78 bit 7 | bit | special_entity_flag | Alt shield path for special entities |
| +0x8a | short | enhanced_loyalty | Bonus from missions, 0-0x7fff |
| +0x96 | short | regiment_strength | Ground troops, 0=destroyed |
| +0x9a | short | hyperdrive_modifier | Han Solo bonus, no upper bound |
| +0xac bit 0 | bit | alive_flag | Combat-ready when set |

### Entity Family Bytes (DatId >> 24)
| Range | Type |
|-------|------|
| 0x08-0x0f | Characters |
| 0x14-0x1b | Troops / Special Forces |
| 0x30-0x3b | Capital Ships + Fighters |
| 0x34 | Death Star (special) |
| 0x71-0x72 | Fighter squadron types |
| 0x73-0x74 | Special combat entity |
| 0x90-0x98 | Star Systems |

### C++ Class Hierarchy
```
CRebObject
└── CNotifyObject
    ├── CCombatUnit
    │   ├── CCapitalShip
    │   ├── CDeathStar
    │   └── CFighterSquadron
    ├── CTroopRegiment
    ├── CSpecialForces
    ├── CFacility
    ├── CCharacter
    │   ├── CMajorCharacter
    │   └── CMinorCharacter
    ├── CStarSystem
    ├── CFleet
    └── CMission
```

## Scholar Documents

Read these when implementing or modding:

| Document | When to Read |
|----------|-------------|
| `ghidra/notes/rust-implementation-guide.md` | Before implementing combat in Rust |
| `ghidra/notes/modders-taxonomy.md` | When designing mod system or total conversion support |
| `ghidra/notes/cpp-class-hierarchy.md` | When mapping C++ objects to Rust structs |
| `ghidra/notes/annotated-functions.md` | When you need exact field offsets or game rules |
| `ghidra/notes/combat-formulas.md` | Master reference for all RE findings |

## Ghidra Scripts (8 total)

Run via Jython: `exec(open("path/to/script.py").read())`

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

1. Open Ghidra → Open Rebellion Ghidra project → double-click REBEXE.EXE
2. Enable GhidraMCP plugin (File → Configure → Developer)
3. Decompile via API: `curl -X POST http://127.0.0.1:8080/decompile -d "FUN_ADDR"`
4. Or use Jython scripts for batch operations
5. Document findings in `ghidra/notes/`
