---
title: "Orbital Bombardment"
description: "Bombardment orders 0x220-0x223: the order and its two command lists, the gate FUN_00582440, the fleet bombard bit, message 0x224 in the scheduler, the run FUN_0058d800/FUN_0058d770 (Death Star phase and character roll, KDY/LNR defence fire, strength and target values from the DAT, per-hit rolls), the support shifts in FUN_0058e320, the messages event 0x182 files, and the port"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-17
---

# Orbital Bombardment

Answers task-graph node `or-q-bombardment-flz`. Every claim below was read in
the `.c` files cited unless marked `hyp:`.

## The orders

STRATEGY RCDATA names (`ai-order-emission.md`): `0x220` Target Military
Facilities, `0x221` Target Civilian Facilities, `0x222` General Bombardment,
`0x223` Destroy System.

The four share one gate, `FUN_00582440`, called from the order's enable
slot `FUN_00581e80` (order vtables `0x00669f1c` 0x221, `0x00669f74` 0x223,
`0x00669fcc` 0x222, `0x0066a024` 0x220, over the base `0x00669ec4`):

- The team is the order's team ids expanded into objects by `FUN_0053f150`
  (class handler slot `+0x2c`, `FUN_0053efa0`), the same expansion every
  fleet order uses (`move-order.md`); it is the ordered group, not every
  fleet of the side. A team entry that is not a fleet (family `0x08`-`0x0f`)
  is skipped.
- One passing fleet is enough: the loop over the team ends at the first fleet
  that passes. A fleet that is not usable (`+0x50` bit 0 clear,
  `FUN_00506480`) is skipped. A fleet passes when the sum of slot `+0x1dc`
  over its usable ships and each ship's usable squadrons is nonzero
  (`FUN_004fd6b0`), and for Destroy System also when it holds a usable ship
  of family `0x18`-`0x1b` (`FUN_005003d0`, `FUN_005131b0`). A fleet with
  strength but no Death Star does not stop the loop; a later fleet may pass.
- No fleet passing gives status `8`/`6` (the last examined fleet had zero
  strength, or none was usable); Destroy System additionally overwrites it
  with `8`/`8` when the last examined fleet had no Death Star ship, including
  when no fleet was examined.
- No side, shield, ion or charge test is made. Before the gate the enable
  slot runs the order base's enable `FUN_0051fe20`: an empty team gives
  `1`/`0x16`, and each fleet command's `+0x18` (`FUN_00553770` →
  `FUN_0057fe90` → fleet slot `+0x1c4`, `FUN_004fdf70` → `FUN_004ffa70`)
  refuses a fleet of another side or en route (`FUN_004f9860`) or one with no
  ship (`8`/`1`); `FUN_00553770` then requires the commands' systems to have
  one holder, else `1`/`0x14`.

## The order, its commands and the day

Traced in `FUN_005820a0` (order Execute), `FUN_00520040`, `FUN_005201c0`,
`FUN_00582200`, `FUN_005535b0`, `FUN_005187f0`, `FUN_005539f0`.

- Execute (`FUN_005820a0`) runs the generic execute `FUN_00520040`, which
  submits the commands in the order's list `+0x40` through `FUN_005539f0`,
  then submits the commands in list `+0x44`.
- List `+0x40` (`FUN_005201c0`, factory slot `+0x48` = `FUN_00582190`): one
  fleet command per team fleet, command class `0x00669b20`, kind `0x220`
  (`FUN_005829f0`). Its execute `FUN_00580030` resolves the fleet and calls
  fleet slot `+0x1d4` = `FUN_004fec00`: fleet slot `+0x1c8` (`FUN_004fe020`
  → `+0x1c4` `FUN_004fdf70` → `FUN_004ffa70`: a ship that is not completed
  (`+0x50` bit 2 clear) gives `1`/`0x10`, an en-route ship `1`/`0x11`), and
  when the status is still ok it sets fleet `+0x58` bit 6 (`FUN_004fe460(1)`).
  A refused fleet is left unflagged.
