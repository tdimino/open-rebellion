---
title: "Ghidra RE Notes — Index"
description: "Master index of 7,279 function-note C files (7,070 by address, 209 named/support copies), Ghidra scripts, and recovered subsystem notes"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-18
---

# Ghidra RE Notes — Index

**7,279 function-note C files (7,070 `FUN_<address>.c` plus 209 named/support copies), Ghidra scripts, and indexed recovery notes**

Provenance: until 2026-09-26, 2,752 `FUN_<address>.c` files were empty placeholders from commit aa2b333. They were filled, and 188 functions cited elsewhere in the repo were added, with Ghidra 12.1.3 headless `CreateAndDecompileTargets.java` against the read-only project. 29 of those files explain instead of decompiling: 27 addresses lie inside another function and name it, 1 (`0x005587d0`) is a jump stub outside any function, and 1 records the only decompiler failure (`FUN_006197d2`, "Overlapping input varnodes"). No note is empty.

## Scholar Documents

| File | Lines | Content |
|------|-------|---------|
| [annotated-functions.md](annotated-functions.md) | 1,071 | Struct layouts (+0x60 hull, +0x64 shield/weapon nibbles, +0x96 CommandKind, +0xac prisoner bit), the story-check chain `FUN_00549910`, renamed variables, 50 event IDs (0x127-0x370), game rules |
| [modders-taxonomy.md](modders-taxonomy.md) | 793 | 10 game systems categorized for total conversion mods (Yuuzhan Vong, Thrawn, KOTOR). Function addresses, GNPRTB params, mod guidance per system |
| [rust-implementation-guide.md](rust-implementation-guide.md) | 417 | Maps decompiled C to Open Rebellion's `advance()` pattern: space battles through the tactical engine, ground combat, bombardment pointers, `MstbTable`, 15 new CapitalShipClass fields |
| [cpp-class-hierarchy.md](cpp-class-hierarchy.md) | 434 | CRebObject → CNotifyObject classes by DAT family, master and side-view copies. 19 vtable slots, 6 vtable pointer constants, complete field layout, setter-notify-event pattern |
| [entity-system.md](entity-system.md) | 678 | Characters (8 enhanced skills, Force fields (force-users.md), betrayal), game objects (5 destruction variants), fleets (4 events), factions (Alliance/Empire/Neutral bits) |
| [mission-event-cookbook.md](mission-event-cookbook.md) | 685 | Mission types by MISSNSD family, 4 story event chains (Dagobah, Vader, Palace, Bounty), event ID registry (0x12c-0x370), Thrawn Campaign example |
| [economy-systems.md](economy-systems.md) | 482 | Resources (energy/material at +0x5c-0x68), 5 ControlKind states, uprising (UPRIS1TB/2TB), blockade manufacturing halt, repair (GNPRTB 0x305/0x386), 36 system notif handlers |

## Atlas

| File | Lines | Content |
|------|-------|---------|
| [rebexe-map.md](rebexe-map.md) | 892 | Every address range of REBEXE.EXE (22 ranges, 0x00401000-0x00629000): subsystems, classes, vtables, key functions, and what the port lacks per range |

## Mod Surface

What the original fixes in code rather than data (epic `or-ep-mod-surface-vdch`).

| File | Lines | Content |
|------|-------|---------|
| [force-users.md](force-users.md) | 125 | Force fields (`+0x8c`/`+0x8e`/`+0x90`, `+0xac` bits 4/5/7), creation, discovery (timer `0x389`), Jedi Training, detection, the Luke/Leia/Vader/Palpatine id rules, R1-R18 |
| [movies.md](movies.md) | 286 | The one movie player, the id-to-file switch `FUN_00421ba0`, app-state and message-class triggers, the trench-run pair, the return path; the 0x482/0x468 wait that sends 0x40d to a capture or modal window |
| [galaxy-limits.md](galaxy-limits.md) | 125 | What fixes 200 systems and 20 sectors: the nearness table, family windows, the galaxy-size filter, the view frame, the save |
| [story-encounters.md](story-encounters.md) | 157 | Each story chain as trigger, test and outcome rows: the `FUN_005438a0` gate, timers `0x392`/`0x393`, the ten flag checks, side branches |
| [ship-type-assets.md](ship-type-assets.md) | 106 | Ship and fighter meshes, portraits, art, icons, encyclopedia and voice: the 38-entry ordinal registry versus DAT arithmetic |
| [side-switches.md](side-switches.md) | 256 | Every per-side switch (DLLs, cockpit art, advisor, voices, movies) and the structures that hold exactly two sides |
| [scenario-literals.md](scenario-literals.md) | 200 | Every object id REBEXE names directly (leaders, Coruscant, Yavin, HQ, named characters, research milestones), and what replacing each breaks |
| [ground-unit-assets.md](ground-unit-assets.md) | 192 | Troop, special-force and facility art from the TEXTSTRA id, the eight facility sprites, build-list rules, the factory registry's exact family bytes, literal ids in finders and the AI |
| [audio-dispatch.md](audio-dispatch.md) | 134 | Sound descriptors (wave id plus module id), the module-to-DLL map, the 317-event tactical table, the 16-track music table and its selectors |
| [text-sources.md](text-sources.md) | 150 | The string hook and its module groups, the text DLLs and their id ranges, the TEXTSTRA version gate, `data0.reb`, and the literals and glue fixed in REBEXE |
| [death-star-strategic.md](death-star-strategic.md) | 143 | The strategic Death Star as CAPSHPSD family `0x18`: build, move, Destroy System, the destroy cascade (fleets survive), victory bits, sabotage, and what a second superweapon hits |
| [character-roster.md](character-roster.md) | 130 | The recruit pick (minors by side), majors-only recruiters, major and minor classes, the mission-lead tier, and officer rank `+0x96` with its DAT eligibility |
| [record-registry.md](record-registry.md) | 109 | The generic DAT record loop, the record-factory and family-to-class registries, producer bands and research slots, special-force classes by id, and why an appended record fails |

