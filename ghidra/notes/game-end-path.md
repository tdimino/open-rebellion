---
title: "Game End: The HQ's Destruction, the Advisor's Ending Script and the Return to the Menu"
description: "What removes the Alliance HQ object (Destroy System through the container cascade, a military bombardment hit, capture), the single writer of object +0x50 bit 3, the advisor reaction slots and ending codes 0x10-0x13, the computer player's progress counters, and the movie window's return to the main menu"
category: "ghidra"
tags: [victory, game-end, headquarters, death-star, bombardment, advisor, movies]
---

# Game end

Answers task-graph node `or-q-game-end-path-xs0`, from the Feature Wiki page
`executable/q-game-end-path` (Worker packet `q-game-end-path`), whose
destroy-cascade and capture claims were reread here. Builds on
`victory-conditions.md` (the win word) and `bombardment.md`. Side numbers are
the game's: 1 Alliance, 2 Empire.

## The destroyed bit

Object `+0x50` bit 3 (destroyed) has one writer, `FUN_004f7560`, called only
by slot `+0x100` (`FUN_004fb200`; the HQ's adapter is `FUN_004f2ad0`), which
the destroy-reason setter `FUN_004f71d0` calls with the new reason byte. Every
destroy reaches `FUN_004f71d0` through slot `+0xac(reason, ctx)` (generic
`FUN_004f8660`, system `FUN_005080d0`) or the disaster loop `FUN_00511930`.

## What removes the Alliance HQ

The HQ is family `0x20` (ALLFACSD `0x20000001`, vtable `0x0065f280`).

| Path | Reason | Removes the HQ |
|---|---|---|
| Destroy System (`0x223`): `FUN_0058d990` → `FUN_0058ce70(system)` → system slot `+0xac`. The system's hook `+0x11c` (`FUN_00510950` → `FUN_004fb520`) calls slot `+0xb4` on every child: `FUN_004f8710` passes a completed child (`+0x50` bit 2) not flagged `0x20` to `FUN_004f7ed0`, which calls the child's `+0xac` with the system's own reason byte | `0xf`, or `0x10` for the bombarder's own system | Yes, in the same call chain. A child not completed gets `+0x50` bit `0x1000` instead (`FUN_004f78e0`) |
| Military bombardment hit (`0x220`, `0x222`, `0x223`): the walker `FUN_0058cba0` visits family `0x20`-`0x21` through `FUN_00526d70`; a successful roll calls `FUN_0058d020` → `FUN_0058ce70(target)` (`bombardment.md`) | `0xf` / `0x10` | Yes. `0x221` walks only `0x28`-`0x2f` |
| Control change: each object's slot `+0xb0` (`FUN_004f8680`) hands a completed object to the new side when its class serves that side (`FUN_004f6f40`), else calls `+0xac(3)` | `3` | Yes when the system turns Empire or neutral: the HQ class serves only the Alliance (`facility-ownership.md`) |
| Scrap `0x200`, the disaster loop | `0x14` / `0xb` | No: the HQ's order list has no `0x200` (`FUN_00526c00`) and the disaster loop walks only `0x22`-`0x2f` |

### An HQ in transit

An HQ is en route while its `+0x50` bit 5 (`0x20`) is set (the setter
`FUN_004f7640`, which writes both side views and calls slot `+0x124`; the
Move leg, `move-order.md`). `FUN_00526860` is not capture code: it builds the
HQ order list `DAT_006b8ff8` (`FUN_00526c00`: `0x201`, `0x202`, `0x204`) and
runs `FUN_00520600` on it.

Both child hooks open with the same test, `FUN_004f8710` (slot `+0xb4`, Destroy
System) and `FUN_004f8680` (slot `+0xb0`, the control-change walk of
`FUN_004fae40`): a child that is not completed (bit 2 clear) or is en route
(bit `0x20`) skips the destroy and owner-handover branches and, while its
destroyed bit 3 is clear, only gets the autoscrap request (`FUN_004f78e0(1)`:
`+0x50` bit `0x1000`, event `0x307`, `death-star-strategic.md`). So an en route
HQ is not destroyed and keeps its owner when its system is destroyed or
captured. Event `0x307` runs `FUN_00515480`: the HQ's container is the
destination system (family `0x90`-`0x97`), so the retreat chain `FUN_005545e0`
searches for a system whose side the HQ class serves (`FUN_005571c0`,
container acceptance `FUN_00553410`: a destroyed container is refused with
`0x22`, a container of another side with `0x24`). The HQ moves to the first
accepting Alliance system, the transit origin among them; with none left it
is destroyed with reason 2. A system captured or destroyed while the HQ moves
away from it does not touch the HQ, which is a child of its destination only.
An HQ standing in a captured system is not en route and dies with reason 3
(table above), as does the one standing in a destroyed system.

Only an Empire hull can Destroy System: CAPSHPSD has one family `0x18`
record (id `0x88`, Empire only) and `FUN_00582440` requires family
`0x18`-`0x1b` for order `0x223`.

The destroy reaches the win word in the same step: `FUN_004fb520` first
clears the existing bit (slot `+0x88`, `FUN_004f7bc0`; `FUN_004f76b0(0)`),
whose hook `+0x128` (HQ `FUN_005582d0` → `FUN_004f2bb0` → `FUN_004fb810`;
system `FUN_005109a0` → `FUN_004fb810`) raises event `0x300`. Its fire slot
`FUN_0057a730` → `FUN_005140c0` calls `FUN_00517cd0` for types `0x20`-`0x21`
and `FUN_00517bb0` for systems; both reach `FUN_005180e0`, where a missing or
destroyed HQ gives AD (`victory-conditions.md`). Destroying Coruscant sets CD,
which counts for the Alliance's Coruscant objective.

## The advisor's ending

- The local player's slot `+0x74` (`FUN_00486890`) passes the word to the
  advisor when GO is set. The Alliance advisor (`FUN_004c3010`) stamps slot
  87 for EW and 86 for AW; the Empire's (`FUN_004c0f10`) slot 84 for EW and
  85 for AW, each at the clock + 40000. `FUN_0041d810` then stops the
  strategic window from posting `0x15e`.
- Each slot holds two always-played records (C3POACT.SPT, IMP22ACT.SPT): a
  command 4 script (sound and animation, no text) and RCDATA `10000`/`11000`,
  step `0x11` with p2 3 and p3 1. The step's finish (`FUN_004124b0`,
  `FUN_00412400` → `FUN_0041d140` → `FUN_00436620` → `FUN_004878d0` →
  `FUN_00439eb0`) sets `DAT_006b28c4` to 3. The scripts play on the agent
  droid only (C-3PO, IMP-22).

  | Side | Slot | Word | Script | Wave |
  |---|---|---|---|---|
  | Alliance | 86 | AW | ALSPRITE 10094 | 1094, 3.07 s |
  | Alliance | 87 | EW | ALSPRITE 10093 | 1093, 1.59 s |
  | Empire | 84 | EW | EMSPRITE 11094 | 1594, 5.26 s |
  | Empire | 85 | AW | EMSPRITE 11093 | 1593, 2.58 s |

  The x094 script is the victory announcement on both sides (ALSPRITE 10094
  Alliance, EMSPRITE 11094 Empire; `victory-flag-word.md`, 5).

