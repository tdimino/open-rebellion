---
title: "Victory Flag Word: Bits, Tie Rule, Leader Mortality, Messages, Ending and Game-Type Readers"
description: "Every bit FUN_005180e0 writes to root +0x74 for both game types, the leader ids, the tie rule, why the six major characters can never be destroyed (kill slot +0x2ec injures them only), when the HQ-destroyed and Coruscant-captured messages file, the ending's side-by-side outcomes, and the four readers of the game type"
category: "ghidra"
tags: [victory, game-type, leaders, assassination, messages, endings, ai]
---

# Victory flag word

Builds on `victory-conditions.md` and `game-end-path.md` (neither is changed
here). Sides are the game's: 1 Alliance, 2 Empire. `conf:` high means read in
the cited decompile, medium means a reading across several functions.

## 1. The word, bit by bit (`FUN_005180e0`)

Inputs (`FUN_00505190` → `FUN_00505230` for characters, `FUN_00505f60` for
Coruscant, `FUN_00506110` for the HQ, root `+0x60` for the game type):

| Id | Is | Local in the decompile |
|---|---|---|
| `0x32000242` | Luke Skywalker | `iVar8` → `uVar15` (LC) |
| `0x30000240` | Mon Mothma | `local_3c[5]` → `uVar14` (MC) |
| `0x35000281` | Darth Vader | `local_24` → `uVar12` (DC) |
| `0x34000280` | Emperor Palpatine | `local_20` → `local_44` (EC) |
| `0x90000109` | Coruscant | `iVar9` |

Names follow the two-letter strings of `FUN_00530a50` and the families (Luke
and Mon Mothma Alliance, Vader and Palpatine Empire); the code itself carries
only the ids. Leia (`0x31000241`) is not read. conf: high.

Return value 0 (word untouched) when any of the five objects is missing or the
root is null. Otherwise the word is rebuilt from zero on every call and
stored only when it differs (`+0x74`, then `FUN_004fd490(root+0x194, ctx)`).
The word is therefore not sticky: a bit clears the moment its condition lapses
(a leader released, Coruscant retaken). Only EW/AW persist, and only through
the tie rule below.

Component bits (set when the stated test holds):

| Bit | Code | Test |
|---|---|---|
| `0x1` | AD | HQ master (`FUN_00506110`) is null, or its `+0x50` bit 3 (destroyed) is set |
| `0x10` | LC | Luke `+0xac` bit 0 (prisoner) |
| `0x20` | MC | Mon Mothma `+0xac` bit 0 |
| `0x100` | CA | Coruscant `+0x24 & 0xc0 == 0x40` (owner Alliance) |
| `0x200` | CD | Coruscant `+0x50` bit 3 |
| `0x1000` | DC | Vader `+0xac` bit 0 |
| `0x2000` | EC | Palpatine `+0xac` bit 0 |
| `0x100000` | EW | `bVar2` below |
| `0x1000000` | AW | `bVar4` below |
| `0x10000000` | GO | EW or AW (`bVar6`) |

Win tests, switch on root `+0x60`:

| `+0x60` | Game | EW | AW |
|---|---|---|---|
| 0 | Standard | AD and LC and MC | DC and EC and (CA or CD) |
| 1 | Headquarters Only | AD | CA or CD |
| other | | returns 0 after the bits are skipped; word unchanged | |

The component bits (`0x10`..`0x2000`, CA, CD, AD) are written for both types,
so the Headquarters Only word still shows LC/MC/DC/EC. conf: high.

## 2. Tie rule

When EW and AW both hold, the block after the type switch tests
`+0x74 & 0x1000000` (old AW): set, AW wins. Else it tests `+0x74 >> 0x14 & 1`
(old EW): set, EW wins. Neither set: AW wins. The loser's bit is cleared, so
GO covers exactly one winner. A tie needs, Standard: AD, LC, MC, DC, EC and
(CA or CD) all true; Headquarters Only: the HQ gone and Coruscant Alliance or
destroyed. conf: high. The port returns the Empire first
(`victory-conditions.md`, Port).

## 3. Can a principal leader die or vanish?

`FUN_00505190(id)` (`FUN_00505230`) returns the master-copy pointer
(`FUN_005844e0` reads `+0x24`) whenever the id exists with family `0x30`-`0x3b`;
the destroyed bit (`+0x50` bit 3) is tested only to decide a release of a
temporary reference, not to return null. A destroyed character is therefore
still returned. Null needs an id missing from the registry, i.e. a modified
scenario. conf: high.

