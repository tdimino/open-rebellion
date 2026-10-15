---
title: "Death Star"
description: "The Death Star as CAPSHPSD family 0x18: build, move, Destroy System, the destroy cascade, the sector bit and victory; and where the port's death_star.rs diverges"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-14
game_system: "death-star"
sources:
  - type: "ghidra"
    file: "ghidra/notes/death-star-strategic.md"
  - type: "ghidra"
    file: "ghidra/notes/bombardment.md"
  - type: "code"
    file: "crates/rebellion-core/src/death_star.rs"
related:
  - "victory"
  - "bombardment"
tags: ["death-star", "construction", "destruction", "victory"]
---

# Death Star

The Death Star is an ordinary capital ship whose `CAPSHPSD` record (136)
has family `0x18` instead of `0x14`. Every rule about it keys on the ship
family range `[0x18,0x1c)`; family `0x34` is the Emperor's character family.
Full trace: [death-star-strategic.md](../../ghidra/notes/death-star-strategic.md).

## What You See

- Every Empire shipyard lists the Death Star from the first day (cost 584,
  maintenance 600, no research gate). It builds and travels like any other
  capital ship, at its class hyperdrive.
- Only one Death Star can stand at a system.
- A fleet carrying it gets the Destroy System order. Firing destroys the
  target system; the only refused target is one already destroyed.
- Every system in the Death Star's sector carries a "Death Star in the
  sector" bit, which feeds the support shifts and uprisings.
- A successful Death Star Sabotage mission destroys it. Losing it ends
  nothing.

## How It Works (original)

### Fire: Destroy System (`0x223`)

The gate `FUN_00582440` needs, in one of the team's fleets, a nonzero
bombard strength and a ship in `[0x18,0x1c)`. It tests no side, shield,
charge or target owner. The kill phase (`FUN_0058d440`, `FUN_0058ce70`)
destroys the system with reason `0x10` when the bombarder holds it, else
`0xf`. Phases and defence fire: [bombardment.md](bombardment.md).

### What destroying a system does

| At the system | Result |
|---|---|
| Fleets of either side, the Death Star's own included | Survive with their ships, fighters, regiments and characters |
| Completed regiments, capital ships, fighters, the HQ, facilities not in transit | Destroyed; uncompleted ones get the autoscrap request |
| Special forces | Destroyed unless in transit |
| Characters at the system, not on a mission | Turn adrift |
| Minor characters not in transit | Rolled against GNPRTB `0x606`; a hit kills |

The owner stays. Victory reads the Death Star only through the AD bit
(Alliance HQ missing or destroyed) and the CD bit (Coruscant destroyed).

## Port

`crates/rebellion-core/src/death_star.rs` diverges from the original:

- **Identification.** `CapitalShipClass::is_death_star`
  (`world/mod.rs`) also accepts family `0x34`, and `Fleet::has_death_star`
  is a stored flag; the original computes it from the fleet's ships in
  `[0x18,0x1c)` (`or-port-death-star-family-qtde`).
- **Destroy cascade.** `cleanup_destroyed_system` removes every fleet at
  the system, the Death Star's own included, and kills every character
  aboard (`or-port-destroy-system-cascade-imrv`).
- **Construction.** A separate countdown of 1,825 ticks
  (`DEATH_STAR_CONSTRUCTION_TICKS`) has no source; the original builds the
  ship on the ordinary build path from its DAT build time.
- **Fire.** `DeathStarSystem::fire` refuses an Empire-held target and
  needs an Empire fleet; the original allows any target not already
  destroyed and tests no side.
- **Nearby warning.** Alliance systems within 300 coordinate units
  (`NEARBY_WARNING_RADIUS`) are warned; the original marks every system in
  the sector instead (bit 11, `FUN_005178b0`; `or-q-port-support-shifts-y0y`).

## Source Material

- **Ghidra RE**: [death-star-strategic.md](../../ghidra/notes/death-star-strategic.md), [bombardment.md](../../ghidra/notes/bombardment.md), [tactical-death-star-path.md](../../ghidra/notes/tactical-death-star-path.md) (the battle)
- **Code**: `crates/rebellion-core/src/death_star.rs` — `DeathStarSystem`, `DeathStarState`, `DeathStarConstruction`

## Related

- [victory.md](victory.md) — implemented faction-specific victory contract
- [Bombardment](bombardment.md)
- [official campaign contract](../reference/campaign-history/official-campaign-contract.md#standard-victory) — source-backed behavior
