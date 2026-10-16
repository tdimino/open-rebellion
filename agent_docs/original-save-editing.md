---
title: "Editing original-game saves"
description: "How to edit our own original-game saves safely to stage a state the original cannot reach on demand (a damaged hyperdrive, a Death Star), and how to accept the edit in the original"
category: "agent-docs"
created: 2026-10-10
updated: 2026-10-13
tags: [original-game, saves, parity, death-star, hyperdrive]
---

# Editing original-game saves

Some behaviour can only be checked in the original once the game reaches a
state that normal play does not produce on demand: a ship whose hyperdrive
is out before a Withdraw, or a finished Death Star with an escort. We get
there by editing a copy of one of **our own** saves (or the one David Adams gave us)
(`tools/original-capture/saves/`), loading it on the Mac Mini capture VM
(`tools/original-capture/README.md`, `vm-game.sh`), and accepting the edit
only after the original loads, plays and re-saves it.

Status:
- **Hyperdrive (recipe 1):** loads and plays. The Withdraw result varies from
  run to run, so the rule is not settled.
- **Death Star (recipe 2):** works for both sides through a real
  manufacturing order (route 1). Converting a ship in place (route 2) crashes
  the game when its fleet moves.
- **Battle staging (recipe 3):** the copies load and show their damage.
- **Squadron size (recipe 4):** loads, re-saves and reloads with the edited
  size.
- **Player side:** cannot be changed by editing.

"Edits that worked" lists each successful edit byte for byte.

## Rules

- Edit a copy, never a library save. Pin the source's SHA-256 and size in
  the script, and refuse to run on anything else.
- Change bytes in place only: the same file length, no inserted or removed
  records. Each named record's first word is often its own file offset, and
  objects refer to each other by those keys, so a length change corrupts
  everything after it.
- Fail closed: assert every field you expect before writing, write to a new
  path (never over the source or an existing file), re-read the output and
  prove that only the intended bytes changed.
- Never commit an edited save until it passes the acceptance gates. When it
  does, add it to `saves/INDEX.md` with its source, its edit and its
  SHA-256 (our own saves are the one game-data exception in `CLAUDE.md`).
- Scripts are stdlib Python. Keep any that worked under
  `tools/original-capture/`, never in `/tmp`.

## Edits that worked

Each edit changes u32 words in place in a copy. The edited copies stay
local (`.artifacts/`, ignored). The library saves named here are the
original's own re-saves after it loaded and played the edit.

| Edit | Source → copy (SHA-256) | Exactly what changed | Result in the original | Library save |
|---|---|---|---|---|
| Hyperdrive out (`save_edit.py hyperdrive`) | `empire-day2-start.sav` `d9da6a3b…` → `7a4c4f1a…` | The ISD's packed damage word (payload word 20 of the `0x81` copy, word 10 of the `0xa1` copy) set to `0x00110000`, one primary and one backup hyperdrive hit. Two bytes: `0x17813`, `0x48f42` | Status reads Hyperdrive Rating 0:80. The fleet will not jump, but a fleet already in hyperspace arrives | `empire-day30-yaga-alert-isd-hyperdrive.sav` |
| Empire Death Star (route 1) | `ds-b.sav` `1e6db63f…` → `ds-b583.sav` `43977b84…` | The Death Star order's progress word, 2 → 583 (cost 584), in both graph copies. Found as the u32 8 bytes before `88000018 48020000 48020000` | Finished "Death Star 1" into Fleet 6 at Coruscant within 11 days. It moves, and it fights | `empire-huge-deathstar-built.sav`, `empire-day660-deathstar-pre-alert.sav` |
| Imperial Death Star in an Alliance game (route 1, converted order) | `fwd1.sav` (Alliance, late game) → `fwd1-ds.sav` `7ecf1706…` | Turned the Empire AI's Star Destroyer order into a Death Star order: 21 words (see "Route 1 in an Alliance game") | Completed as "Devastator" at Coruscant. The Alliance can Take Command at Coruscant, but the Death Star's fleet did not join the battle | `alliance-day731-imperial-deathstar-built.sav` |
| Battle staging at the day-30 alert (`save_edit.py damage`, `subsystems`, `engines`, `fragile`, `stranded`) | `empire-day30-yaga-alert-control.sav` `8a7a9923…` → recipe 3 table | Hull damage and the packed word on the named ships, in every graph copy | Each loads and shows the staged damage; used for tactical A0 captures | none (staging only) |
| Squadron size (`save_edit.py squadron`) | `alliance-deathstar-at-coruscant.sav` `5825d980…` → `e0953abc…` | Fleet 2's first X-wing squadron (id `0x17b8`): its `+0x60` damage word 0 → 7 in the `0x41` and `0x51` copies. Two bytes: `0x3a128`, `0x9e22f` | Status reads Squadron Size 5:12; shield and laser scale with it (25:60, 40:96). The original's re-save keeps 7 in both copies, and a restart and reload still read 5:12 | none (experiment) |

