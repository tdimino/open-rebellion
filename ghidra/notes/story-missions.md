---
title: "Story Missions: Dagobah, Jabba's Palace, the Bounty, Pickup and the Final Battle"
description: "The MissionMgr story-flag word +0x58 (bits 0-14, setters FUN_00543af0-FUN_00543fc0, checks in FUN_00549910), the story characters' +0xb0 flags, the ABODESD locations 0x98000440-0x98000481, and the six story mission classes (Jedi Training, Palace, Bounty, Dagobah, Vacation, Pickup): validator, chance, effect, failure and side; the final battle's Force test"
category: "ghidra"
tags: [story, missions, jedi, dagobah, palace, bounty, final-battle, luke, vader, han]
---

# Story missions

Answers task-graph node `or-q-story-missions-z8e` with `research.md`. Every
claim below was read in the `.c` files cited. The story is a set of checks
on where the story characters are and whether they are prisoners; it has
no day thresholds of its own beyond two timers.

## Characters and places

| Id | Character |
|---|---|
| `0x32000242` | Luke Skywalker |
| `0x31000241` | Leia Organa |
| `0x33000243` | Han Solo |
| `0x38000343` | Chewbacca (MNCHARSD, TEXTSTRA 10819) |
| `0x35000281` | Darth Vader |
| `0x34000280` | Emperor Palpatine |

Family `0x98` is ABODESD. `FUN_00519910` creates the five places and stores
them in the game root:

| Id | Name | Owner | Root |
|---|---|---|---|
| `0x98000440` | Alliance Jail | 1 | `+0xcc` |
| `0x98000441` | Dagobah | 3 | `+0xd0` |
| `0x98000442` | Hermitage | 3 | `+0xd4` |
| `0x98000480` | Empire Jail | 2 | `+0xd8` |
| `0x98000481` | Jabba's Palace | 2 | `+0xdc` |

A character is **free** (`FUN_005439e0`) when it is not a prisoner (`+0xac`
bit 0 clear), is active (`+0x50` bit 0), has `+0x50` bit `0x1000` clear,
and is neither adrift nor on a mission (`+0x78` bits 6 and 7 clear).

## The story-flag word: MissionMgr `+0x58`

Each bit has its own setter (`FUN_0053a640` with one mask: `FUN_00543af0`
bit 0, `FUN_00543b60` bit 2, `FUN_00543bd0`-`FUN_00543fc0` bits 5-14). A
change syncs both views, runs the bit's broadcaster
(`FUN_0054a140`-`FUN_0054a770`) and its handler slot (`+0x1c0`-`+0x1ec`).
Most checks pulse their bit (set, then clear), so the handler runs once.
`FUN_00549910` runs the checks as one chain; Dagobah's phase 8
(`FUN_00548840`) and the mission manager's other ticks call it.

