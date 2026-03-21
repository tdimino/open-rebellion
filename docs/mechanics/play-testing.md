---
title: "Play-Testing Infrastructure"
description: "Headless playtest binary, JSONL telemetry, REPL for LLM agents, command palette, and balance diagnosis tools"
category: "mechanics"
created: 2026-03-17
updated: 2026-03-20
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
tags: ["play-testing", "logging", "command-palette", "headless", "dual-ai", "deterministic-rng", "repl", "jsonl"]
---

# Play-Testing Infrastructure

Open Rebellion ships a standalone **headless binary** (`rebellion-playtest`) for automated campaign testing, balance tuning, and LLM agent play. It runs the full 15-system simulation without rendering and produces structured JSONL telemetry.

## Quick Start

```bash
# Build prerequisite: PATH prefix avoids ~/.local/bin/cc shadow
PATH="/usr/bin:$PATH" cargo build -p rebellion-playtest

# Run a 3000-tick dual-AI campaign and see the summary
PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base \
  --seed 42 --ticks 3000 --dual-ai --summary

# Stream raw JSONL to stdout (pipe to jq, DuckDB, analysis scripts)
PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base \
  --seed 42 --ticks 3000 --dual-ai --jsonl > campaign.jsonl

# Export to file + print summary
PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base \
  --seed 42 --ticks 3000 --dual-ai --output campaign.jsonl --summary

# Interactive REPL for LLM agent play
PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base \
  --seed 42 --repl --dual-ai
```

## CLI Options

| Argument | Default | Description |
|----------|---------|-------------|
| `<data_dir>` (positional) | — | Path to GData directory containing .DAT files (required) |
| `--ticks <n>` | `5000` | Number of simulation ticks to run |
| `--seed <u64>` | `0` | RNG seed for deterministic replay |
| `--dual-ai` | off | Both factions AI-controlled |
| `--output <path>` | — | Export all events as JSONL to a file |
| `--summary` | off | Print enhanced summary on exit |
| `--jsonl` | off | Stream JSONL events to stdout (incompatible with --repl/--exec) |
| `--exec <cmd>` | — | Execute a single command and exit (`--exec list` to see all) |
| `--repl` | off | Interactive REPL with persistent state and JSON output |
| `--ai-faction` | `empire` | Which faction the AI controls (`empire` or `alliance`) |

## Enhanced Summary (`--summary`)

The summary provides immediate balance diagnosis:

```
=== Playtest Summary ===
Total events: 7384
Final tick: 2999

Events by type:
  ai_action                        3863
  mission_resolved                 2992
  fleet_arrived                     427
  ...

Galaxy control:
  Alliance: 3 systems
    - Boordii
    - Geedon V
    - Yavin
  Empire:   9 systems
    - Averam
    - Coruscant
    - ...
  Neutral:  188 systems

Fleets in transit: 1
  Empire fleet: Balmorra → Averam (40%, 6 ticks left)

Combat diagnostics:
  Fleet attack orders:     2
  Fleet reinforce orders:  853
  Space battles:           5
  Ground battles:          0
  Bombardments:            0
```

## JSONL Telemetry

Every game event is a structured JSON object with human-readable names:

```json
{"tick":7,"wall_ms":4,"system":"ai","event_type":"ai_action","details":{"type":"MoveFleet","faction":"Empire","from":"Coruscant","to":"Averam","reason":"Reinforce"}}
{"tick":17,"wall_ms":5,"system":"combat","event_type":"combat_space","details":{"system":"Averam","winner":"draw"}}
{"tick":47,"wall_ms":8,"system":"missions","event_type":"mission_resolved","details":{"kind":"Diplomacy","outcome":"Success","target_system":"Sullust"}}
{"tick":17,"wall_ms":5,"system":"movement","event_type":"fleet_arrived","details":{"system":"Averam","origin":"Coruscant","fleet_faction":"Empire"}}
```

All system keys are resolved to names. AI actions show faction, origin, destination, and reason.

### Analysis with jq

