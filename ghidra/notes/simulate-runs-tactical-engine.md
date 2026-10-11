---
title: "Simulate runs the tactical engine headless"
description: "The Battle Alert's Simulate path, shield persistence, the 1200-second stalemate withdrawal, and what a battle writes back to a strategic ship"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-10
---

# Simulate runs the tactical engine headless

Traced 2026-10-10 in the saved `REBEXE.EXE` project (headless, read-only, on a
scratch copy). Cited `.c` files are in this folder.

## The battle session state machine

`FUN_00413560`, the main message loop, calls `FUN_00401bd0` after every
`PeekMessageA` drain. `FUN_00401bd0` fetches the battle session
(`FUN_00409270`) and steps it with `FUN_0040a050`, a state machine on
`this+8`:

| State | Action |
|---|---|
| 6 | `FUN_0041cfc0` reads the player's choice into `local_b0`; next state is `7 + (local_b0 != 0)` |
| 7 | played battle: opens the tactical window (`FUN_00409690`), then `FUN_00596820`; next `0x14` |
| 8 | no window: `FUN_00596820` only; next `0x15` |
| `0x14` / `0x15` | wait until `FUN_0059a530(FUN_00597530())` reports the battle over; next `0x16` / `0x17` |
| `0x16` / `0x17` | `FUN_0040c780` imports the outcome into the strategic world (`local_a0` = 0 / 1) |

Both paths start the same engine (`FUN_00596820` → `FUN_00599a70(engine, 10)`)
and import through the same function. Simulate is state 8: the tactical
battle, run without its window.

## The headless step

`FUN_0059a530`: when the engine's `+0x668 == 1` it steps the battle object
(`FUN_00596f30`) with `FUN_005ae9c0(battle, DAT_0066badc)`; `DAT_0066badc` is
`1.0`, one second per step. Otherwise it drives the rendered window
(`FUN_005aaf90`). When the step reports the end, the 7-dword outcome is
copied from `battle+0x10` to `engine+0x66c`.

The step moves nothing. Each side ranks its opponents, assigns targets and
applies its groups' damage rates (`FUN_005d14d0`); task-force steering
(`FUN_005cc300`) and the batteries run only in the windowed battle. See
[tactical-engagement-ai.md](tactical-engagement-ai.md).

## Shields

The strategic `CCapitalShip` has no current-shield field: `+0x60` is hull
damage and `+0x64` packs 4-bit fields (shield allocation bits 0–3, weapon
allocation 4–7, then four more at 8–23 with change events at vtable
`+0x2a8`…`+0x2b4`; `FUN_00501640`, `FUN_005016e0`, `FUN_00501780`,
`FUN_00501820`). Shields exist only on the tactical ship, as a pool that
lasts the whole battle and recharges each second; the port already models
that pool (`tactical_view.rs`, `effective_shield_recharge`: hull ratio ×
rate − 0.25 × rate × shield-generator hits).

After the battle `FUN_0040cfc0` writes back, per ship, hull (from the
tactical hull fraction, through `FUN_00501ae0` / `FUN_00501b70` →
`FUN_00501490`), the two allocations (`FUN_00501510`, `FUN_005015a0`) and the
four packed counters. No shield value is written back: every battle starts
with full shields.

## The stalemate rule

`FUN_005ae9c0` keeps, for each side (`battle+0x24c`, `battle+0x250`), the
lowest side strength seen (`side+0x3c`, minimum at `+0x264` / `+0x268`) and a
timer since it last fell (`+0x26c` / `+0x270`, reset on a new low). When
both timers reach `_DAT_0066c3e0` = `1200.0` seconds (1200 steps), the side
with the lower or equal strength loses (both when equal). A side with a
working hyperdrive (vtable `+0xc`) is flagged `side+0xb0` and retreats. A
side without one is destroyed: `FUN_005af7a0` is 100000 damage through
`FUN_005b54d0`, not a move. See
[tactical-engagement-ai.md](tactical-engagement-ai.md).

A Death Star check (`FUN_00596bd0` → `FUN_005a66a0` → `FUN_005ba420`) makes
side `+0x24c`'s figure `-1.0` while the Death Star is operational, so the
side facing it always loses the stalemate and the Death Star's side never
withdraws. Checked in the port 2026-10-10 for both factions
(`.artifacts/native-checks/2026-10-10-ds-stalemate-*`).

A battle therefore ends in a win, mutual destruction, a retreat or a
destruction; it never leaves both sides fighting. The strategic re-fight
while both remain (`FUN_00517270`) covers forces that survive and stay, not
a stalemate inside one battle.

## Correction to space-combat.md

`FUN_00549910`'s seven "phases" set the `+0x58` flag word and notify
observers; it is reached from object-change fan-outs such as `FUN_00514a60`
(object, from, to). It applies no damage. The only hull writers found are the
tactical import above (`FUN_00501490` ← `FUN_00501ae0` / `FUN_00501b70` ←
`FUN_0040cfc0` / `FUN_0040d170` ← `FUN_0040c780`).

## Open

- (Resolved) `side+0x3c` is `FUN_005d0ce0`: firepower against capitals
  (`FUN_005b7a80`) over squadrons, plus capitals not already out-damaged.
  See [tactical-engagement-ai.md](tactical-engagement-ai.md).
- How the import treats a withdrawn fleet on the strategic map.
