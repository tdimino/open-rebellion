# Top-bar resource counters

What the three numbers at the top of the command center show, traced from
the paint path back to the side object's stockpiles.

## Display

`FUN_00422620` takes the viewing side's object (`FUN_004f3dd0`: key
`0xf3000000` with index `0x0f` for side 1, `0x10` otherwise) and calls
`FUN_00429200(window, message, value)`:

| Message | Value | Box (`FUN_00422ce0`) | Tooltip |
|---|---|---|---|
| `0x14c` | side `+0x78` | `+0x178`, left | `0x1510` 5392 "Raw Materials Monitor" |
| `0x14d` | side `+0x7c` | `+0x1ac`, middle | `0x1511` 5393 "Refined Materials Monitor" |
| `0x14e` | side `+0x58` − side `+0x74` | `+0x1e0`, right | `0x1512` 5394 "Maintenance Monitor" |

- `FUN_00429200` writes the number with `FUN_00601e20` into the box. Only
  `0x14e` above 99999 is replaced by TEXTSTRA `0x1302` (MAX).
- `FUN_0041bde0` copies the side's `+0x58` pair; `FUN_00429300` the `+0x68`
  pair; `FUN_004292e0` the `+0x60` pair (`FUN_00520580` copies 8 bytes).
- The tooltip registrations at `0x423058..0x423209` pass the same message id
  with each TEXTSTRA id and rectangle, so the left box is raw materials.

## Raw and refined stockpiles (`+0x78`, `+0x7c`)

These are counts of whole units, changed one at a time by facility cycles.

| Change | Function | Trigger |
|---|---|---|
| raw +1 | `FUN_00530670` → `FUN_0052ef90` | a mine (`0x2c`) entering state 3 |
| raw −1 | `FUN_005306b0` → `FUN_0052fb30` | a refinery (`0x2d`) entering state 1 |
| refined +1 | `FUN_005307e0` → `FUN_0052eff0` | a refinery entering state 3 |
| refined −1 | `FUN_00530820` → `FUN_0052fb80` | a yard (`0x28..0x2b`) entering state 1 |
| refined + value/2 | `FUN_00530270` | a scrapped capital ship (`0x14..0x16`, `+0x50` bit 2): `FUN_004f2980` (class `+0x48`, refined cost) / 2 |

- `FUN_00516360(key, _, state, ctx)` dispatches by family. Its recurring
  caller is `FUN_00578a40`, the fire slot of event `0x315`, which
  `FUN_0053b1d0` raises on a facility state change.
- When the stockpile is empty, `FUN_0052fb30`/`FUN_0052fb80` return
  "none". The facility then queues a request (`FUN_0052fbb0`/`FUN_0052fbd0`,
  side `+0x80` counts raw waiters), and `FUN_0052fbf0` serves the oldest
  waiter when raw arrives.
- A finished mine or refinery cycle can pay the enemy instead:
  `FUN_005166a0` reads the system's record from `FUN_005185c0` and rolls
  `FUN_0053e2f0(|record[0x1b]|)`. On success the unit goes to the other side
  (`FUN_00506f30(2 - (side != 1))`).

### Facility cycle (`+0x58`, `FUN_0053aa40`)

| State | Leaves when | To |
|---|---|---|
| 1 waiting | `+0x60` bit 1 set (input taken) | 2 |
| 2 working | `+0x60` bit 2 set (timer `0x394`, `FUN_0053aa20`) | 3 |
| 3 done | `+0x60` bit 1 cleared (output paid) | 1 |
| 4 | the facility's side bits are `0xc0` | — |

- A mine needs no input: `FUN_00530650` sets bit 1 at once.
- A refinery takes 1 raw and a yard 1 refined (`FUN_005306b0`,
  `FUN_00530820`). Paying the output clears bit 1 (`FUN_0053a860(_, 0)`).
- `FUN_0052fb30` spends raw only while `DAT_006b90e0` is set, which
  `FUN_0051c010` sets when the game mode is `0x1a`: always during play.
