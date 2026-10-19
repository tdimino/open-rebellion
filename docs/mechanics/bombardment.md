---
title: "Orbital Bombardment"
description: "Bombardment orders 0x220-0x223: defence fire, per-hit target rolls, support shifts"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-17
game_system: "bombardment"
sources:
  - type: "ghidra"
    file: "ghidra/notes/bombardment.md"
  - type: "ghidra"
    file: "ghidra/notes/support-shifts.md"
  - type: "code"
    file: "crates/rebellion-core/src/bombardment.rs"
  - type: "prima"
    page: "Chapter 7: Fleet Combat"
related:
  - "space-combat"
  - "gnprtb"
tags: ["combat", "bombardment", "orbital", "death-star"]
---

# Orbital Bombardment

## What You See

A fleet in orbit can bombard a system with one of four orders: Target
Military Facilities, Target Civilian Facilities, General Bombardment, or
Destroy System. The system's planetary batteries fire back, each hit may
destroy a facility, a regiment or a point of energy or raw material, and the
system's support for the bombarding side falls. Only a Death Star can
Destroy System.

## How It Works

### The orders

Orders `0x220`-`0x223` share one gate (`FUN_00582440`): every bombarding
fleet needs a nonzero bombard strength, and Destroy System also needs a
Death Star-family ship. `FUN_0058d800` sets the target classes:

| Order | Military | Civilian | Energy, raw material | Collateral | Destroy System |
|---|---|---|---|---|---|
| Target Military Facilities | yes | | | yes | |
| Target Civilian Facilities | | yes | | | |
| General Bombardment | yes | yes | yes | | |
| Destroy System | yes | yes | yes | | yes |

Military targets are the holder's regiments, defence and production
facilities and the Alliance HQ; civilian targets are its other facilities.
A stalemated system keeps its earlier holder (`FUN_0050b610`).

### The run

1. **Death Star** (`FUN_0058d990`): with Destroy System, a Death Star in the
   bombarding fleets destroys the system (the cascade:
   [death-star.md](death-star.md)); then each minor character (families
   `0x38`-`0x3b`) of either side directly at the system, not aboard a fleet
   and not in transit, dies with chance GNPRTB `0x606` (50)
   (`bombardment::roll_characters`).
2. **Defence fire** (`FUN_0058dc10`): each KDY-150, then each LNR battery
   at the system, whoever owns it, fires once at a random bombarding ship:
   its DEFFACSD attack × (the General's leadership / 40 + 1).
3. **Hits** (`FUN_0058df60`): per fleet, its bombard strength × (the
   Admiral's leadership / 40 + 1), summed, minus the holder's planetary
   shield generators. Each hit picks a random target with value v and
   succeeds with chance (10 − v)/10. Target Military Facilities first rolls
   one collateral hit on the holder's first civilian facility, with chance
   1 in (military targets + 1), always when there is none (`FUN_0058d080`).
4. **Support** (`FUN_0058e320`): when a civilian facility, energy or raw
   material was lost, the system's support for the bombarding side falls by
   20 (GNPRTB 7705), with a sector-wide shift; destroying a system shifts
   support galaxy-wide instead, since a system no longer populated takes no
   support change.

Nothing branches by side, so either side can bombard.

### The port

`bombardment.rs` ports the run above for both sides and the AI (`gate`,
`open`, `bombard`, `finalize`; `integrator::order_strike`); the player orders
it from the Planetary Bombardment submenu.

## Source Material

- Ghidra: [bombardment.md](../../ghidra/notes/bombardment.md) — orders, phases, rolls, decompiles
- Ghidra: [support-shifts.md](../../ghidra/notes/support-shifts.md) — the support changes
- Rust: `crates/rebellion-core/src/bombardment.rs` — `gate`, `bombard`, `finalize`

## Related

- [Space Combat](space-combat.md)
- [Game Balance Parameters](gnprtb.md)
