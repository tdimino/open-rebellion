---
title: "Play-Testing Infrastructure"
description: "Command palette, headless binary, JSONL logging, and dual-AI mode for automated play-testing"
category: "mechanics"
created: 2026-03-17
updated: 2026-03-18
game_system: "play-testing"
sources:
  - type: "code"
    file: "crates/rebellion-core/src/game_events.rs"
  - type: "code"
    file: "crates/rebellion-data/src/simulation.rs"
  - type: "code"
    file: "crates/rebellion-playtest/src/main.rs"
  - type: "code"
    file: "crates/rebellion-render/src/panels/command_palette.rs"
related:
  - "space-combat"
  - "missions"
  - "victory"
  - "story-events"
tags: ["play-testing", "logging", "command-palette", "headless", "dual-ai", "deterministic-rng"]
---

# Play-Testing Infrastructure

Open Rebellion includes two complementary tools for play-testing the simulation: an in-game **command palette** for interactive exploration and a **headless binary** for automated batch runs.

## Command Palette (debug builds only)

Press **backtick** (`` ` ``) to open a VS Code-style command palette overlay. Type to fuzzy-search commands, use arrow keys to navigate, Enter to execute. Escape or click the backdrop to close.

### Commands

| Command | Category | Description |
|---------|----------|-------------|
| Advance 1/10/100/1000 ticks | Time | Step simulation forward immediately |
| Set Speed: Paused/Normal/Fast/Faster | Speed | Change game clock speed |
| Show Game Stats | Inspect | Print tick, system ownership, fleet/character counts to message log |
| List Active Missions | Inspect | Display in-progress missions |
| List Active Fleets | Inspect | Show fleet positions and compositions |
| Show Event Count | Inspect | Number of triggered events |
| Toggle Dual AI | Control | Enable/disable AI for both factions |
| Force Victory Check | Control | Immediately evaluate victory conditions |
| Reveal All Systems | Control | Remove fog of war from all systems |
| Export Game Log | Control | Write message log to file |

### Prefix Modes

Commands support category-scoped filtering:
- Type `time` to see only time control commands
- Type `speed` to see speed commands
- Type `inspect` to see inspection commands
- Type `control` to see control commands

The palette uses nucleo-matcher for fuzzy matching, so partial and out-of-order queries work (e.g., `adv 100` matches "Advance 100 ticks").

The command palette is gated behind `#[cfg(debug_assertions)]` and does not exist in release builds.

## Headless Binary (`rebellion-playtest`)

A standalone binary that runs the simulation without rendering. Produces JSONL output for post-hoc analysis.

```bash
# Run 5000 ticks with default seed (data_dir is positional)
cargo run -p rebellion-playtest -- data/base

# Run with a specific seed for reproducibility
cargo run -p rebellion-playtest -- data/base --ticks 5000 --seed 12345

# Export to file
cargo run -p rebellion-playtest -- data/base --output playtest.jsonl --summary
```

### CLI Options

| Argument | Default | Description |
|----------|---------|-------------|
| `<data_dir>` (positional) | — | Path to GData directory (required) |
| `--ticks <n>` | `5000` | Number of simulation ticks to run |
| `--seed <u64>` | system time | RNG seed for deterministic replay |
| `--output <path>` | — | JSONL output file path |
| `--summary` | off | Print event count summary on exit |
| `--dual-ai` | off | Both factions AI-controlled |
| `--exec <cmd>` | — | Execute a single command and exit (`--exec list` for options) |
| `--repl` | off | Interactive REPL: persistent state, JSON output, LLM-playable |

### REPL Mode (LLM Agent Play)

The `--repl` flag starts an interactive session where the world state persists across commands. Each command produces JSON output suitable for LLM parsing.

```bash
# Pipe commands from an LLM agent
echo -e "show_game_stats\ntick 10\nlist_active_fleets\nforce_victory_check" | \
  cargo run -p rebellion-playtest -- data/base --repl --seed 42

# Or run interactively
cargo run -p rebellion-playtest -- data/base --repl --seed 42
```

**Output format:**
```json
{"command":"show_game_stats","tick":0,"result":"Stats: 200 systems (1 Alliance, 1 Empire), 3 fleets, 60 characters"}
{"tick":10,"advanced":10,"new_events":142}
{"command":"list_active_fleets","tick":10,"result":"Empire fleet at Coruscant — 7 ships\nAlliance fleet at Yavin — 1 ships"}
```

**Special commands in REPL mode:**
- `tick N` — advance N simulation ticks, report new events + victory status
- `help` — list all available commands
- `quit` — exit the REPL

**LLM integration pattern:** spawn `rebellion-playtest --repl` as a subprocess, send commands via stdin, parse JSON from stdout. The LLM reasons about galactic strategy; the simulation executes the mechanics.

### CLI Exec Mode

Execute a single command against freshly loaded game data and exit:

```bash
# List available commands
cargo run -p rebellion-playtest -- data/base --exec list

# Check initial state
cargo run -p rebellion-playtest -- data/base --exec show_game_stats

# Advance 100 ticks and see results
cargo run -p rebellion-playtest -- data/base --exec advance_100_ticks --seed 42
```

### JSONL Schema

Each line is a `GameEventRecord` (defined in `rebellion-core/src/game_events.rs`):

```json
{"tick":47,"wall_ms":1042,"system":"combat","event_type":"combat_space","details":{"system_name":"Coruscant","attacker":"Empire","defender":"Alliance","winner":"attacker_wins"}}
{"tick":102,"wall_ms":2100,"system":"missions","event_type":"mission_resolved","details":{"kind":"Diplomacy","faction":"Alliance","outcome":"success","system":"Sullust"}}
{"tick":200,"wall_ms":4200,"system":"story","event_type":"story_event","details":{"name":"Luke begins Dagobah training","event_id":"0x221"}}
```

24 event types across 15 systems: `manufacturing_complete`, `fleet_arrived`, `fleet_departed`, `combat_space`, `combat_ground`, `bombardment`, `mission_dispatched`, `mission_resolved`, `event_fired`, `ai_action`, `blockade_started`, `blockade_ended`, `uprising_incident`, `uprising_began`, `death_star_construction`, `death_star_fired`, `research_unlocked`, `jedi_tier_advanced`, `jedi_discovered`, `victory_check`, `betrayal`, `character_escaped`, `story_event`, `character_captured`.

### DuckDB Analysis Examples

```sql
-- Load the playtest log
CREATE TABLE log AS SELECT * FROM read_json_auto('playtest.jsonl');

-- System ownership over time
SELECT tick, alliance_systems, empire_systems
FROM log WHERE type = 'tick'
ORDER BY tick;

-- Combat frequency by system
SELECT system, COUNT(*) as battles
FROM log WHERE type = 'combat'
GROUP BY system ORDER BY battles DESC LIMIT 10;

-- Mission success rates by type
SELECT kind, outcome, COUNT(*) as n
FROM log WHERE type = 'mission'
GROUP BY kind, outcome ORDER BY kind, outcome;
```

## Dual-AI Mode

When dual-AI is enabled (via the command palette "Toggle Dual AI" command), both Alliance and Empire are controlled by the AI system. This allows fully automated games for balance testing.

Combined with the headless binary and a fixed seed, dual-AI mode enables deterministic batch comparison of balance changes across mod configurations.

## Deterministic RNG

Both the interactive game and the headless binary use `Xoshiro256++` (rand_xoshiro crate) seeded from either system time or a user-provided seed. All RNG calls in the simulation loop flow through this single generator, making runs reproducible when given the same seed.
