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
To load one, copy it into a slot under that name and pick it from Game
Options (F1), Saved Games.

| File | Side | Day | Settings | State |
|---|---|---|---|---|
| `alliance-day54-advice.sav` | Alliance | 54 | Standard game; difficulty and galaxy size as the menu left them (unchecked) | Briefing done; the nine opening advice topics and Maintenance Points in the Advice index, the rest held; no other messages |
| `empire-day2-start.sav` | Empire | 2 | Standard game; difficulty and galaxy size as the menu left them (unchecked) | Briefing just ended; nothing ordered. Fleet 1 (an Imperial Star Destroyer, a Carrack and six TIE squadrons) at Coruscant |
| `empire-day28-coruscant-fighters.sav` | Empire | 28–30 | As above | One TIE squadron dragged from Fleet 1 onto Coruscant; the planet's Fighter Squadrons tab lists TIE Fighters (nine or more, with a scroll bar). Which of them are based at the planet and which belong to Fleet 1 is not yet counted |
| `empire-fleet-to-yaga-minor.sav` | Empire | about 37 | As above | Fleet 1 in transit from Coruscant to Yaga Minor (held by the Alliance, Sesswenna sector). It arrives about day 43: "Fleet Arrives at Yaga Minor", then "Fleet Initiates Blockade of Yaga Minor". No battle; no Alliance fleet came by day 83 |

## Moving files to and from the VM

The guest has no network. Its transfer disk (drive `E:`, a FAT32 image)
carries files. The VM uses the copy inside its UTM bundle,
`~/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/Data/payload.img`
on the mini; `~/VMs/rebellion/payload.img` is only the original that was
imported, and does not see guest writes.

1. In the guest, copy the slot to `E:\saves-out\`.
2. Take disk 1 offline from an elevated PowerShell
   (`set-disk 1 -isoffline 1`), so Windows holds no cached view of it.
3. On the mini, attach the image read-only
   (`hdiutil attach -readonly -imagekey diskimage-class=CRawDiskImage`),
   copy the file out and detach.
4. Bring the disk back online (`set-disk 1 -isoffline 0`).

## Format

`SAVEGAME.00n` starts with a u16 name length and the slot's name, then u32
words: single player 1, the slot, and the side (1 Alliance, 2 Empire); the
entity records follow. External notes: TheArchitect2018's
"Deep-Dive-into-SW-Rebellion-PC-Game-Internals" wiki, SaveGame page.

## Not included

MetasharpNet's "Savegame Test Pack" (StarWarsRebellionEditor.NET release
`swr-editor.net-savegame-20231015`: all ships in a fleet, the same a day
before combat, all Alliance characters, all Empire characters) targets the
25th Anniversary patch and its repository carries no license, so it is not
copied here.
