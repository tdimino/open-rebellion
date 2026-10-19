---
title: "Original Research, the Strategic AI and the Open Items"
description: "Replace the invented research countdown with the original Research missions and points pools, port the original strategic AI (mode selection, controller, regional records, postures, class I tasks, production modules), and close the open items before one large commit"
type: feat
status: completed
progress: "100% — done: timer fix, class G tasks, Phase B (research missions, save v43), Phase C5 production modules with fleet destinations (save v44), the C2-C4 specs and implementation design, Phase D or-pfv4 (save v45, golden, native checks on both sides), the Personnel icon layers, the story overlay removed, C2 (census, ship tags, module 13 modes and plans, module 6 regiment targets), C3a regional records (Create Fleet) and C3b postures (Join Fleet, Moves, Assault, Bombard, Destroy System; old fleet heuristics removed), C4a fleet tasks, C4b class I officer tasks (MoveCharacterToFleet), the cadence (one AI round per side per day), save v46, the specs folded into ai-order-emission.md, provenance citations (baseline shrunk), module 11 reads the regional records (FUN_004ca5e0), golden v1:5499c2812292dc6a -> v1:eb68a4f5e043edac (causes: the original strategic AI, save v46; the original control rule; the Death Star class node), the Opus docs audit and code review fixes, or-pfrm (a system without regiments goes to the side whose support reaches GNPRTB 7760, FUN_0055a080; playtest control 8/8/84, battles and bombardments return; native holders steady on both sides); native: the Alliance AI gathers its fleets at Vagran by day 40, the Empire keeps its fleets in the Coruscant HQ task; the Death Star class fix (FUN_004739b0 tested a family the seeded id never carries, so the Empire HQ fleet task held every warship; now the Empire fleets move, 7 battles in 1000 days); the blockade rule before the AI round (event 0x311 settles in phase 5; telemetry blockade 30), or-glv7 closed; the original Empire AI (VM, alliance-day54-advice.sav) makes its first fleet attack on day 493: Fleet 7 (the Star Destroyer Tyrant, two TIE Fighter squadrons) on Alliance-held Praesitlyn, met by Fleet 2 (one Bulk Cruiser); or-lzz3 answered (the AI sends damaged ships to the nearest own shipyard, FUN_004c8e90); native e2e after the fix: the Empire holds its fleets at Coruscant through day 120 and builds (as the original); Phase G: 15 tools built and checked live, the Empire research native check passed (Lemelisk, Facility level 1 on day 718), Opus review fixes (16) done; Phase H: module order, damage counters and repair (save v49), persistent research planners with the creation-time refusal (save v47) done, load determinism (save v48, the slotmap free order restored) done; golden v1:7e5c233c69c86cba -> v1:4c13a5dc36e88e1d for the four Phase H causes; the clock and policy fixes checked live (Alliance); native checks: production with destinations on both sides, rank lapse on both sides, blue rectangle closed as not reproduced; module 1's day-1 yard scrap confirmed in the original (VM save), its unreachable construction-yard retire removed; the original's loser survives (VM, Yaga Minor), so the morale retreat FUN_005d19a0 is ported (Empire-first stepping, squadron hyperdrives, Death Star strength) and checked natively on both sides (Hoth, Yavin: the loser withdraws); the AI orders no fleet at the system of an open battle (FUN_0051ddf0); a Move marks its fleet en route at once (FUN_004cf630), which ends the seed-9 Svivren shuttle; the 12 unread AiConfig fields removed (save v50, a second named golden cause), the slot order kept across an emptied arena (save v51), repair's fast interval needs a working own yard (FUN_00528040; no blockaded or rebelling own yard with a damaged ship in seeds 1-20, so its native check needs a staged case), the doc triage applied; the original's AI round scheduler mapped (rate+1 rounds a day, one order a round, 10 orders a side a day; FUN_00485b40, FUN_004361a0) and checked against how long a winner holds (need rules match, FUN_004b1070); the posture trace (posture_execute, posture_regroup); a siege winner holds natively on both sides (seed 7 Empire at Denab, seed 3 Alliance at Bothawui, a bombardment every second day for 40 days); VM: the original's winner stays at least a day after a Retreat and leaves within four (Praesitlyn, days 508 and 532), the port's leaves the next day; divergence.py resolves battles; Damage ship and native repair telemetry: a ship repairs every 2 days at its own working yard and every 30 elsewhere, on both sides (an own blockaded yard never holds its holder's damaged ship: the post-battle pass moves or destroys every loser, FUN_00515480); the battle lock holds every AI order at every system with a pending battle (native, both sides); scheduler steps 1-3 (every module's turn, the round scheduler, the AI as the day's last stage with one shared applier and strike resolver, at most 5 orders a side a day at Fast on seeds 42 and 9, save v52); missions refused at creation (Diplomacy churn gone on seed 9); the Death Star kill's support finalize (FUN_0058e320); timer 0x386 checked in the original (VM, Yaga Minor 67 -> 68 on day 70, 69 on day 100); gates, trace-determinism and invariants scripts; the step-3 fold-ins; step 4 (native rounds through ai_step, the player's advisor in its slot, every dispatch validated, the AI tempo in --config, native checks on both sides, the review's nine findings fixed); fixes 9.2-9.4 (SupportHolder from FUN_0055a080, timer 0x386 with the Maintenance Shortfall notice, bit 11 as a usable Death Star in the sector; native on both sides: Vagran 69 -> 70 without the Death Star and 69 held with it, Denab 35 -> 32); the neutral seeding fix (FUN_00559700); golden v1:dba544d28bce7aa4 -> v1:df820646127bbe3b for every cause from v50 to v53; Phase F: the docs sweep, the review fixes (save v53), every gate passes, traces leave the simulation unchanged (seed 42, 1000 days), the provenance baseline is empty, the scoped mutants closed or equivalent; fast-forwarded onto origin/main (PR #45) with every gate passing there; one commit"
created: 2026-10-15
updated: 2026-10-16
tags: [ai, research, production, postures, missions, parity]
---

# Original Research, the Strategic AI and the Open Items

Date: 2026-10-15. Approved by Tom the same day ("proceed with everything you've scoped").

Everything lands in one large commit after the docs and the review subagents,
and only on Tom's go-ahead. No commit or push before then.

## Rules for this plan

- At most two games run on the Mini, one per checkout. `launch-native.sh`
  stops its checkout's earlier instance before launching. The binary is
  `target/release/open-rebellion`; check liveness by that name.
- Replace sourceless port inventions with the original mechanism. Untraced
  details carry `hyp:` and a source-free port choice carries `port:`.
