---
title: "Combat System"
description: "Space battles through the tactical engine (played, simulated or headless), the Planetary Assault in assault.rs, and bombardment.rs"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-17
tags: [combat, space-combat, ground-combat, simulation]
---

# Combat System

Every space battle runs the tactical engine, `rebellion-core::tactical`:
played in the battle window, simulated from the Battle Alert, or headless
when nobody watches (`FUN_0040a050` state 8,
`ghidra/notes/simulate-runs-tactical-engine.md`). `combat.rs` keeps the
types the battles share; `assault.rs` is the Planetary Assault, the only
ground combat; `bombardment.rs` is the bombardment.

## Trigger

Both sides present at a system (`side_present`, `FUN_00509710`; checked each tick in `main.rs` and `simulation.rs`). A side is present through a usable fleet (`Fleet::usable`, `FUN_004fe540`: one live capital ship not on its own leg) or squadrons the system holds for it. There is no cooldown: when both sides remain after a battle, no result is shown and the battle fires again the next tick (`FUN_0051ddf0` → `FUN_00517270`, `ghidra/notes/battle-result-text-and-cadence.md`).

## Types

| Type | Where | Purpose |
|------|-------|---------|
| `BattleSession` | `core::tactical` | One battle: ships, fighter groups, engagement state, the original's tactical rng |
| `BattleReturn` | `core::tactical::apply` | `summarize_results`: winner, survivors, where the pass sent the player's fleet |
| `SystemSpaceCombatResult` | `data::integrator` | One headless battle: winner, winning fleet, `seconds`, both sides' forces before and after |
| `CombatForce` / `CombatSide` | `core::combat` | The lead fleet a side fights under; Attacker / Defender / Draw |
| `fighting_fleets` | `core::combat` | A side's usable fleets at a system in container order; the lead is the first |
| `AssaultReport` / `AssaultRefusal` | `core::assault` | `order_enabled`, `resolve`; `integrator::order_assault` resolves an order |
| `StrikeReport` / `BombardRefusal` | `core::bombardment` | `gate`, `open`, `bombard`, `finalize`; `integrator::order_strike` resolves an order |

## API

```rust
// A battle nobody watches (simulation.rs, and main.rs when no window opens):
let result = resolve_system_space_combat(world, troop_transport, movement, system, rng_seed, tick);
// The Battle Alert's Simulate, or a played battle's end:
session.simulate_to_end();
let returned = apply_results(&session, world, troop_transport, movement, &mut log);
// Assault and bombardment:
let runs = integrator::order_assault(world, &fleets, side, &mut states, &mut roll)?; // Vec<AssaultEffects>
let runs = integrator::order_strike(world, &fleets, kind, &mut states, &mut roll)?; // Vec<StrikeEffects>
```

## Headless battles

`resolve_system_space_combat` builds one `BattleSession::new_with_forces`
per contested system, the Alliance as attacker, every fleet each side has
there plus the squadrons the system holds for it (`FUN_00516ef0`), seeded
from one roll of the tick's stream. It runs `simulate_to_end`, writes the
result back with `apply_results`, and reports `seconds` (the battle's
simulated length). The Death Star ship takes the station's remaining hull
(`FUN_0040d170`), so a station destroyed in battle destroys its ship. A
result is a `stalemate` only when it is a Draw with both sides still
present.

## Simulate runs the tactical battle

The Battle Alert's Simulate (and Simulate Remainder) runs the tactical
session without its window (`BattleSession::simulate_to_end`, `FUN_0040a050`
state 8), then shows the normal Battle Results. `tactical::apply::apply_results`
writes hulls and squadrons back by `TacticalShip::fleet` /
`TacticalFighter::fleet`. Every fleet a side has at the system takes part.
The write-back moves no ship or fleet (`FUN_0040c780`): a ship that jumped
out survives in its fleet at the system, with the squadrons that docked in
it at the end. The only departure is the post-battle pass
(`FUN_00516ef0.c:142-180`): every usable fleet (mode 1, so not one whose
ships are all on their own legs) and every system squadron of the loser
takes the first place the retreat chain accepts, and never keeps a fleet in
its own system (VM: losers at their own held Carida, Coruscant and Ando
ended at Halowan, Ando and Halowan, the Wine run's loser that never withdrew
included, arriving three days later): the nearest system of its side not
blockaded, else the nearest of its side, else the nearest neutral system,
else the nearest enemy-held one (`FUN_005551e0`, `FUN_005552e0`), and a fleet
is destroyed only when none accepts it. A
winner that jumped stays. The withdrawal's own retreat system (permission
and countdown, `post_battle::retreat_destination`) runs the chain without
stage 1 and never ends on the battle system: the VM's Imperial fleet
withdrew from the Imperial-held Carida to Halowan after a countdown
(`ghidra/notes/morale-retreat.md`). After Simulate the results name the system the pass sent the
player's fleet to.

