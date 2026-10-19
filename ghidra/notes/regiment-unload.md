# Unloading a regiment by hand

Recovered 2026-10-04 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Item 2 of the fleet actions plan
(`~/.claude/plans/open-rebellion-2026-10-04-fleet-actions-and-quadrants.md`).
It builds on `move-order.md`, `fleet-window.md` ("Loading a regiment onto a
fleet"), `object-state-flags.md` and `build-delivery.md`.

## The order

A regiment aboard a fleet is listed in the Fleet window's Troops tab. A drag
out of that list ends in `0x29a`, and the galaxy view issues `0x201` Move
(`0x202` with Ctrl) for the whole selection against the drop window's `+0x70`
object (`move-order.md`, "A drag is a move": window type 4). A regiment's
pop-up Move released on a window asks the same `+0x70` (`move-order.md`, "The
release"). The drop targets:

| Window | `+0x70` |
|---|---|
| Sector (type 1) | the system under the point |
| System (type 9) | `FUN_004aa470`: the window's subject |
| System Defenses (type 10) | `FUN_004aa380`: the list item under the point, else the subject |
| Missions (type 11) | `FUN_004a09a0`: the picture gives `+0x1c8`, the list item under the point, else the subject |
| Fleet (type 4) | the fleet or ship under the point (loading, `fleet-window.md`) |

An item that is not a system, fleet or capital ship fails the route with
`1`/`0x25` (`FUN_005531b0`, `move-order.md` "Route refusals").

`FUN_00487cc0` confirms a `0x201` only when the first member's container is a
blockaded system of the member's side. An embarked regiment's container is
its ship, so an unload never asks.

## Refusals

1. **Can move** (`FUN_00578c00`). The regiment's slot `+0x6c` is
   `FUN_00504350`: `FUN_004f9860` (side, untraced, en route and the rest,
   `object-popup-menu.md`), else `1`/`1`. `FUN_005555e0`'s blockade refusal
   (`0x90`/4) covers families `0x20..0x21` only, not regiments.
2. **The group** (`FUN_0053d430`, the `0x204` sub-order's `+0x50`). It
   refuses `1`/`0x28` when the destination's side bits (`+0x24` bits 6..7)
   differ from the order's side, unless every member is a fleet or capital
   ship, or every member is a regiment and the destination is an existing
   (`+0x50` bit `0x40`) system that is not populated (`+0x88 & 1` clear,
   `FUN_0053d430.c:140`). So regiments cannot be put down on an enemy or
   neutral populated planet by hand. The mask is bit 0, populated
   (`economy-systems.md`).
3. **The command** (`FUN_00555920`). The checks run only for a deployed
   object (slot `+0x38` = `0x004f6410`, `+0x50` bit 16,
   `GameObjDeployedNotif`, `build-delivery.md`):
   - `FUN_00555410` → `FUN_00555460` tells whether the object and the
     destination resolve to the same system (`FUN_00555540` on both; it
     starts at 1, so an unresolved side counts as the same);
   - across systems, a zero speed (slot `+0x34(1)`) gives `1`/`0x18`;
   - across systems, with no refusal yet, a regiment (`0x10..0x13`) whose
     destination's side bits differ from its own (`piVar3[9]`, `+0x24`)
     gives `1`/`0x28`;
   - in the same system neither check runs.
4. **The route and legs** (`FUN_0053d430` after the side test:
   `FUN_00551190`, `FUN_005513a0`, then `FUN_00551630` per member). The leg
   builder picks the container each regiment enters and stores it at the
   command's `+0x48`, so step 3's "destination" is that container, not the
   window's target. See "Choosing the container" below.

The planetary assault (`FUN_0058c810`) is the only other path that lands regiments, up to six (`ground-assault.md`).

## Choosing the container

The route context (`FUN_00551100`) holds the side (`+0x14`, `+0x18`) and the
order kind (`+0x1c`; `0x204` for a regiment group). `FUN_005513a0` builds a
room node (`FUN_00552d10` → `FUN_00550700`) for the destination, its
container and every fleet and ship there (`FUN_004ffe70`, `FUN_00502db0`,
mode 4). Each node asks the object, for nine type ranges (`FUN_0054f5b0`:
`0x14`, `0x18`, `0x1c`, `0x20..0x2f`, `0x2c`, `0x30..0x3f`, `0x10..0x13`,
`0x08..0x0f`, `0x40..0x7f`), whether it accepts the kind (slot `+0x74`) and
how much room it has (slot `+0x78`); a node that accepts but reports no room
and no "unlimited" gives `1`/`0x27`.

For a system (vtable `0x0065e640`):

- **Room** (`FUN_00507750`): regiments, fighters, fleets, sectors,
  characters and `0x40..0x7f` are unlimited. Facilities are limited by
  `FUN_00509650`/`FUN_00509660` (`0x90`/1, `0x90`/2).
- **Acceptance** (`FUN_005073d0`) for a `0x204` regiment:
  - a destroyed system (`+0x50` bit 3) gives `1`/`0x22`;
  - `FUN_00553410` (it always returns 1; the status carries the
    refusal): a system not of the order's side gives `1`/`0x24` or
    `1`/`0x28`; not completed (`+0x50` bit 2 clear) `1`/`0x20`; destroyed
    `1`/`0x22`; en route `1`/`0x21`. For a regiment (members in
    `[0x10,0x14]`) the refusal stands unless the system has `+0x88` bit 1
    set, side 3 and is unpopulated (`+0x88` bit 0 clear); then the status
    is reset to accepted (`FUN_005073d0.c:118-127`). Bit 1 is set by every
    system's Populate (`FUN_005081f0`, from both `FUN_00566d40` and the
    rim class's `FUN_0056a550`, populated or not), so every unpopulated
    side-3 system takes a regiment: the AI's Drop target
    (`FUN_004ae440`). For a fleet (`[0x08,0x10]`) or a character
    (`0x240`) the refusal is lifted outright;
  - a blockaded system (`+0x88` bit `0x20`) gives `0x90`/4. That is lifted
    for fleets and characters (`0x240`, `0x241`) and kind `0x3f0`, not for a
    regiment.

`FUN_00552300` then places each member:

1. It tries the destination itself (`FUN_00552b10`).
2. For a system it also tries the first fleet of the side
   (`FUN_005097d0`, mode 4) that is not destroyed and has `+0x58` bit 1
   (`FUN_00509b40` and `+0x58` bit 2 for kind `0x270`); untraced: who sets
   those bits.
3. If the best status so far is still a refusal, it tries every fleet of
   the side in the system and each of their ships.
4. The best candidate wins (`FUN_0054fbb0`: no refusal beats a refusal).
   Its status becomes the order's, and the candidate becomes the leg.

For a fleet destination it tries the fleet's ships. For a capital ship
destination it tries that ship. A capital ship (vtable `0x0065d650`)
accepts through slot `+0x74`, `FUN_00500ac0` → `FUN_00553410`: of the side,
completed, not destroyed and not en route (`1`/`0x24`, `0x28`, `0x20`,
`0x22`, `0x21`). Its room, slot `+0x78`, is `FUN_00500b40`
(`fleet-window.md`, "Capacity"). The base slots `+0x70` (`FUN_004f6800`)
and `+0x7c` (`FUN_004f6980`) sit at the same offsets in the regiment,
system and ship vtables, which fixes the system vtable at `0x0065e640`.

So a regiment sent to a planet of its side lands on it. One sent to a
blockaded planet of its side, or to another side's planet, boards a friendly
ship in that system that has room. Without one, the order is refused with
the planet's status (`0x90`/4, `1`/`0x24` or `1`/`0x28`).

## A regiment has speed

A regiment's slot `+0x34` is `FUN_004f63f0`: `DAT_006b9050` while the object
exists (`+0x50` bit 6, `object-state-flags.md`), else 0. `DAT_006b9050` is
GNPRTB 1, which is 100 in every column (`FUN_0053e0b0`, `build-delivery.md`
"Speed (`+0x34`) and mobile"). Special forces and facilities share it. So `1`/`0x18` never
refuses a live regiment. A regiment may therefore move on its own to another
system: onto a planet of its side, or aboard a friendly ship there. `fleet-window.md` and the port carried "a regiment has no speed"
(hyp), and the port refuses a regiment's cross-system move with
`TroopTransportError::RegimentCannotTravel`. That refusal is not the
original's.

## Execute (`FUN_00556390`)

```c
obj = resolve(object);
if (!obj || !obj->deployed())          // slot +0x38
    return obj->move_into(dest, ctx);  // slot +0xa8, no transit
ok = same_system(object, dest, &same); // FUN_00555410
if (ok && !(obj->flags & 0x800))       // not autorouting
    ok = obj->set_in_transit(!same);   // FUN_004f7640, +0x50 bit 5
if (!ok || (obj->flags & 8)) return;   // destroyed
obj->move_into(dest, ctx);             // slot +0xa8
```

Slot `+0xa8` is `FUN_004f8630` → `FUN_0053efa0` (id to object) →
`FUN_004f6fd0`, which reparents the object (`FUN_0053a030`, `FUN_004fc600`),
updates both side views (`FUN_004f9d40`) and calls slot `+0xf4` with the old
and new containers.

So:

- **Within a system** (an unload in orbit) the in-transit bit is written
  clear and the regiment enters the planet's container at once. No time
  passes.
- **Across systems** the in-transit bit is set and the regiment changes
  container at once. Its en route bit (`FUN_004f8240`) then holds until the
  per-object transit (`build-delivery.md`) ends.

## What happens to the rest of the cargo

The order moves only the selected regiments; each is its own command object
(`+0x3c`). Regiments left aboard stay in their ship's container. Nothing in
the original lands cargo without an order: the port's automatic landing
(F-007E, "cargo lands only when its faction is the sole orbital faction") has
no source in this trace.

## Port notes

- `TroopTransportState::move_regiment` carries out the order: the en-route
  test, `FUN_0053d430`'s side test, the leg choice (planet, else the first
  fleet of the side there with room) and the execute. Within one system the
  regiment changes container at once; across systems it joins
  `TroopTransportState::transits` and `advance_transit` places it on its
  arrival day (hyp: hold and place, as `delivery.rs` does, so no walk of the
  destination sees it early). A fleet's room counts the regiments on their
  way to it. A regiment bound for a fleet dies with the fleet and follows a
  merge. A destination planet destroyed, or a fleet gone, loses it on
  arrival. The save carries the transits (v25).
- port: the leg builder's `+0x58` bit 1 fleet is not modelled; a fleet's
  room is the whole fleet's. The side-3 unpopulated planet takes a regiment
  (`TroopTransportState::accept_planet`); port: side 3 is a control with no
  Alliance or Empire holder, and bit 1 is taken as set on every system.
- port: a regiment leaving a planet on its own takes no departure roll. Its
  withdraw percent is 100 unless it entered the planet while blockaded,
  which a Move cannot do (`FUN_005073d0`'s `0x90`/4).
- The hold (port:) is per fleet: a hand unload takes regiments out of the
  cargo, and the hold ends once the fleet carries none. A regiment boarding
  by order holds its fleet, as a load does.
- The Fleet window's Troops tab is a drag source (`FUN_006083c0`'s distance
  and list rules); `main.rs` hit-tests the drop with the release windows'
  `+0x70` and issues the move. A surface regiment's pop-up Move does the
  same. A refusal files no text: its status goes to the side's advisor
  (`FUN_00487c90`; `main.rs` `transport_refusal_status`), as does a
  squadron's. A Confirmed Move (`0x202`, the pop-up row or a drop with
  Ctrl) runs the same checks first (`FUN_00487740`), then always asks; a
  Move asks too when the unit stands in a blockaded system of its side,
  under the evacuation warning (`FUN_00487cc0`, `FUN_0049a350` 0x7056).
  The window's line is the unit's name and days (`FUN_0053d200`): its trip
  at the default speed for a regiment, its class's for a squadron, none
  within one system.
- port: travelling regiments are not drawn (no Troops tab entry while en
  route, no map stream).
- Gate: `tools/interface-parity/fleet-window.mjs` cases `unload` (a drop
  on the System window of the fleet's own system), `unload-refused`
  (fixture code 51: the regiment starts aboard, and the drop lands on the
  System window of the other side's populated system, since the open Fleet
  window covers that planet in the sector window) and `travel`, on both
  sides.

## A squadron's Move

A fighter squadron (`0x1c`) takes the same order (2026-10-10, read from the
decompiles above). The regiment exceptions do not cover it:

- `FUN_0053d430`'s unpopulated-system exception is for an all-regiment
  group, so a squadron sent to another side's system, populated or not, is
  refused (`1`/`0x28`).
- `FUN_005073d0`'s `0x204` lifts test the member's type against `8..0x10`
  and `0x10..0x14`; `0x1c` is in neither, so a destroyed planet (`1`/`0x22`),
  another side's (`FUN_00553410`) and a blockaded one (`0x90`/4) refuse it,
  and the leg builder then tries the side's fleets there.
- `FUN_00555920`'s cross-system side check covers regiments
  (`0x10..0x13`) only. Its zero-speed check never refuses a squadron:
  `FUN_00502f80` returns the class hyperdrive, else the damaged rating, else
  the GNPRTB 1 default (`build-delivery.md`).
- A planet's room is unlimited (`FUN_00507750`); a ship's is its class
  `+0x26c` less the squadrons aboard (`FUN_00500b40`), and the leg builder
  keeps the ship with the most room (`FUN_00552b10`), else `1`/`0x27`.

Port: `TroopTransportState::move_squadron`, from the Fleet window's Fighters
tab or the Defenses window's Squadrons page. A cross-system squadron is held
in `TroopTransportState`'s squadron transits and placed on arrival, as a
regiment is; one bound for a fleet follows a merge and dies with the fleet.
port: a fleet's room is its whole hangar space less the squadrons bound for
it. port: a travelling squadron, like a travelling regiment, is out of the
world until it arrives, so upkeep and presence do not count it; the
original's object sits in its new container with the en route bit.
Checked e2e 2026-10-10 on both sides in native seed-42 games: an Alliance
Y-wing sent from Fleet 1 at Yavin to Hoth leaves the fleet with none and
Alliance upkeep at 284 (288 before) until it arrives by day 79, then 288; an Imperial
squadron from Coruscant's defences to Corellia leaves Coruscant at 9 (10
before) and arrives by day 33; a full fleet refuses with `NoSquadronRoom`
and a neutral destination with `OtherSideDestination`.

## Still open

- Who sets fleet `+0x58` bits 1 and 2, which steer the leg builder.
- `blockade-troop-withdrawal.md` gives the system vtable as `0x0065e638`
  with `FUN_00508660` at `+0xd0`. From `0x0065e640` that function is
  `+0xc8`, and `FUN_00514a60` calls `+0xc4`, `+0xcc` and `+0xd0`; which
  objects those calls reach is not checked here.
- `FUN_004f9860`'s untraced condition for a regiment.
- Which walks of a system use modes 3 and 4 and so would see a regiment
  en route (the hold-and-place hyp above, shared with `delivery.rs`).
- The meaning of system `+0x88` bit 1 (set with bit 3 by `FUN_005081f0`,
  system slot `+0xbc`).
- The `0x204`/`0x241` split of a `0x201` group and how `FUN_0053d430`'s
  status reaches the advisor (`mission-dialog.md`, "Refusal").

## Supporting decompiles

`FUN_00556390`, `FUN_00555410`, `FUN_00555460`, `FUN_00555540`,
`FUN_00555920`, `FUN_0053d430`, `FUN_004f7640`, `FUN_004f8630`,
`FUN_004f6fd0`, `FUN_004f63f0`, `FUN_004f8240`, `FUN_00504350`,
`FUN_006158b0`, `FUN_0050a430`, `FUN_0053e0b0`, `FUN_00551630`,
`FUN_00552300`, `FUN_00552b10`, `FUN_005097d0`, `FUN_00509b40`,
`FUN_00550700`, `FUN_00550c90`, `FUN_00550de0`, `FUN_00550af0`,
`FUN_0054f5b0`, `FUN_0054fbb0`, `FUN_005073d0`, `FUN_00507750`,
`FUN_00553410`, `FUN_005513a0`, `FUN_00500ac0`.
