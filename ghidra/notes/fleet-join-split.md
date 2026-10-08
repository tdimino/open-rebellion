# Joining and splitting fleets

Recovered 2026-10-05 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis), with STRATEGY.DLL's menu records and the manual. Item 3 of the
fleet-actions plan. Every function named here has a `FUN_<address>.c` note in
this directory.

The manual (p. 120, "Rearranging Fleets") gives the player's view:

> To move ships or troops from one fleet to another, simply drag the item to
> their new destinations. [...] If you move all the ships out of a fleet, the
> fleet is automatically disbanded. Right-click on a ship to bring up its
> menu. To create a new fleet with the selected ship as its only member,
> select **Create Fleet**. To create a larger fleet, select multiple ships by
> holding down the CTRL key while clicking and then right-click on one of the
> selected ships and select **Create Fleet**.

Page 122 adds: "If you move a ship onto a fleet in a different sector, that
ship will immediately be considered a member of the fleet but will still be in
hyperspace for several days until it arrives."

## What a fleet holds

A fleet's room (slot `+0x78`, `FUN_004fe160`) accepts only capital ships:
the asked type range must lie in `0x14..0x1c`; anything else sets "cannot
hold" (`FUN_004fe160.c:26-41`). Fighters (`0x1c..0x20`), regiments and
characters ride aboard ships (the ship's room `FUN_00500b40`,
`fleet-window.md`). A fleet's direct children are therefore its capital
ships, and each carries its own cargo.

The fleet's acceptance (slot `+0x74`, `FUN_004fe0d0`) is `FUN_00553410` with
"needs completed" set when `+0x58` bits 1 and 2 are both clear: another
side's fleet, an uncompleted or destroyed fleet, and one en route refuse as
for a regiment (`regiment-unload.md`).

## Joining: a move whose destination is a fleet

A fleet's or ship's Move (`0x201`) released on a Fleet window takes the
window's `+0x70` object, the item under the point or the window's fleet
(`move-order.md`). The route accepts a fleet or capital ship as the
destination (`FUN_005531b0`).

- **The team.** A fleet's team handler (slot `+0x2c`, `FUN_004ffc90`)
  expands the fleet into its own-side members (walk mode 3, `FUN_004fcd00`)
  when the order's target is a fleet (`0x08..0x0f`); otherwise the fleet
  itself moves (`FUN_004ffc90.c:19-38`). So each capital ship is routed to
  the target fleet by itself.
- **The execute.** A fleet's reparent (slot `+0xa8`, `FUN_004feca0`) with a
  fleet destination moves every existing (`+0x50` bit 6) and completed
  (bit 2) member into it through the generic reparent `FUN_004f8630`; the
  fleet itself stays. Only a system (`0x90..0x97`) or `0xf2` destination
  moves the fleet whole (`FUN_004feca0.c:28-49`, `:54-76`).
- **Across systems** the per-object execute (`FUN_00556390`) sets the
  in-transit bit and reparents at once (`regiment-unload.md`): the ship is a
  member of its new fleet at once and in hyperspace until it arrives, as
  the manual says.
- **Mover refusals** are the per-object ones of `FUN_00555920`: across
  systems a ship with no speed (`1`/`0x18`). A fleet or ship in hyperspace
  takes no orders (manual p. 121; `FUN_004f9860`).
- `FUN_00553aa0` turns a move into the mover's own fleet into `0x26`
  (`move-order.md`).

## Disbanding

When a fleet's capital ships change (slot `+0xc4`, `FUN_004fe870`, for a
child of type `0x14..0x1b`), `FUN_004fe630` walks its own-side capital ships
(mode 3, `FUN_00502e30`):

- some left: it sets `+0x58` bit 0 (`FUN_004fe230(1)`) and the object's
  created and completed bits (`FUN_004f7480(1)`, `FUN_004f74f0(1)`);
- none left and bit 0 set: it calls slot `+0xac(0x14)`, `FUN_004f8660` →
  `FUN_004f71d0`, which sets the object's state byte (`+0x40`) to `0x14` and
  notifies both side views (`FUN_004fe630.c:21-43`).

