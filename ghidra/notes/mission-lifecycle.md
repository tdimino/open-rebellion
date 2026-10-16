# Mission lifecycle: phases, timer, per-member outcome

Recovered 2026-09-28 from REBEXE.EXE (Homebrew Ghidra 12.1.3, read-only, no
analysis; full decompile plus vtable dumps). Finding F-019. The detection
phases (decoys, FOILTB, betrayal, exposure) and the phase-10 inputs are in
`decoy-roll.md`. Who fills the team and decoy lists is in
`ai-mission-planning.md`.

"First read" marks a function read once by a tracer and not re-read for this
note. Confirm those before porting the rule that rests on them.

## Phase stepping

A mission's phase `+0x68` runs `0..0xb`. `FUN_005227d0` advances it one step
when the ready bit `+0xa4` bit 0 is set (`FUN_005227d0.c`):

```
if end code +0x64 != 0: phase = 0xb                    // FUN_00520e40
elif ready:
    clear ready                                         // FUN_00522130(this, 0)
    next = phase + 1
    if phase == 10 and record.+0x58 (repeat): next = 8  // FUN_00520b60
    if phase == 0xb: next = 0xb
    set phase next                                      // FUN_00521980 -> slot +0x1f4
```

Setting the ready bit (`FUN_00522130(this, 1)`) calls slot `+0x220`. The
handler `FUN_00524b20` steps at once (`FUN_005227d0`), then runs
`FUN_005229c0`.

After each phase change `FUN_00546ea0` runs:
1. the detection manager (`FUN_00547f60`)
2. `FUN_00548120`
3. `FUN_00548370` on phase 2
4. `FUN_005484d0`
5. `FUN_00548840`
6. `FUN_00522980`

`FUN_00522980` sets the ready bit again unless the phase is 4
(`FUN_00520b20`) or 8 (`FUN_00520b30`). So a mission passes through every
phase in the same tick except two:

- **Phase 4, transit.** Entering it starts every member's transit to the target
  (`FUN_00556430` from `+0x6c` to `+0x78`, then into container `+0x74`) and
  calls `FUN_00522280`. That
  function, and `FUN_00545820` on later arrivals, sets the ready bit unless
  every member (`FUN_00525bb0`: team, decoys, and captured) still has its
  en-route active bit (`+0x50` bit 5, `FUN_00556620`), and only with no end
  code set. It keeps two flags, one for a member still travelling and one
  for a member off the road; both set means ready (`FUN_00522280.c:66-100`).
  With no member at all the mission is ready.
- **Phase 8, the mission timer.** Entering it sets the result code `+0x60` to 0
  (`FUN_00521880`) and arms timer `0x38b` (`FUN_0053fa60(0x38b, 1, ...)`).
  Leaving it runs the validator `FUN_00522480`, then disarms the timer
  (`FUN_0053fa60(0x38b, 0, ...)`; `FUN_00524b70.c:50-58`). The timer fires `FUN_00522a10`, which sets the
  ready bit only in phase 8.

The timer's delay record (`+0x54 -> +0x20`) is written at init by
`FUN_005236e0`: minimum = record `+0x50` (`FUN_00520b40`, `FUN_00540230`),
spread = record `+0x54` (`FUN_00520b50`, `FUN_00540270`). A timer fires after
`min + rand(0..=spread)` days (`FUN_00586130`, `timer-scheduler.md`).

### What each phase does (`FUN_00524b70`, entering the new phase)

| Phase | Action | Detection mode (`decoy-roll.md`) |
|-------|--------|---------|
| 2 | `FUN_00522870`: settle the team's location and routing | 2 |
| 3 | none | 1 |
| 4 | start transit to the target; wait for arrival | none |
| 5 | run the validator when no end code is set and the target differs from the current location (`FUN_00520bb0`) | none |
| 6 | land every member at the target (`FUN_004f7640`), validator | none |
| 7 | none | 1 |
| 8 | result code `+0x60` = 0, arm timer `0x38b`; wait | none |
| 9 | none | 4, betrayal |
| 10 | agent classes: the per-member roll (below) | none |
| 0xb | validator; agent classes: slot `+0x284` | none |

Every phase change also raises notification `0x30d` (`FUN_0053f950`).

Phase 2 also runs `FUN_00548370` (first read): it sets a travel-speed
percent of GNPRTB 3083 (shipped 50) when the team holds DatId `0x33000243`,
else 100. `FUN_005484d0` (first read) adds the team's average leadership
divided by GNPRTB 6144 to the target system's Incite or Subdue sum when an
Incite (`0x56`) or Subdue (`0x57`) mission enters or leaves phases 8..10.

