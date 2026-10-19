# Build delivery: en-route manufactured objects

Recovered 2026-09-27 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Finding F-030 in the full-functionality audit. The `+0x50` state
bits are in `object-state-flags.md`.

## State bits used

The fleet view vtable `0x0065d438` slots `+0x178 + 4 * bit` name the rest of
the `+0x50` bits (setters `FUN_004f7720`..`FUN_004f7a30`, `FUN_004f7aa0`):

| Bit | Mask | Name |
|-----|------|------|
| 7 | `0x80` | `GameObjObservedByAlliance` |
| 8 | `0x100` | `GameObjObservedByEmpire` |
| 9 | `0x200` | `GameObjDamagedNotif` |
| 10 | `0x400` | `GameObjHyperdriveActiveNotif` |
| 11 | `0x800` | `GameObjAutoroutingNotif` (`FUN_004f7870`) |
| 12 | `0x1000` | `GameObjAutoscrapRequestNotif` (`FUN_004f78e0`) |
| 13 | `0x2000` | `GameObjLockedNotif` |
| 14 | `0x4000` | `GameObjReadyForDeleteNotif` |
| 15 | `0x8000` | `GameObjConstructedNotif` |
| 16 | `0x10000` | `GameObjDeployedNotif` (`FUN_004f7aa0`, slot `+0x1b8` `FUN_004fc570`) |

En route (bit 4) is recomputed by `FUN_004f8240`:

```c
enroute = FUN_005406d0(obj)            // any object but the galaxy (type 0xf1)
       && deployed(bit 16) && !destroyed(bit 3)
       && (enroute_active(bit 5) || container_of(obj).enroute(bit 4));
```

So an object is en route while it travels itself, or while its container
(`FUN_004f6b50`) is en route, such as a ship in a travelling fleet.

## Build order and product

- The manufacturing manager (the facility-side class with the tick at vtable
  slot `+0x208`, `FUN_0052b960`; vtables near `0x0065fbf8` and `0x006629a0`,
  constructors `FUN_005279d0` and `FUN_0055b2f0`) keeps a queue of product
  objects in the list at `+0x54 -> +0x18`. The queued units are real game
  objects from the moment they are ordered.
  A product enters the list on its own event `0x300`
  (`FUN_005140c0.c:66-77` → `FUN_00529cf0` → `FUN_0052a710`), which removes
  it again once `FUN_00528890` stops accepting it.
- `FUN_0052aac0` chooses the current product: the first queued object that
  `FUN_00528890` accepts. `FUN_00529770` stores its key at manager `+0x74`
  (ProductKey: vtable slot `+0x24c`, notifier `FUN_0052bcb0`
  `ManuMgrProductKeyNotif`, event `0x320`). The DeploymentKey is `+0x78`
  (slot `+0x250`, `FUN_0052bd00` `ManuMgrDeploymentKeyNotif`, event `0x321`).
- Objects carry `DestinationLocationAtDeparture` (key at `+0x3c`, setter
  `FUN_004f7120`, notifier `FUN_004fbe50`) and `DestinationCount` (arrival tick
  at `+0x44`, setter `FUN_004f7390`, notifier `FUN_004fbf10`). The
  `capshp.cpp` validators `FUN_004f7260`/`FUN_004f72f0` guard
  `DeploymentCount` and `DestinationCount`.

## Completion (`FUN_0052b960`)

```c
if (current_product(mgr + 0x1c) && mgr[+0x5c] != 0 && mgr[+0x5c] >= mgr[+0x68]) {
    product = lookup(mgr + 0x74);                 // FUN_0052be30 -> FUN_0053efa0
    if (product) FUN_0052bee0(mgr, product, ctx);
}

FUN_0052bee0(mgr, product, ctx):
    a = mgr->slot_0c();                            // the manager's location key
    b = product->slot_0c();                        // the product's location key
    set_deployed(product, 1);                      // bit 16, FUN_004f7aa0
    if (a != b && a && b) set_enroute_active(product, 1);   // bit 5, FUN_004f7640
    set_completed(product, 1);                     // bit 2, FUN_004f74f0
```

Progress `+0x5c` reaching the cost `+0x68` completes the product. A product
whose location differs from the manufacturing facility's starts travelling.

## Travel

