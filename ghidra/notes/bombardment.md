---
title: "Orbital Bombardment"
description: "Bombardment orders 0x220-0x223: the gate FUN_00582440, the run FUN_0058d800/FUN_0058d770 (Death Star phase, KDY/LNR defence fire, per-hit target rolls), and the support shifts in FUN_0058e320"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# Orbital Bombardment

Answers task-graph node `or-q-bombardment-flz`. Every claim below was read in
the `.c` files cited unless marked *inferred*.

## The orders

STRATEGY RCDATA names (`ai-order-emission.md`): `0x220` Target Military
Facilities, `0x221` Target Civilian Facilities, `0x222` General Bombardment,
`0x223` Destroy System.

The four share one gate, `FUN_00582440`:

- every active team fleet (family `0x08`-`0x0f`) needs a nonzero sum of
  slot `+0x1dc` over its ships and squadrons (`FUN_004fd6b0`), else status
  `8`/`6`;
- Destroy System also needs a ship of family `0x18`-`0x1b` in the fleet
  (`FUN_005003d0`, `FUN_005131b0`), else `8`/`8`.

The gate tests no side and no shield. `FUN_005637c0`, slot 8 of the command
class at vtable `0x00663300`, builds the state (`FUN_0058d720`, vtable
`0x0066a950`) and calls `FUN_0058d800`.

## Setup (`FUN_0058d800`)

| Order | `+0x34` military | `+0x38` civilian | `+0x3c` energy, raw material | `+0x40` collateral | `+0x44` Destroy System |
|---|---|---|---|---|---|
| `0x220` | 1 | | | 1 | |
| `0x221` | | 1 | | | |
| `0x222` | 1 | 1 | 1 | | |
| `0x223` | 1 | 1 | 1 | | 1 |

