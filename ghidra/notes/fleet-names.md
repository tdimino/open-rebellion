---
title: "Fleet and Ship Names: Fleet 1, Galleon 2"
description: "How a fleet or capital ship gets its default name: FUN_00517760 numbers an object still bearing its record's name from its side's counter for that record (FUN_005302c0), which never goes down"
category: "ghidra"
created: 2026-10-06
updated: 2026-10-06
tags: [fleets, capital-ships, names, counters]
---

# Fleet and ship names

Recovered 2026-10-06 with Ghidra 12.1.3 headless (read-only project). The
manual agrees: "The default names of Fleet 1, Fleet 2, etc., are not very
meaningful" (p. 123).

## The name

An object's name is its own `+0x34` string, else its record's `+0x34`, else
empty (`FUN_004f62d0`, `FUN_004f6270`). Side setup (`FUN_004f9190`, from
`FUN_004fef10`) starts a new fleet with the record's name, TEXTSTRA 11523,
"Fleet". `FUN_004f6e60` sets `+0x34` and tells both sides' views.

## The number

`FUN_00517760` runs for an existing object (`+0x50` bit 6) whose class
answers slot `+0x48`. When its name still equals the record's
(`FUN_005f3390`), it:

1. takes its side's counter object (`FUN_00518750`: side `+0x24 >> 6 & 3`;
   `FUN_00506ea0`: `DAT_006b2bb0 + 0xc4` for side 1, `+0xc8` for side 2);
2. asks `FUN_005302c0` for the next number: a list keyed by the record
   (`FUN_00402d80`), whose node `[8]` it adds one to and returns; a missing
   node is made by `FUN_004f42a0` (its constructor `FUN_00540a50` is unread;
   the manual's first fleet is Fleet 1). Nothing lowers the count, so a
   number is never given twice, even after its fleet is gone;
3. writes record name + " " (`0x006a7f40`) + the number (`%ld`) through
   `FUN_004f6e60`.

A renamed fleet no longer bears the record's name, so it is never numbered
again.

## Capital ships

The same routine numbers capital ships, one counter per side and class
(the list's key is the record, `FUN_00402d80`): "Imperial Star Destroyer 1",
"Bulk Cruiser 3". `empire-day30-yaga-alert-control.sav` holds Imperial Star
Destroyer 1, Carrack Light Cruiser 1 and 2, Victory Destroyer 1, Galleon 1
to 3 and Bulk Cruiser 3, beside Fleet 1 to 5; no fighter, regiment or
character carries a number.

The tactical screen shows that name: the battle unit's `+0x144`
(`FUN_005b1610`) is set by the capital ship constructor `FUN_005b05c0` (the
fighter's is `FUN_005b9c60`) from the factory `FUN_00595f60`, which
`FUN_0040b8a0` calls with `FUN_004f62d0(strategic object)`.

## When it runs

`FUN_005140c0` (an object's load) and `FUN_0051b7f0` (a pass over every
object, `FUN_005136d0` state `0x17`) call it. `FUN_005140c0` is also what
event `0x300` fires for an object that becomes existing
(`death-star-product.md`, section 1), so an ordered capital ship is numbered
when that event is served, not by the order itself. The order (list `+0xa4`)
raises the event into queue `+0xb0`, and `FUN_0051df30` runs one item per
frame (phase 5 of `FUN_0041dff0`), so the ship shows its record's name for
the frame between. The ds-b line (Wine, day 169): right after a Build at
Coruscant the ships band read "Death Star"; by day 185 it read "Death
Star 2", the replaced ship having been "Death Star 1". Where a fleet made in play
(Create Fleet, a completed ship) is first numbered is untraced; next to read
is the fleet's slot `+0x114` handler `FUN_004fc290`, reached from
`FUN_004f7480` when a spare fleet becomes real (`FUN_004fe630`).

## Ported

- `GameWorld::fleet_names` (`rebellion-core` `world`): each fleet's name and
  each side's last number; saved (v27).
- `GameWorld::insert_fleet` numbers every fleet the game makes: seeding,
  completed ships and fighters (`integrator.rs`), and Create Fleet's new
  fleet (`fleet_join.rs`). hyp: as it is made, in creation order.
- `fleet_label` (`system_window.rs`) reads the name, so the System window's
  Fleets tab, the Fleet window, the Fleet Finder and the move confirmation
  all show it. A fleet with no number shows "Fleet" (`FUN_004f62d0`).
- Rename (`0x203`, `main.rs`) sets the name through
  `GameWorld::rename_fleet` and `rename_ship`; a renamed object is never
  numbered again.
- `GameWorld::number_capital_ships` gives each living ship with no name of
  its own "<class name> N" from its side's counter for that class (saved,
  v38). It runs after seeding (`seeds.rs`), after an order seats its
  ships (`manufacturing::seat_ordered_ships`) and after a ship completes
  (`integrator.rs`), in fleet slot order, then roster order. port: the
  original's one frame between an order and its ship's number has no
  counterpart.
- The tactical battle takes the world's name (`rebellion-core` `tactical.rs`,
  `BattleSession::new_with_forces`), so the
  panel and the Results window show it.

## Supporting decompiles

`FUN_00517760`, `FUN_00518750`, `FUN_00506ea0`, `FUN_005302c0`,
`FUN_004f62d0`, `FUN_004f6e60`, `FUN_004f9190`, `FUN_004fef10`,
`FUN_005140c0`, `FUN_0051b7f0`, `FUN_005136d0`, `FUN_004fe630`,
`FUN_004f7480`, `FUN_004f7d50`, `FUN_005b1610`.
