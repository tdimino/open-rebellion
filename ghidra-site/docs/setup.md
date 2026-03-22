# Ghidra + Claude Code Workflow

How we decompiled 5,127 functions from REBEXE.EXE using Ghidra, [GhidraMCP](https://github.com/LaurieWired/GhidraMCP), and Claude Code working together.

## Tools

| Tool | Version | Role |
|------|---------|------|
| [Ghidra](https://ghidra-sre.org/) | 11.3.2 | Disassembly + decompilation |
| [GhidraMCP](https://github.com/LaurieWired/GhidraMCP) | LaurieWired v11.3.2 | REST API bridge—exposes Ghidra's decompiler to Claude Code via HTTP |
| [Claude Code](https://claude.ai/claude-code) | Opus 4.6 | AI agent—reads decompiled C, traces call chains, writes scholar docs |
| 8 Jython scripts | Custom | Bulk function extraction, string search, GNPRTB tracing |

## Architecture

```
Ghidra (GUI)
  ├── REBEXE.EXE loaded as x86 PE32
  ├── GhidraMCP plugin on :8080
  │     ├── POST /decompile → decompiled C for any function
  │     ├── GET /methods → function listing
  │     └── GET /strings → string references
  └── Jython console → bulk scripts

Claude Code (terminal)
  ├── curl → GhidraMCP :8080 (decompile specific functions)
  ├── Jython scripts → dump all functions to .c files
  └── Scholar analysis → .md documents
```

## Setup Steps

### 1. Install Ghidra

Download from [ghidra-sre.org](https://ghidra-sre.org/). Requires Java 17+.

### 2. Install GhidraMCP

Clone [LaurieWired/GhidraMCP](https://github.com/LaurieWired/GhidraMCP) and follow the install instructions. The plugin adds a REST API to Ghidra's decompiler.

**Note**: The bethington fork (v4.3.0, 176 endpoints) requires Ghidra 12.0.3. We used LaurieWired's version which works with 11.3.2. The old plugin caps `/methods` and `/strings` at 99 results—use Jython scripts for full access.

### 3. Import the binary

```
File → Import File → REBEXE.EXE
Analyze → Auto Analyze (accept defaults)
```

The binary is a 2.8MB x86 PE32 executable with 22,741 functions.

### 4. Start GhidraMCP

Once Ghidra has the binary loaded and analyzed, enable the GhidraMCP plugin. It starts an HTTP server on port 8080.

Test it:
```bash
curl -X POST http://127.0.0.1:8080/decompile -d "FUN_00519d00"
```

This returns the decompiled C pseudocode for the AI galaxy evaluation function.

### 5. Claude Code workflow

From a Claude Code session in the open-rebellion project:

```bash
# Decompile a specific function
curl -s -X POST http://127.0.0.1:8080/decompile -d "FUN_00549910" | head -50

# Find functions referencing a string
curl -s http://127.0.0.1:8080/strings | grep -i "combat"

# Bulk dump via Jython script (run inside Ghidra console)
exec(open("ghidra/scripts/DumpAllGameFunctions.py").read())
```

Claude Code reads the decompiled C, traces call chains across functions, identifies struct field offsets, and produces the scholar documents in `ghidra/notes/`.

## The Jython Scripts

| Script | Purpose | Output |
|--------|---------|--------|
| `FindAllFunctions.py` | Scan for x86 function prologues | Function count |
| `DumpAllGameFunctions.py` | Decompile every function >100 bytes | 5,127 .c files |
| `DumpStrings.py` | Find functions referencing a keyword | Matching functions |
| `DumpCombatXrefs.py` | Trace string → function cross-references | Call chains |
| `DumpCallers.py` | Find direct callers of a target function | Caller list |
| `DumpCombatRegion.py` | List functions in an address range | Regional catalog |
| `FindCombatMath.py` | Search for math patterns (sqrt, multiply) | Combat formula candidates |
| `DumpGNPRTBXrefs.py` | Trace GNPRTB parameter references | Parameter → function mapping |

## Key Discoveries Made This Way

1. **STRATEGY.DLL is resource-only**—29MB of sprites, 9KB of CRT code. All game logic in REBEXE.EXE.
2. **Bombardment formula**—traced through 3 functions: `FUN_00556430` → `FUN_0055d8c0` → `FUN_0055d860`. Result: `damage = sqrt((atk-def)²) / GNPRTB[0x1400]`.
3. **AI is omniscient**—`FUN_00519d00` (galaxy evaluation) has no fog-of-war check. The AI sees everything.
4. **111 GNPRTB parameters**—traced via `DumpGNPRTBXrefs.py` to map every game balance knob.
5. **C++ class hierarchy**—reconstructed from vtable pointers: `CRebObject` → `CNotifyObject` → `CCombatUnit`/`CCharacter`/`CStarSystem`.

## Limitations

- GhidraMCP (LaurieWired version) caps `/methods` and `/strings` at 99 results. The Jython scripts bypass this.
- Decompiled C is pseudocode—variable names are auto-generated (`iVar1`, `uVar2`). The scholar documents rename them based on context analysis.
- Some functions are too large for single-shot decompilation (e.g., `FUN_004927c0` is 9K lines). These were analyzed in sections.