## The mission record (MISSNSD)

`record` is the mission's class record (`+0x2c`). Its in-memory offsets sit
`0x28` above the file offsets (`tools/dat-dumper/src/types/missions.rs`):

| Memory | File field | Meaning | Source |
|--------|-----------|---------|--------|
| `+0x50` | `max_officers` (0x28) | timer minimum days | `FUN_005236e0` |
| `+0x54` | `base_duration` (0x2c) | timer spread days | `FUN_005236e0` |
| `+0x58` | `flag_col6` | repeat (phase 10 loops to 8) | `FUN_00520b60`, read by `FUN_005227d0` |
| `+0x5c` | `flag_col7` | hidden (OnHiddenMission) | `FUN_00520b70` |
| `+0x60` | `flag_col8` | detection phases on | `FUN_00520b80` |
| `+0x64` | `flag_col9` | members can resign | `FUN_00520b90` |

The dat-dumper's `max_officers`/`base_duration` names are wrong for this
layout; they are now `timer_min_days`/`timer_spread_days` (2026-09-28). The
shipped values read as (min, spread), e.g. `(60, 30)` for record `0x42`,
which matches Jedi Training. The MISSNSD loader was not traced. The same
`+0x28` shift holds for SPECFCSD: `FUN_00503b40` reads the class record's
`+0x98`, which is `mission_id` at file offset `0x70`, and `FUN_00535e40`
reads the skill pairs at `+0x58..+0x94`, file `0x30..0x6c`.

## Member skills

Characters and special forces share the person base (`FUN_005336b0`,
vtable `0x00660d60`), which zeroes eight skill shorts at `+0x58..+0x66`.
At init, `FUN_00535e40` sets each skill to the class record's base plus
`rand(0..=variance)` (`FUN_0053e290`), in the order diplomacy, espionage, ship
design, troop training, facility design, combat, leadership, and loyalty.
The setters are `FUN_00533e20`, `FUN_00533ea0`, `FUN_00533f20`,
`FUN_00533fa0`, `FUN_00534020`, `FUN_005340a0`, `FUN_00534120`, and
`FUN_005341a0`.

The special-force class is vtable `0x0065e160` (constructor `FUN_00503ae0`).
Its slot `+4` returns `0x3c`, and its slots `+0x1dc..+0x1f8` (`0x0065e33c`,
`FUN_00503c30`..`FUN_00503ca0`) return the base shorts directly. A character
returns its effective copies `+0x7c..+0x8a` instead (`decoy-roll.md`). The
shipped SPECFCSD variances are all 0.

`FUN_00520cd0`, the Incite and Subdue leadership term, averages slot
`+0x1f4` over every team, decoy, and captured member (`FUN_00525bb0`).

## Phase 10: the per-member roll and outcome

The agent classes override slot `+0x1f4` with `FUN_00592f50`. It runs the
base handler, then on phase 10:

1. slot `+0x278`;
2. for each team character (`FUN_00526090`), roll slot `+0x274`
   (`FUN_00593320`) and collect the result per member. Then apply slot
   `+0x27c(member, result, ctx, 10)` for each, in map order (`+0x1c`);
3. the same for each team special force (`FUN_00525e70`);
4. slot `+0x280` (the world effect), then slot `+0x284` (the end).

On phase `0xb` it calls `+0x284` only. Decoys and captives never roll.

Slot `+0x274` returns the member's success chance; `+0x27c` draws against
it (`FUN_0053e2f0(chance)`, `FUN_00574630.c:9-11`), sets the out flag, and on
a success records the result. DS Sabotage `00574630` then raises the member's
espionage (`+0x5a`) by GNPRTB 6167 (`0x1817`) and combat (`+0x62`; base skills
run diplomacy `+0x58`, espionage `+0x5a`, ship design `+0x5c`, troop training
`+0x5e`, facility design `+0x60`, combat `+0x62`, leadership `+0x64`, loyalty
`+0x66`)
by GNPRTB 6168 (`0x1818`) when slot `+0x1d8` holds (`FUN_0055bfd0.c:164-171`).

`+0x27c` records a per-mission result in `+0x60` (`FUN_00521880`, values
`0..3`; the classes set 3 on a success). `+0x280` reads `+0x60` and applies
the effect once. `+0x60` is distinct from the end code `+0x64`
(`FUN_00521900`).

### Slots per class