What failed, so it is not repeated:
- **Recipe 1, first try:** wrote the wrong word, word 15. It loaded but
  changed nothing visible.
- **Route 2:** converting the ISD in place (family and class, four bytes)
  crashes the game when the fleet moves.
- **Side swap:** header word 5 switches only the strategic shell. Changing
  either controller's side word crashes on load.

## File layout

```text
u16   name length, then the slot name (ASCII, no terminator)
u32 × 6+  header words; empire-day2-start.sav: 1, 0, 1, 2, 2, 0, then more
          (saves/INDEX.md's three-word summary is short; do not assume the
          records start after the side word)
records   one per object, nested by container
```

A named record:

| Offset | Type | Field |
|---|---|---|
| `+0x00` | u32 | object key: the record's own file offset, or 0 |
| `+0x04` | u32 | family (`0x08` fleet, `0x10` regiment, `0x14` capital ship, `0x18` Death Star, `0x1c` fighter squadron) |
| `+0x08` | u32 | instance id |
| `+0x0c` | u32 | graph key (see below) |
| `+0x10` | u32 | class (`DatId` low byte: ISD `0x85`, Carrack `0x83`, Death Star `0x88`) |
| `+0x14` | u32 | 1 for a named record |
| `+0x18` | u16 | name length |
| `+0x1a` | bytes | name, then the family's payload |

An object appears once per graph that holds it, under the same instance id.
In `empire-day2-start.sav` the ISD appears in `0x80000081` (full payload)
and `0x800000a1` (shorter). Edit every graph copy that a recipe touches.

Graph key bits (low word):

| Bits | Meaning |
|---|---|
| 6-7 | owner: 1 Alliance (`0x40`), 2 Empire (`0x80`), 3 neither (`0xc0`) |
| 4-5 | copy: 0 master, `0x10` the Alliance's view, `0x20` the Empire's view |
| 8 (`0x100`) | an entry in the destroyed-object list |

The owner bits match `obj+0x24 >> 6 & 3` (`FUN_00433e40`); the serializer
was not traced, so that identity is inferred. Bits 0-3 are untraced.

Payload of a master copy (facilities and units checked):

| Word | Field |
|---|---|
| 0 | `+0x50` state (`object-state-flags.md`): `0x00030047` usable, `0x0003000e` destroyed (bit 3), `0x142` in production |
| 1 | producer id (`0xa0…`) while in production |
| 9 | location: system DatId (`0x90…`/`0x92…`), parent fleet (`0x08…`) or ship (`0x14…`); `2` while queued |

A queued object's destination is in its producer's queue entry: the object's
DatId, then the destination system twice, then the name. A scrapped
facility stays in the save as a destroyed-list record (graph `0x100`,
state `0x0e`) at its last system.