Any change of container goes through `FUN_00514a60(obj, from, to, ctx)`, which
calls `FUN_00556430` for a mobile object (slot `+0x38`):

```c
FUN_00556430(obj, from, to, ctx):
    if (!obj->slot_38() || autorouting(bit 11)) return;
    if (system_of(from) != system_of(to)) {        // FUN_00555540 on both
        FUN_00555d30(obj, from, to, &ticks);
        set_enroute_active(obj, 1);                // bit 5
        schedule(0x387, ticks, obj);               // FUN_004f8010 -> FUN_0053fab0
        obj[+0x3c] = key(to);                      // FUN_004f7120
        set_autorouting(obj, 1);                   // bit 11, FUN_004f7870
        obj[+0x44] = clock() + ticks;              // FUN_004fd340 (callback DAT_006b2b08), FUN_004f7390
    }

FUN_00555d30(obj, from, to, &ticks) -> FUN_00555b30(obj->slot_34(), 1, side, from, to, &ticks)
FUN_00555b30: a = container(from), b = container(to)                // FUN_00555540
              ticks = 0 if either is missing or a == b;
              ticks = 1 otherwise, written before the type test;
              ticks = FUN_0055d8c0(coords(a), coords(b), speed)          // FUN_00509620
                  when both are systems (types 0x90..0x97)
FUN_0055d8c0(a, b, speed):
    d = isqrt((b.x - a.x)^2 + (b.y - a.y)^2);      // FUN_0055d860, FUN_0053e1d0
    if (d == 0) return 0;
    t = (d / GNPRTB[5120]) * speed / 100;          // DAT_006bb6e8, FUN_0053e190
    return t == 0 ? 1 : t;
```

GNPRTB 5120 (`0x1400`, loaded by `FUN_0055cb60`) is 5 in every column of the
shipped GNPRTB.DAT. The travelling object is represented by itself: bits 5 and
11, the destination key at `+0x3c`, the arrival tick at `+0x44`, and a pending
galaxy event `0x387`. There is no hidden fleet.

`FUN_00556430`, `FUN_00555d30`, `FUN_00555b30`, `FUN_0055d8c0` and
`FUN_00509620` are this transit-time chain: the shorts are system
coordinates (SYSTEMSD `x_position`/`y_position`, DAT `0x24`/`0x26`, read
through record `+0x2c` then `+0x4c`) and the result is travel ticks.
`FUN_0055d8c0`'s only callers are `FUN_00555b30` and `FUN_0055d8a0`, so
bombardment never reaches it.

### Other users of the distance formula

- `FUN_00516730`, the battle's retreat options: once `FUN_005545e0` has
  chosen the retreat system, its ticks come from `FUN_0055d8a0`, which is
  `FUN_0055d8c0` at the default speed.
- `FUN_00555f90` -> `FUN_00555d90`: an estimate from a class record
  (`FUN_0051cab0`), using class `+0x6c` for families `0x14`-`0x1f` and the
  default otherwise.
- `FUN_004f4850` -> `FUN_005425e0`: a mission's travel, at the speed from
  `FUN_00542990` (Han), set on every member at `+0x1c`.
- `FUN_005562e0` -> `FUN_0058b060`: a nearness table at manager `+0x68`
  (`0x27424` bytes) holding, for each system, all 200 systems sorted by raw
  distance (`FUN_0055d860`), at `(id - 100) * 0x324`.

## Arrival

- Event `0x387` is registered by `FUN_0051ef80` (`FUN_0054f140(0x387,
  FUN_0057be20)`); its class (`FUN_0057bd30`, vtable `0x00669838`) validates
  the target key at `+0x3c` in `FUN_00586150`.
- `FUN_004fb520`, an object vtable slot shared by many classes, clears en
  route active (bit 5), autorouting (bit 11) and autoscrap request (bit 12),
  then recomputes en route (`FUN_004f8240`) and the derived bits.
- `FUN_004fba10` cancels an object's pending `0x387` (`FUN_0053fa60`).
- `FUN_004fc080` raises `GameObjDestroyedOnArrivalNotif` (event `0x303`) for a
  destroyed object that was en route active (bit 5) with a non-zero
  `+0x40 & 0xff0000`.

## Recovered 2026-09-28