## Reference Documents

| File | Lines | Content |
|------|-------|---------|
| [combat-formulas.md](combat-formulas.md) | ~180 | Binary overview, battle-session states, notification setters found from strings, largest functions in `0x4f0000`-`0x540000` with the note covering each, scripted-event strings |
| [COMBAT-SUMMARY.md](COMBAT-SUMMARY.md) | ~80 | Where each kind of combat is resolved, the object-move dispatcher `FUN_00514a60` and its hooks, the DatId family table |
| [system-squadrons-combat.md](system-squadrons-combat.md) | ~190 | System-held fighter squadrons: presence and the battle bit, export into the tactical engine after the fleets, losses through the import (`FUN_0040d260` → `FUN_00503370`), the post-battle pass `FUN_00516ef0` and the presence result `FUN_00517270` |

## Combat Subsystem Docs

| File | Subsystem | Key Functions |
|------|-----------|---------------|
| [space-combat.md](space-combat.md) | The MissionMgr story-flag word `+0x58` (`FUN_00549910`); it applies no damage (see story-missions.md, simulate-runs-tactical-engine.md) | FUN_00549910, FUN_00544030, FUN_00544130, FUN_005443f0, FUN_005444e0, FUN_005445d0 |
| [tactical-impact-effect-path.md](tactical-impact-effect-path.md) | Tactical hit, damage, and destruction sprite dispatch | FUN_005a7500, FUN_005d39a0, FUN_005d3e90, FUN_005d41a0 |
| [tactical-projectile-field-path.md](tactical-projectile-field-path.md) | Retained projectile geometry, interpolation, and tractor/gravity fields | FUN_005d3de0, FUN_005ee590, LAB_005eeb90, FUN_005d3ac0, FUN_005d3cc0 |
| [tactical-subsystem-field-command-path.md](tactical-subsystem-field-command-path.md) | Selected-capital subsystem bands and exact tractor/gravity source identity | FUN_005e45f0, FUN_005e7540, FUN_005e77c0, FUN_005b23e0 through FUN_005b25d0 |
| [tactical-subsystem-damage-path.md](tactical-subsystem-damage-path.md) | Live shield, hull, subsystem-hit, component-capacity, condition, and tractor-cancel rules | FUN_005b54d0, FUN_005b05c0, 0x005b1970, FUN_005b1770 through FUN_005b1bc0 |
| [tactical-subsystem-repair-mobility.md](tactical-subsystem-repair-mobility.md) | Subsystem repair cadence and selection plus engine and tractor mobility formulas | FUN_005b0330, FUN_005b1490, FUN_005b1ab0, FUN_005b17f0, FUN_005b16b0, FUN_005b1790 |
| [tactical-maneuver-movement.md](tactical-maneuver-movement.md) | Maneuver-state bonus, effective-power velocity, constructor direction state, and position integration | FUN_005ad750, FUN_005afb70, FUN_005b2f30, FUN_005cd640, FUN_005b0f70 |
| [tactical-attack-target-lifecycle.md](tactical-attack-target-lifecycle.md) | Typed attack-target invalidation, stable same-class replacement, and the separate weapon-loop boundary | FUN_005a7500, FUN_005a8c50 through FUN_005a8fc0, FUN_005d0b00, FUN_005b3a40, FUN_005b3f10 |
| [tactical-weapon-loop.md](tactical-weapon-loop.md) | Four capital battery arcs, family event order, range, energy queue, and recharge | FUN_005b05c0, FUN_005b3a40, FUN_005b3f10, FUN_005b6530, FUN_005b6320 |
| [tactical-fighter-combat.md](tactical-fighter-combat.md) | Fighter construction, float hull and shields, family and torpedo events, maneuver defense, and strategic return | FUN_005b9c60, FUN_005b49e0, FUN_005b7780, FUN_005b5100 through FUN_005b5f50 |
| [tactical-collision-formation.md](tactical-collision-formation.md) | Mesh collision envelope, strict overlap response, exact fighter-group assignment, and bounded formation facts | FUN_005ab0e0, FUN_005b2e60, FUN_005b2f30, FUN_005ae460, FUN_005c81d0 through FUN_005c83c0 |
| [tactical-death-star-path.md](tactical-death-star-path.md) | Separate Death Star object, laser path, source RNG, timed trench-run producer, ordered chatter, casualties, and exact 201/202 routing | FUN_005ba420, FUN_005ba5e0, FUN_005ba7f0, FUN_005cfec0, FUN_005d04e0, FUN_005d03f0, FUN_005ad7e0, FUN_0061a310 |
| [tactical-results.md](tactical-results.md) | The Battle Summary's per-tab rules: Operational, Destroyed, Survivors, Captured, Killed, with the import writer for each kind and the port's differences | FUN_0046f140, FUN_004714e0, FUN_00472010, FUN_0049b720, FUN_0040d260, FUN_0040cfc0 |
| [tactical-death-star.md](tactical-death-star.md) | The tactical Death Star: the station's export and import, the trench-run state machine, the roll `FUN_005d04e0`, fire and recharge, the shield `+0x70`, the commander slot `+0x950`, the hangar, retreat, the AI, an uncompleted station, films 201 and 202 | FUN_0040b0e0, FUN_005d04e0, FUN_005d0820, FUN_005ada10, FUN_005afe40, FUN_005c8780, FUN_005d14d0 |
| [tactical-selected-ship-panel.md](tactical-selected-ship-panel.md) | Selected capital-ship panel: four font-8 labels, hull and shield bars, ship art and contents list | FUN_005e4110, FUN_005e4880, FUN_005e4b40, FUN_005e4a30, FUN_0060e440 |
| [tactical-fighter-group-panel.md](tactical-fighter-group-panel.md) | Selected fighter-group panel (id 0x17): group name, order, tactics and count labels, art by side, buttons | FUN_005e1170, FUN_005e16a0, FUN_005e1fd0, FUN_005e1550, FUN_005c6530, FUN_005ab650 |
| [tactical-camera-holocube.md](tactical-camera-holocube.md) | The tactical camera's pose, the blue wireframe holocube, the highlight switches' start state and the Battle Paused label's colour | FUN_005d9640, FUN_005d4d10, FUN_005d58c0, FUN_0059eb70, FUN_0059c4d0, FUN_005c14d0 |
| [tactical-group-strips.md](tactical-group-strips.md) | Task-force and fighter-group strips: header and slot positions by side, 15-row squadron buttons, what a click or key 1-8 selects | FUN_0059be80, FUN_0059c030, FUN_0059b130, FUN_0059b450, FUN_0059b620, FUN_0059b730 |
| [observation-levels.md](observation-levels.md) | Observation levels 0-12 (`FUN_0050d5a0`): which object families and system fields a side's view receives per level; callers (informants, espionage, mission ends, Yavin start); when a view copy follows its master (the live mask, `FUN_004f9a60`) | FUN_0050d5a0, FUN_004f7e30, FUN_0050d510, FUN_00539fd0, FUN_004f9a60, FUN_00540630 |
| [side-numbering.md](side-numbering.md) | Side encodings: owner/view bits 1 Alliance, 2 Empire, 3 Neutral; player and AI session slots; tactical 0/1 via `FUN_0040d660` | FUN_004f8c60, FUN_00435bc0, FUN_0040d660, FUN_00595c40 |
| [simulate-runs-tactical-engine.md](simulate-runs-tactical-engine.md) | Every space battle is the battle session `FUN_0040a050`: pending-battle search, each side's Play/Simulate answer (the computer always simulates), the tactical engine windowed or headless, the 1200-second stalemate, write-back without shields; no other resolver | FUN_0040a050, FUN_004362b0, FUN_00485d50, FUN_00516d50, FUN_00596820, FUN_0059a530, FUN_005ae9c0, FUN_0040cfc0 |
| [morale-retreat.md](morale-retreat.md) | A tactical side withdraws when its strength is 0 or below its threshold `+0x24` times the enemy's, if `+0x80` (a strategic retreat system exists) permits; the computer's threshold is 0.9, or own/enemy attack at its guarded systems, 0 at its HQ; the player's is 0 | FUN_005d19a0, FUN_005d0f50, FUN_00516730, FUN_00599c90, FUN_004362b0, FUN_00485d50, FUN_004e6110 |
| [victory-conditions.md](victory-conditions.md) | Win flags at game root `+0x74` (`FUN_005180e0`; codes AD/LC/MC/CA/CD/DC/EC/EW/AW/GO): Standard and Headquarters Only rules, triggers, side objects and notification `0x128`, advisor hand-off, ending movies MDATA 105-108 | FUN_005180e0, FUN_00530a50, FUN_00518460, FUN_00532f90, FUN_00425c90, FUN_00414830, FUN_00421ba0 |
| [support-shifts.md](support-shifts.md) | Sector-wide popular-support shifts (records via `FUN_00518b20`, event `0x373`) after control changes, blockade ends, Alliance HQ removal and Death Star kills, with GNPRTB/SDPRTB `0x1400`-`0x1445`; `+0x88` bit 11 = the Death Star in the sector; the event queue's priority order and the support hook's recompute order | FUN_0055de50, FUN_0055dbf0, FUN_00518be0, FUN_0050c9f0, FUN_005178b0, FUN_0058e320, FUN_0054ee90, FUN_0054f140, FUN_005109f0, FUN_00510820, FUN_00508da0 |
| [ground-combat.md](ground-combat.md) | Character move hook: clears CommandKind `+0x96` of a moved character or of the characters aboard a moved capital ship; Emperor-at-Coruscant flag (family `0x34`); no ground combat | FUN_00560d50, FUN_004ee350, FUN_005617b0 |
| [game-end-path.md](game-end-path.md) | Game end: the single writer of object +0x50 bit 3, what removes the Alliance HQ (Destroy System container cascade, bombardment hit, capture), the advisor ending slots and codes 0x10-0x13, the computer player's progress counters, and the movie window's return to the main menu | FUN_004f7560, FUN_004fb520, FUN_004f8710, FUN_004f8680, FUN_004c3010, FUN_00425c90, FUN_004189b0, FUN_004138a0 |
| [bombardment.md](bombardment.md) | Bombardment orders 0x220-0x223: gate, Death Star Destroy System, KDY/LNR defence fire, per-hit rolls against military (incl. the Alliance HQ), civilian and resource targets, support shifts; the port's formula is the transit chain; a ship's remaining shields derived from its shield counter and hull damage (slot `+0x204`) | FUN_00582440, FUN_005637c0, FUN_0058d800, FUN_0058d990, FUN_0058dc10, FUN_0058df60, FUN_0058e320, FUN_00501160 |
| [death-star-product.md](death-star-product.md) | The Death Star as the shipyard product CAPSHPSD record 136: where an ordered capital ship lives from order to completion, the one-per-system room query (refusal `{0x90,3}`), the `0xf3` maintenance refusal, Sabotage and Death Star Sabotage target validators, the AI's build budget | FUN_005385f0, FUN_00552300, FUN_0057f540, FUN_005744c0, FUN_00551a20, FUN_00482330, FUN_00482560 |
| [ai-death-star.md](ai-death-star.md) | The AI's Death Star: the chain of gates from module 11's hull row to the posture emitter's Destroy System order `0x223`, each gate against the port | FUN_004ca5e0, FUN_004ca910, FUN_004ae240, FUN_00433780, FUN_00484630, FUN_0051ebb0, FUN_0052a2c0 |
| [ai-sabotage-target.md](ai-sabotage-target.md) | The AI's Sabotage and Death Star Sabotage planners: the six posture slot-12 queries, the target system and object draw, the defence figure by unit counts, staging, team and decoys (cap `(sys+0xbc + 2) / 2`) | FUN_004bda60, FUN_004ae480, FUN_004bdbc0, FUN_004bdcd0, FUN_00482330, FUN_00433e40 |
| [ground-assault.md](ground-assault.md) | Planetary Assault (order 0x234): the menu row's enable test, the fleet and system flags, the run (count, defence fire, one engagement per attacking regiment, collateral, landing, cleanup), its GNPRTB values and draws, control change, the report messages and the AI's order | FUN_00563390, FUN_0058c1e0, FUN_0058c120, FUN_0058c3c0, FUN_0058c580, FUN_0058c810, FUN_0055eb00 |
| [jail.md](jail.md) | Jail abodes (family 0x98): a captive takes its captor's side bits, the 1-day trip, the Personnel Finder row, the mission-end way in, escape on timer 0x38a, Rescue targeting, the final battle's Move | FUN_00549580, FUN_004f1a80, FUN_00543040, FUN_00555b30, FUN_00556430, FUN_00561990, FUN_0054b040 |
| [mission-return.md](mission-return.md) | The mission return path: each member's need, route and check (`FUN_00521220`), the hidden Return mission `0x42000002`, the four policy cases and the side-specific items | FUN_00548120, FUN_00521220, FUN_00542ee0, FUN_005475f0, FUN_00547ba0, FUN_00548910 |
| [victory-flag-word.md](victory-flag-word.md) | Every bit of the win word at root +0x74 for both game types, the leader ids, the tie rule, why the major characters cannot die, when Rebel HQ Destroyed and Coruscant Captured file, the ending side by side, the game type's readers; the message kind chooser files on word edges | FUN_005180e0, FUN_00505190, FUN_00505230, FUN_0048e5a0, FUN_0048f790, FUN_004c3010, FUN_00576620, FUN_00488de0 |
| [blockade-troop-withdrawal.md](blockade-troop-withdrawal.md) | Regiments lost running a blockade: withdraw percent, per-regiment copy, departure roll, event 0x340 | FUN_0050b310, FUN_0055a020, FUN_00504990, FUN_00504a00, FUN_00508660 |
| [facility-ownership.md](facility-ownership.md) | Facility sides: seeded facilities take their system's side; a system's side change hands its completed facilities to the new side or removes those whose class cannot serve it; contested systems keep their holder; the HQ is not a yard; mine 0x2c, refinery 0x2d; the holder rule `FUN_0050b610` (both sides present keep the earlier troop side) and every reader of the holder | FUN_00566de0, FUN_0056a6f0, FUN_00559850, FUN_00559a60, FUN_00510d70, FUN_004f6f40, FUN_00510820, FUN_004fae40, FUN_004f8680, FUN_004f27d0, FUN_0050e820, FUN_005267c0, FUN_0050b610 |
| [blockade-bit.md](blockade-bit.md) | The system blockade bit (+0x88 0x20): its rule beside the battle bit, when it reruns, and why the Move confirmation only meets a stale bit; event priorities | FUN_0050b8e0, FUN_0050a5f0, FUN_00509710, FUN_005131d0, FUN_00515ef0 |
| [uprising-incident.md](uprising-incident.md) | Table ids to DAT files; system incident bits; uprising lifecycle, incident score and outcome codes; Subdue support gain; disaster erosion; mission, decoy, foil, escape table consumers | FUN_0058b420, FUN_0050b800, FUN_0050c910, FUN_00559ce0, FUN_0050d030, FUN_0050d150, FUN_0055cb10, FUN_00511930, FUN_00559e10 |
| [decoy-roll.md](decoy-roll.md) | Mission member lists (team, decoy, captured); the six resolution phases; decoy roll per defender (TDECOYTB/FDECOYTB, GNPRTB 3588) and FOILTB detection (GNPRTB 3584/3589); officer rank per detector; mission phases 0..0xb, the 0x4112 mode word, phase-10 per-member MSTB success roll, mission classes and record flags, decoy assignment | FUN_005898f0, FUN_00589a40, FUN_0058a020, FUN_0058a130, FUN_0058a1c0, FUN_00589620, FUN_0055e410, FUN_005888f0, FUN_005236e0, FUN_00521980, FUN_005227d0, FUN_00592f50, FUN_0054bb90 |
| [mission-lifecycle.md](mission-lifecycle.md) | Phase stepping and the ready bit, transit wait (phase 4) and timer 0x38b (phase 8, MISSNSD min + rand(spread)), repeat 10→8, the MISSNSD +0x28 record shift, per-class slots +0x274..+0x284 with first-read outcomes | FUN_005227d0, FUN_00524b70, FUN_00522980, FUN_00522280, FUN_00522a10, FUN_005236e0, FUN_00592f50, FUN_00592c80 |
| [ai-order-emission.md](ai-order-emission.md) | Every order the computer player sends (114 `FUN_004f5cd0` sites): 14 task modules with weights, goal bits and phases (Build/Destination/Scrap/Retire/Rename, Alliance HQ move), fleet tasks, posture emitter (Planetary Assault, Destroy System, bombardments), character Command tasks, planner Move and Abort; the port's four AI actions against them | FUN_004f5cd0, FUN_004bea20, FUN_004b1070, FUN_004ae320, FUN_00474390, FUN_004774c0, FUN_004bbf50, FUN_0047b250, FUN_004c05e0 |
| [story-missions.md](story-missions.md) | The story: MissionMgr flag word `+0x58` (bits 0-14: Dagobah, Jabba's Palace, Pickup, final battle, bounty hunters) and its checks, `+0xb0` character flags, ABODESD places `0x98000440`-`0x98000481`, the six story missions (Jedi Training, Palace, Bounty, Dagobah, Vacation, Pickup) and the final battle's Force test | FUN_00549910, FUN_00544030, FUN_005445d0, FUN_00544da0, FUN_0054b040, FUN_00546fa0, FUN_005712b0, FUN_005751d0, FUN_0056e650, FUN_00575de0, FUN_0056da80 |
| [research.md](research.md) | Research mission `0x53` (ship, facility, troop trees `0x53000020`-`22`): per-member skill rolls into per-side point pools, timer `0x381` points, levels bought at the next order with a class, unlock at order <= level | FUN_0056cdb0, FUN_0056cf30, FUN_00530460, FUN_00567ba0, FUN_005326f0, FUN_0052e580 |
| [ship-damage-repair.md](ship-damage-repair.md) | Capital-ship damage state (hull damage, six 4-bit counters, the damaged bit), the repair timer `0x380` (2 or 30 days, tenth of the hull, damage-control roll), the fast-repair flag from a counted shipyard, how `FUN_00484630` scales the AI ship record, and the AI's repair move `FUN_004c8e90` | FUN_00501c50, FUN_00501c00, FUN_005018c0, FUN_005578d0, FUN_0050c580, FUN_00484630, FUN_004c8e90 |
| [ai-mission-planning.md](ai-mission-planning.md) | Who fills mission team and decoy lists: mission orders 0x240..0x242 into command 0x250, the player dialog's team/decoy buttons, AI planners per kind, the candidate filter, decoy cap (+0xbc + 2) / 2 | FUN_0046c3c0, FUN_004f4a00, FUN_004f4b60, FUN_0042f830, FUN_004bced0, FUN_0047b360, FUN_0047d720, FUN_00403460 |
| [object-state-flags.md](object-state-flags.md) | `+0x50` state bits (usable, created, completed, destroyed, en route, existing) and the direct-child iterator modes | FUN_004f7410, FUN_004f6b90, FUN_005131d0, FUN_00513120, FUN_004fe540 |
| [regiment-unload.md](regiment-unload.md) | Unloading a regiment by hand: drop targets, the group and command refusals (0x28 only toward another side, populated-planet rule), regiment speed GNPRTB 1, immediate same-system container change | FUN_00556390, FUN_00555920, FUN_0053d430, FUN_004f63f0, FUN_004f8630, FUN_004f6fd0 |
| [fleet-join-split.md](fleet-join-split.md) | Joining and splitting fleets: a move onto a fleet hands over its capital ships, an emptied fleet disbands, Create Fleet (0x270) fills a system's hidden spare fleet, a fleet holds only capital ships | FUN_004feca0, FUN_004ffc90, FUN_004fe630, FUN_0050be00, FUN_00512d00, FUN_00580b00, FUN_00509b40 |
| [fleet-finder.md](fleet-finder.md) | The Fleet Finder (window type 0x15, 470x330): cockpit command 0x12e or F3, side tabs, name box with prefix match, a name-sorted list of the side's known fleets or ships, Display and double click open the Sector and Fleet windows | FUN_0042a0c0, FUN_00461750, FUN_00461960, FUN_00462be0, FUN_00462770, FUN_00462a50, FUN_00429440 |
| [sector-window-placement.md](sector-window-placement.md) | Sector windows: two at most, one per column; the first takes its sector's galaxy half (width 1023), a second the free column, a third replaces the window on its half | FUN_00429ce0 |
| [strategic-windowing-toolkit.md](strategic-windowing-toolkit.md) | Shared strategic-window contract: keyed identity registry, separate presentation MRU, modeless stacking, modal boundary, child focus, temporary capture, asynchronous close, and restoration | FUN_00422ce0, FUN_00600310, FUN_006007b0, FUN_00601340, FUN_00606960 |
| [strategic-windowing-runtime-evidence.md](strategic-windowing-runtime-evidence.md) | Sanitized, incomplete original-runtime evidence ledger and required capture queue for strategic stacking, modality, focus, and restoration | Owned REBEXE.EXE under Wine 9.0; corroboration only until exact actions and hashes are retained |
| [troop-finder.md](troop-finder.md) | The Troop Finder (window type 0x16, 470x330): command 0x130 or F4, Alliance and Imperial tabs, a name-sorted list of systems and fleets holding the tab's regiments with five count columns, opening the Defenses or Fleet window | FUN_0042a4d0, FUN_0046ce40, FUN_0046ea10, FUN_0046d8d0, FUN_0046df90, FUN_00429440 |
| [personnel-finder.md](personnel-finder.md) | The Personnel Finder (window type 0x17): command 0x12f or F5, side tabs, Characters and SpecForces views, the "Name - Location ( State ) " row text, open targets by object family | FUN_0042a180, FUN_00463500, FUN_00465bb0, FUN_00465540, FUN_00429440 |
| [production-destination.md](production-destination.md) | Per-area production managers (families 0xa0..0xaf), the Destination order 0x214 and where finished builds go | FUN_0052c170, FUN_00512700, FUN_0052b960, FUN_0052bee0, FUN_0055d8c0 |
| [manufacturing-build-selection.md](manufacturing-build-selection.md) | Manufacturing window type 9 and its 210x261 Build Selection child: pages, tabs, producer bands and their text, side art, band menus (Build/Stop/Destination/Rename/Reserved), facility item states, build-list filtering, exact controls, costs, timing fields, quantity bounds, and command routes | FUN_00452fc0, FUN_00455060, FUN_004568a0, FUN_00458480, FUN_00458080, FUN_00457c90, FUN_00456230, FUN_0052ae30, FUN_00437880, FUN_00437f80, FUN_00438620, FUN_00438800, FUN_00439160, FUN_00537ff0 |
| [manage-automation.md](manage-automation.md) | Manage Garrisons and Manage Production: module states (2 suspended), the toggle counter and bits, the per-cycle order cap, menu checkmarks | FUN_00439d60, FUN_00439e30, FUN_004cc990, FUN_00439950, FUN_00487900 |
| [message-index-rows.md](message-index-rows.md) | Message Index rows, selection, Delete (0x91), Display (0x65), the window's Close (0x28), and the Advice category's speed hold | FUN_004665f0, FUN_00468ab0, FUN_00468f20, FUN_00468fb0, FUN_004697b0, FUN_00469de0, FUN_00487ff0 |
| [agent-advice.md](agent-advice.md) | Agent Advice: bit 0x8000 of DAT_006b28b0 (set is off), its difficulty default, the menu toggle, the Advice category's speed hold | FUN_00439320, FUN_00439e80, FUN_00487ff0, FUN_00439d60 |
| [rename-order.md](rename-order.md) | Rename (0x203): the in-place name field, Enter commits a non-empty name, no length cap, the object's own name else its class name | FUN_004ac950, FUN_004f6270 |
| [agent-menu.md](agent-menu.md) | The Agent menu (STRATEGY list 0xdead, items 0x110..0x11e): right click on the advisor panel, labels, enabled and checked items, Alt+G/U/A accelerators | FUN_004420b0, FUN_00487900, FUN_00422ce0 |
| [status-window.md](status-window.md) | The Status window (type 0x1a, 379x272): object-menu Status 0x103 posts 0x468, the per-family fillers, the Character Status rows, the "Attached: " version check, the Encyclopedia and Close buttons | FUN_00442d70, FUN_00443130, FUN_004486f0, FUN_0042a440, FUN_00486fb0, FUN_00449e00, FUN_0044a2e0, FUN_00406850 |
| [galaxy-view-input.md](galaxy-view-input.md) | The galaxy view's mouse messages: no wheel, no right press, no pan, so the star map is fixed | FUN_00422ce0 |
| [build-delivery.md](build-delivery.md) | Build queue products, completion, per-object transit time `max(1, isqrt(d^2) / GNPRTB 5120 * speed / 100)`, arrival event 0x387; other users of the distance formula | FUN_00555b30, FUN_0052b960, FUN_0052bee0, FUN_00514a60, FUN_00556430, FUN_0055d8c0, FUN_0057be20, FUN_004fb520 |
| [timer-scheduler.md](timer-scheduler.md) | Frame loop and setup sequence, the day scheduler, timers 0x380 to 0x394 with their handlers; refutes the 0x1f0 master-tick claim | FUN_0040a050, FUN_005136d0, FUN_0041dff0, FUN_0051df30, FUN_00586130, FUN_005862a0 |
| [community-address-remap.md](community-address-remap.md) | The community disassembly is a different REBEXE build; region shifts and 43 remapped functions | FUN_00508250, FUN_0050b310, FUN_00559fe0, FUN_0055e410 |

## Interface, Advisor and Tactical Recovery

| File | Subsystem | Key Functions |
|------|-----------|---------------|
| [ai-production.md](ai-production.md) | The side AI's production modules: the per-system record, the goal token and facility modules 3, 1 and 2 (part F), the unit modules 11, 10, 7, 5 and 6 (part U); UNSURE and MISSING mark what the decompile leaves open | FUN_00433e40, FUN_004d9e70, FUN_00419b80, FUN_0041b230, FUN_0041a340, FUN_00509670 |
| [ai-behavior-analysis.md](ai-behavior-analysis.md) | The new-game stage `0xb` and the per-family dispatch checks that precede the computer player (the player itself is in ai-order-emission.md) | FUN_00519d00, FUN_00537180, FUN_005385f0, FUN_00502020, FUN_00508250, FUN_00508660 |
| [battle-alert-lists.md](battle-alert-lists.md) | The pre-battle alert's three list pages: what each lists, the row layout, the list picture and its RCDATA scroll bar | FUN_0044f860, FUN_00450770, FUN_00450e10, FUN_00450660, FUN_0060a490 |
| [battle-result-text-and-cadence.md](battle-result-text-and-cadence.md) | The battle result object and the text of its title, status and body, and the strategic re-battle cadence while both sides remain | FUN_0049ac00, FUN_00516ef0, FUN_00517270, FUN_0049b150, FUN_0049b330, FUN_0049b4d0 |
| [message-display.md](message-display.md) | How a message gets its title, body, picture and sound through the TEXTSTRA formatter, the Message Index's full-size mode 2, the message classes, and the Battle Alert's summary choice | FUN_0060b9d0, FUN_0060b840, FUN_00469de0, FUN_00499760, FUN_0049c0d0 |
| [operation-report-window.md](operation-report-window.md) | The Operation Report (message class 0xf) for bombardment `0x182` and assault `0x183`: the 470x331 window, its rail and six category tabs, what fills each list, picture and text choice, the live alert and the Message Index reopen; the Fleet button's target, caption layout and font, the window's modality and the 0x468/0x482 wait | FUN_00496770, FUN_0044c630, FUN_0044c410, FUN_00496f50, FUN_00496460, FUN_0048a060, FUN_0044df90, FUN_0045c8e0, FUN_00601340, FUN_006026e0 |
| [droid-advisor-triggers.md](droid-advisor-triggers.md) | The cockpit droids: the two players per side, the command queue, the reaction slots, the SPT action tables, the message codes that fill them and idle chatter | FUN_0042adb0, FUN_00441a60, FUN_004727e0, FUN_00472930, FUN_004729c0 |
| [fleet-window.md](fleet-window.md) | The Fleet window (type 4): how it opens, geometry, lists and tabs, hit tests, and loading a regiment onto a fleet | FUN_004a2630, FUN_004ac120, FUN_0045aac0, FUN_004a4b10, FUN_004a5c00 |
| [fleet-names.md](fleet-names.md) | Default fleet and ship names ("Fleet 1", "Galleon 2"): the per-side counter per record that never goes down | FUN_00517760, FUN_005302c0, FUN_004f6e60, FUN_00518750 |
| [move-order.md](move-order.md) | The Fleet pop-up menu, Move (`0x201`) and Confirmed Move (`0x202`), the targeting release, the per-object move command, hit tests, the confirmation window and a drag as a move | FUN_0053c4b0, FUN_00487cc0, FUN_00487740, FUN_00553410, FUN_0053d430, FUN_004f9860 |
| [facility-order-lists.md](facility-order-lists.md) | The orders each facility family offers through `GetOrderList` (vtable `+0x3c`): Scrap alone, plus Encyclopedia and Status, against the combat units' lists | FUN_005399e0, FUN_00539c20, FUN_0051d990, FUN_00520690, FUN_00520600 |
| [squadron-menu-and-transit.md](squadron-menu-and-transit.md) | A planet-held squadron's right-click menu, and where a squadron or regiment lives while it travels between systems | FUN_00503930, FUN_00557ce0, FUN_00558380, FUN_00509710 |
| [sector-quadrants.md](sector-quadrants.md) | The four overlay items around each planet in the sector window: show rules, art, the windows they open (types 9, 10, 4, 11), and the System Defenses and Missions windows | FUN_00459e30, FUN_0045cdc0, FUN_0045ce80, FUN_0045ccc0, FUN_004a7790, FUN_0049f130 |
| [sector-icon-menus.md](sector-icon-menus.md) | How a press on a quadrant icon selects it, which orders its right-click menu lists per kind, and what each order acts on | FUN_004593e0, FUN_0045b1b0, FUN_00507290, FUN_0050f2e0, FUN_00512700 |
| [sector-window-hit-test.md](sector-window-hit-test.md) | The sector window's item structure, hit rect formula, container (move target) and object (selection) hit tests, overlay items and label rect | FUN_004591d0, FUN_0045c830, FUN_0045c6b0, FUN_00459e30, FUN_0045c1f0 |
| [galaxy-view-framing.md](galaxy-view-framing.md) | The galaxy view's backdrop bitmap, system coordinate transform, galaxy-size filter, GID caption, font table and sector icon resources | FUN_00422ce0, FUN_00427010, FUN_00425d00, FUN_0042d230 |
| [gid-legend.md](gid-legend.md) | The Galaxy Display's compact and detailed legends (STRATEGY draw scripts, darkened window, drag) and the GID menu's highlight colors | FUN_00427270, FUN_00452ab0, FUN_00426d00, FUN_004522f0, FUN_00452090 |
| [top-bar-resource-counters.md](top-bar-resource-counters.md) | The three top-bar numbers: raw and refined stockpiles (`+0x78`, `+0x7c`) and maintenance (`+0x58` less `+0x74`), from paint path to stockpile | FUN_00422620, FUN_0052fff0, FUN_0052fb30, FUN_0052f8f0, FUN_0052f6b0 |
| [maintenance-and-vader-start.md](maintenance-and-vader-start.md) | How maintenance capacity and load are computed, and where Darth Vader starts | FUN_0052fff0, FUN_0055f7b0, FUN_00562710, FUN_005323c0 |
| [starting-forces-seeding.md](starting-forces-seeding.md) | The seven steps of the new-game force seeding and the budget pass that spends each side's spare maintenance on CMUNALTB/CMUNEMTB bundles | FUN_0051aa50, FUN_0051ab20, FUN_0051b1f0, FUN_0051bf30 |
| [game-options.md](game-options.md) | The unified Game Options window: entry command `0x68` and F1 `0x133`, controls, context and mutation boundaries, evidence gaps | FUN_00406000, FUN_00406950, FUN_004084b0, FUN_004082b0 |
| [keyboard-shortcuts.md](keyboard-shortcuts.md) | Every key and mouse modifier of the original, by screen: TEXTCOMM accelerator table (33 entries), galaxy F-keys, tactical camera and group keys, dialog, movie and text-field keys, manual page and status per row, VM check list | FUN_00422ce0, FUN_0059b730, FUN_00413560, FUN_00421c70, FUN_00605160, FUN_0041efe0, FUN_00461590, FUN_00463360, FUN_0045fe60 |
| [capital-ship-save-layout.md](capital-ship-save-layout.md) | The original `.sav` capital-ship record (node type `0x14`): the long and fog records, class codes, header and payload fields, from four original saves | FUN_006173b0, FUN_005f5610, FUN_005405a0 |
| [tactical-engagement-ai.md](tactical-engagement-ai.md) | How each side's task forces pick targets by tactic and close to weapon range, the priority list, damage rate, side strength, and what a stalemate does to the loser | FUN_005c9200, FUN_005cc300, FUN_005edf80, FUN_005d1a70, FUN_005b7780, FUN_005d0ce0 |
| [tactical-attack-targeting.md](tactical-attack-targeting.md) | The first execution step of Attack Fighters and Attack Capital Ships, and the implementation boundary | FUN_005c91e0, FUN_005c91c0, FUN_005d0bb0, FUN_005d0b10 |
| [tactical-command-assignment.md](tactical-command-assignment.md) | The selected-unit Maneuvers and Tactics and Missions panels on the 640 by 480 tactical canvas, from the command container at (482, 28) | FUN_0059cac0, FUN_005c5520, FUN_005c55a0, FUN_005df110, FUN_005dd3a0 |
| [tactical-command-progression.md](tactical-command-progression.md) | Capital turn and movement, maneuver arrival and fighter recovery state transitions, and the acceptance boundary | FUN_005cf980, FUN_005cf940, FUN_005cf8f0, FUN_005cf190, FUN_005cd640 |
| [tactical-planet-resource-path.md](tactical-planet-resource-path.md) | The chain that selects a battle's tactical planet resource from the current system object | FUN_0040b0e0, FUN_00509610, FUN_005c2e60, FUN_0059a850 |

## Catalog and Review Documents

| File | Content |
|------|---------|
| [FUNCTION_INDEX.md](FUNCTION_INDEX.md) | Auto-generated catalog of 5,151 decompiled C files by game system, the 20 largest functions, and the functions the scholar documents annotate (generated 2026-03-21; the files in this directory now number more) |
| [review-annotated-functions.md](review-annotated-functions.md) | Cross-check of the `annotated-functions.md` claims against the decompiled C: verified entries and errors found |
| [review-combat-docs.md](review-combat-docs.md) | Spot-check of the offsets, vtable slots and event ids in `cpp-class-hierarchy.md`, `rust-implementation-guide.md` and `modders-taxonomy.md`; the readings the decompiles support |
| [review-new-docs.md](review-new-docs.md) | Cross-reference audit of 46 claims across `entity-system.md`, `mission-event-cookbook.md` and `economy-systems.md`, with accuracy scores |

## Function Notes (7,078 C files)

### By Game System

| System | Address Range | Key Functions | Decompiled |
|--------|--------------|---------------|------------|
| Game init / CRT | 0x401000-0x403e90 | entry, CRT boilerplate | ~100 |
| Galaxy map rendering (GDI) | 0x422000-0x43a000 | FUN_00422ce0 (11K), FUN_00433e40 (6K) | ~50 |
| UI dialogs / windows | 0x43a000-0x470000 | FUN_0044c630 (6K), FUN_004665f0 (6K); [Encyclopedia mapping](encyclopedia-window.md), [Message Index mapping](message-index-window.md), [Mission dialog](mission-dialog.md), [Object pop-up menu and targeting](object-popup-menu.md) | ~200 |
| Game logic / turn processing | 0x490000-0x4a0000 | FUN_004927c0 (9K) | ~50 |
| Character system | 0x4ee000-0x4f4000 | Enhanced skills, Force, loyalty | ~80 |
| Game object base | 0x4f4000-0x500000 | Faction handler, fleet events, mission destroy | ~60 |
| Capital ship combat | 0x500000-0x510000 | Hull/shield/weapon validators, damage setters | ~120 |
| System control | 0x510000-0x530000 | Battle/blockade/uprising/loyalty control, economy | ~150 |
| Side / victory | 0x530000-0x540000 | Victory conditions, recruitment, base skills | ~80 |
| MissionMgr story checks | 0x540000-0x560000 | The `+0x58` flag word and its per-side checks (story-missions.md) | ~60 |
| Bombardment / ground | 0x550000-0x570000 | Bombardment formula, ground combat, repair | ~80 |
| Mission manager | 0x570000-0x580000 | Espionage, scripted events, Jedi training | ~40 |
| Tactical combat | 0x5a0000-0x5b0000 | FUN_005a7500 (4.8K), ship constructor | ~30 |
| DAT / GNPRTB loaders | 0x569000-0x590000 | Type registry, parsers, GNPRTB binding | ~30 |
| Ship database | 0x597000-0x598000 | FUN_00597610 (9K), all ship names | 1 |
| Networking / multiplayer | 0x5f0000-0x610000 | DirectPlay, CommMgr, latency config | ~30 |
| UI controls | 0x600000-0x610000 | Slider, drag list, strobe button | ~20 |
| CRT / runtime | 0x610000-0x660000 | Exception handling, memory, string ops | ~200+ |

### GNPRTB Parameter Functions

| Function | Lines | Purpose |
|----------|-------|---------|
| FUN_0053e450 | 240 | General parameter binding (34 bindings: 28 base + 6 per-side, IDs 0x0a00-0x0a21) |
| FUN_0055cb60 | 84 | Combat parameter binding (77 params: 25 base + 52 per-side, IDs 0x1400-0x1445) |
| FUN_00585640 | 27 | GNPRTB entry constructor (68-byte runtime struct, 8 i32 values) |
| FUN_00569280 | 12 | DAT type registry (5 parser types by info string) |
| FUN_0053e390 | — | Parameter → global address binder |
| FUN_0053e3e0 | — | Per-side parameter → global address binder |

### Validation Functions ("Invalid X value!")

| Function | Field | Offset | Range |
|----------|-------|--------|-------|
| FUN_00501490 | HullValueDamage | +0x60 | 0 to vtable+0x248 max |
| FUN_00501510 | ShieldRechargeRate | +0x64 bits 0-3 | 0-15 (4-bit nibble) |
| FUN_005015a0 | WeaponRechargeRate | +0x64 bits 4-7 | 0-15 (4-bit nibble) |
| FUN_005032c0 | SquadSizeDamage | +0x60 (polymorphic) | 0 to vtable+0x244 max |
| FUN_004ee030 | EnhancedLoyalty | +0x8a | 0 to 0x7fff |
| FUN_004ee470 | MissionHyperdriveModifier | +0x9a | 0 to unbounded |
| FUN_005341a0 | BaseLoyalty | +0x66 | 0-100 |
| FUN_00509cc0 | SystemEnergy | — | — |
| FUN_00509d40 | SystemEnergyAllocated | — | — |
| FUN_00509dc0 | SystemRawMaterial | — | — |
| FUN_00509e40 | SystemRawMaterialAllocated | — | — |

## Ghidra Scripts

| Script | Purpose | Output |
|--------|---------|--------|
| FindAllFunctions.py | Scan .text for x86 prologues, create functions | Console: before/after count |
| DumpStrings.py | Search all strings by keyword → file | ~/Desktop/rebellion-strings.txt |
| DumpCombatXrefs.py | Trace combat string → function xrefs | ~/Desktop/rebellion-combat-xrefs.txt |
| DumpCallers.py | Find direct callers (confirmed virtual dispatch) | ~/Desktop/rebellion-callers.txt |
| DumpCombatRegion.py | List all functions in 0x4f0000-0x540000 | ~/Desktop/rebellion-combat-region.txt |
| FindCombatMath.py | Search for combat math patterns | ~/Desktop/rebellion-combat-math.txt |
| DumpAllGameFunctions.py | Exhaustive 4,938-function catalog with strings | ~/Desktop/rebellion-all-functions.txt |
| DumpGNPRTBXrefs.py | Trace GNPRTB globals to consuming functions | ~/Desktop/rebellion-gnprtb-xrefs.txt |
