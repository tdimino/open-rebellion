# Ghidra Project — Open Rebellion

Reverse engineering of Star Wars Rebellion (1998) binaries for combat formula extraction.

## Structure

```
ghidra/
├── INDEX.md                    ← you are here
├── Open Rebellion Ghidra.gpr   # Ghidra project file (gitignored)
├── Open Rebellion Ghidra.rep/  # Ghidra project data (gitignored)
├── notes/                      # RE documentation and decompiled C
│   ├── INDEX.md                # Full index of all notes
│   ├── combat-formulas.md      # Master reference document
│   ├── COMBAT-SUMMARY.md       # Combat call chain + formulas
│   ├── space-combat.md         # Space combat 7-phase pipeline
│   ├── ground-combat.md        # Ground combat troop resolution
│   ├── bombardment.md          # Bombardment formula (Euclidean distance)
│   └── FUN_*.c                 # Decompiled C pseudocode (~40 files)
└── scripts/                    # Ghidra Jython scripts
    ├── FindAllFunctions.py     # x86 prologue scanner
    ├── DumpStrings.py          # Keyword string search → file
    ├── DumpCombatXrefs.py      # String → function xref tracer
    ├── DumpCallers.py          # Direct caller finder
    ├── DumpCombatRegion.py     # Function listing in combat area
    └── FindCombatMath.py       # Combat math pattern search
```

## Quick Reference

| Binary | Functions | Status |
|--------|-----------|--------|
| REBEXE.EXE | 22,741 | **Primary target** — all game logic |
| COMMON.DLL | TBD | Imported, not yet analyzed |
| STRATEGY.DLL | 43 (CRT) | Resource-only — no game logic |

## How to Use

1. Open Ghidra: `ghidra` (alias in ~/.zshrc)
2. Open project: `Open Rebellion Ghidra.gpr`
3. Double-click REBEXE.EXE in Program Trees
4. GhidraMCP plugin on `:8080` — `curl -X POST http://127.0.0.1:8080/decompile -d "FUN_ADDR"`
5. Jython scripts: Window → Jython → `exec(open("path/to/script.py").read())`

See [notes/INDEX.md](notes/INDEX.md) for the complete function inventory.
