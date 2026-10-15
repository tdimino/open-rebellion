---
title: "Ground Combat"
description: "Planetary assault (order 0x234): what the original shows, what is traced, and the port's interim resolver"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-14
game_system: "ground-combat"
sources:
  - type: "ghidra"
    file: "ghidra/notes/ground-unit-assets.md"
  - type: "ghidra"
    file: "ghidra/notes/ground-combat.md"
  - type: "code"
    file: "crates/rebellion-core/src/combat.rs"
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
Assault (order `0x234`). In the original running in the VM, the attacker
never loses regiments; the defender's losses show under System Forces; the
Empire cannot see an Alliance garrison; and taking the troops off a captured
system hands it back. The battle is report-only: there is no tactical
ground battle.

## How the Original Resolves It

Open. The Planetary Assault order class (the class owning `FUN_00563390`)
leads to `FUN_0058c120`, `FUN_0058bec0` and `FUN_0055eb00`, with thresholds
at `DAT_006bb778`..`DAT_006bb78c` (atlas, map-15). The resolution and the
regiment fields it reads are task-graph questions `or-q-ground-resolution-nms`
and `or-q-regiment-combat-eaxn`.

`FUN_00560d50` is not ground combat. It is the character manager's move
hook: it clears a character's CommandKind (`+0x96`, officer rank) when the
character or its capital ship moves, and runs the Emperor-at-Coruscant
flag `FUN_005617b0` for Palpatine (family `0x34`)
([ground-combat.md](../../ghidra/notes/ground-combat.md)). Regiments are
`TROOPSD` family `0x10` and special forces family `0x3c`; the hook tests
neither.

## Port

`CombatSystem::resolve_ground` in `crates/rebellion-core/src/combat.rs`
rolls every attacking regiment against every defender from the `TROOPSD`
class attack and defence and the current strength, adds an officer combat
bonus and a facility defence bonus, and scales damage by GNPRTB `0x1400`.
None of this has a source: `regiment_strength` is not traced, and
GNPRTB `0x1400` is the transit-distance divisor (`build-delivery.md`). The
port node `or-p-gnd-ykv` (epic `or-ep-gnd-mmf`) replaces it with the
original's report-only flow on both sides.

## Source Material

- Ghidra: [ground-unit-assets.md](../../ghidra/notes/ground-unit-assets.md) — ground-unit classes, families and art
- Ghidra: [ground-combat.md](../../ghidra/notes/ground-combat.md) — the character move hook
- Ghidra: [COMBAT-SUMMARY.md](../../ghidra/notes/COMBAT-SUMMARY.md) — where each kind of combat is resolved
- Rust: `crates/rebellion-core/src/combat.rs` — `CombatSystem::resolve_ground()`

## Related

- [Space Combat](space-combat.md)
- [Bombardment](bombardment.md)