The table below comes from vtable dumps; each family code is read from its
class's slot `+4` (`MOV EAX, imm`).

| Family | Mission | Vtable | `+0x274` | `+0x278` | `+0x27c` | `+0x280` | `+0x284` |
|---|---|---|---|---|---|---|---|
| 0x51 | Diplomacy | `0x00667a18` | `573ff0` | `51ebb0` | `5740a0` | `574120` | `592c80` |
| 0x52 | Espionage | `0x00667390` | `573090` | `51ebb0` | `573540` | `573610` | `592c80` |
| 0x53 | Research | `0x00664db8` | `56cb20` | `56cd70` | `56cdb0` | `56cf30` | `592c80` |
| 0x54 | Reconnaissance | `0x00664b28` | `56bea0` | `51ebb0` | `56bec0` | `56bee0` | `592c80` |
| 0x55 | Recruitment | `0x00664890` | `56b7b0` | `56b940` | `56b9a0` | `576700` | `592c80` |
| 0x56 | Incite | `0x00666a70` | `5719d0` | `51ebb0` | `571a60` | `576700` | `592c80` |
| 0x57 | Subdue | `0x00663b78` | `569b90` | `51ebb0` | `569c20` | `576700` | `592c80` |
| 0x58 | Jedi Training | `0x006667d8` | `5710b0` | `51ebb0` | `5712b0` | `576700` | `592c80` |
| 0x61 | Rescue | `0x00664600` | `56ae30` | `51ebb0` | `56ae50` | `576700` | `592c80` |
| 0x62 | Abduction | `0x00668fd0` | `576e50` | `51ebb0` | `576eb0` | `576700` | `592c80` |
| 0x63 | Assassination | `0x00668ac8` | `5765c0` | `51ebb0` | `576620` | `576700` | `592c80` |
| 0x64 | Palace | `0x006652e8` | `56e620` | `51ebb0` | `56e650` | `56e870` | `592c80` |
| 0x65 | Bounty | `0x006685c0` | `575060` | `51ebb0` | `575de0` | `51ebb0` | `592c80` |
| 0x69 | Sabotage | `0x00663e08` | `56a2d0` | `51ebb0` | `56a300` | `5746e0` | `592c80` |
| 0x6a | DS Sabotage | `0x00667ca8` | `574600` | `51ebb0` | `574630` | `5746e0` | `592c80` |
| 0x71 | Dagobah | `0x00667f38` | `575060` | `51ebb0` | `572470` | `572490` | `592c80` |
| 0x72 | Vacation | `0x00666d00` | `575060` | `51ebb0` | `572470` | `572490` | `592c80` |
| 0x73 | Pickup | `0x00665058` | `575060` | `51ebb0` | `56da80` | `576700` | `592c80` |

`51ebb0` is the shared no-op default; `576700` is the shared `+0x280`.

### Outcomes (first read; confirm before porting each)

- Diplomacy (re-read 2026-09-28): `5740a0` draws against the member's
  chance; a success sets `+0x60 = 3` and, when the member's slot `+0x1d8`
  holds (always, for a character: vtable `0x0065ca70` slot `+0x1d8` is
  `FUN_0040f340`, return 1), raises its base diplomacy (`+0x58`) by GNPRTB 6156 (`0x180c`,
  `FUN_00533e20`). `574120` runs once after the members: `+0x60 == 0`
  becomes 2 (failed). On 3 it finds the target system (`FUN_00586720`)
  and adds `FUN_0055cac0(mission side, system side)` support through
  `FUN_0050c9f0`. The gain is GNPRTB 6183 + rand(0..=6184) when the
  system is the mission side's, 6185 + rand(0..=6186) when its side is 3 (neutral, `FUN_004f8c60`),
  and 0 otherwise (`FUN_0055cac0.c`; ids from `FUN_0055bfd0.c:269-290`).
  One success among the members is enough; more successes add nothing
  beyond their own skill raises.
- Espionage (re-read): `573540` sets 3 on a success and, when the target
  (`FUN_00521070`) belongs to another side, raises the member's espionage by
  GNPRTB 6157 (`0x180d`). `573610`: 0 becomes 2; on 3 and a system target
  (`0x90..0x98`), observation level 10 for the mission side
  (`FUN_0050d5a0`), then the revelation counts (`FUN_0055c940`, first read)
  applied by `FUN_00573170` (not read).
