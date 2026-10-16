---
title: "Simulate runs the tactical engine headless"
description: "The Battle Alert's Simulate path, shield persistence, the 1200-second stalemate withdrawal, and what a battle writes back to a strategic ship"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-14
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
| 4 | next 5 when a battle is pending, else `0xc` |
| 5 | `FUN_0041cfa0` → `FUN_00436280` offers the battle to both session slots (`+0x10` Alliance, `+0x14` Empire) through their slot `+0x44` |
| 6 | `FUN_0041cfc0` reads the player's choice into `local_b0`; next state is `7 + (local_b0 != 0)` |
| 7 | played battle: opens the tactical window (`FUN_00409690`), then `FUN_00596820`; next `0x14` |
| 8 | no window: `FUN_00596820` only; next `0x15` |
| `0x14` / `0x15` | wait until `FUN_0059a530(FUN_00597530())` reports the battle over; next `0x16` / `0x17` |
| `0x16` / `0x17` | `FUN_0040c780` imports the outcome into the strategic world (`local_a0` = 0 / 1) |

**Finding the battle.** `FUN_0041e230` → `FUN_0051ddb0` → `FUN_00516d50`
walks the systems (`0x90`-`0x98`); the first with `+0x88` bit `0x1000`
(battle pending, `FUN_005124c0` "SystemBattlePendingNotif") becomes the
session's battle system `+0x8c`, and `FUN_0050a580(system, 1)` marks it in
progress. State 9 runs `FUN_0041e270` → `FUN_0051ddf0` → `FUN_00516ef0` and
`FUN_00517270` ([battle-result-text-and-cadence.md](battle-result-text-and-cadence.md));
while both sides remain the system stays pending and fights again.

**The choice.** `FUN_004362b0` (state 6) reads each slot's answer through its
slot `+0x3c` (`FUN_00488600`, the slot's `+0x38`) and waits until both are
non-zero; both 1 gives state 8, anything else state 7. The computer player's
slot `+0x44` (`FUN_00485d50`) stores the battle and answers 1 at once. The
local player's (`FUN_004865b0`) answers 0 and hands the battle to the Battle
Alert (`FUN_00488d20`). So the human's alert decides between the window and
the headless run, and a battle between two computer players always runs
headless.

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
damage and `+0x64` packs six 4-bit damage counters (shield bits 0–3, weapon
4–7, then four more at 8–23 (`ship-damage-repair.md`) with change events at vtable
`+0x2a8`…`+0x2b4`; `FUN_00501640`, `FUN_005016e0`, `FUN_00501780`,
`FUN_00501820`). Shields exist only on the tactical ship, as a pool that
lasts the whole battle and recharges each second; the port already models
that pool (`rebellion-core` `tactical.rs`, `effective_shield_recharge`: hull ratio ×
rate − 0.25 × rate × shield-generator hits).

After the battle `FUN_0040cfc0` writes back, per ship, hull (from the
tactical hull fraction, through `FUN_00501ae0` / `FUN_00501b70` →
`FUN_00501490`), the shield and weapon damage counters (`FUN_00501510`, `FUN_005015a0`) and the
four packed counters. No shield value is written back: every battle starts
with full shields.

## The stalemate rule

`FUN_005ae9c0` keeps, for each side (`battle+0x24c`, `battle+0x250`), the
lowest side strength seen (`side+0x3c`, `FUN_005d0ce0`: firepower against
capitals (`FUN_005b7a80`) over squadrons, plus capitals not already
out-damaged; minimum at `+0x264` / `+0x268`) and a
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

## No other resolver

Every space battle goes through this session; nothing else resolves one.
`FUN_00549910` and its checks (`FUN_005442f0`-`FUN_00544fc0`,
[space-combat.md](space-combat.md)) are the MissionMgr story checks
([story-missions.md](story-missions.md)) and apply no damage. The only hull
writers are the tactical import above (`FUN_00501490` ← `FUN_00501ae0` /
`FUN_00501b70` ← `FUN_0040cfc0` / `FUN_0040d170` ← `FUN_0040c780`).

## Port

- Every space battle runs the tactical engine in `rebellion-core`
  (`crates/rebellion-core/src/tactical.rs`): windowed when the player
  commands, through `simulate_to_end` (`headless_step`,
  `apply_headless_damage`) for Simulate, and headless for every battle
  nobody watches. `integrator::resolve_system_space_combat` builds one
  session per contested system, seeded from one roll, and writes it back
  through `tactical::apply::apply_results`; `simulation.rs` and `main.rs`'s
  battles between two computer sides both call it.
- `combat.rs` still scales ground damage by GNPRTB `0x1400`
  (`GNPRTB_COMBAT_DIFFICULTY_MODIFIER`). Its only reader `FUN_0055d8c0` is
  the transit-distance formula (`build-delivery.md`); task-graph node
  `or-p-gnd-ykv` holds the ground fix.

## Open

- Who writes the local player's `+0x38` from the Battle Alert, and what
  slot `+0x48` (`FUN_00486650`) does against a remote human who chose to
  play.
- How the import treats a withdrawn fleet on the strategic map.
