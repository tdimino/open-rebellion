---
title: "Play-Testing Infrastructure"
description: "Command palette, headless binary, JSONL logging, and dual-AI mode for automated play-testing"
category: "mechanics"
created: 2026-03-17
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
# Run 1000 ticks with default seed
cargo run -p rebellion-playtest -- --ticks 1000 --data data/base

# Run with a specific seed for reproducibility
cargo run -p rebellion-playtest -- --ticks 5000 --data data/base --seed 12345

# Pipe to file
cargo run -p rebellion-playtest -- --ticks 10000 --data data/base > playtest.jsonl
```

### CLI Options

| Flag | Default | Description |
|------|---------|-------------|
| `--data <path>` | `data/base` | Path to GData directory |
| `--ticks <n>` | `1000` | Number of simulation ticks to run |
| `--seed <u64>` | system time | RNG seed for deterministic replay |

### JSONL Schema

Each line is a JSON object with a `type` field:

```json
{"type":"tick","tick":1,"alliance_systems":85,"empire_systems":115}
{"type":"combat","tick":47,"system":"Coruscant","winner":"empire","attacker_losses":2,"defender_losses":1}
{"type":"mission","tick":102,"kind":"Diplomacy","faction":"alliance","outcome":"success","system":"Sullust"}
{"type":"event","tick":200,"name":"Luke begins Dagobah training"}
{"type":"summary","ticks":1000,"alliance_systems":92,"empire_systems":108,"fleets":24,"characters":42}
```

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