Bit 0 means "has held ships", so an emptied fleet is disbanded. hyp: state
`0x14` removes the fleet; its view handler (`+0x100`) is not read here.

## Splitting: Create Fleet (`0x270`)

- **The menu.** A capital ship's order list (slot `+0x3c`, `FUN_00500880`)
  is `DAT_006b2b28`, built once by `FUN_00502bd0`: the ship base list
  (`FUN_00557ce0`: `FUN_00558380`'s list plus `0x201`, `0x202`, `0x204`,
  `0x200`), then `0x203` and `0x270`. STRATEGY.DLL's `RT_RCDATA` record 624
  (`0x270`) reads `(0x270, 0, 50, 0, 0, 12319, 0, 0, 0, 0, 0, 2, 7)`: sort 50,
  TEXTSTRA 12319 "Create Fleet". With the shared Encyclopedia and Status the
  menu sorts Move (10), Confirmed Move (12), Create Fleet (50), Rename (500),
  Encyclopedia (1000), Status (1001), Scrap (2000).
- **The order.** Factory `FUN_00580a10` → `FUN_005809c0`, vtable
  `0x00669c08` over the order base `FUN_0051fa20`. Its command slot (`+0x48`,
  `FUN_00580a90` → `FUN_00578ab0`) makes the plain move command (vtable
  `0x00669558`, kind `0x201`, execute `FUN_00578f30` → `FUN_00556390`). Its
  leg slot (`+0x50`, `FUN_00580b00`) routes the team to its own system for
  the team's side (`FUN_00553b80`) through `FUN_00551630`.
- **The leg.** For a system destination the leg builder (`FUN_00552300`)
  asks, for route kind `0x270`, `FUN_00509b40`: the first fleet of the side
  in the system, not destroyed, with `+0x58` bit 2. Other kinds ask
  `FUN_005097d0`, the same with bit 1.
- **Spare fleets.** Those fleets are spares that each system keeps.
  `FUN_0050be00` (bit 2) and `FUN_0050bc60` (bit 1), run from the system's
  slots `+0xc0`, `+0xc4` and `+0xc8` (`FUN_00508250`, `FUN_00508440`,
  `FUN_00508660`), walk the system's fleets (mode 4). A spare that is
  destroyed or holds anything loses its bit (`FUN_004fe310(0)`,
  `FUN_004fe2a0(0)`). A side with no spare gets one from `FUN_00512d00`
  (`FUN_0050be00.c:76-92`):
  - `FUN_004f7d50(system, 0x8000004, side, .., 0, 0)` makes an object of
    family `0x08` in the system through the class factory (`FUN_0053f100`),
    runs its side setup (slot `+0x94`, `FUN_004fe790` → `FUN_004fef10`) and
    reparents it into the system (`+0xa8`), leaving created and completed
    clear (`FUN_004f7480(0)`, `FUN_004f74f0(0)`);
  - clears bit 0 (`FUN_004fe230(0)`) and sets bit 1 or 2.

  A spare is not existing (`+0x50` bit 6 needs created), so no list shows
  it. Create Fleet moves the selected ships into the side's bit-2 spare,
  which becomes a fleet when its first ship sets bit 0, created and
  completed (`FUN_004fe630`); the system then makes a new spare.
- **A ship's plain Move to a system** goes the same way into the bit-1
  spare: a lone ship never sits in a system, whose room refuses capital
  ships (`FUN_00507750`).
- A Create Fleet of every ship in a fleet empties it, so it disbands.

## Names

