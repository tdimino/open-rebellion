# Decoy roll (TDECOYTB / FDECOYTB)

Recovered 2026-09-26 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Finding F-019 in the full-functionality audit.

## Roll

`FUN_0055e410(side, a, c, b, in_fleet, &ok, &hit)`:

```c
x = (a - b) - FUN_0053e190(c, DAT_006bb710);   // c * G / 100
FUN_0053e340((in_fleet != 0) + 10, x, &ok, &hit);
```

- `FUN_0053e190(v, p)` is `FUN_0053e170(v, p, DAT_00661a88)`, which is
  `v * p / DAT_00661a88` (`FUN_0053e150` is plain integer division). The
  divisor is the percent base 100.
- `DAT_006bb710` is GNPRTB 0xe04 (3588), loaded by `FUN_0055e340` through
  `FUN_0053e390(0xe04, &DAT_006bb710)`; shipped value 35.
- Table id `10` is TDECOYTB and `11` is FDECOYTB (resource ids from
  `FUN_0058b420`, see `uprising-incident.md`). FDECOYTB is used when the
  checked object sits in a fleet.
- `FUN_0053e340` looks the table up (`FUN_0053e240` -> `FUN_0055bed0` ->
  `FUN_0058b7e0`, value at record `+0x24`) and then succeeds when
  `FUN_0053e2e0() < value`; `FUN_0053e2e0` is `FUN_0053e290(DAT_00661a88 - 1)`,
  a uniform draw in `0..=99`.

## Inputs (`FUN_00588b90`)

- `a` is slot `+0x1e0` of the decoy candidate, a random member of a pool
  (`FUN_00588700` draws `FUN_0053e290(count - 1)` over the list at `+0x30`).
  In the character vtable 0x0065ca70 slot `+0x1e0` is `FUN_004edc00`, which
  returns the short at `+0x7e`: effective espionage.
- Effective skills are recomputed by `FUN_004f16a0` (`FUN_004eecb0`,
  `FUN_004eecf0`, `FUN_004eed90`, ...). `FUN_004eecf0` sets `+0x7e` through
  `FUN_004edd30` to `base(+0x5a) + base * (+0x8c) / 100` (`FUN_0053e940`).
  The base skills are consecutive shorts from `+0x58` (diplomacy, espionage,
  ship design, troop training, facility design, combat, leadership, loyalty at
  `+0x66`); the effective copies run from `+0x7c` to `+0x8a` (leadership
  `+0x88`, loyalty `+0x8a`), and `+0x8c` is a shared percent modifier.
- `c` is slot `+0x1e0` (effective espionage) of the counterpart found by
  `FUN_00509330(system, key, &out)` (or `FUN_004fd790` on the fleet): it walks
  kind-3 objects of the system holder's side and returns the first whose
  `+0x96` short equals `key`. `key` is slot `+0x1bc` of the checked object.
- `b` is slot `+0x1c4` of the checked object.
- With no counterpart, `c` is 0 and the roll still runs; the lookup only
  feeds the return status (`FUN_00588b90.c:26-50`; detection,
  `FUN_00588a90`, is the same). On success the caller also requires
  `FUN_00534720(candidate, 1, ctx)` and `FUN_00558070(object, 1, ctx)`, then
  decrements `+0x34` (`:60-73`); a failure runs the exposure `FUN_005888f0`.

## Call chain (recovered 2026-09-27)

`FUN_00589620` has no code reference because it is a virtual slot: the pointer
at `.rdata` `0x0066a87c` is slot `+4` of the functor vtable `0x0066a878`, set
by `FUN_005895d0` (base `FUN_00587250`, which stores the manager at `+4`).

1. `FUN_00547f60(mission, ctx)` runs a mission. It checks the mission
   (`FUN_00520e40`, `FUN_00520ad0`), finds the target system (`FUN_00521070`,
   types `0x90..0x98`), builds a phase manager on the stack
   (`FUN_005897c0`), and calls `FUN_005898f0`.
2. `FUN_005898f0` calls `FUN_00589970`, which runs the phases
   `FUN_00589a40`, `FUN_00589e40`, `FUN_00589f10`, `FUN_0058a020`,
   `FUN_0058a130`, and `FUN_0058a1c0`.
3. `FUN_0058a020` builds the decoy functor when manager `+0x4c` is clear and
   walks the target system's defenders through `FUN_005875e0`,
   `FUN_00587600`, and `FUN_00587620`, which are `FUN_00587640` with
   different category flags.
4. `FUN_00587640` visits, at the system held by the pool (manager `+0x10`):
   its fighters (`FUN_005039d0`, types `0x1c..0x20`), its regiments
   (`FUN_00504c40`, `0x10..0x14`), and in each fleet (`FUN_004ffe70`,
   `0x08..0x10`) the capital ships (`FUN_00502db0`, `0x14..0x1c`), fighters,
   and regiments. It calls the functor with (defender, fleet or 0, &stop,
   ctx) and skips a defender with `+0x58` bit 0 (`IsDecoyed`) when asked.
5. `FUN_00589620` draws a random decoy from the pool (`FUN_00588700`, decoy
   count at pool `+0x30`) and, if one exists, calls
   `FUN_00588b90(manager, decoy, defender, fleet, ctx)`. There `a` is the
   decoy's effective espionage, `key` and `b` are the defender's slots
   `+0x1bc` and `+0x1c4`, and FDECOYTB applies when the defender is in a
   fleet.

So a decoy is a character or special force on the mission's decoy list.
Each enemy defender at the target is drawn off by a random decoy on a
TDECOYTB/FDECOYTB roll, and a success decrements the pool's defender count
(`+0x34`). The full rule follows below.