`empire-day2-start.sav` (SHA-256 `d9da6a3b…7cbc79`, 410,763 bytes):

| Object | `0x81` record | `0xa1` record |
|---|---|---|
| Fleet 1 (`0x08`, id `0x77e`) | `0x1772d` | `0x48e98` |
| Imperial Star Destroyer 1 (`0x14`, id `0x77f`, class `0x85`) | `0x1778e` | `0x48ee5` |
| Carrack Light Cruiser 2 (`0x14`, id `0x7aa`, class `0x83`) | `0x17c05` | `0x4921c` |

The ISD's payload starts at `0x177c1`. Word 9 is `0x0800077e`, its parent
(family `0x08`, Fleet 1).

The player's side cannot be changed by editing. The controller section
(`FUN_00435db0`) holds two slots, each a class tag (1 player, 2 or 3 AI) and
then a payload whose first u32 is the side; the tag-1 slot becomes the
player on load (`FUN_00436690`). In `empire-day2-start.sav` the player is
slot 1 at `0x64132` (tag 1, side 2), the AI slot 0 at `0x5564D`. Header
word 5 switches only the strategic shell. Setting the player's side word
to 1 crashes the game on load, and so does swapping both sides, because each
payload carries its own faction's state. To play a state as the other side,
start from that side's save.

## Recipe 1: a ship with its hyperdrive out

The strategic capital ship keeps hull damage at runtime `+0x60` and packed
4-bit damage counters at `+0x64`. `FUN_0040cfc0` writes them back after a
battle in the order shield, weapons, tractor, engines, primary hyperdrive
(`FUN_00501780`, bits 16–19), backup hyperdrive (`FUN_00501820`, bits
20–23). A ship with a backup drive needs a hit on both.

The capital ship serializer (`FUN_00501f00`, the top of the vtable `+0x14`
chain traced in `ghidra/notes/capital-ship-save-layout.md`) streams hull
damage `+0x60` and then the packed word `+0x64` as the **last two payload
words**: words 19 and 20 of the full (`0x81`) copy, words 9 and 10 of the
short (`0xa1`) copy. Payload word 0 is the `+0x50` flags (`0x00030047` here;
bit `0x200` is "Ship Damaged"). For the ISD: `+0x64` at `0x17811` and
`0x48f40`. `save_edit.py hyperdrive` writes `0x00110000` (one primary and
one backup hit) into both; two bytes change (`0x17813`, `0x48f42`).

**Verified in the original (2026-10-11).** The copy (SHA-256
`7a4c4f1a…7fedb8`) loads, and the ISD's Status reads "Hyperdrive Rating:
0:80" (class 80 minus `FUN_005011f0`'s 80, and the damaged rating is 0).
"Ship Damaged" still reads No, since the flag is separate and left clear.

A first attempt wrote word 15 instead, on a field-order guess. It loaded and
survived a re-save but showed 80:80: word 15 is some other field. Map words
from the serializer, never from field order.

**Played on (2026-10-11).** A fleet whose ISD has its hyperdrive out does not
move: dragged to Yaga Minor, Fleet 1 stayed at Coruscant for six days with no
message, while the unedited save left at once. A fleet already in hyperspace
keeps going: the same edit applied to a save taken in transit (unedited
`empire-day2-start.sav`, Fleet 1 ordered to Yaga Minor, saved on day 5;
payload word 2 then holds the destination) still arrived, by day 13 where the
unedited fleet arrives by about day 11, and showed the red X there. On day 31
an Alliance fleet came to break the blockade. Retreat at the Battle Alert gave
"The Imperial fleet is victorious … The Alliance fleet has withdrawn" with
both Imperial ships kept, and so did the unedited control, so that alert
cannot test the Withdraw rule. The rule needs Take Command, then Withdraw
inside the battle. `vm-game.sh wait center` stops at the alert, 336.5 s from the
load before the 2026-10-11 guest-tools install and 360–363 s in three runs
after it (one with the WebDAV service stopped; the cause is open). Give the
wait a 420 s timeout, or skip it: `saves/` holds both states re-saved on day
30 (`empire-day30-yaga-alert-control.sav`,
`empire-day30-yaga-alert-isd-hyperdrive.sav`), and either reaches the alert
within 12 s of the load.