- Recruitment (re-read 2026-09-29): on a success (`FUN_0053e2f0`, the draw
  below the chance) `56b9a0` finds the target system (`FUN_00586720`) and
  picks a recruit for the mission side (`+0x24 >> 6 & 3`) with
  `FUN_0055fc80`. When one was picked it records the recruit on the mission
  (`FUN_0056b1a0` writes `+0xa8` and notifies), sets 3, and raises the
  member's leadership (`+0x64`) by GNPRTB 6159 (`0x180f`, `DAT_006bb5b0`)
  through slot `+0x1d8`. An empty pool leaves the result alone. Each
  successful member recruits one character.
  - `FUN_0055fc80`: the pool is the global character container
    (`FUN_00506e20`, `DAT_006b2bb0 + 0xb0`) filtered by side and the family
    range `0x38..0x3c` (`FUN_0056f450` -> `FUN_00513090`), which is MNCHARSD:
    minor characters only, never the majors of `0x30..0x38`. It counts those
    with `+0x50` bit 1 clear (`FUN_0055ef30`), draws index
    `FUN_0053e290(count - 1)` (0..=count-1), and recruits it with
    `FUN_0055fe70`. When the count was 1 it sets the side's `+0xb8`
    (`FUN_0052f590(side, 1)`), for sides 1 and 2 only.
  - `FUN_0055fe70`: a character with bit 1 clear takes slot `+0xa8(system)`
    (placed at the target), then `FUN_004f7480` and `FUN_004f74f0` set
    `+0x50` bits 1 and 2 (`FUN_0053a640(2 | 4, 1)`) and notify.
- Rescue (re-read): `56ae50` on a success sets 3, raises combat (`+0x62`)
  by GNPRTB 6162 (`0x1812`), and calls the target character's slot `+0x210`
  with the member's key (the release). The target is `FUN_00586c80`.
- Abduction (re-read): `576eb0` on a success sets 3, raises combat by 6163
  (`0x1813`), and calls the target's slot `+0x20c` with the member's key:
  the capture of `decoy-roll.md` "Exposure", captor = the member.
- Assassination (re-read): `576620` on a success calls the target's slot
  `+0x2ec` (kill). Only when the target is then destroyed (`+0x50` bit 3)
  does it set 3 and raise combat by 6164 (`0x1814`).
- Incite (re-read): `571a60` needs the target system (`FUN_00586720`). On a
  success it starts the uprising (`FUN_0050d030`, `uprising-incident.md`).
  The result is 2 when the system's `+0x84` bits 2..3 already equal the
  mission side's opponent code, or `FUN_00509020(system, holder, 1)` holds;
  else 3. On 3 the member's leadership rises by 6160 (`0x1810`).

The target actions of Recruitment, Rescue, Abduction, Assassination, and
Incite run inside `+0x27c`, once per successful member, in member order;
their shared `+0x280` (`576700`) only turns result 0 into 2 (failed).
- Subdue `569c20`: support gain `FUN_0055cb10` (6187 + rand(6188), neutral
  6189 + rand(6190)), then the end-revolt check `FUN_0050c910`.
- Sabotage (re-read): `56a300` sets 3 on a success and raises espionage by
  GNPRTB 6165 (`0x1815`) and combat by 6166 (`0x1816`). DS Sabotage
  `574630` does the same with 6167/6168. Both share `5746e0`: 0 becomes 2;
  on 3 the mission's target object (`FUN_00521030`, not the system) gets
  slot `+0xac(6, ctx)`, the destroy call with cause 6. So a Sabotage mission
  names one target object at creation (order `+0x4c`).
- Palace `56e650`: rescues the target, then frees every prisoner there.
- Reconnaissance `56bec0`: always succeeds, end code 3.
- The missions also raise skills on success; the amounts are not read.

### Ported (F-019 phase 4a, 2026-09-29)

`MissionSystem::roll_members` in `crates/rebellion-core/src/missions.rs`
ports the roll and the in-roll outcomes above, re-read from the `.c` files.

- **Chance (`+0x274`).** The wrapper `FUN_0053e240` finds the row through
  `FUN_0055bed0` -> `FUN_0058b7e0` -> `FUN_00595090`, the step lookup. A
  missing row leaves 0, as does a missing target system (`FUN_00586720`,
  family `0x90..0x97`) or target character (`FUN_00586c80`, `0x30..0x3b`).
- **Support and stormtroopers.** The "opposing support" in the Diplomacy,
  Recruitment, Incite and Subdue inputs is `FUN_00507270(system, 2 -
  (side != 1))`, the same opponent formula as the detection setup. Incite
  and Subdue read the member's own leadership (`+0x1f4`), not the mission
  average.