The bombarding side is the owner shared by every fleet at the system with
`+0x58` bit 6 set (the order's fleets). If two sides have such fleets,
nothing happens. The state keeps the system at `+0x10`, the bombarder at
`+0x14` and the holder (system `+0x24` bits 6-7) at `+0x18`, then runs
`FUN_0058d770`: ship phase, defence phase, effect phase, finalize.

## Targets (`FUN_0058cba0`)

Each iterator takes only the holder's objects.

- Military: regiments `0x10`-`0x13` (`FUN_00504cc0`), facilities
  `0x22`-`0x27` (`FUN_00527050`) and the Alliance HQ family `0x20`-`0x21`
  (`FUN_00526d70`).
- Civilian: facilities `0x28`-`0x2f` (`FUN_0053b760`).
- Energy and raw material: system `+0x5c` and `+0x64`, one target each while
  nonzero.

## Ship phase (`FUN_0058d990`)

- Sets the system's bombard bit (`FUN_0050a660(1)`).
- With `+0x44` set, `FUN_0058d440` looks for a ship of family `0x18`-`0x1b`
  in the bombarding fleets (`FUN_0058ca70`), the Death Star (CAPSHPSD family
  `0x18`, TEXTSTRA 10120). No operational, shield or holder test is made.
- When found, `FUN_0058ce70` destroys the system through slot `+0xac`, cause
  `0x10` when the system is the bombarder's own and `0xf` otherwise; the state
  sets `+0x28`, `+0x2c` and `+0x30`. Every character at the system
  (`FUN_0056f3d0(system, 3)`) without `+0x50` bit 5 is hit with probability
  GNPRTB `0x606` (50) (`FUN_0055e9e0`): slot `+0x2f0(1, 10)`, plus slot
  `+0xac(10)` when `+0x50` bit 6 is set.
- Otherwise it counts the bombarding ships (`+0x1c`) and the targets
  (`+0x20`).

## Defence phase (`FUN_0058dc10`)

- Each KDY-150 (family `0x22`, `FUN_00527550`) fires once, then each LNR
  Series I/II (`0x23`, `FUN_005272e0`), at a random bombarding ship.
- Damage: DEFFACSD attack `+0x5c` (`FUN_00520b70`; KDY 2000, LNR 800/5000)
  × (leadership `+0x1f4` of the character at the system with `+0x96 == 3` /
  GNPRTB `0x600` (40) + 1) (`FUN_00509330`, `FUN_0055e930`). *Inferred:*
  `+0x96 == 3` is the General.
- KDY damage goes to the ship's first accumulator (`FUN_00501ba0`), LNR damage
  to the second (`FUN_00501bc0`). `FUN_0058d540` applies them through
  `FUN_0055e9a0`: the first against the remaining shields (`+0x1e0` −
  `+0x204`), the second against the shields left and then the hull (`+0x248`
  − `+0x274`); shield loss through `0x00501be0`, hull through
  `FUN_00501ae0(…, 0xf, …)`.
- `+0x30` is set when no bombarding ship remains.

## Effect phase (`FUN_0058df60`)

1. Hits: for each bombarding fleet, the slot-`+0x1dc` sum × (leadership
   (slot `+0x1f4`) of its role-2 character (`FUN_004fd790(fleet, 2)`) /
   GNPRTB `0x601` (40) + 1) (`FUN_0055e950`), summed. Subtract the holder's
   shield generators: family `0x24` only, DEFFACSD `+0x60` (`FUN_00527940`,
   `FUN_00520b80`). The Death Star shield `0x25` is not counted. *Inferred:*
   role 2 is the Admiral.
2. Collateral (`+0x40`, order `0x220` only, with hits left): when
   `FUN_0053e290(+0x20 − 1 + +0x24) < +0x24`, one civilian facility
   (`FUN_0058d080`) gets a single roll.
3. Each hit picks a random target index (`FUN_0053e290(+0x20 − 1)`), takes
   its value v (`FUN_0058cf80`) and succeeds when v < `FUN_0053e2c0`(GNPRTB
   `0x602`, `0x603`), a draw from 1 to 10 (`FUN_0055e970`), so with chance
   (10 − v)/10. Values: facilities and the HQ slot `+0x1c0` − `+0x1c4`,
   regiments `+0x1d8` − `+0x1e4`, energy GNPRTB `0x604` (9), raw material
   `0x605` (9).
4. A success (`FUN_0058d020`) destroys the object through `FUN_0058ce70`
   (cause `0xf`, or `0x10` for the bombarder's own) or lowers energy or raw
   material by 1 (`FUN_0058cf20`, `FUN_0058cf50`). The targets are recounted;
   none left sets `+0x30`.

`+0x28` is set by destroying a `0x28`-`0x2f` facility or by any energy or raw
material loss.

## Finalize (`FUN_0058e320`)

- With `+0x28`, the system's support for the bombarding side changes by
  GNPRTB 7705 (−20) (`FUN_0055a0e0`, `FUN_0050c9f0`).
- With `+0x2c`, a galaxy-wide shift (`FUN_0055da90`, `FUN_00518a70`); with
  `+0x28` and not `+0x2c`, a sector-wide shift (`FUN_0055d900`, SDPRTB
  `0x1408`-`0x140b`, `FUN_00518ac0`).
- Clears the system's bombard bit and each bombarding fleet's bit 6
  (`FUN_004fe460`).

## Both sides

Nothing branches by side except the own/enemy cause. Either side can bombard;
only the Empire can Destroy System, because only CAPSHPSD family `0x18`
qualifies (*inferred* from the data).

## The Alliance HQ

The HQ (family `0x20`-`0x21`) is a military target, so Target Military
Facilities, General Bombardment and Destroy System's hit loop can destroy it
with cause `0xf`. Destroy System also destroys every completed object inside
the system with the system's own cause, the HQ included (`FUN_004fb520` →
child slot `+0xb4`, `game-end-path.md`). *Inferred:* its value is ALLFACSD's last field (9), so a hit
that picks it destroys it with chance 1/10.

## Port

- `crates/rebellion-core/src/bombardment.rs` computes one "damage" number
  from the transit-distance formula (`FUN_0055d8c0`, `build-delivery.md`):
  no targets, defence fire, shields, Admiral, General or rolls.
- `crates/rebellion-app/src/main.rs` (~8860-8905) turns that number into a
  popularity cut capped at 0.25; it has no source.
- `crates/rebellion-core/src/victory.rs` `apply_headquarters_bombardment`
  clears `is_headquarters` on any Empire bombardment with damage above 0.
- Destroy System is `PanelAction::FireDeathStar` (`death_star.rs`). It
  refuses Empire-held systems, which the original does not, and has no
  character roll.
- `bombardment::finalize` is the finalize: the GNPRTB 7705 change at the
  system, then the galaxy-wide or the sector-wide shift. The AI's strikes
  (`integrator::resolve_ai_strike`) call it, a Death Star kill with
  `+0x28` and `+0x2c`. A bombardment passes neither flag, because the port
  rolls no hits.

## Open

- Which class field ship and squadron slot `+0x1dc` reads (bombard strength).
- Whether the HQ's slot `+0x1c0` reads ALLFACSD's last field.
- What the command's system slots `+0x1c4`/`+0x1c8` gate, including
  bombarding one's own system.
- When `FUN_005637c0` runs within the day.
- Whether KDY damage beyond the shields reaches the hull in `FUN_0058d540`.

## Decompiles

Order and setup: `FUN_00582440.c`, `FUN_004fd6b0.c`, `FUN_005003d0.c`,
`FUN_005637c0.c`, `FUN_0058d720.c`, `FUN_0058d800.c`, `FUN_0058d770.c`.
Ship phase and walkers: `FUN_0058d990.c`, `FUN_0058d3f0.c`, `FUN_0058d440.c`,
`FUN_0058ca70.c`, `FUN_0058d150.c`, `FUN_0058d1c0.c`, `FUN_0058c960.c`,
`FUN_0058c9b0.c`, `FUN_0058ce70.c`, `FUN_0055e9e0.c`.
Defence: `FUN_0058dc10.c`, `FUN_00527550.c`, `FUN_005272e0.c`,
`FUN_00520b70.c`, `FUN_00509330.c`, `FUN_0055e930.c`, `FUN_0058d540.c`,
`FUN_0055e9a0.c`.
Effect and targets: `FUN_0058df60.c`, `FUN_0055e950.c`, `FUN_00527940.c`,
`FUN_00520b80.c`, `FUN_0058d080.c`, `FUN_0058cba0.c`, `FUN_00504cc0.c`,
`FUN_00527050.c`, `FUN_00526d70.c`, `FUN_0053b760.c`, `FUN_0058cf80.c`,
`FUN_0055e970.c`, `FUN_0053e2c0.c`, `FUN_0053e290.c`, `FUN_0058d020.c`,
`FUN_0058cf20.c`, `FUN_0058cf50.c`.
Finalize: `FUN_0058e320.c`, `FUN_0055a0e0.c`, `FUN_0050c9f0.c`,
`FUN_0055d900.c`, `FUN_0055da90.c`. Parameters: `FUN_0055e840.c` (GNPRTB
`0x600`-`0x606` = 40, 40, 1, 10, 9, 9, 50).