- List `+0x44` (`FUN_00582200`, `FUN_005187f0`): one message-`0x224` command
  (class `0x00663300`) per distinct system that holds the team's fleets
  (the fleet's container `+0x1c`, family `0x90`-`0x97`); a destroyed fleet
  gives `1`/`0x12`. Each command holds the system at `+0x3c` and the order
  data at `+0x40` (kind at `+0xc`).
- Submission is `FUN_004fd370` → `DAT_006b2b00` → `FUN_004fcff0` →
  `FUN_0051d8d0` → `FUN_0054ee30` → `FUN_005f61c0`: an append to the
  scheduler's command queue `+0xac`, so the fleet commands precede the
  `0x224` commands. `FUN_0051df30` serves queue `+0xb0`, then the due timers
  `+0xa8`, then `+0xac`; phase 5 of `FUN_0041dff0` repeats until nothing is
  due, and the day advances only in phase 0 of a later step
  (`timer-scheduler.md`). The `0x224` command therefore runs in the step that
  issued the order, before the next day tick. `hyp:` the head of queue `+0xac`
  is always due (its due slot `+0x1c` is not read).
- The command's slots 6 and 7 call system slots `+0x1c4` and `+0x1c8`
  (`FUN_00563620`, `FUN_005636f0` → `FUN_00507f10`, `FUN_00507fc0`): the only
  refusal is a destroyed system (`+0x50` bit 3, status `1`/`0x22`), for any
  holder and any side, including the bombarder's own system. Slot 8,
  `FUN_005637c0`, runs the bombardment.
- Message `0x224` commands are built only by the four order classes
  (`FUN_00582200`) and the id-to-factory registry (`FUN_0051ef80` →
  `FUN_00563580`); `FUN_0058d800` has one caller, `FUN_005637c0`. The
  tactical engine and the post-battle code reach none of them, so the
  original starts a bombardment only from an order, never after a won
  battle.

### Fleet `+0x58` bit 6 and the system bit

| Event | Effect |
|---|---|
| fleet slot `+0x1d4` (`FUN_004fec00`) accepts the fleet | sets fleet bit 6 |
| run reached `FUN_0058d990` | sets system `+0x88` bit 6 (`FUN_0050a660(1)`) |
| finalize (`FUN_0058e320`) | clears the system bit and bit 6 of every fleet of the bombarder's side at the system (`FUN_004ffef0` mode 4, `FUN_004fe460(0)`) |
| two flagged sides at the system | `FUN_0058d800` returns 0 before any phase: no system bit, no finalize, the fleet bits stay set |
| fleet arrival slot `+0x11c` (`FUN_004ff600`, `param_1` nonzero) | clears fleet `+0x58` bits 4, 7 and 6 (`FUN_004fe380`, `FUN_004fe4d0`, `FUN_004fe460`) |
| flagged fleet in transit | bit stays set; the run reads fleets at the system, so it is absent until arrival |
| second order in the same step | the flag is idempotent; the first `0x224` run's finalize clears the side's bits, so a later `0x224` for that system finds no flagged fleet and returns 0 |

`hyp:` nothing limits how often an order can be issued per day; no per-day
counter was found in the order, command or run.

## Setup (`FUN_0058d800`)

| Order | `+0x34` military | `+0x38` civilian | `+0x3c` energy, raw material | `+0x40` collateral | `+0x44` Destroy System |
|---|---|---|---|---|---|
| `0x220` | 1 | | | 1 | |
| `0x221` | | 1 | | | |
| `0x222` | 1 | 1 | 1 | | |
| `0x223` | 1 | 1 | 1 | | 1 |

The bombarding side is the owner shared by every usable fleet at the system
(`FUN_004ffe70` mode 1, `FUN_0058d800.c:55`) with `+0x58` bit 6 set,
whichever order flagged it. If two sides have such fleets, `FUN_0058d800`
returns 0 and nothing else runs. The state keeps the system at `+0x10`, the
bombarder at `+0x14` and the holder (system `+0x24` bits 6-7) at `+0x18`, then runs
`FUN_0058d770`: ship phase, defence phase, effect phase, finalize. A
stalemate leaves bits 6-7 at the earlier holder (`FUN_0050b610`), so a
stalemated system is bombarded as that side's.

## Targets (`FUN_0058cba0`)

Each iterator takes only the holder's objects.

- Military: regiments `0x10`-`0x13` (`FUN_00504cc0`), facilities
  `0x22`-`0x27` (`FUN_00527050`) and the Alliance HQ family `0x20`-`0x21`
  (`FUN_00526d70`).
- Civilian: facilities `0x28`-`0x2f` (`FUN_0053b760`).
- Energy and raw material: system `+0x5c` and `+0x64`, one target each while
  nonzero.

## Ship phase (`FUN_0058d990`)

- Sets the system's bombard bit (`FUN_0050a660(1)`).
- With `+0x44` set, `FUN_0058d440` looks for a ship of family `0x18`-`0x1b`
  among the usable ships of the usable flagged fleets (`FUN_0058ca70.c:26-34`,
  mode 1 for both walks), the Death Star (CAPSHPSD family `0x18`, TEXTSTRA
  10120), as the gate does (`FUN_00582440.c:70`, `FUN_005003d0(.., 1)`): an
  unfinished Death Star, or one on its own leg, is not found. No shield or
  holder test is made.
- When found, `FUN_0058ce70` destroys the system through slot `+0xac`, cause
  `0x10` when the system is the bombarder's own and `0xf` otherwise; the state
  sets `+0x28`, `+0x2c` and `+0x30`.
- Character roll (traced, `FUN_0058d990.c`, `FUN_0056f3d0.c`,
  `FUN_005131d0.c`, `FUN_004f6b90.c`, `FUN_0055e9e0.c`, `FUN_004ef6d0.c`):
  `FUN_0056f3d0(system, 3)` walks the system container's own children
  (`FUN_00539f70`, so characters not aboard a fleet) of family `0x38`-`0x3b`
  (the minor characters; the major characters `0x30`-`0x35` and special
  forces `0x3c` are outside the range) with no side filter, and mode 3 keeps
  only objects with `+0x50` bit 6 set (`FUN_004f6b90`). Each one with
  `+0x50` bit 5 (in transit) clear is rolled once with probability GNPRTB
  `0x606` (50) (`FUN_0055e9e0`). A hit runs slot `+0x2f0(1, 10)` (`FUN_004ef6d0`:
  injury +1, cause 10), then slot `+0xac(10)` when `+0x50` bit 6 is set. Bit 6
  already holds for every walked object, so a hit always destroys the
  character; a character in transit is skipped.
- Otherwise it counts the bombarding ships (`+0x1c`), from the same mode-1
  walk, and the targets (`+0x20`): an unfinished ship, or one on its own
  leg, neither bombards nor draws battery fire.

## Defence phase (`FUN_0058dc10`)

- Each KDY-150 (family `0x22`, `FUN_00527550`) fires once, then each LNR
  Series I/II (`0x23`, `FUN_005272e0`), at a random bombarding ship. Neither
  iterator takes a side: every battery at the system fires, whoever owns it.
- Damage: DEFFACSD attack `+0x5c` (`FUN_00520b70`; KDY 2000, LNR 800/5000)
  × (leadership `+0x1f4` of the character at the system with `+0x96 == 3` /
  GNPRTB `0x600` (40) + 1) (`FUN_00509330`, `FUN_0055e930`). Traced:
  `+0x96` is the CommandKind, 3 the General (`character-roster.md`, setter
  `FUN_004ee350`, orders `0x260`-`0x263`); `FUN_00509330` takes the first such
  character that is a child of the system with the system's own side
  (`FUN_004f2640` with `+0x24` bits 6-7), so the holder's General defends;
  none gives leadership 0.
- KDY damage goes to the ship's first accumulator (`FUN_00501ba0`: ship
  record `+0x54` field `+0x20`, added to the old value), LNR damage to the
  second (`FUN_00501bc0`: field `+0x24`). The ship record is the 0x2c-byte block
  of `FUN_00502c50` → `FUN_005586a0`, whose fields `+0x20`, `+0x24` and `+0x28`
  start at 0. Once both loops finish, `FUN_0058d540` (slot `+4` of the
  iterator vtable `0x0066a940`, `FUN_0058d4f0`) runs once on every bombarding
  ship. Read in the disassembly: with X the first accumulator, Y the second,
  W the remaining shields (`+0x1e0` − `+0x204`) and Z the remaining hull
  (`+0x248` − `+0x274`), it computes `FUN_0055e9a0(X, W)` (leftover X, reduced
  W), `FUN_0055e9a0(Y, W)` (leftover Y, reduced W) and `FUN_0055e9a0(Y, Z)`
  (leftover Y, reduced Z). KDY overflow past the shields stays in X and never
  reaches the hull; only LNR damage left after the shields lowers Z, applied
  through `FUN_00501ae0(hull loss, 0xf, …)`, which destroys the ship when the
  hull reaches its maximum. X and Y are written back as the leftovers
  (`FUN_00501ba0`, `FUN_00501bc0`) and the shield loss to field `+0x28`
  (`FUN_00501be0`); no code applies that loss to the ship's shield counter.
