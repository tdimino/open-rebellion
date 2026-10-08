---
title: "The Status window"
description: "Window type 0x1a (FUN_00442d70): the object menu's Status command (0x103), its generic 379 by 272 dialog, its per-family fillers, and the Character Status rows"
---

# The Status window

Recovered 2026-10-06 from REBEXE.EXE with Homebrew Ghidra 12.1.4 (headless,
read-only, on a copy of the project), with STRATEGY.DLL's bitmaps, TEXTSTRA
and the manual (p. 101, Fig. 3.46; p. 100). Every function named here has a
`FUN_<address>.c` note in this directory; the vtable slots `FUN_004443a0`
and `FUN_0044b250` are saved under those names.

## Opening

- The object menu's Status item is order kind `0x103` (TEXTSTRA 12293).
  `FUN_0051d990` adds it to every menu, enabled for a single selection that
  is not a system (`object-popup-menu.md`). Its factory `FUN_0043e710`
  (registered by `FUN_0041e580`) builds `FUN_0043e6c0`, vtable `0x006596d8`,
  whose kind slot returns `0x103`.
- The galaxy view's command `0xbbf` runs it through `FUN_0041cdf0` →
  `FUN_00436020` → `FUN_00486fb0`. For `0x100` and `0x103` that resolves the
  selection's id: Encyclopedia goes to `FUN_0041d6b0`, Status to
  `FUN_0041d7f0` → `FUN_0042a440`.
- `FUN_0042a440` posts `0x409` (close menus), then `0x468` with window type
  `0x1a` and a copy of the id.