## Mission members (recovered 2026-09-27)

A mission object keeps its members in three lists, named by their
select-list notifiers: `+0x84` team (`MissionTeamSelectListNotif`), `+0x8c`
decoys (`MissionDecoySelectListNotif`), `+0x94` captured
(`MissionCapturedSelectListNotif`); a fourth notifier covers members that
finished (`MissionMissionMemberFinishedMissionSelectListNotif`).
`FUN_00525870(mission, types, team, decoys, captured)` builds an iterator
over the chosen lists (`FUN_00525a50` steps `+0x84`, `+0x8c`, `+0x94` in
turn). The member types are characters `0x30..0x3c` and special forces
`0x3c..0x40`:

| Constructor | Types | Lists |
|-------------|-------|-------|
| `FUN_00525e70` | special forces | team |
| `FUN_00526090` | characters | team |
| `FUN_00525f30` | special forces | decoys |
| `FUN_00526140` | characters | decoys |
| `FUN_00525bb0` | `0x30..0x40` | team, decoys, captured |

`FUN_00587bb0(pool, functor, chars, sforces, team, decoys, not_decoying,
ctx)` walks those iterators and applies `FUN_005883b0` to each member: a
member counts only if its role flags `+0x78` have bits 2 and 3 clear, it has
a location (`+0x1c`), and, when `not_decoying` is set, bit 5 is clear.

The player's mission dialog and the AI planners both fill the decoy list; see
`ai-mission-planning.md` (recovered 2026-09-28).

## Role and detector flags

Role flags live at character/special-force `+0x78`. Setters
`FUN_005344f0`..`FUN_005348e0` set one bit each (`FUN_0053a640(1 << n)`). For
each side's view of the object (`FUN_00539fd0(this, 1)` and `(this, 2)`) they
call a per-bit copier (`FUN_005358a0` for bit 0), which copies the bit from
the master and calls the view's slot `+0x2a4 + 4 * n`. The setter then calls
its own slot `+0x24c + 4 * n` (`CALL [EDX + 0x24c]` at `0x00534544`), which is
the stub `FUN_00524fb0` (returns 1) in the character (`0x0065ca70`), person
base (`0x00660d60`), and special-force (`0x0065e160`) vtables. All three hold
the notifiers at `+0x2a4 + 4 * n` (`+0x2a4` `FUN_00536940`, `+0x2b8`
`FUN_00536a80`; vtable dumps 2026-09-28):

| Bit | Setter | Notifier | Name |
|-----|--------|----------|------|
| 0 | `FUN_005344f0` | `FUN_00536940` | `RoleDecoyNotif` |
| 1 | `FUN_00534560` | `FUN_00536980` | `RoleMovingBetweenMissionsNotif` |
| 2 | `FUN_005345d0` | `FUN_005369c0` | `RoleMissionRemoveRequestNotif` |
| 3 | `FUN_00534640` | `FUN_00536a00` | `RoleMissionResignRequestNotif` |
| 4 | `FUN_005346b0` | `FUN_00536a40` | `RoleCanResignFromMissionNotif` |
| 5 | `FUN_00534720` | `FUN_00536a80` | `RoleIsDecoyingNotif` |
| 6 | `FUN_00534790` | `FUN_00536ac0` | `RoleAdriftNotif` |
| 7 | `FUN_00534800` | `FUN_00536b00` | `RoleOnMissionNotif` |
| 8 | `FUN_00534870` | `FUN_00536b40` | `RoleOnHiddenMissionNotif` |
| 9 | `FUN_005348e0` | `FUN_00536b80` | `RoleOnMandatoryMissionNotif` |

A defender ("detector") keeps `IsDecoyed` at `+0x58` bit 0, set by
`FUN_00558070` and announced by `FUN_00558310` (`DetectorIsDecoyedNotif`).

## Officer rank and detector slots

Character short `+0x96` is the officer rank: `FUN_004f17f0` adds `0x6800`,
`0x6c00`, or `0x7000` to the name string for ranks 1, 2, and 3 (TEXTSTRA
10816 "Ackbar", 37440 "Commander Ackbar", 38464 "Admiral Ackbar", 39488
"General Ackbar"). The detector interface is slots `+0x1bc` (the rank of the
officer that backs it), `+0x1c0` (class record `+0x58`, `FUN_00520b60`),
`+0x1c4` (class record `+0x5c`, `FUN_00520b70`), and `+0x1d4`
(`IsDecoyed`):

| Detector | Type | Vtable | `+0x1bc` rank | `+0x1c4` `b` |
|----------|------|--------|---------------|--------------|
| Capital ship | `0x14`, `0x18` | `0x0065d650`, `0x0065d930` | 2 Admiral (`0x0043c1f0`) | class `+0x5c` |
| Fighter squadron | `0x1c` | `0x0065def0` | 1 Commander (`FUN_0040f340`) | class `+0x5c` |
| Regiment | `0x10` | `0x0065e438` | 3 General (`FUN_00526f00`) | class `+0x5c` |

In the troop, fighter, and capital-ship DATs the fields after
`research_difficulty` are `uprising_defense`, `detection`, so class `+0x58`
and `+0x5c` read as those two, with `b` the class `detection`. The in-memory
record shift is inferred from that shared order, not traced.

`FUN_00509330(system, rank, &out)` returns the first existing character
(mode 3) of the system holder's side (`+0x24 >> 6 & 3`) held directly by the
system with that rank; `FUN_004fd790(fleet, rank, &out)` searches a fleet.
The counterpart is therefore the defending side's officer of the matching
rank: the fleet's Admiral for a ship, a Commander for fighters, a General
for regiments, looked up in the defender's fleet when it has one.

