---
title: "Research: The Mission, the Points Pools and the Unlock Rule"
description: "Research mission 0x53 (records 0x53000020-22, ship/facility/troop design): per-member skill rolls add 1-7 points to the side's pool for the tree, timer 0x381 adds points from production managers, and FUN_00567ba0 spends points on the next research order that has a class; classes unlock at order <= level"
category: "ghidra"
tags: [research, missions, design, tech, unlock]
---

# Research

Answers the research part of task-graph node `or-q-story-missions-z8e`.
Every claim below was read in the `.c` files cited.

## The three trees

| Record | Name | Skill | Side points | Side level | Producer family |
|---|---|---|---|---|---|
| `0x53000020` | Ship Design Research | ship design `+0x5c` (effective slot `+0x1e4`) | `+0x90` | `+0x9c` | `0x28` |
| `0x53000021` | Facility Design Research | facility design `+0x60` (slot `+0x1ec`) | `+0x98` | `+0xa4` | `0x2a` |
| `0x53000022` | Troop Training Research | troop training `+0x5e` (slot `+0x1e8`) | `+0x94` | `+0xa0` | `0x29` |

`FUN_0056c460` picks the tree from the record. Each MISSNSD record repeats
on a timer of 10 + rand 10 days, targets a system its side holds, may be
resigned and takes no special force.

## The mission

- `+0x274` `FUN_0056cb20` returns 0; the roll is inside `+0x27c`.
- `+0x278` `FUN_0056cd70` zeroes the mission's points `+0xa8` and levels
  gained `+0xac` (`FUN_0056c240`, `FUN_0056c2b0`).
- `+0x27c` `FUN_0056cdb0`, per member: it needs the target system
  (`FUN_00586720`) and draws against the member's effective skill for the
  tree (`FUN_005337f0`, `FUN_0055c6f0`, `FUN_0053e2f0`). A success adds
  GNPRTB 6171 + rand(0..=6172) to `+0xa8` (`FUN_0055c9f0`; 1 + rand 6, so
  1 to 7) and raises the base skill by GNPRTB 6158 (1) through slot
  `+0x1d8` (`FUN_00533850`, `FUN_00534990`).
- `+0x280` `FUN_0056cf30` adds `+0xa8` to the side's points for the tree
  (`FUN_00506f30`, `FUN_00530600`) and stores the levels gained in `+0xac`
  (`FUN_0052eac0`). The result is 3 when any points were earned, else 2.

## Passive points

Timer `0x381` (every GNPRTB 7169 + rand 7170 days, `timer-scheduler.md`)
runs `FUN_00530460`. For every system its side holds it finds the
production managers of kinds 0, 2 and 1 (`FUN_00509670`) and adds each
manager's count from `FUN_00528040` (members whose `+0x60` bit 0 is clear)
to `+0x90`, `+0x94` and `+0x98`. A held system missing any of the three
managers stops the addition for that tick.

## Points to levels

A write to a tree's points fires the side vtable's slot `+0x1e0`, `+0x1e4`
or `+0x1e8` (`0x00660cd8`-`0x00660ce0`): `FUN_005326f0` ships,
`FUN_005327f0` troops, `FUN_005328f0` facilities. Each calls
`FUN_00567ba0` through `FUN_0051cb90` with the side (`FUN_00530a20`) and the
producer family:

1. From the current level + 1 up to the highest order present, the first
   order that has a class becomes the next level, and that class's `+0x54`
   is the cost, scaled to cost × side `+0xc0` / 100 (`FUN_005326f0`,
   `FUN_0055bf80` → `FUN_0053e170`).
2. When the points cover the cost, the level is set and the cost deducted
   (`FUN_0052f280`, `FUN_0052f130`). The deduction rewrites the points and
   fires the slot again, so one large pool can buy several levels.
3. With no higher class, `FUN_0052f440` sets side `+0xac` to 1 (ships).

Empty orders are skipped: the level goes straight to the next order that
has a class.

## Unlock rule

`FUN_0052e580(side, family)` reads the tree's level and keeps the classes
whose order `+0x50` is at or below it (`FUN_00567880` with 1, 1, 0), whose
range `+0x24` holds the producer family (`FUN_00567a70`), which pass
`FUN_0053b860`, and whose side flag is set (`+0x40` Alliance, `+0x44`
Empire).

## The computer player's choice

The Research planners' team selectors `FUN_0047bcd0`, `FUN_0047c2b0` and
`FUN_0047c900` (subtypes 1-3) call `FUN_004035d0` with required flags 2,
capability `0x800` (ships), `0x1000` (troops) or `0x2000` (facilities),
forbidden flags `0x7400051`, skill index 2, 3 or 4, and a skill window of
10 to 10000 (`ai-mission-planning.md`).

## Port

`crates/rebellion-core/src/research.rs`, `ai.rs` `evaluate_research` and
`AIAction::DispatchResearch`:

| | Original | Port |
|---|---|---|
| Form | a repeating mission with a team | one character per side and tree on a countdown |
| Roll | per member against the effective skill | none; it always completes |
| Gain | 1-7 points per success into the side's pool | +1 level when the countdown ends |
| Passive points | timer `0x381` | none |
| Cost | the next class's `+0x54` in points, scaled by side `+0xc0` | the highest `research_difficulty` at the next order, as ticks (at least 10); troop and facility trees use 30 |
| Step | the next order with a class, possibly several | +1, capped at 15 |
| Skill raise | GNPRTB 6158 per success | none |
| Unlock | order at or below the level, side flag | the same (`is_unlocked`) |
| AI pick | skill at least 10 | its own rule, floor 30 |

## Open

- What `FUN_00528040` counts (its `+0x60` bit 0) and which manager kind is
  which tree.
- Who sets the cost scale `+0xc0` (`FUN_005305c0`); what `FUN_0053b860`
  tests; whether the exhausted flags `+0xac`-`+0xb4` stop research
  missions.
- The research planners' team size.

## Decompiles

`FUN_0056c460.c`, `FUN_0056cb20.c`, `FUN_0056cd70.c`, `FUN_0056c240.c`,
`FUN_0056c2b0.c`, `FUN_0056cdb0.c`, `FUN_005337f0.c`, `FUN_0055c6f0.c`,
`FUN_0055c9f0.c`, `FUN_00533850.c`, `FUN_00534990.c`, `FUN_0056cf30.c`,
`FUN_00530600.c`, `FUN_0052eac0.c`, `FUN_00530460.c`, `FUN_00509670.c`,
`FUN_00528040.c`, `FUN_005326f0.c`, `FUN_005327f0.c`, `FUN_005328f0.c`, `FUN_0051cb90.c`, `FUN_00530a20.c`,
`FUN_00567ba0.c`, `FUN_0052f280.c`, `FUN_0052f130.c`, `FUN_0052f440.c`,
`FUN_0052e580.c`, `FUN_00567880.c`, `FUN_00567a70.c`, `FUN_0055bf80.c`, `FUN_0053e170.c`, `FUN_0047bcd0.c`, `FUN_0047c2b0.c`,
`FUN_0047c900.c`.