- Remaining shields W. `+0x1e0` is the thunk `0x00557750` → `FUN_00520b80`, the
  class record's `+0x60` (DAT `0x38`, CAPSHPSD `shield_strength`). `+0x204` is
  `FUN_00501160` (vtable `0x0065d650`, index 129). With S = `+0x1e0`, G = GNPRTB
  `0x800` (`FUN_00500670`, `DAT_006bb388`, 4), c = the ship's `+0x64` bits 0-3 (the
  shield damage counter), H = slot `+0x248` (the class's maximum hull) and d = slot
  `+0x274` (hull damage), it computes, when G and H are nonzero,
  `e1 = S × (G − c) / G` and `e2 = e1 × (H − d) / H` (integer divisions,
  `FUN_0053e170`), else `e2 = S`, and returns `S − e2`. So W = `+0x1e0 − +0x204` =
  e2: the shield strength scaled by the shield counter and by the remaining hull
  fraction, derived on every read and not stored. The ship record's `+0x28`
  field (`FUN_00501be0`) takes this run's W before minus W after and finalize
  resets it; nothing reads it. `FUN_0058d540` (`0x0058d56e`, `0x0058d612`) is the
  only caller of a ship's slot `+0x204` (instruction scan of `+ 0x204]` over the
  image). The counter and the hull recover through repair
  (`ship-damage-repair.md`, "Repair").
- Finalize resets all three fields to 0 on every bombarding ship (iterator
  `FUN_0058d680`, vtable `0x0066a948` slot `+4` at `0x0058d6d0`, which calls
  `FUN_00501ba0`, `FUN_00501bc0` and `FUN_00501be0` with 0).