- Verify each step end to end in the running game (native on `mac-mini-ts`)
  or the headless playtest, for both factions. Don't add new unit tests, and
  don't run mutants during the parity QA.
- Cargo only through `scripts/mini-cargo.sh`. Bump the save version freely,
  with no migrations. Regenerate the seed-42 golden only for a named cause.
- Notes state current facts only. Fold docs into the feature commit.
- Ghidra headless runs locally on the scratch project copies, with Sonnet
  agents at medium effort. Do VM work myself.

## Done, uncommitted

- **Zero-delay timer.** A due phase timer fires in the same pass
  (`uprising.rs`, `missions.rs`; timer-scheduler.md, move-order.md).
- **AI module 12, class G character tasks.**
  - Code: `character_tasks.rs`, the Planner queries, `MoveCharacter` and
    `GiveCommand`, and research reservation by exhausted tree.
  - Save v42, with the golden regenerated.
  - Notes: ai-order-emission.md and ai-mission-planning.md.
- **Traces folded into the notes.**
  - research.md: passive points, cost scale, exhausted flags, validator, the
    roll, the AI planners.
  - ai-order-emission.md: the record lifecycle, the step machine, sector
    bits, the controller, mode-to-kind and mode selection.
- **or-glv7 cause found.** `evaluate_production` is a threshold heuristic.
  - At seed 42 the Empire queues nothing and the Alliance queues 2 ships.
  - A 120-day dual-AI run gives 0 production orders, so there is nothing to
    blockade either.
- **Blank special-force name: not a bug.** All nine SPECFCSD records are
  family `0x3c` in both the header and each record, so every class resolves.
- **Story overlay removed.** The port's invented `event_screen.rs` (a
  STRATEGY 6208-6268 picture with a draining bar on story events; 6208 is
  Mon Mothma) is deleted with its uses in `main.rs` and
  `strategic_windowing.rs`. Native check: no overlay to day 115 while
  `EVT_LUKE_DAGOBAH` fired (`native-checks/2026-10-15-story-no-overlay`).
- **Personnel icon layers.** `FUN_0042c3b0` draws the backdrop (11500, or
  11513/11514 for regiments), the en-route mark, then the object last; the
  port drew the mark on top and no backdrop. `fleet_window::mini_layers`
  composes them for the Defenses, Fleet and Missions lists. Original
  evidence `original-checks/2026-10-15-enroute-character`; native checks
  `2026-10-15-mini-layers-alliance` and `-empire`.

## Phase B: research as the original mission

Spec: `ghidra/notes/research.md`.

1. **State (`research.rs`).**
   - Per-side points pools for ships, troops and facilities, plus levels and
     exhausted flags.
   - Spending (FUN_00567ba0):
     - The next order that has a class costs `research_difficulty` × the
       galaxy's system count / 100, truncated.
     - The loop repeats while the points cover the cost, so it can buy
       several levels.
     - With no higher class, the tree's exhausted flag is set.
   - Passive timer 0x381:
     - It fires every GNPRTB 7169 + rand 7170 days.
     - Each active yard in a held system adds +1. The system must be neither
       in uprising nor blockaded.
     - Shipyards feed ships, training facilities feed troops, construction
       yards feed facilities.
   - The countdown projects are removed.
2. **Missions (`missions.rs`, `mission_planning.rs`).**
   - Kinds Ship Design, Facility Design and Troop Training (records
     `0x53000020`, `…21`, `…22`) on the existing repeat machinery, repeating
     every 10 + rand 10 days.
   - Per member: success when rand(0..99) < effective skill. A success adds
     GNPRTB 6171 + rand 6172 points (1–7) and raises the skill by GNPRTB 6158.
   - The pool is credited at the outcome.
   - Validator (FUN_0056c510):
     - The target is an own system holding a working yard of the tree's kind.
     - The tree is not exhausted, else end code 0xe.
     - At creation, a member with skill 0 is refused.
   - The player's mission dialog lists the three kinds.
3. **AI (`ai.rs`, integrator, `main.rs`).**
   - Remove `evaluate_research` and `AIAction::DispatchResearch`.
   - Research planners:
     - One scientist each, chosen by query capability 0x800 / 0x1000 / 0x2000
       with skill window 10..10000, ascending order, the top popped.
     - The target is the member's own system, which must not be blockaded.
     - The live planner count per tree is capped by the side's characters
       with that capability.
     - One request per cycle, in priority order: Recruitment, then Research
       1, 2, 3.
     - A tree's planners drop when the tree is done.
   - hyp: the controller runs once per round, one round per side per day.
4. **Wrap-up.** Status: save v43, golden, notes and the Alliance native check
   are done; the Empire native check is running.
   - Save v43 and the golden regenerated, with the cause named as "research
     missions".
   - Findings so far:
     - Playtest: the Empire's Lemelisk and Klev research ships at Coruscant
       (levels 1–5 by tick 375). The Alliance sends none, because in the
       headless seed-42 galaxy its one scientist, Wedge, starts at Yavin,
       which holds no yard (research.md, Open).
     - Native Alliance: Wedge's Ship Design Research at Yavin succeeds and
       advances Ship tech to level 1 by day 60. The native galaxy differs
       from the headless one.
     - Native Empire: the Empire starts with no scientist, so the check
       first recruits one (Palpatine or Vader on Recruitment).
   - research.md's port table brought to current facts.
   - E2E:
     - Native, both sides: create a research mission and see the pool and
       level change.
     - Playtest: AI levels climb past where the countdown stalled (about
       level 3 by tick 106 before).

## Phase C: the original strategic AI (production, postures, class I)

The port has no posture engine, no module-13 mode selection and no
controller. Class I tasks hang off regional records, and the production
budgets come from the same controller context. The production-budget trace
(modules 1, 2, 3, 5, 6, 7, 10, 11) is still running. Its report fixes the
sub-steps below, and it goes into ai-order-emission.md before any code.

1. **Context and counters.** Port the controller context fields the modes and
   budgets read. That is R, A, B, X, Y, C and G, plus the per-category spend
   fields `+0x1dc`, `+0x1e0`, `+0x24c`, `+0x254` and `+0x2c4`, as far as they
   are traced. Unknown inputs carry `hyp:`.