- `FUN_0053b1d0(old, new)` runs on every state change. It sets `+0x60` bit 4
  when the state goes from 4 to 1 (the first cycle after the facility comes
  online), clears it on entering 4, and keeps it otherwise. It sets bit 3
  while the state is 2, counts entries to and exits from state 1 in the timer
  block's `+0x24`, and raises event `0x315`.

### Cycle length

Timer `0x394` is armed by `FUN_0053b330` (mine vtable slot 120) for the
delay stored at the object's timer block `+0x20` (the block is a 0x28-byte
object from vtable slot 58, `FUN_0053b650` → `FUN_00584590`, stored at
object `+0x54` by `FUN_004fc5c0`). The setter is `0x501ba0`
(`*(obj+0x54)+0x20 = arg`, a function Ghidra had not split out).

- Yards: `FUN_0052cc90` stores `FUN_00520b70` = class `+0x5c`, the DAT
  `processing_rate`.
- Mines and refineries: `FUN_0055abf0` stores the output of vtable slot
  `0x218` (`FUN_0052d1a0` → `FUN_0055e2b0`), or 0 if it fails:

```
k     = max(1, capacity * GNPRTB[4096] / 100)        // 50 * 20 / 100 = 10
d     = (ceil(allocated / k) + processing_rate) * efficiency / 100
if +0x60 bit 4 (first cycle): d = rand(0..=d) + d / 2
```

- `processing_rate` is PROFACSD 5 for both the mine and the refinery.
- `efficiency` is the facility's `+0x6c` (100 at construction,
  `DAT_00661a88`). `FUN_0050c2b0` → `FUN_0050c560` → `FUN_0055a7a0` copies
  it from the system's `+0x70`, which `FUN_0050b2c0` sets to 100 when the
  system is unheld and otherwise to
  `FUN_00559c10(support) = GNPRTB[7763] * 100 / max(support, 1)`, the
  controlling side's support (`FUN_00507270`: `+0x58` for side 1,
  `100 - +0x58` otherwise). The port already computes this as
  `economy::calculate_collection_rate`.
- `allocated` is the facility's share of the maintenance load
  (`FUN_0052f6b0`/`FUN_0052f8f0` with `FUN_0055a820`/`FUN_0055a960`). Every
  10 units of load add a day to its cycle.

## Maintenance (`+0x58` − `+0x74`)

`FUN_0052fff0` recomputes the side. It walks every object of families
`0x10..0x3f` (`FUN_004f6010`) that exists (`+0x50` bit `0x40`) and shares the
side's bits (`+0x24 & 0xc0`):

- `+0x70` = `+0x74` = Σ `FUN_004f2990` = class `+0x4c`, the DAT
  `maintenance_cost` (`FUN_0053b870`);
- `+0x60` pair = Σ the facility `+0x64` pair over completed (`+0x50` bit 0),
  enabled (`+0x60` bit 0 clear) mines; `+0x68` pair over refineries.

A mine's and a refinery's `+0x64` pair is (capacity, allocated). The
initialisers at `0x52d540` (mine) and `0x52d100` (refinery) set it to
`(0x32, 0)`: **50 capacity each**. `FUN_005323c0`/`FUN_00532520` set side
`+0x58` (`FUN_0052ec90`) to `min(mines.capacity, refineries.capacity)` and
`min` of the allocated halves.

So the right counter is

```
min(50 × mines, 50 × refineries) − Σ maintenance_cost of the side's existing objects
```

which matches the manual: each mine and refinery pair adds 50 units, and
units use them up. `FUN_00530350` (event `0x382`) locks a random object when
`+0x58` falls below `+0x70` or `+0x74`. `FUN_0052f6b0`/`FUN_0052f8f0` spread
the load over the facilities' allocated halves (`FUN_0055a820`,
`FUN_0055a960`).

### Load allocation (`FUN_0052f6b0`, `FUN_0052f8f0`)

`FUN_0052fff0` ends by spreading the load: `FUN_0052f6b0` over the mines,
`FUN_0052f8f0` over the refineries, the same algorithm on each. Side
`+0x60` pair is (capacity C, allocated A) summed over the eligible
facilities; `+0x70`/`+0x74` is the load L.

