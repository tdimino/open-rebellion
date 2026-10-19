# Game loop, scheduler and timers

Recovered 2026-09-27 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). The decompiles cited here are saved beside this note as
`FUN_*.c`. "Inference" marks a claim no decompile states outright.

There is no daily dispatcher that calls each subsystem. The game advances a
day counter, and everything that happens on a schedule is a timer object in a
day-ordered queue. Everything else runs when a state field changes: a setter
notifies the object's views and its own hook slot, and the hook may arm a
timer or raise an event.

## Frame loop

`FUN_0040a050` is the top-level state machine (`this+8`):

- case 1 (`FUN_0040a050.c:78`) runs `FUN_0041df80`, the setup sequence;
- case 2 (`:83`) runs `FUN_0041dff0`, the per-frame game step.

### Setup sequence

`FUN_0041df80` calls `FUN_0051d800`, which calls `FUN_005136d0` once per frame
until it reports done. `FUN_005136d0` switches on the stage in `DAT_006b2bb4`
(`FUN_0051c000`), and each pass advances it by one (`FUN_0051c030` ->
`FUN_0051c010`, which stops at `0x1a`). Stage `0x17` (`FUN_005136d0.c:103`)
runs `FUN_0051b7f0`, which walks every object (`FUN_00504d50`, types
`1..0xff`) and calls its vtable slot `+0xc0` (`FUN_0051b7f0.c:34`).

For a system, slot `+0xc0` is `FUN_00508250`. The system vtable starts at
`0x0065e640` (stored by `FUN_00507130`), and `0x0065e700` holds
`FUN_00508250`. **`FUN_00508250` is a setup-stage pass, not a per-tick loop.**
Its only references are the three system-class vtables `0x0065e700`,
`0x00663758` and `0x00664158`. The only caller of slot `+0xc0` over all
objects found in the corpus is this stage.

### Per-frame step

`FUN_0041dff0` walks a phase counter `this+8` (`FUN_0041dff0.c`):

| Phase | Call | Work |
|-------|------|------|
| 0 | `FUN_0041e390` -> `FUN_0051dea0` | advance the sub-tick and, at the day boundary, the day |
| 1-3, 7-11 | `thunk_FUN_00435a40` family | session synchronisation (multiplayer lockstep; inference from the shared session object) |
| 4 | `FUN_0041e430` -> `FUN_0040f340` | returns 1, no work |
| 5 | `FUN_0041e460` -> `FUN_0051df30` | run one due scheduler item per frame until none is due |
| 6 | `FUN_0041e490` -> `FUN_0051e050` | end-of-step change flags, notification flush (`FUN_0051eaf0`), `+0xbc` callback |

`FUN_0051dea0` (`FUN_0051dea0.c`) increments the sub-tick count `+0xc` and
the in-day count `+0x20`. When the previous step reached the per-day count, it
clears `+0x20` and increments the day `+0x10` (`:18`). The per-day count `+0x1c`
is the larger of the two sides' `+0xc4` (`:24`, `:27`). The game clock that
other code reads (`FUN_004fd340` -> `DAT_006b2b08` = `FUN_004fcf00`) returns
this day `+0x10`. That resolves the open `DAT_006b2b08` item in
`build-delivery.md`.

`FUN_0040a050` message 9 calls `FUN_0041e270` -> `FUN_0051ddf0`. That runs
`FUN_00516ef0` and `FUN_00517270` for one location (`+0xc0` object `+0x8c`),
draining the scheduler after each. Both walk that location's fleets per side
(`FUN_004ffef0`, `FUN_00503a50`). Which message sends 9, and what the two
passes decide, is not traced.

## Scheduler

`FUN_0051ce00` returns the scheduler object. `FUN_004fcde0` installs its
hooks:

| Hook | Function | Effect |
|------|----------|--------|
| `DAT_006b2ad8` (`FUN_004fd350`) | `FUN_004fcfd0` -> `FUN_0051d8a0` | timer `+0x40` = day + delay (`FUN_0051d8a0.c:6`), insert into queue `+0xa8` |
| `DAT_006b2b00` | `FUN_004fcff0` -> `FUN_0051d8d0` | insert into queue `+0xac` |
| `DAT_006b2af8` | `FUN_004fd010` -> `FUN_0051d8f0` | insert into queue `+0xb0` |
| `DAT_006b2ad4` | `FUN_004fd030` -> `FUN_0051d910` | dispatch on the item's slot `+0x10` kind 1..4 |
| `DAT_006b2b08` | `FUN_004fcf00` | day `+0x10` |
| `DAT_006b2b04` | `FUN_004fcee0` | sub-tick `+0xc` |

`FUN_0051df30` serves the queues in the order `+0xb0`, `+0xa8`, `+0xac`
(`FUN_0051df30.c:23-36`). When no queue is due, it pops one notification from
`+0xa4` and runs it through `FUN_0051dd20` (`:40`). It returns 0 after running
an item, so phase 5 of `FUN_0041dff0` repeats until nothing is due, and the day
advances only in a later step's phase 0.

The timer queue `+0xa8` (`FUN_0051e320.c:55-69`, `FUN_0054ed40`, vtable
`0x00661fd8` over the base `FUN_0054ea70`) is a binary tree ordered by `+0x40`
(slot `+0x8` `FUN_0054ed90`: strict `<`, so equal days run first-in first).
Its slot `+0x1c` `FUN_0054edc0` returns the head unless the day
(`FUN_004fd340`, scheduler `+0x10`) is below the head's `+0x40`: an item is
due once its day is reached. A timer armed with delay 0 during the drain is
therefore due at once and fires the same day, behind the items already due.
`FUN_0054ec70` runs slot `+0x1c` and, when due, `FUN_0054ec90`.
`FUN_0054ec90` pops the head and calls its slot `+0x20`. If that returns true it
reinserts the item (`FUN_0054ece0`, `:16`); otherwise it deletes it (`:19`).

## Timers `0x380..0x396`

`FUN_0051ef80` registers 73 id -> factory pairs with `FUN_0054f140` in the
table at `DAT_006b92f0`, and `FUN_0054f100` builds an object by id. Three id
bands use it:

- `FUN_0053fcf0`: notifications `0x100..0x242`;
- `FUN_0053fbd0` (and `FUN_0053f9c0`): events `0x300..0x37f`;
- `FUN_0053fb00`: timers `0x380..0x396` (`FUN_0053fb00.c:18`).

`FUN_0041e580` fills a second table with `FUN_0051f770` (`DAT_006b2fd8`) for
UI message factories.

### Arming

`FUN_0053fa60` and `FUN_0053fab0` change a two-word state record on the object
and arm the timer only if the record changed and its bit 0 is set:

- record word 0 is a change counter;
- bit 0 is armed (`FUN_00540200`);
- bits 1..15 are the delay spread (`FUN_00540270`, capped by `DAT_00661a8c`);
- bits 16..31 are the minimum delay (`FUN_00540230`, capped by `DAT_00661a90`).

`FUN_0053fb00` builds the timer and stores the target key at `+0x3c`, a copy
of the record at `+0x44`, and the context at `+0x20`. It schedules the timer
after `FUN_00586130`:

```
delay = rand(0..spread) + min    // FUN_00586130.c:7-8, FUN_0053e290
```

That `FUN_0053e290(n)` draws uniformly from `0..=n` follows `uprising.rs`
(event `0x38d`); `FUN_005f5700` was not re-read here.

### Firing

Every timer class shares the base vtable `0x0066a230` (constructor
`FUN_00585fe0`). Slot `+0x20` is `FUN_005862a0`:

```
if target resolves and target.record.counter == timer[+0x44].counter:  // :24
    slot +0x30 (fire)                                                  // :27
    timer[+0x40] += rand(0..spread) + min                              // :28-30, repeats
```

A timer whose target record changed since arming does nothing and is deleted.
That is how a timer is cancelled. Otherwise it fires and reschedules itself
with a fresh random delay. Slot `+0x24` returns the id, slot `+0x28` the
target type range (`FUN_00586150` checks the target key's family byte against
it), slot `+0x2c` resolves the target's record, and slot `+0x30` acts.

