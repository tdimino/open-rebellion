# Mission teams and decoys: player dialog and AI planners

Recovered 2026-09-28 from REBEXE.EXE (read-only, no analysis). This answers
the open question in `decoy-roll.md`: who fills a mission's decoy list. Both
the player and the AI can fill it. "First read" marks a tracer's reading
that was not re-read for this note.

## Orders and the create command

Mission orders are registry-A objects. `FUN_0051f4b0` registers them with
`FUN_0051f930`, and `FUN_004f5cd0(id)` builds one. The ids are:

| Id | Constructor |
|----|-------------|
| `0x240` | `FUN_004f5250`, vtable `0x0065d118` |
| `0x241` | `FUN_004f5490`, vtable `0x0065d170` |
| `0x242` | `FUN_004f45b0` |

Slot `+0xc` returns the id (`FUN_0057c4a0`, `FUN_004f5500`, `FUN_0057db20`).

An order holds:
- the team list at `+0x2c` (slot `+0x24` sets it, `FUN_0051fb70`; slot `+0x28` gets it, `FUN_0051fb80`);
- a value at `+0x34` (slot `+0x2c`);
- `+0x44` and `+0x48`;
- the target at `+0x4c`;
- the decoy list at `+0x58`.

The order's mission slots are:

- **`+0x48`** is `FUN_004f4990`. It builds registry-B command `0x250` (`FUN_0054cd80`).
- **`+0x4c`** is `FUN_004f4a00`. It copies the order's lists and values into the command and submits it (`FUN_0054ee30`):
  - team `+0x2c` → command `+0x64`
  - decoys `+0x58` → command `+0x6c`
  - `+0x44`/`+0x48` → command `+0x50`/`+0x54`
  - side `0xf8000006` → command `+0x3c`
- **`+0x50`** is `FUN_004f4b60`, the validator. Its errors:
  - `1/0x16` when the team and decoys are both empty
  - `0x40/0x92` and `0x40/0x93` for target checks
  - `1/0x28` for a target on the wrong side

Command `0x250` executes in `FUN_0054d360`, which calls `FUN_00542b60` and
then `FUN_0054bb90(src, team, decoys, ...)`. `FUN_0054c200` then adds the
team with `as_decoy = 0` and the decoys with `as_decoy = 1`
(`decoy-roll.md`, "Who assigns decoys").

`FUN_004f5380` is the preflight. It runs `FUN_005422f0` on the order's lists
without creating anything, and the dialog code calls it (`FUN_0042a320`,
`FUN_0046a9c0`).

## The player: the mission dialog

`FUN_0046c3c0` is the dialog's message handler. Button notifications from
controls `0xc8` and `0xc9` become cases `0xca` and `0xcb`
(`param_1 = (id != 200) + 0xca`, `FUN_0046c3c0.c:86`).

- Case `0xca` moves the selected characters from the team to the decoys.
- Case `0xcb` moves them from the decoys back to the team.

Each case reads both lists (`FUN_0046c850` team, `FUN_0046c880` decoys),
writes the new decoy list to order `+0x58` with `FUN_004f43b0`
(`FUN_0046c3c0.c:158-166`), and sets the team through slot `+0x24`. The
order sits at dialog `+0x120`.

So the player picks the members and marks any of them as decoys. A new order
starts with no decoys (`FUN_00402720` builds `0x240` and leaves `+0x58`
empty).

## The AI: mission planners

Each side keeps mission requests at side `+0xd8` and planners at side
`+0xa8`. `FUN_004d2490` runs a request's state machine (first read):

1. `FUN_004d2580` builds the primary planner from the side's `+0x318`
   (mission kind) and `+0x31c` (subtype).
2. `FUN_004d2610` and `FUN_004d2690` handle secondary requests.
3. `FUN_004d27b0` ticks the planner through its slot `+0x2c`.

What sets side `+0x318` (the strategic choice of mission) is not traced.

`FUN_0042f830` builds a planner by kind and subtype. Constructor and slot
addresses are first read; the kind switch is re-read (`FUN_0042f830.c:13-198`).