A battle ends when a side has no unit left fighting (`FUN_005a9ec0.c:169-173`
counts the units in flight, in their jump or just left; docked squadrons
count for nothing). A side alone at nothing loses. When both reach nothing
the side that withdrew wins (`FUN_005a9ec0.c:287-290`,
`FUN_0040c780.c:82-93`); when both withdrew, the side that did not arrive
last at the system wins (`FUN_0040c780.c:95-101`, the system's `+0x78`
holder: an Alliance holder gives the Empire, an Imperial holder or none the
Alliance); when neither did, no winner is recorded and the post-battle pass
takes the holder (`post_battle::pass_winner`). A side has withdrawn
(`BattleSession::side_withdrew`, `FUN_005a9ec0.c:245-273`) when none of its
units is alive or in its jump, and a unit of it jumped out or, for the
Empire, its station departed. Every jump happens under a side's withdrawal
(Battle Options Withdraw, the computer's decision, the stalemate) or the
Battle Alert's Retreat; the original has no per-ship retreat command. The
outcome voice follows each side's lost and withdrawn flags, not the winner
(`FUN_005a9ec0.c:291-328`).

The headless battle steps once a second (`headless_step`, `FUN_005ae9c0`).
Nothing moves and no battery fires. Each side in turn ranks its opponents,
assigns targets and applies every group's damage rate to its target
(`FUN_005d14d0`: `FUN_005d1a70`, `FUN_005c9200`, `FUN_005cb650`,
`FUN_005b54d0`), then recharges shields. A unit's rate
(`original_unit_damage_rate`, `FUN_005b7780`) sums its strongest arc's weapon
families against the target's class, times its weapon recharge, over the arc's
weapon count. Ions do nothing to fighters, and fighters add torpedoes against
an unshielded capital ship.

Each side's step opens with its Death Star branch (`FUN_005d14d0.c:20-60`,
`headless_death_star`). The Empire's station, charged to exactly 100.0,
destroys the first capital ship of its ranked opponents and its hangar
outright; it starts full and refills by a millisecond's worth a step, so it
fires once a battle. The Alliance, while the station is attackable and its
listed squadrons outnumber the Empire's, sends its first fighter group on
the headless trench run (`FUN_005aefc0`, `headless_trench_run`): a walk round
the group, at most 100 visits, after which every member left is destroyed;
a success zeroes the station and destroys the squadrons in its hangar. The
shield-first damage and the trench run draw from the headless battle's own
Lehmer generator (`OriginalBattleRng`, `FUN_005af420`), which runs on from
battle to battle on `GameWorld::battle_rng` and is not saved.

Squadrons without a hyperdrive start docked in their carrier, or the
station (`seat_carried_squadrons`, `FUN_005964c0.c:133-219`), before the
layout, which gives a docked squadron no slot and no share of the battle
extent but groups it with its class (`FUN_005ab650.c:148-153`, `:297-406`).
In a played battle each squadron docking into an unarmed hangar arms it at
5 + rand(0,8) past step 3 (`FUN_005b8570.c:13-30`, `arm_docked_hangars`,
run at Take Command); the battle then launches one per carrier each time
the carrier's armed step comes up (`FUN_005b8800`: 5 + rand(0,8) steps of
1000 ms); the station's hangar runs only for a local Empire. Headless, the Empire's first step launches every
docked squadron (`FUN_005d14d0.c:62-82`). At the end each squadron docks in
the first ship of its side with room, else the Empire's station
(`FUN_005bef90`, `end_berths`), and the write-back makes it a child of that
ship's fleet or of the system (`FUN_0040d320`, `apply::squadron_homes`).