- `+0x30` is set when no bombarding ship remains.

## Effect phase (`FUN_0058df60`)

1. Hits: for each bombarding fleet, the slot-`+0x1dc` sum over its usable
   ships and each one's usable squadrons (`FUN_004fd6b0.c:23,32`, mode 1
   for both walks) × (leadership (slot `+0x1f4`) of its role-2 character
   (`FUN_004fd790(fleet, 2)`) /
   GNPRTB `0x601` (40) + 1) (`FUN_0055e950`), summed. Subtract the holder's
   shield generators: family `0x24` only, DEFFACSD `+0x60` (`FUN_00527940`,
   `FUN_00520b80`). The Death Star shield `0x25` is not counted. Traced:
   `FUN_004fd790(fleet, 2)` walks the fleet's existing ships of the fleet's
   side (mode 3) and takes the first existing child character of the ship's
   side with `+0x96 == 2` (`FUN_005006f0`, mode 3), the Admiral (Commander
   1, Admiral 2, General 3, `character-roster.md`); none gives leadership 0.
2. Collateral (`+0x40`, order `0x220` only, with hits left): when
   `FUN_0053e290(+0x20 − 1 + +0x24) < +0x24` (`+0x24` = 1, so always when
   the holder has no military target), one civilian facility gets a single
   roll. `FUN_0058d080` draws an index over the holder's civilian facilities
   but its walk counts up to it without advancing (no `FUN_005130d0` in
   `0x0058d0f8`-`0x0058d116`), so the pick is always the first.
3. Each hit picks a random target index (`FUN_0053e290(+0x20 − 1)`), takes
   its value v (`FUN_0058cf80`) and succeeds when v < `FUN_0053e2c0`(GNPRTB
   `0x602`, `0x603`), a draw from 1 to 10 (`FUN_0055e970`), so with chance
   (10 − v)/10. Values: facilities and the HQ slot `+0x1c0` − `+0x1c4`,
   regiments `+0x1d8` − `+0x1e4`, energy GNPRTB `0x604` (9), raw material
   `0x605` (9) (the DAT sources are in "Strength and target values").
4. A success (`FUN_0058d020`) destroys the object through `FUN_0058ce70`
   (cause `0xf`, or `0x10` for the bombarder's own) or lowers energy or raw
   material by 1 (`FUN_0058cf20`, `FUN_0058cf50`). The targets are recounted;
   none left sets `+0x30`.

`+0x28` is set by destroying a `0x28`-`0x2f` facility or by any energy or raw
material loss.

## Finalize (`FUN_0058e320`)

- With `+0x28`, the system's support for the bombarding side changes by
  GNPRTB 7705 (−20) (`FUN_0055a0e0`, `FUN_0050c9f0`).
- With `+0x2c`, a galaxy-wide shift (`FUN_0055da90`, `FUN_00518a70`); with
  `+0x28` and not `+0x2c`, a sector-wide shift (`FUN_0055d900`, SDPRTB
  `0x1408`-`0x140b`, `FUN_00518ac0`).
- Clears the system's bombard bit and each bombarding fleet's bit 6
  (`FUN_004fe460`).

## Strength and target values

Traced from the unit vtables of `status-window.md` (fleet `0x0065d438`,
capital ship `0x0065d650`, fighter `0x0065def0`, regiment `0x0065e438`) and
the facility vtables of `facility-order-lists.md`. A class record is the DAT
record at object `+0x2c`; class `+off` is DAT `off − 0x28`. The dat-dumper
types (`tools/dat-dumper/src/types/*.rs`) name the fields.

| Role | Slot | Reads | DAT field (port loader) | Base values |
|---|---|---|---|---|
| Ship bombard strength | `+0x1dc` = `+0x25c` − `+0x288` (`FUN_00501470`) | class `+0xd4`; minus `FUN_00500f00` = modifier × hull damage (`+0x274`) / max hull (`+0x248`) (`FUN_0053e170`), or the whole modifier when `+0x24 & 0x30` is clear (a master object) and the ship's first accumulator `+0x20` is nonzero | CAPSHPSD `bombardment_modifier` (DAT `0xac`; `lib.rs` ~320) | 0-5 on the 29 ordinary capital ships, 15 on the Death Star (record 136) |
| Squadron bombard strength | `+0x1dc` = `+0x248` − `+0x258` (`FUN_005032a0`) | class `+0xcc`; minus value × squadron losses / size (`FUN_00503110`) | FIGHTSD `bombardment_defense` (DAT `0xa4`; `lib.rs` ~357) | 0, 2, 1, 2, 0, 0, 2, 1 for records 1-8 |
| Facility value (defence 0x22-0x25, yards, training, mines, refineries, the HQ) | `+0x1c0` = the thunk at `0x00558050` (`JMP FUN_00520b60`) (class `+0x58`), minus `+0x1c4` = 0 | | DEFFACSD, MANFACSD, PROFACSD `bombardment_defense` (DAT `0x30`; `lib.rs` ~496, ~575, ~615) | KDY 5, LNR I 4, LNR II 3, GenCore I 2, GenCore II 3, Death Star shield 1; yards 2/3/3 (ids 1-3), 2/4/4 (4-6); mine 5, refinery 2 |
| HQ value | the same slot on vtable `0x0065f280` | class `+0x58` | ALLFACSD seventh extra field `extra7` (DAT `0x30`, the last field of record `0x20000001`, the file's only record: the last four bytes of `ALLFACSD.DAT` read 9; `rebellion-data` loads it as the class's bombardment value) | 9 |
| Regiment value | `+0x1d8` = the thunk at `0x00557750` (`JMP FUN_00520b80`) (class `+0x60`), minus `+0x1e4` | | TROOPSD `bombardment_defense` (DAT `0x38`; `lib.rs` ~536) | 5, 5, 2, 4, 9, 6, 2, 5, 2, 6 for records 1-10 |

A target with value v survives a hit unless v < the 1-10 draw, so a higher
DAT value protects (the HQ 1 in 10, a KDY 5 in 10, a regiment of value 9 1
in 10). Squadrons contribute their FIGHTSD `bombardment_defense` to the
strength sum despite the field name. In the effect phase a master ship whose
first accumulator is nonzero contributes 0 strength: the accumulator holds KDY
damage that exceeded the ship's shields (defence phase), and finalize clears it.

### Ion damage and the manual

`hyp:` the manual's p. 122 "XDY-150 ion cannon" is the KDY-150 (DEFFACSD
record 1, TEXTSTRA 8704, family `0x22`), the only defence facility whose fire
reaches slot `+0x1dc`. The gate (`FUN_00582440`) and the strength sum read no
ion, shield, weapon or other damage counter of the ship (`+0x64` bits 0-23).
KDY fire is the mechanism that "robs energy": damage past the shields leaves
the first accumulator nonzero, the ship's strength is 0 for that run
(`FUN_00500f00`), and it is not destroyed. The hull-damage term of
`FUN_00500f00` is the only other damage input.