```bash
# Count events by type
cat campaign.jsonl | jq -s 'group_by(.event_type) | map({type: .[0].event_type, count: length}) | sort_by(-.count)'

# Show all combat events
cat campaign.jsonl | jq 'select(.event_type == "combat_space")'

# Show all fleet movements with attack reason
cat campaign.jsonl | jq 'select(.event_type == "ai_action" and .details.type == "MoveFleet" and .details.reason == "Attack")'

# Fleet arrival destinations
cat campaign.jsonl | jq 'select(.event_type == "fleet_arrived") | .details.system' | sort | uniq -c | sort -rn
```

### Analysis with DuckDB

```sql
CREATE TABLE log AS SELECT * FROM read_json_auto('campaign.jsonl');

-- Combat frequency by system
SELECT details->>'system' as system, COUNT(*) as battles
FROM log WHERE event_type = 'combat_space'
GROUP BY 1 ORDER BY 2 DESC;

-- Fleet movement patterns
SELECT details->>'to' as target, details->>'reason' as reason, COUNT(*) as n
FROM log WHERE event_type = 'ai_action' AND details->>'type' = 'MoveFleet'
GROUP BY 1, 2 ORDER BY 3 DESC;

-- Mission success rates
SELECT details->>'kind' as kind, details->>'outcome' as outcome, COUNT(*) as n
FROM log WHERE event_type = 'mission_resolved'
GROUP BY 1, 2 ORDER BY 1, 2;
```

## REPL Mode (LLM Agent Play)

The `--repl` flag starts an interactive session where world state persists across commands. Each response is JSON.

```bash
# Pipe commands from an LLM agent
echo -e "systems\ntick 50\ntransit\nlist_active_fleets\nevents 3\nquit" | \
  PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base --repl --seed 42 --dual-ai
```

### REPL Commands

| Command | Description |
|---------|-------------|
| `tick N` | Advance N simulation ticks, report new events + victory status |
| `systems` | Show controlled systems by faction (Alliance, Empire, neutral counts + names) |
| `transit` | Show all active fleet movement orders with origin, destination, progress, ETA |
| `events N` | Show last N events from the log with full payloads |
| `show_game_stats` | System/fleet/character counts |
| `list_active_fleets` | Fleet positions and compositions |
| `list_active_missions` | In-progress missions |
| `force_victory_check` | Evaluate victory conditions immediately |
| `reveal_all_systems` | Remove fog of war |
| `export_game_log` | Write message log to file |
| `quit` | Exit the REPL |

### LLM Integration Pattern

Spawn `rebellion-playtest --repl` as a subprocess, send commands via stdin, parse JSON from stdout. The LLM reasons about galactic strategy; the simulation executes the mechanics.

```python
import subprocess, json

proc = subprocess.Popen(
    ["cargo", "run", "-p", "rebellion-playtest", "--", "data/base", "--repl", "--seed", "42"],
    stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True
)
proc.stdin.write("tick 100\n")
proc.stdin.flush()
response = json.loads(proc.stdout.readline())
```

## Command Palette (debug builds only)

Press **backtick** (`` ` ``) in the interactive game to open a VS Code-style fuzzy command palette. Uses nucleo-matcher. Gated behind `#[cfg(debug_assertions)]`.

Shares the same command registry as `--exec` mode (16 commands across Time, Speed, Inspect, and Control categories).

## Deterministic RNG

Both interactive and headless modes use `Xoshiro256++` (rand_xoshiro). All RNG calls flow through a single generator seeded from the `--seed` flag, making runs fully reproducible.

## Balance Tuning Workflow

1. **Run a campaign**: `--seed 42 --ticks 5000 --dual-ai --summary`
2. **Diagnose**: Check combat diagnostics, galaxy control, fleet transit patterns
3. **Drill down**: Use `--jsonl` and pipe to jq/DuckDB for specific event analysis
4. **Modify**: Adjust AI parameters, game constants, or mod JSON patches
5. **Compare**: Re-run with the same seed to see the effect of changes
6. **Iterate**: Use the REPL for interactive exploration of specific game states

For automated balance tuning, see the autoresearch scripts (planned) that apply the Karpathy self-improvement loop: edit config, run headless, measure scalar quality metric, keep/discard.
