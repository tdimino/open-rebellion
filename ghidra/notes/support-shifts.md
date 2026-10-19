---
title: "Support Shifts: Capture, Battle, HQ Loss and the Death Star"
description: "The sector-wide popular-support shifts (FUN_00518b20 records, event 0x373) after a control change, a blockade's end, the Alliance HQ's removal and a Death Star kill, with their GNPRTB/SDPRTB 0x1400-0x1445 values; system +0x88 bit 11 is the Death Star's presence in the sector"
category: "ghidra"
updated: 2026-10-16
tags: [support, popularity, control, blockade, death-star, gnprtb, sdprtb]
---

# Support shifts

Answers task-graph node `or-q-support-shifts-hol`. Every claim below was read
in the `.c` files cited. Support is system `+0x58`, the Alliance's share
(0-100); `FUN_00507270(side)` gives `+0x58` for side 1 and `100 - +0x58` for
side 2.

## The record and how it is applied

A shift is a 28-byte record (`FUN_00562ae0`, defaults `FUN_00562ba0`):

| Field | Meaning |
|---|---|
| `[0]` | owner filter: 0 every system, else only systems of that owner |
| `[1]`,`[2]` | system family range (default `0x90`-`0x98`) |
| `[3]` | gate side |
| `[4]` | gate limit |
| `[5]` | direction side |
| `[6]` | amount |

`FUN_00518ac0(object, record)` finds the object's sector and
`FUN_00518b20` walks the sector's systems. For each system `FUN_00518be0`
requires: the family in range, the owner filter, `+0x88` bit 0 (populated)
set and bit 3 clear, and `support(gate side) < gate limit`. It then posts
event `0x373` (`FUN_00562c70`, `FUN_0053fcd0`) carrying `amount`, negated
when the direction side is not 1. The event's fire (`FUN_00562fb0`) calls
`FUN_0050c9f0(system, amount, 1)`:

```
delta = amount
if +0x88 bit 11 and delta > 0:  delta = delta / G7681      ; FUN_00559be0, G7681 = 2
+0x58 = clamp(+0x58 + delta, 0, 100)                        ; FUN_0053e0d0, FUN_00509c00
```

`FUN_0050c9f0` does nothing on an unpopulated system.

**Bit 11 is the Death Star in the sector.** Its only setter `FUN_0050a890`
is called from `FUN_005178b0`, which marks every system of a sector whose
family `0x18`-`0x1b` object (`FUN_005061f0`; CAPSHPSD family `0x18`,
TEXTSTRA 10120 "Death Star") has `+0x50` bit 0 and lies in that sector
(`FUN_00518690`). Event `0x301` reruns it when a family `0x18`-`0x1b` object
changes (`FUN_00514510`), as does `FUN_00514a60`. The same bit:

- halves shifts toward the Alliance (above);
- multiplies the Empire's regiments by G7680 (2) in the support drift
  (`FUN_0050b230` → `FUN_00559c40` → `FUN_00559b60`);
- halves the Empire's garrison requirement and counts its regiments twice
  in the uprising score (`FUN_0050b5a0`, `FUN_0050d030`;
  `uprising-incident.md`).

## Parameters

GNPRTB values are the same in every column. SDPRTB values are
Alliance / Empire.

| Id | Table | Value | Use |
|---|---|---|---|
| `0x1401` | GNPRTB | -2 | Death Star kill, rim amount |
| `0x1402` | GNPRTB | 90 | Death Star kill, rim gate limit |
| `0x1403` | GNPRTB | -10 | Death Star kill, core amount |
| `0x1404` | GNPRTB | 101 | Death Star kill, core gate limit |
| `0x1405` | GNPRTB | 0 | HQ arrival amount |
| `0x1406` | GNPRTB | 0 | HQ arrival gate limit |
| `0x1407` | GNPRTB | -10 | HQ removal amount |
| `0x1408` | GNPRTB | 101 | HQ removal gate limit |
| `0x1420` | GNPRTB | 10 | blockade divisor |
| `0x1410` | SDPRTB | 101 / 101 | blockade gate limit |
| `0x1411` | SDPRTB | 20 / 20 | blockade cap |
| `0x1412` | SDPRTB | 100 / 100 | blockade weight, stronger side base |
| `0x1413` | SDPRTB | by column | blockade weight, stronger side slope |
| `0x1414` | SDPRTB | 0 / 0 | blockade weight, weaker side base |
| `0x1415` | SDPRTB | 100 / 100 | blockade weight, weaker side slope |
| `0x1420` | SDPRTB | 1 / 1 | control change amount per side |
| `0x1421` | SDPRTB | 100 / 100 | control change gate limit |
| `0x1440`, `0x1442`, `0x1444` | SDPRTB | 0, 10, 0 | control change amount: uprising starts, ends, continues |
| `0x1441`, `0x1443`, `0x1445` | SDPRTB | 101 | their gate limits |
| `0x1e01` (7681) | GNPRTB | 2 | Death Star divisor |
| `0x1e19` (7705) | GNPRTB | -20 | Death Star kill, at the dead system |