## Decoy phase (recovered 2026-09-27)

`FUN_00546ea0` (called from `FUN_0054e010`) runs on a mission state change
and calls `FUN_00547f60(mission, ctx)`. That builds the phase manager
(`FUN_005897c0`, vtable `0x0066a888`) with `+0x10` the target system and
`+0x14` the mission; the member pool is the sub-object at `+8`
(`FUN_00587550`, vtable `0x0066a830`), whose own `+8` and `+0xc` are the
system and the mission. `FUN_005898f0` runs `FUN_00589970` twice, with
`+0x44` set and then clear, unless the mission's side is 3.

Manager flags (manager offsets): `+0x18`/`+0x1c` select which members count
by location (`FUN_005883b0`); `+0x20` system defenders, `+0x24` fleet
defenders, `+0x28` capital ships, `+0x2c` fighters, `+0x30` regiments;
`+0x34` team count, `+0x38` decoy count, `+0x3c` defender count; `+0x48`
skip; `+0x4c` detected.

1. `FUN_00589a40` sets up. It skips (`+0x48`) a finished mission (`+0x64`),
   one without its record `+0x2c -> +0x60`, or mode 0. The mode is
   `FUN_00520e50`: a nibble of the mission's runtime block (`+0x54 -> +0x34`)
   chosen by the phase `+0x68` (2, 3, 7, 9); see "Phases, modes, and the
   success roll" below. Mode 2 enables the
   system's regiments when the system is held by the enemy of the mission's
   side; mode 3 also enables enemy fleets' ships and fighters when the
   mission's side has nothing there; modes 1 and 4 act on the first pass.
   It then clears every defender's `IsDecoyed` and every decoy's
   `IsDecoying`, and counts defenders, team members not decoying, and decoys
   (`FUN_005885f0`).
2. `FUN_00589e40` and `FUN_00589f10` run the team's own actions; not traced.
3. `FUN_0058a020`, the decoy phase, when not yet detected:

```
for each enabled defender d not IsDecoyed (FUN_00587640):
    decoy = random member of the decoy list (FUN_00588700: draw(0..count-1)
            over FUN_00587b30 = characters and special forces, decoys list;
            a decoy already decoying may be drawn again)
    if no decoy: stop the walk
    officer = holder's officer with rank d.+0x1bc (in d's fleet if any)
    x = decoy.espionage - d.detection - officer.espionage * GNPRTB[3588] / 100
    table = FDECOYTB if d is in a fleet else TDECOYTB
    if draw(0..99) < table(x):        // FUN_0053e340; a missing row hits
        decoy.IsDecoying = 1          // FUN_00534720
        d.IsDecoyed = 1               // FUN_00558070
        defender count -= 1
    else:
        FUN_005888f0(d, fleet, decoy) // the decoy is exposed, see below
```

4. `FUN_0058a130`, detection: each enabled defender that is not `IsDecoyed`
   rolls FOILTB (id 12) through `FUN_00588a90` and `FUN_0055e470`:

```
x = avg(team espionage)                  // FUN_005887a0, slot +0x1e0 over
                                         // team members not decoying
    - d.detection
    - count(team special forces not decoying)   // FUN_00587b70
    - officer.espionage * GNPRTB[3589] / 100
    - GNPRTB[3584]
detected = draw(0..99) < FOILTB(x)       // sets manager +0x4c, ends the walk
```

5. `FUN_0058a1c0`, when detected: the mission's slot `+0x1dc` receives 3, or
   4 when the mission's phase (`+0x54 -> +0x1c`) is above 4, and each team
   member faces a random defender through `FUN_005888f0`.

`FUN_005888f0(d, fleet, member)` finds the defender's officer, then calls
`FUN_005349e0(member, officer id, officer slot +0x1f0, ctx)`. When the
member has `CanResignFromMission` and no remove request, `FUN_00534640` sets
its resign request, and a member with either request leaves the team or
decoy count. So a decoy that fails is treated like a detected team member,
and one that succeeds keeps its defender out of detection.

GNPRTB (`FUN_0055e340`, shipped values): 3584 = -1, 3585 = 1, 3586 = 60,
3587 = -100, 3588 = 35 (decoy counterpart percent), 3589 = 35 (detection
counterpart percent).

Three more helpers sit beside the table rolls:

- `FUN_0055e520(side, value, &passed, &chance)`: `chance = DAT_00661a88 - value`
  (`DAT_00661a88` is the constant 100, `blockade-troop-withdrawal.md`),
  `passed` is the draw `FUN_0053e2f0(chance)`. `FUN_00588da0` passes the
  object's owner bits (`+0x24 >> 6 & 3`) and its slot `+0x1f8`.
- `FUN_0055e570(x)`: `x >= ` GNPRTB 3585 (`DAT_006bb708`, shipped 1).
  `FUN_005888a0` sets its result when the member's short `+0x8c` passes this,
  `+0x78 & 0xc` is 0 and `+0xac` bit 0 is clear.
- `FUN_0055e9a0(a, b, &over, &under)`: `over = a - b` when `a > b`, else
  `under = b - a`; the other output is 0. No caller in the listing.

Shipped tables (step rows, `threshold -> value`): TDECOYTB and FDECOYTB are
identical, `-20:10, -19:20, -9:40, 0:50, 10:60, 20:70, 30:80, 40:90, 50:92,
60:94, 70:95, 80:96, 90:98, 100:99`. FOILTB is `-20:99, -19:98, -9:96, 1:94,
11:90, 21:80, 31:70, 41:60, 51:50, 61:45, 71:40, 81:35, 91:30, 101:25`.

