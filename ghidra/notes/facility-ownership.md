---
title: "Facility Ownership"
description: "Which side a facility belongs to: the system's side at seeding, the cascade that hands a system's completed facilities to its new side (or removes those whose class cannot serve it), the HQ's class, and the mine/refinery families"
category: "ghidra"
created: 2026-10-07
updated: 2026-10-07
tags: [facility, side, seeding, control, capture, hq, mine, refinery]
---

# Facility Ownership

Recovered 2026-10-07 with Ghidra 12.1.4 headless (read-only project copy).
Every function named here has a `FUN_<address>.c` note in this directory.

An object's side is the two bits `+0x24 >> 6 & 3`. `FUN_004f8c60` names them:
1 Alliance, 2 Empire, 3 Neutral, 0 "UNKNOWN!". A facility stores its own
side, but the original keeps it equal to its system's side. Seeding copies
the system's side, and a change of the system's side cascades to the
facility.

## Seeding

The setup driver `FUN_005136d0` runs one stage per frame:

- **Stage `0xb` (`FUN_00519d00`)** sorts the galaxy's systems into lists by
  side.
- **Stage `0xc` (`FUN_0051a280` → `FUN_0051bc70` → `FUN_004f7d50`)** creates
  each system with its list's side:

  | List | Side |
  |---|---|
  | `+0x100`, `+0x110` | 1 |
  | `+0xe0`, `+0xf0` | 2 |
  | `+0x120`, `+0x130` | 3 |
  | `+0x140`: `0x92000121` | 1 |
  | `+0x140`: `0x90000109` | 2 |

- **Stage `0xf` (`FUN_0051a570`)** walks the systems and calls slot `+0x1cc`
  on each:
  - core systems (vtable `0x00663698`): `FUN_00566de0`;
  - rim systems (vtable `0x00664098`): `FUN_0056a6f0`, only when `+0x88`
    bit 0 (populated) is set.

Each seeder repeats its roll once per energy unit (`+0x5c`):

1. `FUN_00559850` (core, SYFCCRTB, table id 1) or `FUN_00559a60` (rim,
   SYFCRMTB, table id 2) chooses a class:
   - a mine, `0x2c000001`, with chance `(raw +0x64 − mines of the system's
     side) × multiplier`; the multiplier is `DAT_006bb4bc` for core and
     `DAT_006bb45c` for rim (GNPRTB 7766 and 7767);
   - otherwise the table entry at `rand(100)`.
2. `FUN_004f7d50(system, &class, system +0x24 >> 6 & 3, ctx, 1, 1)` creates
   it.

So **every seeded facility takes its system's side**, and a neutral
system's facilities are side 3. The trailing `1, 1` set the created bit
(`+0x50` `0x2`, `FUN_004f7480`) and the completed bit (`+0x50` `0x4`,
`FUN_004f74f0`).

## Control change

1. A system's control word changes; its control kind is `+0x78` bits 0..1.
2. `FUN_00510d70` calls `FUN_004f6f40(system, new & 3)`, the side setter. It
   writes the bits (`FUN_00540670`), updates both side views, and calls slot
   `+0xf0` with (old, new).
3. For a system, slot `+0xf0` is `FUN_00510820`. It first runs
   `FUN_004fae40`, which walks every object in the system (`FUN_004fcd00(this,
   3)`) and calls each one's slot `+0xb0`. It then reruns the derived system
   state: blockade `FUN_0050b890`, informant timers `FUN_0050c820`, and
   others.
