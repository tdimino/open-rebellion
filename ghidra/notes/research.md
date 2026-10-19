---
title: "Research: The Mission, the Points Pools and the Unlock Rule"
description: "Research mission 0x53 (records 0x53000020-22, ship/facility/troop design): per-member skill rolls add 1-7 points to the side's pool for the tree, timer 0x381 adds points from production managers, and FUN_00567ba0 spends points on the next research order that has a class; classes unlock at order <= level"
category: "ghidra"
updated: 2026-10-16
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
runs `FUN_00530460`. For every system whose side bits (`+0x24 & 0xc0`) are
the side's it finds the production managers of kinds 0, 2 and 1
(`FUN_00509670`) and adds each manager's count from `FUN_00528040` to
`+0x90` (ships), `+0x94` (troops) and `+0x98` (facilities). The same kind
mapping is in `FUN_0056c460` and the manufacturing overview.

`FUN_00528040` counts the yards passing `FUN_0052bde0`: the yard's kind
(vtable `+0x204`) is the manager's, yard `+0x50` bit 0 is set, the sides
match, and yard `+0x60` bit 0 is clear. `FUN_0050bbb0` is the only writer of
`+0x60` bit 0: it holds `(system +0x88 & 0x24) != 0`, the system being in
uprising (bit 2) or blockaded (bit 5). So each tick every active yard in a
held system that is neither in uprising nor blockaded gives one point:
shipyards to ships, training facilities to troops, construction yards to
facilities. A held system missing any of the three managers cancels the
whole tick for all three trees. hyp: every system has all three managers;
a yard still under construction counts (only bit 0 was read).

## Points to levels

A write to a tree's points fires the side vtable's slot `+0x1e0`, `+0x1e4`
or `+0x1e8` (`0x00660cd8`-`0x00660ce0`): `FUN_005326f0` ships,
`FUN_005327f0` troops, `FUN_005328f0` facilities. Each calls
`FUN_00567ba0` through `FUN_0051cb90` with the side (`FUN_00530a20`) and the
producer family:

1. From the current level + 1 up to the highest order present, the first
   order that has a class becomes the next level, and that class's `+0x54`
   is the cost, scaled to cost × side `+0xc0` / 100, truncated
   (`FUN_005326f0`, `FUN_0055bf80` → `FUN_0053e190` → `FUN_0053e170` →
   `FUN_0053e150`, divisor `DAT_00661a88` = 100). Class `+0x54` is
   `research_difficulty` (DAT offset `0x2c`). Side `+0xc0` has one writer,
   `FUN_005305c0`, called only from the end of `FUN_00519d00` with the number
   of systems created: 100, 150 or 200 by galaxy size
   (`galaxy-limits.md`). hyp: a save restores it, with no other writer.
2. When the points cover the cost, the level is set and the cost deducted
   (`FUN_0052f280`, `FUN_0052f130`). The deduction rewrites the points and
   fires the slot again, so one large pool can buy several levels.
3. With no higher class, `FUN_0052f440` sets side `+0xac` (ships), `+0xb0`
   (troops) or `+0xb4` (facilities) to 1; nothing clears it. Its only reader
   is `FUN_0052eb10`, from the Research validator `FUN_0056c510`, which
   refuses a new mission (`0x40`/`0x32`) and ends a running one with code
   `0xe`.

Empty orders are skipped: the level goes straight to the next order that
has a class.

## Unlock rule

`FUN_0052e580(side, family)` reads the tree's level and keeps the classes
whose order `+0x50` is at or below it (`FUN_00567880` with 1, 1, 0), whose
range `+0x24` holds the producer family (`FUN_00567a70`), which pass
`FUN_0053b860`, and whose side flag is set (`+0x40` Alliance, `+0x44`
Empire).

## Validator

