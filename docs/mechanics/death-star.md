---
title: "Death Star"
description: "The Death Star as CAPSHPSD family 0x18: build, move, Destroy System, the destroy cascade, the sector bit and victory; and where the port's death_star.rs diverges"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-17
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

No owner field is written; the populated bit clears, so the control word
turns neutral (`support-shifts.md`, "Destroy System's kill"). Victory reads the Death Star only through the AD bit
(Alliance HQ missing or destroyed) and the CD bit (Coruscant destroyed).

## Port

`crates/rebellion-core/src/death_star.rs` and the bombardment run:

- **Destroy System** is order `0x223` and runs the original cascade
  (`cleanup_destroyed_system`); the gate tests no side
  (`ghidra/notes/bombardment.md`, Port).
- **Construction.** The Death Star is a family-`0x18` capital ship on the
  ordinary build path; Build Selection refuses it where one already is or
  is queued (`{0x90, 3}`, `FUN_00507ab0`; `build_selection.rs`).
- **The sector bit.** `refresh_sectors` (`FUN_005178b0`) marks every
  system in a sector holding a usable Death Star
  (`GameWorld::death_star_sectors`, bit 11).
- **Adrift characters.** `sweep_battle_changes` and `sweep_adrift`
  (`FUN_00548e90`, `FUN_00549230`) send the cascade's adrift characters on
  hidden trips by side; one with no place of its side is captured and
  tries the captor's places, and with none again is held in the captor's
  jail (`strand`, `hold_in_jail`, `FUN_00549580`).

## Source Material

- **Ghidra RE**: [death-star-strategic.md](../../ghidra/notes/death-star-strategic.md), [bombardment.md](../../ghidra/notes/bombardment.md), [tactical-death-star-path.md](../../ghidra/notes/tactical-death-star-path.md) (the battle)
- **Code**: `crates/rebellion-core/src/death_star.rs` — `refresh_sectors`, `cleanup_destroyed_system`, `sweep_battle_changes`, `sweep_adrift`, `return_members`, `free_escapee`, `hold_in_jail`

## Related

- [victory.md](victory.md) — implemented faction-specific victory contract
- [Bombardment](bombardment.md)
- [official campaign contract](../reference/campaign-history/official-campaign-contract.md#standard-victory) — source-backed behavior