| Bit | Name | Set by | Condition | Handler |
|---|---|---|---|---|
| 0 | LukeDagobahRequired | timer `0x392` (`FUN_0054e620` → `FUN_00546f80`) | armed at the new game (`FUN_00549d60`) for GNPRTB 6146 (300) + rand 6147 (100) days; cleared at Dagobah phase `0xb` (`FUN_00548840`) | `FUN_0054a800`: runs `FUN_00544030`, disarms `0x392` |
| 2 | LukeDagobah (event `0x221`) | `FUN_00544030`, pulse | bit 0, not bit 6, Luke free | `FUN_0054a860`: Dagobah mission `0x71000043`, Luke to Dagobah (`FUN_005479c0` → `FUN_00547ba0`) |
| 5 | LukePalace | `FUN_00544130`, pulse | bit 6 and Luke free | `FUN_0054a910`: Palace mission `0x64000044`, Luke, target Han |
| 6 | HanCapturedAtPalace | `FUN_005442f0`, held | Han a prisoner, active, not adrift or on a mission, at Jabba's Palace | `FUN_0054a9c0`: clears bit 14; reveals Han's system to the Alliance at level 5 (`FUN_00556c70`) when root `+0xbc` exists |
| 7 | LeiaPalace | `FUN_005443f0`, pulse | bit 6, Leia free | `FUN_0054aa60`: Palace mission, Leia |
| 8 | ChewbaccaPalace | `FUN_005444e0`, pulse | bit 6, Chewbacca free | `FUN_0054ab10`: Palace mission, Chewbacca |
| 9 | DarthPickup | `FUN_005445d0`, pulse | Luke, Leia and Han prisoners; Chewbacca a prisoner or destroyed; a prisoner at Jabba's Palace; Vader free and not on a mission | `FUN_0054abc0`: Pickup mission `0x73000082`, Vader to Jabba's Palace |
| 10 | DarthToLukeFinalBattle | `FUN_00544a20`, pulse | bit 12, Vader not where Luke is | `FUN_0054ac70`: hidden Move mission `0x41000001` takes Vader to Luke (`FUN_00547340`) |
| 11 | DarthToEmperorFinalBattle | `FUN_00544be0`, pulse | bit 12, Vader with Luke and away from the Emperor | `FUN_0054adf0`: when Vader and Luke both have owner 2 and neither is on a mission, a Move mission takes both to the Emperor |
| 12 | FinalBattleReady | `FUN_00544da0`, held | not bit 13; Luke a prisoner, active, not adrift or on a mission, not at Jabba's Palace, Luke `+0xb0` bit 1; Vader and the Emperor free | `FUN_00524fb0` (default) |
| 13 | FinalBattle (event `0x220`) | `FUN_00544fc0` | bit 12, and Luke, Vader and the Emperor in one place | `FUN_0054b040` |
| 14 | BountyHuntersActive | `FUN_00549d60` at the new game; cleared by bit 6's handler | — | `FUN_0054b740`: arms timer `0x393` for GNPRTB 6148 (300) + rand 6149 (300) days |

When Vader is on another mission that he may resign (`+0x78` bit 4), the
DarthPickup check asks him to resign (`FUN_00534640`) instead of pulsing.
When Luke is on the Dagobah mission in state 8 as Han reaches the palace,
`FUN_00544130` calls that mission's slot `+0x1d8`.

## Character flags `+0xb0`

- **Luke** (vtable `0x00665fd8`):
  - bit 0: set when Luke first meets Vader (event `0x360`, `FUN_00560200`
    → `FUN_0056f650`). In that encounter `FUN_00561b60` injures Luke (slot
    `+0x2f0`, cause 10) when his Force `+0x8c` is below GNPRTB `0xc05` (80),
    and Luke and Leia gain Force experience `+0x8e` scaled by the gap to
    Vader or the Emperor.
  - bit 1: bit 0 and `+0x8c` at least GNPRTB `0xc06` (60), recomputed by
    `FUN_0056f7a0`; its change posts event `0x313`.
  - bit 3: Dagobah completed (`FUN_005751d0`, `FUN_0056f730`).
- **Han**: bit 0 while a Bounty roll runs (`FUN_00572690`; change slot
  `FUN_00572b40`, BountyAttack, event `0x212`); bit 1 once the bounty
  hunters hold him (`FUN_00572700`; change slot `FUN_00572b90`).
- **Leia**: bit 0 is set by the discovery timer only when Luke's bit 0 is
  set (`FUN_0056fe90`).

`FUN_0056f8b0` adds GNPRTB `0xc09` (1) to Luke's `+0x8e` on an evade.

## The chains

1. **Dagobah.** 300-399 days into the game bit 0 is set. If Han is not held
   at the palace and Luke is free, Luke gets the hidden Alliance Dagobah
   mission. At phase `0xb` his Force rises and Luke's bit 3 is set, and bit
   0 clears.