`FUN_0056c510` checks a Research mission at creation and on every pass
(the member-skill test only at creation, `FUN_0056c820`: a member whose
effective skill is 0 is refused with `0x40`/4). The target must be a system
(families `0x90`-`0x97`, else `0x40`/`0x24`), pass the base system rules,
hold a yard of the tree's kind that `FUN_00528100` accepts (same kind, same
side, `+0x60` bit 0 clear; else `0x40`/`0x2c`, end code `0xc`), and the
side's tree must not be exhausted (`0x40`/`0x32`, end code `0xe`). The
player's dialog builds its checker with `+4`, `+8` and `+0xc` set
(`FUN_0054c590`). MISSNSD records 11-13 are not hidden, admit both sides,
take no special force, target only the side's own systems, allow uprising
and calm targets, and need no populated container; a yard in uprising or
blockade fails the yard test, so those systems are not valid targets.

## The roll

`FUN_0056cdb0` runs once per member: it succeeds when rand(0..99)
(`FUN_0053e2e0`, `DAT_00661a88` = 100) is below the member's effective skill
for the tree (`FUN_0053e2f0`; `FUN_005337f0` slot `+0x1e4` ships, `+0x1ec`
facilities, `+0x1e8` troops). No GNPRTB value enters the roll.

## The computer player's choice

The Research planners' team selectors `FUN_0047bcd0`, `FUN_0047c2b0` and
`FUN_0047c900` (subtypes 1-3) run while the team is empty and stop after
one member, so a team is one character with no decoys. They call
`FUN_004035d0` with required flag 2 (the side's own records), capability
`0x800` (ships), `0x1000` (troops) or `0x2000` (facilities), forbidden
flags `0x7400051` (unrecruited, injured, prisoner, on a mandatory mission,
an officer rank), skill index 2, 3 or 4 and a skill window of 10 to 10000,
keeping only characters no planner holds (`0x800000` clear). Candidates are
ordered by ascending skill with random ties (`FUN_0041c250`) and popped
from the top (`FUN_004357b0`); the first passing `FUN_0047ba90` is taken.
The target comes from `FUN_004bd0a0`: the planner's `+0x34` and `+0x38`
start empty (`FUN_0047aae0`), so it is the member's current system, which
must be the side's (record bit 0) and not blockaded (`+0x28` bit 1). hyp:
the planner does not look for a yard; the validator refuses the order when
there is none.

The controller's request picker `FUN_004e8290` asks for a Research planner
of subtype 1, 2 or 3 while the tree's controller bit is clear (`0x2000000`
ships, `0x4000000` troops, `0x8000000` facilities, set by `FUN_004198a0`
and its siblings when the level reaches the highest order;
`ai-mission-planning.md`) and the live planner count (context `+0x324`,
`+0x328`, `+0x32c`) is below its limit (`+0x2cc`, `+0x2d0`, `+0x2c8`). Each
limit is the number of the side's characters with that capability
(`FUN_0041bc10`). It asks to drop one when the count exceeds the limit, or
the tree is done and the count is above 0; `FUN_004e5710` then removes the
least advanced planner. Requests go in priority order, Recruitment, then
Research 1, 2, 3, then Diplomacy and the rest, one primary request per
controller cycle. The AI does not read the exhausted flags.

### The planner after its order

