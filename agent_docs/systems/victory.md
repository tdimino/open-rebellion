---
title: "Victory System"
description: "Game-ending condition detection for Alliance and Empire"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-17
tags: [victory, hq-capture, flag-word, simulation]
---

# Victory System

`victory.rs`: Game-ending condition detection.

## Types

| Type | Purpose |
|------|---------|
| `VictoryState` | `{ alliance_hq, empire_hq, word, resolved }`; `word` is the win flag word `FUN_005180e0` last wrote |
| `VictoryOutcome` | HqCaptured / HqDestroyed / DeathStarVictory, for the headless runner |

## The flag word

`victory::word` rebuilds `FUN_005180e0`'s word from zero each step
(`ghidra/notes/victory-flag-word.md`): AD `0x1`, LC `0x10`, MC `0x20`, CA
`0x100`, CD `0x200`, DC `0x1000`, EC `0x2000`, EW `0x100000`, AW `0x1000000`,
GO `0x10000000`. The leaders are known by DatId (Luke `0x32000242`, Mon
Mothma `0x30000240`, Vader `0x35000281`, Palpatine `0x34000280`) and
Coruscant by `0x90000109`; with one missing the word is not written. The
loaded world keeps SYSTEMSD and MJCHARSD ids without their family byte, so
the lookups match the record index (`DatId::is_record`).

- **AD**: the Alliance HQ system is destroyed, or its `is_headquarters` is
  clear, whoever holds it (`FUN_004f8710`; `support_shift::remove_headquarters`).
- **CA / CD**: Coruscant's holder is the Alliance (`bombardment::holder`:
  through a stalemate, the side that held it before, `FUN_0050b610`), or
  Coruscant is destroyed.
- **LC, MC, DC, EC**: the leader is a prisoner (`systems/captivity.md`); a
  prisoner counts whoever holds it, a jailed one too.

Standard: EW = AD, LC and MC; AW = DC, EC and (CA or CD). Headquarters Only:
EW = AD; AW = CA or CD. When both sides win at once, the side whose win bit
the previous word held keeps it, else the Alliance wins. Both modes evaluate
from the first simulation tick.

Each change of the word is notification `0x128`. It files two movie-only
messages (kind 6, `strike_orders::word_movies`), each again on every later
change while its bit holds: Rebel HQ Destroyed `0x1f` for AD (movie `0x73`,
MDATA.102, caption TEXTSTRA `0x7189`) and Coruscant Captured for CA (movie
`0x74`, MDATA.103, caption `0x7171`). Kind 6 never reaches the Message Index
or the advisor (`FUN_0048a060`). Both sides file them; the local player sees
them.

## The ending

With GO set, `0x128` hands the word to the local player's advisor
(`FUN_00486890`), which stamps its ending slot at the step count + 40000
and stops the clock (`FUN_004c3010`, `FUN_004c0f10` → `FUN_0041d810`):

| Viewer | EW (Empire wins) | AW (Alliance wins) |
|---|---|---|
| Alliance | slot 87, ALSPRITE 10093, code `0x11`, MDATA.106 | slot 86, ALSPRITE 10094, code `0x10`, MDATA.105 |
| Empire | slot 84, EMSPRITE 11094, code `0x12`, MDATA.108 | slot 85, EMSPRITE 11093, code `0x13`, MDATA.107 |

The script plays on the agent droid; its last record (command `0x11`, p2 3)
runs cockpit step 3, which turns the word into the ending code
(`FUN_004c3060` → `FUN_00425c90`). Once the galaxy has no modal window the
game is torn down and the code's movie plays (`FUN_00414330`,
`FUN_00414830`); its end, Esc, a click or any key returns to the main menu
(`FUN_004138a0`). There is no way back into the campaign.

In the port `movies::MovieQueue` holds the message movies, the stopped
clock and the code; `AdvisorState::post_word` stamps the slot. port: a slot
with no staged records (the advice table missing) runs step 3 at once, so
the ending still comes. port: the original asks an open modal to close
(`0x40d`); the port waits for it to close. In the browser the shell's
`<video>` covers a message movie's picture only (`movies::video_bounds`), so
the canvas draws the caption field below it.

A saved game keeps its ending. The original's save holds the advisor's slot
table and action (`FUN_004397a0`, `FUN_00439550`), so a game saved after GO
loads with the ending slot stamped; the strategic window is built anew
(`FUN_00421c70` zeroes `+0xac`), so the clock runs until the script ends.
The port saves the word, and the load stamps the slot again from it
(`resume_saved_advisor`).

## Initialization

`VictoryState::new(alliance_hq, empire_hq)`: the seeding marks both HQs
(`seeds.rs` refuses data without Coruscant or Yavin). port: a mod can still
leave a side without one; a campaign start then seeds again and refuses the
galaxy before replacing anything (`headquarters`), and the startup galaxy
keeps null keys until a campaign starts. The original reads a missing
Alliance HQ as AD and finds Coruscant by id (`FUN_005180e0`).

## Lifecycle

1. The game evaluates `VictorySystem::word` each tick until it holds GO,
   then sets `resolved` and evaluates no more.
2. `VictorySystem::check()` returns `None` every tick until the word holds a
   win bit, then `Some(VictoryOutcome)`; the headless runner
   (`integrator::apply_victory`) acts on it and sets `resolved`.

## Source

- `entity-system.md §4.2`: `SideVictoryConditionsNotif`, `FinalBattle`
- Notification `0x128` and the win word (`FUN_005180e0`; `ghidra/notes/victory-flag-word.md`)
- `System::is_headquarters` flag
- [`main-menu-parity.md`](../main-menu-parity.md): original game-type selector and setup contract
- [F-016B evidence](../../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md)
- [Campaign-history source ledger](../../docs/reference/campaign-history/sources.json)
- [Victory-contract verification](../../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-victory-contract.md)