**Withdraw inside the battle (2026-10-11).** Both runs loaded the day-5
transit save, waited for the day-31 alert, then Take Command, Battle Options
(the 640×480 button at (616,318)), Withdraw, the check mark, and unpause.
The unedited control ended "The Imperial fleet is defeated … The Imperial
fleet has withdrawn to Ghorman", and Fleet 1 left for Ghorman. With the
ISD's drive out, the battle ended "The Imperial fleet is victorious … The
Alliance fleet has withdrawn". Fleet 1 stayed at Yaga Minor holding only
Carrack Light Cruiser 2 (Hyperdrive Rating 60:60, hull 1000:1000), and the
re-saved file has no record of the ISD (instance `0x77f`) in any graph: the
ISD was destroyed, not moved. So the stranded ship did not leave, as
`FUN_005a0240` says. The fleet stayed rather than leaving, and the Alliance
withdrew in this run while it won in the control. Each side ran once, so it
is not settled whether the stranded ship's loss and the fleet staying follow
from the rule or from that battle's course. Evidence:
`.artifacts/original-checks/2026-10-11-save-edits/hd-withdraw-*` and
`control-withdraw-*` (ignored).

Three more runs of the edited state, from
`empire-day30-yaga-alert-isd-hyperdrive.sav` with the same steps
(2026-10-11): one ended "The Imperial fleet is defeated … withdrawn to
Ghorman", as the control did, and two ended "The Imperial fleet is
victorious … The Alliance fleet has withdrawn". The result of a Withdraw
varies from run to run with the stranded ship aboard, so no single run
settles the rule; compare several. Evidence:
`.artifacts/original-checks/2026-10-11-tactical-panel/10-*`, `11-*` and
`2026-10-11-vm-verbs/05-results.png` (ignored).

Accept: load the copy, open the ISD's Status at once (Very Slow, not Pause:
see "Driving the original" below) and read Hyperdrive Rating below 80. Then
start a battle, Withdraw, and check that the Carrack leaves while the ISD
stays. Finish the battle, save, reload and see whether the damage persists.

## Recipe 2: a Death Star

The strategic Death Star is an ordinary object of family `0x18`, class
`0x88` (`DatId 0x18000088`; factory registry `FUN_0051ebf0`), held directly
by a fleet like a capital ship. It is not family `0x73`, the tactical
singleton `DAT_006bc470`, or the shield facility `0x25000004`. Battle
construction turns it into the tactical Death Star (`FUN_005a66a0`,
`FUN_005ba420`).

Two routes, safest first:

1. **Finish a real construction.** Order a Death Star in the original and
   save (A). Let one manufacturing cycle pass and save (B). Diff A and B as
   aligned u32 words to find the manager's progress (runtime `+0x5c`) and
   cost (`+0x68`). Prove the progress field over several more saves (rising
   by one cycle at a time, stopping at the cost). Then set progress to
   cost − 1 in a copy of A and let the original complete it. This keeps
   every relationship the original builds. It works; see "Route 1 in the
   original" below.
2. **Convert Fleet 1's ISD in place.** In both copies, change the family
   at `+0x04` from `0x14` to `0x18` and the class at `+0x10` from `0x85` to
   `0x88` (`0x17792`, `0x1779e`, `0x48ee9`, `0x48ef5`; four bytes). Leave the
   name. Both classes allocate a 0x68-byte object, but the Death Star's
   loader may read a different payload, so this is experimental.