A played battle steers instead (`advance_original_engagement`,
`ghidra/notes/tactical-engagement-ai.md`). Both sides' task forces and fighter
groups without orders pick a target by their tactic and close to their weapon
band:

- Surround sends every force at the first target.
- Stand Off commits forces until their combined kill time is 180 s or
  less. Forces with laser cannons rank fighters first.
- A force closes until distance + 2 falls inside the interval where its net
  strength over the target is greatest, and backs off when it is too close.

Player maneuvers, routes, escorts and recoveries keep control of their units.
In a played battle only the player's units take subsystem hits
(`FUN_005c0170` sets `+0x124` on the computer's units, and the damage
receivers skip the roll; `original_windowed_damage_rolls`).

After 1200 s with no new low in either side's strength (`FUN_005d0ce0`), the
weaker side loses (`stalemate_resolve_side`, `FUN_005afe40`). Its squadrons
go first: one with a hyperdrive jumps, any other docks in its own carrier,
else the first ship with room, else (Empire) the operational station, and
is destroyed otherwise; then its hyperdrive ships retreat and the rest are
destroyed. A
side with no working hyperdrive is destroyed outright. While a Death Star is
operational (`FUN_005ba420`), the side facing it counts -1 at the decision,
so that side always leaves and the Death Star's side never does.

The result lines follow `FUN_0049b330` (won), `FUN_0049b4d0` (lost) and
`FUN_0049b150` (draw). They compare the system's holder with the side that
held a blockade there before the battle: cleared, broke the blockade,
maintained it, defended, or now under blockade by the winner.

## Battle Summary

Every battle, played or simulated, ends in the Battle Summary (window kind
8, `FUN_0046f140`; `ghidra/notes/tactical-results.md`), over the command
center. Its rail holds Close, Battle Summary, Alliance Forces, Imperial
Forces and Display. A side's forces view shows "Filters" and four icon
tabs: Capital Ships (families `[0x14,0x1c)`, the Death Star's `0x18`
among them), Fighter Squadrons, Trooper Regiments and Personnel. The first
three list Operational and Destroyed, Personnel Survivors, Captured and
Killed. There is no Damaged state; damage shows as the picture's burn
marks (`+0x50` bit `0x200`: a ship short of its class hull or with a hit
system, `FUN_00501c00`; a squadron that lost craft, `FUN_00503340`).
`core::battle_summary` fills the lists from the Battle Alert's record
of each side's fleets, taken on the battle's first frame (`BattleRecord`):

- A ship is destroyed only with no hull left (`FUN_0040cfc0`); one that
  withdrew is Operational. The Death Star is destroyed when its station's
  hull truncates to 0 (`FUN_0040d170`).
- A squadron is destroyed at craft count 0. The live count is
  `max(1, trunc(hull / 8))`, 0 only once the squadron is gone
  (`FUN_005b8e00`, `TacticalFighter::refresh_craft_count`), so a docked or
  jumped squadron stays Operational.
- A regiment dies with the ship that carries it, by the ship's tag
  (`FUN_004f8710`).
- A lost ship's hook (`FUN_00502840`) sets its characters adrift and
  wounds each with chance `max(G2568, 100 - combat)`; a wounded minor dies
  on G2577 (25%), a major never (`battle_summary::crew_fates`, applied first
  in `apply_results`). The battle takes no one captive: an emptied fleet's
  characters stay adrift at the system for the adrift sweep. The window
  files the dead as Killed, a captive aboard a living ship as Captured, the
  rest aboard as Survivors, and the adrift in no column (`FUN_00472010`
  walks existing ships). port: the rolls draw the battle's stream; the
  wound is not stored (F-026).

## Bombardment

Bombardment is the original's (orders `0x220`-`0x223`, `ghidra/notes/bombardment.md`):
the gate `FUN_00582440`, then the run `FUN_0058d800`. In the run, the defence
batteries fire at the bombarding ships, each hit is rolled against a target's
value, and facilities, regiments and the HQ are destroyed. The finalize
`FUN_0058e320` then shifts support. Either side orders it from the object
menu (`integrator::order_strike`), and the AI through
`integrator::resolve_ai_strike` (`bombardment.rs`). Destroy System
(`0x223`) needs a Death Star ship and runs the destroy cascade
(`ghidra/notes/death-star-strategic.md`).

