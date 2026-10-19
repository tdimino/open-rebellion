---
title: "Ground Combat"
description: "Planetary assault (order 0x234): what the original shows, how it resolves, and the port"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-17
game_system: "ground-combat"
sources:
  - type: "ghidra"
    file: "ghidra/notes/ground-unit-assets.md"
  - type: "ghidra"
    file: "ghidra/notes/ground-assault.md"
  - type: "ghidra"
    file: "ghidra/notes/ground-combat.md"
  - type: "code"
    file: "crates/rebellion-core/src/assault.rs"
  - type: "prima"
    page: "Chapter 8: Ground Operations"
related:
  - "space-combat"
  - "bombardment"
tags: ["combat", "troops", "ground", "regiment"]
---

# Ground Combat

## What You See

A fleet carrying regiments to an enemy system can order a Planetary
Assault (order `0x234`). The battle is report-only: there is no tactical
ground battle. Three runs of the original in the VM
([ground-assault.md](../../ghidra/notes/ground-assault.md), "In the
original"): at Alliance-held Selonia one Stormtrooper Regiment against two
Alliance Army Regiments and a GenCore Level I opened "Troops have defended"
(`0x7092`) at once, one Alliance regiment died and the Imperial regiment
stayed aboard; at Ord Pardon (Imperial) and Drall (Alliance), with no
defender, the assaulting side took the system (`0x7091`) with no
casualties. No run has yet shown an attacker's loss, but the trace kills
attackers in two places: a defence facility's shot (step 1) and an
engagement roll of 4 or less (step 2, `FUN_0055eb00`).

## How the Original Resolves It

The order (`0x234`, [ground-assault.md](../../ghidra/notes/ground-assault.md))
is enabled for a team of one side's fleets that blockade their system, with
a regiment aboard, at a system that is not destroyed and holds fewer than
two shield generators (GNPRTB `0x1e03`) of either side. A neutral system is
never blockaded, so it cannot be assaulted. The run (`FUN_00563390` ->
`FUN_0058c120`) has no rounds:

1. Every defence facility of either side fires once with chance
   attack / 5 percent, killing a random attacking regiment: a KDY-150 or
   LNR battery always fires, a shield generator never.
2. Every attacking regiment engages one random defending regiment:
   `rand[0, 10]` plus its class attack and its fleet General's
   leadership / 30, less the defender's class defence and the system
   General's leadership / 30. 4 or less kills the attacker, 6 or more the
   defender, 5 changes nothing.
3. Each engagement has an 8 percent chance of a collateral hit, which
   destroys a facility of either side (never the Alliance HQ) or takes one
   energy or raw material.
4. With attackers left and no defending regiment, up to six regiments land
   (GNPRTB 7761 / 7762), and the system's holder follows its regiments
   (`FUN_0050b610`): with no regiment of either side left there, it is the
   support-derived side.

Characters at a captured system are untouched. No regiment lands without
the order, and none fights after a space battle.

`FUN_00560d50` is not ground combat. It is the character manager's move
hook: it clears a character's CommandKind (`+0x96`, officer rank) when the
character or its capital ship moves, and runs the Emperor-at-Coruscant
flag `FUN_005617b0` for Palpatine (family `0x34`)
([ground-combat.md](../../ghidra/notes/ground-combat.md)).

## Port

`crates/rebellion-core/src/assault.rs` ports the enable test and the run
(`order_enabled`, `resolve`); `integrator::order_assault` and
`resolve_assault` add the loss counters, the holder's rebuild and its
support shifts. Either side issues it from the fleet's menu
(`rebellion-app/src/assault_orders.rs`) and the AI through
`integrator::resolve_ai_assault`. The Operation Report (`0x7090`-`0x7094`)
goes to both sides.

## Source Material

- Ghidra: [ground-unit-assets.md](../../ghidra/notes/ground-unit-assets.md) — ground-unit classes, families and art
- Ghidra: [ground-combat.md](../../ghidra/notes/ground-combat.md) — the character move hook
- Ghidra: [COMBAT-SUMMARY.md](../../ghidra/notes/COMBAT-SUMMARY.md) — where each kind of combat is resolved
- Ghidra: [ground-assault.md](../../ghidra/notes/ground-assault.md) — the order, the run, its rolls and the control change
- Rust: `crates/rebellion-core/src/assault.rs` — `order_enabled()`, `resolve()`

## Related

- [Space Combat](space-combat.md)
- [Bombardment](bombardment.md)
