---
title: "The System Blockade Bit"
description: "Who sets system +0x88 bit 0x20, the rule FUN_0050b8e0 applies, when it runs relative to arrivals, orders and the computer player, and why the Move confirmation branch stays unreachable"
category: "ghidra"
created: 2026-10-02
updated: 2026-10-15
tags: [blockade, system, move-order, battle-bit]
---

# The System Blockade Bit

Recovered 2026-10-02 with Ghidra 12.1.3 headless (read-only project). Every
function named here has a `FUN_<address>.c` note in this directory.

The system's `+0x88` word holds one flag per bit, each with its own setter
(`FUN_0050a3c0` for `0x1` through `FUN_0050ac10` for `0x100000`, `0x70`
bytes apart). Each setter calls `FUN_0053a640(mask, value, &+0x88)`; when the
bit changes it updates both side views (`FUN_00539fd0(this, 1/2)`) and calls a
notification slot (`+0x20c` for `0x1`, rising by 4 per bit). Bit `0x20`'s
setter is `FUN_0050a5f0` (views `FUN_0050fec0`, slot `+0x220`); its only
caller is `FUN_0050b8e0`.

## The rule (`FUN_0050b8e0`)

`FUN_0050b8e0` recomputes three bits together:

1. `owned`: the system's side bits (`+0x24` bits 6..7) are 1, 2, or 3.
   With side 0 (neutral) all three bits are written clear.
2. `battle` (bit `0x1000`, `FUN_0050a900`): both side 1 and side 2 are
   present (`FUN_00509710(this, 1)` and `(this, 2)`).
3. `blockade` (bit `0x20`, `FUN_0050a5f0`): no battle, the system is
   populated (`+0x88` bit 0) or its side is not 3, and some active fleet in
   the system (`FUN_004ffe70(this, 1)`, types `0x08..0x0f`, no side filter)
   has side bits different from the system's.
4. Bit `0x4000` (`FUN_0050a9e0`): when the blockade bit keeps its value, the
   old `0x4000`; otherwise whether the battle bit changed (old `0x1000` against
   the new battle value). hyp: a "changed this pass" flag; no reader traced.

A side is **present** (`FUN_00509710`) when the system holds an active
(`+0x50` bit 0, mode 1 of `FUN_004f6b90`) fleet (`FUN_004ffef0`, types
`0x08..0x0f`) or an active fighter squadron (`FUN_00503a50`, types
`0x1c..0x1f`) whose side bits equal it (`FUN_005131d0`: type range, mode,
then `+0x24 >> 6 & 3`).

So for a system held by side 1 or 2: blockaded exactly when an enemy fleet
is there and the holder has no active fleet or fighter squadron there. With
both present it is a battle, not a blockade. A contested system (side 3) is
blockaded by any one side's fleets alone, when populated. A neutral system is
never blockaded.

## When it runs

Slot numbers below count from the system vtable `0x0065e640` (stored by
`FUN_00507130`; `FUN_00514510` calls slot `+0xc8` with three arguments, which
fits `FUN_00508660`, not the one-argument `FUN_00508250`). hyp: `0x0065e638`,
stored by `FUN_00504c40`, is a separate small class (iterator).

| Slot | Entry | Function | Called when |
|------|-------|----------|-------------|
| `+0xc0` | `0x0065e700` | `FUN_00508250` | setup stage `0x17` only |
| `+0xc4` | `0x0065e704` | `FUN_00508440` | a child's in-container bit (`+0x50` bit 6) changes, from event `0x320` |
| `+0xc8` | `0x0065e708` | `FUN_00508660` | a child's usable bit (`+0x50` bit 0) changes, from event `0x301` (and event `0x320` for a usable child) |
| `+0x220` | `0x0065e860` | `FUN_00511300` | the blockade bit changed (`FUN_0050a5f0`) |

### Slot `+0xc0` (`FUN_00508250`)