- **Skill raises.** They go to the base skill only through slot `+0x1d8`,
  which a special force lacks (vtable `0x0065e160` slot 118 is
  `FUN_006158b0`, returning 0). The shipped GNPRTB 6156..6168 are all 1, and
  Subdue's is 6161 (`0x1811`, `DAT_006bb538`).
- **Result `+0x60`.** Each class sets it through `FUN_00521880`, so a later
  member's 2 overwrites an earlier 3 (Incite, Subdue).
- **Incite `FUN_00571a60`.** The incident is `FUN_0050d030`. The result is 2
  when the holder keeps a regiment (`FUN_00509020(system, holder, 1)`: kind
  1, as in `FUN_005091f0`). port: the other test, the system's `+0x84` bits
  2..3 against the opponent, reads a field that no recovered function
  writes, so the port takes it as false.
- **Subdue `FUN_00569c20`.** It rolls only while `+0x88` bit 2 (revolting)
  holds. Each success adds `FUN_0055cb10` support and runs `FUN_0050c910`;
  once that ends the uprising, a later member does not roll.
- **Diplomacy support `FUN_0055cac0`.** It is `G6183 + rand(0..=G6184)` only
  when the system's side equals the mission's, and `G6185 + rand(0..=G6186)`
  when it is neutral (side bits 3: `FUN_004f8c60` names 3 "Neutral"). At an
  opposing system it is 0. The Subdue port read side 3 as the port's
  `ControlKind::Contested`; it is the neutral system.
- **Port choices:**
  - The draws come from the mission's seeded stream (`MissionRng`,
    `ROLLS_PER_MISSION` 3: two timers and the seed).
  - A target action runs once, however many members succeed: Rescue's
    release, Abduction's capture and Assassination's kill.
  - The kill always destroys the target.
  - A member's incident reads the world before earlier members' effects
    apply.
- **Still interim.** Espionage's revelation counts (`FUN_0055c940`).

### Ported (F-019 phase 4b)

- `Character::recruited` is `+0x50` bit 1, and
  `GameWorld::recruit_pool_empty` is each side's `+0xb8`.
- port: a character placed at game start begins recruited.
- The pool is walked in `DatId` order.
- Bit 2 is not modelled.
- A later mission in the same step skips an earlier mission's pick.

### Ported (F-019 phase 4c)

- The creation checks read the target object's family (slot `+4`).
  - `FUN_0056a110` (Sabotage) refuses families `0x18..0x1c` with
    `0x40`/`0x28`, and `0x30..0x3c` (characters) with `0x40`/`0x25`.
  - `FUN_005744c0` (DS Sabotage) refuses anything outside `0x18..0x1c` with
    `0x40`/`0x29`.
- The DAT family ranges `[field3, field4)`:

  | Family range | DAT |
  |---|---|
  | `0x08..0x10` | fleets |
  | `0x10..0x14` | troops |
  | `0x14..0x1c` | capital ships |
  | `0x1c..0x20` | fighters |
  | `0x22..0x28` | defense facilities |
  | `0x28..0x2c` | manufacturing facilities |
  | `0x2c..0x30` | production facilities |
  | `0x30..0x38` | major characters |
  | `0x38..0x3c` | minor characters |
  | `0x3c..0x40` | special forces |

  CAPSHPSD's only family `0x18` record is `0x88`, the Death Star.
- `MissionTarget` names a facility, a regiment, a special force, or a
  fleet's Death Star hull. `destroy_target` is slot `+0xac(6)`.
- port: other capital ships and fighters have no identity in the port. A
  removed object reads as the opponent's and destroyed for rule 4.

### The end (`FUN_00592c80`, re-read 2026-09-29)

Slot `+0x284` returns at once when `FUN_00520af0` is false: the mission's
last phase (`+0x54 -> +0x1c`, `FUN_00520ac0`, written by `FUN_00524b70` for
every phase but `0xb`) is below 5 and its target `+0x78` differs from its
origin `+0x6c` (`FUN_00520bb0`), so its members never landed. Otherwise, when the target location `+0x78`
is a system, it raises observation level 6 there for the mission's own side
(`FUN_0050d5a0(system, 6, mission +0x24 >> 6 & 3)`). It then reroutes the
target's container `+0x70`/`+0x74` (`FUN_00521160`, `FUN_00521030`) when that
container is neither a system (`0x90..0x98`) nor of family `0x98..0xa0`,
through the container's slot `+0x24` or `FUN_004f7f20`. It never moves the
members: they stay where phase 4 put them. A repeating mission never reaches
this slot until an end code is set.