## Open

- `FUN_005349e0` (what the officer does to an exposed member), the phases
  `FUN_00589e40` and `FUN_00589f10`, and the mission's slot `+0x1dc` are not
  traced.
- The mission-kind record `+0x34` nibbles are not matched to a MISSNSD
  column, so each mission's mode per state is unknown.
- The player's and the AI's decoy assignment paths: resolved in
  `ai-mission-planning.md`.
- The in-memory shift that puts `detection` at class `+0x5c` is inferred.
- `FUN_00520cd0` (Incite and Subdue leadership in the uprising incident)
  averages over `FUN_00525bb0`, all team, decoy, and captured members; the
  port reads only the mission's one agent.
- The port has no decoy characters on missions. Its invented `is_decoy` roll
  and `MissionSystem::check_decoy` are removed (2026-09-27); the `is_decoy`
  field stays only for the save layout.

## Recovered 2026-09-28

### Exposure (`FUN_005888f0`, `FUN_005349e0`)

`FUN_005888f0(manager, d, fleet, member, ctx)` (`FUN_005888f0.c:24-36`) looks
up the defender's officer by rank `d.+0x1bc`: `FUN_00509330` on the system
when `fleet` is null, else `FUN_004fd790` on the fleet. With an officer it
passes the officer's key and its slot `+0x1f0`; without one it passes the
defender's own key and 0 (`:39-50`). Then:

```
FUN_005349e0(member, key, c, ctx):                 // FUN_005349e0.c:10-24
    x = member.slot_1f0() - c                      // effective combat
    roll RLEVADTB (table 13) at x                  // FUN_0055bfa0.c:5-6, FUN_0053e310
    if the row was found:                          // inferred: EAX of FUN_0053e340
        if draw(0..99) < RLEVADTB(x): member.slot_208(ctx)      // evades
        else:                         member.slot_20c(key, ctx) // captured
```

Slot `+0x1f0` in the character vtable `0x0065ca70` is `FUN_004edc40`, the
short at `+0x86`: effective combat (the effective skills run diplomacy `+0x7c`,
espionage `+0x7e`, ship design `+0x80`, troop training `+0x82`, facility
design `+0x84`, combat `+0x86`, leadership `+0x88`, loyalty `+0x8a`).

- Evade, slot `+0x208` `FUN_004ef450` (`:5-9`): `FUN_00534c20` sets the
  resign request (`FUN_00534640`) when the member can resign (`+0x78` bit 4)
  and `+0x68` holds a key (`FUN_00534c20.c:6-11`); then slot `+0x2e4`.
- Captured, slot `+0x20c` `FUN_004ef480` (`:9-27`): unless destroyed
  (`+0x50` bit 3), slot `+0x2e4`, then `FUN_004ee3e0(member, 0)` sets the
  status short `+0x98` to 0 (`FUN_004ee3e0.c:12-17`; 1 is free, see
  `uprising-incident.md` code 4), then `FUN_004ef190` stores the captor key
  (the officer, or the defender) at `+0x54 -> +0x30` (`FUN_004ef190.c:4-5`).
  The `+0x98` change handler, slot `+0x334` `FUN_004f18e0`, raises event
  `0x30a`.
- Slot `+0x2e4` is `FUN_004ef5f0`, the injury roll already recovered for the
  uprising incident code 3: chance `max(G2565, 100 - combat)`, injury
  `rand(chance) + rand(G2567) + G2566` into `+0x94` through `FUN_0053e990`
  and slot `+0x2f0` `FUN_004ef6d0` with cause 10 (`FUN_004ef5f0.c:8-12`).

So an exposed member (a detected team member, or a decoy whose roll failed)
rolls RLEVADTB on its combat against the matching officer's combat. It either
evades, resigning from the mission and risking injury, or is injured and
captured by that officer, or by the defender when no officer holds that
rank. Afterwards (`FUN_005888f0.c:61-84`), a member that can resign and has
no remove request gets a resign request, and one with either request leaves
the pool's team count (`+0x2c`) or decoy count (`+0x30`, when `+0x78` bit 0
is set). The pool sits at manager `+8`, so these are manager `+0x34` and
`+0x38`.

### Phases 2 and 3 (`FUN_00589e40`, `FUN_00589f10`)

`FUN_00589970` runs, in order, setup `FUN_00589a40`, `FUN_00589e40`,
`FUN_00589f10`, the decoy phase `FUN_0058a020`, detection `FUN_0058a130`,
and the detected outcome `FUN_0058a1c0`, all skipped when setup sets `+0x48`
(`FUN_00589970.c:13-50`).

- `FUN_00589e40` (`:18-41`) first calls `FUN_0058a5b0` (`FUN_0058a5f0`,
  `FUN_0058a6c0`, not read), then walks a key list through `FUN_0058a9e0` and
  `FUN_0058a2c0`. `FUN_0058a9e0` seeds the walk with two fixed DatIds,
  `0x32000242` and `0x35000281` (`FUN_0058a9e0.c:19-35`), looked up in the
  manager's `+0x68` list by `FUN_0058ac50`. This is a phase for specific
  characters, not a general rule; its effect is not traced.
- `FUN_00589f10`, betrayal (`:13-43`), runs when the mission's slot `+0x1c8`
  is true and nothing has detected the mission yet (`+0x4c == 0`):

