---
title: "Fleet Names"
description: "How fleets are named: the original's per-side numbering, and the canonical name bank a game may choose instead (a port extension)"
category: "mechanics"
created: 2026-10-06
updated: 2026-10-06
game_system: "fleet-names"
sources:
  - type: "ghidra"
    file: "ghidra/notes/fleet-names.md"
  - type: "code"
    file: "crates/rebellion-core/src/fleet_name_bank.rs"
---

# Fleet names

## The original: Fleet 1, Fleet 2

Each side numbers its own fleets galaxy-wide, from 1, and never gives a
number twice, even after its fleet is gone (`FUN_00517760`,
`FUN_005302c0`; see `ghidra/notes/fleet-names.md`). This is the default.

## Renaming (order 0x203)

The object pop-up menu's **Rename** edits a fleet's or capital ship's name in
place in the Fleet window (`FUN_004ac950`; see
`ghidra/notes/rename-order.md`). Enter commits a name that is not empty, and
an empty name keeps the field open. The original sets no length limit. A
ship without a name of its own shows its class name (`FUN_004f6270`). A
renamed fleet keeps its name under both modes, so a signature name never
takes it over. port: Escape or a click elsewhere drops the edit unissued.

## Canonical names (port extension)

The original has no other naming. Open Rebellion adds one, chosen in the
**Fleet Registry**: the chip left of the music toggle on the main menu. It
is off by default. The choice applies to the next new game and is saved with
it, so a loaded game never renames a fleet.

Under canonical naming:

1. A new fleet takes the first name in its side's bank that no living fleet
   holds. The bank order is fixed, so replays stay deterministic.
2. A name returns to the bank when its fleet is gone: destroyed, disbanded,
   or emptied by a join.
3. Once every name is held, a new fleet falls back to "Fleet N". The side's
   counter runs under both modes, so N continues the side's own count.
4. Two **signature names** wait for a flagship instead of going in turn.
   When no fleet holds one, it goes to the side's first fleet (in slot order)
   that holds the flagship class, and that fleet's old name returns to the
   bank. This happens when the fleet is made, when ships join it, and at the
   end of each day (for builds). A fleet that loses its flagship keeps the
   name.
   - **Death Squadron**: a Super Star Destroyer (CAPSHPSD.DAT `0x86`).
   - **Rebel Command Fleet**: a Mon Calamari Cruiser (CAPSHPSD.DAT `0x40`).

## The bank

Every name is a Galactic Civil War fleet formation of its side, spelled as
its Wookieepedia article titles it. Each article's infobox and lead were read
through the MediaWiki API on 2026-10-06. The in-game notes are our own
paraphrases.

### Empire (15)

| Name | Status | Source | Note |
|---|---|---|---|
| Death Squadron | Canon | [Death Squadron](https://starwars.fandom.com/wiki/Death_Squadron) | Signature: waits for a Super Star Destroyer. |
| Seventh Fleet | Canon | [Seventh Fleet (Galactic Empire)](https://starwars.fandom.com/wiki/Seventh_Fleet_(Galactic_Empire)) | |
| Third Fleet | Canon | [Third Fleet](https://starwars.fandom.com/wiki/Third_Fleet) | |
| Eleventh Fleet | Canon | [Eleventh Fleet](https://starwars.fandom.com/wiki/Eleventh_Fleet) | |
| First Naval Fleet | Legends | [First Naval Fleet](https://starwars.fandom.com/wiki/First_Naval_Fleet) | |
| 96th Task Force | Canon | [96th Task Force](https://starwars.fandom.com/wiki/96th_Task_Force) | |
| One Oh Third Task Force | Canon | [One Oh Third Task Force](https://starwars.fandom.com/wiki/One_Oh_Third_Task_Force) | |
| One Twenty-Fifth Task Force | Canon | [One Twenty-Fifth task force](https://starwars.fandom.com/wiki/One_Twenty-Fifth_task_force) | |
| Task Force 231 | Canon | [Task Force 231](https://starwars.fandom.com/wiki/Task_Force_231) | |
| Task Force Admonitor | Legends | [Task Force Admonitor](https://starwars.fandom.com/wiki/Task_Force_Admonitor) | |
| Task Force Vengeance | Legends | [Task Force Vengeance](https://starwars.fandom.com/wiki/Task_Force_Vengeance) | |
| Corrupter Task Force | Legends | [Corrupter Task Force](https://starwars.fandom.com/wiki/Corrupter_Task_Force) | |
| Hunter Fleet | Canon | [Hunter Fleet](https://starwars.fandom.com/wiki/Hunter_Fleet) | |
| Lothal Sector Fleet | Canon | [Lothal sector fleet](https://starwars.fandom.com/wiki/Lothal_sector_fleet) | |
| Qeimet Fleet | Legends | [Qeimet fleet](https://starwars.fandom.com/wiki/Qeimet_fleet) | |

### Alliance (10)

| Name | Status | Source | Note |
|---|---|---|---|
| Rebel Command Fleet | Legends | [Rebel Command Fleet](https://starwars.fandom.com/wiki/Rebel_Command_Fleet) | Signature: waits for a Mon Calamari Cruiser. |
| Alpha Group | Canon | [Alpha Group](https://starwars.fandom.com/wiki/Alpha_Group) | |
| Beta Group | Canon | [Beta Group](https://starwars.fandom.com/wiki/Beta_Group) | |
| Gamma Group | Canon | [Gamma Group](https://starwars.fandom.com/wiki/Gamma_Group) | |
| Delta Group | Canon | [Delta Group](https://starwars.fandom.com/wiki/Delta_Group) | |
| Fourth Division | Canon | [Fourth Division](https://starwars.fandom.com/wiki/Fourth_Division) | |
| Sixth Division | Canon | [Sixth Division](https://starwars.fandom.com/wiki/Sixth_Division) | |
| Seventh Division | Canon | [Seventh Division](https://starwars.fandom.com/wiki/Seventh_Division) | |
| Massassi Group | Canon | [Massassi Group](https://starwars.fandom.com/wiki/Massassi_Group) | |
| Phoenix Cell | Canon | [Phoenix Cell](https://starwars.fandom.com/wiki/Phoenix_Cell) | |

### Left out

| Name | Reason |
|---|---|
| Maw Fleet, Ackbar's Fleet | The article title is conjectural |
| Assertor Battle Group, Ilum Battle Group | The article title is conjectural |
| Fleet Group Protector, Relentless, Right to Rule, Stalwart; Carrion Spike Task Force | Cited only a category page, so not verified |
| Scourge Squadron | A Pentastar Alignment squadron after Endor |
| Black Sword Command | The source covers the Duskhan League's Black Fleet |
| Crimson Command | A command division that became a warlord flotilla after Endor |
| Task Force Whirlwind | Not clearly a standing fleet formation |
| Tempest Force | A garrison, not a fleet |
| Alliance Fleet, 14th Roving Line | The source covers the whole Rebel navy |
| Salient Battle Group, Mon Cala Mercantile Fleet | Not an Alliance formation |
| Barma Battle Group | A New Republic formation |
| Sullust Home Guard | An allied home guard, not an Alliance Fleet formation |
