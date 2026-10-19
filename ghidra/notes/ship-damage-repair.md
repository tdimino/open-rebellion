# Ship damage, the damaged flag, repair, and what the side AI does with them

Recovered 2026-10-15 from REBEXE.EXE with Homebrew Ghidra 12.1.4 (read-only,
headless). Every decompile cited is saved beside this note as `FUN_*.c`.
"hyp:" marks a reading no decompile states outright.

Notation: `ship+` a capital ship object (`CCapitalShip`, vtable `0x0065d650`,
family bytes `0x14`-`0x1b`), `cls+` its class record, `sys+` a system object.
A ship's virtual slots are named by offset; the slot-to-function map is the
vtable dump of `0x0065d650`.

## Damage state of a capital ship

| Field | Meaning | Source |
|---|---|---|
| `ship+0x60` | hull damage (slot `+0x274` returns it). The ship is destroyed when it reaches the maximum hull (slot `+0x248`, `cls+0xc0`) | `FUN_00503100.c`, `FUN_00501ae0.c` |
| `ship+0x64` bits 0-3 | shield damage counter (setter `FUN_00501510`) | `FUN_00501510.c` |
| `ship+0x64` bits 4-7 | weapon damage counter (setter `FUN_005015a0`) | `FUN_005015a0.c` |
| `ship+0x64` bits 8-11, 12-15 | two further counters (setters `FUN_00501640`, `FUN_005016e0`) | `FUN_00501640.c` |
| `ship+0x64` bits 16-19, 20-23 | the two drive counters (setters `FUN_00501780`, `FUN_00501820`); their class figures are slots `+0x1ec`, `+0x1f0`. hyp: the primary and the backup hyperdrive | `FUN_005011f0.c`, `FUN_00501230.c` |

Each counter runs from 0 to GNPRTB `0x800` (shipped 4 on every difficulty),
held in `DAT_006bb388` (`FUN_00500670.c`, bound by `FUN_00558510.c`; the range
check is `FUN_0053fc90(value, 0, max)` in the setters).

The amount a counter removes from a class figure is figure × counter / 4
(`FUN_0053e170`, `FUN_0053e150`): the weapon figure `cls+0xdc` (slot `+0x264`,
`FUN_005005f0.c`) loses `+0x290` (`FUN_00500f60.c`, the weapon counter) and
the shield figure `cls+0xe0` (slot `+0x268`, `FUN_00500610.c`) loses `+0x294`
(`FUN_00500fa0.c`, the shield counter). `cls+0xd8` is the Damage Control
Rating (slot `+0x260`, `FUN_005005d0.c`; `status-window.md`).

### Applying damage (`FUN_005018c0`)

`FUN_005018c0(amount)` asks `FUN_005585e0(maxHull - hullDamage, amount, &part)`:

- remaining hull below 1: part 7, the ship is destroyed (slot `+0xac`);
- amount 0: part 1;
- otherwise `rand(0..=GNPRTB 0x801)` plus the percentage `amount / (amount +
  remaining hull)` (`FUN_0053e1b0`) reaches GNPRTB `0x802` (shipped 50): the
  part is read from table record type 5 at `rand(0..=GNPRTB 0x803)` (shipped
  50) (`+0x24` of that record, 2 to 6); below that threshold the part is 1.

Part 1 adds hull damage only, parts 2 to 6 add one to the shield, weapon, bits
8-11, bits 12-15 and drive counters (part 6 the primary drive, else the backup
once the primary is at its maximum), part 7 destroys. The shipped GNPRTB
values are all 50, 50, 50 (`GNPRTB.DAT` ids `0x801`-`0x803`). Files:
`FUN_005018c0.c`, `FUN_005585e0.c`.

### The damaged bit

`ship+0x50` bit 9 (`0x200`) holds "damaged": slot `+0x298` (`FUN_00501c00.c`)
sets it when the hull damage is not 0 or `FUN_005006a0` (the sum of the six
counters, `FUN_005006a0.c`) is not 0 (setter `FUN_004f7800.c`, `FUN_0053a640`
mask `0x200`). The hull-change notification `FUN_00502930.c` (slot `+0x29c`)
and each counter-change notification `FUN_00502980.c` (slots `+0x2a0`-`+0x2b4`)
call it.