| Kind | Subtype | Team selector | Decoy selector |
|------|---------|---------------|----------------|
| Diplomacy `0x51` | all | `004815a0` | `0047f500`: none, trims |
| Espionage `0x52` | 6 | `00480f30` | `00481100`: none |
| Espionage `0x52` | other | `004808c0` | `00480aa0` |
| Research `0x53` | 1, 2, 3 | `0047bcd0`, `0047c2b0`, `0047c900` | none |
| Recon `0x54` | 5 | `0047f790` | none |
| Recon `0x54` | other | `0047f320` | `0047f500`: none, trims |
| Recruitment `0x55` | all | `0047db40` | none |
| Incite `0x56` | all | `0047ff90` | `00480110` |
| Subdue `0x57` | all | `0047cee0` | none |
| Rescue `0x61` | all | `0047e270` | `0047e3f0` |
| Abduction `0x62` | not 4 | `00483290` | `00483410` |
| Abduction 4, Assassination `0x63` | all | `00482ce0` | `00482ec0` (mask from `+0x7c`) |
| Sabotage `0x69` | all | `0047d520` | `0047d720` |
| DS Sabotage `0x6a` | all | `00482180` | `00482330` |

`FUN_00481100` sets only the "decoys done" flag (`+0x20 |= 2`).
`FUN_0047f500` also trims the list (`FUN_0047b9e0`). So Diplomacy,
Recruitment, Research, Subdue, Recon, and Espionage subtype 6 send no AI
decoys. The table puts Recon subtype 5 and Research among the "none" cases
through `00481100`; that reading is first read.

### The planner state machine (`FUN_004bced0`)

The state is at `+0x1c` and the flags are at `+0x20`.

| State | Action |
|-------|--------|
| 1 | exits: flags `0x60000000` → `FUN_0047b250`; `0x10000000` → `FUN_0047aff0`; else state 2 |
| 2 | `FUN_0047b110`: drop team (`+0x3c`) and decoy (`+0x44`) members that fail `FUN_0047ba40`; `0x100` if any remain |
| 3 | slot `+0x48`: the team selector |
| 4 | slot `+0x4c`: the decoy selector |
| 5 | slot `+0x50` |
| 6 | `FUN_004bd0a0`: choose the target, flag 4 |
| 7 | `FUN_004bd570`: members not at the target get a move order; flag 8 when the team is there, `0x10` when the decoys are |
| 8 | once flags 8 and `0x10` are both set: `FUN_0047b360` emits order `0x240` |

`FUN_0047b360` builds the order:
- the team is taken from planner `+0x3c` (slot `+0x24`);
- the decoys are taken from planner `+0x44` (into order `+0x58`, `FUN_004f43b0`);
- each member is kept only when its record's `+0x30` has none of the bits `0x74000f1`, and it is then marked with `0x4000`;
- the target is taken from planner `+0x2c`;
- planner `+0x18` is copied into order `+0x44`.

The AI moves decoys to the target before it sends the order, the same as the
team.

### Selectors

The planner's candidate pool is its side's records (planner `+0x64`). A
record carries:
- flags at `+0x30`;
- a capability mask at `+0x34`;
- a skill array at `+0x40`, indexed diplomacy 0, espionage 1, ship design 2,
  troop training 3, facility design 4, combat 5, leadership 6, loyalty 7.

`FUN_00403460(pool, req_flags, req_caps, forbid_flags, forbid_caps, skill,
min, max, order)` returns the records that meet all of these
(`FUN_00403460.c`):
- all of `req_flags` and none of `forbid_flags` in `+0x30`;
- all of `req_caps` and none of `forbid_caps` in `+0x34`;
- `min <= skill[+0x40] <= max`.

`FUN_004035d0` adds one argument, `extra`, and passes a record when
`extra < record.+0x2c` or the record has `0x800000`. Every call passes 0.

**Ranking (re-read 2026-09-30).** Each hit goes into a sorted list
(`FUN_0041be80`, order flag at `+0x1c`). `FUN_005f59f0` walks from the head
and inserts the new entry before the first one the comparator
`FUN_0041c250` puts it ahead of:
- order 1 (every skill query): the lower value goes first;
- order 2 (the nearness query `FUN_00403750`): the higher value goes first;
- equal values: a coin flip, `FUN_0041cd80(10) < 5`.

`FUN_0041c230` then numbers the entries 1, 2, 3… into `+0x14`. The score is
that position, not the skill.

