---
title: "Original-game save library"
description: "Saves made in the original Star Wars Rebellion (GOG build) on the capture VM, to jump to a scenario for reference captures"
created: 2026-10-09
---

# Original-game save library

Saves we made in the original game on the capture VM
([`../README.md`](../README.md)), each at a state worth a reference capture.
The owner authorized committing them; saves downloaded from elsewhere stay
out. The one exception is `alliance-deathstar-at-coruscant.sav`, which
David Adams ([@davidadas](https://github.com/davidadas), author of
[rebellion2](https://github.com/davidadas/rebellion2)) made and gave us;
the owner authorized committing it with that credit. The game keeps ten slots,
`C:\Rebellion\SaveGame\SAVEGAME.001`..`.010` (the folder beside `REBEXE.EXE`;
the `app\SaveGame` folder the GOG installer makes is not the one it uses).
To load one, copy it into a slot (`vm-game.sh save-in FILE SLOT`) and pick it
from Game Options (F1), Saved Games.

| File | Side | Day | Settings | State |
|---|---|---|---|---|
| `alliance-day54-advice.sav` | Alliance | 54 | Standard game; difficulty and galaxy size as the menu left them (unchecked) | Briefing done; the nine opening advice topics and Maintenance Points in the Advice index, the rest held; no other messages |
| `alliance-transit-day149.sav` | Alliance | 149 | As above | `alliance-day54-advice.sav` played on; Fleet 1 (Corellian Corvette 1, Medium Transport 1) ordered from Ithor to Coruscant and saved in transit (slot name "alliance transit day149", SHA-256 `7aa35bab…599ed8`). About 400 s after the load the Battle at Coruscant alert opens against Judicator and Tyrant; the fleet is destroyed about a minute after Resume Battle. Fleet 2 (damaged) stays at Ithor. Show Planet and the other display options reload On |
| `empire-sixteen-tie-day415.sav` | Empire | 415 | As above | A new Empire game, Huge galaxy, Easy (5 starting capitals, maintenance 738). Coruscant built three Orbital Shipyards and three Carrack Light Cruisers. Fleets 2–4 were dragged onto Fleet 1, which then holds all eight Imperial capitals. Six of Coruscant's own TIE squadrons were dragged aboard (16 of 16), and the game was saved at Coruscant (slot name "empire sixteen tie", SHA-256 `1f8381e5…2f9baa`). Dragging Fleet 1 to Xyquine (Corellian sector, Alliance) opens the Battle at Xyquine alert about 21 days later. Take Command shows task forces 1–4 and red, blue, green and gold fighter groups; splitting each two-ship task force into the blank slots gives 1–8 (TAC-02-C001, C003–C005, C007) |
| `empire-day2-start.sav` | Empire | 2 | Standard game; difficulty and galaxy size as the menu left them (unchecked) | Briefing just ended; nothing ordered. Fleet 1 (an Imperial Star Destroyer, a Carrack and six TIE squadrons) at Coruscant |
| `empire-day28-coruscant-fighters.sav` | Empire | 28–30 | As above | One TIE squadron dragged from Fleet 1 onto Coruscant; the planet's Fighter Squadrons tab lists TIE Fighters (nine or more, with a scroll bar). Which of them are based at the planet and which belong to Fleet 1 is not yet counted |
| `empire-fleet-to-yaga-minor.sav` | Empire | about 37 | As above | Fleet 1 in transit from Coruscant to Yaga Minor (held by the Alliance, Sesswenna sector). It arrives about day 43: "Fleet Arrives at Yaga Minor", then "Fleet Initiates Blockade of Yaga Minor". No battle; no Alliance fleet came by day 83 |
| `empire-day30-yaga-alert-control.sav` | Empire | 30 | As above | `empire-day2-start.sav` with Fleet 1 ordered to Yaga Minor, saved in transit on day 5, played on unedited to day 30 (slot name "transit3 day30", SHA-256 `8a7a9923…975636`). The Battle at Yaga Minor alert ("The Alliance fleet is attempting to break the Imperial blockade") opens within 12 s of the load |
| `empire-day30-yaga-alert-isd-hyperdrive.sav` | Empire | 30 | As above | The same day-5 transit save with the ISD's hyperdrive edited out (`agent_docs/original-save-editing.md`), played on to day 30 (slot name "transit3-hd day30", SHA-256 `ac7022d0…9ebd23`). The same alert opens within 12 s of the load |
| `empire-day53-yaga-orbit.sav` | Empire | 53 | As above | `empire-fleet-to-yaga-minor.sav` played on; Fleet 1 (ISD, Carrack, three Stormtrooper Regiments) in orbit of Alliance-held, ungarrisoned Yaga Minor, with the orbit window open (slot name "yaga orbit", SHA-256 `d6a15289…fdacef5`). Right-click Fleet 1, Planetary Assault: the troops take the system and the Assault Summary opens (TAC-08-C001, C003, C004, C007, C008) |
| `empire-day101-assault-ready-drall.sav` | Empire | 101 | As above | The same game after the Yaga Minor assault: one Stormtrooper Regiment dragged back onto Fleet 1, which was flown to Alliance-held Drall (Corellian sector) and sits in orbit there (slot name "drall orbit", SHA-256 `6af95cb4…1111eb4`). Drall's garrison is unknown to the Empire. Meant for a weak assault (failure, casualties) |
| `empire-day128-assault-fails-selonia.sav` | Empire | 128 | As above | The same game played on: Fleet 1 with one Stormtrooper Regiment in orbit of Alliance-held Selonia (Corellian sector), SHA-256 `e96f481d…0bec9488`. Selonia holds two Alliance Army Regiments; Planetary Assault fails, destroys one of them and costs the Empire nothing. The report reopens from the Message Index (TAC-08-C002, C005, C006) |
| `empire-day488-late.sav` | Empire | 488 | As above | The late Empire game slot 5 holds on the VM (SHA-256 `9b9774e5…51d2e7b0`); keep slot 5 unchanged |
| `empire-huge-deathstar-built.sav` | Empire | not recorded | Huge galaxy | Edited (route 1 in `agent_docs/original-save-editing.md`): the Death Star order set one short of cost, after which the original finished "Death Star 1" into Fleet 6 at Coruscant (SHA-256 `5a9661fe…94e48b`) |
| `empire-day660-deathstar-pre-alert.sav` | Empire | 660 | Huge galaxy | The same Death Star game; Fleet 6 heading back to Coruscant. The Battle at Coruscant alert opens on day 676, after a Carida alert on day 669 (slot name "ds pre alert", SHA-256 `5973cf91…78de1`) |
| `alliance-day731-imperial-deathstar-built.sav` | Alliance | about 731 | As `alliance-day54-advice.sav` | An Alliance game whose Imperial ISD order was edited into a Death Star (21 words; plan doc, session 5); the original completed it as "Devastator" at Coruscant (SHA-256 `502ac1cb…d67904`) |
| `alliance-day830-after-coruscant-battle.sav` | Alliance | about 830 | As above | The same game after the Alliance withdrew from a second Battle at Coruscant. The Death Star sits at Coruscant but has not joined Alliance-initiated battles there; only Fleet 302 fights (SHA-256 `abe0dc2d…6dce32`) |
| `alliance-deathstar-at-coruscant.sav` | Alliance | 1119 | Not recorded | Made and given by David Adams ([@davidadas](https://github.com/davidadas)); header name "Death Star at Coruscant", SHA-256 `5825d980…a4c5`. Alliance Fleet 1 (transports and corvettes, no fighters) is bound for Imperial Coruscant, and the Battle at Coruscant alert opens on day 1123 against Fleet 16 (Judicator, Punisher, Death Star 1), Fleet 190 (Dictator, Oppressor, Bear) and Fleet 192 (Despot): 22 TIE squadrons in all. Fleet 2 at Delaya carries 30 X-wing squadrons, enough for Attack Death Star, which needs more Alliance fighter groups than Imperial ones |
| `alliance-day1169-deathstar-trench-run.sav` | Alliance | 1169 | As `alliance-deathstar-at-coruscant.sav` | David's save played on, unedited (slot name "ds trench run", SHA-256 `ad68f6f4…dd9a0`). Fleets 2 (30 X-wing squadrons) and 7 (29 squadrons, from Pantolomin) were sent to Averam on day 1119 and merged there into Fleet 2: 26 ships, 59 fighter squadrons. Fleet 1's own Battle at Coruscant on day 1123 was auto-resolved. Fleet 2 is in transit to Coruscant; the Battle at Coruscant alert opens on day 1173 against Fleet 16 (Punisher, Death Star 1) and Fleet 193 (Dictator, Despot, Ruler, Oppressor, Bear, Cronos, Lord), 23–24 TIE squadrons, so Attack Death Star's fighter majority holds. Take Command needs a fresh launch |

## The library on the VM

`C:\Rebellion\SaveLibrary\` on the guest holds a copy of every save above
under a descriptive name, plus `INDEX.txt` (name, SHA-256, one-line
description). To swap one in on the guest, copy it over a slot
(`copy /y C:\Rebellion\SaveLibrary\NAME.sav C:\Rebellion\SaveGame\SAVEGAME.007`)
and load it from Saved Games. From this Mac, `vm-game.sh save-in FILE SLOT`
does the same with the repo copy. Slot 5 stays on `empire-day488-late`.

`tools/original-capture/save-library/push.sh` rebuilds the library from
`manifest.tsv` (repo path, library name, description). It copies each save
through spare slot 10, checks its SHA-256 on the guest, and rewrites
`INDEX.txt`. When you add a save here, add a manifest row and rerun it. It
uses the guest agent, so never run it alongside an `a0` capture.

## Moving files to and from the VM

The guest has no network. From this Mac, `vm-game.sh` moves saves through
the QEMU guest agent, which runs in the background and leaves the game
focused, and prints both SHA-256 values:

```sh
tools/original-capture/vm-game.sh save-out 6 transit3     # slot 6 -> ./transit3.sav
tools/original-capture/vm-game.sh save-in transit3.sav 6  # ./transit3.sav -> slot 6
```

Saved Games lists the filled slots and one empty row, so a slot after a gap
does not appear. Files pass through `~/VMs/rebellion/exchange/` on the
mini, which the guest also sees as `Z:` (the SPICE WebDAV shared folder);
`VM_XFER=webdav` copies through `Z:` with a command typed into Win+R
instead of the agent. Both are described in [`../README.md`](../README.md).

The transfer disk (drive `E:`, a FAT32 image) is still attached and still
works, but nothing needs it any more. The VM uses the copy inside its UTM
bundle, `~/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/Data/payload.img`
on the mini. To use it: copy the file to `E:\saves-out\`, take disk 1
offline from an elevated PowerShell (`set-disk 1 -isoffline 1`), read it on
the mini with mtools (the FAT32 partition starts 1 MiB in), and bring the
disk back online (`set-disk 1 -isoffline 0`):

```sh
I="$HOME/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/Data/payload.img@@1048576"
MTOOLS_SKIP_CHECK=1 mcopy -i "$I" SAVEGAME.006 ::/saves-in/   # in
MTOOLS_SKIP_CHECK=1 mtype -i "$I" ::/saves-out/x.sav > x.sav  # out
```

## Format

`SAVEGAME.00n` starts with a u16 name length and the slot's name, then u32
header words (`empire-day2-start.sav`: 1, 0, 1, 2, 2, 0, then more), then
the entity records. The record layout, and how to edit a copy safely, is in
[`agent_docs/original-save-editing.md`](../../../agent_docs/original-save-editing.md). External notes: TheArchitect2018's
"Deep-Dive-into-SW-Rebellion-PC-Game-Internals" wiki, SaveGame page.

## Not included

MetasharpNet's "Savegame Test Pack" (StarWarsRebellionEditor.NET release
`swr-editor.net-savegame-20231015`: all ships in a fleet, the same a day
before combat, all Alliance characters, all Empire characters) targets the
25th Anniversary patch and its repository carries no license, so it is not
copied here.