The six majors (`0x30`-`0x35`) cannot be destroyed by any path found:

- **Kill slot `+0x2ec`** (`FUN_004ef690`, the same in every character vtable;
  checked on `0x006657f0`, `0x006663e0`, `0x00665fd8`, `0x00666f90`,
  `0x00667628`, `0x006681d0`, `0x0066b608`, `0x00665be0`) rolls an injury
  amount (`FUN_0053eae0`) and calls slot `+0x2f0` with reason 7.
- **Slot `+0x2f0`** (`FUN_004ef6d0`) adds the injury (`FUN_004ee2d0`), then
  calls the destroy slot `+0xac(reason)` only if slot **`+0x2e0`** returns
  non-zero. Slot `+0x2e0` is `FUN_006158b0` (returns 0) in all six major
  vtables and `FUN_0056f160` for minors (`FUN_0053eb30`: a
  `FUN_0053e2f0(DAT_006b90c8)` percentage roll). So a major is injured but
  never destroyed; a minor is destroyed on a successful roll.
- **Callers of `+0x2ec`/`+0x2f0`:** Assassination (`FUN_00576620`, kill with
  reason 7, destroyed check afterwards), `FUN_004ef5f0`, `FUN_00549a20`,
  `FUN_00561b60`, `FUN_00560200` (meeting injury) and Destroy System's minor
  roll (`FUN_0058d990`, minors only, reason 10). All reach `FUN_004ef6d0`.
- **Container destruction** (Destroy System cascade, ship or fleet loss, slot
  `+0xb4` = `FUN_004ef2e0`) flags a character adrift (`+0x78` bit 6 or
  `FUN_004f78e0`) instead of destroying it. A capital ship's own hook
  `+0x11c` (`FUN_00502840`) then gives each character aboard slot `+0x2e8`
  (`FUN_004ef640`), a wound through `+0x2f0`, which a minor may die of and a
  major cannot (`tactical-results.md` 3). **Control change** (slot `+0xb0` =
  `FUN_004ef260`) releases a prisoner (`+0x218`) and flags adrift.

Assassination of a major therefore succeeds as a mission (the injury roll
counts) but the target is not destroyed, so `FUN_00576620` skips its reward
branch and the leader stays on the map, injured. Consequence for the word:
the leader bits can only change through capture and release (`+0xac` bit 0),
and the word never reads "dead". conf: high for the slot chain, medium for
"no other path" (a whole-binary caller sweep of `+0xac(reason)` on characters
was not run; ground combat was not traced for aboard characters; the
callers of `+0x2ec`, `+0x2f0`, `+0x2e4` and `+0x2e8` are listed in
`tactical-results.md` 3).

## 4. When the two messages file