- The galaxy view's `0x468` handler (`FUN_00422ce0`) drains pending messages,
  then for type `0x1a` builds the window only when the id is known to the
  player (`FUN_004f2d10` on the view's side `+0x9c`) or is the empty id. It
  allocates `0x1d8` bytes, calls `FUN_00442d70`, shows it, adds it to the
  view's children and gives it the focus. This is the move confirmation's
  path (type `0x10`).

## The window (`FUN_00442d70`, vtable `0x00659c18`)

`FUN_00606380(.., 0x17b, 0x110, ..)`: 379 by 272. `FUN_00606980` places it
from the galaxy view's rectangle (`+0xcc..+0xd8`). The subject id is `+0x124`,
the player's view of the object `+0x12c` (`FUN_004f2d10`), and GOKRES is
`+0x1d4`.

### Create (slot 14, `FUN_00443130`)

- **Background** (STRATEGY, module 7), by the player's side `+0x9c` and the
  object's side bits `(+0x24 >> 6) & 3`; the second number is for a player
  who is not side 1:

  | Object | Side-1 player | Other player |
  |---|---|---|
  | side 1 | 11554 (`0x2d22`) | 11557 |
  | side 2 | 11555 | 11558 |
  | other | 11556 | 11559 |
  | the empty id or the galaxy (`0xf1`) | 11554 | 11558 |

  All six are 379 by 272.
- **List** `+0x118` (`FUN_00607ea0`): at (18, 47), 208 by 204, with a
  vertical scroll bar. The rows are drawn into one bitmap 207 (`0xcf`) wide.
- **Title** `+0x130`: at (15, 18), 211 by 18, font 5, white, format `0x21`
  (one line, centred).
- **Name** `+0x198`: at (242, 137), 130 by 44, font 5, white, format `0x11`
  (centred, word-wrapped). `FUN_0044a210` fills it: the object's name
  (`FUN_004f62d0`), TEXTSTRA 12563 "Galaxy Overview" for the galaxy, 12564
  "Objectives" for the empty id.
- **Picture**: by family, `FUN_0042c3b0(gokres, id, object, 1, 1)` (the
  portrait, with its marks); for a mission (`0x50..0x7f`), GOKRES
  `class +0x30 & 0xfff`, plus `0x1000` unless its side bits are 1; for a
  production manager, GOKRES 263 (`0xa0..0xa1`), 262 (`0xa2..0xa3`) or 264;
  for a fleet, `FUN_0042c3b0` draws STRATEGY 10425 (`0x28b9`) for side 1
  and 10475 (`0x28eb`) otherwise; for every other object, GOKRES at the
  class record's resource (object vtable `+0x30`) `& 0xfff`, the list mini
  less `0x4000`. With its last argument set it then draws marks: a
  regiment's STRATEGY 11514 (`0x2cfa`, through `FUN_005fcc30`), the en
  route, damage and unbuilt marks (`+0x50` bits 4, 9 and 2);
  STRATEGY 11590 (`0x2d46`) for the galaxy and the empty id.
  It is blitted keyed (`FUN_005fd0f0`) centred on (307, 64).
- **Rows**: by the id's family:

  | Family | Filler |
  |---|---|
  | `0x08..0x0f` fleet | `FUN_00449200` |
  | `0x14..0x1b` capital ship | `FUN_00446fd0` |
  | `0x30..0x3b` character | `FUN_004486f0` |
  | `0x20..0x27` facilities | `FUN_00444e20` |
  | `0x1c..0x1f` fighters | `FUN_004444c0` |
  | `0x10..0x13` regiment | `FUN_00445280` |
  | `0x28..0x2f` facilities | `FUN_00446c50` |
  | `0x90..0x97` system | `FUN_00446250` |
  | `0x50..0x7f` mission, not hidden (`FUN_00520b70`) | `FUN_00445c60` |
  | `0x3c..0x3f` special force | `FUN_00445780` |
  | `0xa0..0xaf` production manager | `FUN_0044acd0` |
  | `0xf1` galaxy, or the empty id | lists, `FUN_0044b2d0` (side 1) / `FUN_0044bb70` |

  Each filler appends a label to one list and a value to the other
  (`FUN_0044b1d0(row, text, list)`). `FUN_00449e00` lays them out in two
  columns: labels at x 0, values at x 103, each 103 wide, font 10, white,
  format `0x10` (word-wrapped). A row is as tall as its taller cell, and the
  value is pushed down to the label's bottom when the label is taller.
  `FUN_0044a050` draws them.

### Controls (slot 16, `FUN_00443020`; slot 18, `FUN_004443a0`)

- `0x66` at (258, 218), STRATEGY 11552/11553, help `0x1505`: closes the
  window and opens the Encyclopedia on the object (`FUN_0041d6b0`). It is
  disabled, drawing 11612 (`0x2d5c`), for the galaxy and the empty id.
- `0x65` at (324, 218), STRATEGY 10370/10371, help `0x1954`: closes.
- Message `0x40d` closes (slot 5, `FUN_0044b250`); closing posts `0x467` to
  the galaxy view.

No key slot is overridden; the base dialog's keys are not traced.

## Character Status (`FUN_004486f0`)

The title is TEXTSTRA 34595 "Character Status". The rows, in order:

| Label | Value |
|---|---|
| 34596 "Commanding:" | the name of the object at `+0xa8`, else 34644 "None" |
| "Attached: " (see below) | the name of the character's container `+0x1c` (`FUN_0044a2e0`) |
| 34406 "Status:" | the first that holds: `+0x50` bit 4 (en route) 34628 "Enroute"; `+0xac` bit 0 34630 "Captured"; `+0x94` non-zero 34631 "Injured"; a mission at its key `+0x68` that is not hidden (`FUN_00520b70`) 34629 "On Mission"; else 34632 "Awaiting Orders" |
| 34662 "ETA Destination:" | only when en route: 14356 "Day " and the arrival day `+0x44` (`FUN_004fd2b0` → `FUN_005560b0`) |
| 34597 "Force Ranking:" | by the Force value `+0x8c`: below 10 34644 "None", below 20 34633 "Novice", below 80 34640 "Trainee", below 100 34641 "Jedi Student", below 120 34642 "Jedi Knight", else 34643 "Jedi Master" |
| 34598 "Diplomacy Rating:" | `+0x7c` (slot `+0x1dc`) |
| 34599 "Espionage Rating:" | `+0x7e` (`+0x1e0`) |
| 34600 "Combat Rating:" | `+0x86` (`+0x1f0`) |
| 34601 "Leadership Rating:" | `+0x88` (`+0x1f4`) |
| 34608 "R&&D Capabilities" | empty |
| 34609 " Ship Design" | 34646 "Yes" when `+0x80` is non-zero, else 34645 "No" |
| 34610 " Troop Training" | `+0x82`, the same way |
| 34611 " Facility Design" | `+0x84`, the same way |
| 34615 "Possible Command Ranks" | empty |
| 34435 "Admiral:" | class `+0xa8` (`FUN_004ed1c0`), Yes or No |
| 34433 "General:" | class `+0xb0` (`FUN_004ed200`) |
| 34434 "Commander:" | class `+0xac` (`FUN_004ed1e0`) |

"R&&D" is the DrawText escape for "R&D".

### "Attached"

`FUN_0044a2e0` takes the literal `"Attached: "` (`0x006a8738`) when
`FUN_004067d0` returns `DAT_006b0df8`, else TEXTSTRA 34736. `FUN_00406600`
sets that flag from `FUN_00406850`: true when TEXTSTRA.DLL's version resource
FileVersion has zero in the fields after the first dot. The shipped
TEXTSTRA.DLL reads "1.00.00", so the label is the literal, as Fig. 3.46 shows.
This TEXTSTRA has no string 34736.

### Manual cross-check (p. 101, Fig. 3.46; p. 100)

| Claim | Status |
|---|---|
| Right-click a character, choose Status (p. 101, Fig. 3.45) | Confirmed: `0x103` |
| Rows Commanding, Attached, Status, Force Ranking, the four ratings, R&D Capabilities and its three areas, Possible Command Ranks (p. 101, Fig. 3.46) | Confirmed: `FUN_004486f0` |
| "Attached" is the location, or the destination when en route (p. 101) | Confirmed: the container, which for an object in hyperspace is its destination (`FUN_00556390`) |
| Portrait top right, name under it, two buttons below (Fig. 3.46) | Confirmed: (307, 64), `+0x198`, `0x66`/`0x65` |
| The Finder's Display opens "system defenses, fleet or mission" for a character (p. 100) | Confirmed: `FUN_00429440` opens the Missions window (kind 11) for a character on a visible mission, the Defenses window (kind 10) otherwise |
| Ratings range from 0 to 150 or higher (p. 101) | Not checked: data range |

## Defense Facility Status (`FUN_00444e20`)

Families `0x20..0x27`. The title is TEXTSTRA 34403 "Defense Facility
Status". The rows, in order:

| Label | Value |
|---|---|
| 34662 "ETA Destination:" | only when en route (`FUN_004fd2b0` → `FUN_005560b0` gives a non-zero arrival day and an open end): 14356 "Day " and the arrival day. It comes first, above Location |
| 34404 "Location:" | the name of the container `+0x1c` (`FUN_004f62d0`) |
| "Status:" (`FUN_0044a470`, below) | `+0x50` bit 4 (`0x10`) 34628 "Enroute"; else bit 2 (`0x4`) 34659 "Active"; else 34647 "Under Construction" |
| 34385 "Maintenance Cost:" (`FUN_0044a620`, below) | class maintenance (`FUN_004f2990` → `FUN_0053b870`, class `+0x4c`) |
| 34393 "Weapons Rating:" | only for families `0x22..0x27`: vtable `+0x1cc` minus `+0x1d4` = `FUN_00520b70` (class `+0x5c`, DEFFACSD `attack_strength`) minus 0 |
| 34627 "Shield Strength" | only for families `0x22..0x27`, no colon: `+0x1d0` minus `+0x1d8` = `FUN_00520b80` (class `+0x60`, DEFFACSD `shield_strength`) minus 0 |
| 34409 "Bombardment Defense Strength:" | `+0x1c0` minus `+0x1c4` = `FUN_00520b60` (class `+0x58`, DEFFACSD `bombardment_defense`) minus 0 |

The vtables of families `0x22..0x25` (`0x0065f828`, `0x0065f640`,
`0x0065fa10`, `0x0065eaf8`) put the thunks `0x558050`, `0x558060` and
`0x557750` (jumps to `FUN_00520b60`, `FUN_00520b70`,
`FUN_00520b80`) at `+0x1c0`, `+0x1cc` and `+0x1d0`, and `0x6158b0`
(`xor eax, eax; ret`) at `+0x1c4`, `+0x1d4` and `+0x1d8`. Every
subtraction therefore subtracts 0. The class record is the DAT record at
`+0x28`, so class `+0x58`/`+0x5c`/`+0x60` are DAT `0x30`/`0x34`/`0x38`.

### Shared facility rows

- `FUN_0044a470` writes the "Status:" row. Its label follows the
  "Attached" rule: the literal `"Status:"` (`0x006a8744`) when
  `FUN_004067d0` is set (the shipped TEXTSTRA), else 34406 "Status:". The
  value reads the object's flags `+0x50`, as above.
- `FUN_0044a620` writes 34385 "Maintenance Cost:". The value is
  `FUN_004f2990` (class `+0x4c`) when the object's family is in
  `0x10..0x3f`, else 0.

### Port

- Location: the system whose `defense_facilities` holds the key.
- Status: a port facility exists as a world object only once complete, so
  it is always "Active". Under construction it is a queue item
  (`ManufacturingState`), and in transit it is a `Delivery`; neither is
  selectable. port: Enroute and ETA never show.
- Maintenance: `buildable_classes[class].maintenance_cost`.
- Bombardment defense: `defense_facility_classes[class].bombardment_defense`.
- Weapons and shield: port: no runtime field. DEFFACSD `attack_strength`
  and `shield_strength` are parsed (`tools/dat-dumper`
  `DefenseFacility`) but no runtime class keeps them.

## Manufacturing Status (`FUN_00446c50`)

Families `0x28..0x2f`: shipyards, training facilities and construction yards
(`0x28..0x2a`), mines (`0x2c`) and refineries (`0x2d`). The title is TEXTSTRA
34560 "Manufacturing Status". The rows, in order:

| Label | Value |
|---|---|
| 34404 "Location:" | the name of the container `+0x1c` |
| 34662 "ETA Destination:" | only when en route, as above: 14356 "Day " and the arrival day. Here it follows Location |
| "Status:" (`FUN_0044a470`) | 34628 "Enroute" / 34659 "Active" / 34647 "Under Construction" |
| 34385 "Maintenance Cost:" (`FUN_0044a620`) | class `+0x4c` |
| 34561 "Standard Processing Rate:" | `FUN_00520b70`: class `+0x5c`, MANFACSD/PROFACSD `processing_rate` |
| 34392 "Bombardment Value:" | vtable `+0x1c0` minus `+0x1c4` = `FUN_00520b60` (class `+0x58`, `bombardment_defense`) minus 0 |

The vtables of `0x28`, `0x29` and `0x2a` (`0x00660280`, `0x00660070`,
`0x0065fe60`), `0x2c` (`0x006608c8`) and `0x2d` (`0x006606a0`) all hold
`0x558050` at `+0x1c0` and `0x6158b0` at `+0x1c4`.

### Port

- Location: the system whose `manufacturing_facilities` or
  `production_facilities` holds the key.
- Status, Enroute and ETA: as for defense facilities (always "Active").
- Maintenance and processing rate: `buildable_classes[class]`
  (`maintenance_cost`, `processing_rate`).
- Bombardment value: port: no runtime field. MANFACSD/PROFACSD
  `bombardment_defense` is parsed but not kept.

## System Status (`FUN_00446250`)

Families `0x90..0x97`. The title is TEXTSTRA 34432 "System Status". The rows
come at fixed positions 2 to 21:

| Label | Value |
|---|---|
| 34406 "Status:" (TEXTSTRA, not the literal) | the side bits `+0x24` bits 6–7: 1 34648 "Alliance", 2 34649 "Empire", else (0 or 3) 34657 "Neutral" |
| 34433 "General:" | `FUN_00509330(system, 3)`: the first character at the system whose `+0x96` is 3, by name; else 34658 "Not Assigned" |
| 34434 "Commander:" | `FUN_00509330(system, 1)`: `+0x96` is 1; else 34658 "Not Assigned" |
| 34436 "Loyalty to Alliance:" | `FUN_00507270(1)`: `+0x58` |
| 34437 "Loyalty To Empire:" | `FUN_00507270(2)`: 100 minus `+0x58` |
| 34438 "Energy Consumption:" | `+0x5c` (energy), ":" (`0x006a872c`), `+0x60` (energy allocated) |
| 34439 "Raw Material Consumption:" | the same `+0x5c` ":" `+0x60`. The original reads the energy fields here too, not raw material `+0x64`/`+0x68` |
| 34440 "Facilities:" | empty (`0x006b120c`) |
| 34441 "Mines:" | count of family `0x2c` (`FUN_0052d370`) |
| 34448 "Refineries:" | `0x2d` (`FUN_0052cf40`) |
| 34449 "Shipyards:" | `0x28` (`FUN_0052c8c0`) |
| 34450 "Training Facilities:" | `0x29` (`FUN_0052c5a0`) |
| 34451 "Construction Yards:" | `0x2a` (`FUN_0052c270`) |
| 34452 "Planetary Shields:" | `0x24` (`FUN_005276d0`) |
| 34453 "Defensive Batteries:" | `0x22..0x27` (`FUN_00526e90`), which also counts shields |
| 34454 "Death Star Shields:" | `0x25` (`FUN_0051c0d0`) |
| 34455 "Defense" | empty |
| 34456 "Regiments:" | `0x10..0x13` (`FUN_005044f0`) |
| 34457 "Fighter Squadrons:" | `0x1c..0x1f` (`FUN_00503550`), but the value goes to row 21 while the label sits on row 20 |

Each count is `FUN_004f6c00(system, [lo, hi), side, 3)` over the system's
contents. The side is the system's own side (`+0x24` bits 6–7), so the
counts are the holder's objects.

**Reachability.** `FUN_0051d990` disables Status (`0x103`) for a system.
`FUN_0041d7f0` is the only caller of `FUN_0042a440`, which opens the window.
It is reached only from `FUN_00486fb0`, on commands `0x103` (the object
menu), `0x113` (Galaxy Overview, the galaxy object `0xf1000004`) and `0x114`
(Objectives, the empty id). No traced path opens this filler, so in the
shipped game it is unreachable.

### Port

- Status: `System::control` (`ControlKind`). Contested maps to "Neutral".
- General and Commander: port: no command assignment (`+0x96`), so both
  always read "Not Assigned".
- Loyalty: `popularity_alliance` and `popularity_empire`, as percentages.
- Energy: `total_energy`, and `SystemEconomy::energy_allocated`. The Raw
  Material row repeats them, as the original does.
- Counts: the system's `production_facilities` (`is_mine`), its
  `manufacturing_facilities` by class family, its `defense_facilities` by
  class family, its `ground_units`, and the fighter squadrons of its
  fleets. The holder filter needs each object's side.

## Mission Status (`FUN_00445c60`)

Families `0x50..0x7f`, for a mission not hidden. The title is TEXTSTRA 34422
"Mission Status". The filler walks the viewing side's characters
(`FUN_004f30a0(view side +0x9c)`, families `0x30..0x3f`). Those whose
mission key `+0x68` is this mission make up the team. The first of them is
remembered, and those with `+0x78` bit 0 set are counted as decoys.

| Label | Value |
|---|---|
| 34423 "Target:" | if the first member is on the viewer's side, the mission's target id `+0x70`. Otherwise, for a member's mission of family `0x50..0x5f`, the member's location (vtable `+0xc`) when it is a system. The value is the name of `FUN_004f2d10(view side, id)`, or 34087 "Target Unknown" when there is no team, the id is empty or the viewer cannot see the target |
| 34662 "ETA Destination:" | only when the first member is en route: 14356 "Day " and its arrival day |
| 34424 "Team Size:" | the count of members, decoys included |
| 34425 "Decoys:" | the count with `+0x78` bit 0 |

`FUN_004f3220(view side, +0x78)` is called and its result is ignored.

### Port

- Team: the `ActiveMission`'s `team` and `decoys`, which together give
  Team Size (whether `captured` counts is untraced). Decoys is
  `decoys.len()`.
- Target: `target_character` / `target_object` / `target_system` by name,
  when the viewer sees it (fog).
- ETA: `MissionState::en_route`, the members' arrival day.

## Production Manager Status (`FUN_0044acd0`)

Families `0xa0..0xaf`: the band's manager (`production-destination.md`). Its
system is `+0x1c`. The title and yard count come from the family:

| Family | Title | Yards counted (at the system, its side) | Busy word |
|---|---|---|---|
| `0xa0..0xa1` | 6195 "Facilities Under Construction" | `0x2a` construction yards (`FUN_0052c270`) | 34672 "Building" |
| `0xa2..0xa3` | 6185 "Ship Construction" | `0x28` shipyards (`FUN_0052c8c0`) | 34672 "Building" |
| `0xa4..0xa5` | 6193 "Troops in Training" | `0x29` training facilities (`FUN_0052c5a0`) | 34673 "Training" |

The title is re-styled first: `FUN_00403e90(title, 0x11)`, `+0x134 = 0x24`,
`FUN_00601b30(title, 0xf, 0x12)` (font `0x11`, unlike the other fillers).

| Label | Value |
|---|---|
| 34404 "Location:" | the name of the system `+0x1c` |
| 34406 "Status:" | no yards: 34674 "No Facilities"; else, with nothing queued (`+0x58` is 0): 34665 "Idle"; else the busy word |
| 34664 "Items to Build:" | only when busy: `+0x58`, the number queued |
| 34661 "Estimated Day of Completion:" | only when busy and the system's side equals the manager's (`+0x24` bits 6–7): `FUN_004fd280` → `FUN_00555e60`, today (`FUN_004fd340`) plus the best time `FUN_00529360` |

The filler ends by adding the last value again on its own row
(`FUN_0044b1d0(last row, value)`). The copy lands on the same row and
position, so it draws over itself.

### Port

- The band's area: `ProductionArea` (ConstructionYard, Shipyard,
  TrainingFacility) at the system. Yards counted: the system's
  `manufacturing_facilities` of that family, for the holder.