## Ground combat: the Planetary Assault

Regiments fight only on the Planetary Assault order `0x234`
(`ghidra/notes/ground-assault.md`); nothing lands a regiment or fights on
the ground by itself, before or after a space battle. The order is enabled
(`assault::order_enabled`, `FUN_005814d0`) for a team of the player's
fleets at one holder's systems, each blockading (fleet `+0x58` bit `0x20`,
the system's blockade bit), with a regiment aboard, at a system not
destroyed and holding fewer than GNPRTB `0x1e03` (2) shield generators of
either side. A neutral system is never blockaded, so never assaulted.

The run (`assault::resolve`, `FUN_00563390` -> `FUN_0058c120`) has no
rounds:

1. Count the attackers (the flagged fleets' ships' regiments, in fleet,
   ship, regiment order) and the defenders (the holder's regiments at the
   system).
2. Each defence facility of either side fires with chance attack / GNPRTB
   `0x200` percent at a random attacker index over the starting count; a
   KDY or LNR always fires, a shield generator never.
3. Each attacker picks a random defender index over the starting count and
   on a hit rolls `rand[0, 10] + attack + General/30 - defence - General/30`:
   4 or less kills the attacker, 6 or more the defender.
4. Each engagement rolls an 8 percent collateral hit; each hit destroys a
   facility (never the HQ) or takes one energy or raw material.
5. With attackers left and no defender, up to six regiments land.

The draws follow the original's ledger (`ghidra/notes/ground-assault.md`,
"Draw ledger"): defence fire per facility, then the engagements, then the
collateral rolls, each from one generator. The original's generator holds
its state within a day, so repeated orders resolved under one state repeat
their outcome. port: `assault::resolve` seeds a stream from the caller's
one roll and does not carry the original's state between orders.

Each dead regiment is reason `0xe` in the blockade loss counter A. The
holder is then rebuilt from the regiments left (`FUN_0050b610`, as each
kill's and landing's event reaches `FUN_00508660`), with the facilities'
hand-over and the change's support shift. A holder change frees the old
holder's prisoners of the new side, sends its other characters there to the
nearest system of their side (`FUN_004ef260`, event `0x307`;
`death_star::sweep_requests`) and turns or removes its special forces by
class (`FUN_00503d90`). The player issues it from the fleet's menu
(`assault_orders::issue`), the AI through `integrator::resolve_ai_assault`;
the Operation Report (`0x7090`-`0x7094`) goes to both sides.

## Effect Application

- Space: `core::tactical::apply::apply_results` writes hulls and squadrons back by `TacticalShip::fleet` / `TacticalFighter::fleet` (moving no ship or fleet), removes destroyed units and empty fleets, then runs the post-battle pass (`core::post_battle`, `FUN_00516ef0`): the loser's fleets and system squadrons stay at a system of their own or leave for the chain's first accepted place, and are destroyed when none accepts them (`ghidra/notes/system-squadrons-combat.md`).
- Operation Report: a damaged survivor is listed under Operational with the damage art over its portrait; only a destroyed object moves to Destroyed (`ghidra/notes/operation-report-window.md`).
- Assault: `integrator::resolve_assault` records each kill's loss, rebuilds the holder and fires the shifts; a regiment has no strength, so a kill removes it whole.

## Source

- `ghidra/notes/simulate-runs-tactical-engine.md` — Simulate and the stalemate
- `ghidra/notes/tactical-engagement-ai.md` — targets, steering, the band
- `ghidra/notes/tactical-results.md` — the Battle Summary's tabs, lists and rules
- `ghidra/notes/ground-assault.md` — the assault order, run, rolls and control change
- `ghidra/notes/ground-unit-assets.md` — what `FUN_00560d50` is, regiment family gates
- `ghidra/notes/bombardment.md` — orders, defence fire, hit rolls, support shifts
- `ghidra/notes/combat-formulas.md` — GNPRTB parameter mapping
