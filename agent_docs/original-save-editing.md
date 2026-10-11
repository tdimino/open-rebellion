---
title: "Editing original-game saves"
description: "How to edit our own original-game saves safely to stage a state the original cannot reach on demand (a damaged hyperdrive, a Death Star), and how to accept the edit in the original"
category: "agent-docs"
created: 2026-10-10
updated: 2026-10-11
tags: [original-game, saves, parity, death-star, hyperdrive]
---

# Editing original-game saves

Some behaviour can only be checked in the original once the game reaches a
state that normal play does not produce on demand: a ship whose hyperdrive
is out before a Withdraw, or a finished Death Star with an escort. We get
there by editing a copy of one of **our own** saves
(`tools/original-capture/saves/`), loading it on the Mac Mini capture VM
(`tools/original-capture/README.md`, `vm-game.sh`), and accepting the edit
only after the original loads, plays and re-saves it.

Status (2026-10-11): both recipes load in the original. Recipe 1 has run
its Withdraw check against an unedited control (below): the ship with no
drive did not withdraw and was lost. One run per side, so not accepted yet.
Recipe 2's in-place conversion crashes the original when its fleet moves,
so it fails; a Death Star that moves needs route 1.

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
and `0x800000a1` (shorter). A later Empire save (day 488) also has
`0x80000091`/`0x80000092` for Imperial ships and `0x80000041`, `0x80000051`,
`0x80000062` for Alliance ships: bit 7 of the low byte is set for the
Imperial copies and clear for the Alliance ones. What the rest encodes has
not been traced. Edit every graph copy that a recipe touches.

`empire-day2-start.sav` (SHA-256 `d9da6a3b…7cbc79`, 410,763 bytes):

| Object | `0x81` record | `0xa1` record |
|---|---|---|
| Fleet 1 (`0x08`, id `0x77e`) | `0x1772d` | `0x48e98` |
| Imperial Star Destroyer 1 (`0x14`, id `0x77f`, class `0x85`) | `0x1778e` | `0x48ee5` |
| Carrack Light Cruiser 2 (`0x14`, id `0x7aa`, class `0x83`) | `0x17c05` | `0x4921c` |

The ISD's payload starts at `0x177c1`. Word 9 is `0x0800077e`, its parent
(family `0x08`, Fleet 1).

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
   every relationship the original builds. It needs a save where the Empire
   can order a Death Star, which the library does not have yet.
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

Never insert or clone a record: that needs every container count, every
later object key and every reference updated in both graphs.

Accept: the slot loads; Fleet 1 shows a Death Star and no ISD; the fleet can
be selected, moved and inspected; save to a new slot, quit, restart and
reload that one; bring an enemy fleet in, and the battle shows the Death
Star as its own object and returns to the galaxy; save and reload once
more.

## Checksums

None found in the decompiled serializer (`FUN_004f9450` / `FUN_004f9340`,
`FUN_0053a1e0`) or the saves. That is not proof, so a load, re-save,
restart and reload in the original is part of every acceptance.

## Moving a save to and from the VM

`tools/original-capture/saves/INDEX.md`, "Moving files to and from the VM".
Copying into the guest goes through mtools on the mini, because `hdiutil`
can only attach the image read-only while QEMU holds it.

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