**Picking (`FUN_004357b0`).** The selector merges its lists
(`FUN_00435790`). For each entry of the first list, `FUN_00435980` adds its
position in every other list; an entry missing from any list leaves the
first list. The highest total wins, the earliest on a tie (strict `<`), and
leaves the first list. So the best candidate has the highest skill, and a
two-query selector takes only candidates that pass both.

**The nearness query `FUN_00403750`** keys each record at a system by the
squared difference (`FUN_0041b7b0`) of the two system records' `+0x34`
halves. It joins only when the planner's `+0x38` is a system, and state 6
(`FUN_004bd0a0`) sets `+0x38` after the first cycle's selectors have run.
The per-system record's constructor builds `+0x34` as a DatId
(`FUN_004762b0`), yet a tracer read it as packed coordinates; which it holds
is unresolved.

**Team selectors** (every query `max` 10000, order 1; verified at each call
site):

| Kind | Selector | Size | Query | req flags | req caps | forbid flags | forbid caps | skill ≥ |
|------|----------|------|-------|-----------|----------|--------------|-------------|---------|
| Diplomacy | `004815a0` | 1 | `004035d0` | `0x2` | `0x1` | `0x7404051` | 0 | diplomacy 69 |
| Espionage 6 | `00480f30` | 1 | `00403460` | `0x10800002` | `0x8` | `0x7400051` | 0 | espionage 51 |
| Espionage | `004808c0` | 2 | `00403460` | `0x10800002` | `0x8` | `0x7400051` | 0 | espionage 51 |
| Recruitment | `0047db40` | 1 | `004035d0` | `0x2` | `0x400` | `0x7400051` | 0 | leadership 80 |
| Incite | `0047ff90` | 1 | `004035d0` | `0x2` | `0x100` | `0x4f400051` | 0 | leadership 49 |
| Subdue | `0047cee0` | 1 | `004035d0` | `0x2` | `0x20` | `0x7400051` | 0 | leadership 49 |
| Rescue | `0047e270` | 2 | `004035d0` | `0x2` | `0x2` | `0x4f400051` | `0x7000000` | combat 51 |
| Abduction | `00483290` | 2 | `004035d0` | `0x2` | `0x10` | `0x0f400051` | `0x7000000` | combat 51 |
| Abduction 4, Assassination | `00482ce0` | 2 | `00403460` | `0x10800002` | `+0x7c` | `0x47400051` | 0 | combat 51 |
| Sabotage | `0047d520` | 2 | `00403460` ×2 | `0x800002` | `0x4` | `0x4f400051` | 0 | combat 51 and espionage 51 |
| DS Sabotage | `00482180` | 2 | `004035d0` ×2 | `0x2` | `0x80` | `0x47400051` | 0 | combat 51 and espionage 51 |

The selectors for Espionage, Abduction 4/Assassination, and Sabotage also
join the nearness query when `+0x38` is a system. `+0x7c` is `0x40` for
Assassination and `0x10` for Abduction 4.

**Decoy selectors** all query `FUN_00403460(pool, 0x10800002, cap_bit,
0x7400051, 0, 1 espionage, 51, 10000, 1)`, plus the nearness query. AI
decoys are therefore unassigned special forces of the side with espionage
51 or more. Their caps:
- Espionage (`00480aa0`), Incite (`00480110`), Assassination/Abduction 4
  (`00482ec0`), and Sabotage (`0047d720`): `(rec.+0xbc + 2) / 2`, where `rec`
  is the per-system record of the planner's `+0x34` (`FUN_00403d30` on
  context `+0x2c`). Its `+0xbc` is the mission kind that system's posture
  recommends (`FUN_00478000` through posture slot 6, `FUN_004b7150`, shared
  by all six posture classes), or 0. The cap is 1 with no recommendation
  and 42 to 53 with one, so every eligible decoy goes.
- Rescue (`0047e3f0`), Abduction (`00483410`), and DS Sabotage
  (`00482330`): planner slot `+0x44`, `FUN_004047d0`, which returns 4.
- Diplomacy, Recruitment, Subdue, Research, Recon, and Espionage 6 take
  none (`FUN_0047f500` or `FUN_00481100`, or no selector).

`FUN_004b7150` recommends, in order: Recon `0x54` (few Recon missions and
sector bits `0x18`), Espionage `0x52`, Sabotage `0x69` (sector bit 8), and
Abduction/Assassination `0x62`/`0x63` subtype 4 (sector bits `0x40008`), each
gated by the system record's per-kind counters `+0xa0..+0xac`.