All lines cite `ghidra/notes/<function>.c`. "Inference" marks a reading the
code does not state outright.

### Transit arithmetic

`FUN_0055d8c0.c:11` computes `FUN_0053e190(d / DAT_006bb6e8, speed)`, and
`FUN_0053e190(x, s)` is `FUN_0053e170(x, s, DAT_00661a88)`
(`FUN_0053e190.c:5`), which is `(s * x) / 100` (`FUN_0053e170.c:5`,
`FUN_0053e150.c:5`; `DAT_00661a88` is 100, `blockade-troop-withdrawal.md`).
With integer division at each step:

```c
d = isqrt(dx*dx + dy*dy);                  // 0 -> 0 ticks (same position)
t = ((d / GNPRTB[5120]) * speed) / 100;    // GNPRTB 5120 = 5 in every column
return t == 0 ? 1 : t;
```

No difficulty or side factor enters: `DAT_006bb6e8` is GNPRTB `0x1400`
(`FUN_0055cb60.c:5`) and is 5 in all eight columns of the shipped GNPRTB.DAT.
`speed` is a time percentage: 100 is the default and a larger value is slower.

`FUN_00555d30` calls the mobile slot `+0x38` (`FUN_00555d30.c:12`) and then
the speed slot `+0x34` with argument 1 (`FUN_00555d30.c:16`, disassembly
`0x00555d49`/`0x00555d6f`).

### Speed (`+0x34`) and mobile (`+0x38`) per class

Vtables were found through the shared side-key slot `+0xc` (`FUN_004f6a80`,
46 object vtables) and read with `DumpSlots`.

| Class (type) | Vtable | `+0x34` speed | `+0x38` mobile |
|---|---|---|---|
| Fleet (`0x08`) | `0x0065d438` | `FUN_004fd900`: the largest speed among member capital ships (`FUN_00502db0`, types `0x14..0x1c`) that are completed and not in transit; any such member returning 0 makes the result 0 (`FUN_004fd900.c:33-45`) | `FUN_004fda10`: at least one member ship completed and not in transit (`FUN_004fda10.c:23`) |
| Capital ship (`0x14`, `0x18`) | `0x0065d650`, `0x0065d930` | `FUN_00500820`: class `+0x6c` minus `FUN_005011f0`; if 0, class `+0x70` minus `FUN_00501230`; if still 0 and `+0x40` byte 1 is 0, the default (`FUN_00500820.c:10-19`) | `FUN_004f6410`: Deployed, bit 16 (`FUN_004f6410.c:5`) |
| Fighter squadron (`0x1c`) | `0x0065def0` | `FUN_00502f80`: the same, with both subtrahends `FUN_006158b0` = 0, so class `+0x6c`, else `+0x70`, else the default (`FUN_00502f80.c:19`) | Deployed |
| Regiment (`0x10`) | `0x0065e438` | `FUN_004f63f0`: the default (`FUN_004f63f0.c:9`) | Deployed |
| Special forces (`0x3c`) | `0x0065e160` | `FUN_005338a0` -> `FUN_004f63f0(1)`: the default | Deployed |
| Facilities (`0x20..0x2d`, `0x80`, `0xa0..0xa4`) | e.g. `0x00660280`, `0x006629a0` | `FUN_004f63f0`: the default | Deployed |
| Characters (`0x30..0x35`, `0x38`) | `0x006657f0`..`0x006681d0` | `FUN_004ed370`: `+0x9a`, MissionHyperdriveModifier (`FUN_004ed370.c:9`) | Deployed |

- The default is `DAT_006b9050`, loaded by `FUN_0053e0b0` as GNPRTB 1
  (`FUN_0053e0b0.c:7`, `FUN_0053e390` is the GNPRTB lookup); GNPRTB 1 is 100
  in every column. Every speed slot returns 0 for an object that is not
  existing (`+0x50` bit 6).
- Class `+0x6c`/`+0x70` are CAPSHPSD/FIGHTSD `hyperdrive` and
  `hyperdrive_if_damaged`: the class record getters read `+0x6c`
  (`FUN_00557470`) and `+0x70` (`FUN_00557480`), and the in-memory record is
  the DAT record shifted by `0x28` (max hull at `+0xc0`, `FUN_00500550`, is
  DAT `hull` at `0x98`; `hyperdrive` is DAT `0x44`). Shipped values: capital
  ships 50..100 (damaged 0..180); fighters 60 or 0.