A name is the object's own string (`+0x34`) or its record's (`+0x2c`,
`FUN_004f6270`, `FUN_004f62d0`). The fleet's creation sets neither. hyp: the
side views hold "Fleet N" (manual p. 123: "The default names of Fleet 1,
Fleet 2, etc."); untraced.

## Port notes

- The original's ships carry the fighters, regiments and characters, so
  they go with their ship; nothing re-homes them in a split or join. The
  port keeps the rosters on the fleet, each squadron and regiment naming
  its ship (`rebellion-core/src/carriage.rs`, `fleet-window.md` "Which
  ship"): a split takes those aboard the ships that leave, and the
  characters when the fleet's first living ship leaves (they ride it). A
  join keeps every unit's ship. port: regiments held on the fleet itself
  (no ship had room) that the staying ships cannot carry, counting
  regiments on their way to board, go with the ships that leave, the last
  in key order first, so `destroy_untransportable_cargo` loses none; a hold
  goes with them.
- port: a Death Star whose class is a capital ship's
  (`CapitalShipClass::is_death_star`) carries the fleet's Death Star flag
  with it in a split or join; a flag with no Death Star ship is the separate
  tactical object and stays with its fleet.
- port: a ship order keeps the fleet's roster (`fleet_join::roster`, its
  ships' classes in order) from when the ship was chosen, and is refused
  ("the fleet's ships have changed") if a ship was lost or gained since:
  the clock runs while a menu, drag or targeting is open, and an index
  would otherwise name another ship.
- A fleet's Move onto a fleet is checked as a move to a system is
  (`FUN_00487740`, `validate_join`) before anything asks; Confirmed Move
  asks, and Move asks when the fleet leaves a blockaded system of its side
  (`FUN_00487cc0`, `join_confirms`). port: a ship's own Move never asks.
- port: a fleet that joins on arrival comes in after the other arrivals of
  its pass, so a target arriving in the same pass is already there.
- port: cargo that joins a fleet holding regiments loaded where it orbits
  shares that hold, so the automatic landing (itself unsourced, roadmap)
  skips it until the fleet next arrives somewhere.
- port: no spare fleets. The port creates the fleet when Create Fleet
  executes; the result a player sees, a new fleet of the selected ships in
  their system, is the same.
- Fleets are runtime objects in the saved `GameWorld` slotmap; no DAT
  identity or id allocator is involved.
- The port's `apply_fleet_arrival` merged same-side fleets without
  characters or a Death Star whenever one arrived where another orbited. No
  original rule does this; the original keeps fleets apart until a player
  joins them, and the merge would undo a split at the next arrival. It is
  retired (Tom, 2026-10-05): a fleet now merges on arrival only into the
  fleet its move named (`MovementOrder.join`, save v26), and only when that
  fleet is still there, of its side and not in hyperspace; otherwise it
  stays a fleet of its own (hyp).
- port: a fleet moved onto a fleet in another system departs whole (its
  ships split first when only some move) and joins on arrival, where the
  original reparents each ship at once and lets it travel (manual p. 122).
  The Death Star stays with its fleet unless the whole fleet joins.

## Still open

- Who calls the system slots `+0xc0`, `+0xc4` and `+0xc8`, and so when a
  spare appears.
- The view handler for state `0x14` and whether a disbanded fleet's id is
  reused.
- Where "Fleet N" comes from, and its counter.
- `FUN_00558380`'s base order list (`DAT_006bb368`).
- Ctrl multi-selection in the Fleet window's lists (the port moves one
  ship per drag or menu order).
- A system window drag (`0x214`) released on a fleet: the port still
  refuses it.

## Supporting decompiles

`FUN_004feca0`, `FUN_004fe0d0`, `FUN_004fe160`, `FUN_004ffc90`,
`FUN_004fcd00`, `FUN_004fe630`, `FUN_004fe870`, `FUN_004fe230`,
`FUN_004fe2a0`, `FUN_004fe310`, `FUN_004f7480`, `FUN_004f74f0`,
`FUN_004f71d0`, `FUN_0050be00`, `FUN_0050bc60`, `FUN_00512d00`,
`FUN_004f7d50`, `FUN_004fef10`, `FUN_005131b0`, `FUN_00500880`,
`FUN_00502bd0`, `FUN_00557ce0`, `FUN_005809c0`, `FUN_00580b00`,
`FUN_00553b80`, `FUN_00509b40`, `FUN_005097d0`, `FUN_00552300`,
`FUN_00507750`, `FUN_004f6270`, `FUN_004f62d0`, `FUN_004fe790`,
`FUN_0053f100`, `FUN_00558380`, `FUN_00502e30`.
