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
- The build-delivery note's manager keeps `+0x74` (ProductKey), `+0x78`
  (DeploymentKey), `+0x5c` (progress), `+0x68` (cost), and the Destination
  order writes its `+0x7c` (below; `rebexe-map.md` names it TargetKey).
- Family `0xa0..0xa4` appears in the vtable list alongside facility families
  (`0x20..0x2d`, `0x80`).

## How the player picks a destination

### From the sector window's facility icon menu

1. Right-click the facility icon (kind 4) in the sector window.
2. Select "Destination" (`0x214`).
3. hyp: the cursor changes to targeting crosshairs (the same targeting mode
   as Mission and Build orders, `FUN_00422ce0`'s `+0xc0` mode). The player
   clicks a system on the galaxy map or a system's Defense/Fleet window.
4. The target becomes the manager's destination (`+0x7c`), and every
   product the manager has queued moves into the container its new leg
   chooses (below).

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
b = product->slot_0c();  // the product's location key (its container's system)
set_deployed(product, 1);
if (a != b && a && b) set_enroute_active(product, 1);  // starts transit
set_completed(product, 1);
```

The manager's destination is its `+0x7c`: `FUN_005298d0` writes it and
notifies both side views, and `FUN_004c27b0` reads it. The product sits in
its destination container from its order, so at completion `slot_0c()` gives
the destination's system, and the comparison with the manager's location
decides whether the product travels.

## The order's checks and execute

`FUN_00537180` (slot `+0x50`) checks each command's object:

- **Remaining (`+0x58`) is zero** (`FUN_00537180.c:160-306`), and the object
  is the facility manager (family `[0xa0,0xa2)`, `0xa0000005`,
  `FUN_0050e200.c:24-38`): the destination goes through `FUN_00553410` and
  must be a system, else `1`/`0x25` (`:194-212`).
- **Remaining is zero, and the object is any other manager,** the ship
  manager (`0xa2…`) or the troop manager (`0xa4…`): the destination must be
  completed (`+0x50` bit 2), else `1`/`0x20`; not en route (bit 4), else
  `1`/`0x21`; and not destroyed unless it is a system, else `1`/`0x22`
  (`:214-256`). It must be a system (`0x90..0x97`), a fleet (`0x08..0x0f`) or
  a capital ship (`0x14..0x1b`), else `1`/`0x25` (`:262-306`). The checked
  destination is stored at command `+0x48`.
- **Remaining is non-zero** (`:321-345`): `FUN_00552210` lists the manager's
  products (`FUN_005287f0`), and each gets a new leg through `FUN_00551900` →
  `FUN_00552300`. The pair (chosen container, product key) goes into the
  command's list at `+0x50` (`FUN_00583f50`).

The command's execute calls the manager's slot `+0x200`, `FUN_00529cd0` →
`FUN_0052a380` (vtable `0x0065fbf8` slot 128). It stores the destination
(`FUN_005298d0`), then reparents each listed product into its container
(slot `+0xa8`). A queued capital ship therefore moves at once into the new
destination system's spare fleet, or into a destination fleet. The fleet it
leaves disbands when it holds no other ship (`FUN_004fe630`,
`death-star-product.md` section 1).

The facility icon's team is every family-`0xa0..0xaf` object of the system
(`FUN_00512700`, kind 4), and in container order the facility manager comes
first. So a fleet released from the icon meets that manager's `1`/`0x25`.
The Manufacturing window's ship and troop bands each issue `0x214` for their
own manager (`manufacturing-build-selection.md`, the band menus).
In the route-1 Empire game played on from its day-123 save
(`death-star-product.md`, "Play evidence"), the icon's Destination released
on Fleet 1 changes nothing and shows no advisor line. The ships band's
Destination released on the same fleet moves the queued Death Star into it,
and the band then reads "Destination: Fleet 1". hyp: the icon's team holds
all three managers; the iterator's filter arguments are not read.

## How new queued products inherit the destination

The ship is created at order time (`FUN_0057f890` → `FUN_00529ab0` → `FUN_0052a140` → `FUN_004f7d50`), and its `+0x38` holds the manager's key (`death-star-product.md`). Its leg goes to the manager's `+0x7c` (`FUN_005385f0.c:204-208` → `FUN_004c27b0`). The container is the destination system's bit-1 spare fleet, or for a fleet destination the fleet itself, which accepts capital ships (`FUN_004fe160`).

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
- A squadron or regiment bound for a fleet rides the ship its Build chose
  (`QueueItem::carrier`), and an order its fleet's ships lack room for is
  refused with `1`/`0x27` (`FUN_005385f0` -> `FUN_00551a20` ->
  `FUN_00552300`, `build_selection::order_refusal`). hyp: a Destination
  order to a fleet moves the area's queued squadrons and regiments to its
  ships the same way, and is refused, changing nothing, when they lack the
  room (`build_selection::order_destination`).
- Gate: `tools/interface-parity/fleet-window.mjs`, case `destination`
  (fixture code 54, a player shipyard at the primary system), both sides.
- Best Time to Deployment = Best Time to Completion + transit time, where
  transit time is the same formula as fleet/product travel
  (`build-delivery.md`).
- port: the Destination targeting mode (cursor crosshairs, click a system)
  parallels the Mission targeting mode.

## Open

- Whether the Destination order fires a targeting mode or opens a dialog.
- The `0xa0..0xaf` object's full field layout and vtable.
- Whether the Build Selection window computes "Best Time to Deployment"
  at display time from the current destination, or stores it.

## Supporting decompiles

`FUN_0052c170`, `FUN_00512700`, `FUN_0052b960`, `FUN_0052bee0`,
`FUN_0055d8c0`, `FUN_00429440`, `FUN_00604500`, `FUN_00537180`,
`FUN_00553410`, `FUN_00551900`, `FUN_00552210`, `FUN_005287f0`,
`FUN_00583f50`, `FUN_00529cd0`, `FUN_0052a380`, `FUN_005298d0`,
`FUN_004c27b0`.