## Repair

Repair is a timer. The per-class timer record is `cls`-side record `+0x18`
(`FUN_0053fa60`, `FUN_0053fab0`; `timer-scheduler.md`, "Arming").

| Step | Function | Fact |
|---|---|---|
| Under-repair flag | `FUN_005578d0.c` (called by the status-change hook `FUN_00557b60.c`) | `ship+0x5c` bit 0 = damaged (`+0x50` bit 9) and usable (`+0x50` bit 0) |
| Arm or cancel | `FUN_005577f0.c` → slot `+0x22c` `FUN_00557c00.c` | `FUN_0053fa60(0x380, flag, ship, record)`: timer `0x380` armed while the flag is 1 |
| Fast-repair flag | `FUN_0050c580.c` → `FUN_00557860.c` | `ship+0x5c` bit 1 = the ship's system has `sys+0x88` bit 10 (`0x400`) and the ship's side (`+0x24` bits 6-7) is the system's |
| Interval | `FUN_00557860.c` → slot `+0x230` `FUN_00557c30.c`; slot `+0x1d8` `FUN_005010a0.c` | `FUN_0053fab0(0x380, slot +0x1d8, ...)` sets the timer's minimum delay: GNPRTB `0x804` with the fast flag (shipped 2), GNPRTB `0x805` without (shipped 30), in days |
| System flag | `FUN_0050bb70.c` → `FUN_0050a820.c` | `sys+0x88` bit 10 = the system's shipyard producer (`FUN_00509670(sys, 0)`) counts a yard: `FUN_00528040(producer, 0) != 0` |
| Flag push | `FUN_0050c350` (callers `FUN_00508250`, `FUN_00508da0`, `FUN_00510820`) | for every ship and squadron in the system's fleets, `FUN_0050c580` recomputes the fast-repair flag; `FUN_00508da0` does it for a unit that arrives |
| Timer class | factory `FUN_0057c160.c`, ctor `FUN_0057c080.c`, vtable `0x00669870`, range `FUN_0057c1d0.c` (`0x14`-`0x20`), resolve `FUN_0057c1f0.c`, fire `FUN_0057c2c0.c` | the fire calls slot `+0x228` of the target |
| Repair step | slot `+0x228` `FUN_00501c50.c` | see below |

The repair step `FUN_00501c50`:

1. With hull damage above 0, it removes one tenth of the maximum hull
   (`FUN_00501b70(maxHull / 10)`).
2. With any counter above 0, it rolls `rand(0..=99) < damage control`
   (`FUN_00558680.c` → `FUN_0053e2f0` → `FUN_0053e2e0`; damage control is slot
   `+0x260` minus slot `+0x28c`, which returns 0). On success it removes one
   point: `FUN_0053e290(sum - 1)` picks a point at random and the counters
   are walked in the order shield, weapon, bits 8-11, bits 12-15, primary
   drive, backup drive, the first counter whose running total covers the point
   losing one.

Repair therefore runs on every damaged usable ship (`+0x50` bit 0: existing,
completed and not en route, `object-state-flags.md`), every 30 days, and every
2 days while its system holds a counted shipyard of its own side. A ship in
hyperspace has `+0x50` bit 4 set, so its under-repair flag is 0 and the timer is
cancelled (the manual, "Damaged ships", says the same).
hyp: the timer's minimum delay for a ship that never changed its fast-repair
flag is the record's creation value.

The tactical write-back is the only other writer of the damage fields
(`FUN_0040cfc0`, `FUN_0040d170`; `simulate-runs-tactical-engine.md`).

## What the side AI reads

### Ship record scaling (`FUN_00484630`)

The AI's ship record is rebuilt from the object by `FUN_00484630` (caller
`FUN_00433620`, the fleet refresh). With `W` = `cls+0xdc`, `S` = `cls+0xe0`,
`wl` = `W × weapon counter / 4` and `sl` = `S × shield counter / 4`:

| Ship record | Value |
|---|---|
| `+0x50` shield | `FUN_00473910(node, S - sl)` = node `+0x3c` × (`S - sl`) / node `+0x54` |
| `+0x7c`, `+0x90` attack | `FUN_004738d0(node, W - wl)` = node `+0x30` × (`W - wl`) / node `+0x50` |
| `+0x80`, `+0x94` manoeuvre (laser ships, `+0x24` bit 2) | `FUN_004738f0(node, W - wl)` = node `+0x34` × (`W - wl`) / node `+0x50` |
| `+0x44` hull | node `+0x40` (the class hull) |
| `+0x48` bombard flag | node `+0x38`, only when the ship is not flagged `0x1000` |

The fleet refresh sums these into the fleet record (`FUN_00433780`: ship
`+0x7c` into fleet `+0x58`, `+0x80` into `+0x5c`, `+0x50` into `+0x60`, `+0x90`
into `+0xb4`, `+0x94` into `+0xb8`); `FUN_00477cb0` sums fleet `+0xb4`, `+0xb8`
into the regional record's `+0x6c`, `+0x70`, whose shortfalls drive the posture
needs and module 11 (`ai-production.md`, U10 item 2). Damage therefore lowers
the figures the AI compares with its needs. `FUN_00484630` is the one AI function in the corpus that calls the damage slots (`+0x264`, `+0x268`, `+0x290`, `+0x294`) of a ship.

### Ship flags `0x1000` and `0x800000`

`FUN_00484630` sets, when `ship+0x50` bit 9 is set (damaged), ship record
`+0x24` bit `0x800000` when `FUN_00500820(1)` (slot `+0x34`, `FUN_00500820.c`)
is 0, else bit `0x1000`. Slot `+0x34` returns the primary drive's remaining
figure (`+0x1ec - +0x210`), the backup's (`+0x1f0 - +0x214`) when that is 0,
and a final id comparison (`FUN_004f63f0`) when both are 0. hyp: `0x1000` is a
damaged ship that can still jump, `0x800000` one whose drives are both out.

Readers:

- `FUN_00433780.c`: a ship with either bit is left out of the fleet's class
  counts (`+0x98`, `+0x9c`, `+0xa0`) and does not give the fleet bit `0x8000`;
  `0x1000` gives the fleet `0x1000`, `0x800000` gives it `0x2000`.
- `FUN_004749b0.c` (fleet task state 2) and `FUN_00477a40.c` (regional record
  Create Fleet): when the fleet carries `0x3000` and none of `0xf0000802`, a
  Create Fleet order splits the ships with `0x801000` that are not en route
  (`0x800` clear) into a new fleet.
- `FUN_004c94c0.c` queries fleets excluding `0x3802`: it skips damaged
  fleets.
- `FUN_004c8e90.c`: the repair move below.

### The repair move (`FUN_004c8e90`, module 11 phase `0x3f8` step 2)

`FUN_004c8ba0.c` runs phase `0x3f8` as step 2 `FUN_004c8e90`, step 3
`FUN_004c94c0`, step 4 `FUN_004c9cb0`. Step 2:

1. Candidates: `FUN_0041a4c0(c, 0x1005, 0x802, 0, largest)`: fleets that are
   the side's (`+0x38` bit 1), carry no record or task (`+0x38` bit 4, set at
   creation by `FUN_00432e40` and when a record releases a fleet by
   `FUN_004791d0`, cleared when a record or task adopts it by `FUN_00478fd0`,
   `FUN_00475fe0`), carry a damaged ship
   (`0x1000`), are not en route (`0x2`) and hold no en-route ship (`0x800`),
   ranked by ship count (`+0x50`), largest first, ties by the rank-sum
   selection (`ai-production.md`, F0.3).
2. Pick: the first candidate whose system is held by the other side
   (`sys+0x30` bit 4); when none, the first whose system record lacks the
   shipyard role (`sys+0x28` bit `0x200000`).
3. Ships: the candidate's ships whose record has `0x1000` and none of
   `0x800800`.
4. Target: the nearest sector to the fleet with sector `+0x24` bit 8 (own, not
   blockaded) and `+0x2c` bit `0x20000` (a shipyard-role system that is not
   blockaded), then in it the nearest system with `sys+0x28` bit `0x200000`
   and bit 2 clear (`FUN_0041a400(c, sector, fleet, 0x200000, 0, 0, 2, 0, 0,
   nearest)`, `FUN_00431c00.c`). With none: the nearest sector with `+0x24`
   bit 8 and `+0x2c` bit 4 (an own system), then its nearest own system with
   `sys+0x28` bit 2 clear.