## Members travel and stay (re-read 2026-09-29)

Phase 4 (`FUN_00524b70.c:79-128`) runs the validator, then for every member
of the team, decoy, and captured lists (`FUN_00525bb0`, families
`0x30..0x40`, all three lists):

1. `FUN_00556430(member, +0x6c, +0x78)` starts its transit from the origin
   location to the target location (`build-delivery.md`, "Travel"): no
   transit when both are the same system;
2. `FUN_00556390(member, +0x74)` moves it into the target container `+0x74`
   through the member's slot `+0xa8`.

`FUN_00522280` then sets the ready bit unless every member is still en
route (bit 5). The original deletes a destroyed member, so a team destroyed
on the way counts as arrived. With no origin (`FUN_00504dc0` null),
`FUN_00556430` starts no transit (`:31-33`) and the member stays put. Phase 6 clears
every member's en route bit (`FUN_004f7640(member, 0)`) and runs the
validator; phases 5 and 6 act only when the target differs from the origin
(`FUN_00520bb0`). After the end the members are still in the target
container.

On phase 2, `FUN_00548370` sets every character member's travel speed
`+0x9a` (`FUN_004ee470`; characters in all three lists, `FUN_00525fe0`): GNPRTB
3083 (50, `FUN_005725a0` -> `DAT_006bb748`) when Han Solo (`0x33000243`,
`FUN_00506f50`) holds this mission's key at `+0x68`, is not a prisoner
(`+0xac` bit 0), and no special force is a member (`FUN_00525dc0`,
`FUN_00525a00`; `FUN_00542990`). Otherwise the GNPRTB 1 default (100). A
special force travels at the default (`build-delivery.md`).

When an object is destroyed, `FUN_00545240` walks every mission. A mission
whose container `+0x74` (`FUN_00520c70`) or target (`FUN_00520cb0`) is that
object runs the validator at once. Then, if the container was destroyed, no
end code is set, and the mission's phase is above 6 (`FUN_00520ae0` over
`+0x54 -> +0x1c`), past the landing, it gets end code 7 through slot
`+0x1dc` (`FUN_00545240.c:85-111`). A container lost while the members
travel ends nothing unless the validator's column 11 asks for it. The check
runs once, on the destruction. A destroyed target therefore ends through the
validator's rule 4 (code 6) when the record's column 14 asks for it.

## The validator and the end codes (`FUN_00522480`, re-read 2026-09-29)

The validator runs on leaving phase 8, on entering 4, 5 (conditionally), 6,
and `0xb`. With no end code set yet:

1. End code 5 (message `0x40`/`0x91`) when every team member has a remove or
   resign request (`+0x78 & 0xc`; team only, `FUN_00525c60`), unless `+0xa4`
   bit 1 is set.
2. `FUN_005830a0` summarises the three lists (`FUN_00582fb0`): the OR of the
   special forces' mission masks, the OR of the characters' masks, and which
   sides the members belong to. It sets status `0x14` when both sides appear
   and `0x16` when none does. A status skips every later check; it is not an
   end code.
3. Slot `+0x1bc` (`MOV [EAX], 0x592aa0; MOV [EAX+4], validator`) yields the
   class's validator, called over a checker built by `FUN_00582b90` with
   `+4` = 0 (the creation-only checks are off), `+0xc` = 1 (running),
   `+0x10` side, `+0x14` the MISSNSD record, `+0x24..+0x2c` the lists,
   `+0x34` the container `+0x74`, and `+0x38` the target `+0x70`
   (`FUN_00521050`, `FUN_00521030`) once `FUN_00520af0` holds, from phase 5
   or with the target at the origin. Before that the checker reads the
   side's own copy of the target and container (`FUN_00521160`,
   `FUN_005211c0` through `FUN_004f2d10`; `FUN_00522480.c:62-80`). The port
   has no per-side copies and reads the real objects.
4. In phase `0xb` with no code, the end code is 1.

The first rule that fires sets the status and wins: later rules check
`status +4 == -1`. A non-zero code goes to slot `+0x1dc`, then
`FUN_00521900`, and the next step jumps to `0xb`.

### Validators per class