```
for each team or decoy member (characters and special forces) not decoying:
                                         // FUN_00587b90 -> FUN_00587bb0(1,1,1,1,1)
    betrays = draw(0..99) < 100 - member.loyalty
                                         // functor 0x0066a840 slot +4 FUN_005890d0
                                         // -> FUN_00588da0: slot +0x1f8 FUN_004edc60,
                                         // loyalty +0x8a; FUN_0055e520: DAT_00661a88 (100) - x
    if betrays: pool +0x38 = member      // FUN_00588da0.c:22-27; the walk stops
if pool +0x38 holds a member:            // FUN_0048a640
    detected = 1                         // manager +0x4c
    for each other member:               // functor 0x0066a868 slot +4 FUN_00589490
        if draw(0..99) < member.+0x8c:   // FUN_00588e80, FUN_0055e550 (x itself)
            member.+0xa0 = traitor       // FUN_004ee5c0 twice (clear, then set), slot +0x340
```

So a disloyal member betrays the mission before the decoys act. Setting
detected skips the decoy phase and detection, and sends the whole team to
`FUN_0058a1c0` (exposure). Each other member may learn the traitor
(`+0xa0`); what `+0xa0` and the `+0x8c` short mean is inferred only. The
earlier note reads `+0x8c` as a shared percent modifier. Which mission kinds
answer true in slot `+0x1c8` is not traced.

### The detected outcome (`FUN_0058a1c0`) and mission slot `+0x1dc`

When the mission is detected (`+0x4c`), `FUN_0058a1c0` (`:17-24`) calls the
mission's slot `+0x1dc` with 3, or 4 when `FUN_00520ad0` (the phase
`+0x54 -> +0x1c` above 4) is true. It then walks the team through
`FUN_00587f80` (each member faces a random defender through `FUN_005888f0`),
and through `FUN_00587b70` (`:25-40`).

Slot `+0x1dc` (`FUN_005233d0` -> `FUN_00521900`) sets the mission's end
code `+0x64`, validated `0..0x10`, not the phase; see "Phases, modes, and the
success roll" below. The mission validator
`FUN_00522480` computes a new state and passes it to the same slot
(`FUN_00522480.c:137`). It uses state 5 when no member lacks a remove or
resign request (`+0x78 & 0xc`, `:40-52`, message `0x40`/`0x91`) and state 1
when the current state `+0x68` is `0xb` (`:130-133`). `FUN_00545240.c:104`
sets 7. So detection moves the mission to state 3 (4 for the later mission
kinds) and exposes every team member. A non-zero end code ends the mission
(`FUN_00520e40`), and the next phase step jumps to `0xb`. The names of end
codes 1, 3, 4, 5, and 7 are not recovered.

### Adding members (`FUN_00522b30`)

`FUN_00522b30(mission, member_key, as_decoy, ctx)` is a virtual slot shared by
every mission class; the data references at `0x0065efc4`, `0x00663d44`,
`0x00663fd4`, and eight more vtables point to it. It is the only caller that
sets the Decoy role (`FUN_005344f0`, call at `0x00522e32`; the other caller,
`FUN_00536220`, clears it when a member leaves). It accepts a
member only when all of these hold (`FUN_00522b30.c`):

- its DatId family is `0x30..0x3f`, a character or a special force;
- it is not already in the team, decoy, or captured list (`FUN_00520c30`:
  `FUN_00520bd0`, `FUN_00520bf0`, `FUN_00520c10`);
- it exists and is on the mission's side (`+0x24 & 0xc0` equal);
- it is not asked to decoy while a prisoner (slot `+0x1d4` `FUN_004edc70`,
  `+0xac` bit 0; `(prisoner == 0) || !as_decoy`, `:55-61`);
- it has no mission yet (`+0x68` key empty, `FUN_0042d170`);
- it travels with the current members: same location key (slot `+0xc`), the
  same en route active bit (`+0x50` bit 5), and, when autorouting (bit 11),
  the same arrival tick (`+0x44`).

It then records the mission on the member (`FUN_00534230`, `FUN_005342e0`),
sets OnHiddenMission (bit 8) when `FUN_00520b70(mission)` holds,
OnMandatoryMission (bit 9) from mission `+0xa4` bit 2, Decoy (bit 0) =
`as_decoy`, and CanResign (bit 4) from `FUN_00520b90(mission)`. Last, it
inserts the member into the decoy list `+0x8c` (`FUN_00521ef0`) when
`as_decoy`, else the captured list `+0x94` for a prisoner (`FUN_00521fb0`),
else the team `+0x84` (`FUN_00521e30`).

When a member leaves, `FUN_00536220` clears role bits 0, 2, 3, 4, and 5
(`FUN_005344f0`, `FUN_005345d0`, `FUN_00534640`, `FUN_005346b0`,
`FUN_00534720`) and recomputes OnMission (bit 7) through `FUN_00534950`: it
clears the bit when the member's mission key `+0x68` is empty and bit 1 is
clear. When given a key it clears only bit 1 (`FUN_00534560`). It does not
touch OnHiddenMission (bit 8); where that bit clears is not traced.

No count limit appears in this function. The decoy flag arrives only as an
argument of this slot. Who calls the slot with `as_decoy = 1` (the player's
mission dialog, or the AI) is not traced, since its callers are virtual.

## Phases, modes, and the success roll (recovered 2026-09-28, pass 3)

Betrayal (slot `+0x1c8`, `FUN_00589f10`) is on only in phase 9 for every agent
class (`0x592500`: `+0x68 == 9`). Move, Return, Autorouting, and Adrift return
0 (`FUN_006158b0`).

Sources are a full read-only decompile of REBEXE.EXE (22,808 functions) plus
vtable dumps of the 29 mission classes.

### Mission classes