2. **Mode selection (FUN_004bf490) and the controller (FUN_004e5440).**
   - The six controller states and the plan size n, with grow and shrink.
   - The slot-5 plan: the lead kind by mode, the common tail, and rank-sum
     sector picks with stickiness.
   - The cadence: replace the invented intervals (`AiConfig::tick_interval`
     7 in tuning.rs, `AI_TICK_INTERVAL` 5 in ai.rs) with the original's one
     module-scheduler call per main-loop pass, alternating sides
     (ai-order-emission.md, from line 758; `FUN_0041cf20` -> `FUN_004361a0`,
     `FUN_00485b40`). Taken over from the archived Resolve-AI-Guesses plan.
3. **Regional records (module 8).**
   - The record lifecycle on the +0x18c sum and sector assignment.
   - The six-step machine: fleet sums against the posture requirement,
     shortfall bits, flagship, staging and satisfaction bits.
   - The posture step's Move and the arrival emitter: assault, bombardment
     and Destroy System.
   - Replace the invented fleet heuristics (`evaluate_fleet_deployment`,
     `evaluate_ds_escort`, `evaluate_troop_deployment`) where the posture
     engine covers them. Anything left stays with a `port:` tag.
4. **Class I tasks (module 12).**
   - Admiral, General and Commander tasks per record, created through
     posture slot 7.
   - The cancel tests and release on a kind-0 sector.
   - The target is read from the record's staging system and flagship.
5. **Production modules.** Replace `evaluate_production` with modules 1, 2,
   3, 5, 6, 7, 10 and 11.
   - Orders: Build 0x210, 0x211 and 0x212, Destination and Scrap.
   - Each module runs under its budget percentage and goal bit, with the
     module weights and cadence as traced.
6. **Wrap-up.**
   - Save bump and golden regenerated, with the cause named as "original
     strategic AI".
   - E2E:
     - Dual-AI playtest: production, fleet moves and blockades appear, and
       `telemetry_coverage` passes without a changed expectation. That closes
       or-glv7.
     - Native, both sides: AI fleets form and move against the player.

Phase C is large, so each sub-step gets its own e2e check before the next.

**Order and design (from the production trace).** C5 goes first, with the
census part of C1, because it closes or-glv7. Then C2, C3 and C4.

- A new `crates/rebellion-core/src/ai_production.rs`.
  - **Census per round:** capacity and load (`resources::maintenance_capacity`,
    `maintenance_used`), the spent counters, yard maintenance per system, and
    own mines and refineries.
  - **Saved `AIState` fields:** the goal token (pending, token, seed = 0,
    `0x80`, `0x40`), the adaptive targets (3,3,2,1,1,0,0,3,9,0,6,12) and the
    module cursor. This needs a save bump.
- The modules as traced: producer, class pick by key with a random tie-break,
  and a count cut to budget, cost and yards.
- The orders become `AIAction`s:
  - `EnqueueProduction` gains a count and an optional destination
    (`set_destination`);
  - Scrap goes through `scrap.rs`;
  - Retire for module 5.
- Inputs that wait on C3's regional records take `hyp:` defaults:
  - module 11 uses its "none" rows;
  - module 10 uses its `FUN_0041a4c0` fallback;
  - the module system lists are own systems in id order.
- Cadence (hyp): one round per side per day.
- Steps, each with a dual-AI playtest:
  - (a) census, budgets, token and modules 1, 2, 3;
  - (b) module 11;
  - (c) modules 10 and 7;
  - (d) modules 5 and 6;
  - (e) `evaluate_production` deleted, then the save bump, golden and
    `telemetry_coverage`.

**C5 status (done, uncommitted).**

- `ai_production.rs` ports modules 1, 2, 3, 5, 6, 7, 10 and 11, in the
  scheduler's ascending kind order (`FUN_004be980`; or-3ioq). `evaluate_production` and
  `ProductionConfig` are deleted, with the four tests of their invented
  behaviour.
- The executor's maintenance check refuses only a nonzero total
  (`FUN_0052e970`).
- Every Build of modules 10, 7, 6 and the facility modules goes out after its
  Destination, even to the producer itself. Modules 11 and 5 issue none.
- A production Destination may name a fleet (`manufacturing::Destination`).
  The product boards the fleet on arrival, or is lost if the fleet is gone.
  This applies to the player's Destination release on a fleet too.
- Save v44. Golden `v1:ad62df643f543d66` -> `v1:23fab501537a2a37`, cause
  "original production AI and fleet destinations".
- Gates: the workspace tests, the provenance check (baseline shrank by 5)
  and the golden pass.
- `telemetry_coverage` finds manufacturing, blockade and repair (15 repair
  events since or-39er).
- Playtest: the Empire builds yards, mines, regiments, fighters and ships
  with destinations. The fighter build-and-scrap churn is gone.
- Module 1 scraps a side's starting shipyards on its first pass; nothing
  exempts them (ai-production.md, U10 item 6). In the headless seed-42
  galaxy it removes the Alliance's Hoth yard on day 1.
- Next: a native e2e of production on both sides, with the release build.

**C2-C4 status (done).**

- C2, mode selection and the controller: stay thresholds X < own+1 and
  Y < own/2+1, fresh ones X < own/2+1 and Y < own/4+1; controller states 1-6
  (slots 5-8), plan size +-1, build goal FUN_004e5b10; the census
  FUN_00418500 runs between rounds; sector bits 4 own, 8 enemy, 0x10
  neutral, 0x40000 foreign fleet; HQ move FUN_004bf9e0/004bfe30/004c05e0.
- C3, regional records: module 8 is a four-state driver over six posture
  kinds; the requirement fields only grow, so they are saved; FUN_004b1070
  issues Assault, Bombardment and Destroy System; `evaluate_ds_escort` has no
  original counterpart. Ten items stay open.
- C4, fleet tasks and class I: only controller slot 8 (FUN_004e69d0) creates
  task records, states 1-8; class I runs through R+0xe0 (Admiral 0x262,
  General 0x263, Commander 0x261). It supersedes the `hq_defended` heuristic.
- 39 new decompiles are in `ghidra/notes/`.
- Next: fold the specs into ai-order-emission.md and the AI notes, then
  replace `evaluate_fleet_deployment`, `evaluate_ds_escort` and
  `evaluate_troop_deployment`; recheck or-glv7 once module 8 returns damaged
  fleets.

**C2-C4 implementation design (2026-10-15).** Specs:
`scratchpad/spec-c2-modes-controller.md`, `spec-c3-regional-records.md`,
`spec-c4-fleet-tasks-class-i.md`; they fold into `ai-order-emission.md`
(sections "Module 13", "Regional records", "Fleet tasks") before each step's
code.