- The next advisor pass (`FUN_004c0a60` Empire, `FUN_004c2b20` Alliance) sees
  action 3 and runs `FUN_004c0f60`/`FUN_004c3060`, which pass the side
  object's word (`FUN_004f3dd0`) to `FUN_0041d830` → `FUN_00425c90`. That
  writes the ending code at strategic window `+0xa8`: Alliance viewer EW
  `0x11`, AW `0x10`; Empire viewer EW `0x12`, AW `0x13`.

## The computer player

Slot `+0x74` of vtable `0x0065abf0` is `FUN_00485ea0`, which calls
`FUN_004189b0(this + 0x84, word)`. That ORs 2 into the AI object's `+4` and
recounts `+0x190` (own progress) and `+0x194` (the opponent's): for side 1,
CA or CD, DC and EC against AD, LC and MC, and the reverse for side 2. It
plays nothing and stops nothing. `FUN_00435bc0` builds the computer player
with side 2 when the local player is the Alliance and 1 when it is the
Empire (`FUN_004857a0` → `FUN_00417cb0`).

## Event 0x300 and what it runs

`FUN_004f76b0(value)` writes object `+0x50` bit `0x40` (active) and calls slot
`+0x128` with the value; for the HQ that reaches `FUN_004fb810`, which raises
event `0x300` (`FUN_0053f9c0` → `FUN_0053f950`). The event's fourth argument
(its `+0x50`) is that value, and its `+0x4c` records whether it was 0.
`FUN_0053ff90` fills the event's `+0x3c` (the object), `+0x40` (the object's
parent id, `FUN_00540450`) and `+0x44` (slot `+0xc` of the object, `0x004f6a80`,
a walk up the parent chain). `FUN_004f7bc0` writes 1 only while the object is
created (`+0x50` bit 1) and not destroyed (bit 3), so destroying an object
raises event `0x300` with fourth argument 0.