Every mission class shares `FUN_00522b30` at vtable slot `+0x1cc`. Slot `+4`
returns the class's MISSNSD family (`MOV EAX, imm`): Move `0x41`, Return
`0x42`, Autorouting `0x43`, Adrift `0x44`, Diplomacy `0x51`, Espionage `0x52`,
Research `0x53`, Reconnaissance `0x54`, Recruitment `0x55`, Incite `0x56`,
Subdue `0x57`, Jedi Training `0x58`, Rescue `0x61`, Abduction `0x62`,
Assassination `0x63`, Palace `0x64`, Bounty `0x65`, Sabotage `0x69`, Death
Star Sabotage `0x6a`, Dagobah `0x71`, Vacation `0x72`, Pickup `0x73`.

### The phase `+0x68` and the end code `+0x64`

- `FUN_00521980` sets the phase `+0x68` (validated `0..0xb`) and calls slot
  `+0x1f4` with the old and new phase. The base handler `FUN_00524b70` stores
  the new phase in the runtime block (`+0x54 -> +0x1c`, except `0xb`) and
  recomputes `+0x54 -> +0x18`: 1 when no member has `+0x50` bit 11
  (autorouting) (`FUN_00522a90`).
- `FUN_005227d0` steps the phase one at a time. After phase 10 a mission whose
  record `+0x58` is set loops back to 8. A mission with an end code
  (`FUN_00520e40`, `+0x64 != 0`) goes straight to `0xb`.
- In `FUN_00524b70`, phase 4 starts each member's transit to the target
  (`FUN_00556430` into `+0x6c`/`+0x78`), phase 6 lands them
  (`FUN_004f7640`), and phase 8 schedules the mission timer (event `0x38b`,
  range from record `+0x50`/`+0x54`, `FUN_005236e0`).
- `FUN_00546ea0` runs on each phase change. In order: the decoy manager
  (`FUN_00547f60`), then `FUN_00548120`, then `FUN_00548370` on phase 2, then
  `FUN_005484d0`, then `FUN_00548840`.

### The mode word (`+0x54 -> +0x34`)

The word is set once, at init (slot `+0x94`), through `FUN_00522a60`, which
copies it into `+0x54 -> +0x30/+0x34`. No DAT column holds it; it is a
constant in code.

- `FUN_005236e0` builds `(0 & ~0xf ^ 2) & 0xffff411f | 0x4110 = 0x4112`.
  Every concrete class reaches it: slot `+0x94` is `00576d10` or `00574420`
  (both calling `FUN_00574080`), or `005761b0` (calling `FUN_00576250`), each
  ending in `FUN_00593a80` -> `FUN_005236e0`.
- Adrift alone (`00576960` -> `FUN_00576a20`) overrides it with `0x4103`.

So for every mission except Adrift:

| Phase | Mode | Setup (`FUN_00589a40`) |
|-------|------|------------------------|
| 2 | 2 | first pass: system defenders and regiments when the system is held by the mission side's enemy |
| 3 | 1 | first pass: enemy fleets' ships and fighters when the mission side has no fleet there, plus the system when `FUN_00520bb0` (target != current location) |
| 7 | 1 | as phase 3 |
| 9 | 4 | first pass: the target's container (`FUN_00521160`); the system's regiments when it is the target system, else the enemy fleets |

The mode applies only when `+0x54 -> +0x18` is set, meaning no member is
still autorouting. Mode 0 and the second pass skip, except that mode 3 runs
the mode-2 body on the second pass. Adrift uses 3, 0, 1, 4.

### Record flags (`+0x2c`)

The in-memory MISSNSD record sits `0x28` above the file offsets. This is
inferred from the pattern below: `+0x50`/`+0x54` are the file's
`max_officers`/`base_duration`, which read as a duration base and spread,
e.g. Jedi Training `(60, 30)`.

- `+0x58`, file col6, repeat. Phase 10 loops to 8. Set for Diplomacy,
  Research, Incite, and Subdue.
- `+0x5c`, file col7, hidden (`FUN_00520b70`, OnHiddenMission). Set for
  Move, Return, Autorouting, Adrift, Dagobah, Palace, Vacation, Pickup, and
  Bounty.
- `+0x60`, file col8, decoy phases on (`FUN_00520b80`, setup skips when 0).
  Set for every mission except Bounty.
- `+0x64`, file col9, CanResign (`FUN_00520b90`). Set for the agent missions.

### The success roll (phase 10)

The agent classes override slot `+0x1f4` with `FUN_00592f50`: every MSTB
mission, plus Reconnaissance, Research, Palace, and Bounty. Jedi Training
(`0x571410`) and Dagobah (`0x5751d0`) have their own handlers; Vacation and
Pickup (`0x594460`) forward straight to `FUN_00592f50`, and Move, Return, Autorouting, and Adrift keep the base
`FUN_00524b70`. `FUN_00592f50` runs `FUN_00524b70`, then on phase 10:

1. It calls slot `+0x278`.
2. For each team character (`FUN_00526090`: the team list `+0x84` only,
   families `0x30..0x3c`), it rolls `FUN_00593320` -> slot `+0x274(member)`.
   It collects the results by member and applies them through slot
   `+0x27c(member, success, ctx, 10)`, in order of the map's `+0x1c`.
3. For each team special force (`FUN_00525e70`, `0x3c..0x40`), it rolls and
   applies the same way.
4. It calls slots `+0x280` and `+0x284`. On phase `0xb` it calls `+0x284`
   only.

Decoys (`+0x8c`) and captives (`+0x94`) never roll success
(`FUN_00525870`/`FUN_00525a50`: flags `(1,0,0)` select the team). Slot
`+0x274` per class, with `a`, `b`, `c` the table wrapper's arguments
(`uprising-incident.md`):

