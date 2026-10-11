---
title: "Combat System"
description: "Space and ground combat resolution via combat.rs and bombardment.rs"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-10
tags: [combat, space-combat, ground-combat, simulation]
---

# Combat System

`combat.rs` + `bombardment.rs` — Space and ground combat resolution.

## Trigger

Both sides present at a system (checked each tick in `main.rs` and `simulation.rs`). There is no cooldown: when both sides remain after a battle, no result is shown and the battle fires again the next tick (`FUN_0051ddf0` → `FUN_00517270`, `ghidra/notes/battle-result-text-and-cadence.md`).

## Types

| Type | Purpose |
|------|---------|
| `CombatSystem` | Stateless resolver — `resolve_space()` and `resolve_ground()` |
| `SpaceCombatResult` | `{ winner, attacker_fleet, defender_fleet, ship_damage, fighter_losses }` |
| `GroundCombatResult` | `{ winner, system, troop_damage }` |
| `ShipDamageEvent` | `{ fleet, ship_index, hull_after }` — hull=0 means destroyed |
| `FighterLossEvent` | `{ fleet, fighter_index, squads_before, squads_after }` |
| `TroopDamageEvent` | `{ troop, strength_after }` — strength ≤ 0 means destroyed |
| `CombatPhaseFlags` | Custom bitfield (NOT `bitflags!`) — uses `.set()` not `.insert()` |
| `CombatSide` | Attacker / Defender / Draw |
| `BombardmentSystem` | `resolve_bombardment(world, fleet, system, difficulty, tick)` |

## API

```rust
let result = CombatSystem::resolve_space(&world, atk, def, sys, difficulty, &rolls, tick);
let ground = CombatSystem::resolve_ground(&world, sys, alliance_attacks, &rolls, tick);
let brd = BombardmentSystem::resolve_bombardment(&world, fleet, sys, difficulty, tick);
```

## Space Combat: 7-Phase Pipeline

From Ghidra RE of REBEXE.EXE:

1. **Weapon fire** — each ship fires its class's turbolaser, laser and ion attack strengths (each already the total over the four arcs in CAPSHPSD), damage rolls
2. **Shield absorption** — shield_strength absorbs damage
3. **Hull damage** — remaining damage reduces hull_current
4. **Fighter engagement** — squadron vs squadron, torpedo bombing
5. **Result** — winner determined by surviving fleet strength

Each side fights with every fleet it has at the system plus the squadrons
the system holds for it (`FUN_00516ef0` marks all of them); `CombatForce`
names the lead fleet the result and Battle Results show. A side survives a
pass on a living ship, a held squadron, or a carried squadron whose ship
lives. `apply_space_combat_result_inner` resolves every event to its roster
slot before applying any. The strategic pass fights pass after pass until
a side is gone or a pass changes nothing.

> **Corrected 2026-10-10:** the original's seven "phases" only set flags and
> notify observers; damage comes from the tactical engine
> (`ghidra/notes/simulate-runs-tactical-engine.md`).

## Simulate runs the tactical battle

The Battle Alert's Simulate (and Simulate Remainder) runs the tactical
session without its window (`BattleSession::simulate_to_end`, `FUN_0040a050`
state 8), then shows the normal Battle Results. `tactical_flow::apply_results`
writes hulls and squadrons back by `TacticalShip::fleet` /
`TacticalFighter::fleet`. Every fleet a side has at the system takes part. A
withdrawn ship survives and hyperspaces to the nearest friendly or neutral
system; after Simulate the results name that system for the player's fleet.
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

Key gotchas from RE:
- Phase gate: `ACTIVE && !PHASES_ENABLED` (inverted from initial expectation)
- Alt-shield path: exact `== 0x71` family byte (not range `0x71..=0x72`)
- Fighter losses must be computed from initial vs surviving counts

## Bombardment

The bombardment formula is in FUN_0055d8c0 (damage formula) and FUN_0055d860 (Euclidean distance): `damage = sqrt((atk-def)²) / GNPRTB[0x1400]` with difficulty modifier, minimum 1 damage (only after division, not on zero raw_power). FUN_005617b0 is the CharacterMgr SeatOfPower check, not a bombardment function.

## Effect Application (main.rs)

- `apply_space_combat_result`: Maps destroyed hulls (hull_after == 0) to fleet `capital_ships` count decrements. Removes empty fleets from system fleet lists.
- `apply_ground_combat_result`: Updates `TroopUnit.regiment_strength`. Removes destroyed troops from system and arena.

## Source

- `ghidra/notes/space-combat.md` — the flag pipeline (see its correction)
- `ghidra/notes/simulate-runs-tactical-engine.md` — Simulate and the stalemate
- `ghidra/notes/tactical-engagement-ai.md` — targets, steering, the band
- `ghidra/notes/ground-combat.md` — troop iteration
- `ghidra/notes/bombardment.md` — Euclidean distance formula
- `ghidra/notes/combat-formulas.md` — GNPRTB parameter mapping
