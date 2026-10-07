# Production Destination

Recovered 2026-10-06 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis), with STRATEGY.DLL's bitmaps, TEXTSTRA and the manual. Every
function named here has a `FUN_<address>.c` note in this directory. See also
`build-delivery.md` (transit mechanics) and `sector-icon-menus.md` (the
facility-icon menu).

The manual (pp. 44-46, 84, 128; Figs. 2.41, 2.42, 3.58, 3.75-3.76) gives
the player's view:

> DESTINATION: Set the destination for the trooper regiment being trained.
> (p. 128)

Fig. 3.75 (Manufacturing and Production window) shows the context menu with
"Destination" as the first item, and "Reserved" as the last. Fig. 2.42
(Build Selection window) shows "Best Time to Completion: Days: 8" and
"Best Time to Deployment: Days: 88" as separate fields.

## The order kind `0x214` (Destination)

From `sector-icon-menus.md`, the facility icon's right-click menu lists
Destination (`0x214`, TEXTSTRA 12290, sort 120) and Reserved (`0x216`,
TEXTSTRA 12294, sort 1002) among its orders.

The order resolves its team through `FUN_00512700(system, order, kind=4,
out)`. For kind 4 with Destination (`0x214`) or Reserved (`0x216`), the
team is the system's objects of family `0xa0..0xaf` (`FUN_0052c170`).

## The `0xa0..0xaf` family

`FUN_0052c170` is a constructor that initializes an iterator over family
`0xa0..0xaf`:

```c
FUN_0052c170(this, param_1, param_2):
    local_14 = 0xa0;
    local_10 = 0xb0;
    FUN_00513050(this, param_1, {0xa0, 0xb0}, param_2);
    vtable = PTR_FUN_0065fe54;
```

This constructs a range iterator for objects whose family byte is
`0xa0..0xaf`. These objects appear in a system's child list alongside
facilities, fleets, and regiments.

From `build-delivery.md`, the manufacturing manager (the facility-side class
at vtable `0x0065fbf8` and `0x006629a0`, constructors `FUN_005279d0` and
`FUN_0055b2f0`) keeps a queue at `+0x54 -> +0x18`. The manager has a
per-system production-area identity. hyp: the `0xa0..0xaf` objects are
**manufacturing managers** -- per-production-area manager objects that exist
one per facility type per system (e.g., one for the shipyard, one for the
training yard, one for the construction yard). Each manager tracks its
queue, current product, progress, and deployment destination.

Evidence:
- `FUN_00512700` returns them for kind 4 (facility icon), not for kind 8
  (defense) or 0x10 (fleet).
- The Destination order and Reserved order both act on these objects.
- The build-delivery note's manager keeps `+0x74` (DeploymentKey), `+0x5c`
  (progress), `+0x68` (cost).
- Family `0xa0..0xa4` appears in the vtable list alongside facility families
  (`0x20..0x2d`, `0x80`).

## How the player picks a destination

### From the sector window's facility icon menu

