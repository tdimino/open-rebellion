---
title: "Combat System"
description: "Space battles through the tactical engine (played, simulated or headless), ground combat in combat.rs, and bombardment.rs"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-16
tags: [combat, space-combat, ground-combat, simulation]
---

# Combat System

Every space battle runs the tactical engine, `rebellion-core::tactical`:
played in the battle window, simulated from the Battle Alert, or headless
when nobody watches (`FUN_0040a050` state 8,
`ghidra/notes/simulate-runs-tactical-engine.md`). `combat.rs` keeps ground
combat and the types both share; `bombardment.rs` is the port's bombardment.

## Trigger

Both sides present at a system (`side_present`, `FUN_00509710`; checked each tick in `main.rs` and `simulation.rs`). There is no cooldown: when both sides remain after a battle, no result is shown and the battle fires again the next tick (`FUN_0051ddf0` → `FUN_00517270`, `ghidra/notes/battle-result-text-and-cadence.md`).

## Types

| Type | Where | Purpose |
|------|-------|---------|
| `BattleSession` | `core::tactical` | One battle: ships, fighter groups, engagement state, the original's tactical rng |
| `BattleReturn` | `core::tactical::apply` | `summarize_results`: winner, survivors, withdrawals |
| `SystemSpaceCombatResult` | `data::integrator` | One headless battle: winner, winning fleet, `seconds`, both sides' forces before and after |
| `CombatForce` / `CombatSide` | `core::combat` | The lead fleet a side fights under; Attacker / Defender / Draw |
| `GroundCombatResult` / `TroopDamageEvent` | `core::combat` | Ground result; strength ≤ 0 means destroyed |
| `BombardmentSystem` | `core::bombardment` | `resolve_bombardment(world, fleet, system, difficulty, tick)` |

## API

```rust
// A battle nobody watches (simulation.rs, and main.rs when no window opens):
let result = resolve_system_space_combat(world, troop_transport, movement, system, rng_seed, tick);
// The Battle Alert's Simulate, or a played battle's end:
session.simulate_to_end();
let returned = apply_results(&session, world, troop_transport, movement, &mut log);
reconcile_death_star_result(world, death_star, victory);
// Ground and bombardment:
let ground = CombatSystem::resolve_ground(&world, sys, alliance_attacks, &rolls, tick);
let brd = BombardmentSystem::resolve_bombardment(&world, fleet, sys, difficulty, tick);
```

## Headless battles

`resolve_system_space_combat` builds one `BattleSession::new_with_forces`
per contested system, the Alliance as attacker, every fleet each side has
there plus the squadrons the system holds for it (`FUN_00516ef0`), seeded
from one roll of the tick's stream. It runs `simulate_to_end`, writes the
result back with `apply_results`, and reports `seconds` (the battle's
simulated length). The caller reconciles a destroyed Death Star. A result is
a `stalemate` only when it is a Draw with both sides still present.

## Simulate runs the tactical battle

The Battle Alert's Simulate (and Simulate Remainder) runs the tactical
session without its window (`BattleSession::simulate_to_end`, `FUN_0040a050`
state 8), then shows the normal Battle Results. `tactical::apply::apply_results`
writes hulls and squadrons back by `TacticalShip::fleet` /
`TacticalFighter::fleet`. Every fleet a side has at the system takes part. A
withdrawn ship survives and hyperspaces to the first system the retreat
chain accepts (`post_battle::retreat_destination`: the nearest of its side
not blockaded, else the nearest of its side); after Simulate the results
name that system for the player's fleet.
When only some of a fleet's survivors withdrew (a ship whose hyperdrive was
knocked out stays), the ones that withdrew leave as a new fleet with the
squadrons aboard them, and the rest hold the system (port: the original's
split is untraced).

The headless battle steps once a second (`headless_step`, `FUN_005ae9c0`).
Nothing moves and no battery fires. Each side in turn ranks its opponents,
assigns targets and applies every group's damage rate to its target
(`FUN_005d14d0`: `FUN_005d1a70`, `FUN_005c9200`, `FUN_005cb650`,
`FUN_005b54d0`), then recharges shields. A unit's rate
(`original_unit_damage_rate`, `FUN_005b7780`) sums its strongest arc's weapon
families against the target's class, times its weapon recharge, over the arc's
weapon count. Ions do nothing to fighters, and fighters add torpedoes against
an unshielded capital ship.

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
weaker side loses (`stalemate_resolve_side`). Its hyperdrive ships retreat and
dock what squadrons their carriers can berth; everything else is destroyed. A
side with no working hyperdrive is destroyed outright. While a Death Star is
operational (`FUN_005ba420`), the side facing it counts -1 at the decision,
so that side always leaves and the Death Star's side never does.

The result lines follow `FUN_0049b330` (won), `FUN_0049b4d0` (lost) and
`FUN_0049b150` (draw). They compare the system's holder with the side that
held a blockade there before the battle: cleared, broke the blockade,
maintained it, defended, or now under blockade by the winner.

## Bombardment

The original's bombardment (orders `0x220`-`0x223`, `FUN_00582440`,
`FUN_0058d800`) fires defence batteries at the bombarding ships, rolls each
hit against a target's value, destroys facilities, regiments and the HQ, and
shifts support (`ghidra/notes/bombardment.md`). The port's `bombardment.rs`
instead computes one number from the transit-distance formula
(`FUN_0055d8c0`, `ghidra/notes/build-delivery.md`) for the headquarters check
and the records. The support shifts are the original's finalize
(`bombardment::finalize`, `FUN_0058e320`); the port rolls no hits, so a
bombardment shifts none and a Death Star kill shifts every sector. Task-graph
node `or-port-bombardment-8usw` replaces the rest.

## Ground combat

`CombatSystem::resolve_ground` cites `FUN_00560d50` and `FUN_004ee350`.
`FUN_00560d50` is the character manager's object-move hook (gates on capital ships `0x14`-`0x1b` and characters
`0x30`-`0x3b`; `0x34` routes to the Emperor-at-Coruscant flag
`FUN_005617b0`), and `FUN_004ee350` sets a character's CommandKind `+0x96`
(`ghidra/notes/ground-unit-assets.md`, `character-roster.md`). Where the
original resolves regiment against regiment is open (`or-q-regiment-combat-eaxn`);
`or-p-gnd-ykv` ports the original's report-only ground assault.

## Effect Application

- Space: `core::tactical::apply::apply_results` writes hulls and squadrons back by `TacticalShip::fleet` / `TacticalFighter::fleet`, moves withdrawn ships, removes destroyed units and empty fleets, then runs the post-battle pass (`core::post_battle`, `FUN_00516ef0`): the loser's fleets and system squadrons stay at a system of their own or leave for the chain's first accepted place, and are destroyed when none accepts them (`ghidra/notes/system-squadrons-combat.md`).
- Ground: `apply_ground_combat_result_inner` (integrator) updates `TroopUnit.regiment_strength` and removes destroyed troops.

## Source

- `ghidra/notes/simulate-runs-tactical-engine.md` — Simulate and the stalemate
- `ghidra/notes/tactical-engagement-ai.md` — targets, steering, the band
- `ghidra/notes/ground-unit-assets.md` — what `FUN_00560d50` is, regiment family gates
- `ghidra/notes/bombardment.md` — orders, defence fire, hit rolls, support shifts
- `ghidra/notes/combat-formulas.md` — GNPRTB parameter mapping
