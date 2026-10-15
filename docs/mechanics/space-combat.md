---
title: "Space Combat"
description: "Every fleet battle runs the tactical engine: played, simulated or headless"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-14
game_system: "space-combat"
sources:
  - type: "ghidra"
    file: "ghidra/notes/simulate-runs-tactical-engine.md"
  - type: "ghidra"
    file: "ghidra/notes/tactical-engagement-ai.md"
  - type: "code"
    file: "crates/rebellion-core/src/tactical.rs"
  - type: "prima"
    page: "Chapter 7: Fleet Combat"
related:
  - "ground-combat"
  - "bombardment"
  - "gnprtb"
tags: ["combat", "fleet", "tactical", "simulate"]
---

# Space Combat

## What You See

When two opposing fleets occupy the same system, space combat triggers. Capital ships exchange turbolaser, ion cannon, and laser fire. Shields absorb incoming damage before it reaches the hull. Fighters launch from carriers and engage enemy squadrons and capital ships. Ships that lose all hull points are destroyed. The side with surviving ships wins.

## How It Works

Every space battle runs one tactical engine (`FUN_0040a050`). The player can
command it in the battle window, or choose Simulate from the Battle Alert;
battles the player doesn't watch run the same engine headless (state 8). The
strategic side never computes damage itself. `FUN_00549910` and the flag word
`+0x58` belong to MissionMgr story checks and apply no damage.

### Forces

A side fights with every fleet it has at the system plus the squadrons the
system holds for it (`FUN_00516ef0`). A battle starts on any day both sides
are present (`FUN_00509710`, `FUN_0050b8e0`), and it starts again the next
day while both remain.

### The headless step

The headless battle steps once a second (`FUN_005ae9c0`). Nothing moves and
no battery fires. Each side in turn ranks its opponents, assigns targets and
applies each group's damage rate to its target (`FUN_005d14d0`), then
recharges shields. A unit's rate (`FUN_005b7780`) sums its strongest arc's
weapon families against the target's class, times its weapon recharge, over
the arc's weapon count. Ions do nothing to fighters, and fighters add
torpedoes against an unshielded capital ship.

### Played battles

In the window, task forces and fighter groups without orders pick a target
by their tactic (Surround or Stand Off) and close to the band where their
net strength over the target is greatest. Batteries fire through their arcs
and recharge, projectiles travel, and only the player's units take subsystem
hits.

### Shields

Shields exist only on the tactical ship, as a pool that lasts the battle and
recharges each second. No shield value is written back: every battle starts
with full shields. Hull damage, the allocations and the packed counters are
written back to the strategic ship (`FUN_0040cfc0`).

### The stalemate rule

After 1200 seconds with no new low in either side's strength
(`_DAT_0066c3e0`, `FUN_005d0ce0`), the weaker side loses. Its ships with a
working hyperdrive retreat; a side with none is destroyed. While a Death Star
is operational, the side facing it always loses the stalemate. A battle
therefore ends in a win, mutual destruction, a retreat or a destruction.

### Results

The Battle Results lines follow `FUN_0049b330` (won), `FUN_0049b4d0` (lost)
and `FUN_0049b150` (draw), and compare the system's holder with the side
that held a blockade there before the battle.

## Source Material

- Ghidra: [simulate-runs-tactical-engine.md](../../ghidra/notes/simulate-runs-tactical-engine.md) — Simulate, the headless step, shields, the stalemate
- Ghidra: [tactical-engagement-ai.md](../../ghidra/notes/tactical-engagement-ai.md) — targets, steering, the band
- Ghidra: [tactical-weapon-loop.md](../../ghidra/notes/tactical-weapon-loop.md) — batteries and recharge
- Ghidra: [battle-result-text-and-cadence.md](../../ghidra/notes/battle-result-text-and-cadence.md) — result text and re-fights
- Rust: `crates/rebellion-core/src/tactical.rs` (`BattleSession`), `crates/rebellion-data/src/integrator.rs` (`resolve_system_space_combat`)

## Related

- [Ground Combat](ground-combat.md)
- [Bombardment](bombardment.md)
- [Game Balance Parameters](gnprtb.md)
