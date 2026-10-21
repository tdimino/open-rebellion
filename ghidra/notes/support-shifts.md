---
title: "Support Shifts: Capture, Battle, HQ Loss and the Death Star"
description: "The sector-wide popular-support shifts (FUN_00518b20 records, event 0x373) after a control change, a Coruscant control change, a blockade's end, the Alliance HQ's removal and a Death Star kill, with their GNPRTB/SDPRTB 0x1400-0x1445 values; system +0x88 bit 11 is the Death Star's presence in the sector and bit 3 a gate that keeps a system out of every sector shift"
category: "ghidra"
updated: 2026-10-18
tags: [support, popularity, control, blockade, death-star, gnprtb, sdprtb]
---

# Support shifts

Answers task-graph node `or-q-support-shifts-hol`. Every claim below was read
in the `.c` files cited. Support is system `+0x58`, the Alliance's share
(0-100); `FUN_00507270(side)` gives `+0x58` for side 1 and `100 - +0x58` for
side 2.

## The record and how it is applied

A shift is a 28-byte record (`FUN_00562ae0`, defaults `FUN_00562ba0`):

| Field | Meaning |
|---|---|
| `[0]` | owner filter: 0 every system, else only systems of that owner |
| `[1]`,`[2]` | system family range (default `0x90`-`0x98`) |
| `[3]` | gate side |
| `[4]` | gate limit |
| `[5]` | direction side |
| `[6]` | amount |

`FUN_00518ac0(object, record)` finds the object's sector (its parent,
`FUN_004f6b50`) and `FUN_00518b20` walks the sector's systems. For each system
`FUN_00518be0` requires: the family in `[record[1], record[2])` (default
`0x90`-`0x98`, so families `0x90`-`0x97`; the range is set by `FUN_00562ba0`
and `FUN_0055de00`), the owner filter (system `+0x24` bits 6-7 equal
`record[0]`, or `record[0]` is 0), `+0x88` bit 0 (populated) set and bit 3
clear, and `support(gate side) < gate limit`. The record is valid only with
`record[0]` 0-3, gate side 1 or 2, direction 1-3 and a nonzero amount. It
then posts event `0x373` (`FUN_00562c70`, `FUN_0053fcd0`) carrying the system
id and `amount`, negated when the direction side is not 1. The gate, the
sign and the amount are fixed when the event is posted.

