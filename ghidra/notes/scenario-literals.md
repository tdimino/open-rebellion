---
title: "Scenario Literals: The Object Ids the Executable Names"
description: "Every character, system, facility, mission and troop id REBEXE names directly, grouped by system (victory, seeding, game types, recruitment, research, AI and advisor, story), with what a scenario replacing each one breaks and whether a DAT field could stand in"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [scenario, literals, victory, seeding, coruscant, yavin, characters, research, mod-surface]
---

# Scenario literals

Answers task-graph node `or-q-mod-scenario-mlpg` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-scenario`; the victory, research-milestone, AI-marker and stormtrooper literals were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code.

## Answer

**Method.** `SweepDatIds.java` lists every instruction scalar and every initialised non-code dword whose top byte is `0x10`-`0xfa` and whose low 24 bits are below `0x1000`: 1,050 hits. About 330 of them are bare masks (`0x10000000`, `0x20000000`, `0x40000000`, `0x80000000`, `0xbf000000`, `0x3f000000` floats and the like) and are dropped. The rest are 120 or so real ids in the functions below. `SweepFamilyChecks.java` lists the 1,200 places that shift an id right by 0x18 and the compare constants that follow. Their ranges are dominated by `0x90`-`0x98` (systems and abodes; 396 sites), `0x80`-`0x90` (sectors) and `0x30`-`0x40` (characters), nearly all in the interface and the AI planners. Those range checks only separate whole object classes; none names a particular id. The one family check that is a game rule is the Alliance HQ's `0x20`-`0x21` (`FUN_00506110`, `FUN_004f3330`).

**What the executable fixes, in five lines.**

1. **Seven characters are literals everywhere they matter.** Luke `0x32000242`, Mon Mothma `0x30000240`, Vader `0x35000281` and Palpatine `0x34000280` decide victory (`FUN_005180e0`). With Leia `0x31000241`, Han `0x33000243` and Chewbacca `0x38000343` they drive the story and the new-game placement (`FUN_0055f7b0`). No DAT field marks "this is the leader".
2. **Two systems are literals.** Coruscant `0x90000109` is the Empire's reserved capital, the Alliance's victory target and the Emperor's seat; Yavin `0x92000121` is the Alliance's reserved start system and the rebel-base anchor of the character seeding. Each is named by the control dealer, the seeding steps, the victory check, the AI and the advisor (tables below). The Alliance HQ's object id `0x20000005` and family range `0x20`-`0x21` are fixed too.
3. **The tables that decide what starts where are DAT.** The CMUN*TB, FACL*TB and SYFC*TB seed tables, the SDPRTB percentages, the maintenance budget, and which minor characters exist for each side (class flags `+0x40` Alliance and `+0x44` Empire) all come from DAT files. A mod can replace those freely. The registry ids binding CMUN*TB to the seeding steps are in the executable.
4. **Research is data-driven but tied to three fixed trees.** The ship tree reads producer family `0x28` and the facility tree `0x2a` (`FUN_0051cc30`, `FUN_0051cbd0`, `FUN_0051cc10`); the three trees are the MISSNSD ids `0x53000020`-`22` (`FUN_0056c460`). Orders, costs and per-side flags are class fields (DAT). Three milestones are named: the CC-7700 Frigate `0x1400004a` (Alliance), the Interdictor Cruiser `0x14000082` (Empire) and the Death Star Shield `0x25000004` raise an advisor flag (`FUN_004198a0`, `FUN_004197b0`).
5. **The game type is read by `FUN_005180e0`, `FUN_004182a0` (AI context bit 0) and the status window's Victory Conditions page (`FUN_0044b2d0`, `FUN_0044bb70`)** (Standard = 0, Headquarters Only = 1; `victory-flag-word.md`, 6).

### Victory (`FUN_005180e0`, `victory-conditions.md`)

| Literal | Address | Names | A scenario replacing it breaks | DAT stand-in? |
|---|---|---|---|---|
| `0x32000242` | `FUN_005180e0`@0051812f; `FUN_00507010`@00507027 (getter; callers `FUN_005434b0`, `FUN_005751d0`) | Luke Skywalker | LC bit: the Standard Empire win (AD and LC and MC) needs Luke captured. If the id does not exist, `FUN_005180e0` returns 0 and the win word never changes | No. The id is a constant; there is no "leader" flag |
| `0x30000240` | `FUN_005180e0`@0051815a | Mon Mothma | MC bit; same missing-object result | No |
| `0x35000281` | `FUN_005180e0`@00518188; `FUN_00507070`@00507087 (caller `FUN_00543760`) | Darth Vader | DC bit (Alliance win) | No |
| `0x34000280` | `FUN_005180e0`@005181b6; `FUN_005070d0`@005070e7 (callers `FUN_00543800`, `FUN_005617b0`) | Emperor Palpatine | EC bit | No |
| `0x90000109` | `FUN_005180e0`@005181e4 (via `FUN_00505f60`) | Coruscant | CA and CD bits: no Alliance win | No (the system id is the literal) |
| family `0x20`-`0x21` | `FUN_00506110`@`{0x20,0x22}`; `FUN_0053f030` | the Alliance HQ: first object of the family | AD bit: no Empire win | Partly: ALLFACSD holds one record (`0x20000001`, "Alliance Headquarters"). A second facility class in family `0x20`/`0x21` is ignored (first object only) |
| game type `0`/`1` | `FUN_005180e0`@005182bd (root `+0x60`) | Standard / Headquarters Only | any other value: returns 0, the win word never updates | No; the new-game screen sets it from a fixed 1/2 (`FUN_004142d0`) |

Side branch: the Empire wins on AD (plus LC and MC in Standard), the Alliance on CA or CD (plus DC and EC in Standard). Each side reads the flags only through its own advisor (`FUN_004c0f60` Empire, `FUN_004c3060` Alliance), whose ending scripts also name leaders (see AI targeting).

### Seeding and HQ placement (`FUN_005136d0` stages, `starting-forces-seeding.md`)

| Literal | Address | Names | A scenario replacing it breaks | DAT stand-in? |
|---|---|---|---|---|
| `0x90000109` | `FUN_00519d00`@00519edd; `FUN_0051a280`@0051a42f; `FUN_0051a870`@0051a88e; `FUN_0051ae20`@0051ae3e (step 4); `FUN_0051b080`@0051b09f (step 6); `FUN_0055f7b0`@0055f7d9 | Coruscant | `FUN_00519d00` removes Coruscant from the dealt pool and reserves it for the Empire (one strong slot less); CMUNCRTB and the Empire fleet (CMUNEFTB) go to it; Palpatine goes to it | No for the reservation and the placements; the seed *contents* (CMUNCRTB, CMUNEFTB, FACLCRTB) are DAT |
| `0x92000121` | `FUN_00519d00`@00519e95; `FUN_0051a280`@0051a3f6; `FUN_0051acb0`@0051acce (step 2); `FUN_0051af00`@0051af52 (step 5); `FUN_0051b990`@0051bba6; `FUN_0055f7b0`@0055f806 | Yavin | reserved for the Alliance by the dealer; CMUNYVTB and the Alliance fleet (CMUNAFTB) go to it; `FUN_0051b990` (stage 0x18) calls `FUN_0050d5a0(Yavin,10,2)`; Luke, Leia, Han and three Alliance minors go to it | No (literal). Contents are DAT |
| `0x20000005` | `FUN_00513a50`@00513b6e (stores root `+0xb4`); `FUN_0051a680`@0051a809 (step 3); `FUN_004bf9e0`@004bfa13, `FUN_004c05e0`@004c0641, `FUN_004c30c0`@004c320e (Alliance HQ relocation module and advisor tour); `FUN_0050ad80`@0050af01 | the Alliance HQ object, an instance id | no HQ object resolves: victory, relocation and the HQ garrison step fail | No. hyp: the instance id comes from the FACLHQTB item `0x20000001` and the registry; a second HQ class would not be found |
| `0x10000002`, `0x10000008` | `FUN_0051ab20`@0051abc9, 0051aba4 (step 1) | Alliance Army Regiment, Imperial Army Regiment | low-support garrison on every held system | No. A mod replacing the Army regiment classes with other ids leaves step 1 creating a regiment of that id; if the id is absent the step creates nothing (hyp: untraced when the class is missing) |
| `0x2c000001` | `FUN_00559850`@005598a2, `FUN_00559a60`@00559ab2 | Mine (PROFACSD 1) | the mine roll writes this class directly; the refinery (`0x2d000002`) is found by the same path in `FUN_004db2f0`/`FUN_004d6e70` (family bits `0x2c`/`0x2d`) | No for the mine; the mix of mines and yards after it is SYFCCRTB/SYFCRMTB (DAT) |
| CMUN*TB registry ids `0x50`-`0x54`, `3`, `4`, `0x55`, `0x56` | `FUN_0055d520`..`FUN_0055d5e0`, `FUN_0055d730`, `FUN_0055d780`; `FUN_0055cb60` binds GNPRTB `0x140a`-`0x1413` | which file feeds which step | a scenario can replace the file, not the step it feeds | The *table content* is DAT; the step each table feeds is code |
| SDPRTB `5168`-`5170`, `7680`, `7681` | `FUN_0055d670`, `FUN_0055d6d0`, `FUN_005597e0` | the maintenance budget and strong/weak share per side | none | Yes: SDPRTB (values per difficulty and side; Alliance and Empire columns) |

**Where characters start (stage 0x15, `FUN_0055f7b0`, read in full).** Stage `0x14` (`FUN_0055f730` → `FUN_00562520`) creates every class of families `0x30` and `0x38` as an object, with the side from the class flags: `+0x40` set only, Alliance; `+0x44` set only, Empire; both set or neither, not created. Stage `0x15` then places and marks recruited (`FUN_0055fe70`: slot `+0xa8` places, `FUN_004f7480(1)`, `FUN_004f74f0(1)` set bits 1 and 2 of `+0x50`) as follows:

| Character | Address (immediate) | Placed |
|---|---|---|
| Palpatine `0x34000280` | `FUN_0055f7b0`@0055f85c | Coruscant (`0x90000109`, `FUN_0055fe70`) |
| Vader `0x35000281` | @0055f890 | **a random Empire system or fleet's capital ship** (`FUN_00562680` → `FUN_00562710`; `maintenance-and-vader-start.md` section 2) |
| `0x38000380`, `0x38000381`, `0x38000382`, `0x38000383`, `0x3800038c` (Jerjerrod, Ozzel, Piett, Veers, Needa) | @0055f8c8, f900, f938, f970, f9a8 | each by the same random Empire rule |
| Mon Mothma `0x30000240` | @0055fa41 | the HQ's system (`*(HQ+0x1c)`, where `FUN_00506110` finds the HQ) |
| Luke `0x32000242`, Leia `0x31000241`, Han `0x33000243`, Chewbacca `0x38000343`, Jan Dodonna `0x38000344`, Wedge Antilles `0x38000341` | @0055fa7e, fabb, faf8, fb35, fb72, fbaf | Yavin (`0x92000121`) |
| N more random characters per side | `FUN_0055d7d0(side, galaxy size)` → `FUN_00562610` → `FUN_0055fc80` | N = SDPRTB `0x1400`/`0x1401`/`0x1402` (5120-5122: 1, 2, 4 for both sides at galaxy size 1, 2, 3), each drawn from the side's not-yet-recruited minor characters (family `0x38`-`0x3b`, `+0x50` bit 1 clear) and placed on a random system or ship (`FUN_00562710`) |

Both sides: the Empire list runs first (`FUN_0055d7d0(2, …)`), then the Alliance's. A random target is drawn from `n_systems + n_fleets` of the side: an index below `n_systems` picks a system, else a fleet and then a random child (`FUN_00562710`; hyp: the child is a capital ship, since `FUN_00502e30` iterates the fleet's mode-1 children). Replacing Yavin or Coruscant by another system (DAT: SYSTEMSD) keeps no named character in place: the literals stay.

### Game types (Standard, Headquarters Only)

| Literal | Address | Names | Breaks | DAT stand-in? |
|---|---|---|---|---|
| `1`, `2` (new-game screen), `0`, `1` (game type) | `FUN_00414830` state 7 (`FUN_0051c740` copies the app's `+0xec` to settings `+8`), `FUN_004142d0`, `FUN_005180e0`@005182bd | the two game types | a third game type would return 0 from `FUN_005180e0` | No |

Everything else about the type is in `victory-conditions.md`. A mod that changes what "Standard" needs for each side must patch the flag expression in `FUN_005180e0`; leaders and Coruscant are not parameters.

### Recruitment pool and unique characters

| Literal | Address | Names | Breaks | DAT stand-in? |
|---|---|---|---|---|
| families `0x30`, `0x38` | `FUN_0055f730` via `FUN_0051cad0(0x30)`, `FUN_0051cad0(0x38)` | every major and minor class is made an object at stage `0x14` | a new character family byte (for example `0x39`) is not created at game start | No for the two families; yes for adding characters within them (MNCHARSD rows, side flags `+0x40`/`+0x44`) |
| family range `0x38`-`0x3c` | `FUN_0056f450`; `FUN_0055ef30`, `FUN_0055fc80` | the minor-character pool the random draw reads | same | Yes: the pool is whatever MNCHARSD holds, minus those placed by name |
| `0x38000341`, `0x38000343`, `0x38000344`, `0x38000380`-`83`, `0x3800038c` | `FUN_0055f7b0` (above) | the named minors | none if absent: the placement is skipped (hyp: `FUN_0055fe70` fails for a missing object and the stage returns 0, which would abort the seeding; not traced) | No |
| `0x38000343` Chewbacca | `FUN_00543690`@005436c8, `FUN_005438a0`@0054391f, `FUN_0054ab10` (recovered), `FUN_0056e1f0`@0056e2f5 | story membership (below) | the Pickup condition and Palace roster | No |
| `0x3c000009` Bounty Hunters | `FUN_00546fa0`@00547058; `FUN_004955b0`@004957a6; `FUN_00545e90`@00545f4f; `FUN_00575ae0`, `FUN_00575cc0` | the special force the bounty timer creates | the Bounty mission has no force to create | Partly: SPECFCSD holds it; the id is the literal |
| `0x31000241`, `0x32000242`, `0x33000243`, `0x34000280`, `0x35000281`, `0x30000240` as a *set* | `FUN_004025f0`; subset of four `FUN_004024d0`; pair `FUN_00402cf0` (Luke, Vader) | "is a main character": AI object record flags (`FUN_00401d20`, `FUN_00484630`) | new leaders get no main-character treatment in the AI record or its sighting | No |

### Troop and special-force ids in rules and interface

These are named for the interface or for one rule, and a mod adding a type has to add it by hand.

| Literal | Address | Role |
|---|---|---|
| `0x10000006` Stormtrooper Regiment | `FUN_005091f0`@00509230 (counts Empire stormtroopers in a system); callers `FUN_0050d030` (uprising incident), `FUN_00569b90` (Subdue mission chance) | an input to the uprising table (UPRIS1TB/2TB via `FUN_00559ce0`) and to the Subdue roll (`FUN_0055c780`). A replacement class changes both. `FUN_0050d030`'s other callers and `FUN_005719d0` (Incite) and `FUN_00573ff0` (Diplomacy) use the same count: hyp: they are the same rule, since `FUN_005091f0` is the only reader |
| `0x10000001`-`0x1000000a` | `FUN_0046d8d0` (TroopFinder row), `FUN_0042c3b0` (`0x10000002`, `0x10000008` map markers) | five columns per side. A sixth troop type shows in no column |
| `0x3c000001`-`0x3c000008` | `FUN_00402e80` (AI class bit), `FUN_00465540` (PersonnelFinder), `FUN_00472cc0` (profile fill), `FUN_0042c3b0` (`0x3c000003`, `0x3c000005`) | four columns per side; the AI record sorts the eight types into four capability bits (`+0x34 |= 0x10000000`..`0x80000000`) |
| `0x10000004`, `0x10000005`, `0x21000000`, `0x1000000`, `0x2000000` (call args) | `FUN_004c7650`@004c767a-004c77bd | these are *query flag arguments* to `FUN_0049d610`/`FUN_0049da10` for the garrison module, not troop ids (as `manage-automation.md` records) |

### Research trees and per-side buildability

| Literal | Address | Names | Breaks | DAT stand-in? |
|---|---|---|---|---|
| `0x53000020`, `0x53000021`, `0x53000022` | `FUN_0056c460`@0056c471; `FUN_00402720`@00402b3a; `FUN_0047bba0`, `FUN_0047c750`, `FUN_0047c170` | the three research missions: ship, facility, troop trees (tree index 0, 1, 2) | a fourth tree has no mission, points slot or level field (side `+0x90`-`+0xa4`) | No for the three; MISSNSD holds their timers and flags |
| producer family `0x28`, `0x28`, `0x2a` | `FUN_0051cc30`, `FUN_0051cbd0` (ship: family `0x28`, MANFACSD), `FUN_0051cc10` (`0x2a`: construction) | which facility family a tree's orders are read from (`FUN_0051cb90`, `FUN_0051cb60`) | `FUN_005326f0`/`FUN_005327f0`/`FUN_005328f0` call these; the troop tree's family `0x29` (training) is the third caller in the same family (`research.md`) | No |
| class `+0x50` order, `+0x40` Alliance flag, `+0x44` Empire flag, `+0x54` cost | `FUN_0052e580`, `FUN_00567ba0` | the unlock rule and per-side buildability | none | **Yes**: all per-class DAT fields |
| `0x1400004a` CC-7700 Frigate (Alliance), `0x14000082` Interdictor Cruiser (Empire) | `FUN_004198a0`@004198ee, 0041993b | researching that class sets side-controller `+4` bit `0x1000000` (an advisor milestone flag) | the advisor reaction for "first major ship tech" never fires for a replacement | No. The flag is milestone logic keyed by id |
| `0x25000004` Death Star Shield | `FUN_004197b0`@004197f5 | the same flag for the facility tree (`FUN_0051cc10`) | same | No |

The DAT stand-in for what a side may build is already complete: research orders, per-side flags and costs per class, and the producer facility's family. What a mod cannot add is a fourth tree or a different set of three trees.

### Advisor tour, AI targeting and the Coruscant special cases

| Literal | Address | Names | Breaks | DAT stand-in? |
|---|---|---|---|---|
| `0x90000109`, `0x92000121` | `FUN_004c0fc0` (Empire advisor: `0x109`, `0x121` by `FUN_005f55d0`); `FUN_004c30c0` (Alliance advisor: `0x109`, `0x121`) | the tour's list of systems | a replacement system is shown with the old caption | No |
| `0x34000280`, `0x35000281`, `0x32000242`, `0x30000240` | `FUN_004c0fc0`@004c1177, 004c12c2; `FUN_004c30c0`@004c3374, 004c34be | the ending scripts: which character the advisor looks at in the win screens. Empire: Palpatine (case 7), Vader (case `0x13`); Alliance: Luke (case `0x13`), Mon Mothma | none | No |
| `0x90000109` | `FUN_004d76f0` (resource AI: Coruscant treated apart from other mines/refineries); `FUN_004e5870` (strategy state 6); `FUN_004c94c0`@004c9780 (a system-level check); `FUN_00433e40` (AI world model: tests the system id's low 24 bits against `0x109` and sets `+0x30 |= 0x40000000`, then `0x121` for `+0x30 |= 0x8000000`; no 32-bit literal, so the sweep missed it); `FUN_004ebe40` (system assessment: low 24 bits `0x109` sets `+0x28 |= 0x20000000`) | the AI treats Coruscant as a special system (garrison and mine rules) | a different capital loses those special cases | No |
| `0x121` (low 24 bits) | `FUN_00433e40` (flag `0x8000000`) | the AI world model's rebel-base marker | same | No |
| `0x90000109` | `FUN_005617b0`@0056182d (Seat of Power); `FUN_00573610`@0057378e (Espionage reveal roll: target is Coruscant and Empire-held) | the Emperor's presence at Coruscant makes `FUN_005617b0` set its flag (`FUN_005070d0` is Palpatine); the Espionage reveal-count roll gets a bigger count when the target is the Empire's Coruscant (`FUN_0055c940`) | both | No |
| `0x90000109` | `FUN_00488de0`@00489117, 00489154; `FUN_0048f790`@0048f7bc, 0048f7ec | message center: notification `0x128` (win word) and the event `0x150` text refer to Coruscant | interface text only | No |
| `0x20000005` | `FUN_004bf9e0`, `FUN_004c05e0` | Alliance HQ relocation module (side 1 only; the Empire branch does nothing) | Alliance AI cannot move an HQ of another id | No |

### Story (literals only; traced in `story-missions.md`, `or-q-mod-encounters-36xh`, `or-q-mod-force-dqed`)

| Literal | Addresses | Names |
|---|---|---|
| `0x98000440`, `0x98000441`, `0x98000442`, `0x98000480`, `0x98000481` | `FUN_00513a50`@00513c9a-00513d62 (root `+0xcc`-`+0xdc`); `FUN_00519910`@00519a74-00519b4c (creation); `FUN_00549580`; `FUN_00574b10`@00574bb1; `FUN_00571f20`@00571fc1; `FUN_00543040`@0054316f; `FUN_005442f0`@0054437d; `FUN_00544da0`@00544f14; `FUN_0056d4f0`@0056d591; `FUN_0056df70`@0056e011; `FUN_00490340`@004905e4 | Alliance Jail, Dagobah, Hermitage, Empire Jail, Jabba's Palace (ABODESD, owners 1, 3, 3, 2, 2) |
| `0x32000242`, `0x31000241`, `0x33000243`, `0x38000343`, `0x35000281`, `0x34000280` | `FUN_005438a0`, `FUN_00544030`-`FUN_00544fc0` family, `FUN_0054a860`-`FUN_0054adf0`, `FUN_00560200`, `FUN_00560a30`, `FUN_0056e1f0`, `FUN_0056df70`, `FUN_00575900`, `FUN_005425e0`, `FUN_00546fa0`, `FUN_0058a9e0`, `FUN_0058ab00` | Luke, Leia, Han, Chewbacca, Vader, Palpatine in the story-flag checks and encounters |
| mission ids `0x41000001` Move, `0x42000002` Return, `0x43000003` Autorouting, `0x44000004` Adrift | `FUN_0053c810`@0053c95b, `FUN_00547180`, `FUN_00547340`, `FUN_00547480` (Move); `FUN_005475f0`, `FUN_00547740`, `FUN_00547880` | the engine-owned missions the story launches |
| `0x71000043` Dagobah, `0x64000044` Palace, `0x73000082` Pickup, `0x65000083` Bounty, `0x72000045`/`46` (Vacation, Sabbatical) | `FUN_00548840`, `FUN_0054a860`, `FUN_0054a910`, `FUN_0054abc0`, `FUN_00546fa0` | the six story mission classes (all hidden in MISSNSD) |

A scenario replacing any of these characters or abodes loses the matching story chain, which is a code-level feature rather than data (`story-missions.md`).

### Singletons (the game root's fixed ids)

`FUN_00513a50` stores these into the game root: `0xf2000003` (`+0xb0`), `0x20000005` (`+0xb4`), `0xf8000006` (`+0xb8`), `0xf9000007` (`+0xbc`), `0xfa000008` (`+0xc0`, the character manager read by `FUN_00506e20`), `0xf300000f` and `0xf3000010` (`+0xc4` and next: the two side objects), and the five abodes. `FUN_00519910` creates them (`0xf2000002`, `0xf8000008`, `0xf9000009`, `0xfa00000a`, `0xf3000003` twice, the abodes). They are not scenario content; a scenario cannot change them, and none needs to.

### Chain of stage numbers (`FUN_005136d0`)

Stages 2-0x19: `8` `FUN_00519800` (`0xf1000001`), `9` `FUN_00519910` (singletons and abodes), `0xa` `FUN_00519c20`, `0xb` `FUN_00519d00` (control dealing; Coruscant, Yavin, the HQ system), `0xc` `FUN_0051a280` (systems through the seven lists, `galaxy-limits.md`), `0xd` `FUN_0051a530`, `0xe` `FUN_0051a550`, `0xf` `FUN_0051a570`, `0x10` `FUN_0051a640`, `0x11`/`0x13` `FUN_0051b750`, `0x12` `FUN_0051aa50` (forces), `0x14` `FUN_0051b6d0`, `0x15` `FUN_0051b710`, `0x17` `FUN_0051b7f0`, `0x18` `FUN_0051b940`, `0x19` `FUN_0040f340`. Stages `0x14`/`0x15` run `FUN_0055f730` (create characters by class) and `FUN_0055f7b0` (place the named ones), above.

## Functions

| Function | Address | Role |
|---|---|---|
| `FUN_005180e0` | 005180e0 | win-flag word: Luke, Mon Mothma, Vader, Palpatine, Coruscant, HQ, game type |
| `FUN_00506110` | 00506110 | the Alliance HQ: first object of family `0x20`-`0x21` |
| `FUN_00505190` / `FUN_00505f60` | 00505190 / 00505f60 | character / system lookup by id |
| `FUN_004024d0` / `FUN_004025f0` / `FUN_00402cf0` | 004024d0 / 004025f0 / 00402cf0 | "is a leader" (4 ids), "is a main character" (6), "is Luke or Vader" |
| `FUN_00507010` / `FUN_00507070` / `FUN_005070d0` / `FUN_00506f50` / `FUN_00506fb0` / `FUN_004f3e50` / `FUN_004f3eb0` / `FUN_004f3f10` | 00507010 etc. | constant-id getters: Luke, Vader, Palpatine, Han, Leia, and Luke, Vader, Palpatine by side lookup |
| `FUN_005136d0` | 005136d0 | seeding driver, stages 2-0x19 |
| `FUN_00513a50` | 00513a50 | root children by id: singletons, HQ `0x20000005`, abodes |
| `FUN_00519d00` | 00519d00 | control dealing: reserves Coruscant (Empire) and Yavin (Alliance), picks the HQ system |
| `FUN_0051a280` / `FUN_0051bc70` | 0051a280 / 0051bc70 | stage `0xc`: creates characters from the seven lists |
| `FUN_0055f730` / `FUN_00562520` | 0055f730 / 00562520 | stage `0x14`: create every `0x30`/`0x38` class by its side flags |
| `FUN_0055f7b0` | 0055f7b0 | stage `0x15`: places Palpatine, Vader, five Imperial officers, Mon Mothma, Luke, Leia, Han, Chewbacca, Dodonna, Wedge, then random extras |
| `FUN_0055fe70` / `FUN_00562680` / `FUN_00562610` / `FUN_00562710` / `FUN_0055fc80` / `FUN_0055d7d0` / `FUN_0055ef30` | see names | place and mark recruited; random system-or-ship pick; random recruit; counts from SDPRTB `0x1400`-`0x1402` |
| `FUN_0051aa50` etc. (steps 1-7) | 0051ab20, 0051acb0, 0051ad90, 0051ae20, 0051af00, 0051b080, 0051b1f0 | forces seeding (`starting-forces-seeding.md`); step 1 names `0x10000002`/`0x10000008` |
| `FUN_005091f0` | 005091f0 | counts Empire `0x10000006` regiments in a system |
| `FUN_0056c460`, `FUN_0051cc30`, `FUN_0051cbd0`, `FUN_0051cc10`, `FUN_0051cb90`, `FUN_005326f0` | see names | research tree selectors |
| `FUN_004198a0` / `FUN_004197b0` | 004198a0 / 004197b0 | advisor milestones for CC-7700, Interdictor, Death Star Shield |
| `FUN_004c0fc0` / `FUN_004c30c0` | 004c0fc0 / 004c30c0 | advisor tour and ending scripts naming Coruscant, Yavin and leaders |
| `FUN_004bf9e0` / `FUN_004c05e0` | 004bf9e0 / 004c05e0 | Alliance HQ relocation (id `0x20000005`) |
| `FUN_004d76f0`, `FUN_004e5870`, `FUN_004c94c0`, `FUN_00433e40`, `FUN_004ebe40` | see names | AI special cases for Coruscant and Yavin |
| `FUN_00573610`, `FUN_005617b0` | 00573610, 005617b0 | Espionage reveal roll and Seat of Power |
| `FUN_00546fa0`, `FUN_00544030`-`FUN_00544fc0`, `FUN_0054a860`-`FUN_0054adf0` | see `story-missions.md` | story literals (not traced here) |

Decompiles for the named functions are in `../assets/decompiles/FUN_xxxxxxxx.c`. The six recovered story handlers `FUN_0054a910`, `FUN_0054aa60`, `FUN_0054ab10`, `FUN_0054abc0`, `FUN_0054ac70`, `FUN_0054adf0` are new in this branch; `FUN_0054a860` and `FUN_00519130` were already there.

## Evidence

- Sweep outputs: 804 distinct-site rows for DatId-shaped instruction scalars outside bare masks; 1,200 family-byte sites. The ids found outside the interface (counts are sites): Luke 23, Han 16, Leia 12, Vader 21, Palpatine 13, Mon Mothma 5, Coruscant 16, Yavin 7, Chewbacca 5 (`0x38000343`), the five abodes 3-9 each, the HQ `0x20000005` 6, Bounty Hunters 6, the Stormtrooper regiment 2, the three research missions 3 each, `0x1400004a`, `0x14000082`, `0x25000004` 1 each. No other class id (capital ship, fighter, defense, manufacturing class) is named by rule code; only `0x14000045`/`46`/`85`, `0x1c00000x`, `0x2a000003` and the like appear *inside DAT seed tables* (CMUNAFTB, CMUNEFTB, CMUNCRTB, FACLHQTB, FACLCRTB), which are data. TEXTSTRA names were read from the resource table (`Corellian Corvette` `0x14000045`, `Medium Transport` `0x14000046`, `Imperial Star Destroyer` `0x14000085`, `Alliance Headquarters` `0x20000001`).
- Seed tables read from the DAT: CMUNYVTB (lead 0, two `0x10000002`, `0x1c000003`, `0x1c000004`), CMUNHQTB (four X-wings, two Y-wings, `0x10000002` x2, `0x10000001`), CMUNCRTB (six `0x10000006`, ten `0x1c000005`), CMUNAFTB (`0x14000045`; `0x14000046`, two `0x10000001`), CMUNEFTB (`0x14000085`, six `0x1c000005`, three `0x10000006`), FACLHQTB (`0x20000001`; `0x2c000001`, `0x2d000002`, `0x2a000003`, `0x24000003`), FACLCRTB (`0x2a000003`).
- MISSNSD: ids to families (DatId = family byte then id): Move `0x41000001`, Return `0x42000002`, Autorouting `0x43000003`, Adrift `0x44000004`, Diplomacy `0x51000010`, Rescue `0x61000011`, Sabotage `0x69000012`, Espionage `0x52000013`, Reconnaissance `0x54000015`, Recruitment `0x55000016`, Abduction `0x62000017`, the three research missions `0x53000020`-`22`, Incite `0x56000040`, Death Star Sabotage `0x6a000041`, Jedi Training `0x58000042`, Dagobah `0x71000043`, Palace `0x64000044`, Vacation/Sabbatical `0x72000045`/`46`, Subdue `0x57000080`, Assassination `0x63000081`, Pickup `0x73000082`, Bounty `0x65000083`. Of these, `FUN_00402720` (the AI's capability probe) names Diplomacy, Rescue, Sabotage, Espionage, Reconnaissance, Recruitment, Abduction, the research missions, Incite, Jedi Training, Subdue, Assassination and Death Star Sabotage as a list of mission ids to ask the AI about; a mission type absent from that list is never offered to the computer player (hyp: the list is exhaustive for MISSNSD; reading `FUN_00402720` in full would confirm).
- DEFFACSD: class `0x4` is "Death Star Shield" (family byte `0x25`), confirming `0x25000004`. CAPSHPSD: `0x4a` CC-7700 Frigate and `0x82` Interdictor Cruiser.
- Stage `0x15` read in full (`FUN_0055f7b0`, 255 lines), with `FUN_0055fe70`, `FUN_00562680`, `FUN_00562610`, `FUN_00562710`, `FUN_0055fc80`, `FUN_0055d7d0`, `FUN_0055ef30`, `FUN_00562520`. The side rule of `FUN_00562520` is `+0x40`/`+0x44`; research.md uses the same offsets for the Alliance and Empire buildability flags, so they are the class's per-side flags (hyp: the same fields; `FUN_0051cab0`'s `+0x40` read in `FUN_0051bc70` is then a different record: not resolved).

## Open questions

- Does `FUN_0055fe70` returning false for an absent named character (a mod that removes Chewbacca or Needa) abort the new game? `FUN_0055f7b0` ANDs every result and returns it as the stage result, and `FUN_005136d0` stops on a stage failure. Settle by running a data set with one named minor removed, or by reading `FUN_005136d0`'s failure branch (`LAB_00513826` returns to `switchD_0051370f_default`).
- `FUN_00402720`'s mission list: which mission ids the AI is allowed to run.
- The HQ instance id `0x20000005`: which table assigns instance ids (the second-to-last digit run is sequential for the singletons: `0xf2000003`, then `0x20000005`, `0xf8000006`...). Settle by reading the registry factory `FUN_0051ebf0`.
- What `FUN_004c94c0`, `FUN_004d76f0` and `FUN_004e5870` do differently for Coruscant (the flags `+0x30 & 1`, `+0x28 & 2`, `+0xdc`).

## Mod surface

What a mod adding an era scenario would have to change in REBEXE (the rest is DAT):

1. **Different leaders** (another Emperor, another Luke): patch the four ids in `FUN_005180e0` (0051812f, 0051815a, 00518188, 005181b6) and the eight lookups of `FUN_004024d0`/`FUN_004025f0`/`FUN_00402cf0`/`FUN_00507010`/`FUN_00507070`/`FUN_005070d0`/`FUN_00506f50`/`FUN_00506fb0`/`FUN_004f3e50`/`FUN_004f3eb0`/`FUN_004f3f10`; add the story ids if their chains are to survive.
2. **A different capital or rebel base**: patch `0x90000109` (16 sites) and `0x92000121` (7), notably the dealer (`FUN_00519d00`), the seeding steps (`FUN_0051acb0`, `FUN_0051ae20`, `FUN_0051af00`, `FUN_0051b080`), the victory check, the advisor tour and the AI cases above; or keep the ids and rebuild SYSTEMSD so that those ids hold the era's systems.
3. **New named characters at game start**: patch the placement list in `FUN_0055f7b0`; extra characters within families `0x30`/`0x38` need no code (class flags `+0x40`/`+0x44`).
4. **A different HQ**: keep ALLFACSD record `0x20000001`; a different instance id needs `0x20000005` patched in `FUN_00513a50`, `FUN_0051a680`, `FUN_004bf9e0`, `FUN_004c05e0`, `FUN_004c30c0`, `FUN_0050ad80`.
5. **A new troop or special-force type**: it works in the simulation as data, but appears in no TroopFinder or PersonnelFinder column (`FUN_0046d8d0`, `FUN_00465540`), and the AI record maps only the eight special forces to four capability bits (`FUN_00402e80`).
6. **Research**: classes, orders, costs and per-side flags are DAT. A fourth tree, or the three trees tied to other producer families, needs `FUN_0051cc30`, `FUN_0051cbd0`, `FUN_0051cc10` and the side slots `+0x90`-`+0xa4`.
7. **A third game type**: patch `FUN_005180e0`'s game-type switch and `FUN_004142d0`.

Proposed follow-up questions:

- **What happens when a named character is missing from MJCHARSD/MNCHARSD?** Start from `FUN_0055f7b0`, `FUN_0055fe70`, `FUN_005136d0` (stage failure) and `FUN_0053efa0`/`FUN_0053ef00` (lookup of a missing id). Settles whether a total-conversion can drop characters by data alone.
- **Which mission ids may the AI run?** `FUN_00402720` in full and its callers: whether a new MISSNSD mission can ever be offered to the computer player.
- **Registry ids for the object classes.** `FUN_0051ebf0` and `FUN_0058b420`: which family bytes the loader accepts, which are the limit for new classes (this and `or-q-mod-*` nodes for ships and facilities).
- **The CMUN registry binding.** `FUN_0055d520`-`FUN_0055d5e0`: whether a scenario can add a seed table or only replace the nine.