A planner is a persistent object in the side's planner list (side `+0xa8`). Its
state machine `FUN_004bced0` returns a finished flag after every pass and
restarts at state 1; the planner stays in the list with its member claimed
(record `+0x24` is the planner's id, record `+0x30` bit `0x800000` clear).

- State 8 `FUN_0047b360` builds Create Mission `0x240` on every pass. It copies
  into the order's team only members whose record `+0x30` has none of
  `0x74000f1`, so a member on the mission (`0x80`) or in transit (`0x20`)
  leaves the team empty. hyp: validity (`FUN_0051ff30`) refuses such an order
  and the module's failure slot `FUN_004dd810` clears its step, phase and
  order counter and deletes the order.
- State 2 `FUN_0047b110` keeps a member for which `FUN_0047ba40` holds: record
  present, none of `0x7400051` (unrecruited, injured, prisoner, on a mandatory
  mission, an officer rank), claimed by this planner, `FUN_0047ba90` (the
  member is not in transit and its system record has `+0x28` bit 2 clear), and
  the planner's own predicate (slot `+0x3c`). A running research mission
  (`0x80` only) leaves the member in the team. The selector (state 3) runs only
  while the team is empty.
- Retirement has one source in the corpus: the controller's drop request
  (`FUN_004e5710` stores the planner id in side `+0x314`; `FUN_004d2580` calls
  `FUN_0042f690`, which sets planner flag `0x10000000`; state 1 then runs
  `FUN_0047aff0`, which releases the team and decoys with `FUN_0047b870`,
  `FUN_0047b8e0`, calls `FUN_0041b530` and slot `+0x38`). The request fires
  when the live count exceeds the limit, or the tree's controller bit is set
  and the count is above 0 (`FUN_004e8290`).
- Planner flags `0x20000000` and `0x40000000` make state 1 run `FUN_0047b250`,
  which sends Abort `0x250` for the planner's mission while the mission still
  exists. No function in the corpus sets either flag. hyp: a mission event
  handler sets them.

So a research planner ends when its tree is finished or the side has more
planners than capable characters. The order going out ends nothing, and the
mission's own end (validator codes `0xc` no yard, `0xe` exhausted) frees the
member for the next pass.

### What moves a scientist

- The research planner's target is the member's own system: `FUN_004bd0a0`
  with planner flag 1 clear takes the member's container (`+0x38`), walks up to
  the system (`FUN_004f2d10`, slot `+0x1c`), and accepts it when the system
  record is the side's (`+0x30` bit 1) and `+0x28` bit 2 is clear. State 7
  `FUN_004bd570` therefore issues no Move for a research planner, and no step
  tests for a yard.
- Command tasks (module 12, `ai-order-emission.md`, "Character tasks") move
  officers, not scientists as such. Kinds 1, 3, 5 exclude a reserved character
  (`0x4f4000f1` contains `0x8000000`). Kind 4 (General, mask `0x474000f1`)
  accepts a reserved character whose class may be a general (`+0x34` bit
  `0x4000000`) and Moves it to the posture's staging system or onto the
  flagship, after which Command General gives it a rank and the research
  planner's mask drops it. The staging system (`FUN_004adde0`) is the kept
  anchor, else in the sector a system the side holds with `+0x28` bit 1 and bit
  0 clear and, in order, `+0x28` `0x80000000` (headquarters or Coruscant), then
  any of `0x3e00000` (a listed role system, which includes the shipyard role),
  then any, then random. The tests are role bits; no yard count enters.
- Posture Moves (`FUN_004b06b0`) and the fleet task's character step
  (`FUN_00474ff0`) move characters aboard fleets. hyp: a scientist at a system
  is not aboard a fleet.
- No other function in the corpus builds a Move for a character from a yard
  test: the callers of `FUN_0052cab0` (shipyards at a system) outside the
  production modules are `FUN_004ae480` (enemy facilities as posture targets),
  `FUN_004bfe30` (the Alliance headquarters planner's Scrap), the player's
  Manage automation `FUN_004c6d90`, `FUN_004c7860` and battle export
  `FUN_0040b0e0`.
## Port

`crates/rebellion-core/src/research.rs` (points, passive timer, spending,
unlock), `missions.rs` (`MissionKind::Research`, the roll, the validator),
`mission_planning.rs` (the team selectors) and `ai_research.rs` (the
planners and the controller's requests):

| | Port |
|---|---|
| Form | `MissionKind::Research(tech)` on the repeating mission machinery, 10 + rand 10 days |
| Roll | per member, rand(0..99) below the effective skill; a success adds GNPRTB 6171 + rand 6172 points and raises the skill by GNPRTB 6158 |
| Passive points | timer `0x381` every GNPRTB 7169 + rand 7170 days, one point per working yard in a held system; hyp: armed on the first day |
| Cost | the class at the next order with a class, `research_difficulty` × the galaxy's system count / 100; hyp: the lowest class id at that order is priced |
| Exhausted | `GameWorld::research_exhausted`, set when no higher class exists; the validator ends a mission with `0xe` |
| Validator | a working yard of the tree's kind at the target (else `0xc`); a member with skill 0 is refused at creation |
| AI pick | one member, capability `0x800`/`0x1000`/`0x2000`, skill 10 or more, not held by a planner or task; the target is the member's own system, held and not blockaded |
| AI planners | `ResearchPlanners` in the saved AI state, stepped one a module-4 turn from a cursor: a planner holds one member and emits the order with no yard test of its own; `research_target_refusal` (`FUN_0056c510`) refuses it before a mission exists; a planner whose member is on the mission does not run (`FUN_004bda40`); a member in transit goes out as an empty team, so no action; a member with a released flag (`0x7400051`) leaves the planner |
| AI requests | ships, troops, facilities: add a planner while the tree's exhausted bit is clear and the planner count is below the side's capable characters; drop one when the count exceeds them, or the tree is done and the count is above 0; module 13's turn makes at most one add and one drop (`FUN_004e5710`), which module 4's next turn consumes; hyp: the drop takes the oldest unflagged planner of the tree (its slot `+0x18` key is untraced) |

## Open

- The setters of planner flags `0x20000000` and `0x40000000`.
- The value planner slot `+0x18` returns, which `FUN_004e5710` minimises to
  choose the planner to drop.
- The yard test at creation for orders other than the AI's. The AI's planners
  call `missions::research_target_refusal` (the test `running_end_code` shares)
  before emitting, so a refused order creates no mission; `dispatch_guarded`
  itself has no target input, and `dev_actions` and the native player path
  reach the validator only through it. In the headless seed-42 run the
  Alliance's scientists all stand at Yavin, which holds no yard, so each
  planner's order is refused every day and no Alliance Research mission
  exists; the side's points come from the passive timer alone.

## Decompiles

`FUN_0056c460.c`, `FUN_0056cb20.c`, `FUN_0056cd70.c`, `FUN_0056c240.c`,
`FUN_0056c2b0.c`, `FUN_0056cdb0.c`, `FUN_005337f0.c`, `FUN_0055c6f0.c`,
`FUN_0055c9f0.c`, `FUN_00533850.c`, `FUN_00534990.c`, `FUN_0056cf30.c`,
`FUN_00530600.c`, `FUN_0052eac0.c`, `FUN_00530460.c`, `FUN_00509670.c`,
`FUN_00528040.c`, `FUN_005326f0.c`, `FUN_005327f0.c`, `FUN_005328f0.c`, `FUN_0051cb90.c`, `FUN_00530a20.c`,
`FUN_00567ba0.c`, `FUN_0052f280.c`, `FUN_0052f130.c`, `FUN_0052f440.c`,
`FUN_0052e580.c`, `FUN_00567880.c`, `FUN_00567a70.c`, `FUN_0055bf80.c`, `FUN_0053e170.c`, `FUN_0047bcd0.c`, `FUN_0047c2b0.c`,
`FUN_0047c900.c`, `FUN_004e8290.c`, `FUN_004bced0.c`, `FUN_0047b110.c`, `FUN_0047b250.c`, `FUN_0047aff0.c`, `FUN_0047b360.c`, `FUN_0047ba40.c`, `FUN_0047ba90.c`, `FUN_004d2490.c`, `FUN_004d2580.c`, `FUN_004e5710.c`, `FUN_0042f690.c`, `FUN_004adde0.c`, `FUN_004dd810.c`, `FUN_0052eb10.c`, `FUN_0053a7f0.c`, `FUN_0041b570.c`.