- **Census (`ai_production.rs`, extended in place).** `SystemRecord` gains
  the `sys+0x2c` (Q) and `sys+0x30` (S) words and the figures module 8 reads
  (foreign stationary fleet sums `+0xe4/+0xe8/+0xf4`, squadron figures
  `+0xb0/+0xb4`, shield strength `+0x68`, `+0x110`, last seen `+0x44`).
  `SectorRecord` gains the `+0x28` and `+0x2c` words and the strategic slots
  of `FUN_00432350`. New per-side `FleetRecord` and `ShipRecord` folds
  (`FUN_00433620`, `FUN_00433780`, `FUN_00484630`) and the context sums
  (`FUN_0041b7e0`, `FUN_0041ba80`, `c+0x164..0x174`). One census feeds every
  module.
- **Ship identity.** Ships carry their world tag from the first AI pass
  (`carriage::tag_ship` over untagged ships, run by the integrator before the
  AI); port: the original's object id.
- **Persistent marks** live in `AIState`, keyed by ship tag and `FleetKey`:
  ship record/plan tag/free/flagship/task claim, fleet record/plan
  tag/unadopted/task-owned/need bits/target, system claim bits, sector
  kind/param/record/plan entry/planned/exhausted.
- **Steps, each with a dual-AI playtest before the next:**
  1. C2: `ai_strategy.rs`, module 13 (mode selection, controller states 1-6,
     plan and write-back, tagging, slot 6 and 7 requests, slot 8 fleet-task
     count) and the Alliance HQ move (Scrap `0x200`, Move HQ `0x201`).
  2. C3a: `ai_regions.rs`, records (create, delete, sector assignment,
     release), steps 1-4 and 6, Create Fleet (`AIAction::CreateFleet` through
     `fleet_join::move_ships_to_system`).
  3. C3b: the posture engine (states 1-14, slots 13-21) and its orders
     (fleet Move, squadron and regiment loading, unloading and drops,
     Planetary Assault, Bombardment, Destroy System, character disembark);
     delete `evaluate_fleet_deployment`, `evaluate_ds_escort`,
     `evaluate_troop_deployment` with their state (`battle_cooldowns`,
     `targeted_counts`).
  4. C4a: `fleet_tasks.rs`, module 9 (states 0-8, revalidation, Join Fleet).
  5. C4b: class I tasks in `character_tasks.rs` (Admiral, General, Commander
     per record; the record request and cancel; flagship and staging moves).
  6. Cadence: the scheduler (`FUN_00485b40`) as a per-side step machine over
     the modules in list order; hyp: each side runs passes until every armed
     module finished once per day, replacing `AiConfig::tick_interval` and
     `AI_TICK_INTERVAL`. Then the save bump, golden ("original strategic
     AI"), `telemetry_coverage`, or-glv7, and native e2e on both sides.

## Phase D: or-pfv4, a traveller listed at its destination

- From the order on, a travelling character or special force is listed in
  the destination's Personnel with the hyperspace backdrop. Its pop-up
  disables Move, Confirmed Move, Mission and Retire until event 0x387.
- Readers traced (move-order.md, "Who reads the en route bit"). The icon
  mark is STRATEGY 11501, plus 11520 for a special force's portrait.
- Design:
  - `depart` places the member at its destination (and aboard the target
    fleet) at the order. The transit stays in `MissionState::en_route` as
    the en-route bit, and arrival only clears it.
  - Readers that skip a member en route:
    - the AI records (already `EN_ROUTE`);
    - the menu enables (Move, Confirmed Move, Mission, Retire, Command);
    - mission rule 5 and container acceptance;
    - uprising incidents and regiment counts, mission defenders, the story
      "free character" tests, and Force discovery candidates.
  - Readers that keep it as present: the Personnel lists, a system changing
    hands, a container destroyed, the counter-intelligence officer, and the
    Vader/Luke/Emperor co-location checks.
  - Remove the Missions window's transit workaround in `listed_members`.
  - Add the mark to the Personnel icons.
- Original UI only. Both factions in native, compared with the original.

**D status (done, uncommitted; native checks passed on both sides).**

- Members stand in their destination container from the order with an
  `en_route` flag; arrival only clears it (`MissionEffect::EnRoute`).
  `MemberMoved` always names a system, and `MemberTransit` keeps member,
  mission and arrival only (save v45).
- Skip readers: the Move, Confirmed Move, Mission and Command gates and
  dispatch (`MemberState::en_route`), rule 5 for character and special force
  targets, uprising incidents 3-5, the AI records' `0x20`, the AI's Retire,
  the bounty's free-Han test, the port-only AI target lists and officer
  bonus. Phase 4's wait still reads the transit schedule, which arrival
  updates within the day.
- Render: the Missions window lists the system's members with their own
  flag (the transit append is gone); the Finder's Enroute, the Status rows
  (Attached names the destination) and the en-route mark on the Fleet and
  Defenses Personnel lists read the flag. The Finder's special-force usable
  brush stays: its mapping is an inference.
- Gates: workspace tests green, golden regenerated (`v1:2eaa06dba4648717`
  -> `v1:236fe77dd59ec2d1`, cause named), provenance baseline 93 -> 92.
- Empire native check passed:
  Vader, ordered to Corellia on day 0, is listed in Corellia's Personnel on
  days 2 and 24 with the hyperspace mark; Move, Confirmed Move, Mission and
  Retire are disabled and Command enabled, as in the original's day-24
  menu. On day 25 the plain portrait returns with the rows enabled.
- Alliance native check passed (`2026-10-15-pfv4-alliance`): Luke, Yavin
  to Ghorman, listed with the mark on days 2 and 85, plain on day 86; Jan
  Dodonna's rows disabled while travelling (days 89, 173) and enabled on
  arrival (174, 175). Both evidence folders carry a README.
- Finding, out of scope: after arrival Luke's Move and Mission stay
  disabled because the port's invented `EVT_LUKE_DAGOBAH` puts him on a
  mandatory mission without moving him; filed on `or-port-story-research-c80`.

## Phase E: the remaining checks

- **Blue rectangle.**
  - The pure-blue block was seen once, from a mid-development binary
    (`2026-10-15-character-move-a/05-after-confirm`). It coincided with a
    message being filed.
  - The current build is clean frame by frame
    (`2026-10-15-blue-rect`).
  - Closed as not reproduced (2026-10-16, `2026-10-16-blue-rect5` README):
    five runs, including the old layout, the confirm during message filing
    and the advisor's slot 80 idle animation, show no block.
- **Earlier unverified e2e checks:** surviving loser relocation, the holder
  tie rule.
  - The cutscene music recheck passed (2026-10-16, `2026-10-16-blue-rect5`
    README): skipped story cutscenes leave the music playing.
  - Rank lapse on a mission passed on both sides (2026-10-16): General
    Vader and Commander Skywalker, ranked through the Command menu, lose
    rank and ranked name when dispatched (`2026-10-16-rank-lapse-*`).
  - The post-battle pass: four simulated battles destroyed the loser
    outright, and a retreat leaves before the pass, so neither a surviving
    loser nor a no-winner battle was staged yet
    (`2026-10-16-postbattle-empire2`). Headless, seeds 1-12, 7 and 42 over
    1000 days: every pass had no loser object left (battles run to the
    end), and the two holder-rule passes had one side gone. In the original
    the loser survives: auto-resolving the Yaga Minor blockade break
    (`empire-day30-yaga-alert-control`) gives "The Alliance fleet has
    withdrawn", Bulk Cruiser 3 operational, no casualties. The deciding
    rule is being read (or-q-loser-survives-yd7o); the port destroys every
    loser.
- **Module 1's day-1 scrap is the original's** (VM save `yards-day2`: the
  Empire AI scrapped two shipyards and a training facility at its mine site
  and queued a mine there; `FUN_004dc650`). The port's construction-yard
  retire on a negative budget is unreachable in the original
  (`FUN_004dbd60`) and is being removed.

## Phase G: native test tooling and telemetry

The native checks spend most of their time waiting, clicking pixel
coordinates and reading screenshots. Two facts shape the work:
- the native game runs its own inline copy of the tick in `main.rs`, drawing
  from `sim_rng`;
- headless runs call `run_simulation_tick` with pre-rolled values, so one
  seed does not play the same game on both paths;
- native writes no `GameEventRecord`, only free-text `game.log` lines.

Built in parallel by five agents, each owning its files. All tooling is
dev-only and changes no original UI. Nothing touches the RNG, the tick
order or the save format, so the golden stays unchanged.

| # | Item | Kind |
|---|---|---|
| 1 | `Fast forward: on\|off`: one day per frame | speed |
| 2 | Waits refuse at once when a modal holds the clock, naming it; `Wait until mission resolved / message / pause alert / tech` | speed |
| 3 | `dev-send.sh --file`, `dev-run.sh`: a whole script over one ssh | speed |
| 4 | `Battle policy: simulate\|retreat\|stop`, `Advice policy: close`, through the real buttons, logged `[dev-policy]` | speed |
| 5 | `List characters`, `List missions`, `Show research`, `List production`, `Show side` (dev truth through fog of war) | log |
| 6 | `Save:` / `Load:` from the dev channel; `launch-native.sh --load` | staging |
| 7 | `Mission: <chars> <kind> at <system>` through the dialog's own dispatch; never stands in for a dialog check | speed |
| 8 | Named control registry: `Click control: <window>/<control>`, `Locate`, `List controls` | speed |
| 9 | Native `telemetry.jsonl` in the `GameEventRecord` schema, `source` and `seq` added | log |
| 10 | Decision traces per subsystem (missions, ai, research, blockade, combat, control), `OPEN_REBELLION_TRACE` / `--trace`; the golden must match with traces on and off | log |
| 11 | `scripts/telemetry.py`; `dev-report.py` interleaves telemetry and `[dev-policy]` lines | log |
| 12 | Playtest `--save-out` / `--load`; the REPL `advance_N` restarts its day count at 1, fixed | staging |
| 13 | `scripts/calibrate.py`: distributions over many seeds against cited reference values | calibration |
| 14 | `scripts/divergence.py`: native and headless from one save, first divergent tick | calibration |
| 15 | `Recruit: <character> for <side>`, the Recruitment mission's own success effect | staging |

Out of scope, needs Tom's approval: native calling `run_simulation_tick`
(an architecture change); item 14 measures the drift until then.

Status: all 15 built and checked live on both sides (below). An Opus review
found 16 defects (4 high: the WASM build lost its cfg gates, the battle
policy clicked on the frame a battle-alert wait was met, `divergence.py`
compared the RNG and fog that differ by construction, `calibrate.py` never
enabled traces); the fixes are in progress with the four tool agents. Then
the retreat policy, fast forward during waits only and the wait budget get a
native recheck, and the docs a `/claude-md-manager` pass.

The Empire research check (Phase B) passed on 2026-10-16 (`2026-10-16-tools-empire`, README there): `Recruit:` staged Bevel Lemelisk at Coruscant; his Facility Design Research went through the real Create Mission path by control names; the traces show his rolls (chance = facility skill 81-83; day 682 failed, 694 +3, 705 +6, 718 +1) and Facility tech reaching level 1 on day 718, with "Empire Facility tech advanced to level 1" in the Message Index. Coruscant holds no shipyard, so Ship Design is not offered. The same run checked items 1-8, 9-11 and 15 live. Two tooling fixes followed: fast forward ran between commands (about 30 days per round trip), and a pending wait could not be cancelled. A concurrent-sync race in `scripts/mini-cargo.sh` (one run's deletions reading another's half-written manifest) deleted files on the Mini; the sync now holds a lock on the Mini.

Two natural Recruitment rounds at Coruscant had brought Grammel, Zuggs,
Daala and Piett, all Espionage only (`2026-10-16-research-empire-c`), which
is why the check staged Lemelisk with `Recruit:`.

## Phase H: what the Ghidra answers port

From the 2026-10-16 decompiles (`ship-damage-repair.md`, `ai-production.md`
U10, `research.md`), after Phase G's review fixes land:

1. **or-3ioq, the module order.** The scheduler walks a tree keyed by kind
   (`FUN_005f4f10`, `FUN_004be980`), so it runs 1, 2, 3, 5, 6, 7, 10, 11.
   - `Module::ALL` follows the reading from `FUN_00485a50`; reconcile the
     two readings first.
   - Small: one array and its comment. The golden changes, cause named.
   - Done: the node constructor (`FUN_004ec940`) stores the kind as the
     tree key; `Module::ALL` follows it.
2. **or-39er, damage and repair.** Steps, one file group each:
   - (a) the six damage counters on the ship (`ship+0x64`), renaming the
     misread recharge fields;
   - (b) `ship_record` scaling and the `0x1000`/`0x800000` flags
     (`FUN_00484630`);
   - (c) `FUN_004c8e90` in module 11 phase `0x3f8`;
   - (d) `repair.rs` with the 30-day and 2-day cadences and the
     damage-control roll (`FUN_00501c50`);
   - then a save bump, the golden and `telemetry_coverage` (repair).
   - Done, save v49: seed 42, 1000 days, 4 repairs started and finished; the
     AI moved two damaged fleets to system 78 by the shipyard-role query;
     `telemetry_coverage` shows 15 repair events.
3. **or-rhik, research planners.** Persistent per-tree planners holding one
   claimed member, the order emitted without the yard pre-test, retired only
   on exhaustion or the count limit (`FUN_004bced0`, `FUN_004e8290`).
   - Done: the planners persist; the AI's order at a system without a
     working yard of the tree, or for an exhausted tree, is refused before a
     mission exists (`FUN_0056c510`, status `0x40`/`0x2c`, `0x40`/`0x32`).
     Seed 42, 1000 days: the Empire resolves 82 Ship and 53 Facility
     missions at Coruscant; the Alliance's scientists stand at yardless
     Yavin and research by the passive timer only. Save v47.
4. **or-ahj2, load determinism.** A loaded save reuses freed slotmap slots
   in a different order from the uninterrupted game, so it plays
   differently once a new object fills a freed slot (one facility, 10–15
   days after a day-60 load). Restore each slotmap's free order, or allocate
   independently of slot reuse; save bump.
   - Done, save v48: each arena saves its free order and the load rebuilds
     it. Seed 42 (day 60 + 15, day 300 + 700) and seed 7 (day 60 + 15, day
     300 + 300) loaded runs match the straight runs; four 1000-day runs per
     seed give one fingerprint. An arena with no live value keeps slotmap's
     default order.

5. **or-q-loser-survives-yd7o, the morale retreat** (Tom approved
   2026-10-16). The original's loser survives: the tactical engine's
   per-second check `FUN_005d19a0` withdraws a side below its threshold
   times the enemy's strength (computer 0.9, less at its own system, 0 at
   its HQ, `FUN_004e6110`; human 0), when a retreat system exists and its
   hyperdrives work (`ghidra/notes/morale-retreat.md`). The port runs every
   battle to the end (222 battles over 20 seeds, no loser survived). Steps:
   the check and Empire-first stepping in `tactical.rs`; thresholds and
   permission where sessions are built; the AI threshold. Then a golden
   with the cause named, and native checks on both sides, which also
   unblock the surviving-loser and holder-tie checks.