- `FUN_005011f0` is `class+0x6c * nibble` where the nibble is ship `+0x64`
  bits 16..19 (`FUN_005011f0.c:16-20`), set only to 0 or 1 by `FUN_00501780`
  (validated against `FUN_00500680`). Inference: that bit marks the hyperdrive
  as damaged, so a damaged ship uses `hyperdrive_if_damaged`, and one with
  both disabled falls back to the default only while `+0x40` byte 1 (the
  `DeploymentCount` byte, `FUN_004f7260`) is 0.
- Characters start at the default (`FUN_004eceb0.c:30`). On a mission state
  change (`FUN_00546ea0` -> `FUN_00548370`), every member of the mission gets
  `FUN_004ee470(speed)` (`FUN_00548370.c:65`), where `FUN_00542990` gives GNPRTB
  3083 (50) when Han Solo (`0x33000243`, `FUN_00506f50.c:13`; TEXTSTRA 10307)
  qualifies and the default otherwise (`FUN_00542990.c:15`,
  `FUN_0055e5b0.c:80`). The qualifying test (Han's `+0x68` matching the
  mission key and `+0xac` bit 0 clear, and an empty mission list from
  `FUN_00525a00`) is read but its fields are unnamed.
- A fleet moves as one object: its speed is its slowest member ship's. Fleet
  members are inside the fleet container, so they are en route through the
  container rule (`FUN_004f8240`) rather than their own timers. The fleet's
  own `+0x11c` arrival slot `FUN_004ff600` runs the base `FUN_004fb520` and
  then clears three fleet fields (`FUN_004fe380`, `FUN_004fe4d0`,
  `FUN_004fe460`, not read).

### When transit starts

- The only caller of `FUN_00514a60` is `FUN_0057ba50.c:16`, slot 8 of
  vtable `0x006697d8`, the fire slot of event `0x320` (transit start; factory
  `FUN_0057b8d0`, `FUN_0051ef80.c:26`). `FUN_004faf50` and `FUN_0052bcb0`
  raise it through `FUN_0053f9f0`/`FUN_0053fe40(0x320, ...)`, which store the
  two keys at `+0x4c`/`+0x50`. hyp: travel starts after the container change,
  with the object already in its new container; the arrival handler does not
  move it again (below).
- `FUN_004fb4b0` (object vtable `+0x118`) sets Deployed (`FUN_004fb4b0.c:10`)
  and raises event `0x308` (`FUN_004fb4b0.c:18`), whose handler
  `FUN_005150c0` (via `FUN_0057a9e0`) does, for types `0x10..0x3f`:
  - when the object's key at `+0x38` names a manager (`FUN_00505750`), it
    calls `FUN_00529d70` on it (`FUN_005150c0.c:47`), which sets that
    manager's progress to `min(cost +0x68, +0x60)` and subtracts it from
    `+0x60` (`FUN_00529d70.c:11`);
  - for a non-character, `FUN_00556430(obj, +0x38, own location)`
    (`FUN_005150c0.c:93`): the product travels from the key at `+0x38` to the
    container it already sits in. Inference: `+0x38` holds the producing
    facility's key, so the travel clock starts at deployment, not at the
    order.
- Who calls slots `+0x118` and `+0x148` is not traced.

### Build progress

- `FUN_0052b960.c:8` completes the product when `0 < +0x5c` and
  `+0x68 <= +0x5c`.
- `FUN_0052a430` adds one to progress `+0x5c` while it is below the cost
  `+0x68`, otherwise one to `+0x60` (`FUN_0052a430.c:8`).
- Its only caller is `FUN_00530950.c:18` (facility `+0x60` bit 2 set, then
  `FUN_0053a860`), called from `FUN_00516360.c:142` in mode 3, whose recurring
  caller is `FUN_00578a40.c:16`, the fire slot of event `0x315` (class
  `0x00669528`, factory `FUN_005788f0`, `FUN_0051ef80.c:25`). `FUN_0053b1d0`
  raises `0x315` on a facility state change (`FUN_0053b1d0.c:37`).