Event `0x373` goes to the event queue (`+0xb0`; `FUN_004fd390` →
`FUN_0051d8f0`), with priority slot `+0x28` = `0x100` (`FUN_0048bc60`, the
base class's answer; `FUN_0054f4e0`). It does not run inside the handler that
posted it: `FUN_0051df30` serves the event queue first and repeats until
every queue is empty (`blockade-bit.md`, "The event queue"), so the shift
lands in the same frame step's phase 5, after the events already queued.
Each fire is one clamp: a system hit by two shifts is clamped after the first
and again after the second. The fire (`FUN_00562fb0`) looks the system up and
calls `FUN_0050c9f0(system, amount, 1, ctx)`:

```
delta = amount
if +0x88 bit 11 and delta > 0:  delta = delta / G7681      ; FUN_00559be0, G7681 = 2
+0x58 = clamp(+0x58 + delta, 0, 100)                        ; FUN_0053e0d0, FUN_00509c00
```

The halving reads bit 11 when the event fires, not when it was posted.
`FUN_0050c9f0` does nothing on an unpopulated system (`+0x88` bit 0 clear),
and returns true. Direct callers of `FUN_0050c9f0` (the destroyed system's own
GNPRTB 7705 change) skip the bit 3 test, which only `FUN_00518be0` makes.

**Bit 11 is the Death Star in the sector.** Its only setter `FUN_0050a890`
is called from `FUN_005178b0`, which marks every system of a sector whose
family `0x18`-`0x1b` object (`FUN_005061f0`; CAPSHPSD family `0x18`,
TEXTSTRA 10120 "Death Star") has `+0x50` bit 0 and lies in that sector
(`FUN_00518690`). Event `0x301` reruns it when a family `0x18`-`0x1b` object
changes (`FUN_00514510`), as does `FUN_00514a60`. The same bit:

- halves shifts toward the Alliance (above);
- multiplies the Empire's regiments by G7680 (2) in the support drift
  (`FUN_0050b230` → `FUN_00559c40` → `FUN_00559b60`);
- halves the Empire's garrison requirement and counts its regiments twice
  in the uprising score (`FUN_0050b5a0`, `FUN_0050d030`;
  `uprising-incident.md`).

**Bit 3 gates every sector shift.** `FUN_00518be0` skips a system whose
`+0x88` bit 3 is set. Writers of bit 3 (`FUN_0050a510`, mask `8`, notification
slot `+0x218`), all three:

- `FUN_005081f0`, system slot `+0xbc` (the base class; vtable `0x0065e640`),
  sets bit 1 and bit 3 to 1. `FUN_0051bc70` calls slot `+0xbc` right after
  creating each system in setup stage `0xc`.
- The core class's slot `+0xbc` is `FUN_00566d40`: it calls `FUN_005081f0`,
  rolls the populated bit, support, energy and raw material, then clears bit 3
  (`FUN_0050a510(0)`). Every core system (families `0x90`-`0x91`) leaves
  creation with bit 3 clear.
- The rim class's slot `+0xbc` (`0x0056a550`, `FUN_0056a550`) calls
  `FUN_005081f0`, sets populated and support for a held system, and never
  clears bit 3.
- `FUN_0050b890` (called from `FUN_00510820`, the control-change slot `+0xf0`,
  and from `FUN_00510e30`, the populated bit's hook `+0x20c`) acts only while
  bit 3 is set: it writes bit 3 = 1 when the system is neutral (side bits 3) or
  unpopulated, else 0. Once clear, nothing in these three sets it again.

Both hooks run only in play (`DAT_006b90e0` set exactly at setup stage `0x1a`,
`FUN_0051c010`; `FUN_00510820` and `FUN_00510e30` test it before
`FUN_0050b890`). A rim system held and populated since creation therefore
starts play with bit 3 set and is out of every sector shift until its holder
changes or its populated bit flips; core systems are never out.

In the original (VM, `empire-huge-deathstar-built.sav` played to day 240):
a Death Star kill of Alliance-held Yaga Minor moved none of the 140 rim
systems (family `0x92`), the two Alliance-held ones included, while every
core system below its limit moved. Each core sector moved by one amount:
10 from the kill's galaxy shift plus 1 for each system in that sector whose
holder changed as the shift tipped it (Corellian 12 with two changes; 14,
15, 16 and 18 elsewhere). In the Death Star's own sector the 10 was halved
by bit 11 and Yaga Minor's change to neutral took 1 back: 4.

## Event order

Queue `+0xb0` (vtable `0x00662018`) is a binary tree ordered by slot 2,
`FUN_0054ee90`: `a.priority < b.priority`, strict, with the priority the
item's slot `+0x28` (`FUN_0054ee90.c`). An insert (`FUN_005f61c0.c`) goes
left of a node only when the new item's priority is strictly lower, so an
equal priority goes behind the items already queued. The head (`+8`,
`FUN_0041c210`) is the lowest priority. `FUN_0054ec90.c` unlinks it
(`FUN_005f6250.c`), runs it through slot `+0x20` and deletes it, and
`FUN_0051df30` repeats until the queue is empty. Events therefore run lowest
priority first and in raising order within one priority, and an event raised
while the queue drains is inserted by its priority, ahead of every queued item
with a higher number.

Slot `+0x28` per event class (ids from the factory table filled by
`FUN_0054f140` in `FUN_0051ef80`; class vtable slot 10, address `+0x28` in the
vtable):

| Event | Factory → init | Vtable | Slot `+0x28` | Priority |
|---|---|---|---|---|
| `0x300` | `FUN_0057a5e0` → `FUN_0057a500` | `0x00669688` | `FUN_0048af30` | `0x10` |
| `0x301` | `FUN_0057a090` → `FUN_00579fa0` | `0x00669628` | `FUN_0048af30` | `0x10` |
| `0x305` | `FUN_00563f20` → `FUN_00563e40` | `0x00663388` | `FUN_0048af30` | `0x10` |
| `0x306` | `FUN_0057ab40` → `FUN_0057aa50` | `0x006696e8` | `FUN_0048af30` | `0x10` |
| `0x31b` | `FUN_005649e0` → `FUN_005648f0` | `0x00663448` | `FUN_0048af30` | `0x10` |
| `0x320` | `FUN_0057b8d0` → `FUN_0057b7e0` | `0x006697d8` | `FUN_0048af30` | `0x10` |
| `0x373` | `FUN_00562d70` → `FUN_00562c70` | `0x006632a8` | `FUN_0048bc60` | `0x100` |
| `0x311` | `FUN_00564ce0` → `FUN_00564bf0` | `0x00663478` | `FUN_0048e670` | `0x400` |
| `0x31a` | `FUN_0057bbb0` → `FUN_0057bac0` | `0x00669808` | `FUN_0057bd20` | `0x999` |

One drain therefore runs every holder rebuild (`0x301`, `0x320`) and holder
change (`0x31b`) first, in raising order, then the shifts they posted
(`0x373`) in posting order, then the blockade rerun (`0x311`). A holder change
a shift causes posts a `0x31b` that runs before the next shift. The control
write posts `0x311` before `0x31b` (`FUN_00510d70` → `FUN_004f6f40` → slot
`+0xf0` `FUN_00510820` → `FUN_0050ac10(1)`, which sets system `+0x88` bit
`0x100000`, whose hook slot `+0x258` `FUN_00511b30` posts `0x311` and clears
the bit; then `FUN_00510d70.c:27` posts `0x31b`), and the priorities still run
`0x31b`, its shifts and every holder change they cause before `0x311`.

## The support hook and the other recomputes

The support setter `FUN_00509c30` writes `+0x58` only when the value changes
(0-100), copies it to both side views (`FUN_0050f390`) and calls slot `+0x1d0`.
That is `FUN_005109f0` (`0x0065e810`; the third class's `FUN_00567110` calls it
first). It runs in this order (`FUN_005109f0.c:14-29`), each step through the
field's own setter, which writes the field, copies it to both views and calls a
notification slot:

1. `FUN_0050b610`: the control word (above).
2. `FUN_0050b5a0`: the garrison requirement, `FUN_00559fe0(owner side,
   support of that side, +0x88 bit 11, +0x88 bit 2, 1)`, stored at `+0x80`
   (`FUN_0050a290`, notification slot `+0x204`). Zero for a neutral owner or an
   unpopulated system. The owner is `+0x24` bits 6-7.
3. `FUN_0050b230`: the drift, `FUN_00559c40` over the counts at the system
   (`FUN_00509140`, `FUN_00509020`, `FUN_00508fb0`, `FUN_00509090`) and bit 11,
   stored at `+0x6c` clamped to -100..100 (`FUN_00509ec0`, slot `+0x1e4`).
4. `FUN_0050b2c0`: the collection rate, `FUN_00559c10(support of the owner)`
   (100 for a neutral owner), stored at `+0x70` (`FUN_00509f40`, slot `+0x1e8`).
5. With the blockade bit `0x20` set only: the blockade timer minimum
   (`FUN_00509980`, `FUN_0053fab0`; `timer-scheduler.md`).

There is no clock pass for these four. Their other triggers:

| Value | Recomputed by |
|---|---|
| garrison requirement `+0x80` | setup `FUN_00508250`; the owner hook `FUN_00510820.c:55` (after the child walk); the support hook; the populated hook `FUN_00510e30`; `FUN_00510f20` (the hook that posts event `0x303` and arms timers `900` and `0x38d`); `FUN_00511740` (garrison requirement, then drift; a virtual slot) |
| drift `+0x6c` | setup; the owner hook (`FUN_00510820.c:41` writes 0 through `FUN_00509ec0`, `:48` recomputes); the support hook; `FUN_00511740`; slot `+0xd4` `FUN_00508da0` (system vtable index 53, called by event `0x301`'s handler `FUN_00514510` on its third argument's container) for a regiment (`0x10`-`0x13`) and for a ship or squadron (`0x14`-`0x1f`) |
| collection rate `+0x70` | setup and the support hook only |

For a regiment (`0x10`-`0x13`) the slot `+0xc8` (`FUN_00508660`) reruns the
control word (`FUN_0050b610`), then `FUN_0050ba90` and `FUN_0050b500` (the
owner's regiment count `FUN_00513180` minus `+0x80`, stored through
`FUN_0050a230`); it does not run the drift, the garrison requirement or the
collection rate.

## Parameters

GNPRTB values are the same in every column. SDPRTB values are
Alliance / Empire.

| Id | Table | Value | Use |
|---|---|---|---|
| `0x1401` | GNPRTB | -2 | Death Star kill, rim amount |
| `0x1402` | GNPRTB | 90 | Death Star kill, rim gate limit |
| `0x1403` | GNPRTB | -10 | Death Star kill, core amount |
| `0x1404` | GNPRTB | 101 | Death Star kill, core gate limit |
| `0x1405` | GNPRTB | 0 | HQ arrival amount |
| `0x1406` | GNPRTB | 0 | HQ arrival gate limit |
| `0x1407` | GNPRTB | -10 | HQ removal amount |
| `0x1408` | GNPRTB | 101 | HQ removal gate limit |
| `0x1420` | GNPRTB | 10 | blockade divisor |
| `0x1410` | SDPRTB | 101 / 101 | blockade gate limit, by gate side |
| `0x1411` | SDPRTB | 20 / 20 | blockade cap, by direction side |
| `0x1412` | SDPRTB | 100 / 100 | blockade weight, stronger side base |
| `0x1413` | SDPRTB | by column | blockade weight, stronger side slope |
| `0x1414` | SDPRTB | 0 / 0 | blockade weight, weaker side base |
| `0x1415` | SDPRTB | 100 / 100 | blockade weight, weaker side slope |
| `0x1420` | SDPRTB | 1 / 1 | control change amount per side |
| `0x1421` | SDPRTB | 100 / 100 | control change gate limit |
| `0x1422` | SDPRTB | 10 / 5 | Coruscant control change amount per side |
| `0x1423` | SDPRTB | 100 / 90 | Coruscant control change gate limit |
| `0x1408`, `0x1409` | SDPRTB | -2 / -1, 101 | bombardment's sector shift, rim amount and gate limit, by direction side (`FUN_0055d900`) |
| `0x140a`, `0x140b` | SDPRTB | -4 / -3, 101 | bombardment's sector shift, core amount and gate limit, by direction side (`FUN_0055d900`) |
| `0x1440`, `0x1442`, `0x1444` | SDPRTB | 0, 10, 0 | control change amount: `0x30` bits set in the new word only, clear in the new word only, set in both; by direction side |
| `0x1441`, `0x1443`, `0x1445` | SDPRTB | 101 | their gate limits, by direction side |
| `0x1e01` (7681) | GNPRTB | 2 | Death Star divisor |
| `0x1e19` (7705) | GNPRTB | -20 | a bombardment's support loss at its system (nil at a destroyed one) |

SDPRTB `0x1413` is 400 for the player's side in every column; for the
computer's side it is 200 (Easy), 150 (Medium), 100 (Hard), and 400 in
development and multiplayer. The loaders are `FUN_0055cb60` (`0x14xx`) and
`FUN_00558bb0` (`0x1e00`-`0x1e02`); `FUN_0053e390` reads GNPRTB (table
`0x3c`), `FUN_0053e3e0` SDPRTB (table `0x3d`) for side 1 or 2.

## The shifts

### Control change (`FUN_00517aa0`, `FUN_0055de50`)

Event `0x31b` (`FUN_00510d70` raises it on every control-word write, then
`FUN_00515c00`; the word is system `+0x84`, written only by `FUN_0050a300`,
whose only caller is `FUN_0050b610`, `FUN_0050a300.c:25`, `:38`,
`FUN_0050b610.c:71`) gives a system's old and new control words (bits 0-1
holder, 1 Alliance, 2 Empire, 3 Neutral; bits 2-3 the support-derived side,
`FUN_0055a080` of `+0x58`, 3 for an unpopulated system,
`FUN_0050b610.c:26-33`; bits 4-5 the side that has usable regiments at the
system, below). `FUN_0050b610` rebuilds the word on every call
(`FUN_0050b610.c:24-70`): bits 4-5 come from two walks of the system's direct
children in `[0x10,0x14)` that are usable (`+0x50` bit 0) and whose owner
(`+0x24` bits 6-7) is side 1, then side 2 (`FUN_00504cc0`, `FUN_00513090`,
`FUN_005131d0.c:15-28`, `FUN_004f6b90` mode 1): none gives 0, only side 1's
gives 1, only side 2's gives 2, both keep the old bits (`:45-61`). The holder
takes bits 2-3 when bits 4-5 are 0, else bits 4-5 (`:64-70`). The uprising
state is a separate word: `+0x78` bits 4-5 (`FUN_0050a130`, `FUN_0050b4d0`,
`FUN_0050b800`) and `+0x88` bit 2, whose setters call the empty slot `+0x1f8`
(`FUN_00524b10`) and never raise `0x31b`. Everything below needs the holders to
differ (`(old ^ new) & 3`). A shift goes to every system of the changed
system's sector (`FUN_00518ac0`):

- `from` = old holder, `to` = new holder; a holder of 3 (Neutral) counts as the
  opposite of the other end (old 3 and new 1 give `from` 2). Direction `to`,
  gate side `from`, gate limit by `to`, so a side that loses a system to
  Neutral shifts the sector toward the opposite side.
- No `0x30` bits before or after (both words clear): amount = SDPRTB
  `0x1420[1]` if either raw holder is 1, plus `0x1420[2]` if either is 2
  (base `DAT_00661a84` = 0): 2 when the Alliance and the Empire trade a
  system, 1 between a side and Neutral. Gate limit `0x1421[to]` (100).
- `0x30` bits clear in the new word only: amount `0x1442[to]` (10), limit
  `0x1443[to]`. Set in the new word only: `0x1440[to]` (0), `0x1441[to]`.
  Set in both: `0x1444[to]` (0), `0x1445[to]`. A zero amount posts nothing
  (`FUN_0055de50.c:50-89`). The SDPRTB rows `0x1440`-`0x1445` are named
  for these regiment bits here (`hyp:` their designers' meaning).
- The per-step cap (`FUN_00518040`, only when neither word has `0x30` bits
  and the game root exists): the game root (`FUN_00506e10`) `+0x78` holds the
  last step number, `+0x7c`/`+0x80` count for old holder 1 and `+0x84`/`+0x88`
  for old holder 2. A different step number (`FUN_004fd330` is the scheduler's
  step counter `+0xc`, `FUN_004fcee0`, which `FUN_0051dea0` raises on every
  frame step and never resets; the day is `+0x10`) zeroes all four first. The
  change is let through (returns 1) when its old holder's two counters are 0,
  and then both rise by 1; a second change with the same old holder in the
  same step returns 0 and posts no shift. An old holder of 3 has no counter
  and always passes; changes with `0x30` bits skip the check. A day runs the
  larger side speed word plus one steps (5 to 601, `timer-scheduler.md`).
  Timers come due by day (`FUN_0051d8a0.c:6`) and phase 5 runs every due
  item in the day's first step (`FUN_0051df30`), so the changes timers cause
  share that step's counters; an order (an assault, a Destroy System kill)
  runs in the step that issues it, with counters of its own when that step
  is a later one.
- Coruscant (system `+0x18 == 0x109`) then runs `FUN_00517be0` on every event
  `0x31b` (`FUN_00517aa0` calls it whether or not the holders changed).

- **Destroy System's kill.** The populated-bit clear runs hook `+0x20c`
  (`FUN_00510e30.c:31`), whose first rebuild is `FUN_0050b610`: the populated
  bit is already 0 (`FUN_0050a3c0.c:13`), so bits 2-3 become 3
  (`FUN_0050b610.c:26-28`), bits 4-5 are rebuilt from the usable regiments the
  system still holds (they die later, in the cascade's child walk,
  `death-star-strategic.md` step 3), and the holder is bits 4-5 when set, else
  3. Branch by the system's usable regiments of side 1 or 2 at that moment:
  none: both words have no `0x30` bits and the holder goes from 1 or 2 to 3, so
  `FUN_0055de50` takes the first branch, SDPRTB `0x1420` amount 1 (the VM's
  "took 1 back"); one or more:
  bits 4-5 are unchanged, the holder is unchanged, `(old ^ new) & 3` is 0 and
  `FUN_00517aa0` (`FUN_00517aa0.c:21`) posts nothing then. Each regiment's
  death then clears its usable bit (`FUN_004f7ed0` → `FUN_004f71d0` →
  `FUN_004f7560` → `FUN_004fb810` → `FUN_004f7b80` → `FUN_004f7410`), whose
  hook `+0x110` (`FUN_004ff5e0` → `FUN_004fb310`) posts event `0x301` with the
  system as container; `FUN_00514510` calls the system's slot `+0xc8`
  (`FUN_00508660`), whose regiment branch reruns `FUN_0050b610` whatever the
  flag. With none left the holder is 3 and the old word alone has `0x30`
  bits: `FUN_0055de50` posts SDPRTB `0x1442[to]` (10), limit `0x1443[to]`.
  `hyp:` `FUN_00504dc0` resolves the destroyed system's id for the event.

### Coruscant's own shift (`FUN_00517be0`, `FUN_0055e010`)

`FUN_00517be0` recomputes the win word first (`FUN_005180e0`,
`game-end-path.md`), then, when the holders differ, posts a second record to
Coruscant's sector, with no per-step cap and no `0x30`-bit test: `from`, `to`
mapped as above, amount = SDPRTB `0x1422[1]` if either raw holder is 1 plus
`0x1422[2]` if either is 2 (10 for the Alliance's end, 5 for the Empire's, 15
when they trade it), gate side `from`, limit `0x1423[to]` (100 toward the
Alliance, 90 toward the Empire). It follows the generic shift of the same
event, so Coruscant changing hands moves its sector by 2 or 1 and by 15, 10 or
5 more.

### Blockade end (`FUN_00515f40`, `FUN_00517ee0`, `FUN_0055dbf0`)

Per-system loss counters live in the record at system `+0x54`: A at
`+0x44 + 4·side` (`FUN_005099c0`, `FUN_0050ce20`), B at `+0x50 + 4·side`
(`FUN_005099f0`, `FUN_0050ce50`), C at `+0x64 + 4·side` (`FUN_00509ae0`,
`FUN_0050cee0`), the blockading side at `+0x60` (`FUN_00509ac0`,
`FUN_0050ce80`, never 3), side 1-3. When the blockade bit (`+0x88` `0x20`) is
set, `FUN_00511300` zeroes A and B for sides 1-3 and stores the blockading
side (`FUN_00509890`); clearing it leaves them. C is not reset there.

`FUN_00508c60` (system slot `+0xd0`, vtables `0x0065e640`, `0x00663698`,
`0x00664098`) is the only writer. Event `0x300` (`FUN_005140c0`) calls it
on the system of the event's `+0x44` container with the object, the event's
fourth argument (the new value of the object's active bit `+0x50` `0x40`) and
the context. It acts when that argument is 0, the object's destroyed bit
(`+0x50` bit 3) is set and its family is `0x10`-`0x1f` (regiments, capital
ships and squadrons). It takes
`v` = slot `+0x1c0` minus slot `+0x1c8` (on the regiment, capital ship and
squadron vtables `0x0065e438`, `0x0065d650`, `0x0065def0`, `+0x1c0` is the thunk
`0x00558050` → `FUN_00520b60`, the class record's `+0x58`, DAT `0x30`
`uprising_defense` in TROOPSD, CAPSHPSD and FIGHTSD, and `+0x1c8` is
`FUN_006158b0`, which returns 0), `s` = the object's owner (`+0x24`
bits 6-7, 1-3) and the reason byte (`+0x40`): `0xe` adds `v` to A[s], `0xf` to
B[s], `0xc` to C[s], any other reason nothing. Reason `0xf` is a bombardment
hit on an enemy object or a bombarding ship killed by defence fire, and the
destroy cascade's reason for a destroyed enemy system's regiments (`0x10` for
the bombarder's own system adds nothing; `bombardment.md`,
`game-end-path.md`). Reason `0xe` is the argument of slot `+0xac` in
`FUN_0058c580`, `FUN_0058bec0` and `FUN_0058c3c0` (the casualty reason of the
planetary assault, `ground-assault.md`). C's reader is `FUN_00517270` and `FUN_00560d00`
(not traced here).

When the blockade bit clears (event `0x305` with 0; `FUN_00515f40`):

- `X` = the blockading side. Unless the system is destroyed (`+0x50` bit 3),
  `X` becomes the other side when the blockader is not the system's holder
  (`+0x24` bits 6-7).
- `L1`, `L2` = A + B for each side, plus side 3's A + B when that side is not
  the blockader (`FUN_00509a20`; 0 when the stored blockader is not 1 or 2).
- `T1`, `T2` = slot `+0x1c0` summed over every family `0x10`-`0x1f` object
  with `+0x50` bits `0x40` and `0x4` set, by owner (1 or 2).

```
S = the side with more (T1 > T2 → 1, else 2); W = the other; d = T_S - T_W
k_S = 0x1412[S] + 0x1413[S]·d / T_S             ; percent, 0 when T_S = 0
k_W = 0x1414[W] + 0x1415[W] - 0x1415[W]·d / T_S
w_i = L_i · k_i / 100
amount = (2·w_O - w_X) / G0x1420                 ; O = the side other than X; integer division
```

The record starts with direction `X`. A negative amount turns the direction to
`O` and is made positive. The amount is clamped to `0 .. 0x1411[direction]`
(20). The gate side is the other side of the direction, limit `0x1410[gate
side]` (101); no owner filter, so every populated, bit-3-clear system of the
sector shifts. So the side whose opponent lost more troops gains up to 20
points across the sector, and the galaxy's stronger side's losses weigh more.
A zero amount (including `T_S` = 0) posts nothing. A system destroyed while
blockaded keeps `X` = the blockader.

### Alliance HQ removed or arriving (`FUN_00517cd0`, `FUN_00517de0`)

- Event `0x300` (`FUN_005140c0`) for a family `0x20`-`0x21` object whose
  fourth argument is 0 (the object's active bit `+0x50` `0x40` just cleared:
  `FUN_004f76b0(0)`, hook `+0x128`, `FUN_004fb810`; `FUN_004f7bc0` clears it
  when the object is destroyed or never created): owner filter 0, gate side 1,
  limit GNPRTB `0x1408` (101), direction 1, amount GNPRTB `0x1407` (-10)
  (`FUN_0055e1f0`). Direction 1 with a negative amount lowers `+0x58`
  by 10 (the Empire gains 10) in every populated, bit-3-clear system of the
  sector of the system resolved from the event's `+0x44` container
  (`FUN_00504dc0`; a failed resolve skips the shift, hyp that a destroyed
  HQ's container is its system). No halving applies, since the delta is
  negative. The victory check (`FUN_005180e0`) runs after it.
  Every removal of an HQ reaches it: the destroy cascade, a military hit, and
  a control change that calls `+0xac(3)` (`game-end-path.md`); the shift
  repeats for each HQ-family object removed.
- Event `0x301` (`FUN_00514510`) for the HQ en route (`+0x50` bit 4)
  arriving at a system: owner filter 1, direction 1, amount `0x1405` (0),
  limit `0x1406` (0) (`FUN_0055e150`). A zero amount posts nothing.

### Bombardment and the Death Star kill (`FUN_0058e320`)

A bombardment (`bombardment.md`) sets the state's `+0x28` when it destroys a
civilian facility (`0x28`-`0x2f`, `FUN_0058ce70`) or lowers energy or raw
material (`FUN_0058cf20`, `FUN_0058cf50`). Destroy System with a family
`0x18`-`0x1b` ship sets `+0x28` and `+0x2c` (`FUN_0058d990` →
`FUN_0058ce70`). Finalize then, with `+0x28`:

- changes the system's support for the bombarding side by GNPRTB 7705
  (-20) (`FUN_0055a0e0`, `FUN_0050c9f0`); for the Empire that change favours
  the Alliance, so the Death Star's own presence halves it to -10. After a
  Destroy System the change is nil: `FUN_0050c9f0` returns at once for a
  system whose `+0x88` bit 0 (populated) is clear, and the cascade has
  cleared it;
- with `+0x2c`, across the galaxy (`FUN_00518a70`, every sector), with the bombarding
  side as direction and gate side (`FUN_0055da90`): core systems
  (`0x90`-`0x91`) shift by `0x1403` (-10) where that side's support is below
  `0x1404` (101); rim systems (`0x92`-`0x93`) by `0x1401` (-2) where it is
  below `0x1402` (90).

So an Imperial Death Star kill raises the Alliance's support by 10 in every
core system and 2 in rim systems where the Empire has under 90, halved in
the Death Star's own sector. Any other bombardment that sets `+0x28` takes the
sector-wide branch (`FUN_0055d900`, `FUN_00518ac0`, SDPRTB `0x1408`-`0x140b`:
-2/-1 and -4/-3) after the GNPRTB 7705 change at the system.

## Port

`crates/rebellion-core/src/support_shift.rs` holds the record
(`ShiftRecord`), `post` (`FUN_00518b20`, `FUN_00518be0`: the family, the
populated bit, bit 3 and the gate read at post time, the sign fixed then) and
`fire` (phase 5: the queue's head served until empty, `FUN_0051df30`, the
lowest priority first, `raise` inserting as `FUN_005f61c0` by
`FUN_0054ee90`;
event `0x373` through `uprising::apply_support_change`, each clamped on its
own; a regiment's event `0x301`/`0x320` and a holder change's `0x31b`).
Queued events wait on `GameWorld::support_events`, and each operation that
raises them serves them before it returns: a strike
(`integrator::resolve_strike`), an assault, an applied AI order
(`simulation::ai_step`), a player's order and scrap (native), the day's
overdraft scraps, builds and regiment arrivals, the blockade events, a
mission's results and the uprising events. The finalize's GNPRTB 7705 change
at the system calls `apply_support_change` at once, as `FUN_0050c9f0`'s
direct callers do.
`apply_support_change` is the one setter (`FUN_00509c30`): it does nothing on
an unpopulated system and halves a positive Alliance delta under bit 11.

- **The holder.** `support_shift::rebuild_control` is `FUN_0050b610`: it
  rebuilds the troop bits (`GameWorld::troop_held`) and the holder
  (`economy::system_control`: one side's regiments, populated or not, else
  the support holder, neutral for an unpopulated system), and a changed
  control runs the write's hook (`FUN_00510d70`): the facilities' hand-over,
  the owner change (bit 3, the HQ's removal, the child walk below) and event
  `0x31b` on the queue. Its callers are the original's: the support setter's
  hook (slot `+0x1d0`, `FUN_005109f0`; `support_changed`, at once), the
  regiment hook (slot `+0xc8`, `FUN_00508660`; `regiments_changed` queues
  the event at every port site that adds, moves or removes a regiment at a
  system: loading, unloading, a regiment's own move and arrival, a finished
  build, scrap, a kill by a bombardment, an assault, Destroy System or
  sabotage, an uprising incident's loss) and the populated bit's hook
  (`FUN_00510e30`, Destroy System). No pass reruns it on a clock; the
  seeding leaves every system's holder as `FUN_00508250` would. A kill
  whose shift turns a system, or a load that takes a system's last
  regiment, changes its holder in that step. The events are served by
  priority (Event order, above): every holder rerun and holder change
  (`0x10`) before any shift (`0x100`). The support
  hook (`support_changed`) then reruns the garrison requirement
  (`FUN_0050b5a0`), drift (`FUN_0050b230`) and collection rate
  (`FUN_0050b2c0`) into `GameWorld::support_values`
  (`economy::refresh_support_values`); `port:` the daily economy pass
  also recomputes them, and timer `0x386`'s delay is checked once a tick
  (`blockade.rs`). The assault's and the kill's blockade and battle
  reruns (event `0x311`, `0x400`) run after a `fire` that drains every
  event below them, and a second `fire` serves what they post.
- **The owner change's child walk** (`FUN_00510820` → `FUN_004fae40`, slot
  `+0xb0`; `ground-assault.md`, section 4): `support_shift` releases a
  prisoner whose captor's side differs from the new holder, flags each
  character still of another side (the autoscrap request, taken as the
  adrift flag), and turns or removes each special force by its class's
  sides. `death_star::sweep_requests` (event `0x307`) then sweeps each
  system holding a flagged character, which leaves for the nearest system
  of its side unless the battle bit is set.
- **No other support write.** The drift `FUN_0050b230` -> `FUN_00559c40`
  goes to `+0x6c` through `FUN_00509ec0`, the output diversion roll's odds
  (`FUN_005166a0`), never to `+0x58` (`SupportValues::support_drift`).
  Timer `0x384` costs a revolting holder GNPRTB 7697 support every 7698 days
  (`uprising-incident.md`).

- **Control change.** Event `0x31b`'s handler (`FUN_00517aa0`,
  `FUN_0055de50`) posts the change's shift with the `0x30`-bit variants (the
  troop bits of the old and the new word) and the per-step cap
  (`ControlChangeCap`, `FUN_00518040`), then Coruscant's own shift
  (`FUN_0055e010`). The cap's step is `support_shift::step(day, in-day
  step)`: the day's own work (timers, builds, arrivals, missions) takes the
  day's first step (0); an AI order takes the in-day step whose `ai_step`
  applies it (`StrikeStates::step`). `hyp:` the player's order takes the scheduler's
  next step (`Scheduler::next_step`): its command waits in queue `+0xac`
  for that step's phase 5, before the step's AI round. `port:` the victory
  rerun at Coruscant is the per-tick `VictorySystem::check`. Both sides'
  regiments keep the troop bits and so the earlier holder
  (`FUN_0050b610.c:45-70`, `economy::resolve_system_control`): a landing
  beside the holder's regiments posts nothing.
- **Blockade end.** `blockade_ended` (`FUN_00517ee0`, `FUN_0055dbf0`) reads
  `BlockadeState`'s loss record: `start_losses` (`FUN_00511300`) zeroes A and
  B and stores the blockader as the bit sets, and `record_loss`
  (`FUN_00508c60`) adds a destroyed unit's value by reason. Counter B is fed
  by a bombardment's regiment kills, the bombarding ships its batteries
  destroy and the squadrons and regiments aboard them (the ship's hook
  `+0x11c`, `FUN_00502840` → `FUN_004fb520`, passes its reason byte to
  each child through slot `+0xb4`, `FUN_004f8710` → `FUN_004f7ed0`), and
  Destroy System's regiments and squadrons; counter A by a planetary
  assault's regiment kills, attackers and defenders alike
  (`assault::resolve`, reason `0xe`). The strength sums take existing,
  completed objects (`+0x50` bits `0x40` and `0x4`, `FUN_00517ee0.c:31`)
  whatever their transit bits, so an ordered capital ship counts from its
  completion, and one on its own leg (bit `0x20`) counts. `port:` a
  regiment or squadron exists only once complete, and squadrons in transit
  count. The `blockade_ended` telemetry record carries the blockader, both
  loss totals, both strength sums and the record's direction and amount
  (`blockade_end_record`); the blockader and the record are null
  when none was made.
  `port:` the sums and counters are read at the day's blockade step, after
  its missions and events; the original reads them as the bit clears
  (`FUN_00517ee0`).
- **HQ removal.** `remove_headquarters` (`FUN_00517cd0`, `FUN_0055e1f0`) on a
  bombardment hit, Destroy System and a capture (the owner change).
  `port:` Coruscant's `is_headquarters` is the Empire seat and never posts it.
- **Bit 3** is `GameWorld::shift_excluded`: set at load for every system
  whose SYSTEMSD family is not core (`0x90`-`0x91`), cleared by
  `refresh_exclusion` when a control change leaves the system held and
  populated. The family is `GameWorld::system_family` (SYSTEMSD; a system's
  `DatId` carries no family byte).
- **Bombardment.** `bombardment::finalize` posts its sector or galaxy record
  (GNPRTB `0x1401`-`0x1404`, SDPRTB `0x1408`-`0x140b`) like the others.
- **Timer `0x386`** is `blockade.rs` (`advance_support_shift`), applied at
  once through `apply_support_change`.
- **Telemetry.** Every fired shift records `support_shift` (cause, amount,
  delta, Alliance support after). `port:` the cause is the port's; event
  `0x373` carries none.

`+0x88` bit 11 is `GameWorld::death_star_sectors`, refreshed by
`death_star::refresh_sectors` wherever a Death Star's usable bit changes
(event `0x301`, `FUN_00514510.c:149-158`): its completion, a fleet's
departure and arrival, Send Ships (`fleet_join`), and its loss in battle, to
batteries, to scrap or to sabotage; and after a load or a new game. It holds
the sectors where a fleet carrying the Death Star sits at a system, not in
transit (`FUN_005178b0`).
It gates the halving in `uprising.rs` (`support_change`), the Empire's
doubled regiments in the drift (`economy.rs`) and the uprising score, and
the Empire's halved garrison.

## Open

- What reads counter C (`FUN_00517270`, `FUN_00560d00`) and which removals
  carry reason `0xc`; the producers of reason `0xe` beyond those three
  callers.
- Whether the destroyed HQ's event `+0x44` container resolves to its system
  (`FUN_0053ff90`, slot `+0xc` at `0x004f6a80`), which decides the HQ shift's
  sector when the HQ is removed by a capture or a cascade.

## Decompiles

`FUN_00562ae0.c`, `FUN_00562ba0.c`, `FUN_00518ac0.c`, `FUN_00518b20.c`,
`FUN_00518be0.c`, `FUN_00562fb0.c`, `FUN_0050c9f0.c`, `FUN_00559be0.c`,
`FUN_00507270.c`, `FUN_005178b0.c`, `FUN_005061f0.c`, `FUN_00518690.c`,
`FUN_00559b60.c`, `FUN_0055cb60.c`, `FUN_00558bb0.c`, `FUN_0053e390.c`,
`FUN_0053e3e0.c`, `FUN_00515c00.c`, `FUN_00558880.c`, `FUN_00517aa0.c`,
`FUN_0055de50.c`, `FUN_0055de00.c`, `FUN_00518040.c`, `FUN_00506e10.c`,
`FUN_004fd330.c`, `FUN_00517be0.c`, `FUN_0055e010.c`, `FUN_00510d70.c`,
`FUN_00515f40.c`, `FUN_00511300.c`, `FUN_00508c60.c`, `FUN_00509a20.c`,
`FUN_005099c0.c`, `FUN_005099f0.c`, `FUN_00509ae0.c`, `FUN_00509ac0.c`,
`FUN_0050ce80.c`, `FUN_0050ceb0.c`, `FUN_0050cee0.c`, `FUN_00517ee0.c`,
`FUN_0055dbf0.c`, `FUN_0053e170.c`, `FUN_0053e190.c`, `FUN_0053e0f0.c`,
`FUN_00517cd0.c`, `FUN_0055e1f0.c`, `FUN_00517de0.c`, `FUN_0055e150.c`,
`FUN_005140c0.c`, `FUN_0053f950.c`, `FUN_0053f9c0.c`, `FUN_0053ff90.c`,
`FUN_00562c70.c`, `FUN_0054f4e0.c`, `FUN_0048bc60.c`, `FUN_0054ee90.c`,
`FUN_0051df30.c`, `FUN_0051dea0.c`, `FUN_004fcee0.c`, `FUN_005081f0.c`,
`FUN_0051bc70.c`, `FUN_00566d40.c`, `FUN_0056a550.c`, `FUN_0050b890.c`,
`FUN_00510820.c`, `FUN_00510e30.c`, `FUN_0051c010.c`, `FUN_0050a510.c`,
`FUN_00504dc0.c`, `FUN_004f6b50.c`, `FUN_0058e320.c`, `FUN_0058d990.c`,
`FUN_0058ce70.c`, `FUN_0055d900.c`, `FUN_0055da90.c`, `FUN_0055a0e0.c`,
`FUN_00518a70.c`, `FUN_0058cf20.c`, `FUN_0058cf50.c`, `FUN_0058d540.c`,
`FUN_0058c580.c`, `FUN_0058bec0.c`, `FUN_0058c3c0.c`, `FUN_0054ec90.c`,
`FUN_005f6250.c`, `FUN_005f61c0.c`, `FUN_0054f100.c`, `FUN_0054f140.c`,
`FUN_0048af30.c`, `FUN_0048e670.c`, `FUN_0057bd20.c`, `FUN_00562c70.c`,
`FUN_005648f0.c`, `FUN_00579fa0.c`, `FUN_0057a500.c`, `FUN_00564bf0.c`,
`FUN_0057b7e0.c`, `FUN_00563e40.c`, `FUN_0057aa50.c`, `FUN_0057bac0.c`,
`FUN_00511b30.c`, `FUN_0050ac10.c`, `FUN_004f6f40.c`, `FUN_0050a300.c`,
`FUN_0050a970.c`, `FUN_00509c30.c`, `FUN_005109f0.c`, `FUN_00510820.c`,
`FUN_0050b5a0.c`, `FUN_0050b230.c`, `FUN_0050b2c0.c`, `FUN_0050a290.c`,
`FUN_00509ec0.c`, `FUN_00509f40.c`, `FUN_00508da0.c`, `FUN_00508660.c`,
`FUN_00511740.c`, `FUN_00510f20.c`, `FUN_0050ba90.c`, `FUN_0050b500.c`,
`FUN_00520b60.c`, `FUN_00520b80.c`, `FUN_006158b0.c`.
