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

## After the ending movie

- `FUN_00414330` states `0x10`-`0x13` tear the game down (the strategic and
  tactical children, the CommMgr through `FUN_00597050`, the session through
  `FUN_00409370` → `FUN_005975a0`) and `FUN_00414830` plays movie `0x6b`,
  `0x6a`, `0x6d` or `0x6c` (MDATA 105, 106, 108, 107) through `FUN_004146e0`
  → `FUN_004213e0`.
- The movie window ends on its notifications `0x402`-`0x404`, `WM_COMMAND`
  `0xf1` or a key other than Esc, then posts `0x405` (`FUN_00421790`). The
  application's handler (`FUN_004138a0`) destroys it and, for ids `0x66` and
  `0x6a`-`0x6d`, calls `FUN_00414330(3)`: the main menu (`FUN_00414a80`),
  with the campaign's fields cleared.

## Open

- The HQ's bombardment value (its slot `+0x1c0`), and so the chance a hit on
  it succeeds.
- Whether an HQ in transit (`+0x50` bit `0x20`) survives Destroy System and
  capture: `FUN_004f8710` and `FUN_004f8680` skip such an object.
- Whether excess-facility removal (`FUN_0050ad80`, `FUN_004f8760`, reason 4),
  autoscrap (reason `0x15`) or sabotage (reason 6) can reach the HQ.
- Who reads the AI object's `+0x190`/`+0x194`.
- Who sends `0x402`-`0x404` to the movie window, and what Esc does there.
- Which of each side's two waves is spoken as victory and which as defeat
  (named here by length only).

## Decompiles

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
`FUN_004138a0.c`, `FUN_00414a80.c`.