**Adding a member** (`FUN_0047b610` team, `FUN_0047b730` decoys) clears the
record's `0x800000`, so a query that requires it (`0x800002`, `0x10800002`)
no longer finds that record. A team member is therefore never a decoy.

`FUN_0047ba90` accepts a candidate only when it meets all of these:
- the side's record list (context `+0x8c`) holds it;
- when its container is a system, its record's `+0x30` bit `0x20` is clear;
- that system's record `+0x28` bit `0x2` is clear. `+0x28` of a system
  record (context `+0x2c`, class `FUN_00433b10`) is a flag word, and
  `FUN_00433e40` sets bit 2 from the system object's `+0x88` bit `0x20`
  (hyp: a contested system).

The order (`FUN_0047b360`) keeps only members with none of `0x74000f1`: a
selected member on a mission (`0x80`) or en route (`0x20`) is dropped.

The state machine advances every state unconditionally, so a planner whose
decoy list is short still emits. State 7 moves every member to the target,
and state 8 emits once the team and decoys are all there.

## The AI records (re-read 2026-09-28, 2026-09-30)

`FUN_00401d20` builds a record (vtable `0x006584a0`) for one live object,
and `FUN_00402230` refreshes it. The record's `+0x3c` is the side
controller (`FUN_004178f0` stores a self-pointer at `+0x9c`), whose `+4` is
a flag word (`FUN_005f4960`).

**Kind and side flags at `+0x30`** (set by the constructor):

- `0x800000` is set, and cleared while a planner holds the object.
- `0x10000000` marks a special force (DatId family `0x3c..0x3f`).
- `0x20000000` marks a character.
- `0x2` means the object is on the record's own side; `0x4` means the other
  side. A character prisoner counts on the side that holds it.
- `0x40000000` marks the four leaders (`FUN_004024d0`: `0x30000240`,
  `0x32000242`, `0x34000280`, `0x35000281`) while the controller's bit `0x1` is
  clear; `0x400` additionally marks `0x34000280`. The bit's only writer is
  `FUN_004182a0`, from a game setting (`FUN_004fce50`), untraced.
- `0x8` marks the six main characters (`FUN_004025f0`). Any other character
  gets capability bits in the record's `+0x34` word: `0x1000000`, `0x2000000`,
  `0x4000000` when its class record may be an admiral, commander, or general
  (`+0xa8`, `+0xac`, `+0xb0`; `FUN_004ed1c0`, `FUN_004ed1e0`, `FUN_004ed200`).
- Special forces also get a class bit in `+0x34` (`FUN_00402e80`): classes
  `0x3c000001`/`5` → `0x10000000`, `2`/`6` → `0x20000000`,
  `3`/`7` → `0x40000000`, `4`/`8` → `0x80000000`.

**State flags at `+0x30`**, recomputed by `FUN_00402230` after masking with
`0x7080440e`:

| Bit | Meaning | Source |
|-----|---------|--------|
| `0x80` | on a mission | role `+0x78` bit 7 |
| `0x400000` | on a mandatory mission | role bit 9 |
| `0x20` | en route | `+0x50` bit 4 |
| `0x1` | `+0x50` bit 2 clear (a character the side has not recruited) | |
| `0x40` | prisoner | `+0xac` bit 0 |
| `0x10` | injured | `+0x94` != 0 |
| `0x4000000`, `0x1000000`, `0x2000000` | officer rank 1, 2, 3 | `+0x96` |
| `0x8000000` | reserved (see below) | |
| `0x100` | Force level 1..99, not Luke or Vader | `+0x8c`, `FUN_00402cf0` |
| `0x200` | Force level 100 or more, Luke or Vader | `+0x8c`, `FUN_00402cf0` |

Bits `0x40`, `0x10` and the rank bits are set for characters only.

`0x8000000` marks the object as reserved:
- when it is legal for a Research record (capability `0x800`, `0x1000`,
  `0x2000`) and the controller's bit `0x2000000`, `0x4000000`, or
  `0x8000000` is clear. Those bits start clear and are set by
  `FUN_004198a0`, `FUN_00419870`, and `FUN_004197b0` when a callback's
  argument matches the controller's research targets at `+0x140..+0x148`;
  when that fires is untraced. The shipped Research records take every
  character and no special force.