## Both sides

Nothing in the gate, the command or the run branches by side except the
own/enemy cause. Either side can issue an order, and the system's holder can
be the bombarder: no check compares the holder with the bombarder, only the
destroyed-system refusal (`FUN_00507f10`) applies, so a side can bombard its
own system and the target iterators then take its own objects. The enable
slot tests the fleet's side, never the system's holder (`FUN_0051fe20`'s fleet
commands, above), so the menu enables the orders over an own system as over
any other. Only the Empire can Destroy System, because only CAPSHPSD family
`0x18` qualifies (`hyp:` from the data).

## Messages

`FUN_004fe460` writes fleet `+0x58` bit 6 and copies it into each side's view
of the fleet (`FUN_004ff450`, sides 1 and 2). A view whose bit changes calls
fleet slot `+0x208` (vtable `0x0065d438`, `FUN_004ff840`), which raises event
`0x182` (FleetBombard) with the new bit. The message classes take it through
that side's view (`FUN_004f36a0`), so a side is told only when its view holds
the fleet:

- Operation Reports `0xf` (`FUN_00496460`, visitor `FUN_00496770`): title
  TEXTSTRA `0x7098`, body `0x7099` (bombarder, holder, system) or `0x709a` for
  a neutral system, picture `0x2b9a`, `0x2b9b` when the report's destroyed-at-system list
  `+0x80` is non-empty, `0x2b9c` when the destroyed-with-fleet list `+0x88` is
  non-empty (`FUN_00496770` code 2; the lists are filled by event `0x302`
  through slot 7 `FUN_00497380`, `operation-report-window.md`); advice code
  `0x2e` when the viewing side is not the fleet's owner (`+0x6c`, the fleet's
  `+0x24` bits 6-7). The report window opens as a popup only for the bombarder
  (message side == fleet owner, class 7, `FUN_0048a060`); the target side's
  report goes to the Message Index with advice `0x2e`
  (`operation-report-window.md`).
- Planet Destroyed `0x15` (`FUN_00492470`, visitor `FUN_004950d0`, which
  walks the fleet's family `0x18`-`0x1b` ships, `FUN_005003d0`): title
  `0x70c8`, body `0x70c9` or `0x70ca`, movie `0x6f` (`movies.md`).

The HQ's removal files Rebel HQ Destroyed `0x1f` through event `0x128`
(`FUN_0048e5a0`, `movies.md`).

## The Alliance HQ

The HQ (family `0x20`-`0x21`) is a military target, so Target Military
Facilities, General Bombardment and Destroy System's hit loop can destroy it
with cause `0xf`. Destroy System also destroys every completed object inside
the system with the system's own cause, the HQ included (`FUN_004fb520` →
child slot `+0xb4`, `game-end-path.md`). Its value is ALLFACSD `extra7` = 9
(traced above), so a hit that picks it destroys it with chance 1/10.

