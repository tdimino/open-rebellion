# Ghidra Reverse Engineering

STRATEGY.DLL (29MB) and TACTICAL.DLL (7.5MB) contain the combat formulas, mission probability tables, and AI decision logic for the original game. Ghidra RE is needed to fully understand these mechanics.

## Setup

| Component | Location |
|-----------|----------|
| Ghidra install | `~/ghidra/` |
| Launch | `ghidra` (alias in ~/.zshrc → `~/ghidra/ghidraRun`) |
| Java | Temurin JDK 25.0.2 (via `brew install --cask temurin`) |
| Project | `open-rebellion/ghidra/Open Rebellion Ghidra.gpr` |
| GhidraMCP plugin | Enabled in Developer Plugins — REST API on `:8080` |
| Bridge script | `~/ghidra/GhidraMCP/bridge_mcp_ghidra.py` |

## MCP Access

Two MCP servers configured in `~/.claude/settings.json`:

1. **`ghidra`** (bridge): `bridge_mcp_ghidra.py` → REST API at `http://127.0.0.1:8080/`
2. **`pyghidra-mcp`** (direct): targets project files via stdio

**Important**: MCP tools likely do NOT propagate to spawned teammate agents. For swarm tasks requiring Ghidra, the lead agent should query Ghidra and pass results to teammates via task descriptions.

## Target Files

| DLL | Size | Contains | Priority |
|-----|------|----------|----------|
| STRATEGY.DLL | 29MB | Combat resolution, AI strategy, mission formulas, event triggers, diplomacy calculations | High — blocks War Machine milestone |
| TACTICAL.DLL | 7.5MB | Tactical combat (2D battle view), weapon damage, shield mechanics, squadron behavior | Medium — needed for tactical combat mode |
| TEXTSTRA.DLL | 150KB | String resources (entity names) | Done — parsed via pelite in Rust |

## What to Look For

### Combat Formulas (STRATEGY.DLL)
- Space combat: damage calculation per weapon arc, shield absorption, hull damage
- Ground combat: troop engagement, orbital bombardment effectiveness
- Fleet auto-resolve: how the game decides battle outcomes without tactical view
- Blockade mechanics: when/how blockades trigger

### GNPRTB Parameter Usage
GNPRTB has 213 entries but only ~29% are documented. Each has 8 values (difficulty levels). Finding which GNPRTB index maps to which game mechanic requires tracing references in STRATEGY.DLL.

Known mappings (from community + rebellion2):
- Travel time multipliers
- Combat recovery rates
- Mission reward/penalty amounts
- Diplomacy shift magnitudes
- Uprising thresholds

Unknown (~150 parameters): Ghidra RE target. Look for functions that index into the GNPRTB table (likely a global array of 213 × 8 i32 values loaded at startup).

### Mission Probability Tables
The `*MSTB.DAT` files (IntTableEntry format) define skill threshold → probability curves for each mission type. STRATEGY.DLL reads these at mission execution time. Trace the code path from mission dispatch → table lookup → probability roll → outcome.

## Analysis Tips

1. **Auto-analysis**: Run Analysis → Auto Analyze with all options checked. The initial 43-function count was likely incomplete.
2. **String search**: Search for known string literals from TEXTSTRA to find cross-references
3. **Table access patterns**: Look for array indexing with constants matching GNPRTB entry counts (213) or mission counts (25)
4. **C++ vtables**: STRATEGY.DLL is likely C++ — check for vtable patterns to identify class hierarchies
5. **Export table**: Win32 DLL exports may reveal function names

## Project Files

```
open-rebellion/ghidra/
├── Open Rebellion Ghidra.gpr     # Ghidra project
├── Open Rebellion Ghidra.rep/    # Project repository (analysis DB)
├── notes/                        # RE notes (findings go here)
└── scripts/                      # Ghidra Python scripts
```

## Workflow

1. Open Ghidra: `ghidra` in terminal
2. Open project: `open-rebellion/ghidra/Open Rebellion Ghidra.gpr`
3. Double-click STRATEGY.DLL in project tree to open CodeBrowser
4. Ensure GhidraMCP plugin is active (Developer Plugins)
5. Use MCP tools from Claude Code, or browse manually in the decompiler
6. Document findings in `ghidra/notes/` as markdown files