6. **The AI round scheduler** (Tom approved 2026-10-16, with Fast as the
   playtest default). Seed 9 over 1000 days issues a median 12 AI orders a
   day (maximum 20), against the original's 5 a side at Fast and 10 at the
   other speeds (`FUN_00485b40`, `FUN_00485de0`, the cap set in
   `FUN_00485a50`). The port runs three passes a day, each capped at 10.
   The original runs rate+1 rounds a day (5, 13, 61, 601 from Fast to Very
   Slow, `FUN_004361a0`), one order a round, 10 orders a side a day, reset
   on the day's last step. A VM check could not tell the two apart: the
   original's winner stays at least a day after a Retreat and leaves within
   four (Praesitlyn, days 508 and 532), the port's leaves the next day.
   Steps, three files at most each:
   - 1a `ai_regions.rs` and `fleet_tasks.rs` emit one order a round; the
     en-route mark (fix A) goes, and a whole-fleet Join to another system
     marks its fleet and ships en route;
     Done: `Regions::turn`, `FleetTasks::turn` and their `reject`
     (`FUN_004cf630`, `FUN_004cf2c0`, `FUN_004dd810`), and `Round::dirty`,
     set where the port writes a sector record (`FUN_004317d0`). The daily
     `pass` loops run on the same calls until step 2, so seed 42 keeps
     fingerprint `v1:3458e7d4f49df57c` over 1000 days, with traces on and
     off (`scripts/trace-determinism.sh`);
   - 1b `ai_production.rs`; 1c `character_tasks.rs` and `ai_research.rs`.
     Done: production turns, one phase each, with the phase's later orders
     queued and a bundled Destination and Build owing a second round
     (`FUN_004c8ba0`); module 12 one task a turn (`FUN_004c8740`,
     `FUN_004bba90`); module 4 one planner a turn over the research
     planners and the port's five mission evaluators (`FUN_004d2490`,
     `FUN_004d2730`). Save v52. Over seed 42's 1000 days the telemetry,
     `wall_ms` removed, is byte-identical to the step 1a run;
   - 2 `AISystem::round` and the scheduler state; the daily cap and the
     rounds per speed join the `--config` AI tuning, defaulting to the
     original's values;
     Done: `ai_scheduler.rs` (`Scheduler`, `AISystem::round`, `settle`,
     `end_day`; the priority scan and round-robin of `FUN_004bede0` and
     `FUN_004be980`; orders counted at issue, `FUN_00485b40.c:39-48`), the
     `ai/round` trace, the old per-day passes deleted. `advance` runs one
     round a day until step 3. Core 750 and data 156 pass unchanged; the
     AI tuning knobs move to step 3 with the per-day loop;
   - 3 `simulation.rs`, `integrator.rs`, the playtest `--speed` (default
     Fast), and an orders-per-side-per-day row in `telemetry.py` and
     `calibrate.py`, graded against `calibration-reference.json`;
   - 4 `main.rs` and `tick.rs` run the rounds inside the day.
   Then seeds 1-20 (at most 5 orders a side a day at Fast) and native runs
   on both sides with `scripts/native-checks/` (tempo, battle hold,
   mid-day save and reload; `check-scheduler.py`). The native runs also
   show a refused AI dispatch with its refusal in the outcome record, the
   in-game check for the creation-time refusal.
   - Fold-ins for step 3 (the steps 1-2 review and the open-items trace,
     2026-10-16): module 11 reads module 8's live record bits; production
     resumes after any step-2/3 order (`FUN_004c8ba0`); the missing
     "records changed" marks (production claims and targets, the loading
     system); the base target rules before the class checks; a Research
     planner sits out while its mission flies (`FUN_004bda40`); module
     13's planner request consumed once per module-4 turn; module 14 ends
     the round on its last rename (`FUN_004d29b0`); Espionage targets per
     the original's q bit `0x200` if `FUN_004b1bf0` confirms it;
     `max_covert_ops_per_eval` and `max_recon_per_eval` removed. Step 4:
     the player's advisor agent runs one module call per step in its index
     slot (`FUN_00439a10`).
   - The AI runs as the day's last stage, after blockade, uprising, betrayal
     and the Death Star (phase 8 follows phase 5's due timers; Tom approved
     2026-10-16).
   - Tooling folded in (Tom approved 2026-10-16): one outcome record per AI
     order on both paths, applied, refused with its reason, or held at a
     battle, each with its faction (steps 3-4); an `ai/round` trace (step 2);
     `scripts/invariants.py` with the orders-per-side-per-day rule (step 3);
     `scripts/gates.sh` for the Phase F gates; a trace on/off determinism
     script for the golden step.
   - Decided 2026-10-16 (Tom: "use your judgement"): one AI applier, one
     strike resolver and one round driver in `rebellion-data`, called by
     both the headless and native paths, so both record the same outcomes;
     the native bombardment's sourceless popularity loss gives way to the
     original's finalize (`FUN_0058e320`: support for the bombarding side
     by GNPRTB 7705, then the sector or galaxy shift; `bombardment.md`),
     for player and AI alike, a named golden cause; the day's first round
     waits while a player battle is open (hyp: the Battle Alert is modal
     before the step's phase 8), with the lock kept as a guard that
     records `held` and resets the module; outcome fields `outcome`,
     `refusal`, `module`; a daily `ai_day` record carries the speed; the
     AI runs after research, repair and Jedi and before flagship naming,
     rank lapses and the victory check (port:); the regiment-run pass runs
     after each applied round. Steps (`step34-prep`): 3m, 3a, 3b, 3t, 3c,
     3d, 3e, 4, 4b, plus the strike finalize.
   - Step 4 done (2026-10-16). The AI tempo knobs are `--config` AI
     tuning (`day_orders`, 10; `rounds_per_day`, 601/61/13/5). The native
     game runs the same `ai_step` as headless: round 0 at the AI stage, the
     later steps over the day's frames (`GameClock::in_day_step`), the rest
     at the next day's start. A battle or cutscene opened that frame holds
     round 0 (hyp:), and a saved `day_started` flag closes that day even
     when the next frame crosses it. The player's advisor takes the
     player's slot, one module a round (`FUN_00439a10`, `FUN_00486500`), its
     order applied at the next step. Every dispatch goes through the
     creation validator (`dispatch_guarded` deleted). The player's
     bombardment and Death Star end with `bombardment::finalize`. Native
     and headless write the same records, stamped with the round's day.
     An Opus review's nine findings are fixed; its advisor-speed finding
     was wrong (phase 8 calls a side's slot until it returns 1,
     `FUN_004361a0.c:8-13`), so that one is doc-only. Checked: native
     tempo, battle hold (a day-crossing frame and orders held at a second
     contested system) and mid-day save and reload, seed 42, both sides;
     refused AI dispatches natively on both sides (seed 11 Alliance
     Sabotage, target destroyed; seed 20 Empire Assassination, target
     missing); headless seeds 1-20 at most 5 orders a side a day. A held
     order re-issued every second round at a battle the combat loop defers
     to the next tick is br or-nn7x (does the original fight every battle
     of a day before it moves on?).
   - After the commit: `Pause policy: resume` and `Include:` for scripts,
     `Show system`/`List fleets` readouts, a day-by-day divergence re-sync,
     the proxy DLL writing our JSONL, the remaining invariants, and an
     `Uprising at` command for the rebelling-yard repair check.
7. **Repair at an own blockaded yard.** `Damage ship: <fleet>` halves a
   ship's hull, and native telemetry logs each repair step. Natively: a
   ship at its own working yard repairs every 2 days (Alliance Fleet 3 at
   Mon Calamari, Empire Fleet 1 at Coruscant); without a yard, or at an
   enemy system under the player's blockade (Empire Fleet 1 at Corsin), it
   repairs every 30. An own blockaded yard never holds a damaged ship of
   its holder: a battle with both sides present names a winner by the
   holder rule, and every fleet of the loser retreats or is destroyed
   (`FUN_00516ef0` -> `FUN_00515480`, `system-squadrons-combat.md` 3b,
   "Sides and ties"), so neither side keeps a fleet beside the other. An
   own rebelling yard can hold one; no command starts an uprising, so its
   check waits for `Uprising at`.

8. **Missions refused at creation** (Tom approved 2026-10-16), between
   scheduler steps 2 and 3. Seed 9 over 1000 days dispatched Diplomacy
   9,052 times and resolved it 32 times; 8,380 dispatches sent the same
   character to the same target the next day. The port's target pickers
   pick targets the mission forbids, `dispatch_guarded` accepts them, and
   the next day's validator ends them before anyone leaves. The original
   refuses them at creation (`FUN_00542b60` -> `FUN_0054c590`). Port the
   creation-time refusal, filter the AI pickers as the original's planner
   queries do, and record refused dispatches and early mission ends in
   telemetry. A named golden cause.
9. **The provenance review's behaviour findings** (Tom approved 2026-10-16
   for this commit). The review found three port rules that differ from
   their cited sources:
   - `SupportTier::from_support_int`'s top cut against `FUN_0055a080`, where
     support of 60 already holds a system (`economy.rs:621` reads it);
   - `CombatEntityKind::from_family_byte`'s mapping, which contradicts the
     family list it cites (test-only callers);
   - the port's maintenance timer, which has no source. The original's
     7694/7696 delay is timer `0x386` on blockaded systems, which shifts
     support by 7693/7695 (`timer-scheduler.md:152`).

   Each gets the original's rule after the scheduler's step 4, in this
   order, under save v52 (Tom approved the approach):
   1. Delete `from_family_byte` and its test: nothing calls it, and the
      test's mapping contradicts `annotated-functions.md:13-22`. Done: the
      `CombatEntityKind` enum, its mapping and the test are gone (no golden
      effect).
   2. Reduce `SupportTier` to `FUN_0055a080`'s three outcomes, keeping
      `SupportChanged` as `port:` telemetry. Golden cause. Done:
      `SupportHolder` (`support_holder`) serves control and
      `SupportChanged`, which fires when the holder changes.
   3. Replace the maintenance cooldowns and `MaintenanceShortfall` with timer
      `0x386` per blockaded system. Native blockade check on both sides.
      Golden cause. Spec: the timer's arm/re-arm/disarm trace
      (`timer-scheduler.md`). The original checked in the VM
      (`empire-day53-yaga-orbit.sav`): the Empire blockading Alliance-held
      Yaga Minor without the majority, Alliance support 67 on day 53, 68
      on day 75 and 69 on day 106, with the saved timer's due day moving
      70 -> 100 -> 132 (a fresh timer around day 102, `timer-scheduler.md`).
      The port's "maintenance shortfall across N systems" message goes with
      the cooldowns. The original's notification `0xc` is the autoscrap
      notice (`FUN_00498970`, event `0x304`): "Maintenance Shortfall", "The
      following units have been destroyed at {system} due to a maintenance
      shortfall:" and one line per unit (RCDATA `0x7080`-`0x7082`), filed
      when the overdraft timer `0x382` scraps a unit. The port files it from
      `StockpileEvent::OverdraftScrap`, checked natively on both sides.
      Done: `blockade.rs` `advance_support_shift` (the majority weighs the
      Alliance's points against 100 minus them, `FUN_00509930`), telemetry
      `blockade_support_shift`; the cooldowns are gone, and each overdraft
      scrap files the notice natively and a `maintenance_shortfall` record.
      Native, seed 42: an Imperial blockade of Vagran (69/31) restarted on
      days 27, 37 and 76 after Alliance attacks and shifted on days 67 and
      106, 30 days after each start, moving support to 70/30; an Alliance
      blockade of Denab (35/65) shifted on days 31, 61 and 91 to 32/68; an
      Empire TIE Fighter scrap at Coruscant filed the notice.
   4. System bit 11 is the Death Star in the sector (`FUN_0050a890` via
      `FUN_005178b0`), not strong support: replace `is_strong_support`
      (`economy.rs`, `uprising.rs`) as the flag of `FUN_00559be0` (halve
      Alliance gains and Empire losses) and `FUN_00559b60` (Empire regiment
      weight doubled). Golden cause (Tom approved 2026-10-16). Only a
      usable Death Star counts: existing, completed and not en route
      (`+0x50` bit 0, `FUN_004f7b80.c:8-12`, tested at
      `FUN_005178b0.c:40`), so one under construction or in transit does not.
      Done: `death_star::refresh_sectors` sets `GameWorld::death_star_sectors`
      at the start of each tick and after arrivals, read through
      `GameWorld::death_star_in_sector`. Native, seed 42: with the Death
      Star in the Imperial fleet blockading Vagran, the day-67 shift (-1, a
      change toward the Alliance) is halved to 0 and support stays 69/31.

The golden was regenerated for the Phase H causes
(`v1:7e5c233c69c86cba` -> `v1:4c13a5dc36e88e1d`) and once more for every
later cause, from save v50 to save v53
(`v1:dba544d28bce7aa4` -> `v1:df820646127bbe3b`; the list is in
`deterministic-replay.md`). The same values come back
with every trace subsystem on and with all off.

Phase G found and fixed one determinism bug in this plan's AI. The
system-fighter Scrap (`FUN_004d4800`, `scrap_system_fighter`) drew one roll
per candidate in `HashMap` order; it now walks the world's slotmap order. Ten
400-tick runs and thirteen 1000-tick runs on three seeds each give one
fingerprint, and the golden is unchanged.

## Phase F: docs, reviews, commit

1. Update the docs:
   - save-load.md and deterministic-replay.md (the causes list);
   - ai-order-emission.md and research.md port tables;
   - the task graph (`--append-notes`, `scripts/task-index.py`, then the
     local sync script);
   - close the closed nodes;
   - ai-parity-tracker.md: row 1 maps `FUN_00519d00` to
     `evaluate_galaxy_state()`, but it is new-game stage `0xb`
     (ai-behavior-analysis.md:54); the "Map visibility: Omniscient FAITHFUL"
     row contradicts observation-levels.md and should point at
     `or-q-port-observation-levels-jgy`; line 52 reads `FUN_004927c0` as a
     day tick, but it is a message-`0x1f0` handler (timer-scheduler.md
     182-197); review the "BY DESIGN" rows; and drop the stale "Unported
     validators" comment in ai.rs (around line 533).
   - Done 2026-10-16: the tracker rows and research.md's AI section match
     the code; no BY DESIGN rows remain. The ai.rs comment is gone, and
     deterministic-replay.md carries the golden with its causes by save
     version. The docs sweep (2026-10-16) matched the Port tables, seeding,
     tick order, systems pages, game-domain, roadmap and README to the code.
2. Launch the review subagents on the whole uncommitted change: correctness,
   parity with the notes, provenance tags, simplicity. Fix every finding.
   - Done 2026-10-16 for fixes 9.2-9.5: bit 11 is rebuilt after a load and
     a new game; a regiment scrapped in transit stands at its destination
     (`FUN_00556430`); timer `0x386` fires on its due day when a frame
     covers several, the next due a delay after it (`FUN_005862a0`); the
     economy record's unread bit-11 copy is gone (save v53, golden); the
     neutral seeding has a unit test; the provenance baseline is empty;
     native support drift and support-change telemetry. Native, seed 42,
     Empire: with the Death Star blockading Vagran, a save and reload on
     day 40 keeps the halving (the day-67 and day-106 shifts are 0; Vagran
     69/31, then 75/25 after a Diplomacy success on day 104), and the
     native trace carries 110 support drift and 7 support-change records.
     A second pass verified them and found: a regiment scrapped in transit
     kept its transit and landed as a dead key (now dropped,
     `forget_transit`); travelling squadrons cost maintenance but were no
     overdraft candidates (now `ScrapTarget::TravellingSquadron`,
     `FUN_00556430`); every economy record now comes from one function on
     both loops (`telemetry::economy_record`); bit 11 also refreshes after
     the battles. Scoped mutants on these functions: the survivors are
     closed by tests (the timer's delay rewrite, the exact drift brackets,
     the summary, the overdraft's side and maintenance filters in every
     family, a scrap that takes only the squadron it names) or are
     equivalent (the drift's fleet term after the early return, the
     timer's positive-delay guard, a zero uprising delta).
3. Run the gates (done 2026-10-16: all pass, and again after the
   fast-forward onto `origin/main`; traces leave seed 42 unchanged over
   1000 days):
   - `test --workspace`;
   - the ignored replay golden and `telemetry_coverage`;
   - `provenance-scan check`;
   - the two leak greps on `.beads/issues.jsonl` (worker ids, the private
     workspace name), both 0.
4. Report to Tom. On his go-ahead, make one commit:
   - stage explicit paths, with `git add -f` for the new `ghidra/notes/*.c`, `ghidra/notes/ai-production.md` and the moved, still-ignored `docs/plans/completed/2026-03-24-001-knesset-demiurge-ui-parity.md`; rebase onto `origin/main` (PR #45, `9533b516`, touches `fleet_window.rs` and `main.rs`) and rerun the gates first;
   - no Co-Authored-By trailer;
   - then push.
   - Done 2026-10-16 on Tom's go-ahead: main fast-forwarded to
     `9533b516`, the uncommitted work re-applied with main.rs,
     fleet_window.rs and strategic_windowing.rs resolved (the event-screen
     references dropped), the gates rerun, one commit pushed.

## Out of scope (carried queue)

Force fields, movie triggers, music selector steps B and C, and the
Planetary Bombardment submenu targets.