- Its one system caller is `FUN_0051b7f0`, which walks every object of types
  `1..0xff` (`FUN_00504d50`) and calls slot `+0xc0` (`FUN_0051b7f0.c:34`).
  `FUN_005136d0` runs it once, at stage `0x17` of the setup sequence
  (`FUN_0040a050` -> `FUN_0041df80` -> `FUN_0051d800` -> `FUN_005136d0`;
  `timer-scheduler.md`). It does not run per day, per arrival or per order.
- Other `CALL [reg+0xc0]` sites: `FUN_005cfec0` (from `FUN_005caf50`),
  `FUN_005d04e0` (from `FUN_005cfec0`) and the call at `0x0043b631`. hyp: the
  first two act on tactical battle objects (float arguments, offsets
  `+0x3cc`), the third forwards a user-interface notification payload to a
  view; none was shown to reach a system.
- The same applies to `CALL [reg+0xc8]` at `0x0043b461`, `FUN_005b1f60` and
  `FUN_005b2c70` (both from `FUN_005a96b0`): hyp: not system calls.

### The recompute request: bit `0x100000`

`FUN_0050b8e0` has two callers: `FUN_00508250` and `FUN_00515ef0`.
`FUN_00515ef0` is called only by `FUN_00564e40`, slot `+0x20` of the class
with vtable `0x00663478`, built by factory `FUN_00564ce0` (constructor
`FUN_00564bf0`), which `FUN_0051ef80.c` registers as event `0x311`. The
event's `+0x3c` is the system key, its `+0x50` the flag; with the flag set
`FUN_00515ef0` runs `FUN_0050b8e0` (`FUN_00515ef0.c`).

Event `0x311` is raised only by `FUN_00511b30` (`FUN_0053f9c0(0x311, this, 1,
ctx)`, `FUN_00511b30.c`), which is the change hook of system bit `0x100000`
(slot `+0x258`, `0x0065e898`). `FUN_0050ac10` sets and clears that bit; the
hook posts the event and clears the bit again. So `0x100000` is a "recompute
pending" request, and the rule runs on the event, not at the call that asked.

`FUN_0050ac10(this, 1)` is called by four functions:

- `FUN_00508660` (slot `+0xc8`), for a fleet (types `0x08..0x0f`) and for a
  fighter squadron (`0x1c..0x1f`), whatever the flag (`FUN_00508660.c`):
  every change of a fleet's or squadron's usable bit requests a recompute;
- `FUN_00510820` (slot `+0xf0`, the side-bits change hook);
- `FUN_00510e30` (slot `+0x20c`, the hook of `+0x88` bit 0);
- `FUN_00511b30` itself, with 0, to clear it.

`FUN_00508440` (slot `+0xc4`) does not request it: a fleet joining or leaving
a system's list refreshes the counts (`FUN_0050bc60`, `FUN_0050be00`) and the
fleets' copy (`FUN_0050c0b0`) only.

### What "present" means during a move

`FUN_00509710` and `FUN_0050b8e0` read fleets through mode 1 of
`FUN_004f6b90`, which tests `+0x50` bit 0 (`FUN_004f6b90.c`). That bit is the
usable bit: `FUN_004f7b80` sets it when bits 6 and 2 are set and bit 4 is
clear (`FUN_004f7b80.c`). Bit 4 (en route) is set from bit 5, the in-transit
bit that `FUN_004f7640` writes (`FUN_004fb760` -> `FUN_004f8240` ->
`FUN_004f75d0` -> hook `FUN_004fb6a0` -> slot `+0x84` `FUN_004f7b80`).

- Move departure (`FUN_00556390`, called by `FUN_00515440` from the order's
  execute slot `FUN_00578f30`; `move-order.md`): `FUN_004f7640(1)` sets bit 5
  and, in the same call chain, clears the usable bit; the object then changes
  container at once (slot `+0xa8`: `FUN_004feca0` -> `FUN_004f8630` ->
  `FUN_004f6fd0`, which relinks the object to the destination's list and
  calls slot `+0xf4` `FUN_004faf50`). Both happen inside the order's
  execution; no later "start of movement" step exists.
