---
title: "Death Star System"
description: "The Death Star as a shipyard product, its one-per-system cap, Sabotage target, Destroy System, sector bit 11 and the tactical station"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-17
tags: [death-star, product, planet-destruction, tactical, simulation]
---

# Death Star System

`death_star.rs`: sector bit 11, the destroy cascade and the adrift sweep.
The product rules live with the orders that apply them.

## The product

The Death Star is the capital ship of family `0x18`-`0x1b` (CAPSHPSD record
136, Empire only), built as any shipyard product. A fleet holds it while one
of its live ships is of that family (`Fleet::has_death_star`,
`FUN_005003d0`); no state is stored beside the ship.

- An ordered capital ship exists from its order, unfinished, in its side's
  bit-1 spare fleet at its destination or in its destination fleet
  (`ShipInstance::completed`, `GameWorld::spare_fleets`, `FUN_005097d0`,
  `FUN_0050bc60`); its yard completes it in place. A ship whose fleet lies
  in another system, or is in hyperspace, then flies its own leg to that
  fleet's destination (`FUN_005150c0.c:90-102` -> `FUN_00556430`;
  `manufacturing::finish_ships`, `movement::listed_location`). An unfinished
  ship may change fleets by Move, but Create Fleet refuses it
  (`FUN_004f9860.c:45-50`, `FleetOrderError::Unfinished`); it neither
  bombards nor draws battery fire, and adds nothing to a blockade's
  strength sums.
- One per system (`FUN_00507ab0`): Build, the Destination order, Move and
  the retreat candidate test count the Death Stars standing at the system
  or in fleets bound there, not those in fleets ordered away, other yards'
  queued ones included (`movement::death_stars_counted_at`;
  `FUN_00551a20`, `FUN_00551900`, `FUN_00551630`, `FUN_005571c0`, all
  through `FUN_00552dd0` -> `FUN_00550fb0`). Build and Destination leave out
  their own area's products, which the Build replaces in the same fleet
  (`FUN_00552210`; `build_selection::order_refusal`,
  `build_selection::destination_refusal`,
  `manufacturing::seat_ordered_ships`). A refusal
  is status `{0x90, 3}`, with no text: the Empire's advisor has a line
  for it (slot `+0x2c`), the Alliance's gives its general refusal.
- When a yard's system changes holder, its manager changes side, and every
  product it lists dies with reason `0x16` and no refund, wherever the
  product sits (`FUN_004f8680.c:28-31` -> `FUN_004f6f40` -> manager `+0xf0`
  `FUN_0052b790` -> `FUN_0052a2c0`). An emptied fleet then disbands
  (`support_shift`, `manufacturing`). Other yards' ships in fleets at the
  system are untouched (`ghidra/notes/facility-ownership.md`, "Control
  change").
- A squadron or regiment built for a fleet takes the room of a ship of that
  fleet from the order (`QueueItem::carrier`, chosen by
  `carriage::seat_products`; `FUN_005385f0` -> `FUN_00551a20` ->
  `FUN_00552300`), and the room counts it while queued and travelling
  (`carriage::ProductRoom`). With no such room the order is refused
  `{1, 0x27}` (`build_selection::order_refusal`, `OrderRefusal::NoRoom`).
- Build refuses `{0xf3, 1}` when the units' maintenance passes the side's
  spare (`FUN_0052e970`).
- Sabotage and Death Star Sabotage name their object
  (`MissionTarget::DeathStar` for the fleet); an uncompleted Death Star is
  no target (`FUN_00593500`). The shipped AI never plans Death Star
  Sabotage (`FUN_004e8290.c:90-99` asks for it only from an Alliance with
  its own Death Star hull).
- A sabotaged Death Star plays movie `0x72` (MDATA.104) to both sides,
  caption TEXTSTRA `0x71be` with the system's name
  (`strike_orders::sabotage_movies`, `FUN_0048d470`).

## Tactical station

A Death Star in a battle is the station, not a capital ship
(`BattleSession::death_star`, `FUN_0040b0e0`): hull from its ship, charge
100.0, the system's Death Star Shield, its hangar. A Death Star at the head
of a queue fights as an uncompleted station (`seat_unfinished_station`,
`FUN_0040b0e0.c:184-203`). The superlaser, the trench run, the hangar and
their headless forms are in `combat.md`; the ship takes the station's
remaining hull afterwards (`FUN_0040d170`).

## Destroy System

Bombardment order `0x223` (`integrator::order_strike`; `combat.md`,
"Bombardment"). The gate needs a fleet with bombard strength and a live ship
of family `0x18`-`0x1b`; the only target refusal is a destroyed system. The
kill runs `cleanup_destroyed_system`, and `sweep_adrift` moves the characters
it sets adrift the same day, after the character roll; one with no system of
its side to go to is captured (`systems/captivity.md`). The kill plays movie
`0x6f` (MDATA.101), caption `0x70c9` (`0x70ca` for a neutral system), to
the bombarder and the holder (`strike_orders::strike_movies`,
`FUN_004950d0`); like every kind-6 message it files no Message Index row
and no advice.

## Sector bit 11

`death_star::refresh_sectors` sets `GameWorld::death_star_sectors`, system
`+0x88` bit 11: a usable Death Star (complete, not en route) in the sector
(`FUN_005178b0`). It reruns wherever a Death Star's usable bit changes
(event `0x301`, `FUN_00514510.c:149-158`): its completion, a fleet's
departure and arrival, Send Ships (`fleet_join`), and its loss in battle, to
batteries, to scrap or to sabotage; and after a load or a new game. It is
not saved. Readers: the garrison and Incite halving
(`uprising.md`) and the support-change halving (`blockade.md`). A kill ends
with the bombardment finalize (`FUN_0058e320`, `combat.md`, "Bombardment").

## Source

- `ghidra/notes/death-star-strategic.md`
- `ghidra/notes/bombardment.md`: the Destroy System run
- `ghidra/notes/death-star-product.md`: product, cap, placement, refusals
- `ghidra/notes/tactical-death-star.md`: the station and the trench run
