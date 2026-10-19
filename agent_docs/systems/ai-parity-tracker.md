---
title: "AI Parity Tracker"
description: "Module-by-module map of the original computer player (14 task modules and the strategy controller) to the port's code, with the port heuristics that still stand in for untraced original mechanisms"
category: "agent-docs"
created: 2026-03-21
updated: 2026-10-16
tags: [ai, parity, ghidra, modules, controller, production, fleet-tasks, character-tasks]
---

# AI Parity Tracker

Maps the original REBEXE.EXE computer player to the port. Order-by-order
detail lives in `ghidra/notes/ai-order-emission.md`; this page tracks status.

## Status Key

| Status | Meaning |
|--------|---------|
| **DONE** | Ported from the notes; remaining untraced items are listed in the notes' "Open" sections |
| **PARTIAL** | Ported in part; the gap is named in the row |
| **MISSING** | No port |

Tags in code and notes: `hyp:` is a reading the decompile does not settle,
`port:` is an invention or a simplification still to be replaced.

## Scheduling

The computer player (`FUN_004857a0`) builds 14 task modules for either side
(`FUN_00485a50`: ids 13, 2, 1, 3, 6, 7, 4, 5, 8, 9, 10, 11, 12, 14) into a
tree keyed by id (`FUN_004ec940`, `FUN_005f4f10`), so the scheduler visits
them in ascending id, 1 to 14 (`FUN_004be980`). The
original steps one armed module per main-loop pass, the sides taking turns
(`FUN_0041cf20` -> `FUN_004361a0`, `FUN_00485b40`; "Scheduler and executor" in
`ai-order-emission.md`). The port runs that round (`AISystem::round`,
`ai_scheduler.rs`, `FUN_00485b40`) once a side per in-day step, rate+1 steps
a day by speed (`FUN_0051dea0.c:11-30`: 5 at Fast, 13, 61 and 601), the
Alliance first (`FUN_004361a0`), and applies each round's order at the next
step (`FUN_00436250`). `simulation::ai_step` drives both paths: headless runs a
day's steps after its daily stages, native spreads them over the day's frames
(`GameClock::in_day_step`). The human player's advisor agent takes that side's
slot (`FUN_00486500`, `FUN_00439a10`). Message `0x1f0` is handled by `FUN_004927c0`, a session
handler, not the computer player's turn (`ghidra/notes/timer-scheduler.md`,
"`FUN_004927c0` and message `0x1f0`").
`FUN_00519d00` is new-game stage `0xb`, which splits the created systems into
side lists, and runs once at setup (`ghidra/notes/ai-behavior-analysis.md`,
section 1).

## Modules

