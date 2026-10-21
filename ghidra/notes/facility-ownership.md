---
title: "Facility Ownership"
description: "Which side a facility belongs to: the system's side at seeding, the cascade that hands a system's completed facilities to its new side (or removes those whose class cannot serve it), the HQ's class, and the mine/refinery families"
category: "ghidra"
created: 2026-10-07
updated: 2026-10-19
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
4. Slot `+0xb0` is `FUN_004f8680` in the facility classes, all ten of
   them. The fleet class (vtable `0x0065d438`) has `FUN_0051ebb0` there,
   which returns 1 and changes nothing, so a fleet and the ships in it keep
   their side. `FUN_004f8680` reads the object's `+0x50`:
   - **completed (`0x4`) and not `0x20`:** if the class accepts the
     container's new side (slot `+0x50`), it calls `FUN_004f6f40(object,
     side)`, so the facility changes hands. Otherwise it calls slot
     `+0xac(3)` → `FUN_004f8660` → `FUN_004f71d0`, which sets the state byte
     `+0x40` to 3. That is the removal path: reason `0x14` disbands a fleet
     (`fleet-join-split.md`), and the incidents destroy with reasons 8 and
     `0xb` (`uprising-incident.md`). So the facility is removed.
   - **otherwise** (not completed, or on its own leg `0x20`), unless `0x8`
     is set, `FUN_004f78e0(1)` sets `+0x50` `0x1000`, the autoscrap request,
     with no side test (`FUN_004f8680.c:14-17`). It raises event `0x307`
     through slot `+0x13c` (`FUN_004f2bd0` → `FUN_004fba70`), and
     `FUN_00515480` looks for
     a place for the object (`system-squadrons-combat.md`, section 3b): the
     system keeps it while it accepts the object's side and is not
     blockaded, else the hint (an unfinished product's producer system),
     else the nearest system of its side; with none it is destroyed with
     reason 2. An unfinished product moves with no transit and keeps its
     manager's queue slot.
5. **Production managers.** The three managers (`0xa0`, `0xa2`, `0xa4`;
   vtables `0x0065fbf8` and `0x006629a0`) are completed children, and slot
   `+0x50` is `FUN_0051ebb0`, so each takes the new side (`FUN_004f8680.c:28-31`
   → `FUN_004f6f40`). Its side hook, slot `+0xf0` `FUN_0052b790`, runs
   `FUN_0052a2c0`, then `FUN_0052a360(0)` (`FUN_00529710`: `+0x70` = 0)
   (`FUN_0052b790.c:7-14`). `FUN_0052a2c0` destroys every product in the
   manager's list with reason `0x16` (`FUN_0052a2c0.c:20-36`): facilities,
   regiments, squadrons, special forces and capital ships, wherever they
   sit, none refunded (`FUN_00530270`). A product that sits in the system
   but belongs to another system's manager is only flagged as above; a
   capital ship product sits in a fleet, which the walk leaves alone. The
   Empire game of the Duros line shows it: on day 306 Duros
   (`0x9000008f`) is Imperial and its facility manager `0xa0000523` lists
   an unfinished Orbital Shipyard (`0x28000e75`, `+0x50` `0x42`); on day
   430 Duros and the manager are the Alliance's, the manager lists only the
   Alliance's own Build (`0x29001059`), and the shipyard has no record in
   any graph, though Duros's construction yard (`0x2a00077e`) stands. A
   replay of that line saved on day 378, after the capture, holds the
   shipyard as a destroyed record (`+0x50` `0xa`, reason word `0x16`,
   Empire-owned) and the Imperial special force `0x3c000a17`, alive at Duros
   on day 320, destroyed with reason 3 (`FUN_00503d90`, its slot `+0xb0`),
   so the walk reached Duros's children. The Alliance's own Build already
   stood on day 378, and it destroys with the same reason, so these saves
   agree with the trace without separating the capture from that Build.
6. **Yards.** A yard's side hook (slot `+0xf0`, `FUN_0052cd40` →
   `FUN_0053b0f0`) stops it (`FUN_0053ab10`, state 4).

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

### The holder rule (`FUN_0050b610`)

The system keeps no contested value. Its holder (`+0x24` bits 6-7, written
from the control word's bits 0-1 by `FUN_00510d70` → `FUN_004f6f40`) is 1, 2
or 3 (neutral). `FUN_0050b610` rebuilds the control word `+0x84` on every call
(`FUN_0050b610.c:24-70`):

- bits 2-3 are the support side (`FUN_0055a080` of `+0x58`; 3 when the system
  is unpopulated, `:24-33`);
- bits 4-5 are the troop side: `FUN_005131b0` over the system's usable
  regiments (`FUN_00504cc0(system, 1, side)`) for side 1 and for side 2. None
  gives 0, only side 1 gives 1, only side 2 gives 2, and both sides give the
  word's old bits 4-5 unchanged (`:34-61`; the copy is `FUN_00558840` of the word,
  read back as `local_30 >> 4 & 3`);
- bits 0-1, the holder, are bits 4-5 when they are nonzero, else bits 2-3
  (`:64-70`).

With both sides' regiments present, the side that held the system's troop word
before keeps it. When that word was 0 (the first rebuild of a system that both
sides' regiments reached together, as in `FUN_00508250` at setup), the holder
is the support side, which is 3 for an unpopulated system. Nothing stores a
separate "earlier holder". A system therefore reads as side 3 exactly when
its troop bits are 0 and its support side is 3: it is unpopulated, or no
side's support reaches GNPRTB 7760 (`FUN_0055a080`). The port's
`ControlKind::Uncontrolled` is that state (`economy::resolve_system_control`,
the troop bits `GameWorld::troop_held`), and the render side code of an
`Uncontrolled` system or a neutral facility is 3 (`fleet_window::control_side`,
`faction_side`).

### What reads the holder

Every reader below sees the earlier side, never a contested state. Locating
instruction: `SHR reg, 6` on `+0x24` (306 hits in 175 functions across all
object classes); the role is from the cited note.

| Reader | Original | Note |
|---|---|---|
| Blockade bit | `FUN_0050b8e0` | `blockade-bit.md` |
| Sector shift owner filter and gate | `FUN_00518be0`, `FUN_00517aa0`, `FUN_0055de50` | `support-shifts.md` |
| Drift, garrison requirement, collection rate | `FUN_0050b230`, `FUN_0050b5a0`, `FUN_0050b2c0` | `support-shifts.md` |
| Uprising state, score and timers | `FUN_0050bb00`, `FUN_0050b4d0`, `FUN_0050b800`, `FUN_0050caa0`, `FUN_0050cad0`, `FUN_0050d030` | `uprising-incident.md` |
| Informant draw | `FUN_0050d510` (holder not 3; reveals to the opponent) | `observation-levels.md` |
| Blockade end | `FUN_00515f40`, `FUN_00517ee0` | `support-shifts.md` |
| Facility hand-over and invariant | `FUN_00510820` → `FUN_004fae40`, `FUN_0050e820` | above |
| Victory word (CA: Coruscant owner 1) and Coruscant's shift | `FUN_005180e0`, `FUN_00517be0` | `victory-flag-word.md`, `support-shifts.md` |
| Move and Confirmed Move choice | `FUN_00487cc0` | `move-order.md`, `blockade-bit.md` |
| Computer player's system record | `FUN_00433e40.c:94` | `ai-order-emission.md`, `ai-production.md` |
| System status window, system window, sector window icons and name colours | `FUN_00446250` (state = side bits), `FUN_004a8790` (side shown), `FUN_0045b770` → `FUN_0045ca80`, fleet icon `FUN_0045d140` (the system's side bits) | `status-window.md`, `sector-quadrants.md`, `fleet-window.md` |
| Bombardment and assault events (the event's side field is the holder at the event) | `FUN_00512320`, `FUN_00512370` | `operation-report-window.md` |

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
  it is removed. A facility still under construction or en route is not
  handed over: it takes the autoscrap request. The system's own production
  managers take the new side and destroy every product they list
  (`manufacturing::serve_manager_sides`).
- **Contested.** A battle does not change ownership. The holder keeps its
  facilities until control changes; both sides' regiments keep the earlier
  holder (`FUN_0050b610.c:45-70`).
- **Neutral.** Side 3, served by neither side's managers. A facility moves to
  neutral only when its class serves both sides.
- **HQ.** Family `0x20` is not a producer. Do not count it as a shipyard,
  training facility or construction yard. It is removed when its system
  leaves the Alliance.
- Deriving the side from the system on read matches every case above except
  the removal of a facility whose class cannot serve the new holder. That
  removal has to happen once, when control changes.

### Port of the holder change

At a holder change the port serves what the child walk reaches in the
system, after the change's phase 5 and before its operation returns:

- **The system's own managers' products** die
  (`manufacturing::serve_manager_sides`).
- **Facility, regiment and squadron products** of another system's
  managers bound for the system take the autoscrap request:
  `FUN_004f8680.c:14-17` (facilities, regiments) and `FUN_00503510`
  (squadrons, slot `+0xb0` of vtable `0x0065def0`). Event `0x307` runs
  `FUN_00515480`. The system keeps the product while it accepts the
  product's side and is not blockaded (`FUN_00553410`,
  `FUN_005073d0.c:126-141`); else the hint, the producer's system
  (`FUN_00515480.c:82-124`), then the nearest system of its side with
  room (`FUN_005545e0`, `FUN_005571c0` → `FUN_00551630`); else `+0xac(2)`.
  The move is a reparent with no transit, and the product keeps its queue
  slot (`FUN_00528890`).
- **Special-force products.** Slot `+0xb0` is `FUN_00503d90`: the product
  takes the new side when its class serves it (slot `+0x50`, `FUN_004f27d0`),
  else `+0xac(3)`. `port:` a queued special force whose class serves both
  sides keeps the side the port reads from its class.
- **Completed products, regiments and squadrons on their way** to the
  system (`+0x50` `0x20`) take the request too (`FUN_004f8680.c:14-17`),
  with their transit origin as the hint (`FUN_00515480.c:125-153`), and
  keep their arrival day (`FUN_00556390.c:17-21`).
- **Capital-ship products** of another system's manager sit in a fleet,
  which the walk leaves alone (`FUN_0051ebb0`).

Destroy System's cascade serves the same requests with the system refusing
everything (`death_star::request_bound_for`); a holder change serves them
with `death_star::serve_holder_requests`.

## Uncertainties

- The meaning of `+0x50` bit `0x4000` is untraced. Bit `0x20` is the
  object's own transit leg (`GameObjEnrouteActiveNotif`,
  `object-state-flags.md`).
- `+0x40` state 3 is read as a removal from the other reason codes. Its view
  handler (`+0x100`) is not read here.
- The full list of control-word writers (battle won, uprising, loyalty) is
  not enumerated. They converge on `FUN_00510d70` → `FUN_004f6f40`.
- Lists `+0x140` hold the two special systems by id. hyp: they are the
  Alliance base and Coruscant.