- when it is Diplomacy-capable and its diplomacy is above 79.

**Skills at `+0x40`..`+0x5c`** are the eight effective skills, read through
slots `+0x1dc`..`+0x1f8`.

**Capability bits at `+0x34`** are set by `FUN_00402720`, which builds a
dummy order `0x240` holding only this object. `FUN_004f52c0` ->
`FUN_005420d0` then lists every non-hidden MISSNSD record that passes its
class's legality check (`FUN_0054c440`), and each legal record sets one bit:

| Bit | Record |
|-----|--------|
| 0 | Diplomacy `0x51000010` |
| 1 | Rescue `0x61000011` |
| 2 | Sabotage `0x69000012` |
| 3 | Espionage `0x52000013` |
| 4 | Abduction `0x62000017` |
| 5 | Subdue `0x57000080` |
| 6 | Assassination `0x63000081` |
| 7 | DS Sabotage `0x6a000041` |
| 8 | Incite `0x56000040` |
| 9 | Recon `0x54000015` |
| 10 | Recruitment `0x55000016` |
| 11 | Research `0x53000020` |
| 12 | Research `0x53000022` |
| 13 | Research `0x53000021` |

So the AI uses the same legality rules as mission creation. The probe names
no target (context `+0xc` = 0), so only the member check below decides.

### Legality (`FUN_0054c440`)

`FUN_0054cb70` builds a rule table with one entry per MISSNSD record. It
constructs each class and calls its slot `+0x1bc`, which returns two
functions.

- **The shared resolver** is `FUN_00592aa0` for every agent class. It
  finds the target's location.
- **The per-class validator** differs by kind:

| Kind | Validator |
|------|-----------|
| Subdue | `569b10` |
| Sabotage | `56a110` |
| Rescue | `56adb0` |
| Recruitment | `56b370` |
| Incite | `571950` |
| Espionage | `573010` |
| Diplomacy | `573ee0` |
| DS Sabotage | `5744c0` |
| Assassination | `576540` |
| Abduction | `576dd0` |