| Id | Purpose | Original | Port | Status |
|---|---|---|---|---|
| 13 | Alliance HQ move, mode pick, strategy controller | `FUN_004bf320`, `FUN_004bf490`, `FUN_004e5440`, `FUN_004bf9e0`, `FUN_004c05e0` | `ai_strategy.rs` (`cycle`) | PARTIAL: the HQ move (`FUN_004bf3e0`) is missing (`or-alliance-hq-move-o9ms`); mode inputs A and B are untraced; port: one module-13 cycle per round instead of eight scheduler calls |
| 8 | Regional records and postures | `FUN_004cf600`, `FUN_004774c0`, `FUN_004adb70` | `ai_regions.rs` | PARTIAL: the Create Fleet target queries (`FUN_004782a0`, `FUN_00478690`, `FUN_00478aa0`) are untraced |
| 9 | Fleet tasks, one per headquarters system | `FUN_004cf290`, `FUN_00474070` | `fleet_tasks.rs` | DONE |
| 12 | Character tasks: class G (General, Emperor), class I (Admiral, General, Commander) | `FUN_004c84a0`, `FUN_004bbb00`, `FUN_004bc480` | `character_tasks.rs` | PARTIAL: hyp: a kind 1 task takes the first free tagged system in id order (`FUN_0047a870` values untraced) |
| 4 | Mission planners: team, decoys, Mission, Move, Abort | `FUN_004bced0`, `FUN_0047b360`, `FUN_004bd570`, `FUN_004be230`, `FUN_0047b250` | `mission_planning.rs`, `AISystem::dispatch_planned` | PARTIAL: side `+0x318`/`+0x31c` (which mission, which target) is untraced, so `ai.rs` picks them (below); the planner does not read the regional record's `+0xbc` (posture slot 6, `ai_regions.rs`); Abort has no port |
| 11 | Capital ships | `FUN_004c8b30`, `FUN_004ca5e0`, `FUN_004cba90` | `ai_production.rs` (`Module::CapitalShips`) | DONE |
| 10 | Fighters for fleets | `FUN_004cc9c0`, `FUN_004cd040` | `ai_production.rs` (`Module::FleetFighters`) | DONE |
| 7 | Fighters for systems | `FUN_004d3100`, `FUN_004d4370` | `ai_production.rs` (`Module::SystemFighters`) | DONE: why it scraps fighters when no source is found is open |
| 6 | Regiments | `FUN_004dd7a0`, `FUN_004df440` | `ai_production.rs` (`Module::Regiments`) | DONE |
| 5 | Special forces | `FUN_004cfed0`, `FUN_004d0480`, `FUN_004d0610` | `ai_production.rs` (`Module::SpecialForces`) | DONE: the Retire arrives as a Scrap; the sort key of its pick is open |
| 3 | Defences, mines, refineries | `FUN_004d6d30`, `FUN_004d76f0`, `FUN_004d8520` | `ai_production.rs` (`Module::Defences`) | DONE: the Coruscant case is open |
| 1 | Construction yards, mines, refineries | `FUN_004dabe0`, `FUN_004dbd60` | `ai_production.rs` (`Module::Yards`) | DONE |
| 2 | Deficit relief | `FUN_004e2120`, `FUN_004e2940` | `ai_production.rs` (`Module::DeficitRelief`) | DONE |
| 14 | Rename: names for up to 10 capital ships per pass | `FUN_004d29b0`, `FUN_004d2b30` (`fleet-names.md`) | none | MISSING |

Budgets, class picks, goal bits and the per-module census are in
`ghidra/notes/ai-production.md` and the "Production" section of
`ai-order-emission.md`.

## Orders

The "Port" section of `ghidra/notes/ai-order-emission.md` maps each original
order to its `AIAction`. Orders with no port: Move of the HQ, Rename `0x203`,
Abort `0x250`.

## Port heuristics without an original counterpart

These run as module 4's pseudo-planners after the Research planners, one
pass a turn (`Pseudo`, `ai_research.rs`), and are `port:` until the original
selection is traced and ported.

| Function | What it does now | Original mechanism it stands in for |
|---|---|---|
| `evaluate_officers` | The targets the original's queries take over the round's system records (`FUN_00433e40` `sys+0x2c` bits) and the creation validator admits: Incite at our most popular enemy system with our support above 50 (bit `0x100`); Diplomacy at our systems below support 70, then neutral ones (bit `0x40`), least popular first, skipping a system our Diplomacy works; Recruitment at our most popular calm, unblockaded system while its recruit pool holds anyone. The planner picks the members | Side `+0x318`/`+0x31c`, the strategic choice of mission and target among the candidates (untraced) |
| `ResearchPlanners::turn` (`ai_research.rs`) | Persistent planners per tree, each holding one scientist and re-emitting the Research order at the member's own system with no yard test, skipped while its mission is in flight; module 13 requests an add while the tree is open and the count is below the capable characters, or a drop on the exhausted bit or an excess count, and module 4's next turn consumes it | The controller's Research requests and the planner's states (`FUN_004e8290`, `FUN_004e5710`, `FUN_004d2580`, `FUN_004bced0`, `FUN_004bda40`; `ghidra/notes/research.md`); the drop's choice among planners is hyp |
| `evaluate_espionage` | Sabotage at enemy systems with the most manufacturing facilities, Assassination then Abduction of enemy characters, Espionage at explored systems the side or its enemy holds (`sys+0x2c` bit `0x200`), the enemy's first, galaxy-wide (`FUN_004b1bf0` asks within a regional record's sector) | Side `+0x318`/`+0x31c` |
| `evaluate_rescue` | Rescue missions for characters the enemy holds captive; the Rescue planner (`FUN_0047e270`, `FUN_0047e3f0`) picks each team | Side `+0x318`/`+0x31c` |
| `evaluate_reconnaissance` | Espionage missions at explored enemy systems, ranked by `system_strength` | None traced; `system_strength` is a port score of ship hulls, regiments and defence facilities and cites no source |
| `evaluate_uprising_prevention` | Diplomacy to the two systems with own popularity below 0.4 | None traced |