- The usable-bit change runs hook `+0x110` `FUN_004ff5e0` -> `FUN_004fb310`,
  which posts event `0x301` with the object's container captured at that
  moment (`FUN_0053ff90.c`: `+0x3c` object, `+0x40` container). Event `0x301`
  (vtable `0x00669628`) fires `FUN_0057a1e0` -> `FUN_00514510`, which calls the
  container's slot `+0xc8` with the usable value as the flag. The container
  change posts event `0x320` (`FUN_004faf50.c`, vtable `0x006697d8`), which
  fires `FUN_0057ba50` -> `FUN_00514a60`: slot `+0xc4` on the old and the new
  container, and slot `+0xc8` only for an object whose `+0x50` bit 0 is set.
- The departing fleet is therefore out of the old system's list and
  unusable before event `0x311` can run (`0x311` is the last event, below), so
  it is never counted at that refresh.
- Arrival: timer `0x387` (`FUN_0057bf80`) -> `FUN_004f7fc0` clears
  Autorouting (`FUN_004f7870`) -> hook `+0x138` `FUN_004fba10` cancels the
  timer and raises event `0x306` -> `FUN_0057ac90` -> `FUN_00515a50` ->
  `FUN_00556620`, then `FUN_004f7640(0)` when no leg remains
  (`build-delivery.md`, "Arrival"). Clearing bit 5 clears bit 4, sets the
  usable bit and posts event `0x301` with the destination as container, so
  the destination's `FUN_00508660` runs with flag 1 and requests the
  recompute. The fleet sits in the destination's child list from the order
  (`FUN_004f6fd0`) and is counted only from this point.

### The event queue and the order of work

- Events (ids `0x300..0x37f`, `FUN_0053f9c0` -> `FUN_0053fcd0` -> `FUN_004fd390`
  -> `FUN_0051d8f0`) go to queue `+0xb0` (vtable `0x00662018`). The queue
  orders by the item's slot `+0x28` (`FUN_0054ee90`, strict `<`), the head is
  always due (slot `+0x1c` is `FUN_0041c210`), and an item that ties goes
  behind those already queued (`FUN_005f61c0`). Priorities: `0x301`, `0x306`,
  `0x320` and the other fleet events `0x10` (`FUN_0048af30`); `0x311` `0x400`
  (`FUN_0048e670`); `0x31a` `0x999` (`FUN_0057bd20`).
- `FUN_0051df30` serves queue `+0xb0` first, then timer queue `+0xa8`, then
  `+0xac`, and only when none ran does it take one entry of list `+0xa4`
  (`FUN_0051dd20`: slot `+0x20`, then delete). It reports done (1) only when
  all four are empty (`FUN_0051df30.c`). Frame phase 5 (`FUN_0041e460`) repeats
  it, so every due timer, every queued order and every event they raise,
  down to the recompute, finishes inside phase 5 of one frame step.
- Player and computer orders reach list `+0xa4` through
  `FUN_0048a960` (player slot `+0x2c`, run by `FUN_00436250` in phase 3):
  `FUN_0041db80` -> `FUN_004fd220` -> `FUN_0051d880` -> `FUN_00536fe0`. That is
  the only path: `FUN_0041db80` has one caller (inside `FUN_0048a960`),
  `FUN_004fd220` one, `FUN_0051d880` one.
- The computer player submits an order from phase 8 (`FUN_0041cf20` ->
  `FUN_004361a0` -> player slot `+0x24` `FUN_00485b40` -> `FUN_0048aa90` ->
  `FUN_0041cee0` -> `FUN_004360f0` -> player slot `+0x1c` `FUN_0048a920`).
  `FUN_0048a920` only appends the order to the player's own list (`this+4`);
  it executes nothing.