The value enters one place: the per-hit roll of the effect phase
(`FUN_0058cf80` reads slot `+0x1c0` = the thunk `0x00558050` → `FUN_00520b60`
→ class `+0x58`, minus slot `+0x1c4` = 0; `FUN_0055e970(9)` succeeds only when
the 1-10 draw exceeds 9). Destroy System's cascade never rolls it: the HQ
dies there through `FUN_004f8710`, unless it is en route (`game-end-path.md`,
"An HQ in transit").

## In the original (VM)

From `empire-day128-assault-fails-selonia.sav` (Fleet 1, an Imperial Star
Destroyer and a Carrack, in orbit of Alliance-held Selonia):

- The fleet's object menu lists Planetary Bombardment with Target Military
  Facilities, Target Civilian Facilities and General Bombardment enabled and
  Destroy System listed but disabled (no family-`0x18` ship).
- General Bombardment resolves at once: the "Orbital bombardment of Selonia"
  alert ("Imperial ships have conducted an orbital strike on the Alliance
  system of Selonia") opens within a second of the order, on day 137 to 138.
- Nothing died. Selonia holds one family-`0x24` shield generator, two Alliance
  regiments, civilian facilities, energy 12 and raw materials 5; a save on
  day 140 holds the same objects and amounts; with nothing destroyed and no
  resource lost the finalize sets no `+0x28`, so the run changes no support
  (`FUN_0058e320`). The shield's DEFFACSD defence met the fleet's hits.

From `alliance-day1169-deathstar-trench-run.sav` (an Alliance fleet of one
capital ship and six regiments in orbit of Imperial-held Ord Pardon, day
1170): the fleet's menu shows Planetary Bombardment disabled and Planetary
Assault enabled, the gate's refusal for a fleet whose bombard strength sums
to 0. The Empire runs at Yaga Minor are in `death-star-strategic.md`.

From `alliance-day927-orto-orbit.sav` (Fleet 1, Corellian Corvette 1, Medium
Transport 1 and two Alliance Fleet Regiments, blockading Imperial-held Orto:
three GenCore Level I shields, KDY and LNR Series I batteries, an Orbital
Shipyard, a Mine, Stormtrooper and Dark Trooper regiments):

- Target Military Facilities, ordered while paused, opens the "Orbital
  bombardment of Orto" alert at once ("Alliance ships have conducted an
  orbital strike on the Imperial system of Orto"), on the same day.
- The defence fire destroyed Corellian Corvette 1; Medium Transport 1 and both
  regiments survived. Orto lost nothing: every tab of its System Forces view
  reads "No Casualties".

## Manual cross-check

Read depth: the gate, the enable slot, setup, phases and finalize
`read-in-full`. Port ratings as of this note.

| Claim | Status | Port |
|---|---|---|
| p. 122: the submenu is available "in orbit around an enemy or neutral system" | Contradicted in part: the enable slot and the gate refuse only a destroyed system and mixed holders, and enable the orders over an own system too | Ported (`order_enabled`) |
| p. 57: bombard "an enemy (rather than a neutral) system" | Contradicted: neutral systems are legal targets | Ported |
| p. 122: batteries fire back; the XDY-150 "robs energy… preventing it from being able to bombard" | Confirmed for KDY-150 and LNR fire (`FUN_0058dc10`); KDY overflow zeroes a ship's strength (`FUN_0058d540`), `hyp:` the manual's XDY-150 is the KDY-150 | Ported |
| p. 122: combined Bombardment Modifiers against the shield's strength; what passes hits troops and facilities | Confirmed (`FUN_0058df60`; VM Selonia) | Ported |
| p. 122: Target Military "may cause collateral damage" | Confirmed (0x220's collateral roll) | Ported |
| p. 122: Target Civilian "hurts loyalty"; p. 90: civilian damage, collateral included, costs support | Confirmed (`FUN_0058e320`: GNPRTB 7705 and the sector shift) | Ported |
| p. 122: Destroy System only for the Empire with the Death Star in the fleet | Confirmed through the data (CAPSHPSD 136 is the only family-0x18 class; the gate tests the family, not the side) | Ported |
| p. 90: destroying a planet decreases support across the galaxy | Confirmed for core systems; rim systems still carrying setup bit 3 do not move (VM Yaga Minor, `support-shifts.md`) | Ported |
| p. 124: "a system destroyed is a system unavailable for conquest" | Confirmed (`FUN_005073d0` refuses landings, `death-star-strategic.md`) | Ported for regiment landings (`troop_transport.rs`) |
| p. 136: the Alliance HQ dies to bombardment plus assault or to the Death Star, and cannot be retaken | Confirmed: the HQ is a military target and a cascade child (`game-end-path.md`) | Ported (`support_shift::remove_headquarters`) |

## Port

- `crates/rebellion-core/src/bombardment.rs`: `gate` (`FUN_00582440`, with
  `FUN_00553770`'s mixed holders and the `0x224` command's destroyed
  refusal), `order_enabled` (the enable slot `FUN_00581e80`: the empty team,
  the fleet commands, the gate), `flag_fleet` (`FUN_004fec00`;
  `Fleet::bombarding`, saved; fleet arrival clears it), `open`
  (`FUN_0058d800`'s bombarder and the ship phase's Death Star test),
  `bombard` (`FUN_0058dc10`, `FUN_0058d540`, `FUN_0058df60`),
  `roll_characters`, `finalize` (`FUN_0058e320`) and `clear_flags`.
  `CapitalShipClass::family` is the CAPSHPSD family; `is_death_star` is
  families `0x18`-`0x1b`.
- `integrator::order_strike` runs the gate, flags each ordered fleet not in
  transit, and runs `resolve_strike` once per system holding a flagged fleet,
  in the step that issued the order. `resolve_strike` takes the Kill branch
  (`death-star-strategic.md`, Port) or the defence and effect phases, then
  the finalize, the flags' clearing and the shifts the run posted
  (`support-shifts.md`, Port). The player's order (a menu row, or the
  `Bombard …:` and `Destroy system:` dev commands, both through
  `strike_orders::issue`) seeds its run from `sim_rng`; the AI's
  (`resolve_ai_strike` inside `simulation::ai_step`) from the AI step's roll
  stream. No battle bombards.
- Each target list is in container order (`object-state-flags.md`: by
  family, then id), so the collateral pick is the holder's shipyard
  (`0x28`) before a training facility, construction yard, mine or refinery.
  `hyp:` within a family the port's per-kind list is the id order.
- `port:` the run draws from a stream seeded by one roll and the system id; a
  squadron counts its full FIGHTSD value (the strategic fleet keeps no
  squadron losses). A ship's remaining shields are derived from its shield
  counter and hull damage on each run (`bombardment::remaining_shields`,
  Defence phase, "Remaining shields W").
- `StrikeReport.losses` holds each killed regiment, each bombarding ship
  the batteries destroyed, and the squadrons and regiments aboard it (its
  hook `+0x11c`, `FUN_00502840` → `FUN_004fb520`, calls each child's slot
  `+0xb4`, `FUN_004f8710` for a squadron and a regiment, and
  `FUN_004f7ed0` passes the ship's reason; `bombardment::bombard` records
  the squadrons, `integrator::resolve_strike` the regiments as it clears
  the lost transports' cargo), with reason `0xf`, or `0x10` at the bombarder's
  own system, for the blockade loss counters (`record_loss`,
  `support-shifts.md`). A hit on the HQ runs
  `support_shift::remove_headquarters`.
- Messages (`strike_orders::strike_messages`), to the bombarder and to the
  holder when it is the other side: a run files Operation Report `0xf` on the
  Conflict rail with advice `0x2e` for the holder, and Rebel HQ Destroyed
  `0x1f` (`0x7188`/`0x7189`, advice `0x11`, Resource rail, both sides) when a
  hit destroyed the HQ; Destroy System files Planet Destroyed `0x15` on the
  Mission rail, Character Health `0x1a` per killed character to its side
  (title `0x7164`, body `0x7365`, picture `0x42e` for the Alliance and `0x433`
  for the Empire, `FUN_0048fdf0.c:121-140`, WAVE `0x474` at `+0x30`, `:42`; the
  event is `0x106`, filed for a side only when `FUN_004f2d10(side, id)` finds
  the character in that side's view, and not at all for a destroyed character
  whose `+0x40` byte is 7, `:11-37`), and Rebel HQ Destroyed `0x1f` when the cascade
  removed the HQ (the AD change, `FUN_0048e5a0`); Destroy System files no Operation
  Report: `FUN_00488de0` maps `0x14f` to kind 0 when the side's view of the
  system has its destroyed bit set, and maps `0x302` of a system-family object
  to kind `0x15` (`FUN_00488de0.c:176-188`, `:354-366`;
  `operation-report-window.md`). A fleet's bombard bit is set on the master
  (`FUN_004fe460`), copied to both views (`FUN_004ff450`) and raises `0x182`
  in each view copy whose mask is live (`FUN_005404d0`; `observation-levels.md`,
  "When a view copy follows its master"); a message
  class finds the fleet only through its own side's view (`FUN_004f36a0`).
  `hyp:` the holder's view holds every fleet in orbit of its system, and a
  bystander's view of a neutral system none (the objects a view holds are
  copied by `FUN_004f9510`, `FUN_0050d5a0` levels and the setup stage
  `FUN_0051b990`; which of them a fleet's arrival reaches is untraced). Destroy System plays movie `0x6f` (System Destroyed) to the
  sides told (`strike_orders::strike_movies`), and a lost HQ plays `0x73`
  (Headquarters Destroyed) through the win word's AD change
  (`strike_orders::word_movies`). The original files Rebel HQ Destroyed only
  when the win word's AD bit goes from 0 to 1 and Coruscant Captured only when
  its CA bit goes from 0 to 1 and AD does not (`victory-flag-word.md`, section
  4); `port:` `word_movies` files from the new word on every change.
- The menu rows: `object-popup-menu.md`, Ported. Telemetry: one `bombardment`
  record per run (order, hits, kills by kind, defence damage, ships lost,
  `support_change`). Dev commands: `agent_docs/dev-commands.md`.

## Open

- The readers of the system's bombard bit (`+0x88` bit 6); `FUN_0050a660`
  also calls slot `+0x224` of the system when the bit changes.
- Whether the head of the scheduler's command queue `+0xac` is always due.
- Kind `0xf` needs the `0x14f` node with value 0 (system bombard bit cleared);
  `0x182` only supplies the fleet lists and never selects a kind
  (`FUN_00488de0`). Whether the VM alert is raised on the cleared bit is for an
  A0 capture.

## Decompiles

Order and setup: `FUN_00582440.c`, `FUN_004fd6b0.c`, `FUN_005003d0.c`,
`FUN_005637c0.c`, `FUN_0058d720.c`, `FUN_0058d800.c`, `FUN_0058d770.c`,
`FUN_00581e80.c`, `FUN_00553770.c`, `FUN_005820a0.c`, `FUN_00520040.c`,
`FUN_005201c0.c`, `FUN_00582190.c`, `FUN_00582200.c`, `FUN_005535b0.c`,
`FUN_005187f0.c`, `FUN_0053f150.c`, `FUN_005539f0.c`, `FUN_00563490.c`,
`FUN_00563580.c`, `FUN_005636f0.c`, `FUN_00563620.c`, `FUN_00507f10.c`,
`FUN_00507fc0.c`. Fleet commands and bits: `FUN_005829f0.c`,
`FUN_0057fe90.c`, `FUN_0057ff60.c`, `FUN_00580030.c`, `FUN_004fec00.c`,
`FUN_004fe020.c`, `FUN_004fdf70.c`, `FUN_004ffa70.c`, `FUN_004fe460.c`,
`FUN_004ff600.c`, `FUN_0050a660.c`. Queue: `FUN_004fd370.c`,
`FUN_004fcff0.c`, `FUN_0051d8d0.c`, `FUN_0051df30.c`.
Ship phase and walkers: `FUN_0058d990.c`, `FUN_0058d3f0.c`, `FUN_0058d440.c`,
`FUN_0058ca70.c`, `FUN_0058d150.c`, `FUN_0058d1c0.c`, `FUN_0058c960.c`,
`FUN_0058c9b0.c`, `FUN_0058ce70.c`, `FUN_0055e9e0.c`, `FUN_0056f3d0.c`,
`FUN_005131d0.c`, `FUN_004f6b90.c`, `FUN_004ef6d0.c`.
Defence: `FUN_0058dc10.c`, `FUN_00527550.c`, `FUN_005272e0.c`,
`FUN_00520b70.c`, `FUN_00509330.c`, `FUN_0055e930.c`, `FUN_0058d540.c`,
`FUN_0055e9a0.c`, `FUN_0058d4f0.c`, `FUN_0058d230.c`, `FUN_0058d680.c`,
`FUN_00501160.c`, `FUN_00520b80.c`, `FUN_00500670.c`, `FUN_0053e170.c`,
`FUN_0058d6a0.c`, `FUN_00501ba0.c`, `FUN_00501bc0.c`, `FUN_00501be0.c`,
`FUN_00501ae0.c`, `FUN_005006e0.c`, `FUN_0053a6e0.c`, `FUN_004fd790.c`,
`FUN_005006f0.c`. Values: `FUN_00501470.c`, `FUN_005032a0.c`,
`FUN_00500f00.c`, `FUN_00503110.c`, `FUN_00520b60.c`.
Effect and targets: `FUN_0058df60.c`, `FUN_0055e950.c`, `FUN_00527940.c`,
`FUN_00520b80.c`, `FUN_0058d080.c`, `FUN_0058cba0.c`, `FUN_00584320.c`, `FUN_00504cc0.c`,
`FUN_00527050.c`, `FUN_00526d70.c`, `FUN_0053b760.c`, `FUN_0058cf80.c`,
`FUN_0055e970.c`, `FUN_0053e2c0.c`, `FUN_0053e290.c`, `FUN_0058d020.c`,
`FUN_0058cf20.c`, `FUN_0058cf50.c`.
Finalize: `FUN_0058e320.c`, `FUN_0055a0e0.c`, `FUN_0050c9f0.c`,
`FUN_0055d900.c`, `FUN_0055da90.c`. Enable and messages: `FUN_0051fe20.c`,
`FUN_004f9860.c`, `FUN_004ff450.c`, `FUN_004ff840.c`, `FUN_00496460.c`,
`FUN_00496770.c`, `FUN_00492470.c`, `FUN_004950d0.c`. Parameters: `FUN_0055e840.c` (GNPRTB
`0x600`-`0x606` = 40, 40, 1, 10, 9, 9, 50).