`FUN_005140c0` serves it: for a family `0x20`-`0x21` object with fourth
argument 0, `FUN_00517cd0` posts the HQ's sector shift (-10 Alliance support
across the sector, `support-shifts.md`) and then reruns the win word
(`FUN_005180e0`). For a system (`0x90`-`0x97`) it calls `FUN_00517bb0`, which
reruns the win word when the system is Coruscant (`+0x18 == 0x109`) and the
fourth argument is 0. A regiment's event `0x300` reaches the system's slot
`+0xd0` (`FUN_00508c60`, the blockade loss counters).

## Coruscant and the win word

`FUN_005180e0` recomputes the word from scratch each call, from the four
leaders, Coruscant, the HQ and the game type (`victory-conditions.md`):

| Flag | Set by |
|---|---|
| AD `0x1` | `FUN_00506110` (the HQ master) is null, or its `+0x50` bit 3 is set |
| CA `0x100` | Coruscant's `+0x24 & 0xc0` is `0x40` (Alliance) |
| CD `0x200` | Coruscant's `+0x50` bit 3 |
| LC `0x10`, MC `0x20`, DC `0x1000`, EC `0x2000` | the leader's `+0xac` bit 0 |
| EW `0x100000` | Standard: AD, LC and MC; Headquarters Only: AD |
| AW `0x1000000` | Standard: DC, EC and (CA or CD); Headquarters Only: CA or CD |
| GO `0x10000000` | EW or AW |

The game ends when GO is set (`FUN_00486890`, above). When EW and AW both
hold, a set AW bit stays, else a set EW bit stays, else AW. The word is stored only when it changed.

Callers: event `0x300` for the HQ (`FUN_00517cd0`) and for Coruscant
(`FUN_00517ca0`), event `0x31b` for any control word write at Coruscant
(`FUN_00517aa0` → `FUN_00517be0`, first statement, before Coruscant's own
support shift), and a leader's prisoner event (`FUN_00516220`). A Coruscant
control change therefore sets or clears CA, and Alliance control wins a
Headquarters Only game and satisfies the Standard objective's last clause the
moment the control word changes; Coruscant destroyed sets CD through event
`0x300`.

## After the ending movie

- `FUN_00414330` states `0x10`-`0x13` tear the game down (the strategic and
  tactical children, the CommMgr through `FUN_00597050`, the session through
  `FUN_00409370` → `FUN_005975a0`) and `FUN_00414830` plays movie `0x6b`,
  `0x6a`, `0x6d` or `0x6c` (MDATA 105, 106, 108, 107) through `FUN_004146e0`
  → `FUN_004213e0`.
- The movie window ends on its notifications `0x402`-`0x404`, `WM_COMMAND`
  `0xf1` or a key other than Esc, then posts `0x405` (`FUN_00421790`). The
  Smacker pump `FUN_005f7240` sends `0x402` after the last frame and `0x404`
  while Esc or the left button is down (`movies.md`, "The return path"). The
  application's handler (`FUN_004138a0`) destroys it and, for ids `0x66` and
  `0x6a`-`0x6d`, calls `FUN_00414330(3)`: the main menu (`FUN_00414a80`),
  with the campaign's fields cleared.

## Port

The three removals end in `support_shift::remove_headquarters`, which clears
`System::is_headquarters` and posts the HQ shift (`support-shifts.md`, Port);
the win word's AD and CD are `victory-conditions.md`, Port. `port:` the HQ
never travels (`System::is_headquarters`, no Move order), so the in-transit
branch above has no port counterpart; a capture or Destroy System removes the
flagged HQ at once (`support_shift::owner_changed`, `death_star::cleanup_destroyed_system`).

The ending is `agent_docs/systems/victory.md`, "The ending":
`AdvisorState::post_word` stamps the slot, cockpit step 3 stores the code in
`movies::MovieQueue`, and the movie's end returns to the main menu.

## A game saved after GO

