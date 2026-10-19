---
title: "Research System"
description: "Per-side points pools for the ship, troop and facility trees, fed by the passive timer and Research missions and spent into levels"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-16
tags: [research, tech-tree, missions, simulation]
---

# Research System

`crates/rebellion-core/src/research.rs`. The original's mechanism and its
sources are in `ghidra/notes/research.md`; this page maps it onto the code.

## Types

| Type | Holds |
|------|-------|
| `ResearchState` | `alliance` and `empire` `ResearchLevels` |
| `ResearchLevels` | `ship`, `troop`, `facility` levels, `points: [u32; 3]`, `passive_due` (timer `0x381`) |
| `TechType` | `Ship`, `Troop`, `Facility`; `skill()`, `producer_family()` |
| `ResearchResult` | `TechUnlocked { faction_is_alliance, tech_type, new_level }` |
| `GameWorld::research_exhausted` | per side and tree, set when no higher class exists |

## Flow

- `ResearchSystem::advance` runs timer `0x381` per side: every working
  yard in a held system adds one point to its tree.
- A Research mission (`MissionKind::Research(tech)`, `missions.rs`) rolls
  per member and returns `MissionEffect::ResearchPoints`; the integrator
  credits them with `ResearchSystem::apply_mission_points`.
- `add_points` spends the pool: while the next order with a class costs no
  more than the points, the level moves to it. The cost is
  `research_difficulty` × the galaxy's system count / 100.
- `is_unlocked`, `ship_class_is_available` and
  `fighter_class_is_available` gate production on the levels.

## AI

`ai_research.rs` keeps each side's Research planners in `AIState` (saved). A
planner claims one scientist (`mission_planning.rs` selectors) and, at its
module-4 turn, targets the member's own system, held and not blockaded, with
no yard test of its own; it does not run while its mission flies
(`FUN_004bda40`). The order then meets the creation-time refusal of
`FUN_0056c510` (`research_target_refusal`, `missions.rs`): a target with no
working yard of the tree (status `0x40`/`0x2c`, end code `0xc`) or an
exhausted tree (`0x40`/`0x32`, end code `0xe`) sends no order, and the planner
keeps its claim. The same function ends a running mission through
`running_end_code`. A planner retires only on the controller's drop request
(`FUN_004e8290`): the tree's exhausted bit with a live planner, or more
planners than capable characters.
