# Mission return path (`FUN_00548120`)

Tags: **code read**, **inference**, **needs VM**. Citations are `file:line` in this folder.

## 1. `FUN_00521220(mission, member, &need, &route, &check)`

- Zeroes all three (`FUN_00521220.c:12-14`). `FUN_00520e40` is `mission +0x64 != 0` (end code, `FUN_00520e40.c:6`).
  With no end code and the member's `+0x78 & 0xc` clear (bit 2 remove request, bit 3 resign request), it returns with all three 0 (`:15-18`). code read
- Otherwise `need = 1` (`:25`).
- Side test: `FUN_004f6b50` reads the member's `+0x1c`, the object that holds it now (system, or the fleet or ship it is aboard), not the mission's container (`FUN_004f6b50.c:7`). `same = ((member +0x24 ^ container +0x24) & 0xc0) == 0` (`FUN_00521220.c:23`). A member with no container counts as different. code read
- `route = 1` when member `+0x50` bit 11 is set, or policy == 2, or (policy == 1 and not `same`); else 0 (`:26-33`). `check = route && policy != 2` (`:35-40`). Policy is record `+0x68` (`FUN_00520ba0.c:5-8`). code read
- Phases: it acts on every call of `FUN_00546ea0` (`FUN_00546ea0.c:24`), i.e. after every phase change. `need` holds mid-mission only for a member with a remove or resign request (for example an evader, `decoy-roll.md` "Exposure"). code read
- A member with bit 2 (remove request) is skipped by `FUN_00548120` (`FUN_00548120.c:71-72`) and by `FUN_00548910` (`FUN_00548910.c:73`). No other handler found. needs trace

## 2. A member with `route == 0`

`FUN_00548120.c:62-100`: slot `+0x1d0` = `FUN_00522fc0(member, 0, ctx)`. In `FUN_00522fc0`:
- flag 0 with en-route bit 5 set and autoroute bit 11 clear: `FUN_004f7640(member, 0)` clears the en-route bit (`FUN_00522fc0.c:61-65`). code read
- `FUN_00534560(member, flag)` writes role bit 1 "MovingBetweenMissions" (`:66`; `decoy-roll.md:131`). code read
- `FUN_00534230` and `FUN_005342e0` (null key) clear member `+0x68`, `+0x6c` (`:78-91`). Bit 7 "on a mission" is then recomputed. code read
- Removal from team, decoy or captured list (`:100-138`), then validator `FUN_00522480` (`:142`), `FUN_00522870` when the member is `+0x7c` (`:149`), `FUN_005229c0`. code read
- Nothing moves the member: it stays in its current container, the mission's `+0x74` container after phase 4. code read
- If the mission has ended and the member was team or decoy, `FUN_00522070` appends it to mission list `+0x9c` (`FUN_00548120.c:101-113`, `FUN_00522070.c:23-35`). Readers of `+0x9c` not traced. needs trace

## 3. `FUN_00542ee0` and `+0x70`

- Keys outside `0x50..0x7f` go to `FUN_00543040` (`FUN_00542ee0.c:27-36,63-66`). code read
- With `check` set: `FUN_00554170(member, &st)` (`:44`). It builds a route context for the member's own side (`FUN_005511d0.c:22`) with order `0x241` for families `0x30..0x3f` (`(-(1) & 0x3d) + 0x204`), takes the member's container id (`FUN_00554170.c:38`), resolves it (`FUN_00551270`) and builds a leg (`FUN_00551c80`). `st.+4 == -1` means accepted; it returns 0 only on a structural failure. code read
- In plain terms it accepts the member's current container when it is a legal Move destination for the member's side: of the member's side (else `1/0x24`), not destroyed (`0x22`), completed (`0x20`), not en route (`0x21`) (`FUN_00553410.c:22-62`; `move-order.md` "Route refusals"), room unlimited for characters, blockade refusal lifted for `0x241`. Refusal-code detail for systems is inference from those notes. inference
- Accepted: destination = that container id (`FUN_00542ee0.c:46-48`). A refusal status goes to `FUN_005543c0(member, &member+0x70, &st)` (`:50-56`): the same test on the stored key, accepted → destination = member `+0x70`. If `FUN_00554170` returned 0, `+0x70` is not tried (`:45`). code read
- Writer of `+0x70`: `FUN_00536490` is entry `0x0065e3c8` / `0x00660fc8` = slot `+0x268`, the OnMission (bit 7) change notifier (`FUN_00534800.c` calls `+0x268`). Bit set: `+0x70` = the member's container id at that moment (`FUN_00536490.c:28-34`), that is, at joining the mission, before phase 4 moves it. Bit cleared: `+0x70` = null (`:20-27`). code read
- `FUN_00534390` (`+0x70` setter) calls slot `+0x244` = `FUN_00536300`, which stores that container's location in `+0x74` (`FUN_00536300.c:23-31`). So `+0x74` is the system of the `+0x70` container. `FUN_005431e0` and `FUN_00543360` read `+0x74`, not a mission container (corrects the `jail.md` wording). code read
- Other writers: Pickup (`FUN_0056da80.c:77`), a story hook (`FUN_00546370.c:372`). code read
- Retreat chain `FUN_00543040`: `FUN_00543360` offers `+0x70` (when `+0x74` is a system) to `FUN_00554ab0` first (`FUN_00554ab0.c:115,150,171`). Not fully read. inference
- `+0x70` is the key of the object that held the member when it joined (`FUN_00536490.c:28-34`): a system, a fleet or a capital ship. Only `+0x74`, its system, is a system. Under policy 1 the return therefore tests the container itself: `FUN_005543c0` builds the route context for the member's own side (`FUN_005511d0`, kind `0x241`) and `FUN_00554490` places the member in the stored container (`FUN_005543c0.c:28-31`). An accepted fleet container puts the member aboard its first accepting ship (room for characters is unlimited, `regiment-unload.md`, "Choosing the container"); an accepted ship holds it; an accepted system holds it. The destination `FUN_00542ee0` hands on is that container id, not its system. Chain stage 2 offers the same key, then its parent and grandparent (`FUN_00554ab0`). code read for the call path, inference for the placement result
- **Order against the outcome.** The phase-10 handler (`FUN_00592f50`) rolls the members, applies the world effect (slot `+0x280`) and ends the mission (slot `+0x284`) before `FUN_00546ea0` runs `FUN_00548120` for that phase change (`FUN_00546ea0.c:17-24`). A Diplomacy effect writes the system's support at once (`FUN_0050c9f0` → `FUN_00509c30`), and the support write's hook `FUN_005109f0` runs `FUN_0050b610` in the same call, which rewrites the control word and the holder bits (`support-shifts.md`, "The support hook"); only event `0x31b` is queued. So the return reads the system's side after the outcome: a Diplomacy that turned its system finds `same` true (`FUN_00521220.c:23`) and its envoy stays. code read

