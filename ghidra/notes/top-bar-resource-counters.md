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

The port keeps no side stockpiles and runs no facility cycles
(`manufacturing-build-selection.md`: "refined material is not drawn down"),
so `game_speed::draw_resource_counters` stays unwired. The maintenance
counter needs only the formula above and class `maintenance_cost`. The two
stockpiles need the facility cycle above.

## Open

- How `FUN_0052f6b0`/`FUN_0052f8f0` choose which facilities carry the load
  (walk order, `FUN_0055a820`'s proportional share). This sets `allocated`
  and so the cycle lengths.
- The enemy-diversion roll's source record (`FUN_005185c0`, `record[0x1b]`).
- The starting stockpiles (side `+0x78`, `+0x7c` at seeding).