1. Right-click the facility icon (kind 4) in the sector window.
2. Select "Destination" (`0x214`).
3. hyp: the cursor changes to targeting crosshairs (the same targeting mode
   as Mission and Build orders, `FUN_00422ce0`'s `+0xc0` mode). The player
   clicks a system on the galaxy map or a system's Defense/Fleet window.
4. The target system becomes the destination for all products currently
   queued at that production area.

### From the Build Selection window

The Build Selection window (Fig. 2.42) shows "Best Time to Completion" and
"Best Time to Deployment" as separate lines. "Best Time to Deployment"
includes transit time from the manufacturing system to the destination.

## Where the destination is stored

From `build-delivery.md`:
- Each manufactured product carries `DestinationLocationAtDeparture` (key at
  `+0x3c`, setter `FUN_004f7120`, notifier `FUN_004fbe50`).
- `DestinationCount` (arrival tick at `+0x44`, setter `FUN_004f7390`,
  notifier `FUN_004fbf10`).
- The manager itself has a slot `+0xc` (`slot_0c()`) that returns its
  location key, and the product has the same slot.

When a product completes (`FUN_0052bee0`):
```c
a = mgr->slot_0c();    // the manager's location key
b = product->slot_0c();  // the product's destination key
set_deployed(product, 1);
if (a != b && a && b) set_enroute_active(product, 1);  // starts transit
set_completed(product, 1);
```

hyp: the Destination order sets the manager's destination (or each queued
product's `+0x3c` key) to the target system's key. New products queued after
the Destination is set inherit the same destination. The product's
`slot_0c()` returns this destination, and at completion the game compares it
to the manager's location to decide whether transit is needed.

## How new queued products inherit the destination

hyp: when the player queues a new build order, the product object is created
with its `+0x3c` (DestinationLocationAtDeparture) set to the current
destination of the production area. This is either the system where the
facility is located (default, no transit needed) or the target system the
player selected with the Destination order.

The manager's `+0x74` (DeploymentKey) may serve as the persistent per-area
destination that new products copy.

## "Best Time to Completion" and "Best Time to Deployment"

From the Build Selection window (Fig. 2.42):
- **Best Time to Completion**: the time until the product is finished
  building. This is `cost(+0x68) - progress(+0x5c)` ticks, converted to
  days by the facility's processing rate (the `0x394` timer period at
  facility `+0x54 + 0x20`, per `build-delivery.md`).
- **Best Time to Deployment**: completion time plus transit time from the
  facility's system to the destination system. Transit time uses
  `FUN_0055d8c0(coords(from), coords(to), speed)`:
  ```c
  d = isqrt(dx*dx + dy*dy);
  t = ((d / GNPRTB[5120]) * speed) / 100;
  return t == 0 ? 1 : t;
  ```
  with GNPRTB 5120 = 5 and speed = the product's hyperdrive value.

hyp: the displayed values are in absolute days from the current tick, not
relative. "Days: 8" means the product finishes in 8 game days; "Days: 88"
means it arrives at its destination in 88 game days.

## Reserved (`0x216`)

TEXTSTRA 12294 "Reserved". The STRATEGY record has bitmap 11902 (the check
mark) at word 10, indicating it is a toggleable state. From the manual
(p. 86):

> Reserve the construction yard. This means that if you turn over
> Maintenance Production to your agent, the agent won't use that facility
> to build mines and refineries. You might do this if you want to keep a
> construction yard available for your own purposes.

hyp: the Reserved flag is stored on the `0xa0..0xaf` manager object, and the
AI's Manage Production/Manage Garrisons agent skips reserved managers.

## Port notes

- The Destination order acts on `0xa0..0xaf` manager objects. port: the
  port keeps one destination per system and production area
  (`ManufacturingState::set_destination`) in their place.
- The facility icon's menu enables Destination when the system has a
  manufacturing facility of the player's (port: its production areas). The
  targeting release on a planet or a fleet sets every area's destination
  (`PanelAction::SetDestination`), logged as `[interface] command=0x214
  destination=production_destination status=set`. Reserved stays disabled.
- Gate: `tools/interface-parity/fleet-window.mjs`, case `destination`
  (fixture code 54, a player shipyard at the primary system), both sides.
- Best Time to Deployment = Best Time to Completion + transit time, where
  transit time is the same formula as fleet/product travel
  (`build-delivery.md`).
- port: the Destination targeting mode (cursor crosshairs, click a system)
  parallels the Mission targeting mode.

## Open

- The exact Destination order handler (which function processes `0x214`
  after the player selects a target system). It likely updates the manager's
  destination key and all its queued products' `+0x3c`.
- Whether the Destination order fires a targeting mode or opens a dialog.
- The `0xa0..0xaf` object's full field layout and vtable.
- Whether the Build Selection window computes "Best Time to Deployment"
  at display time from the current destination, or stores it.
- Which `FUN_0051f8f0` factory entry handles kind `0x214`.

## Supporting decompiles

`FUN_0052c170`, `FUN_00512700`, `FUN_0052b960`, `FUN_0052bee0`,
`FUN_0055d8c0`, `FUN_00429440`, `FUN_00604500`.
