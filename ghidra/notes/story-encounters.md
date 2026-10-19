---
title: "Story Encounters as Trigger, Test and Outcome Rows"
description: "How each story chain is triggered (character change events through the FUN_005438a0 gate and two timers), tested (the ten MissionMgr flag checks) and resolved, what is DAT and what is a literal id, the side branches, and what a mod adding an encounter changes"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [story, encounters, missions, luke, vader, palpatine, dagobah, bounty, mod-surface]
---

# Story encounters

Answers task-graph node `or-q-mod-encounters-36xh` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-encounters`; the gate, the flag-check combiner and the final-battle Force test were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code.

## Answer

### The shape: one flag word, one gate, one chain

- **State.** MissionMgr `+0x58` holds bits 0-14. Each bit has its own setter (`FUN_00543af0` bit 0, `FUN_00543b60` bit 2, `FUN_00543bd0`..`FUN_00543fc0` bits 5-14). A change that really flips the bit runs both side views' broadcaster and the manager's handler slot `+0x1c0`..`+0x1ec`.
- **Chain.** `FUN_00549910` calls ten checks in a fixed order and ANDs the results; no check short-circuits: Han at palace (`FUN_005442f0`, bit 6), final battle ready (`FUN_00544da0`, bit 12), Dagobah (`FUN_00544030`, bit 2), Luke palace (`FUN_00544130`, bit 5), Leia (`FUN_005443f0`, bit 7), Chewbacca (`FUN_005444e0`, bit 8), pickup (`FUN_005445d0`, bit 9), Vader to Luke (`FUN_00544a20`, bit 10), Vader to Emperor (`FUN_00544be0`, bit 11), final battle (`FUN_00544fc0`, bit 13).
- **Gate.** Nothing runs on a calendar except two timers. The chain runs when a hook fires on one of six objects: `FUN_005438a0` is true only when the object's id is Luke `0x32000242`, Leia `0x31000241`, Han `0x33000243`, Chewbacca `0x38000343`, Vader `0x35000281` or Palpatine `0x34000280` (literals in the function). Its callers, each gated by it, are the entity hooks `FUN_00545240` (object change, from event `0x300` through `FUN_005140c0`), `FUN_00545540` (event `0x301`), `FUN_005457f0` (event `0x320`), `FUN_00545df0` (events `0x307`, `0x30e`), `FUN_00545e90` (event `0x312`), `FUN_00546370` (event `0x30a`), `FUN_00546e30`; and Dagobah's phase handler `FUN_00548840` (phase 8 re-runs the chain; phase `0xb` clears bit 0). The "trigger" for a check is therefore *any change event on a story character*; the check then reads where everyone is. hyp: the event-to-handler map for `0x307`/`0x312` is from `map-12-52a380.md`, not re-read here.
- **Free character** (`FUN_005439b0`, `FUN_005439e0`): `+0xac` bit 0 clear, `+0x50` bit 0 set, `+0x50` bit `0x1000` clear, `+0x78` bits 6 and 7 clear.
- **Fixed-id accessors.** `FUN_005434b0` (Luke, via `FUN_00507010`), `FUN_00543550` (Leia, `FUN_00506fb0`), `FUN_005435f0` (Han, `FUN_00506f50`), `FUN_00543690` (Chewbacca, `FUN_005066a0` with family `0x38`..`0x3b`), `FUN_00543760` (Vader, `FUN_00507070`), `FUN_00543800` (Palpatine, `FUN_005070d0`). Each looks up one literal id; none is data.

### Trigger, test, outcome rows

Column "Limit" says where a number comes from: **DAT** = a GNPRTB parameter (file `GNPRTB.DAT`, loaded once by `FUN_0055bfd0` through `FUN_0053e390`, columns per difficulty, the shipped file has one value in all columns for these ids); **code** = a literal or family check in the named function; **table** = a table in the executable. The numbers are the shipped GNPRTB values.

#### Chain 1: Dagobah (Alliance; the Empire sees nothing)

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 1a | New game: `FUN_00549d60` arms timer `0x392` (`FUN_0053fa60`) with delay parameters `FUN_00540230(DAT_006bb560)` and `FUN_00540270(DAT_006bb5bc)` | none | Delay 300 + rand(100) days. Limit: GNPRTB `0x1802` = 300 and `0x1803` = 100 (**DAT**); clamps `0x7fff` and `0x3fff` are words at `0x00661a90` and `0x00661a8c` (**table**, read with `DumpMemory`) |
| 1b | Timer `0x392` fires: `FUN_0054e620` (only if the timer's own flag word is clear and the delay passed) calls `FUN_00546f80` | `FUN_00543af0(1)` sets bit 0 | Handler `FUN_0054a800` runs `FUN_00544030` and disarms `0x392` (`FUN_0053fa60(0x392, 0, ...)`) |
| 1c | `FUN_00544030`: bit 0 set and bit 6 clear (Han not held at the palace) | Luke free (`FUN_005439e0`), **code** | Pulses bit 2 (`FUN_00543b60(1)` then `(0)`). Handler `FUN_0054a860`: Dagobah mission `0x71000043`, Luke `0x32000242`, place `0x98000441`, through `FUN_005479c0` (all literals, **code**) |
| 1d | Bit 2 set, Luke on the Dagobah mission in state 8, and Han reaches the palace (bit 6) | `FUN_00544130` calls mission slot `+0x1d8` | hyp: slot `+0x1d8`'s effect not read |
| 1e | Mission phase `0xb` | `FUN_00548840`: `FUN_00520ac0() >= 8` (mission state, **code**) then `FUN_00543af0(0)` | Clears bit 0. Training amounts are in `story-missions.md` (GNPRTB `0x181e`, `0x181f`) |
| 1f | Messages. Event `0x221` (`FUN_0054b7b0`, notification `MissionMgrLukeDagobahNotif`) and `0x210` (`FUN_0056fc70`, `LukeDagobahCompletedNotif`) both map to message class `0x25` (`FUN_00488de0`) | Class `0x25` handlers `FUN_0048d190` (0x221) and `FUN_0048d0f0` (0x210) show only when the side is 1 (**code**: `param_1+0x20 == 1`) | Alliance: "Luke Goes to Dagobah" / "Luke has been sent to Dagobah to be trained by Yoda." (`0x71b8`/`0x71b9`, picture `0x421`, bitmap `0x46c`, voice `FUN_004c4990(0x11d)`); then "Luke Leaves Dagobah" / "I have finished my training with Yoda." (`0x71ba`/`0x71bb`, picture `0x412`, bitmap `0x47c`). Empire: no message |

#### Chain 2: the bounty (Empire acts; both sides see it)

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 2a | New game: `FUN_00549d60` sets bit 14 (`FUN_00543fc0(1)`) | none | Handler `FUN_0054b740` arms timer `0x393` (`FUN_0053fa60(0x393, ...)`). Delay GNPRTB `0x1804` = 300 and `0x1805` = 300 (**DAT**); hyp: which of the two is base and which random is by analogy with `0x392`, the call site is `FUN_00549d60`'s second `FUN_00540230`/`FUN_00540270` pair |
| 2b | Timer `0x393` fires: `FUN_0054e970` calls `FUN_00546fa0` | Roll `FUN_0053e2f0(DAT_006bb5b8)`: GNPRTB `0x1806` = 100 (**DAT**); `FUN_0053e2e0` compares against `FUN_0053e290(DAT_00661a88 - 1)`. hyp: this is a percent roll; `FUN_0053e290` not read | If the roll fails the function returns 1 and does nothing. hyp: whether timer `0x393` re-arms (not found) |
| 2c | Roll passes | Han (`FUN_005435f0`) exists, not a prisoner (`+0xac` bit 0), active (`+0x50` bit 0), **code** | Creates special force `0x3c000009` (`FUN_004f7d50`) on the side of Jabba's Palace (`+0x24 >> 6 & 3` of root `+0xdc`, `FUN_00506ed0`; the seeder `FUN_00519910` stores `0x98000481` there); reveals Han's system to side 2 at level 5 (`FUN_00556c70(0x33000243, 2)`); starts Bounty mission `0x65000083` against Han (`FUN_005479c0`) |
| 2d | Bounty mission effect `FUN_00575de0` | `FUN_00572690(1)` sets Han bit 0 (event `0x212`, `BountyAttack`); `FUN_005349e0` rolls RLEVADTB table 13 (`FUN_0055bfa0`: `FUN_0053e310(0xd, ...)`) against Han's slot `+0x1f0` (see `decoy-roll.md`). Table 13 is a **DAT** table (`RLEVADTB`); not re-read | Caught: result 3, `FUN_004eeb10(Han, 0)` clears `+0xac` bit 8, Han bit 1 (`FUN_00572700`). Not caught: result 2. `FUN_00572690(0)` clears bit 0 at the end. A prisoner of a Bounty mission is moved to Jabba's Palace `0x98000481` by `FUN_00543040` (literal) |
| 2e | Messages. Event `0x212` maps to class `0x1b` (`FUN_0048fa00`) | Han's `+0xac` bit 0 and the side | Han not captured and Alliance: "Han Solo Attacked by Bounty Hunters" / "...who failed to capture him." (`0x7168`/`0x7169`). Han not captured and Empire: if Han's container holds a system (family `0x90`-`0x97`), "Bounty Hunters Locate Solo" / "...located on |name." (`0x716a`/`0x716b`, class `0x2d`); else no message. Han captured, both sides: "Han Solo Captured by Bounty Hunters" / "...taken to Jabba's Palace." (`0x716c`/`0x716d`, class `0x21`). hyp: the message is filed when Han's bit 0 clears, which is why the handler reads his prisoner bit |

#### Chain 3: Jabba's palace (Alliance rescue; the Empire's prisoners)

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 3a | `FUN_005442f0`: Han a prisoner, active, `+0x78` bits 6 and 7 clear, `+0x50` bit `0x1000` clear, container `0x98000481` (literal) | none | Holds bit 6 (`FUN_00543c40(1)`). Handler `FUN_0054a9c0`: clears bit 14 (`FUN_00543fc0(0)`) and reveals Han's system to the Alliance at level 5 (`FUN_00556c70(0x33000243, 1)`) |
| 3b | `FUN_00544130` (Luke, bit 5), `FUN_005443f0` (Leia, bit 7), `FUN_005444e0` (Chewbacca, bit 8): each needs bit 6 and the character free | `FUN_005439e0` | Pulse the bit; handlers `FUN_0054a910` (Luke), `FUN_0054aa60` (Leia), `FUN_0054ab10` (Chewbacca, `0x38000343`) each start Palace mission `0x64000044` with Han `0x33000243` as target (literals). Palace mission chance and effects: `story-missions.md` (GNPRTB `0x180a`, `0x180b`, `0x181a`, `0x1819`) |
| 3c | Messages | none | No message class is mapped for events in this chain (`FUN_00488de0` has no case for the flag setters). hyp: none is filed |

#### Chain 4: Pickup

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 4a | `FUN_005445d0`: Luke, Leia, Han prisoners (`+0xac` bit 0); Chewbacca a prisoner or destroyed (`+0x50` bit 3); some prisoner at `0x98000481` is found (loop over `FUN_004f2640(side object, 1, 2)`) | Vader free and not on a mission, else if Vader is on a mission he may resign (`+0x78` bit 4) and it is not mission family `0x73` (Pickup), `FUN_00534640(1)` asks him to resign | Pulses bit 9 (`FUN_00543d90`). Handler `FUN_0054abc0`: Pickup mission `0x73000082`, Vader `0x35000281`, place `0x98000481` (literals) |
| 4b | Pickup mission effect (`story-missions.md`) | takes every prisoner at the palace | While it runs MissionMgr `+0x58` bit 2 is held (`FUN_0055f6c0`). Event `0x231` (`PickupInProgress`, `FUN_005624a0`, via `FUN_0055f650` setting a flag and calling `FUN_005622e0` on both side views) maps to class `0x27` (`FUN_0048b6e0`): "Jabba Delivers Prisoners" / "Jabba has deliverd Luke, Han, Chewbacca and Leia to Darth Vader. They are now prisoners of the Empire." (`0x7200`/`0x7201`), both sides, picture `0x42a` or `0x42b` by the side's major character's `+0xb0` bit 3 (Dagobah done), bitmap `0x479`/`0x47a`, class `0x1b` (Vader's advice code). hyp: who calls `FUN_0055f650` is not traced |

#### Chain 5: the final battle

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 5a | Luke meets Vader or the Emperor: event `0x360` from `FUN_004f2040` (`CharacterEncounterKeyNotif`). Team members holding a key at `+0x9c` reach `FUN_00588d30` → `FUN_00560200` (`decoy-roll.md`: "not read") | `FUN_00560200`: both parties' `FUN_004ed300` (`+0x54`→`+0x34`) equal and non-zero; `FUN_005349e0` (RLEVADTB 13 vs slot `+0x1f0`, as 2d) | If Luke's Force `+0x8c` is below GNPRTB `0xc05` = 80 (**DAT**) and Luke's `+0xb0` bit 0 is clear: Luke injured (slot `+0x2f0`, cause 10, `FUN_00561b60`). Luke's `+0xb0` bit 0 is set (`FUN_0056f650`). Force experience `+0x8e` of Luke, and of Leia when she has `+0xac` bit 4 and not bit 0, rises by max(constant, f(Vader's or the Emperor's `+0x8c` minus theirs)); constants and scales are GNPRTB `0xc00`..`0xc03`, `0xc0c`..`0xc0f` (25, 20, 20, 20, 1, 1, 1, 1) (**DAT**, loaded by `FUN_0055e5b0`); f is `FUN_0053e190`, not read |
| 5b | `FUN_00544da0` (bit 12, held): bit 13 clear | Luke a prisoner, active, `+0x78` bits 6 and 7 clear, `+0x50` bit `0x1000` clear, container not `0x98000481`, `+0xb0` bit 1 (bit 0 and `+0x8c` at least GNPRTB `0xc06` = 60, `FUN_0056f7a0`); Vader and Palpatine free (`FUN_005439e0`) | Sets bit 12 |
| 5c | `FUN_00544a20` (bit 10) and `FUN_00544be0` (bit 11): bit 12 set | Bit 10: Vader's container differs from Luke's. Bit 11: Vader with Luke, away from the Emperor | Handlers `FUN_0054ac70` (hidden Move mission `0x41000001` for Vader to Luke, `FUN_00547340`) and `FUN_0054adf0` (when Vader and Luke both belong to the Empire (`+0x24` bits 6-7 `0x80`) and neither is on a mission, Move mission for both to the Emperor, `FUN_00547340`) |
| 5d | `FUN_00544fc0`: bit 12 set, Luke, Vader and Palpatine in one container | all three exist and are present | Sets bit 13 (`FUN_00543f50`), runs `FUN_00544da0` again. Notification `MissionMgrFinalBattleNotif` / event `0x220` (`FUN_0054ba00`) |
| 5e | `FUN_0054b040` (the bit 13 handler) | **The Force test:** Luke's `+0x8c` (`(short)param_1[0x23]`) against `DAT_006bb564`, loaded from GNPRTB `0x1807` = 100 (**DAT**, `FUN_0055bfd0`). Alliance and Empire see the same test; it does not depend on side | If the place is a system (family `0x90`-`0x97`) both sides' knowledge of it is raised to level 6 (`FUN_0050d5a0(system, 6, 1)` and `(.., 6, 2)`). Below 100: result 2, Luke's `+0xac` bit 8 cleared (`FUN_004eeb10`), Luke injured (`FUN_00549a20`, slot `+0x2f0`, cause 10, amount `FUN_0055c650`). 100 or more: result 1, Luke slot `+0x218`, Vader and Palpatine slot `+0x20c` (capture), `FUN_004eeb10` on both. Then, when the winner's container is of the other side, the three characters move together by Move mission `FUN_00547480` (`0x41000001`) to the winner's `FUN_00543040` destination; with none, no one moves. No abode is involved (`jail.md`, "The final battle"). hyp: the captor key passed to slot `+0x20c`; the decompile mixes registers (`unaff_EBP`, `unaff_ESI`) |
| 5f | Messages. Event `0x220` and event `0x360` both map to class `0x24` (`FUN_00488de0`; handler `FUN_0048d7f0`) | Event `0x220` sets the "final battle seen" bit `0x10`. For `0x360` the pair is accepted when the first id is Luke or Leia and the second Vader or Palpatine (literals `0x32000242`, `0x31000241`, `0x35000281`, `0x34000280` in `FUN_0048d7f0`); otherwise `FUN_0048e1d0` | Final battle, both sides (`FUN_0048e3c0`): "The Final Battle" `0x71e0`; body `0x71e2` "Luke has been brought before the Emperor by Vader. The Emperor confronted Luke and captured him." when Luke is a prisoner after the test, else `0x71e1` "...Luke confronted and overcame the Emperor. Both Emperor Palpatine and Darth Vader were lost." Picture `0x428` (Luke's `+0xb0` bit 3 set) or `0x427` |

#### Chain 6: the encounter message (before the final battle) and Force discovery

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 6a | Event `0x360` filed as class `0x24` with the pair from 5f | `+0x64` bit 0 (event `0x211`, `LukeKnowsHeritage`, `FUN_0056fbe0`) clear | `FUN_0048df20`: "|Luke Confronts |Vader" / "|Luke has fought |Vader." (`0x71a8`/`0x71a9`) plus one line per party: captured `0x71aa`, injured `0x71ab`, `0x71ac`, `0x71ad`; neither: `0x71ae` "Both combatants escaped uninjured." The party's captured and injured state is read from `+0xac` bit 0 and `+0x94` |
| 6b | Same event, `0x211` already seen, and the pair is exactly Luke (`0x32000242`) and Vader (`0x35000281`) | none | `FUN_0048dc50`: "Luke Discovers His Heritage" / "Luke has learned that Vader is his father." (`0x71a0`/`0x71a1`), then `0x71a2` ("captured by Vader and taken to the Emperor", when both are in the Emperor's container) or `0x71a3`, `0x71a4`/`0x71a5` (Luke injured / escaped) and `0x71a6`/`0x71a7` (Vader captured / injured). hyp: the 0x211 trigger is raised when Luke's `+0xb0` bit 0 is set (slot `+0x3ec` of `FUN_0056f650`), not traced |
| 6c | Timer `0x389` (`FUN_00560a30`, `story-missions.md`) and event `0x362` (`FUN_004f20e0`, ForceUserDiscovered) | `FUN_0048f1e0` (class `0x1c`): both ids in family `0x30`-`0x3b` and the discoverer on the player's side (`FUN_004edc80`) | "Future Jedi Discovered" `0x7178`/`0x7179` (the discoverer cannot teach yet) or `0x717a`; Leia discovered by Luke: "Leia Uses Force" `0x71d8`/`0x71d9` |

#### Chain 7: the Emperor's seat (a seventh piece the six chains omit)

| # | Trigger | Test | Outcome |
|---|---|---|---|
| 7a | Hooks on the Emperor's container (`FUN_005615b0`, `FUN_005616a0`, `FUN_00561340`, `FUN_00560d50`, `FUN_005620e0`) call `FUN_005617b0` | Palpatine (`FUN_005070d0`) not a prisoner and active, his container id is `0x90000109` (Coruscant) and its side is the Empire (`+0x24` bits 6-7 equal `0x80`). All literals, **code** | `FUN_0055f650` sets a flag on the character manager and calls slot `+0x1d8`; event `0x230` (`SeatOfPower`, `FUN_00562450`) maps to class `0x26` (`FUN_0048ce80`), Empire only (**code**: side 2): "Emperor Arrives at Coruscant" / "I have returned to the Seat of Power at Coruscant." (`0x71c8`/`0x71c9`) |

### Movies and text

- **None of these chains plays a movie.** The movie id of a message sits at message `+0x5c`. A search of `0x480000`-`0x4a0000` for stores to `+0x5c` finds four: `0x72` (`FUN_0048d470`, Death Star sabotaged), `0x73` (`FUN_0048e5a0`, "Alliance Headquarters Destroyed", from notification `0x128`), `0x74` (`FUN_0048f790`, "Coruscant Captured"), `0x6f` (`FUN_004950d0`, Death Star destroys a system). `FUN_00421ba0` maps `0x6f` to MDATA 101, `0x73` to 102, `0x74` to 103, `0x72` to 104. None of class `0x24`, `0x25`, `0x1b`, `0x26`, `0x27` sets `+0x5c`. hyp: the consumer of `+0x5c` (movie request `FUN_004019a0`, reached from `FUN_00406000`, `FUN_00422ce0` message `0x482`, `FUN_0059cc60`, `FUN_0059cc80`) was not read to the end. So the port's `story_event_to_cutscene` (`crates/rebellion-app/src/main.rs`) mapping `0x221`, `0x210`, `0x212`, `0x220` to MDATA 101-105 has no support in the executable: 101-104 belong to the Death Star, HQ and Coruscant messages; 105-108 are the four ending movies (`victory-conditions.md`).
- **Text, picture and sound of a message** are filled in the message-class handlers: title and body TEXTSTRA ids at `+0x44`/`+0x50` (`FUN_0060b9d0`), picture at `+0x2e`, bitmap at `+0x30`, voice at `+0x32` from `FUN_004c4990(id)` (advisor voice slot), side filter in the handler. The class number is chosen from the event id by `FUN_00488de0` (a `switch` in code), built by `FUN_00489740` (a `switch` in code), filed by `FUN_0048a060`.

### What no chain has: a new pair (Vader vs Palpatine)

1. **A pair with no Force test in either direction.** Every test above compares Luke's `+0x8c` with a constant (`FUN_0054b040`, `FUN_00560200`). Vader vs Palpatine would need a test between two characters, as the 5a Force-experience gap already does, but with an outcome (who is captured) rather than only an experience change.
2. **A flag bit and a setter.** Bits 0-14 are all taken; the word is 32 bits wide (`uint` at `+0x58`), but each bit needs its own setter function (`FUN_0053a640` with a mask) and broadcaster (`FUN_0054a140`..`FUN_0054a770`) and handler slot (`+0x1c0`..`+0x1ec`).
3. **A check in the `FUN_00549910` chain** and a fixed accessor for Vader and Palpatine (`FUN_00543760`, `FUN_00543800` already exist), with the new check's own location and freedom rules.
4. **A trigger that does not start from Luke.** Both the gate (`FUN_005438a0`) and every check treat Luke, Leia, Han, Chewbacca, Vader and Palpatine as the cast. Vader and Palpatine are already in the gate, so a change event on either already runs the chain.
5. **A message class.** The encounter text class (`0x24`) accepts only first id Luke or Leia and second id Vader or Palpatine (`FUN_0048d7f0`); a Vader and Palpatine pair would need a new case in `FUN_00488de0` and `FUN_00489740`, a handler, and TEXTSTRA entries.
6. **A place.** `ABODESD.DAT` holds five records; any new place needs a record, an executable creation (`FUN_00519910`) and a literal id wherever it is tested. Vader vs Palpatine in the Emperor's seat needs `0x90000109` (Coruscant) literal as in `FUN_005617b0`.
7. **A mission class** if the encounter is a mission (Pickup, Palace, Bounty, Dagobah, Vacation each have a class in the mission class table with its validator, chance and effect).
8. **Side handling.** None of the existing chains is symmetric. The Empire never sees Dagobah, the Alliance never gets a "seat of power" message; a symmetric Vader-vs-Palpatine pair needs a handler per side or a side-neutral one (as class `0x24` is).

## Functions

| Function | Address | Role |
|---|---|---|
| [FUN_00549910](FUN_00549910.c) | `00549910` | Runs the ten checks, ANDed |
| [FUN_005438a0](FUN_005438a0.c) | `005438a0` | Gate: id is one of the six story characters |
| [FUN_005439b0](FUN_005439b0.c), [FUN_005439e0](FUN_005439e0.c) | `005439b0`, `005439e0` | Free character |
| [FUN_005434b0](FUN_005434b0.c), [FUN_00543550](FUN_00543550.c), [FUN_005435f0](FUN_005435f0.c), [FUN_00543690](FUN_00543690.c), [FUN_00543760](FUN_00543760.c), [FUN_00543800](FUN_00543800.c) | `005434b0`-`00543800` | Luke, Leia, Han, Chewbacca, Vader, Palpatine by literal id |
| [FUN_00543af0](FUN_00543af0.c), [FUN_00543b60](FUN_00543b60.c), [FUN_00543fc0](FUN_00543fc0.c) | `00543af0`.. | Flag setters (bits 0, 2, 14; others in the same family) |
| [FUN_00544030](FUN_00544030.c), [FUN_00544130](FUN_00544130.c), [FUN_005442f0](FUN_005442f0.c), [FUN_005443f0](FUN_005443f0.c), [FUN_005444e0](FUN_005444e0.c), [FUN_005445d0](FUN_005445d0.c), [FUN_00544a20](FUN_00544a20.c), [FUN_00544be0](FUN_00544be0.c), [FUN_00544da0](FUN_00544da0.c), [FUN_00544fc0](FUN_00544fc0.c) | `00544030`-`00544fc0` | The ten checks |
| [FUN_0054a800](FUN_0054a800.c), [FUN_0054a860](FUN_0054a860.c), [FUN_0054a910](FUN_0054a910.c), [FUN_0054a9c0](FUN_0054a9c0.c), [FUN_0054aa60](FUN_0054aa60.c), [FUN_0054ab10](FUN_0054ab10.c), [FUN_0054abc0](FUN_0054abc0.c), [FUN_0054ac70](FUN_0054ac70.c), [FUN_0054adf0](FUN_0054adf0.c), [FUN_0054b740](FUN_0054b740.c) | `0054a800`-`0054b740` | Bit handlers: start the story missions |
| [FUN_0054b040](FUN_0054b040.c) | `0054b040` | Final battle: Force test, capture, group Move |
| [FUN_00546f80](FUN_00546f80.c), [FUN_0054e620](FUN_0054e620.c) | `00546f80`, `0054e620` | Timer `0x392` fire → bit 0 |
| [FUN_00546fa0](FUN_00546fa0.c), [FUN_0054e970](FUN_0054e970.c) | `00546fa0`, `0054e970` | Timer `0x393` fire → bounty roll, force, mission |
| [FUN_00549d60](FUN_00549d60.c), [FUN_0053fa60](FUN_0053fa60.c), [FUN_00540230](FUN_00540230.c), [FUN_00540270](FUN_00540270.c) | `00549d60`.. | New-game arming of `0x392`, bit 14; timer delay fields |
| [FUN_005617b0](FUN_005617b0.c), [FUN_0055f650](FUN_0055f650.c), [FUN_00562450](FUN_00562450.c) | `005617b0`.. | Emperor at Coruscant, event `0x230` |
| [FUN_00560200](FUN_00560200.c), [FUN_00588d30](FUN_00588d30.c) | `00560200`, `00588d30` | Keyed encounter (Luke/Leia vs Vader/Emperor) |
| [FUN_0055bfd0](FUN_0055bfd0.c), [FUN_0055e5b0](FUN_0055e5b0.c), [FUN_0053e390](FUN_0053e390.c) | `0055bfd0`.. | GNPRTB `0x1800`-`0x182e` and `0xc00`-`0xc0f` loaders |
| [FUN_00488de0](FUN_00488de0.c), [FUN_00489740](FUN_00489740.c), [FUN_0048a060](FUN_0048a060.c) | `00488de0`.. | Event id to message class; factory; filing |
| [FUN_0048d7f0](FUN_0048d7f0.c), [FUN_0048dc50](FUN_0048dc50.c), [FUN_0048df20](FUN_0048df20.c), [FUN_0048e1d0](FUN_0048e1d0.c), [FUN_0048e3c0](FUN_0048e3c0.c) | `0048d7f0`.. | Class `0x24` (encounter, final battle) |
| [FUN_0048d190](FUN_0048d190.c), [FUN_0048d0f0](FUN_0048d0f0.c), [FUN_0048fa00](FUN_0048fa00.c), [FUN_0048ce80](FUN_0048ce80.c), [FUN_0048b6e0](FUN_0048b6e0.c), [FUN_0048f1e0](FUN_0048f1e0.c) | `0048d190`.. | Classes `0x25` (Dagobah), `0x1b` (bounty), `0x26` (Emperor), `0x27` (pickup), `0x1c` (Force discovery) |
| [FUN_00421ba0](FUN_00421ba0.c), [FUN_004146e0](FUN_004146e0.c), [FUN_004213e0](FUN_004213e0.c) | `00421ba0`.. | Movie id to MDATA file; movie window |

## Evidence

- Chain order, gate, bit setters: `FUN_00549910.c`, `FUN_005438a0.c`, `FUN_00543af0.c`..`FUN_00543fc0.c`, caller lists from `DecompileTargets.java`.
- Timers: `FUN_00549d60.c` (arming `0x392`), `FUN_0054e620.c`, `FUN_0054e970.c`, `FUN_0053fa60.c`; clamp words read with `DumpMemory.java` at `0x00661a8c` (`ff 3f 00 00`) and `0x00661a90` (`ff 7f 00 00`).
- GNPRTB: id to variable in `FUN_0055bfd0.c` (`0x1800`-`0x182e`) and `FUN_0055e5b0.c` (`0xc00`-`0xc0f`); values from `base/json/GNPRTB.json` (one value per id across all eight difficulty columns for every id used here).
- Messages: event to class in `FUN_00488de0.c`, vtables dumped with `DecompileVtable.java` (`0x0065b338` class `0x26`, `0x0065b380` class `0x25`, `0x0065b3c8` class `0x23`, `0x0065b408` class `0x24`, `0x0065b5a8` class `0x1b`); text from `TEXTSTRA.DLL` RT_RCDATA (read by resource id).
- Movies: `+0x5c` stores found with `FindOperandText.java` over `0x480000`-`0x4a0000` ("0x5c],0x6f" through "0x74"); `FUN_00421ba0.c` switch.
- ABODESD.DAT: 136 bytes, 5 records of 24 bytes with ids `0x440`, `0x441`, `0x442`, `0x480`, `0x481` (family byte `0x98`); the executable also builds them in `FUN_00519910` (`story-missions.md`).

## Open questions

See Proposed nodes in the report; in short: the re-arm of timer `0x393`, `FUN_0053e290` and `FUN_0053e170` (the roll and the experience scale), who sends `0x211` and `0x231`, the captor key in `FUN_0054b040`, mission slot `+0x1d8`, and the consumer of message `+0x5c`.

## Mod surface

What a mod adding encounter-style content would have to change in `REBEXE.EXE`:

- **Data only, today.** GNPRTB `0x1802`-`0x1807` (timer delays, bounty roll, final-battle Force threshold) and `0xc00`-`0xc0f` (Force experience): these change when and how hard the existing chains fire. RLEVADTB table 13 (the capture roll). ABODESD records and TEXTSTRA text (names and every message string above). Nothing in a DAT adds a chain.
- **Code, for a new chain.** A new flag bit with setter, broadcaster and handler slot; a new check in the `FUN_00549910` chain (a new function, plus the call in `FUN_00549910`); handler code to start the mission; literal ids for every character and place; a message class (a `FUN_00488de0` case, a `FUN_00489740` case, a handler class with a vtable, TEXTSTRA entries); a movie id (`FUN_00421ba0` case) if a movie is wanted, plus MDATA files.
- **Fixed cast.** The gate `FUN_005438a0`, the accessors `FUN_005434b0`..`FUN_00543800` and the encounter pair filter in `FUN_0048d7f0` name six characters by literal id. A different cast needs those edited, not a table row.
- **Fixed places.** Jabba's Palace `0x98000481` is a literal in six functions (`FUN_00543040`, `FUN_005442f0`, `FUN_00544da0`, `FUN_0054abc0`, `FUN_0056d4f0`, `FUN_0056df70`) and Coruscant `0x90000109` in `FUN_005617b0`.

Proposed follow-up questions:

1. Does timer `0x393` re-arm after a failed roll, and what do `FUN_0053e290` and `FUN_0053e170` compute (is the Force-experience gain a percent of the gap)? Start from `FUN_0053e2e0`, `FUN_0053e290`, `FUN_0053e170`, `FUN_00546fa0`.
2. Who raises events `0x211` (`LukeKnowsHeritage`) and `0x231` (`PickupInProgress`)? Start from `FUN_0056fbe0`, `FUN_0056f650` slot `+0x3ec`, `FUN_0055f650`, `FUN_005622e0`.
3. In `FUN_0054b040`, which register holds the captor key passed to slot `+0x20c` on Vader and Palpatine? Start from `FUN_0054b040`.
4. What consumes message `+0x5c` and which movie does each of the four `+0x5c` ids play and when? Start from `FUN_004019a0`, `FUN_00406000`, `FUN_00405560`.
5. What does Dagobah's slot `+0x1d8` do when Han reaches the palace? Start from `FUN_00544130`, mission class `0x71000043`.
6. Which events (`0x307`, `0x312`, `0x30a`) call `FUN_00545df0`, `FUN_00545e90`, `FUN_00546370`, and can any story check be skipped by an event that does not pass the `FUN_005438a0` gate? Start from `FUN_00515480`, `FUN_00541520`, `FUN_00516220`.