| Mission | Slot fn | Table | Input |
|---------|---------|-------|-------|
| Diplomacy | `00573ff0` | DIPLMSTB | a = member diplomacy (`+0x1dc`), b = system support for the side (`FUN_00507270`), c = `FUN_005091f0`: `(c - b) + a` |
| Espionage | `00573090` | ESPIMSTB | member espionage (`+0x1e0`) |
| Rescue | `0056ae30` | RESCMSTB | member combat (`+0x1f0`) |
| Sabotage | `0056a2d0` | SBTGMSTB | (espionage + combat) / 2 |
| DS Sabotage | `00574600` | DSSBMSTB | (espionage + combat) / 2 |
| Recruitment | `0056b7b0` | RCRTMSTB | member leadership (`+0x1f4`) - `FUN_00507270` |
| Incite | `005719d0` | INCTMSTB | (leadership - `FUN_00507270`) - `FUN_005091f0` |
| Subdue | `00569b90` | SUBDMSTB | (`FUN_005091f0` - `FUN_00507270`) + leadership |
| Abduction | `00576e50` | ABDCMSTB | member combat - target combat (`FUN_00586c80`) |
| Assassination | `005765c0` | ASSNMSTB | member combat - target combat |

Slot `+0x274` also covers the missions without an MSTB. Reconnaissance
always succeeds (`0x56bea0`: 100) and Research returns 0 (`0x56cb20`). Palace
rolls `FUN_0055c910` on (espionage + combat) / 2, not read. Bounty and the
leisure classes return `DAT_00661a88`, which is 100.

Character slots `+0x1dc..+0x1f8` return the effective skills `+0x7c..+0x8a`
in order: diplomacy, espionage, ship design, troop training, facility design,
combat, leadership, loyalty. `FUN_00507270` and `FUN_005091f0` take the
target system with the mission side's index and are not read; the notes
cite them only by role.

### Who assigns decoys

A mission is created through `FUN_005422f0` or `FUN_00542b60`.
`FUN_0054bb90(mission, team_in, decoy_in, ...)` takes separate team and decoy
key lists from the requester. It moves any prisoner (slot `+0x1d4`) from
either list to the captured list, and fails with message `0x40`/`0x91` when
the team is empty. `FUN_0054c200` then adds the team with `as_decoy = 0`,
the decoys with 1, and the captured with 0, through slot `+0x1cc`
(`FUN_00522b30`).

Decoys are therefore chosen explicitly by whoever issues the request. The
request is mission-create command `0x250` (command object `FUN_0054cd80`,
vtable `0x00661e28`; validate `FUN_0054d280`, execute `FUN_0054d360`; factory
registered by `FUN_0051ef80`). A mission order (`0x240`..`0x242`) fills it:
`FUN_004f4a00` copies the order's team `+0x2c` to command `+0x64` and its
decoys `+0x58` to `+0x6c`. The player's mission dialog (`FUN_0046c3c0`)
moves chosen characters between the two lists, and the AI planners pick
decoys for Sabotage, Rescue, Incite, Espionage, DS Sabotage, Abduction, and
Assassination (`ai-mission-planning.md`, recovered 2026-09-28).

### Still open (2026-09-28, after pass 3)

- The sender of command `0x250`: resolved in `ai-mission-planning.md`.
  Phase stepping and the phase-10 outcomes are in `mission-lifecycle.md`.
- The phase-2 character hook (`FUN_00589e40`, DatIds `0x32000242` and
  `0x35000281`), `FUN_0058a5b0`, and the meaning of `+0xa0` and `+0x8c`.
- Slots `+0x278`, `+0x27c` (the per-member outcome), `+0x280`, and `+0x284`,
  and the end codes' names.
- `FUN_00507270` and `FUN_005091f0` (the support and counter terms in the
  Diplomacy, Recruitment, Incite, and Subdue inputs).

## Re-read for the port (2026-09-29, F-019 phase 3)

Read-only decompiles of the functor bodies (vtables `0x0066a818`,
`0x0066a828`, `0x0066a850`, `0x0066a858`, `0x0066a860`, `0x0066a880`, slot
`+4`), the member iterators, and the special-force slots.

### The run (`FUN_00547f60`, `FUN_005898f0`)

- It returns at once with an end code. It reads the location from the
  target `+0x78` (`FUN_00521070`) when the phase (`+0x54 -> +0x1c`) is above 4
  (`FUN_00520ad0`), else the members' current location (`FUN_00520f40`), and
  runs only at a system (`0x90..0x98`).
- `FUN_005898f0` runs `FUN_00589970` twice on the same manager, first with
  `+0x44` = 1, unless the side is 3. Setup only sets its flags, so the second
  pass inherits them. Modes 1, 2, and 4 set skip (`+0x48`) on the second pass,
  which still runs the validator and the counts.
- A run that leaves an end code calls `FUN_004f9510(mission, opponent, ...)`
  (`FUN_00547f60.c:71-92`); not read.
- `FUN_00546ea0` runs the manager after every phase change, then
  `FUN_00548120`, `FUN_00548370` on phase 2, `FUN_005484d0`, `FUN_00548840`,
  and `FUN_00522980` (ready).

### Setup (`FUN_00589a40`)

It runs the validator `FUN_00522480` first, then skips on an end code, a
record without column 8 (`FUN_00520b80`), or mode 0. The opponent is
`2 - (side != 1)`. `local_70` is "the opponent has a fleet here"
(`FUN_004ffef0` + `FUN_005131b0`), `bVar1` "the side has a fleet here"
(`FUN_005275d0`), and `bVar10` "the system's holder is the opponent".
Manager flags: `+0x18` counts members standing at a system, `+0x1c` members
in a fleet (`FUN_005883b0` compares the member's container with its
system), `+0x20` walks the system, `+0x24` the fleets, `+0x28` capital
ships, `+0x2c` fighters, `+0x30` regiments.

