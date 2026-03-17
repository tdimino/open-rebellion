---
title: "Ghidra Project — Open Rebellion"
description: "Top-level index for the Ghidra RE project structure and resources"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# Ghidra Project — Open Rebellion

Exhaustive reverse engineering of Star Wars Rebellion (1998, LucasArts) for game reimplementation and total conversion modding.

## Structure

```
ghidra/
├── INDEX.md                    ← you are here
├── Open Rebellion Ghidra.gpr   # Ghidra project file (gitignored)
├── Open Rebellion Ghidra.rep/  # Ghidra project data (gitignored)
├── notes/                      # RE documentation and decompiled C
│   ├── INDEX.md                # Full index of all notes
│   ├── combat-formulas.md      # Master RE reference
│   ├── COMBAT-SUMMARY.md       # Combat call chain + confirmed formulas
│   ├── space-combat.md         # Space combat 7-phase pipeline
│   ├── ground-combat.md        # Ground combat troop resolution
│   ├── bombardment.md          # Bombardment formula (Euclidean distance)
│   ├── annotated-functions.md  # 1,662 lines — struct layouts, renamed vars, game rules
│   ├── modders-taxonomy.md     # 805 lines — 10 game systems for total conversion mods
│   ├── rust-implementation-guide.md  # 1,267 lines — C→Rust translation for Knesset Tiamat
│   ├── cpp-class-hierarchy.md  # 445 lines — vtable map, inheritance, field layouts
│   └── FUN_*.c                 # ~4,900 decompiled C pseudocode files
└── scripts/                    # Ghidra Jython scripts (8 total)
    ├── FindAllFunctions.py     # x86 prologue scanner
    ├── DumpStrings.py          # Keyword string search → file
    ├── DumpCombatXrefs.py      # String → function xref tracer
    ├── DumpCallers.py          # Direct caller finder
    ├── DumpCombatRegion.py     # Function listing in combat area
    ├── FindCombatMath.py       # Combat math pattern search
    ├── DumpAllGameFunctions.py # Exhaustive 4,938-function catalog with strings
    └── DumpGNPRTBXrefs.py      # GNPRTB parameter → function tracer
```

## Quick Reference

| Binary | Size | Functions | Decompiled | Status |
|--------|------|-----------|------------|--------|
| REBEXE.EXE | 2.8MB | 22,741 | **~4,900** | Exhaustive — every function >100 bytes |
| COMMON.DLL | 2.9MB | TBD | 0 | Imported, not yet analyzed |
| STRATEGY.DLL | 29MB | 43 (CRT) | N/A | Resource-only — no game logic |

## Scholar Documents

| Document | Lines | Purpose |
|----------|-------|---------|
| annotated-functions.md | 1,662 | Struct layouts, renamed variables, game rules, 50 event IDs |
| modders-taxonomy.md | 805 | 10 game systems categorized for Yuuzhan Vong / Thrawn / KOTOR total conversions |
| rust-implementation-guide.md | 1,267 | Maps decompiled C to Open Rebellion's advance() pattern |
| cpp-class-hierarchy.md | 445 | CRebObject → CNotifyObject → CCombatUnit hierarchy, 19 vtable slots |

## Key Discoveries

- **STRATEGY.DLL is resource-only** — all game logic in REBEXE.EXE
- **Bombardment formula**: `damage = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²) / GNPRTB[0x1400]`
- **111 GNPRTB parameters mapped**: 34 general (28 base + 6 per-side) + 77 combat (25 base + 52 per-side)
- **Hull and squad_size share offset +0x60** — polymorphism by vtable, not separate structs
- **Shield/weapon recharge packed into 4-bit nibbles** at offset +0x64
- **Observer/notification architecture** — all combat dispatched via vtable, not direct calls
- **C++ class hierarchy reconstructed**: CRebObject → CNotifyObject → CCombatUnit/CCharacter/CStarSystem

## How to Use

1. Open Ghidra: `ghidra` (alias in ~/.zshrc)
2. Open project: `Open Rebellion Ghidra.gpr`
3. Double-click REBEXE.EXE in Program Trees
4. GhidraMCP plugin on `:8080` — `curl -X POST http://127.0.0.1:8080/decompile -d "FUN_ADDR"`
5. Jython scripts: Window → Jython → `exec(open("path/to/script.py").read())`

See [notes/INDEX.md](notes/INDEX.md) for the complete function inventory.