Both are message classes bound to notification `0x128`
(`SideVictoryConditionsNotif`), one instance per side's message manager
(`this+0x20` is the side). Notification `0x128` posts whenever a side object's
`+0xbc` word value changes (`victory-conditions.md`, "From the flags to the
ending", step 1), with the new word as argument.

| Message | Class | Files when | Fields |
|---|---|---|---|
| Rebel HQ Destroyed (`0x1f`) | `FUN_0048e5a0` | a `0x128` delivery whose word has AD (`param_4 & 1`); no side test: both sides | `+0x28 = 0x11` (advice), TEXTSTRA `0x7188`/`0x7189`, `+0x5c = 0x73` (movie MDATA.102) |
| Coruscant Captured | `FUN_0048f790` | any `0x128` delivery while Coruscant, seen through this side's view (`FUN_004f3220(side, 0x90000109)`), has owner Alliance (`+0x24 & 0xc0 == 0x40`); the word's bits are not read | `+0x28 = (side != 1) + 1`, `+0x64 = 1`, TEXTSTRA `0x7170`/`0x7171`, `+0x5c = 0x74` (MDATA.103) |

- Each files on an edge of the word, not on every change. The hook that posts
  `0x128` (`0x00532fb0`, a notification slot with the hook arguments old, new
  and context; `FUN_0053fcf0(0x128, side object, old, new, ctx)` at
  `0x00533033`) stores the old word at node `+0x24` and the new word at `+0x28`.
  The manager's kind chooser `FUN_00488de0` maps a `0x128` group to kind `0x1f`
  when the new word has AD and the old word does not, and otherwise to kind
  `0x1e` when the new word has CA and the old word does not
  (`FUN_00488de0.c:134-142`); any other change maps to no message. A word that
  gains AD and CA together files only Rebel HQ Destroyed, and a later LC change
  with AD still set files nothing. The class bodies' own tests (above) then
  decide whether the message stays filed. Conf: high for the chooser, medium for
  the old/new order of the two stored words (read from the hook's stack
  arguments).
- Capture only: a destroyed Coruscant sets CD but not an Alliance owner, so
  `FUN_0048f790` does not file; the HQ message is destruction or removal
  (AD), not capture, though capture of the HQ system removes the HQ
  (`game-end-path.md`).
- A side with no view of Coruscant gets nothing (the lookup returns null).
- Each side's view of Coruscant holds the master's holder when `0x128` is
  delivered. A view copy's `+0x24` bits 0-3 are a live mask, 1 or 2
  (`FUN_004f9a60`, run first in `FUN_004fae40`, the owner hook's child walk): 1
  when the object's owner is the view's side, or the object's `+0x50` bit 7
  (Alliance view) or 8 (Empire view), "ObservedBy", is set with bit 2 set and bit 4
  clear; else 2. For a system `FUN_0050ac80` sets the observed bit for a side
  that owns it or has an active fleet there. `FUN_004f9ca0` copies the owner to a
  view only when `FUN_005404d0(view, context +8)` passes, and a default context
  has `+8` = 1 (`FUN_004fd400`), so a view with mask 1 is updated at the write and
  a view with mask 2 is not. A view going from mask 2 to 1 is refreshed in full
  (`FUN_004f9a60`: view slot `+0x24`, `FUN_004f8340`). At a capture the order is
  `FUN_004f6f40` (master owner, then `FUN_004f9ca0` on both views), then slot
  `+0xf0` `FUN_00510820` → `FUN_004fae40` → `FUN_004f9a60`, then the `0x31b` event,
  whose handler `FUN_00517be0` rebuilds the word. The loser held the system, so its
  view's mask was 1 at the write and took the new owner there; the winner's mask
  becomes 1 in `FUN_004f9a60` and is refreshed. Both views show the new holder
  before the word is rebuilt, so the CA bit and each side's view agree, and
  `FUN_0048f790` files for both sides. A side whose view stays at mask 2 (it neither
  owns Coruscant nor has seen it) would lag; no capture path leaves one. Conf:
  medium-high (the contexts of individual orders are not read).
- Only the local human's pump plays the movie (`movies.md`, "Both sides").

## 5. Ending, side by side

Chain, unchanged from `victory-conditions.md` and `game-end-path.md`:
notification `0x128` → local player slot `+0x74` (`FUN_00486890`, GO only) →
advisor slot `+0x10` (`FUN_004c3010` Alliance, `FUN_004c0f10` Empire) → stamps
reaction slot at clock + 40000 and calls `FUN_0041d810` (strategic window
`+0xac = 1`) → advisor script, whose step `0x11` finish sets
`DAT_006b28c4 = 3` → `FUN_004c3060` / `FUN_004c0f60` read the side object's
word → `FUN_0041d830` → `FUN_00425c90` stores code `0x10`-`0x13` at strategic
window `+0xa8` → `FUN_0042db70` posts `0x482` → `FUN_00422ce0` →
`FUN_00401040` → `FUN_00414330` state `0x10`-`0x13` teardown →
`FUN_00414830` → `FUN_004146e0` movie window.

| Viewer | Outcome | Reaction slot (`+0x168` block) | Script | Code | Movie |
|---|---|---|---|---|---|
| Alliance | Alliance wins (victory) | `+0x158` (86) | ALSPRITE 10094 | `0x10` | `0x6b` MDATA.105 |
| Alliance | Empire wins (defeat) | `+0x15c` (87) | ALSPRITE 10093 | `0x11` | `0x6a` MDATA.106 |
| Empire | Empire wins (victory) | `+0x150` (84) | EMSPRITE 11094 | `0x12` | `0x6d` MDATA.108 |
| Empire | Alliance wins (defeat) | `+0x154` (85) | EMSPRITE 11093 | `0x13` | `0x6c` MDATA.107 |

Victory is the `..94` script and the longer wave on both sides; defeat is
`..93`. The slot choice is read in `FUN_004c3010` (`0x100000` → `+0x15c`,
`0x1000000` → `+0x158`); the slot-to-script table is
`game-end-path.md`. conf: high for the mapping to EW/AW, medium that `..94` is
heard as "victory" (never listened to; inferred from the slot).

Player options: the movie ends on its last frame (`0x402`), on Esc or a left
click (the frame pump's `0x404`, `FUN_005f7240`), or on any key but Esc.
Each posts `0x405`; `FUN_004138a0` handles ids `0x6a`-`0x6d` with
`FUN_00414330(3)`, the main menu (`FUN_00414a80`), campaign fields cleared.
No continue-playing path exists: the teardown precedes the movie. conf:
medium-high.

VM, Empire viewer, Alliance wins (Standard game; `empire-day660-deathstar-pre-alert.sav`
with Palpatine moved into the Alliance Jail, between days 682 and 692): the ending film
runs on its own (the Imperial command deck, an explosion, then the crawl "Your failure is
now complete. The Galactic Empire lies in ruins and the galaxy is in the hands of the
Rebels. The Emperor and Lord Vader are lost. …") and ends on the main menu, "Standard
Game" on its panel.

## 6. Readers of the game type

The type is the settings record's `+8` (root `+0x58`, so root `+0x60`).
Settings are read by `FUN_004fce50` (copies the scheduler's settings
`FUN_0051d780`, else a default `FUN_0051c740`) and the root's `+0x58` directly.
Callers of the copy: three; a scan of every function that touches the root
(`DAT_006b2bb0`, getter `FUN_00506e10`) for a `+0x60` read found one.

| Reader | Reads | Effect |
|---|---|---|
| `FUN_005180e0`@005182bd | root `+0x60` | the win tests of section 1 |
| `FUN_004182a0` case 1 | settings `+8 == 1` (Headquarters Only) | sets bit 0 of the AI side context `+4` |
| `FUN_0044b2d0` (Alliance), `FUN_0044bb70` (Empire) | settings `+8 == 0` (Standard) | the status window's Victory Conditions page lists the leader rows (Mon Mothma/Luke/Vader/Palpatine captured, `0x2b05`..) only in Standard |

AI context `+4` bit 0 (set = Headquarters Only) is read by:

- `FUN_00401d20`: when clear (Standard) the four leader ids get record bit
  `0x40000000` (`FUN_004024d0`), plus `0x400` for the Emperor; Headquarters
  Only drops them from the leader class.
- `FUN_00484630`: when clear, adds 1 to the counter at `+0x8c` for each leader
  seen (`FUN_004024d0`).
- `FUN_00474b90`: task-fleet need is `fleet +0xb4 - ctx+0x164`, or minus
  twice `ctx+0x164` when set; a Headquarters Only game asks twice the
  reserve (an HQ-defence rule).

conf: high for these four readers; not exhaustive for `+4` bit 0 readers
beyond the `FUN_005f4960` callers listed (the sweep matched that accessor).
AI context `+4` bit 1 (`|2`) is set by `FUN_004189b0` (word received), not by
the game type.

## Open

- Whether a destroyed minor's id can re-enter lookups (not needed for the
  word; majors never reach it).
- Whether the message manager suppresses a repeated Rebel HQ Destroyed or
  Coruscant Captured on later word changes.
- A VM check of ESC or click during the ending movie.
- The wound itself in the VM. Two Assassinations of Han Solo
  (`0x33000243`, combat 100) by Daala at combat 220 (`save_tree.py
  assassin`, `original-save-editing.md`) each ended within about ten days
  with Han's record unchanged (injury 0), no Imperial mission message, and
  Daala on the hidden Return `0x42000002`: two failed rolls (about one
  chance in five at the 55 percent row) or a gate before `+0x2ec`.

## Decompiles

`FUN_005180e0.c`, `FUN_00505190.c`, `FUN_00505230.c`, `FUN_0053efd0.c`,
`FUN_005844e0.c`, `FUN_004ef690.c`, `FUN_004ef6d0.c`, `FUN_004ef260.c`,
`FUN_004ef2e0.c`, `FUN_0056f160.c`, `FUN_0053eb30.c`, `FUN_0053eae0.c`,
`FUN_004ee2d0.c`, `FUN_00576620.c`, `FUN_0056ece0.c`, `FUN_0056fd40.c`,
`FUN_00572530.c`, `FUN_005739c0.c`, `FUN_005753e0.c`, `FUN_0048e5a0.c`,
`FUN_0048f790.c`, `FUN_004c3010.c`, `FUN_004c0f10.c`, `FUN_00486890.c`,
`FUN_0041d810.c`, `FUN_004fce50.c`, `FUN_0051d780.c`, `FUN_0051c790.c`,
`FUN_00513640.c`, `FUN_004182a0.c`, `FUN_0044b2d0.c`, `FUN_0044bb70.c`,
`FUN_00401d20.c`, `FUN_00484630.c`, `FUN_00474b90.c`, `FUN_004189b0.c`,
`FUN_005f4960.c`.