Frame step (`FUN_0041dff0`, phases in order): 0 advance the sub-tick, and the
day when the in-day count has reached the per-day count (`FUN_0051dea0`);
3 move queued orders to list `+0xa4`; 5 drain events, timers and orders;
6 end-of-step flags and notification flush (`FUN_0051e050`); 8 the computer
player's round, each side once; the step then wraps to phase 0.

### The case: an AI fleet arrives, then gets a Move

1. Step S, phase 0: the day becomes D, so timer `0x387` is due.
2. Step S, phase 5: `0x387` fires (arrival); events `0x306`, `0x301` and
   `0x311` follow in queue order; `FUN_0050b8e0` runs with the fleet usable
   and the system held by the other side with no usable defender, sets bit
   `0x20` (`FUN_0050a5f0`) and calls slot `+0x220` `FUN_00511300`, which
   copies the bit to the fleets (`FUN_0050c0b0`), raises event `0x305` and
   arms timer `0x386` (`FUN_0053fa60`; the target record's counter changes on
   the next flip, which cancels a pending `0x386`). Phase 5 ends only after
   all of it.
3. Step S, phase 8: the computer player sees a fleet that is usable in a
   blockaded system and may submit its Move. The order is only queued.
4. Step S+1, phase 3 hands the order to list `+0xa4`; phase 5 runs it
   (`FUN_00578f30`): the fleet becomes unusable and leaves the list, events
   `0x301` (flag 0, old system), `0x320` and `0x311` run, and `FUN_0050b8e0`
   finds no enemy fleet: bit `0x20` clears and slot `+0x220` fires again.

So the blockade bit is set, with its notification, and stays set through the
computer player's round of the arrival step and through the end-of-step
notification flush; it clears one step later. Step S+1 falls on the same day:
S is the day's first step (in-day count 0), and a day lasts the per-day count
(`FUN_0051dea0.c`: the larger of the sides' `+0xc4`, at least 4) plus one
steps (`FUN_0051dea0.c:11-30`).

hyp: that the order submitted in phase 8 reaches list `+0xa4` in phase 3 of the
next step rests on `FUN_0048a920` and `FUN_0048a960`; the key test in
`FUN_0048a960` (`+0x1c` against the item's `+0x18`, the counter
`FUN_004885f0`) is read but not tied to a day.

## Consequence for Move (`0x201`)

`FUN_00487cc0` confirms a Move when the first member's system has the
blockade bit and the member's side equals the system's
(`move-order.md`, "Choosing Move or Confirmed Move"). A freshly computed bit
cannot meet that: the member is an active fleet of the holder's side, so the
holder is present, and the enemy fleet that would blockade makes the other
side present too, which is a battle. The recompute for an arrival finishes in
the same frame phase (`FUN_0051df30`, above), so a player looking at the
screen between steps reads a settled bit. hyp: the branch is unreachable for
a fleet.

## Port

`crates/rebellion-core/src/blockade.rs` `system_is_blockaded` ports the rule
(2026-10-02): both sides' fleets make a battle; a system held by one side
(`Controlled` or `Uprising`) is blockaded by the other side's fleets; a
populated `Contested` system by either side's fleets; a neutral one never.

- presence is a side's fleets or the squadrons the system holds for it
  (`System::fighters`, 2026-10-10); port: every fleet counts as active.
  The withdraw percent counts those squadrons too
  (`blockade-troop-withdrawal.md`);
- hyp: `ControlKind::Contested` is side 3, and an `Uprising` system keeps its
  holder's side bits;
- port: the rule reruns once per tick (`BlockadeSystem::advance`), after
  arrivals, battles and landings and before the AI round
  (`crates/rebellion-data/src/simulation.rs`, `crates/rebellion-app/src/main.rs`),
  as event `0x311` settles before phase 8 above: a fleet the AI sends on
  the day it arrives blockades for that day, and the bit clears on the next
  tick after it leaves. The port holds no pending bit.