| Mode | First pass |
|------|------------|
| 1 | `+0x1c`; `+0x18` when the target is remote (`FUN_00520bb0`); `+0x24 +0x28 +0x2c` when the opponent has a fleet here and the side has none |
| 2 | `+0x18`; `+0x20 +0x30` when the holder is the opponent |
| 4 | `+0x18 +0x1c`; when the target (side copy, `FUN_00521160`) is a system, `+0x20 +0x30` if the holder is the opponent, else `+0x24 +0x28 +0x2c` if the opponent has a fleet here |

It then clears every defender's `IsDecoyed` and every decoy's `IsDecoying`
and counts the defenders (`+0x34`, the full walk `FUN_005875e0`), the team
(`+0x2c`, `FUN_00587b50`: team characters and special forces not decoying),
and the decoys (`+0x30`, `FUN_00587b30`).

### The walks

- `FUN_00587bb0(pool, f, chars, sforces, team, decoys, not_decoying)` visits
  team special forces, team characters, decoy special forces, then decoy
  characters, each counted by `FUN_005883b0`: no remove or resign request,
  not decoying when asked, a location, and the location flag above.
- `FUN_00588000` (the exposure walk, `FUN_00587f80` = `(1,1,1,0,1)`) visits
  team characters of families `0x30..0x37`, then `0x38..0x3b`
  (`FUN_00526350`, `FUN_005261f0`), then the decoy lists when asked
  (`FUN_00526400`, `FUN_005262a0`).
- `FUN_00587640` walks the system when `+0x20`: its fighters, then its
  regiments when `+0x30` and asked. It walks each fleet when `+0x24`: each
  capital ship when `+0x28` and asked, then that ship's fighters when
  `+0x2c`, then its regiments when `+0x30`. `FUN_005875e0` asks for all
  three kinds, `FUN_00587600` ships, `FUN_00587620` fighters, each skipping
  decoyed defenders.

### The phases

- Decoys (`FUN_0058a020`): with the system flag, the full walk; with the
  fleet flag, the ships walk, then the fighters walk. So a defender whose
  decoy failed meets another decoy on a later walk. The functor
  (`FUN_00589620`) draws `draw(0..decoys-1)` and takes that counted decoy
  (`FUN_00587360` -> `FUN_005873c0`), stopping the walk when there is none.
- Detection (`FUN_0058a130`, functor `FUN_005896e0`): nothing when the team
  count is 0. The first call takes the team's average espionage (sum over
  `FUN_00587b50` / team count, `FUN_005887a0`) and the count of team special
  forces not decoying (`FUN_00587b70`, `FUN_005872a0`). Each defender then
  rolls FOILTB until one detects.
- Betrayal (`FUN_00589f10`) walks `FUN_00587b90` and stops at the first
  traitor. Its second walk (`FUN_00587f60`) lets each other member learn the
  traitor (`+0xa0`); the field is not ported.
- The detected outcome (`FUN_0058a1c0`) sets end code 3, or 4 past phase 4,
  only when the record's column 9 (can resign) is set. Each team character,
  then each team special force, not decoying: a member holding a key at
  `+0x9c` goes to `FUN_00588d30` (not read); the others face a random
  defender (`FUN_00588650`: `draw(0..defenders-1)` over the full walk), and
  the walk stops when none is left (`FUN_005891e0`).

### Exposure (`FUN_005888f0`, `FUN_005349e0`)

With no officer the captor is the defender and the counterpart's combat is
0 (`:26-53`). `FUN_005349e0` rolls RLEVADTB on combat minus that and acts
only when the row is found. Afterwards any member that can resign (bit 4),
has no remove request, and is on a mission gets a resign request; one with
either request leaves the team count, or the decoy count when it holds the
Decoy role.

Special forces (vtable `0x0065e160`) read their base skills: espionage
`+0x5a` (`FUN_00503c40`), combat `+0x62` (`FUN_00503c80`), loyalty `+0x66`
(`FUN_00503ca0`). One that evades only gets the resign request
(`FUN_00503ea0` -> `FUN_00534c20`, no injury); one captured is destroyed
(`FUN_00503eb0`: slot `+0xac(9)`).

### The validator's rule 1

`FUN_00522480.c:42-52` gives end code 5 unless some team member has no
remove or resign request, or `+0xa4` bit 1 is set. An empty team therefore
ends with 5, and so does a team whose members were all deleted.

### Port decisions (`mission_detection.rs`)

- The setup sets `+0x28 +0x2c` only with `+0x24` and `+0x30` only with
  `+0x20`, so the port folds each kind flag into its walk flag.
- With no officer ranks, the officer terms of the decoy and detection rolls
  (`espionage * G3588 / 100`, `espionage * G3589 / 100`) are 0 and omitted.
- A captured member's capture applies after the run, so the run marks it
  gone from its lists at once, as it does a destroyed special force.
- Only a decoy decoys, and the lists that skip decoying members never meet
  one, so the port keeps no `IsDecoying` mark. A resigning team member's
  exit from the team count is dropped too: nothing reads that count once
  the team is exposed.
- The draws come from a SplitMix64 stream seeded per mission and tick by one
  caller roll (`DETECTION_SEED_ROLL`).

### Still open

`FUN_00589e40` (the character hook), `FUN_00588d30` (the `+0x9c` branch),
the traitor's own fate, `FUN_004f9510`, `+0xa4` bit 1, and the officer ranks
of a side (the port assigns none).
