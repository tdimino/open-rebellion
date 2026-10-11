---
title: "Original-game save library"
description: "Saves made in the original Star Wars Rebellion (GOG build) on the capture VM, to jump to a scenario for reference captures"
created: 2026-10-09
---

# Original-game save library

Saves we made in the original game on the capture VM
([`../README.md`](../README.md)), each at a state worth a reference capture.
The owner authorized committing them; saves downloaded from elsewhere stay
out. The game keeps ten slots,
`C:\Rebellion\SaveGame\SAVEGAME.001`..`.010` (the folder beside `REBEXE.EXE`;
the `app\SaveGame` folder the GOG installer makes is not the one it uses).
To load one, copy it into a slot (`vm-game.sh save-in FILE SLOT`) and pick it
from Game Options (F1), Saved Games.

| File | Side | Day | Settings | State |
|---|---|---|---|---|
| `alliance-day54-advice.sav` | Alliance | 54 | Standard game; difficulty and galaxy size as the menu left them (unchecked) | Briefing done; the nine opening advice topics and Maintenance Points in the Advice index, the rest held; no other messages |
| `empire-day2-start.sav` | Empire | 2 | Standard game; difficulty and galaxy size as the menu left them (unchecked) | Briefing just ended; nothing ordered. Fleet 1 (an Imperial Star Destroyer, a Carrack and six TIE squadrons) at Coruscant |
| `empire-day28-coruscant-fighters.sav` | Empire | 28–30 | As above | One TIE squadron dragged from Fleet 1 onto Coruscant; the planet's Fighter Squadrons tab lists TIE Fighters (nine or more, with a scroll bar). Which of them are based at the planet and which belong to Fleet 1 is not yet counted |
| `empire-fleet-to-yaga-minor.sav` | Empire | about 37 | As above | Fleet 1 in transit from Coruscant to Yaga Minor (held by the Alliance, Sesswenna sector). It arrives about day 43: "Fleet Arrives at Yaga Minor", then "Fleet Initiates Blockade of Yaga Minor". No battle; no Alliance fleet came by day 83 |
| `empire-day30-yaga-alert-control.sav` | Empire | 30 | As above | `empire-day2-start.sav` with Fleet 1 ordered to Yaga Minor, saved in transit on day 5, played on unedited to day 30 (slot name "transit3 day30", SHA-256 `8a7a9923…975636`). The Battle at Yaga Minor alert ("The Alliance fleet is attempting to break the Imperial blockade") opens within 12 s of the load |
| `empire-day30-yaga-alert-isd-hyperdrive.sav` | Empire | 30 | As above | The same day-5 transit save with the ISD's hyperdrive edited out (`agent_docs/original-save-editing.md`), played on to day 30 (slot name "transit3-hd day30", SHA-256 `ac7022d0…9ebd23`). The same alert opens within 12 s of the load |

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