5. Order: Move `0x201` of those ships to the target.

The target set is the systems that modules 11, 10 or 7 list as shipyard sites
(`ai-production.md`, F1.2 `0x200000`); a system holding a shipyard that no
module has listed qualifies only through the own-system fallback. The
fast-repair interval applies in whichever system the fleet stands.

## Port

- `ShipInstance::damage_counters` holds the six counters (shield, weapon,
  two further read as tractor and engines, primary and backup drive; hyp for
  the two further), `is_damaged` is `FUN_00501c00`, and `repair_timer` is
  timer `0x380`. The battle import starts from the counters and
  `tactical::apply::apply_results` writes all six back (`FUN_0040cfc0`).
- `ai_records::ship_record` scales attack, manoeuvre and shield by the
  counters over GNPRTB `0x800` against the class `weapon_recharge_rate` and
  `shield_recharge_rate` (`cls+0xdc`, `cls+0xe0`), sets `0x1000` or
  `0x800000` from `movement::capital_ship_instance_speed`, and drops the
  bombard figure under `0x1000`. `fold` leaves damaged ships out of the
  fleet ready bit and laser count and gives the fleet `0x1000` or `0x2000`.
  The splits in `fleet_tasks.rs` and `ai_regions.rs` read those bits.
- `ai_production.rs` module 11 cycles `0x3ec`, `0x3f8`, `0x3f9`. Phase
  `0x3f8` runs step 2 (`repair_move`) and emits `Order::MoveShips`, which
  becomes `AIAction::CreateFleet` to the target. Steps 3 and 4
  (`FUN_004c94c0`, `FUN_004c9cb0`) emit nothing, and no Move goes to the
  fleet's own system (hyp).
- `repair.rs` runs timer `0x380`: 30 days (GNPRTB `0x805`), 2 days
  (`0x804`) at a system holding a family `0x28` facility of the ship's side
  (hyp for "counted"); a change of interval rearms; the timer spread is 0
  (hyp). A step heals max hull / 10 and, on a roll under damage control,
  removes one counter point chosen over the counters in the order above,
  with two rolls from the simulation stream. The port keeps the hull as
  remaining hull (`ShipInstance::hull_current`) where the object keeps
  damage.

## Decompiles

`FUN_00484630.c`, `FUN_00433780.c`, `FUN_00433620.c`, `FUN_004c8e90.c`,
`FUN_004c8ba0.c`, `FUN_004c8b30.c`, `FUN_004749b0.c`, `FUN_00477a40.c`,
`FUN_00501c50.c`, `FUN_00501c00.c`, `FUN_00502930.c`, `FUN_00502980.c`,
`FUN_005018c0.c`, `FUN_005585e0.c`, `FUN_00558510.c`, `FUN_00558680.c`,
`FUN_005010a0.c`, `FUN_005005d0.c`, `FUN_005005f0.c`, `FUN_00500610.c`,
`FUN_005005b0.c`, `FUN_005006a0.c`, `FUN_005006e0.c`, `FUN_00500820.c`,
`FUN_00500f60.c`, `FUN_00500fa0.c`, `FUN_005011f0.c`, `FUN_00501230.c`,
`FUN_00501510.c`, `FUN_005015a0.c`, `FUN_00501ae0.c`, `FUN_00501b70.c`,
`FUN_00503100.c`, `FUN_004f7800.c`, `FUN_005577f0.c`, `FUN_00557860.c`,
`FUN_005578d0.c`, `FUN_00557b60.c`, `FUN_00557c00.c`, `FUN_00557c30.c`,
`FUN_0050c580.c`, `FUN_0050bb70.c`, `FUN_0050a820.c`, `FUN_00528040.c`,
`FUN_0057c160.c`, `FUN_0057c080.c`, `FUN_0057c1d0.c`, `FUN_0057c1f0.c`,
`FUN_0057c2c0.c`, `FUN_0041a4c0.c`, `FUN_0041a400.c`, `FUN_00431c00.c`,
`FUN_00432350.c`.