- `delta = min(L − A, C − A)`. Zero does nothing: a new facility gets no
  share until the load or the capacity changes.
- The walk visits the side's completed (`+0x50` bit 0), enabled (`+0x60`
  bit 0 clear) facilities of the family in object order (`FUN_00506630`,
  `FUN_00506960`, then `FUN_004f6010`), and repeats the walk while any of them
  changed and has room left.
- Adding (`FUN_0055a820`): a facility with pair (c, a) takes
  `clamp(A × c / C − a + 1, 0, min(delta, c − a))`, and A grows by it. With
  every facility at c = 50 this hands out one unit per facility per walk:
  the load is dealt round-robin, so shares differ by at most one, the
  earlier facilities in walk order holding the extra unit.
- Removing (`FUN_0055a960`): a facility gives up
  `clamp(a − (A − 1) × c / C, 0, delta)`, so units leave the fullest first.
- `FUN_0055a6e0` writes the facility pair and calls slot `+0x204`; a
  facility that goes offline drops its share (`FUN_0055ab60`: pair second
  half set to 0), and A, rebuilt from the facilities each time, shrinks with
  it.
- `FUN_0052fff0`'s callers are object add, remove, completion and side change
  handlers (`FUN_005140c0`, `FUN_00514510`, `FUN_00514890`, `FUN_00515cd0`,
  `FUN_00515f40`): the load is respread on change, not on a clock.

### Waiting for input (`FUN_0052fbb0`, `FUN_0052fbd0`)

- A refinery in state 1 with no raw left (`FUN_005306b0`) appends a 0x18-byte
  request naming itself to the side's raw queue (`+0x88`, count `+0x80`,
  `FUN_00533570`); a yard does the same for refined (`FUN_00530820` →
  `FUN_00533610`, count `+0x84`).
- `FUN_0052fbf0` (raw) and `FUN_0052fd30` (refined) pop the head
  (`FUN_005f5e90`: first in, first out). If that facility is still in state
  1 and the request is current (`FUN_0053a6e0`), they take one unit and set
  its bit 1; a stale request is dropped. They report whether another waiter
  and unit remain, so their events (`FUN_00577a20`, `FUN_00577d40`) repeat.
- `FUN_0052fb30` spends no raw when `DAT_006b90e0` is clear (before play);
  `FUN_0052fb80` has no such guard.

### Diverted output (`FUN_005166a0`)

- `FUN_005185c0` returns the system holding the facility (a parent key in
  `0x90..0x97`); `FUN_005166a0` reads its `+0x6c` and, when it is not 0,
  rolls `FUN_0053e2f0(|v|)`: `rand(0..=99) < |v|` (`FUN_0053e2e0`,
  `FUN_0053e290`).
- On success the finished mine's raw or refinery's refined goes to the other
  side (`FUN_00516360`: `local_14`, `FUN_00506f30(2 − (side != 1))`).
- System `+0x6c` is written by `FUN_00509ec0` (range ±100) from
  `FUN_0050b230` → `FUN_00559c40(side, support, bit 11, …)`: the support
  drift the port already computes in `economy::calculate_support_drift`,
  negative for side 2. `economy-systems.md` named it `production_modifier`.
  It is 0 above the drift threshold (40) or while a friendly fleet is
  present, so diversion happens only in restless systems.

### Yards and the build (`FUN_00530950`, `FUN_00529dd0`)

- A yard at the end of its cycle (state 3) clears bit 1 and, when its system
  has a manager for the yard's family (`FUN_0052eb60` → `FUN_00509670`),
  adds one to that manager's progress (`FUN_0052a430`: `+0x5c` while below
  the cost `+0x68`, else the next unit's credit `+0x60`).