**Route 2 in the original (2026-10-11).** The copy (SHA-256
`d5b71411…bdb61`) loads. Fleet 1 lists a Death Star icon under the old name
"Imperial Star Destroyer 1", still carrying its six TIE squadrons, three
regiments and one officer. Its Status (the Capital Ship Status window) reads
Class: Death Star, Maintenance Cost 600, capacity 24 fighter squadrons and 18
regiments, Hull 25000:25000, Hyperdrive 80:80. Saved, the game quit,
restarted and reloaded, the fleet still holds it, and the re-saved file
writes the record back as family `0x18`, class `0x88`, with the same payload
words in both graphs, so the Death Star shares the capital ship's payload
format. Still open: a battle against it (no Alliance fleet comes to
Coruscant on demand) and whether anything else that keys on the Death Star
(the construction manager, the planet-destruction order) treats a converted
one as real.

**Route 2 fails the move gate (2026-10-11).** Ordered to move (Fleet 1
dragged from Coruscant to Yaga Minor on the day-6 reload), the converted
fleet crashes the original on Windows: the game closes to the desktop. A
Worker saw the same crash under Wine for both a drag and the Move context
menu, so this is the conversion, not Wine. The converted object still
loads, simulates a battle (under Wine, from a save converted after Fleet 1
had already reached Yaga Minor: the Alliance fleet destroyed), saves and
reloads as Class: Death Star, so whatever movement reads from a Death Star
is missing from a converted ISD. Use route 1 for anything that moves the
fleet. Under Wine, Take Command leaves a black tactical screen, so tactical
checks need the VM.