## Dispatch validators

The original's checks before an order (`FUN_00508250`, `FUN_00508660`, the
object checks per family) are in `ghidra/notes/ai-behavior-analysis.md`,
sections 3, 4 and 6. `FUN_0050b8e0` writes the system's battle, blockade and
change bits and is covered in `ghidra/notes/blockade-bit.md`.

## Behavioral properties

| Property | Original | Port |
|---|---|---|
| Map visibility | No fog parameter in the traced AI functions | PORT DIFFERENCE, open (br `or-q-port-observation-levels-jgy`, status open): whether the port's fog should follow the original's per-side observation levels (informants, espionage, mission ends, events, new-game reveals; `ghidra/notes/observation-levels.md`) and how a level maps onto `fog.rs` |
| Cadence | One armed module per main-loop pass, sides alternating; rate+1 rounds a day | The same rounds, a whole round per call; at most `day_orders` (10) orders a side a day |
| Side differences | The Death Star request (side 2), the shield flag, class side flags, module 13 (Alliance only); no traced production function reads a difficulty or GNPRTB value | The same modules for both sides; the HQ move is missing |

## Augmentations

`AiConfig` (`tuning.rs`, `--config`) holds the AI's tempo, defaulting to the
original's values so it can be calibrated:

| Field | Default | Reader |
|---|---|---|
| `day_orders` | 10 (`+0x428`, `FUN_00485a50.c:35`) | `ai_scheduler.rs` `round` |
| `rounds_per_day` | 601 / 61 / 13 / 5 by speed (`FUN_0051dea0.c:11-30`) | `simulation.rs` `day_rate` |

Each module-4 pseudo-planner turn sends at most one mission
(`FUN_004d2490`, `FUN_00485b40`).

## Validation process

1. Run a headless campaign:
   `scripts/mini-cargo.sh -- run --release -q -p rebellion-playtest --color never -- data/base --seed 42 --ticks 400 --dual-ai --summary`
2. Add `--output campaign.jsonl` and score the log:
   `python3 scripts/eval_game_quality.py campaign.jsonl`
3. Confirm a non-degenerate run: score above 0.0 and battles at 3 or more systems.
4. Run both sides: every change is checked for Empire and Alliance alike.
5. After simulation changes, run `replay_manifest --ignored` and
   `scripts/mini-cargo.sh -- run -q -p provenance-scan -- check`.

## Sources

- `ghidra/notes/ai-order-emission.md`, `ai-production.md`, `ai-mission-planning.md`, `research.md`, `timer-scheduler.md`, `ai-behavior-analysis.md`, `blockade-bit.md`
- `crates/rebellion-core/src/`: `ai.rs`, `ai_strategy.rs`, `ai_regions.rs`, `fleet_tasks.rs`, `character_tasks.rs`, `ai_production.rs`, `ai_records.rs`, `mission_planning.rs`, `tuning.rs`