- Items to Build: `ManufacturingState::queue(system, area)` length.
- Estimated Day of Completion: `ProductionQueue::completion_days(today)`
  (its last day). The side check is `System::control` against the
  manager's side.

## Unit fillers: shared conventions

The fillers below run on the player's view of the object (`+0x12c`). Each
writes a label into the left list and a value into the right one with
`FUN_0044b1d0(row, text, list)`. Two number formats are used, `FUN_005f31f0`
(`"%lu"`) and `FUN_005f31a0` (`"%ld"`). A "current:maximum" value appends
the literal `":"` (`0x006a872c`, not a slash) and the maximum. An empty
value (`0x006b120c`, `""`) marks a heading row.

The unit vtables were found by their slot `+4` type code: fleet
`0x0065d438` (`0x08`), capital ship `0x0065d650` (`0x14`), fighter
`0x0065def0` (`0x1c`), regiment `0x0065e438` (`0x10`), and special force
`0x0065e160` (`0x3c`). Their stat slots are thunks to class-record getters
(`[this+0x2c]+off`). The class record is the DAT record at `+0x28`, so
class `+off` is DAT `off - 0x28`. Wherever a "minus" slot is `0x6158b0`
(`xor eax, eax; ret`), the subtraction subtracts 0.

The ETA row is the same in every unit filler. `FUN_004fd2b0(obj, {1, -1},
&day)` is called, and when the second field is still -1 and `day` is
non-zero, the row is 34662 "ETA Destination:" with value 14356 "Day " and
`day` (`%ld`). It takes the next row number, and every later row shifts by
one.