| Id | Vtable | Fire (`+0x30`) | Target range | Action | Armed by | Delay record, GNPRTB (shipped) |
|----|--------|----------------|--------------|--------|----------|------------------|
| `0x380` | `0x00669870` | `FUN_0057c2c0` | `0x14..0x20` | target slot `+0x228` | `0x557c17`, `0x557c4b` (unnamed) | not traced |
| `0x381` | `0x006694b8` | `FUN_005783b0` | `0xf3..0xf4` | `FUN_00530460`: sum `FUN_00528040` over three per-system lists of every same-side system, store side `+0x90/+0x94/+0x98` | `FUN_00532350` | `+0xd4`: min 7169 (30), spread 7170 (30) |
| `0x382` | `0x00669480` | `FUN_00578060` | `0xf3..0xf4` | `FUN_00530350`: when side `+0x58` < `+0x70` or `+0x74`, set Locked (bit 13, `FUN_004f7950`) on one random same-side existing, completed object passing `FUN_004f2990` | `0x532a07` (unnamed) | `+0xcc`: min 7168 (10) (inference: `+0xcc` is the only other record `FUN_00532350` sets) |
| `0x383` | `0x006634d8` | `FUN_00565400` | `0x90..0x98` | `FUN_0050ca80`: if system `+0x88` bit 2, clear it (`FUN_0050a4a0(0)`) | `0x5116f6` (unnamed) | not traced |
| `0x384` | `0x00663510` | `FUN_00565730` | `0x90..0x98` | `FUN_0050caa0`: if `+0x88` bit 2, `FUN_0050c9f0(GNPRTB 7697 (-1), side)` | `FUN_00510f20` | system `+0x54`+`0x20`: min 7698 (30), set by `FUN_0050e200` |
| `0x385` | `0x00663548` | `FUN_00565a60` | `0x90..0x98` | `FUN_0050cad0`: if `+0x88` bit 8, side bits = 2 and not bit 2, `FUN_0050c9f0(GNPRTB 7691 (0), side)` | `0x511690` (unnamed) | not traced |
| `0x386` | `0x00663580` | `FUN_00565d90` | `0x90..0x98` | `FUN_0050cb20`: blockade support shift, below | `FUN_00511300` (blockade bit), `FUN_005109f0` (support) | system `+0x54`+`0x30`: min 7694 (30) or 7696 (30), no spread |
| `0x387` | `0x00669838` | `FUN_0057bf80` | `0x01..0xff` | `FUN_004f7fc0`: arrival; if Autorouting, clear it and the arrival tick (`build-delivery.md`) | `FUN_004f8010`; cancelled by `FUN_004fba10` | object `+0x54`+`0x10`; delay = travel ticks |
| `0x388` | `0x00669900` | `FUN_0057cca0` | `0x30..0x3c` | `FUN_004ef100`: injury recovery. When character injury `+0x94` (`uprising-incident.md:141`) != 0, reduce it by GNPRTB 2563 (2) or 2564 (1), chosen by `+0xac` bit 2, through `FUN_0053e120`, and store it with `FUN_004ee2d0` (clamped `0..0x7fff`) | `0x4f1b18`, `0x4f1b57` (unnamed) | not traced |
| `0x389` | `0x00669938` | `FUN_0057cff0` | `0x30..0x3c` | `FUN_00560a30` (149 lines, not read) | `0x4f1b87` (unnamed) | not traced |
| `0x38a` | `0x00669970` | `FUN_0057d330` | `0x30..0x3c` | `FUN_004ef150`: if `+0xac` bit 8, pulse `+0xac` bit 10 on then off (`FUN_004eebf0`) | `0x4f1bb7` (unnamed) | not traced |
| `0x38b` | `0x00661e50` | `FUN_0054d6c0` | `0x40..0x80` | `FUN_00522a10`: if `+0x68` == 8, set `+0xa4` bit 0 (`FUN_00522130`) | `FUN_00524b70` (twice) | object `+0x54`+`0x20`, not traced |
| `0x38c` | `0x00661e88` | `FUN_0054da10` | `0x40..0x80` | `FUN_00522a30`: if `+0x68` == 11, own slot `+0xac(0x14)` | `FUN_005229c0` (from `FUN_00522fc0`, `FUN_00524b20`) | object `+0x54`+`0x28`, not traced |
| `0x38d` | `0x006635b8` | `FUN_005660c0` | `0x90..0x98` | `FUN_0050cb80`: uprising incident on, then off (`FUN_0050aa50`) | `FUN_00510f20` | GNPRTB 7701 (30) + rand 7702 (70) (`uprising.rs`) |
| `0x38e` | `0x006635f0` | `FUN_00566410` | `0x90..0x98` | `FUN_0050cbe0`: when held (side bits != 3) and `FUN_00559f90` passes, informant incident on, then off (`FUN_0050aac0`) | `FUN_0050c820` (from `FUN_00510820`, `FUN_0050e200`) | object `+0x54`+`0x40`: 7703 (30) + rand 7704 (970), set by `FUN_0050e200` |
| `0x38f` | `0x00663628` | `FUN_00566760` | `0xf9..0xfa` | `FUN_00556b50`: disaster on one random existing system (`FUN_0050cdc0`) | `FUN_00556fa0` | GNPRTB 7717 (1) + rand 7718 (399) |
| `0x390` | `0x00663660` | `FUN_00566a90` | `0xf9..0xfa` | `FUN_00556be0`: resource incident on one random existing system (`FUN_0050cc70`) | `FUN_00556fa0` | GNPRTB 7719 (1) + rand 7720 (499) |
| `0x392` | `0x00661f48` | `FUN_0054e620` | `0xf8..0xf9` | `FUN_00546f80` -> `FUN_00543af0(1)`: set `+0x58` bit 0 | `FUN_00549d60`, `FUN_0054a800` | GNPRTB 6146 (300) + rand 6147 (100) |
| `0x393` | `0x00661f80` | `FUN_0054e970` | `0xf8..0xf9` | `FUN_00546fa0` (90 lines, not read) | `0x54b754` (unnamed) | `+100`: GNPRTB 6148 (300) + rand 6149 (300) (inference: set beside `0x392` in `FUN_00549d60`) |
| `0x394` | `0x006694f0` | `FUN_00578700` | `0x28..0x30` | `FUN_0053aa20`: if `+0x58` == 2, `FUN_0053a8d0(1)` sets `+0x60` bit 2 | `FUN_0053b330` | object `+0x54`+`0x18`; min from `+0x20` |