The advisor's save (`FUN_004397a0`) writes its slot count `+0x164` and every
slot stamp of `+0x168`, with `DAT_006b28c4` (the action step `0x11` sets to
3) and `DAT_006b28c8`; the load (`FUN_00439550`) reads them back in the same
order. A game saved after the advisor stamped its ending slot therefore
loads with the stamp, and one saved after the script finished loads with
action 3; either way the next advisor passes reach `FUN_004c0f60` /
`FUN_004c3060` and the ending. The win word (root `+0x74`, the side objects'
`+0xbc`) loads with the game, and notification `0x128` posts only on a
change (`FUN_0052f600`, `FUN_005322c0`), so the load delivers no word. The
stop flag is the strategic window's `+0xac`, which its constructor
`FUN_00421c70` zeroes with the ending code `+0xa8`, so the loaded game's
clock runs until the ending.

## The movie caption

The movie window builds its caption field on `WM_CREATE` only for a
non-empty caption (`FUN_00421790`, `this[0x2a]`): a TextScrollField
(`FUN_0041ecf0`) at (4, 404), 580 by 70, font 4 (`FUN_00420550`), white
(`0x2ffffff`), control id `0xf2`, over a window-sized surface built from
bitmap `0x2b35` of module 12 and cleared. The message movies pass the
message body (`entry + 0x50`) as that caption (`FUN_00422ce0`, `0x482`
wParam 1). The window builds no other control, and its `WM_PAINT` only
validates (`BeginPaint`, `EndPaint`): the movie and the caption are all it
shows.

## Open

- Whether excess-facility removal (`FUN_0050ad80`, `FUN_004f8760`, reason 4),
  autoscrap (reason `0x15`) or sabotage (reason 6) can reach the HQ.
- Who reads the AI object's `+0x190`/`+0x194`.

## Decompiles

`FUN_004f76b0.c`, `FUN_0053f9c0.c`, `FUN_0053f950.c`,
`FUN_0053ff90.c`, `FUN_00540450.c`, `FUN_005180e0.c`, `FUN_00517be0.c`,
`FUN_00517ca0.c`, `FUN_00517aa0.c`, `FUN_00508c60.c`,
`FUN_004f7560.c`, `FUN_004fb200.c`, `FUN_004f2ad0.c`, `FUN_004f71d0.c`,
`FUN_004f8660.c`, `FUN_005080d0.c`, `FUN_004fb520.c`, `FUN_00510950.c`,
`FUN_004f8710.c`, `FUN_004f7ed0.c`, `FUN_004f8680.c`, `FUN_004f8760.c`,
`FUN_00526e60.c`, `FUN_00526860.c`, `FUN_004f7bc0.c`, `FUN_004f76b0.c`,
`FUN_004fb810.c`, `FUN_005582d0.c`, `FUN_004f2bb0.c`, `FUN_005109a0.c`,
`FUN_0057a730.c`, `FUN_005140c0.c`, `FUN_00517cd0.c`, `FUN_00517bb0.c`,
`FUN_00582440.c`, `FUN_0058d990.c`, `FUN_0058ce70.c`, `FUN_0058cba0.c`,
`FUN_00526d70.c`, `FUN_0058d020.c`, `FUN_00486890.c`, `FUN_004c3010.c`,
`FUN_004c0f10.c`, `FUN_0041d810.c`, `FUN_004124b0.c`, `FUN_00412400.c`,
`FUN_0041d140.c`, `FUN_00436620.c`, `FUN_004878d0.c`, `FUN_00439eb0.c`,
`FUN_004c0a60.c`, `FUN_004c2b20.c`, `FUN_004c0f60.c`, `FUN_004c3060.c`,
`FUN_004f3dd0.c`, `FUN_0041d830.c`, `FUN_00425c90.c`, `FUN_00485ea0.c`,
`FUN_004189b0.c`, `FUN_00435bc0.c`, `FUN_00417cb0.c`, `FUN_00414330.c`,
`FUN_00414830.c`, `FUN_00597050.c`, `FUN_00409370.c`, `FUN_005975a0.c`,
`FUN_004146e0.c`, `FUN_004213e0.c`, `FUN_00421790.c`, `FUN_00421340.c`,
`FUN_004138a0.c`, `FUN_00414a80.c`, `FUN_004397a0.c`, `FUN_00439550.c`,
`FUN_00421c70.c`, `FUN_0052f600.c`.