**The member check (re-read 2026-09-30).** `FUN_005830a0` walks the team,
decoy, and captured lists and builds a mask with `FUN_00582fb0`: a special
force ORs its class record `+0x98` (SPECFCSD's mission mask, `FUN_00503b40`)
into word 0, a character ORs `0x10000` (`FUN_004ed260`) into word 1, and each
member marks its side. `FUN_00583320`, called from `FUN_00523450`, then
requires:
- word 0 a subset of the record's `+0x48` (file `0x20`);
- word 1 a subset of `+0x4c` (file `0x24`);
- one side only (both is status `0x14`, none `0x16`), and that side's flag,
  `+0x40` (Alliance, file `0x18`) or `+0x44` (Empire, file `0x1c`).

A failure is message `0x40`/`0x01` with no end code, so it refuses creation
but never ends a running mission. No legality function reads a skill or a
rank.

| Record | `+0x48` | `+0x4c` | Alliance | Empire |
|--------|---------|---------|----------|--------|
| Diplomacy `0x51000010` | 0 | `0x10000` | 1 | 1 |
| Rescue `0x61000011` | `0x802` | `0x10000` | 1 | 1 |
| Sabotage `0x69000012` | `0x402` | `0x10000` | 1 | 1 |
| Espionage `0x52000013` | `0x208` | `0x10000` | 1 | 1 |
| Abduction `0x62000017` | `0x802` | `0x10000` | 1 | 1 |
| Subdue `0x57000080` | `0x401` | `0x10000` | 1 | 1 |
| Assassination `0x63000081` | `0x800` | `0x10000` | 0 | 1 |
| DS Sabotage `0x6a000041` | 2 | `0x10000` | 1 | 0 |
| Incite `0x56000040` | `0x401` | `0x10000` | 1 | 1 |
| Recon `0x54000015` | `0x104` | 0 | 1 | 1 |
| Recruitment `0x55000016` | 0 | `0x10000` | 1 | 1 |
| Research `0x530000(20, 21, 22)` | 0 | `0x10000` | 1 | 1 |

Diplomacy's validator (`FUN_00573ee0`) runs the common agent rules
`FUN_005868c0`. It then fails with message `0x40`/`0x33` and end code `0xf`
when the side's support at the target (`FUN_00507270`) is 100. A running
Diplomacy mission therefore repeats until the side's support there is
full.

`FUN_005868c0` fails with `0x40`/`0x24` (code -1) when the target is not a
system (`0x90..0x98`). It fails with `0x40`/`0x20` (code 6) when the
target's `+0x88` bit 2 and the record's `+0x88`/`+0x8c` do not match. It
chains to `FUN_00592600`, whose target checks run only with a target
(context `+0xc`).

## Ported (F-019 phase 5a)

`crates/rebellion-core/src/mission_planning.rs` ports the records, the
queries, the ranking, the pick, and the selectors above for the port's ten
kinds, and `MissionState::dispatch_guarded` refuses members the record does
not admit (`MissionRefusal::MembersNotAllowed`). The port departs from the
original as follows:
- port: the planners run in one cycle, so the nearness query never joins
  and the order keeps only the members standing with its first (the
  original gathers them in state 7);
- port: a plan with no team keeps no decoys, and a member one plan sends is
  not taken by the next in the same evaluation;
- port: the tie draws come from a stream seeded by the day and the side;
- hyp: the posture that sets `+0xbc` is not ported, so the four posture-cap
  kinds take one decoy;
- hyp: the controller's leader and research bits are read as clear, their
  constructor values, so the four leaders stay off Incite, Rescue, Sabotage,
  Assassination, and DS Sabotage teams, and every character legal for
  Research is reserved from Incite, Rescue, Sabotage, and Abduction teams;
- port: Espionage takes the class for subtypes other than 6 and Abduction
  its default class (side `+0x31c` is untraced); the mission kind and target
  are the port's (side `+0x318`).

## Still open

- What sets side `+0x318`/`+0x31c`, the strategic choice of mission.
- The posture (`FUN_00478000`, `FUN_004b7150`) and its sector bits.
- When the controller's research bits (`FUN_004198a0`, `FUN_00419870`,
  `FUN_004197b0`) and leader bit (`FUN_004182a0`) are set.
- Whether the per-system record's `+0x34` is a DatId or coordinates.

## Supporting decompiles

`FUN_0046c3c0`, `FUN_0046c850`, `FUN_0046c880`, `FUN_00402720`,
`FUN_004f4990`, `FUN_004f4a00`, `FUN_004f4b60`, `FUN_004f5380`,
`FUN_004f5cd0`, `FUN_0051f4b0`, `FUN_0051f8f0`, `FUN_0051fb70`,
`FUN_0051fb80`, `FUN_0051fbb0`, `FUN_0042f830`, `FUN_004bced0`,
`FUN_004bd0a0`, `FUN_004bd570`, `FUN_0047b110`, `FUN_0047b250`,
`FUN_0047b360`, `FUN_0047b610`, `FUN_0047b730`, `FUN_0047b9e0`,
`FUN_0047ba90`, `FUN_00481100`, `FUN_0047f500`, `FUN_0047d520`,
`FUN_0047d720`, `FUN_00480110`, `FUN_00480aa0`, `FUN_00482ec0`,
`FUN_0047e3f0`, `FUN_00482330`, `FUN_00483410`, `FUN_00403460`,
`FUN_00403750`, `FUN_00403d30`, `FUN_0041be80`, `FUN_004d2490`,
`FUN_004d2580`, `FUN_004d27b0`, `FUN_0054d360`, `FUN_004357b0`,
`FUN_00435790`, `FUN_00435980`, `FUN_005f59f0`, `FUN_0041c070`,
`FUN_0041c230`, `FUN_0041c250`, `FUN_0041c360`, `FUN_0041cd80`,
`FUN_0041b7b0`, `FUN_004047d0`, `FUN_00401d20`, `FUN_00402230`,
`FUN_004024d0`, `FUN_004025f0`, `FUN_00402e80`, `FUN_004ed1c0`,
`FUN_004ed1e0`, `FUN_004ed200`, `FUN_005f4960`, `FUN_00478000`,
`FUN_004b7150`, `FUN_004762b0`, `FUN_00419870`, `FUN_005830a0`,
`FUN_00582fb0`, `FUN_00583320`, `FUN_00523450`.
