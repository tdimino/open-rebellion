---
title: "Victory Conditions: The Game Root's Flag Word +0x74"
description: "FUN_005180e0 computes the win flags (named by the binary's own two-letter codes) from four leaders, Coruscant and the Alliance HQ for the Standard and Headquarters Only game types; the flags reach each side object (+0xbc, notification 0x128), the player's advisor, and the strategic window, which plays one of four ending movies (MDATA 105-108)"
category: "ghidra"
tags: [victory, game-over, leaders, coruscant, headquarters, endings, movies]
---

# Victory conditions

Answers task-graph node `or-q-victory-conditions-8zx`. Every claim below was
read in the `.c` files cited.

## The flag word

`FUN_005180e0(ctx)` computes the win state and stores it in the game root's
(`DAT_006b2bb0`, type `0xf1`) word `+0x74`. When the word changes it copies
`ctx` into the root's `+0x194` (`FUN_004fd490`). It reads:

- Luke Skywalker `0x32000242`, Mon Mothma `0x30000240`, Darth Vader
  `0x35000281` and Emperor Palpatine `0x34000280` (`FUN_00505190`), each one's
  `+0xac` bit 0 (prisoner; `ai-mission-planning.md`);
- Coruscant `0x90000109` (`FUN_00505f60`): its owner bits (`+0x24 & 0xc0`)
  and its `+0x50` bit 3 (destroyed);
- the Alliance HQ (`FUN_00506110`: the master copy of the first object of
  family `0x20`-`0x21`, `FUN_0053f030`; ALLFACSD `0x20000001`,
  `facility-ownership.md`): missing, or its `+0x50` bit 3 set;
- the game type, root `+0x60`.

If any leader or Coruscant is missing, or the game type is not 0 or 1, it
returns 0 and leaves `+0x74` unchanged.

`FUN_00530a50` prints the word for `SideVictoryConditionsNotif` with one
two-letter code per bit (strings at `0x006aa56c`-`0x006aa594`):

| Bit | Code | Set when |
|---|---|---|
| `0x1` | AD | the Alliance HQ is missing or destroyed |
| `0x10` | LC | Luke is a prisoner |
| `0x20` | MC | Mon Mothma is a prisoner |
| `0x100` | CA | Coruscant's owner is the Alliance (1) |
| `0x200` | CD | Coruscant is destroyed |
| `0x1000` | DC | Vader is a prisoner |
| `0x2000` | EC | Palpatine is a prisoner |
| `0x100000` | EW | the Empire wins |
| `0x1000000` | AW | the Alliance wins |
| `0x10000000` | GO | the game is over (EW or AW) |

## The rules

The game type comes from the new-game screen: its `+0xf4` (1 Standard Game,
2 Headquarters Only; `main-menu-parity.md`) goes through `FUN_004010a0` to
`FUN_004142d0`, which stores 1 at the app's `+0xec` for type 2 and 0
otherwise. The new-game start (`FUN_00414830`, state 7) copies it into the
settings record at `+8` (`FUN_0051c740`), which the root holds at `+0x58`, so
it lands at root `+0x60` (the galaxy size, record `+4`, lands at `+0x5c`).

| Game type | Root `+0x60` | Empire wins (EW) | Alliance wins (AW) |
|---|---|---|---|
| Standard Game | 0 | AD and LC and MC | DC and EC and (CA or CD) |
| Headquarters Only | 1 | AD | CA or CD |

- The Empire's objective is the HQ object, not its system. The HQ is gone
  when it is destroyed, or when its system leaves the Alliance, which removes
  it (`facility-ownership.md`). No occupation is required.
- The Alliance's objective is Coruscant held by the Alliance or destroyed.
- A prisoner counts whoever holds it.
- If both sides qualify at once, the side whose win bit was already set keeps
  it: AW if `+0x74` already had `0x1000000`, else EW if it had `0x100000`,
  else AW.
- There is no minimum day.
- The word is rebuilt from zero on each call; only the tie rule reads an earlier EW or AW.
- The principal leaders cannot be destroyed: kill slot `+0x2e0` returns 0 for all six majors (`victory-flag-word.md`, 3).

## When the check runs

| Caller | Trigger |
|---|---|
| `FUN_00516220` | event `0x30a` on a major character (family `0x30`-`0x37`) |
| `FUN_00517aa0` → `FUN_00517be0` | event `0x31b` control change of Coruscant (system `+0x18 == 0x109`) |
| `FUN_005140c0` → `FUN_00517cd0` | event `0x300` object removal |
| `FUN_005140c0` → `FUN_00517bb0` → `FUN_00517ca0` | event `0x300` when the object is Coruscant |

## From the flags to the ending

1. **Side objects.** `FUN_0051e050` (the session update, via `FUN_0041e490`)
   calls `FUN_00518460`, which gives `+0x74` to both side objects
   (`FUN_00506f30(1)`, `(2)`; ids `0xf300000f`, `0xf3000010`) through
   `FUN_0052f600`. That stores it at the side object's `+0xbc` and
   refreshes both views (`FUN_005322c0`). A view whose value changes calls
   its slot `+0x260`, `FUN_00532f90` (entry `0x00660d58`), which posts
   notification `0x128` (`SideVictoryConditionsNotif`, "VictoryConditions";
   factory `FUN_0043c5b0`, registered in `FUN_0041e580`).