| Family | Mission | Validator | Rules |
|---|---|---|---|
| 0x51 | Diplomacy | `573ee0` | system rules, then end `0xf` (message `0x40`/`0x33`) when the side's support at the target is 100 (`FUN_00507270 == DAT_00661a88`) |
| 0x52 | Espionage | `573010` | system rules |
| 0x55 | Recruitment | `56b370` | system rules, then end `0x10` (`0x40`/`0x35`) when the side object's `+0xb8` is set |
| 0x56 | Incite | `571950` | system rules |
| 0x57 | Subdue | `569b10` | system rules |
| 0x61 | Rescue | `56adb0` | character rules |
| 0x62 | Abduction | `576dd0` | character rules |
| 0x63 | Assassination | `576540` | character rules |
| 0x69 | Sabotage | `56a110` | object rules (its own checks are creation-only) |
| 0x6a | DS Sabotage | `5744c0` | object rules (creation-only checks) |

Side `+0xb8` is set by `FUN_0052f590(side, 1)` from `FUN_0055fc80`, the
recruit pick: it counts the side's recruitable characters (`FUN_0055ef30`),
picks index `rand(0..=count-1)` among those not yet recruited (`+0x50` bit 1
clear), recruits it (`FUN_0055fe70`), and sets `+0xb8` when that was the
last one. So Recruitment ends once the pool is empty.

### The running checks

`R` is the MISSNSD record (in-memory offsets; file word = `(offset -
0x28) / 4`, column = `(offset - 0x40) / 4`,
`tools/dat-dumper/src/types/missions.rs`), `T` the target, `C` the
container. Every family runs the base rules; the system, character, and
object rules add their own.

Base (`FUN_00523450`, then `FUN_00592600`):

1. `T` or `C` missing: the remaining checks are skipped.
2. `R +0x6c` (col 11) and `C` destroyed (`+0x50` bit 3): end 7 (`0x40`/`0x12`).
3. `T`'s side (`+0x24 >> 6 & 3`) against the mission's: the same side reads
   `R +0x7c` (col 15), the opponent (`1 <-> 2`) `R +0x84` (col 17), any other
   `R +0x80` (col 16). A zero column ends 8 (`0x40`/`0x23`).
4. `R +0x78` (col 14) and `T` destroyed: end 6 (`0x40`/`0x21`).
5. `T` en route (`+0x50` bit 4): end 6 (`0x40`/`0x22`).
6. `T`'s location key (slot `+0xc`) empty or different from `C`'s: end 6
   (`0x40`/`0x20`). Inference: the target left its container.
7. `R +0x74` (col 13) and `C` not a populated system (`+0x88` bit 0): end
   `0xd` (`0x40`/`0x16`).

System rules (`FUN_005868c0`): `T`'s uprising bit (`+0x88` bit 2,
`uprising-incident.md`) set needs `R +0x88` (col 18), clear needs `R +0x8c`
(col 19); otherwise end 6 (`0x40`/`0x20`).

Character rules (`FUN_00586e20`): when `T` is a character (`0x30..0x3c`), a
prisoner target (`+0xac` bit 0) needs `R +0x90` (col 20) and a free one
`R +0x94` (col 21); otherwise end 6 (`0x40`/`0x2a`, `0x2b`).

Object rules (`FUN_00593500`): the base rules only, while running.

The shipped columns (`data/base/MISSNSD.DAT`, column `n` = file word `6 + n`):

| Family | c11 | c12 | c13 | c14 | c15 own | c16 other | c17 opp | c18 | c19 | c20 | c21 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x51 Diplomacy | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 |
| 0x52 Espionage | 0 | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 1 | 0 | 0 |
| 0x53 Research | 1 | 1 | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 0 | 0 |
| 0x54 Recon | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| 0x55 Recruitment | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 0 | 0 |
| 0x56 Incite | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| 0x57 Subdue | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 |
| 0x61 Rescue | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 1 | 0 |
| 0x62 Abduction | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 0x63 Assassination | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 0x69 Sabotage | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 0 |
| 0x6a DS Sabotage | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |

The three Research records (`0x53000020`..`22`) share their columns. Recon
(`0x54000015`) has no port kind.

Column 12 (`R +0x70`) is a creation-only rule (`FUN_00592600.c:73-102`):
when `C` is a system (`0x90..0x97`), it needs `+0x88` bit 1, else
`0x40`/`0x11`; any other container passes. What sets `+0x88` bit 1 is not
traced.

So Diplomacy stops on an enemy-held or revolting system, Subdue once the
revolt is over, and Rescue once its target is free.

Inference: Rescue's own-side column is 0, so a prisoner must read as its
captor's side for a Rescue to run; the side a system reads as is taken to be
its holder. Neither is traced to a writer.

## The creation checks (`FUN_00542b60`)