4. Slot `+0xb0` is `FUN_004f8680` in every object class, all ten facility
   classes included. It reads the object's `+0x50`:
   - **completed (`0x4`) and not `0x20`:** if the class accepts the
     container's new side (slot `+0x50`), it calls `FUN_004f6f40(object,
     side)`, so the facility changes hands. Otherwise it calls slot
     `+0xac(3)` → `FUN_004f8660` → `FUN_004f71d0`, which sets the state byte
     `+0x40` to 3. That is the removal path: reason `0x14` disbands a fleet
     (`fleet-join-split.md`), and the incidents destroy with reasons 8 and
     `0xb` (`uprising-incident.md`). So the facility is removed.
   - **otherwise**, unless `0x8` is set, `FUN_004f78e0(1)` sets `+0x50`
     `0x1000`. hyp: an unfinished facility is flagged rather than handed over;
     the meanings of `0x20` and `0x1000` are not traced.

Every facility class uses `FUN_004f27d0` for slot `+0x50`:

| Family | Factory → constructor | Vtable |
|---|---|---|
| `0x20` HQ | `FUN_00526ad0` → `FUN_005267c0` | `0x0065f280` |
| `0x22` | `FUN_00527450` → `FUN_00527370` | `0x0065f828` |
| `0x23` | `FUN_005271c0` → `FUN_005270e0` | `0x0065f640` |
| `0x24` | `FUN_00527740` → `FUN_00527660` | `0x0065fa10` |
| `0x25` | `FUN_0051c140` → `FUN_0051c060` | `0x0065eaf8` |
| `0x28` | `FUN_0052c930` → `FUN_0052c850` | `0x00660280` |
| `0x29` | `FUN_0052c610` → `FUN_0052c530` | `0x00660070` |
| `0x2a` | `FUN_0052c2e0` → `FUN_0052c200` | `0x0065fe60` |
| `0x2c` | `FUN_0052d3e0` → `FUN_0052d300` | `0x006608c8` |
| `0x2d` | `FUN_0052cfb0` → `FUN_0052ced0` | `0x006606a0` |

The factory registry is `FUN_0051ebf0` → `FUN_00540540(family, factory)`.

`FUN_004f27d0(side)` accepts any side 1..3 when the object's `+0x50` bit
`0x40` is clear. When it is set, the class must serve the side:

- side 1: class `+0x40`, the DAT `is_alliance`;
- side 2: class `+0x44`, `is_empire`;
- side 3: both.

`uprising-incident.md` calls `+0x50` bit `0x40` GameObjExisting, so the check
applies to every live facility. Classes such as systems use `FUN_0051ebb0`,
which always accepts.

The system invariant check `FUN_0050e820` confirms the rule. It walks the
system's object lists and requires each member's side to equal the
system's (`((obj +0x24 ^ sys +0x24) & 0xc0) == 0`). It exempts members with
`+0x50` bit `0x8` and bit `0x4000`, and characters with `+0x78` bit 7.

## Contested systems

Ownership is never read from the system on use. It is the object's own
side, kept equal to the system's side by the cascade above. A battle
(both sides present) only sets the system's battle bit `+0x88` `0x1000`
(`blockade-bit.md`, `FUN_0050b8e0`). The side does not change, so the
holder keeps its yards and mines until control actually changes.

## The Alliance HQ (family `0x20`)

- ALLFACSD has one record, `0x20000001`, with TEXTSTRA 9024, `is_alliance` 1
  and `is_empire` 0. The layout matches MANFACSD.
- Its constructor `FUN_005267c0` builds on the generic object base
  `FUN_00539980`. The yards build on `FUN_0052cbc0`, the defense facilities
  on `FUN_00526e00`, and the mines on `FUN_0055a600`.
- The production managers map their producer types only to `0x28`, `0x2a`
  and `0x29` (`FUN_00537ff0`, `manufacturing-build-selection.md`).

**So the HQ is not a yard and produces nothing.** Because its class serves
only the Alliance, a completed HQ whose system turns Empire (side 2) or
Neutral (side 3) is removed with reason 3. `move-order.md` records its one
other traced role: an ALLFACSD object in a blockaded system restricts
departures (`FUN_005555e0`).

## Mine and refinery

`FUN_00559850` and `FUN_00559a60` seed a mine as `0x2c000001`, and the
Manufacturing window shows mines on page `0x6c` (family `0x2c`) and
refineries on page `0x6b` (`0x2d`). **The mine is `0x2c` and the refinery is
`0x2d`.**

## Port implications

- **Seeding.** A facility seeded from SYFCCRTB, SYFCRMTB or FACLHQTB takes
  its system's side at seeding. A neutral system's facilities belong to
  neither side, and neither side's production managers use them. A mine
  is `0x2c`; `0x2d` from the SYFC tables is a refinery.
- **Capture and control change.** When a system's holder changes, each
  completed facility there passes to the new holder if its class serves
  that side (DAT `is_alliance` / `is_empire`; both for neutral). Otherwise
  it is removed. hyp: a facility still under construction or en route is not
  handed over.
- **Contested.** A battle does not change ownership. The holder keeps its
  facilities until control changes. The port's `ControlKind::Contested`
  carries no faction, so the port must remember the last holder rather than
  drop the facilities' side.
- **Neutral.** Side 3, served by neither side's managers. A facility moves to
  neutral only when its class serves both sides.
- **HQ.** Family `0x20` is not a producer. Do not count it as a shipyard,
  training facility or construction yard. It is removed when its system
  leaves the Alliance.
- Deriving the side from the system on read matches every case above except
  the removal of a facility whose class cannot serve the new holder. That
  removal has to happen once, when control changes.

## Uncertainties

- The meanings of `+0x50` bits `0x20`, `0x1000` and `0x4000` are untraced.
  The handling of unfinished facilities above follows the code path only.
- `+0x40` state 3 is read as a removal from the other reason codes. Its view
  handler (`+0x100`) is not read here.
- The full list of control-word writers (battle won, uprising, loyalty) is
  not enumerated. They converge on `FUN_00510d70` → `FUN_004f6f40`.
- Lists `+0x140` hold the two special systems by id. hyp: they are the
  Alliance base and Coruscant.