Ids `0x391`, `0x395` and `0x396` have no factory in `FUN_0051ef80` and no
arming site.

The family-byte names are inferences:

- `0x90..0x98`: systems (the port's SYSTEMSD `family_id`)
- `0x30..0x3c`: characters (`decoy-roll.md`)
- `0x14..0x20`: ships and fighters (`decoy-roll.md`)
- `0x40..0x80`: missions (the `FUN_00525bb0` mission iterator region)
- `0xf3`/`0xf4`: the two sides
- `0xf8`, `0xf9`: galaxy-level managers

`entity-system.md` line 383 gives conflicting ranges (`0x30–0x40` = ships),
which contradict both iterator recoveries.

### Timer `0x386`: the blockade support shift

Recovered 2026-10-16 (Ghidra 12.1.4 headless, read-only). Support is system
`+0x58`, the Alliance's share; `FUN_00507270(side)` reads it per side
(`support-shifts.md`). The blockade bit is `+0x88` bit 5 (`blockade-bit.md`).

**The record.** The timer's record is system `+0x54`+`0x30`. `FUN_0050e200`
(called by `FUN_0056a7d0`) sets its minimum to GNPRTB 7694
(`FUN_0050e200.c:50`) and never sets its spread, and no other caller of
`FUN_00540270` writes `+0x30`, so the delay is exactly the minimum
(`FUN_00586130.c:7-8`). Shipped: 7694 = 30, 7696 = 30.

**Arming sites.** Both run only when `DAT_006b90e0` is set, which
`FUN_0051c010` sets exactly when the setup stage is `0x1a`
(`FUN_0051c010.c`); during setup neither arms.

- `FUN_00511300`, system slot `+0x220` in all three system vtables
  (`0x0065e860`, `0x006638b8`, `0x006642b8`), the hook of the blockade bit's
  setter `FUN_0050a5f0`. It raises event `0x305` (`FUN_00511300.c:99`) and
  calls `FUN_0053fa60(0x386, new bit, …)` (`:106`), which writes the
  record's armed bit (`FUN_0053fa60.c:10`). On a change to 1 it arms a timer
  with the record's current minimum (`:12`, `FUN_0053fb00`): the value
  `FUN_005109f0` last wrote during an earlier blockade, else 7694. It does
  not re-test the majority.
- `FUN_005109f0`, system slot `+0x1d0` (`0x0065e810`, `0x00664268`; the
  third class's slot `+0x1d0` is `FUN_00567110`, which calls it first). Its
  only caller is the support setter `FUN_00509c30`, which calls slot
  `+0x1d0(old, new, ctx)` whenever `+0x58` changes (`FUN_00509c30.c:28-36`).
  It is the support-change hook: it reruns control (`FUN_0050b610`, which
  reads `FUN_0055a080(+0x58)`, GNPRTB 7760), the garrison requirement
  (`FUN_0050b5a0`), the drift (`FUN_0050b230`) and the collection rate
  (`FUN_0050b2c0`). Then, only with bit 5 set (`FUN_005109f0.c:36`), it
  picks the minimum: 7694 when the blockading side holds the majority of
  support, else 7696 (`:37-47`), and calls `FUN_0053fab0` (`:48`). That
  writes the minimum (`FUN_00540230`) and arms a new timer only when the
  value changed and the armed bit is set (`FUN_0053fab0.c:12`).

**Firing and re-arming.** The class's slot `+0x20` is the base
`FUN_005862a0` (vtable `0x00663580` read at `+0x20`), so `0x386` is
periodic: when the record's change counter still equals the copy taken at
arming (`FUN_005862a0.c:25`), it fires slot `+0x30` (`:27`) and adds the
same delay again (`:28-30`) and is reinserted. Fire (`FUN_00565d90`)
resolves the system (`FUN_00565d90.c:27`) and calls `FUN_0050cb20(ctx)`
(`:35`).

**Disarming.** Every write that changes the record bumps its counter
(`FUN_00540200.c:12`, `FUN_00540230.c:10`). A pending timer whose copy no
longer matches is deleted at its due day without firing
(`FUN_005862a0.c:25`, `FUN_0054ec90`). So:

- the blockade bit clearing (`FUN_00511300` with 0) disarms it;
- a changed minimum while blockaded replaces it with a fresh timer from that
  day; an unchanged one leaves the running timer alone;
- arming never stacks: at most one timer per system matches the record;
- what a destroyed system does to a pending timer is not traced (slot
  `+0x1c`, `0x00586290`, is not read).

A control change has no direct path to the record; it reruns the blockade
rule (`blockade-bit.md`), and only a resulting bit change disarms.

**The shift (`FUN_0050cb20`).** At fire time, with bit 5 still set:

- `FUN_00509890` returns the side bits of the first active fleet at the
  system (`FUN_00509890.c:19-26`, iterator types `0x08..0x0f`), the
  blockading side, since a blockaded system holds only one side's active
  fleets;
- `FUN_00509930` returns the majority side of support: 1 when the Alliance
  has more, 2 when the Empire has more, 3 on a tie (`FUN_00509930.c`);
- `FUN_00509980` sets its flag when the two are equal
  (`FUN_00509980.c:14-15`).

The delta is GNPRTB 7693 (+1) when the blockader holds the majority, else
7695 (-1), applied to the blockader's own support
(`FUN_0050cb20.c:15-21`). `FUN_0050c9f0` requires a populated system
(`+0x88` bit 0) and a side of 1 or 2 (`FUN_0050c9f0.c:11`), divides a
change toward the Alliance by GNPRTB 7681 (2) when bit 11 (Death Star in the
sector) is set (`:23`, `FUN_00559be0.c`; integer division, so ±1 becomes 0),
clamps the side's support to `0..100` (`:24-25`) and stores it through
`FUN_00509c00`/`FUN_00509c30` (`:26`). A blockader with the majority gains a
point per period; one without it loses one.

The shift raises no event or notification of its own. `FUN_00509c30` copies
the value to both side views (`FUN_0050f390`, whose hook is slot `+0x25c`)
and calls the support-change hook above, so a shift can change control
through `FUN_0055a080` (GNPRTB 7760, 60) in the same call.

**In the original (VM).** From `empire-day53-yaga-orbit.sav`, the Empire
blockades Alliance-held Yaga Minor (DatId `0x9000010d`) without the
majority. The saved timer and Alliance support, read from later saves of
the same game:

| Day | Alliance support | Timer due | Counter copy |
|-----|------------------|-----------|--------------|
| 53 | 67 | 70 | not read |
| 75 | 68 | 100 | 2 |
| 106 | 69 | 132 | 3 |

Each fire moves the Empire's support down a point, so the Alliance's rises.
The day-106 timer carries a new counter copy and is due 32 days after the
day-100 fire, so the record changed around day 102 and a fresh timer armed
then. With the minimum unchanged (7694 = 7696 = 30), the likely change is
the blockade bit clearing and setting again; the save does not show which.

The port runs it in `blockade.rs` (`advance_support_shift`,
`agent_docs/systems/blockade.md`).

## `FUN_004927c0` and message `0x1f0`

`FUN_004927c0` is not a master tick and does not run the AI:

- **What it handles:** message `0x1f0` (`FUN_004927c0.c:106`). It picks string
  ids `0x412`/`0x47c` (`:114-115`) or `0x415`/`0x480` by the DatId family of
  `param_2+0x20` (`0x51..0x64`), then loads strings with `FUN_004c4990`
  (twelve calls, from `:1995`).
- **What `0x1f0` is:** a notification sent only by `FUN_00525040` (send site
  `0x525061`, through `FUN_0053fcf0`), in the mission-object code. It is
  registered only as a UI factory, `FUN_0051f770(0x1f0, FUN_0043c9f0)`
  (`FUN_0041e580.c:58`).
- **Where the misread sits:** `agent_docs/ghidra-re.md:155` and
  `agent_docs/systems/ai-parity-tracker.md:52,92,123` call it a day tick that
  drives the AI. Nothing supports that.

## Notification and event senders

`FUN_0053fcf0` send sites cover notifications `0x100`, `0x120..0x128`,
`0x140..0x156`, `0x160..0x162`, `0x1a0`, `0x1c0..0x1c6`, `0x1e0..0x1e5`,
`0x1f0` and `0x1f1`. All have UI factories in `FUN_0041e580`.

`FUN_0053f9c0` send sites cover events `0x300..0x31d`, with factories in
`FUN_0051ef80`. Among them:

- `0x300`: `FUN_004fb810`
- `0x303`: `FUN_00510f20`
- `0x305`: `FUN_00511300`
- `0x306`: `FUN_004fba10`, route cancel
- `0x319`: `FUN_00510950`

The event factories' fire slots are not decompiled.

## Open

- Which timers use system records `+0x18` (GNPRTB 7699 (20) + rand 7700
  (20)) and `+0x28` (7692 (1000)), both set by `FUN_0050e200`; hyp: `0x383`
  and `0x385`. The record delays for `0x380`, `0x388..0x38c` and `0x394`.
- `FUN_00560a30` (`0x389`) and `FUN_00546fa0` (`0x393`).
- Target slot `+0x228` for `0x380`.
- The meaning of side `+0x58/+0x70/+0x74` (`0x382`) and system `+0x88`
  bits 2 and 8.
- Who calls the system field hooks `FUN_00510820` and `FUN_00511740`.
  `FUN_005109f0` (slot `+0x1d0`) is the support-change hook (timer `0x386`,
  above). These run support recalculation `FUN_0050b230` -> `FUN_00559c40`,
  so support is recomputed when an input changes, not on a clock.
- The queue `+0xb0` producers, and the message that sends frame case 9.