SDPRTB `0x1413` is 400 for the player's side in every column; for the
computer's side it is 200 (Easy), 150 (Medium), 100 (Hard), and 400 in
development and multiplayer. The loaders are `FUN_0055cb60` (`0x14xx`) and
`FUN_00558bb0` (`0x1e00`-`0x1e02`); `FUN_0053e390` reads GNPRTB (table
`0x3c`), `FUN_0053e3e0` SDPRTB (table `0x3d`) for side 1 or 2.

## The shifts

### Control change (`FUN_00517aa0`, `FUN_0055de50`)

Event `0x31b` (`FUN_00515c00`) gives a system's old and new control words
(`+0x78`: bits 0-1 holder, bits 4-5 uprising side; `FUN_00558880`). When
the holder changes, the record goes to every system of the sector:

- `from` = old holder, `to` = new holder; a holder of 3 counts as the
  opposite of the other end. Direction `to`, gate side `from`.
- No uprising before or after: amount = SDPRTB `0x1420` for each side
  among the old and new holders, so 2 when the Alliance and the Empire
  trade a system and 1 between a side and neutral; limit `0x1421`.
  Only the first such change per day whose old holder is a given side
  counts (`FUN_00518040`: per-day counters on the game root `+0x78`-`+0x88`,
  keyed by the old holder; returns 0 after the first).
- An uprising ends with the change: amount `0x1442` (10). Starts or
  continues: `0x1440`/`0x1444` (0).
- Coruscant (system `+0x18 == 0x109`) then reruns the victory check
  (`FUN_00517be0`, `victory-conditions.md`).

### Blockade end (`FUN_00515f40`, `FUN_00517ee0`, `FUN_0055dbf0`)

When a system's blockade bit (`+0x88` `0x20`) is set, `FUN_00511300` zeroes
the per-side loss counters A (`+0x54 → +0x44 + 4·side`) and B
(`+0x50 + 4·side`) and stores the blockading side at `+0x54 → +0x60`.
`FUN_00508c60` adds a destroyed regiment's (family `0x10`-`0x1f`) slot
`+0x1c0` minus slot `+0x1c8` to A for removal reason `0xe` and to B for
reason `0xf`, by the regiment's owner. When the bit clears (event `0x305`
with 0):

- `X` = the blockading side; unless the system is destroyed, `X` becomes the
  other side when the blockader does not hold the system.
- `L1`, `L2` = A + B for each side, plus side 3's when that side is not the
  blockader (`FUN_00509a20`).
- `T1`, `T2` = slot `+0x1c0` summed over every completed, existing
  family `0x10`-`0x1f` object in the galaxy, by owner.

```
S = the side with more (T1 > T2 → 1, else 2); W = the other; d = T_S - T_W
k_S = 0x1412[S] + 0x1413[S]·d / T_S             ; percent, 0 when T_S = 0
k_W = 0x1414[W] + 0x1415[W] - 0x1415[W]·d / T_S
w_i = L_i · k_i / 100
amount = (2·w_O - w_X) / G0x1420                 ; O = the side other than X
```

A negative amount turns the direction to `O` and is made positive. The
amount is capped at SDPRTB `0x1411`[direction] (20); the gate side is the
other side of the direction, limit `0x1410` (101). So the side whose
opponent lost more troops gains up to 20 points across the sector, and the
galaxy's stronger side's losses weigh more.

### Alliance HQ removed or arriving (`FUN_00517cd0`, `FUN_00517de0`)

- Event `0x300` (`FUN_005140c0`) for a family `0x20`-`0x21` object with
  fourth argument 0, at a system: amount GNPRTB `0x1407` (-10) toward the
  Alliance, so +10 for the Empire, across the sector (`FUN_0055e1f0`), then
  the victory check.
- Event `0x301` (`FUN_00514510`) for the HQ en route (`+0x50` bit 4)
  arriving at a system: only Alliance-held systems, amount `0x1405` (0)
  (`FUN_0055e150`). No effect in the shipped data.

### Bombardment and the Death Star kill (`FUN_0058e320`)