## Fleet Status (`FUN_00449200`)

Families `0x08..0x0f`. The title is TEXTSTRA 34616 "Fleet Status". There is
no "Attached" row and no maintenance row. The rows, in order:

| Label | Value |
|---|---|
| 34406 "Status:" | `+0x50` bit `0x10` (en route) 34628 "Enroute"; else bit `0x4` (completed) clear 34647 "Under Construction"; else bit `0x20` (en route, active) 34628 "Enroute"; else 34632 "Awaiting Orders" |
| 34662 "ETA Destination:" | the shared ETA row |
| 12358 "Admiral" + ":" | `FUN_004fd790(fleet, 2)`: the first character on the fleet's capital ships (`FUN_00502e30`, side-filtered, mode 3) whose short `+0x96` equals 2 (`FUN_005006f0`): its name (`FUN_004f62d0`), else 34658 "Not Assigned" |
| 12360 "General" + ":" | the same with rank 3 |
| 12356 "Commander" + ":" | the same with rank 1 |
| 34624 "Number Of Ships" | no colon: `FUN_00501e70(fleet, +0x24 bits 6..7, 3)`, the count of families `0x14..0x1b` in mode 3 (`%ld`) |
| 34564 "Capacity:" | empty (heading) |
| 14368 "Fighter Squadrons" + ":" | the sum of each ship's slot `+0x26c` (fighter capacity), walking `FUN_00502db0(fleet, 3)` (`0x14..0x1b`, mode 3) |
| 34565 "Trooper Regiments:" | the sum of slot `+0x270` (troop capacity) |
| 34566 "Embarked:" | empty (heading) |
| 14368 "Fighter Squadrons" + ":" | the sum of slot `+0x23c`, the squadrons aboard each ship |
| 34565 "Trooper Regiments:" | the sum of slot `+0x240`, the regiments aboard each ship |
| 34567 "Personnel:" | the sum, over the ships, of families `0x30..0x3f` (characters **and** special forces) aboard in mode 3 (`FUN_00536da0`, `FUN_00513180`) |
| 34625 "Damaged Ships:" | the count of ships whose `+0x50` bit `0x200` is set |
| 34387 "Hyperdrive Rating:" | fleet slot `+0x34` (`FUN_004fd900`, a walk of the fleet's completed, active ships) non-zero means yes. When `FUN_004067d0` is set (the shipped TEXTSTRA, see "Attached") the value is the literal `"Yes"`/`"No"` (`0x006a8734`/`0x006a8730`), otherwise 34646 "Yes" / 34645 "No" |

### Port

- Status: `MovementOrder` present means Enroute. A port fleet exists only once
  complete, so Under Construction never shows.
- ETA: `MovementOrder` arrival.
- Admiral, General and Commander: port: no field. The port keeps no
  character command rank (`+0x96`). `Character::can_be_admiral` and its
  siblings are capabilities, not assignments, so each row reads "Not
  Assigned".
- Number Of Ships: `Fleet::ship_count()`.
- Capacity: the sums of `CapitalShipClass::fighter_capacity` and
  `troop_capacity` over the alive ships.
- Embarked fighters: the sum of `FighterEntry::count`. The port keeps
  fighters on the fleet, not on a ship. Embarked regiments come from
  `TroopTransportState` (`troop_transport.rs`, the fleet's regiments aboard).
- Personnel: `Fleet::characters` plus the special forces aboard. port:
  carriage is kept per fleet, not per ship.
- Damaged Ships: port: no flag (`+0x50` bit `0x200`). `hull_current < class
  hull` is the nearest field. That reading is unverified.
- Hyperdrive Rating: port: `FUN_004fd900` is not traced past its walk. The
  nearest field is any alive ship's class `hyperdrive` being non-zero
  (unverified).

## Capital Ship Status (`FUN_00446fd0`)

Families `0x14..0x1b`, vtable `0x0065d650`. The title is TEXTSTRA 34562
"Capital Ship Status". There is no "Attached" row. The rows, in order:

| Label | Value |
|---|---|
| 34819 "Class:" | the class name, a string at class object `+0x34` (`FUN_004486d0`) |
| 34563 "Fleet:" | the name of the container `+0x1c` (`FUN_004f62d0`) |
| 34406 "Status:" | `+0x50` bit `0x10` 34628 "Enroute"; else bit `0x4` 34632 "Awaiting Orders"; else 34647 "Under Construction" |
| 34662 "ETA Destination:" | the shared ETA row |
| 34385 "Maintenance Cost:" | `FUN_004f2990` (class `+0x4c`, CAPSHPSD `maintenance_cost`) |
| 34564 "Capacity:" | empty (heading) |
| 34457 "Fighter Squadrons:" | `+0x26c` → class `+0xe4` (CAPSHPSD `fighter_capacity`) |
| 34565 "Trooper Regiments:" | `+0x270` → class `+0xe8` (`troop_capacity`) |
| 34566 "Embarked:" | empty (heading) |
| 34457 "Fighter Squadrons:" | `+0x23c` (`FUN_00500fe0`): families `0x1c..0x1f` aboard, mode 3 (`FUN_005039d0`) |
| 34565 "Trooper Regiments:" | `+0x240` (`FUN_00501040`): families `0x10..0x13` aboard, mode 3 (`FUN_00504c40`) |
| 34567 "Personnel:" | families `0x30..0x3b` (characters only) aboard, mode 3 (`FUN_004f25c0`, `%ld`) |
| 34568 "Ship Damaged:" | `+0x50` bit `0x200`: 34646 "Yes", else 34645 "No" |
| 34569 "Hyperdrive Rating:" | `+0x1ec` (class `+0x6c`, `hyperdrive`) minus `+0x210` (`FUN_005011f0`: `hyperdrive × +0x64` bits 16..19). If that is 0, use `+0x1f0` (class `+0x70`, `hyperdrive_if_damaged`) minus `+0x214` (bits 20..23) instead. Then ":" and `+0x1ec` (`%ld`) |
| 34576 "Hull Value:" | `+0x248` (class `+0xc0`, `hull`) minus `+0x274` (`FUN_00503100`, the instance's hull damage `+0x60`), ":" `+0x248` |
| 34577 "Damage Control Rating:" | `+0x260` (class `+0xd8`, `damage_control`) minus 0 |
| 34578 "Shield Recharge Rate:" | `+0x268` (class `+0xe0`, `shield_recharge_rate`) minus `+0x294` (`FUN_00500fa0`: `rate × (+0x64 & 0xf) / DAT_006bb388`), ":" `+0x268` |
| 34388 "Maximum Shield Strength:" | `+0x1e0` (class `+0x60`, `shield_strength`), no subtraction |
| 34579 "Tractor Beam Power:" | `+0x24c` (class `+0xc4`, `tractor_beam_power`) minus `+0x278` (`FUN_00500e60`, scaled by `+0x64` bits 8..11 and hull damage), ":" `+0x24c` |
| 34580 "Sub-Light Engine Rating:" | `+0x1e4` (class `+0x64`, `sub_light_engine`) minus `+0x208` (`FUN_005010c0`, scaled by `+0x64` bits 12..15 and hull damage), ":" `+0x1e4` |
| 34390 "Maneuverability:" | `+0x1e8` (class `+0x68`) minus 0 |
| 34391 "Detection Rating:" | `+0x1c4` (class `+0x5c`, `detection`) minus 0 |
| 34581 "Weapon Recharge Rate:" | `+0x264` (class `+0xdc`) minus `+0x290` (`FUN_00500f60`, `+0x64` bits 4..7), ":" `+0x264` |
| 34582 "Bombardment Modifier:" | `+0x25c` (class `+0xd4`) minus `+0x288` (`FUN_00500f00`: `modifier × damage / hull`, or 0 when `+0x24 & 0x30` is clear and `FUN_005006e0` holds) |
| 34583 "Forward Weapons Arc Rating:" | empty (heading) |
| 34584 "Turbo Laser:" | `+0x1fc(arc 0, weapon 0)` minus `+0x220(0, 0)` |
| 34401 " Ion Cannon:" | `+0x1fc(0, 1)` minus `+0x220(0, 1)` |
| 34585 "Laser Cannon:" | `+0x1fc(0, 2)` minus `+0x220(0, 2)` |
| 34592 "Aft Weapons Arc Rating:" | heading; then the three weapons for arc 1 |
| 34593 "Starboard Weapons Arc Rating:" | heading; then the three for arc **3** |
| 34594 "Port Weapons Arc Rating:" | heading; then the three for arc **2** |

`+0x1fc(arc, weapon)` is `FUN_00557490` → `FUN_0053bb80`: class
`+0x74 + 4 × (3 × arc + weapon)`, which is DAT `0x4c..0x78`. Arcs run fore 0,
aft 1, port 2, starboard 3, and weapons run turbolaser 0, ion 1, laser 2.
The order of the pushes was read from the disassembly at `0x447f2c..0x448650`,
because the decompiler drops the arguments. The window lists Starboard
before Port. `+0x220` (`FUN_00501270`) scales the arc value by `+0x64`
bits 4..7 and hull damage. Every weapon value is the current one only, with
no ":" maximum.

### Port

- Class: `CapitalShipClass::name`. Fleet: the fleet holding the
  `ShipInstance`. A ship's own name (`ShipInstance::name`) is the window's
  name line (`FUN_0044a210`), not a row.
- Status and ETA: as for the fleet. A port ship is never under construction.
- Maintenance, the capacities, maximum shield, damage control, maneuverability,
  detection, and every arc maximum: `CapitalShipClass` fields of the same
  names.
- Embarked: port: no per-ship field, because fighters, regiments and
  characters are kept per fleet.
- Ship Damaged: port: no flag.
- Hull: `ShipInstance::hull_current` stands for the value after damage
  (class `hull` minus `+0x60`). The maximum is `CapitalShipClass::hull`.
- The damage terms read `+0x64` nibbles. The port keeps only bits 0..7, in
  `ShipInstance::shield_weapon_packed`, and treats them as recharge
  allocation. Bits 8..23 (tractor, sub-light, hyperdrive, backup
  hyperdrive) have no port field, so those rows show the class value. The
  global `DAT_006bb388` is untraced.

## Fighter Squadron Status (`FUN_004444c0`)

Families `0x1c..0x1f`, vtable `0x0065def0`. The title is TEXTSTRA 34384
"Fighter Squadron Status". There is **no "Status:" row**. The rows, in
order:

| Label | Value |
|---|---|
| "Attached: " | `FUN_0044a2e0` (the Character Status rule): the container |
| 34662 "ETA Destination:" | the shared ETA row |
| 34385 "Maintenance Cost:" | `FUN_004f2990` (class `+0x4c`) |
| 34386 "Squadron Size:" | `+0x244` (class `+0xc8`, FIGHTSD `squadron_size`) minus `+0x254` (`FUN_00503100`, losses `+0x60`), ":" `+0x244` |
| 34387 "Hyperdrive Rating:" | `+0x1ec` (class `+0x6c`, `hyperdrive`) minus 0 (`%ld`) |
| 34388 "Maximum Shield Strength:" | `+0x1e0` (`FUN_00503160`: class `+0x60` `shield_strength` × size) minus `+0x204` (`shield × losses`), ":" `+0x1e0` |
| 34389 "Sub-Light Engine Rating:" | `+0x1e4` (class `+0x64`) minus 0 |
| 34390 "Maneuverability:" | `+0x1e8` (class `+0x68`) minus 0 (`%ld`) |
| 34391 "Detection Rating:" | `+0x1c4` (class `+0x5c`, `detection`) minus 0 |
| 34392 "Bombardment Value:" | `+0x248` (class `+0xcc`, DAT `0xa4`, dumper name `bombardment_defense`) minus `+0x258` (`FUN_00503110`: `value × losses / size`), ":" `+0x248` |
| 34393 "Weapons Rating:" | empty (heading) |
| 34400 " Laser Rating:" | `+0x1f8(2)` (`FUN_005031b0` → `FUN_0053bbd0`: class `+0xb8`, `laser_cannon_attack_strength`, × size) minus `+0x21c(2)` (× losses), ":" `+0x1f8(2)` |
| 34401 " Ion Cannon:" | the same with index 1 (class `+0xb4`, `ion_cannon_attack_strength`) |
| 34402 " Torpedoes:" | `+0x23c` (`FUN_005030c0`: class `+0xc0` `torpedoes` × size) minus `+0x24c` (× losses), ":" `+0x23c` |

### Port

- The class values: `FighterClass` fields of the same names (`squadron_size`,
  `hyperdrive`, `shield_strength`, `sub_light_engine`, `maneuverability`,
  `detection`, `bombardment_defense`, `laser_cannon_attack_strength`,
  `ion_cannon_attack_strength`, `torpedoes`). Maintenance:
  `FighterClass::maintenance_cost`.
- Losses (`+0x60`): port: no field. `FighterEntry` keeps a count of whole
  squadrons, so every "current" equals its maximum.
- Attached: the fleet holding the `FighterEntry`. A port squadron is not a
  separate object, so the window has to open on an entry (class within a
  fleet).

## Trooper Regiment Status (`FUN_00445280`)

Families `0x10..0x13`, vtable `0x0065e438`. The title is TEXTSTRA 34405
"Trooper Regiment Status". The rows, in order:

| Label | Value |
|---|---|
| "Attached: " | `FUN_0044a2e0`: the container |
| 34406 "Status:" | `+0x50` bit `0x10` 34628 "Enroute"; else bit `0x4` 34632 "Awaiting Orders"; else 34673 "Training" |
| 34662 "ETA Destination:" | the shared ETA row |
| 34385 "Maintenance Cost:" | `FUN_004f2990` (class `+0x4c`) |
| 34407 "Attack Strength:" | `+0x1dc` (class `+0x64`, TROOPSD `attack_strength`) minus `+0x1e8` (0) |
| 34408 "Defense Strength:" | `+0x1e0` (class `+0x68`, `defense_strength`) minus `+0x1ec` (0) |
| 34392 "Bombardment Value:" | `+0x1d8` (class `+0x60`, DAT `0x38`, dumper name `bombardment_defense`) minus `+0x1e4` (0) |
| 34417 "Detection Value:" | `+0x1c4` (class `+0x5c`, `detection`) minus `+0x1cc` (0) |

The regiment's current strength `+0x96` is not shown.

### Port

- Attached: the system whose ground units hold the regiment, or the fleet
  carrying it (`TroopTransportState`).
- Status: Enroute while `TroopTransportState` reports it in transit, else
  "Awaiting Orders". A port regiment exists only once trained, so
  "Training" never shows.
- Maintenance: `buildable_classes[class_dat_id].maintenance_cost`.
- Attack, defense, detection: `troop_classes[class_dat_id]` (`TroopClassDef`).
- Bombardment Value: port: no runtime field. TROOPSD `bombardment_defense`
  is parsed by the dumper, but `TroopClassDef` does not keep it.

## Spec Forces Status (`FUN_00445780`)

Families `0x3c..0x3f`, vtable `0x0065e160`. The title is TEXTSTRA 34416
"Spec Forces Status". The rows, in order:

| Label | Value |
|---|---|
| "Attached: " | `FUN_0044a2e0`: the container |
| 34406 "Status:" | `+0x50` bit `0x10` 34628 "Enroute"; else the mission key `+0x68` not the empty id (`FUN_004ece30`, 2) 34629 "On Mission"; else bit `0x4` 34632 "Awaiting Orders"; else 34647 "Under Construction" |
| 34662 "ETA Destination:" | the shared ETA row |
| 34385 "Maintenance Cost:" | `FUN_004f2990` (class `+0x4c`) |
| 34418 "Diplomacy Rating:" | slot `+0x1dc` (`FUN_00503c30`, the base short; `mission-lifecycle.md` "Member skills") |
| 34419 "Espionage Rating:" | slot `+0x1e0` |
| 34420 "Combat Rating:" | slot `+0x1f0` |
| 34421 "Leadership Rating:" | slot `+0x1f4` |

The labels are 34418..34421, not the Character Status strings 34598..34601.

### Port

- Attached: the system or fleet listing the key (`System::special_forces`,
  or a fleet carrying it).
- Status: the unit's mission flag (`SpecialForceUnit`, "currently a mission
  member") means "On Mission". A port special force exists only once built,
  so "Under Construction" never shows. En route follows its mission transit,
  as for characters.
- Maintenance: `buildable_classes[class_dat_id].maintenance_cost`.
- Ratings: `SpecialForceUnit::skills` in `Skill` order (diplomacy 0,
  espionage 1, combat 5, leadership 6).

## Port notes

- `status_window.rs` draws the window and `status_rows.rs` fills it for
  characters, special forces, fleets, capital ships, fighter squadrons,
  regiments, defense, manufacturing and production facilities, missions
  and production managers. The object menu enables Status for any single
  object that is not a system (`FUN_0051d990`); System Status is not wired,
  since no shipped path opens it.
- port: a facility's background follows its own side, which equals its
  system's holder except in a contested system, where it keeps the last
  holder (`facility-ownership.md`). A production manager's follows the
  system's holder.
- port: the picture's marks (the regiment's 11514, en route, damage and
  unbuilt) are not drawn.
- port: each family's own Port section lists the rows the port cannot hold
  (command ranks, damage nibbles, fighter losses, Under Construction and
  Training states, facility Enroute).
- port: the window is drawn above the modeless windows and takes the pointer
  only over itself, as the move confirmation does.
- port: Commanding is always "None": the port keeps no command assignment
  (`+0xa8`).
- port: "Injured" never shows: the port keeps no injury (`+0x94`).
- port: a character is en route while its mission transit lasts
  (`MissionState::en_route`). Its ETA is that transit's arrival day, and
  Attached names the transit's destination.
- port: the Force value is the character's `jedi_level.base`; the port keeps
  no other per-character Force value.
- port: the ratings are each skill's `base`.
- port: the list scrolls with the mouse wheel and draws no scroll bar.
- port: the Encyclopedia button opens the Encyclopedia as the object menu's
  Encyclopedia does, on its index.
- port: Escape closes the window. No key slot is traced.
- The Personnel Finder's Display follows `FUN_00429440`: a character on a
  visible mission opens its system's Missions window with the mission's row
  and the member's tab selected (slot 27, `FUN_004a1e10`).