2. **Player.** Notification `0x128` calls its listener's slot `+0x74` with the
   new word (`FUN_0043c630`). For the local player (vtable `0x0065ad70`,
   `FUN_004861b0`) that is `FUN_00486890`: on GO it calls slot `+0x10` of the
   player's advisor (`+0xc0`). The computer player's slot (`FUN_00485ea0`,
   vtable `0x0065abf0`) passes the word to its `+0x84` object
   (`FUN_004189b0`).
3. **Advisor.** The Empire advisor's slot (`FUN_004c0f10`) sets reaction slot
   `+0x150` (EW) or `+0x154` (AW) of its `+0x168` block to the current time
   plus 40000; the Alliance advisor's (`FUN_004c3010`) uses `+0x15c` (EW) and
   `+0x158` (AW). Both set the strategic window's `+0xac` (`FUN_0041d810`).
4. **Ending code.** When the advisor's pending action (`DAT_006b28c4`) is 3,
   its tick (`FUN_004c0f60` Empire, `FUN_004c3060` Alliance) reads the word
   from its own side object's view (`FUN_004f3dd0(2,2)` or `(1,1)`, `+0xbc`)
   and passes it to `FUN_0041d830` → `FUN_00425c90`, which sets the strategic
   window's (`DAT_006b14cc`) `+0xa8` from the winner and the window's side
   `+0x9c`. Action 3 is set by `FUN_00439eb0`, through `FUN_004878d0`,
   `FUN_00436620` and `FUN_0041d140`, from the finish slot `0x00412400` of
   advisor script step `0x11` (vtable `0x00658848`; `droid-advisor-triggers.md`).
5. **Movie.** `FUN_0042db70` posts message `0x482` with the code;
   the strategic window (`FUN_00422ce0`, `0x482` with wParam 0) passes it to
   `FUN_00401040` → `FUN_00414330`, whose states `0x10`-`0x13` close the
   strategic and tactical windows and run `FUN_00414830`, which opens the
   movie window (`FUN_004146e0`, `FUN_00421ba0`):

| Code | Player | Winner | Movie id | File |
|---|---|---|---|---|
| `0x10` | Alliance | Alliance | `0x6b` | `MDATA\MDATA_105` |
| `0x11` | Alliance | Empire | `0x6a` | `MDATA\MDATA_106` |
| `0x12` | Empire | Empire | `0x6d` | `MDATA\MDATA_108` |
| `0x13` | Empire | Alliance | `0x6c` | `MDATA\MDATA_107` |

`FUN_00421ba0` maps the other movie ids: `0x65`-`0x69` to MDATA 000-004,
`0x6e` 005, `0x6f` 101, `0x70` 202, `0x71` 201, `0x72` 104, `0x73` 102,
`0x74` 103.

## Port

`crates/rebellion-core/src/victory.rs` against the table above:

- **Empire objective (AD).** The HQ system destroyed, or its
  `is_headquarters` flag clear, whoever holds it. The flag clears through
  `support_shift::remove_headquarters` (`game-end-path.md`, Port): a
  successful hit on the HQ as a military target, Destroy System, or a
  control change that leaves the system Empire or neutral.
- **Alliance objective (CA or CD).** Coruscant's holder the Alliance
  (`bombardment::holder`, which keeps the earlier holder through a
  stalemate, as `FUN_0050b610` keeps `+0x24` bits 6-7), or Coruscant
  destroyed.
- **Cadence.** `port:` the game rebuilds the word (`VictorySystem::word`)
  and the headless runner checks it (`VictorySystem::check`) every tick, in
  place of `FUN_005180e0`'s event-driven reruns.
- **Both at once.** `VictorySystem::word` keeps the side whose win bit the
  previous word held, else gives AW, as the original does.
- **Leaders.** The port reads each leader's prisoner bit
  (`Character::is_captive`) and no captor, so a leader the adrift sweep
  jailed with no captor counts too (`death-star-strategic.md`, "Jail
  abodes").
- **Ending.** The local player's advisor stamps its ending slot and the
  clock stops; the script's step 3 stores the ending code, and its movie
  plays after the teardown, then the main menu (`movies.rs`;
  `agent_docs/systems/victory.md`, "The ending").

The HQ's removal (Destroy System's container cascade, a bombardment hit,
capture), the advisor's ending script, the computer player's counters and the
return to the main menu after the movie are in `game-end-path.md`.

## Decompiles

`FUN_005180e0.c`, `FUN_00530a50.c`, `FUN_00506110.c`, `FUN_0053f030.c`,
`FUN_00516220.c`, `FUN_00517aa0.c`, `FUN_00517be0.c`, `FUN_00517bb0.c`,
`FUN_00517ca0.c`, `FUN_00517cd0.c`, `FUN_0051e050.c`, `FUN_00518460.c`,
`FUN_0052f600.c`, `FUN_005322c0.c`, `FUN_00532f90.c`, `FUN_0041e580.c`,
`FUN_0043c5b0.c`, `FUN_0043c630.c`, `FUN_00486890.c`, `FUN_00485ea0.c`,
`FUN_004c0f10.c`, `FUN_004c3010.c`, `FUN_004c0f60.c`, `FUN_004c3060.c`,
`FUN_004f3dd0.c`, `FUN_0041d830.c`, `FUN_0041d810.c`, `FUN_00425c90.c`,
`FUN_00439eb0.c`, `FUN_004878d0.c`, `FUN_00436620.c`, `FUN_0041d140.c`,
`FUN_0042db70.c`, `FUN_00422ce0.c`, `FUN_00401040.c`, `FUN_00414330.c`,
`FUN_00414830.c`, `FUN_004146e0.c`, `FUN_00421ba0.c`, `FUN_00406000.c`,
`FUN_004010a0.c`, `FUN_004142d0.c`.