- The yard is not always cycling. `FUN_00529dd0` keeps the number of active
  yards (states 1..3) equal to the work left,
  `+0x6c − +0x60 − +0x5c`:
  - too few: start idle yards (state 4 → 1, `FUN_0053aac0`), the one with
    the lowest `processing_rate` first, one at a time;
  - too many: stop yards (→ 4, `FUN_0053ab10`), waiting ones (state 1)
    before working (2) before done (3), the slowest first;
  - no queued work: `FUN_0052a2c0`.
- So each yard cycle draws one refined unit and adds one unit of work, and
  an idle yard draws nothing.

### Overdrawn maintenance scraps units (`FUN_00530350`)

- `FUN_0052f670` sets side `+0xa8` to 1 when the load (`+0x70` or `+0x74`)
  exceeds the capacity `+0x58`, else 0 (`FUN_0052f3d0`). Its change hook,
  side vtable (`0x00660af8`) slot `+0x1f8` (`0x5329f0`, a function Ghidra had
  not split out), enables or disables the side's timer `0x382` with that
  value (`FUN_0053fa60(0x382, new, side, side + 0xcc)`).
- The timer's record `+0xcc` takes its period from GNPRTB 7168, 10 days
  (`FUN_00532350`, `timer-scheduler.md`).
- Each firing (`FUN_00578060` → `FUN_00530350`), while still overdrawn,
  counts the side's objects (families `0x10..0x3f`, `FUN_0052e8b0`) that
  exist (`+0x50` bit 6), share the side, have `+0x50` bit 2 set and a
  non-zero class `maintenance_cost` (`FUN_0052e530`), draws
  `r = rand(0..=count − 1)` (`FUN_0053e290`), and sets Locked (`+0x50`
  bit 13, `FUN_004f7950`) on the r-th of them in walk order (the counter is
  the stack slot `[ESP + 0x18]`, `0x530418`; Ghidra's `local_4`).
- Setting Locked calls slot `+0x140` (`FUN_004fbaa0` on the capital ship
  vtable `0x0065d650`) → slot `+0xe0` (`FUN_004f8850`): a completed
  (`+0x50` bit 0) Locked object calls slot `+0xac` (`FUN_004f8660` →
  `FUN_004f71d0`) with 0x15, which writes 0x15 (Autoscrap) into its
  destruction-reason byte `+0x40`. The unit is scrapped.
- Mines and refineries (maintenance cost 0) are never picked. One unit goes
  per 10 days until the load fits; scrapping returns its maintenance, which
  ends the overdraft sooner.
- Manual p. 30 (tutorial): "If your maintenance capacity falls below zero,
  you will find your facilities, troops, or ships will begin to be
  scrapped." Confirmed.
- Correction: `FUN_00530270` reads `+0x40 & 0xff`, the destruction reason,
  not the family. The half-cost refund applies to any object destroyed with
  reason 0x14..0x16 whose `+0x50` bit 2 is set, so an autoscrapped unit
  refunds half its refined cost too.

### The player's Scrap order (`0x200`)

- Offered by a fleet (`FUN_004ff8e0`), a regiment (`FUN_00504b30`), a
  capital ship (`FUN_00557ce0`) and the sector window's facility, defense
  and fleet icons (`FUN_00512700`: facilities `FUN_0053b6e0`; regiments,
  squadrons and defenses; fleets `FUN_004ffe70`).
- Order vtable `0x006619b0` (constructor `FUN_0053dcd0`, factory
  `FUN_0053dd20`, registered by `FUN_0051f4b0`): kind `FUN_0048b450`,
  enabled `FUN_0051fe20`, validator `FUN_0051ff30`, execute `FUN_00520040`,
  command factory `FUN_0053dda0` → `FUN_00579570` (vtable `0x006695a8`,
  whose execute `FUN_00579860` calls the object's `+0x98`).
- Enabled (the object's `+0x5c`; `FUN_004f9860`, a fleet's `FUN_004ffa70`):
  the order's side, completed (`+0x50` bit 2), not destroyed, not en route
  (bit 4), not on a mission (`+0x78` bit 7); a fleet also needs every ship
  completed (status `1`/`0x10`) and none en route (`1`/`0x11`).
- `FUN_00487cc0` always confirms kind `0x200`. `FUN_0049a350` case `0x200`:
  text TEXTSTRA RCDATA `0x7050` "Are you sure you want to scrap the
  following units?", then for each object of the team `FUN_0049a880`
  appends `0x7054` ("\n" and the name `+0x30`); picture `+0x2e` 1032 for
  side 1, else 1033. The window is the move confirmation's
  (`FUN_0044f060`): checkmark `0x14`, X `0x15`, Enter and Escape.
- Execution is immediate: `+0x98` (`FUN_004f84e0` on a fleet) → `+0xa0`
  (`FUN_00534c00`) → `+0xac(0x14)` → `FUN_004f71d0` writes reason `0x14`
  and sets destroyed (`+0x100`, `FUN_004fb200` → `FUN_004f7560`). Event
  `0x302` follows; only reason `0x15` adds `0x304` (DestroyedAutoscrap).
- The destroy handler `FUN_005140c0` recomputes maintenance
  (`FUN_0052fff0`) and refunds (`FUN_00530270`): any object with reason
  `0x14..0x16` and `+0x50` bit 2 returns half its class's refined cost.
- Manual p. 86: "This returns to you the maintenance and some of the refined
  material the unit used." "You can scrap any facility, troop, or ship in
  this way."
- Open: what scrapping a fleet does to the ships and squadrons it carries
  (the fleet's own destroy path); reason `0x16`.

### Starting stockpiles

The only writes to side `+0x78` in the side class's code
(`0x52c000..0x536000`) are `FUN_0052ef90` and the stream readers
`FUN_00531a70`, `FUN_00533300` and `FUN_005336b0`. Nothing seeds the
stockpiles, so a new game starts at 0 raw and 0 refined (hyp: the
constructor zeroes them; not read).

## Corrections

- `FUN_00433620`/`FUN_00433780`/`FUN_00484630` do not feed the counters.
  `FUN_00418500` runs them to refresh a cached fleet record for the galaxy
  view: `FUN_004f2e20` resolves capital ships (`0x14..0x1b`), and the loops
  walk fighters (`FUN_005039d0`, `0x1c..0x20`) and regiments
  (`FUN_00504c40`, `0x10..0x14`).
- `ai-behavior-analysis.md` and `COMBAT-SUMMARY.md` call `0x14..0x1b` troops
  and `0x1c..0x20` facilities. The iterators above and `decoy-roll.md` show
  capital ships and fighters.

## Port

- `rebellion-core/src/stockpiles.rs`: each side's raw and refined
  (`SideStock`), every facility's cycle (`FacilityCycle`: stage, ready
  tick, first-cycle flag, load share, request number), the load spread
  (`spread_load`), the yard assignment, the diversion roll and the
  overdraft timer. It runs each tick between the economy and
  manufacturing, in `rebellion_data::simulation` and the app's own loop.
- `rebellion-core/src/manufacturing.rs`: a unit's progress is units of work
  (`work_done` of `total_cost`, its refined cost) with the manager's credit;
  only yard cycles add it. Build Selection's and the Status window's days
  stay the best case over the yards' periods.
- `rebellion-core/src/scrap.rs`: the Scrap order's team, gate, names and
  the scrap itself with its half-cost refund; the overdraft's candidates.
- `rebellion-core/src/economy.rs`: `SystemEconomy::support_drift`, the
  system's `+0x6c`.
- The app draws all three monitors and opens the Scrap confirmation from
  the object menus.
- port: a unit's daily order is finish, assign, start; the original runs
  them on events within the day.
- hyp: a new game starts with empty stockpiles; the AI and agent order only
  what their maintenance covers (manual p. 30); a scrapped fleet scraps its
  ships and squadrons, and a fleet left with no unit is removed, its
  characters staying in the system.

## Open

- The manager's `+0x6c` (the queue's total work) and its writer; what
  `FUN_0052a2c0` does with no yards.
- Whether the original respreads load when a facility is added with no load
  change (the trace says no; untested in play).
- Who calls `FUN_0055aa80`, which starts a completed mine or refinery.