**Route 1 in the original.** A new Empire game (Huge, Easy)
ordered a Death Star at Coruscant on day 115 and saved (A), then saved again
8 days later (B). The ship records themselves (family `0x18`, class `0x88`,
named "Death Star 1", three graph copies) did not change. The progress lives
in the manufacturing order. Each of its two graph copies holds, in order,
`1`, progress, `0`, `0x18000088`, `584`, `584`, `0`, `0x18000df5` (the
ship's key), then the name "Death Star". Progress went from 0 in A to 2 in B
(one refined unit per yard cycle, with one Orbital Shipyard). Find it in B
by searching for the bytes `88000018 48020000 48020000`; progress is the u32
8 bytes before. Setting it to 583 in both copies (equal length, nothing else
changed) and loading the copy finished the Death Star within 11 days. The
order record was gone, and the ship's status word after its name went from
`0x42` to `0x30147`. "Death Star 1" then sits in a new fleet (Fleet 6) at
Coruscant. Unlike route 2, it moves: it flew to Yavin and back to Coruscant
(about 80 days a hop on a Huge map, hyperdrive 80), and fought a battle
there. The edited copy and the saves after it are in
`.artifacts/original-checks/s4/` (`ds-b583.sav`, `ds-built.sav`,
`ds-pre-alert.sav`). The last two are in the library as
`empire-huge-deathstar-built.sav` and `empire-day660-deathstar-pre-alert.sav`.

**Route 1 in an Alliance game.** The Alliance player cannot order a Death
Star, so an order the Empire AI already has was converted instead.
- **Source.** `fwd1.sav`, a late-game Alliance save (the Death Star was finished by day 731), holds an Imperial order
  for an Imperial Star Destroyer (DatId `0x14000085`, ship key `0x14000d36`)
  at Coruscant.
- **Template.** The words were copied from the Empire's real Death Star
  order in `ds-a.sav`.
- **The edit.** In `fwd1-ds.sav` (`7ecf1706…`), 21 u32 words changed and the
  length did not:
  - in the order record, the DatId became `0x18000088`, the ship key
    `0x18000d36`, the cost 584 and the progress 583;
  - every other reference to the key was updated the same way, including two
    AI planning records;
  - the placeholder ship record's family went from `0x14` to `0x18`, and its
    class from `0x85` to `0x88`.
- **Offsets.** `0x27534`, `0x27540`, `0x27f10`, `0x27f18`, `0x27f20`,
  `0x27f24`, `0x27f28`, `0x27f34`, `0x4d71c`, `0x4d728`, `0x69c28`,
  `0x69c34`, `0x6a2f4`, `0x6a2fc`, `0x6a300`, `0x6a304`, `0x6a310`,
  `0x87604`, `0x8760c`, `0x87610`, `0x878ec`.
- **Names.** Left as they were; a different length would shift every later
  key.
- **Result.** The original finished the order as the Death Star
  "Devastator" in Imperial Fleet 325 at Coruscant (library:
  `alliance-day731-imperial-deathstar-built.sav`).
- **Still open.** An Alliance attack on Coruscant drew only Fleet 302 into the
  battle, so getting the Death Star onto the field is unsolved.

Never insert or clone a record: that needs every container count, every
later object key and every reference updated in both graphs.

Accept: the slot loads; Fleet 1 shows a Death Star and no ISD; the fleet can
be selected, moved and inspected; save to a new slot, quit, restart and
reload that one; bring an enemy fleet in, and the battle shows the Death
Star as its own object and returns to the galaxy; save and reload once
more.

## Recipe 3: battle states at the day-30 alert

The tactical A0 cells for damage, subsystems and results need ships that
enter the battle already hurt. `save_edit.py` stages them from
`empire-day30-yaga-alert-control.sav` (SHA-256 `8a7a9923…975636`, 421,096
bytes), which reaches the Yaga Minor alert within seconds of the load.
Each Imperial ship there has three graph copies (`0x80000081` full,
`0x80000091` and `0x800000a1` short); the Alliance Bulk Cruiser has two
(`0x80000041` full, `0x80000051` short). Hull damage and the packed word are
words 19–20 of a full copy and 9–10 of a short one, as in recipe 1:

| Ship | id | class | records | flags (word 0) | hull (CAPSHPSD) |
|---|---|---|---|---|---|
| Imperial Star Destroyer 1 | `0x77f` | `0x85` | `0x1ae28`, `0x32c96`, `0x4bbed` | `0x000301c7` | 2750 |
| Carrack Light Cruiser 2 | `0x7aa` | `0x83` | `0x1b29f`, `0x32f41`, `0x4bf24` | `0x000301c7` | 1000 |
| Bulk Cruiser 3 | `0x796` | `0x41` | `0x1b385`, `0x32feb` | `0x00030056` | 1200 |

The packed word's nibbles, low first: shield, weapons, tractor, engines,
primary hyperdrive, backup hyperdrive (`FUN_0040cfc0` through
`FUN_00501510`…`FUN_00501820`). The script asserts each record's family,
id, class, name and flags, and zero hull and packed words, before writing.

| Recipe | Writes (hull, packed) | Output SHA-256 | Seen in the original (2026-10-12) |
|---|---|---|---|
| `damage` | ISD (2200, `0x1121`), Carrack (400, 0) | `db514c7f…cf2032` | ISD hull bar about a fifth, Carrack about three-fifths; Battle Results lists both as Operational |
| `subsystems` | ISD (0, `0x2222`) | `75f52eac…364bde` | shield, weapons and tractor icons show two hits; two engine hits leave its icon unchanged |
| `engines` | ISD (0, `0x4000`) | `ebe9dd3b…cf18c5` | four engine hits (the limit) change the engine icon |
| `fragile` | Bulk Cruiser (1190, `0x00110000`) | `66dea9a8…0e0d80` | destroyed in the battle; without the drive hits (v1, `79003906…`) it withdrew |
| `stranded` | Bulk Cruiser (0, `0x00110000`) | `2a22dda9…abf476` | cannot withdraw, so the battle runs until it is destroyed |

`doomed` (ISD 2740 hull damage, Bulk Cruiser stranded; output
`afeedd3c…342bd0`) does not give a destroyed ISD: at 10 hull its weapon
icons are dark, and even ordered to attack ("Attacking Bulk Cruiser 3")
neither ship moved or fired in over five minutes. Its first copy, with the
cruiser's drives intact, ended with the Alliance withdrawn. A ship lost in
battle needs an enemy that can fight, so the Alliance battle.

Low hull also dims the shield, weapons and tractor icons, so isolate a
subsystem at full hull. Battle Results has only Operational and Destroyed
columns. None of these copies has passed the re-save and reload gates; they
are staging for captures, not library saves.

## Recipe 4: a squadron's size

A fighter squadron is an unnamed `0x1c` record (header word 5 is 0, so no
name), class `1`–`4` for the Alliance's fighters and `5`–`8` for the
Empire's. Every full squadron reads 12 (`FIGHTSD.DAT` `squadron_size`).
Like a ship's hull, runtime `+0x60` holds damage, not the size left
(`FUN_005032c0`), and Status shows `12 − damage`.

| Copy | Graph (Alliance / Empire) | Length | `+0x60` damage |
|---|---|---|---|
| Full | `0x41` / `0x81` | 23 words | word 22; word 15 is the carrier's key (`0x14` family, its id) |
| Short | `0x51` / `0xa1` | 16 words | word 15 |
| The other side's sighting | `0x62` or `0xe2` / `0x92` or `0xd2` | 16 words | word 15; may lag the other two |

Words count from the record start (word 6 is the `+0x50` flags). The full
and short copies always agree; the third does not always, so treat it as
the other side's last sighting and leave it. In the Coruscant save, six
squadrons already differ there (one reads 11, a 1:12 squadron). Edit the
full and short copies together.

Damage 12 should empty a squadron, which `FUN_005032c0` treats as
destroyed. Removing squadrons that way is untested.

## Checksums

None found in the decompiled serializer (`FUN_004f9450` / `FUN_004f9340`,
`FUN_0053a1e0`) or the saves. That is not proof, so a load, re-save,
restart and reload in the original is part of every acceptance.

## Moving a save to and from the VM

`tools/original-capture/saves/INDEX.md`, "Moving files to and from the VM".
`vm-game.sh save-in FILE SLOT` and `save-out SLOT NAME` do it through the
guest agent. Every library save is also on the guest in
`C:\Rebellion\SaveLibrary\`, with an `INDEX.txt`: copy one over a slot to
swap it in. Slot 5 holds `empire-day488-late`; leave it.

## Driving the original for an acceptance run

- The Saved Games list shows the filled slots and one empty row, and does
  not scroll; a save in a slot after a gap (slot 8 with 6 and 7 empty) is
  not listed. Put a copy in the first empty slot.
- Pause (right-click the day readout) raises a modal "Resume Game Play?"
  that blocks every click. Inspect at Very Slow instead.
- To see what the original did with an edit, load it, press F1 at once (time
  stops on that screen), save over a slot whose library copy is in the repo,
  and diff the result against the edited file.

## Sources

- Codex research runs, 2026-10-10 (offsets re-checked by hand against the
  save).
- `ghidra/notes/capital-ship-save-layout.md` (the serializer chain and
  payload word map, 2026-10-11).
- `ghidra/notes/FUN_0040cfc0.c`, `FUN_00501780.c`, `FUN_00501820.c`,
  `status-window.md`, `tactical-subsystem-damage-path.md`,
  `FUN_0051ebf0.c`, `build-delivery.md`, `object-state-flags.md`,
  `FUN_004f9450.c`, `FUN_004f9340.c`, `FUN_0053a1e0.c`.
- TheArchitect2018, "Deep-Dive-into-SW-Rebellion-PC-Game-Internals" wiki,
  SaveGame page; MetasharpNet StarWarsRebellionEditor.NET,
  `documentation/SAVEGAME_FORMAT_SPEC.md` (the envelope only; its local
  `SAVEGAME.cs` does not parse payloads).