Command `0x250` creates a mission only when `FUN_00542b60` passes with no
status (`FUN_0054d360.c:28-41`); its validity slot `FUN_0054d280` makes the
same call. A refused order creates no mission object. The checks run in
order, and each later one tests `status +4 == -1`:

1. The order's family: `0x240` needs mission family `0x50..0x7f`,
   `0x241` `0x40..0x4f` (`FUN_00542b60.c:36-70`; `1`/`1`).
2. `FUN_0054bb90`: prisoners move to the captured list; an empty team is
   `0x40`/`0x91` (`:79-87`).
3. `FUN_0054bf00`, `FUN_0054c200`: the per-member adds (`:90-106`).
4. `FUN_005830a0`: members of both sides `1`/`0x14`, of none `1`/`0x16`
   (`:110-118`).
5. The side's own view of the target (`FUN_004f2d10`) is missing:
   `0x40`/`0x21` (`:129-137`).
6. The record (`FUN_0051cab0`) and `FUN_0054c590` (`:139-150`): the class's
   resolver yields the container, and the checker `FUN_00582b90(1, 1, 1, ..)`
   runs the class validator with the creation checks (`+4`) and the running
   checks (`+0xc`) both on (`FUN_0054c590.c:33-48`). A resolver failure
   refuses with no status.

So every running check of the class also runs at creation. The
creation-only checks are:

| Check | Where | Status |
|---|---|---|
| Member masks, one side, the record's side flag | `FUN_00523450.c:30-33` -> `FUN_00583320.c:21-76` | `0x40`/`1` |
| The container in flight (`+0x50` bit 4) | `FUN_00523450.c:38-47` | `0x40`/`0x13` |
| The container's family not `0x90..0x97` or `0x98..0x9f` | `FUN_00592600.c:35-72` | `0x40`/`0x10` |
| Column 12 and a system container without `+0x88` bit 1 | `FUN_00592600.c:73-102` | `0x40`/`0x11` |
| A system kind's target not a system (`0x90..0x97`) | `FUN_005868c0.c:25-48` | `0x40`/`0x24` |
| A character kind's target not a character (`0x30..0x3b`) | `FUN_00586e20.c:25-48` | `0x40`/`0x26` |
| An object kind's target not an object (`0x10..0x3f`) | `FUN_00593500.c:25-48` | `0x40`/`0x27` |
| The object not completed (`+0x50` bit 2) | `FUN_00593500.c:49-58` | `0x40`/`0x34` |
| Sabotage of a Death Star (`0x18..0x1b`), or of a character | `FUN_0056a110.c:25-69` | `0x40`/`0x28`, `0x40`/`0x25` |
| DS Sabotage of anything but a Death Star | `FUN_005744c0.c:25-47` | `0x40`/`0x29` |
| Recruitment: a member outside `0x30..0x37` (`FUN_0056b550.c:29-52`) | `FUN_0056b370.c:24-52` | `0x40`/`6` |
| Research: a member with skill 0 for the tree | `FUN_0056c510` -> `FUN_0056c820` | `0x40`/`4` |

At creation the running checks set the same statuses as while running
(the table in "The running checks"), without an end code.

## Still open

- Which object a character mission stores as its target `+0x70` and
  container `+0x74` (the per-class slot `+0x1bc` init is not a defined
  function in the database); Sabotage's is its order's object (`+0x4c`).
- What slot `+0xac(6)` does beyond destroying the object (messages,
  cargo aboard a destroyed hull).
- What system slot `+0xc` returns, for the location rule above.
- `FUN_0055c940`/`FUN_00573170` (what Espionage reveals).
- What `+0x50` bit 2 of a recruit means, the `+0xc` test in
  `FUN_0056b370`, and its `FUN_0056b680`/`FUN_0056b550` rules.
- `FUN_00548120` and `FUN_00548840` (run on every phase change).
- What end codes 1, 3, 4, 5, and 7 are shown as.

## Supporting decompiles

`FUN_005227d0`, `FUN_00524b70`, `FUN_00524b20`, `FUN_00522980`,
`FUN_00522130`, `FUN_00522280`, `FUN_00522a10`, `FUN_00522a90`,
`FUN_00520b20`, `FUN_00520b30`, `FUN_00520b40`, `FUN_00520b50`,
`FUN_00521880`, `FUN_005236e0`, `FUN_00540230`, `FUN_00540270`,
`FUN_00546ea0`, `FUN_00592f50`, `FUN_00592c80`, `FUN_00548370`,
`FUN_005484d0`, and the per-class `+0x27c`/`+0x280` files listed above.