2. **The bounty.** Timer `0x393` (`FUN_00546fa0`) draws against GNPRTB 6150
   (100). When Han is free and active it creates special force
   `0x3c000009` ("Bounty Hunters") for the palace's owner, the Empire,
   reveals Han's system to the Empire at level 5 (`FUN_00556c70`), and
   starts a Bounty mission `0x65000083` against Han. A prisoner of a Bounty
   mission is moved to Jabba's Palace (`FUN_00543040`).
3. **The palace.** With Han held there (bit 6), Luke, Leia and Chewbacca
   each get a hidden Alliance Palace mission.
4. **Pickup.** Once Luke, Leia, Han and Chewbacca are all taken, Vader's
   Empire Pickup mission collects the prisoners at the palace.
5. **The final battle.** When the Empire holds Luke and Luke has met Vader
   with Force at least 60 (bit 12), Vader moves to Luke (bit 10), then both
   move to the Emperor (bit 11). With all three together `FUN_0054b040`
   reveals the place at level 6 to both sides and compares Luke's `+0x8c`
   with GNPRTB 6151 (100):
   - below 100: Luke loses `+0xac` bit 8 and is injured (`FUN_00549a20`,
     slot `+0x2f0`, cause 10);
   - 100 or more: Luke's slot `+0x218` runs, Vader and the Emperor are
     captured (slot `+0x20c`) and lose bit 8.

   When the winner's container is of the other side, the three move
   together by Move mission `FUN_00547480` to the winner's `FUN_00543040`
   destination; no abode is involved (`jail.md`). Vader and the Emperor as
   prisoners are the DC and EC bits of `victory-conditions.md`.
6. **Force discovery.** Timer `0x389` (`FUN_00560a30`) runs for a character
   with `+0xac` bit 7. Same-side characters at its location that are free,
   have `+0xac` bit 5 and not bit 4 get bit 4 (`FUN_004ee950`) and `+0xa4`
   set to the discoverer (`FUN_004ee680`). Leia qualifies only when the
   discoverer is Luke with bit 0 set.

## The six mission classes

Slots from `mission-lifecycle.md`. Each class's validator is its slot
`+0x1bc` (`FUN_00592aa0` plus the class function); its target and member
checks are creation-only (checker `+4`), so a running mission skips them.
The story creates its missions directly with `FUN_00547ba0`, which does not
call the validator. Common member refusals: prisoner (`0x40/9`) and `+0x78`
bit 0 (`0x40/8`). All but Jedi Training are hidden in MISSNSD (`+0x5c`).

| Mission | Id | Side | Validator | Chance | Effect | Failure |
|---|---|---|---|---|---|---|
| Jedi Training | `0x58000042` | both | `FUN_00570690` | `FUN_005710b0` | `FUN_005712b0` | `FUN_00576700`: no change |
| Palace | `0x64000044` | Alliance | `FUN_0056df70` | `FUN_0056e620` | `FUN_0056e650` | `FUN_0056e870`: every member captured |
| Bounty | `0x65000083` | Empire | `FUN_00575900` | `FUN_00575060` (100) | `FUN_00575de0` | none (`0x51ebb0`) |
| Dagobah | `0x71000043` | Alliance | `FUN_00574b10` | 100 | `FUN_00572470` (always 3); the training is `+0x1f4` `FUN_005751d0` | `FUN_00572490` |
| Vacation, Sabbatical | `0x72000045`, `0x72000046` | Alliance | `FUN_00571f20` | 100 | `FUN_00572470` | `FUN_00572490` |
| Pickup | `0x73000082` | Empire | `FUN_0056d4f0` | 100 | `FUN_0056da80` | `FUN_00576700` |

- **Jedi Training** (timer 60 + rand 30). The team needs a teacher
  (`FUN_00570e40`, `FUN_00570f80`, `FUN_004ed310`; else `0x40/0x90`), two or
  more members (`0x94`), every member `+0x8c` non-zero (`5`), and no
  student at or above the teacher's `+0x8c` (`0xd`). Phase 2 stores the
  teacher at mission `+0xa8` (`FUN_00571410`, `FUN_005703b0`). The chance
  is teacher `+0x8c` minus student `+0x8c` (`FUN_0055c7c0`). A success sets
  new = L + rand(0..(T - L) × GNPRTB `0x181d` (50) / 100) (`FUN_0055ca20`)
  and raises the student's `+0x90` by new - L (`FUN_004ee1d0`).