## 4. Hidden Return `0x42000002` (`FUN_005475f0` → `FUN_00547ba0`)

- `FUN_0054bf00` finds the group's common location; unequal locations give status `0x40/3` and **no mission is created** (`FUN_005475f0.c:36-40`), with the members already removed from the old mission. code read
- Setters in `FUN_00547ba0.c:145-171`: phase 1; `+0x6c` origin = common location; `+0x70` target = destination id; `+0x74` container = destination id; `+0x78` target location = the destination's location (slot `+0xc`, system or abode required, `:84-140`); `+0xa4` bit 1 = 0, bit 2 = the old mission's `+0xa4` bit 2. Side must be 1 or 2 (`:46`). code read
- Record (MISSNSD id 2): timer 0/0, hidden 1, detection 1, resign 0, policy 0, column 11 = 1 (data dump). code read
- Phases are the base `FUN_00524b70` (`decoy-roll.md:510`): 2 speed (`FUN_00548370`: 100, GNPRTB 3083 = 50 with Han Solo and no special force), 4 transit `FUN_00556430(member, +0x6c, +0x78)` and reparent into `+0x74` at once (`mission-lifecycle.md` "Members travel and stay"), 6 land, `0xb` end code 1. Ticks per `FUN_00555b30`: 0 within one system, 1 when an end is not a system, else `max(1, isqrt(dx²+dy²)/5 * speed/100)`. A same-system Return completes on the day it starts. code read / inference for the same-day claim (as Move, verified in the VM for Move only)
- Arrival: the member is already a child of the destination container; the en-route bit clears at phase 6. Role bit 1 stays set while `+0x68` is empty, which keeps bit 7 and `+0x70` (`mission-lifecycle.md` bit 7 recompute). Where bit 1 clears after the Return is not traced. needs trace
- Visibility: record hidden → OnHiddenMission (bit 8). The Missions window skips it; the Defenses window lists it; a travelling member shows "Enroute" (`personnel-finder.md:251-263`). No Return-specific message found. inference; needs VM for messages and detection.

## 5. Policies and the four cases

Column 10 (file offset `0x40` in the record, dumped from `data/base/MISSNSD.DAT`), TEXTSTRA name id in brackets. code read
- 0: Move (11265), Return (11266), Autorouting (11267), Adrift (11268).
- 1: Diplomacy (11280), Rescue (11281), Sabotage (11282), Espionage (11283), Reconnaissance (11285), Recruitment (11286), Abduction (11287), Research x3 (11296-11298), Incite Uprising (11328), Death Star Sabotage (11329), Jedi Training (11330), Subdue Uprising (11392), Assassination (11393).
- 2: Dagobah (11331), Palace (11332), Vacation (11333), Sabbatical (11334), Pickup (11394), Bounty (11395).

At the end (`+0x64 != 0`), policy 1, the member's side against the system it stands in:
- Diplomacy at a neutral system: sides differ, so it goes home: the `+0x70` container if it is still a legal destination, else the retreat chain (nearest own-side system). It stays only if the system already has the member's side bits. inference; needs VM (does the system flip to the mission's side before phase `0xb`?).
- Recruitment at one's own system: same side, `route = 0`, removed in place, stays. code read
- Espionage at an enemy system: differs, Return to `+0x70` (or the chain). inference
- Sabotage at an enemy system: same as Espionage. inference

## 6. Side-specific items

Only side data, no Alliance/Empire branch: member and container `+0x24` bits 6-7 (`FUN_00521220.c:23`); the side passed to `FUN_00547ba0` (1 or 2); Yavin `0x92000121` for side 1 and Coruscant `0x90000109` otherwise (`FUN_005431e0`); the jail by `+0x24 & 0xc0 == 0x40` (`jail.md` section 4). Han Solo's speed rule is by key, not side. code read

## Needs a VM run

Diplomacy at a neutral system through phase `0xb` (does ownership flip before the sweep, and the Return target); Espionage/Sabotage return target with an intact origin, a destroyed origin, and an en-route origin; whether a Return raises any message or detection roll.