A bombardment (`bombardment.md`) sets the state's `+0x28` when it destroys a
civilian facility (`0x28`-`0x2f`, `FUN_0058ce70`) or lowers energy or raw
material (`FUN_0058cf20`, `FUN_0058cf50`). Destroy System with a family
`0x18`-`0x1b` ship sets `+0x28` and `+0x2c` (`FUN_0058d990` →
`FUN_0058ce70`). Finalize then, with `+0x28`:

- changes the destroyed system's support for the bombarding side by GNPRTB
  7705 (-20) (`FUN_0055a0e0`, `FUN_0050c9f0`); for the Empire that change
  favours the Alliance, so the Death Star's own presence halves it to -10;
- with `+0x2c`, across the galaxy (`FUN_00518a70`, every sector), with the bombarding
  side as direction and gate side (`FUN_0055da90`): core systems
  (`0x90`-`0x91`) shift by `0x1403` (-10) where that side's support is below
  `0x1404` (101); rim systems (`0x92`-`0x93`) by `0x1401` (-2) where it is
  below `0x1402` (90).

So an Imperial Death Star kill raises the Alliance's support by 10 in every
core system and 2 in rim systems where the Empire has under 90, halved in
the Death Star's own sector. Any other bombardment that sets `+0x28` takes the
sector-wide branch (`FUN_0055d900`, `FUN_00518ac0`, SDPRTB `0x1408`-`0x140b`:
-2/-1 and -4/-3) after the GNPRTB 7705 change at the system.

## Port

`crates/rebellion-core` has the bombardment and Death Star shifts only:
`bombardment::finalize` reads GNPRTB `0x1e19` and `0x1401`-`0x1404` and
SDPRTB `0x1408`-`0x140b`, and applies each through
`uprising::apply_support_change` at once rather than through event
`0x373`. The blockade shift is timer `0x386` in `blockade.rs`
(`advance_support_shift`), applied through the same call. The
control-change and HQ shifts are not ported. `+0x88` bit 11 is
`GameWorld::death_star_sectors`, refreshed by `death_star::refresh_sectors`
at the start of each tick, after arrivals, after the battles, and after a
load or a new game: the sectors where a fleet
carrying the Death Star sits at a system, not in transit (`FUN_005178b0`).
It gates the halving in `uprising.rs` (`support_change`), the Empire's
doubled regiments in the drift (`economy.rs`) and the uprising score, and
the Empire's halved garrison.

## Open

- What system `+0x88` bit 3 is (set with bit 1 by system slot `+0xbc`,
  `FUN_005081f0`); it blocks shifts.
- Which removals carry reason `0xe`, and reason `0xf` outside bombardment.
  In bombardment, `0xf` marks an enemy object destroyed by a hit
  (`FUN_0058ce70`; `0x10` for the bombarder's own) and a bombarding ship
  killed by defence fire (`FUN_00501ae0(…, 0xf, …)` in `FUN_0058d540`).
- What event `0x300`'s fourth argument is.

## Decompiles

`FUN_00562ae0.c`, `FUN_00562ba0.c`, `FUN_00518ac0.c`, `FUN_00518b20.c`,
`FUN_00518be0.c`, `FUN_00562fb0.c`, `FUN_0050c9f0.c`, `FUN_00559be0.c`,
`FUN_00507270.c`, `FUN_005178b0.c`, `FUN_005061f0.c`, `FUN_00518690.c`,
`FUN_00559b60.c`, `FUN_0055cb60.c`, `FUN_00558bb0.c`, `FUN_0053e390.c`,
`FUN_0053e3e0.c`, `FUN_00515c00.c`, `FUN_00558880.c`, `FUN_00517aa0.c`,
`FUN_0055de50.c`, `FUN_00518040.c`, `FUN_00515f40.c`, `FUN_00511300.c`,
`FUN_00509a20.c`, `FUN_005099c0.c`, `FUN_005099f0.c`, `FUN_00517ee0.c`,
`FUN_0055dbf0.c`, `FUN_0053e170.c`, `FUN_0053e190.c`, `FUN_0053e0f0.c`,
`FUN_00517cd0.c`, `FUN_0055e1f0.c`, `FUN_00517de0.c`, `FUN_0055e150.c`,
`FUN_0058e320.c`, `FUN_0058d990.c`, `FUN_0058ce70.c`, `FUN_0055d900.c`,
`FUN_0055da90.c`, `FUN_0055a0e0.c`, `FUN_00518a70.c`, `FUN_005081f0.c`,
`FUN_0058cf20.c`, `FUN_0058cf50.c`, `FUN_0058d540.c`.