- **Palace** (timer 5 + rand 10). The container must be Jabba's Palace
  (`0x15`), the target must not be Han when ordered (`0x30`), and the
  members may be only Luke, Leia or Chewbacca (`0xa`). Chance: slot
  `+0x1e0` (espionage) / GNPRTB `0x180a` (3) + slot `+0x1f0` / `0x180b` (3)
  (`FUN_0055c910`). Success raises combat `+0x62` by `0x181a` (1) and
  espionage `+0x5a` by `0x1819` (1), releases the target (slot `+0x210`) and
  then every prisoner at the palace. Failure captures every member (slot
  `+0x20c`, no captor) and clears their `+0xac` bit 8 (`FUN_004eeb10`).
- **Bounty** (timer 0). Members must be special force `0x3c000009`
  (`FUN_00575cc0`, `FUN_00575ae0`; `0xb`); an ordered Bounty may not target
  Han (`0x30`). The effect acts on Han (family `0x33`): it sets Han's bit 0,
  rolls `FUN_005349e0` (table 13 against Han's slot `+0x1f0`) and on 0 has
  the member capture him (slot `+0x20c`), else slot `+0x208`. If Han is then
  a prisoner: result 3, `+0xac` bit 8 cleared, Han's bit 1 set; else 2.
  Han's bit 0 clears at the end.
- **Dagobah** (timer 100). The target must be Dagobah (`0x2f`) and the
  members Luke (family `0x32`, `FUN_00574f30`, `FUN_00574cf0`; `7`). Phase 8
  stores the day at mission `+0xa8` (`FUN_00574960`). Phase `0xb` raises
  Luke's `+0x8e` (`FUN_004ee150`) by F × GNPRTB `0x181e` (60) / 100 on
  result 3 (`FUN_0055ca60`), else by F × ((now - first day) / `0x181f`
  (2)) / 100 (`FUN_0055ca90`), F being `+0x8c`, and sets Luke's bit 3.
- **Vacation and Sabbatical** (timers 60 and 1000). The target must be the
  Hermitage (`0x2e`) and the members Luke (`FUN_00572340`, `FUN_00572100`).
  No skill or Force change; the phase handler is the plain one
  (`FUN_00594460`).
- **Pickup** (timer 0). The target must be Jabba's Palace (`0x31`) and the
  members Vader (family `0x35`, `FUN_0056d910`, `FUN_0056d6d0`). The effect
  sets MissionMgr `+0x58` bit 2 for its duration (`FUN_0055f6c0`) and takes
  every prisoner at the target without `+0x78` bit 7 into the mission (slot
  `+0x1cc`), setting its `+0x70` to the member's location (`FUN_00534390`).

## Port

- `MissionKind` has none of the six classes; `missions.rs` defers Jedi
  Training.
- `crates/rebellion-core/src/story_events.rs` scripts the Dagobah, Vader
  and final-battle chains with tick gates (`TickAtLeast` 30 to 90), system
  counts, random gates and invented events (carbonite failures
  `0x398`-`0x39a`, Emperor arrival `0x230`). It reuses the original's timer
  ids `0x390`-`0x397` as its own event ids. It has no flag word, ABODESD
  places, Han-at-the-palace trigger, Pickup, Move-mission convergence or
  Force test against GNPRTB 6151.
- `crates/rebellion-core/src/jedi.rs` raises Force on per-tick experience
  tiers with random detection; the original raises it through Jedi
  Training, Dagobah and encounters, and discovers on timer `0x389`.

## Open

- What `+0x8c`, `+0x8e` and `+0x90` each hold (the checks read `+0x8c`;
  training raises `+0x8e` and `+0x90`), and `+0xac` bits 4, 5, 7, 8.
- What Dagobah's slot `+0x1d8` does when Han reaches the palace.
- The captor in the final battle's capture, and Luke's slot `+0x218`.
- Who creates Vacation and Sabbatical, and who calls Luke's slot `+0x400`
  (`FUN_0056fc70`, DagobahCompleted, event `0x210`).
- `FUN_005349e0`'s table 13 and slot `+0x208`; whether timer `0x393`
  re-arms; where event `0x313` goes.
- Root `+0xbc` (`0xf9000009`) and `+0xc0` (`0xfa00000a`).

## Decompiles

`FUN_00519910.c`, `FUN_005439e0.c`, `FUN_005439b0.c`, `FUN_00543af0.c`,
`FUN_00543b60.c`, `FUN_00543bd0.c`, `FUN_00543fc0.c`, `FUN_00549910.c`,
`FUN_00548840.c`, `FUN_0054e620.c`, `FUN_00546f80.c`, `FUN_00549d60.c`,
`FUN_0054a800.c`, `FUN_00544030.c`, `FUN_0054a860.c`, `FUN_005479c0.c`,
`FUN_00547ba0.c`, `FUN_00544130.c`, `FUN_0054a910.c`, `FUN_005442f0.c`,
`FUN_0054a9c0.c`, `FUN_00556c70.c`, `FUN_005443f0.c`, `FUN_0054aa60.c`,
`FUN_005444e0.c`, `FUN_0054ab10.c`, `FUN_005445d0.c`, `FUN_0054abc0.c`,
`FUN_00544a20.c`, `FUN_0054ac70.c`, `FUN_00547340.c`, `FUN_00544be0.c`,
`FUN_0054adf0.c`, `FUN_00544da0.c`, `FUN_00544fc0.c`, `FUN_0054b040.c`,
`FUN_00549a20.c`, `FUN_0054b740.c`, `FUN_00534640.c`, `FUN_00546fa0.c`,
`FUN_00543040.c`, `FUN_00547480.c`, `FUN_00560200.c`, `FUN_0056f650.c`,
`FUN_00561b60.c`, `FUN_0056f7a0.c`, `FUN_0056f730.c`, `FUN_0056f8b0.c`,
`FUN_0056fe90.c`, `FUN_00560a30.c`, `FUN_004ee950.c`, `FUN_004ee680.c`,
`FUN_00572690.c`, `FUN_00572700.c`, `FUN_00570690.c`, `FUN_00570e40.c`,
`FUN_00570f80.c`, `FUN_004ed310.c`, `FUN_00571410.c`, `FUN_005703b0.c`,
`FUN_005710b0.c`, `FUN_0055c7c0.c`, `FUN_005712b0.c`, `FUN_0055ca20.c`,
`FUN_004ee1d0.c`, `FUN_00576700.c`, `FUN_0056df70.c`, `FUN_0056e620.c`,
`FUN_0055c910.c`, `FUN_0056e650.c`, `FUN_0056e870.c`, `FUN_004eeb10.c`,
`FUN_00575900.c`, `FUN_00575cc0.c`, `FUN_00575ae0.c`, `FUN_00575060.c`,
`FUN_00575de0.c`, `FUN_005349e0.c`, `FUN_00574b10.c`, `FUN_00574f30.c`,
`FUN_00574cf0.c`, `FUN_00572470.c`, `FUN_00572490.c`, `FUN_005751d0.c`,
`FUN_00574960.c`, `FUN_0055ca60.c`, `FUN_0055ca90.c`, `FUN_004ee150.c`,
`FUN_00571f20.c`, `FUN_00572340.c`, `FUN_00572100.c`, `FUN_00594460.c`,
`FUN_0056d4f0.c`, `FUN_0056d910.c`, `FUN_0056d6d0.c`, `FUN_0056da80.c`,
`FUN_0055f6c0.c`, `FUN_00534390.c`, `FUN_00592aa0.c`.