- Facility timer `0x394` (`FUN_0053b330`, fire `FUN_0053aa20`) sets that
  facility `+0x60` bit 2 when `+0x58` is 2 (`timer-scheduler.md`). Inference:
  each facility cycle adds one unit of progress, so the per-day rate is one
  unit per `0x394` period, read from the record at facility `+0x54` field
  `+0x20`. That record's source (MANFACSD `processing_rate` is a candidate)
  is not traced.
- `FUN_00528890` accepts a queued product that is created (bit 1) and not
  completed, destroyed or en route (bits 2..4, `FUN_00528890.c:19`), and whose
  key from `FUN_0048a640` equals the manager's own key. Inference: the product
  belongs to this manager. No side test is made.

### Arrival

- Event `0x387`'s class `0x00669838` fires `FUN_0057bf80`: after validating the
  target at `+0x3c`, it calls `FUN_004f7fc0` (`FUN_0057bf80.c:35`), which, for
  an autorouting object, clears Autorouting (bit 11) and the arrival tick
  `+0x44` (`FUN_004f7fc0.c:9`).
- Clearing bit 11 calls slot `+0x138`, `FUN_004fba10`, which cancels the
  pending `0x387` and raises event `0x306` (`FUN_004fba10.c:9`). Its handler
  `FUN_0057ac90` -> `FUN_00515a50`:
  - a character (`0x30..0x3f`) goes to `FUN_00545820` (`FUN_00515a50.c:39`),
    which resolves its mission or group and calls `FUN_00522280`;
  - any other object goes to `FUN_00556620` (`FUN_00515a50.c:51`), which
    compares its location (slot `+0xc`) with the destination `+0x3c` and, if
    they differ, starts the next leg with `FUN_00556430`
    (`FUN_00556620.c:31`); when no leg remains, `FUN_004f7640(obj, 0)` clears
    en route active (bit 5).
- Clearing bit 5 runs slot `+0x124`, `FUN_004fb760`, which zeroes the
  `DestinationCount` byte (`+0x40` byte 2, `FUN_004fb760.c:9`,
  `FUN_004f72f0`) and recomputes en route and usable.
- No container change and no battle or blockade test is in this path. The
  object joins its container when the move is made (event `0x31a`), and the
  arrival only ends its en-route state. Where battles are triggered on arrival
  is not traced.

## Port

`crates/rebellion-core/src/movement.rs`:

- `transit_ticks_between` divides by GNPRTB 1 (`GNPRTB_DEFAULT_SPEED`) where
  the original divides by the constant 100 (`DAT_00661a88`). They agree on
  the shipped data and diverge only if GNPRTB 1 changes.
- The port computes only system-to-system moves. It has no "same container
  gives 0, a different container that is not a system gives 1" rule.
- With both drives disabled the port returns 0 only when the class has a
  drive; the original falls back to the default while `+0x40` byte 1 is 0
  (tagged `port:`).
- No port code uses the retreat time (`FUN_00516730`), the class estimate
  (`FUN_00555f90`) or the nearness table (`FUN_0058b060`). Han's mission
  speed is ported (`missions.rs`).

## Open

- Whether `FUN_00555540` returns the system for an object inside a fleet,
  which decides whether a move between two fleets in one system costs 0.
- Who reads the nearness table at manager `+0x68`, and who uses
  `FUN_00555f90`'s estimate.
- The callers of object slots `+0x118` (deploy, event `0x308`) and
  `+0x148` (container change, event `0x31a`). The order path that creates a
  queued product in its container is in `death-star-product.md` section 1.
- The `0x394` delay record at facility `+0x54` `+0x20`, which sets the build
  rate.
- The meaning of ship `+0x64` bits 16..23 (hyperdrive damage, inference) and
  of `FUN_0048a640`'s key in `FUN_00528890`.
- Where a fleet's arrival triggers battle or blockade checks.
- The `+0x40 & 0xff0000` field in the destroyed-on-arrival test is
  `DestinationCount` (`FUN_004f72f0`); the test's other operand is unnamed.
- `FUN_004fd340` returns the value of the callback at `DAT_006b2b08`,
  read here as the game clock because it is added to a tick delay; the
  callback itself is not traced.
- The player's destination choice and the en-route display were not looked
  up.
