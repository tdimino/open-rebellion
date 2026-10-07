---
title: "Ghidra RE Notes — Index"
description: "Master index of 5,408 decompiled C files (5,207 by address, 201 named copies), Ghidra scripts, and recovered subsystem notes"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-06
---

# Ghidra RE Notes — Index

**5,408 decompiled C files (5,207 `FUN_<address>.c` plus 201 named copies), Ghidra scripts, and indexed recovery notes**

Provenance: until 2026-09-26, 2,752 `FUN_<address>.c` files were empty placeholders from commit aa2b333. They were filled, and 188 functions cited elsewhere in the repo were added, with Ghidra 12.1.3 headless `CreateAndDecompileTargets.java` against the read-only project. 29 of those files explain instead of decompiling: 27 addresses lie inside another function and name it, 1 (`0x005587d0`) is a jump stub outside any function, and 1 records the only decompiler failure (`FUN_006197d2`, "Overlapping input varnodes"). No note is empty.

## Scholar Documents

| File | Lines | Content |
|------|-------|---------|
| [annotated-functions.md](annotated-functions.md) | 1,662 | Struct layouts (+0x60 hull, +0x64 shield/weapon nibbles, +0x96 strength, +0xac alive), renamed variables, 50 event IDs (0x127-0x370), game rules |
| [modders-taxonomy.md](modders-taxonomy.md) | 805 | 10 game systems categorized for total conversion mods (Yuuzhan Vong, Thrawn, KOTOR). Function addresses, GNPRTB params, mod guidance per system |
| [rust-implementation-guide.md](rust-implementation-guide.md) | 1,267 | Maps decompiled C to Open Rebellion's `advance()` pattern. `CombatSystem::resolve_space()`, `GnprtbParams`, `MstbTable`, 15 new CapitalShipClass fields |
| [cpp-class-hierarchy.md](cpp-class-hierarchy.md) | 445 | CRebObject → CNotifyObject → CCombatUnit hierarchy. 19 vtable slots, 6 vtable pointer constants, complete field layout, setter-notify-event pattern |
| [entity-system.md](entity-system.md) | 668 | Characters (8 enhanced skills, Force/Jedi 6-tier system, betrayal), game objects (5 destruction variants), fleets (4 events), factions (Alliance/Empire/Neutral bits) |
| [mission-event-cookbook.md](mission-event-cookbook.md) | 724 | 9+ mission types (FUN_0050d5a0 13-case switch), 4 story event chains (Dagobah, Vader, Palace, Bounty), event ID registry (0x12c-0x370), Thrawn Campaign example |
| [economy-systems.md](economy-systems.md) | 478 | Resources (energy/material at +0x5c-0x68), 5 ControlKind states, uprising (UPRIS1TB/2TB), blockade manufacturing halt, repair (GNPRTB 0x305/0x386), 36 system notif handlers |

## Reference Documents

| File | Lines | Content |
|------|-------|---------|
| [combat-formulas.md](combat-formulas.md) | ~200 | **Master reference** — binary overview, 111 GNPRTB mappings, confirmed functions, scripted events, Ghidra scripts |
| [COMBAT-SUMMARY.md](COMBAT-SUMMARY.md) | ~115 | Combat call chain diagram, confirmed formulas, entity type codes, implementation readiness |

## Combat Subsystem Docs

| File | Subsystem | Key Functions |
|------|-----------|---------------|
| [space-combat.md](space-combat.md) | Space combat auto-resolve — 7-phase pipeline | FUN_00549910, FUN_00544030, FUN_00544130, FUN_005443f0, FUN_005444e0, FUN_005445d0 |
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
| [ground-combat.md](ground-combat.md) | Ground combat — troop iteration + per-unit resolution | FUN_00560d50, FUN_004ee350, FUN_005617b0 |
| [bombardment.md](bombardment.md) | Orbital bombardment — Euclidean distance formula | FUN_00556430, FUN_0055d8c0, FUN_0055d860 |
| [blockade-troop-withdrawal.md](blockade-troop-withdrawal.md) | Regiments lost running a blockade: withdraw percent, per-regiment copy, departure roll, event 0x340 | FUN_0050b310, FUN_0055a020, FUN_00504990, FUN_00504a00, FUN_00508660 |
| [blockade-bit.md](blockade-bit.md) | The system blockade bit (+0x88 0x20): its rule beside the battle bit, when it reruns, and why the Move confirmation only meets a stale bit | FUN_0050b8e0, FUN_0050a5f0, FUN_00509710, FUN_005131d0, FUN_00515ef0 |
| [uprising-incident.md](uprising-incident.md) | Table ids to DAT files; system incident bits; uprising lifecycle, incident score and outcome codes; Subdue support gain; disaster erosion; mission, decoy, foil, escape table consumers | FUN_0058b420, FUN_0050b800, FUN_0050c910, FUN_00559ce0, FUN_0050d030, FUN_0050d150, FUN_0055cb10, FUN_00511930, FUN_00559e10 |
| [decoy-roll.md](decoy-roll.md) | Mission member lists (team, decoy, captured); the six resolution phases; decoy roll per defender (TDECOYTB/FDECOYTB, GNPRTB 3588) and FOILTB detection (GNPRTB 3584/3589); officer rank per detector; mission phases 0..0xb, the 0x4112 mode word, phase-10 per-member MSTB success roll, mission classes and record flags, decoy assignment | FUN_005898f0, FUN_00589a40, FUN_0058a020, FUN_0058a130, FUN_0058a1c0, FUN_00589620, FUN_0055e410, FUN_005888f0, FUN_005236e0, FUN_00521980, FUN_005227d0, FUN_00592f50, FUN_0054bb90 |
| [mission-lifecycle.md](mission-lifecycle.md) | Phase stepping and the ready bit, transit wait (phase 4) and timer 0x38b (phase 8, MISSNSD min + rand(spread)), repeat 10→8, the MISSNSD +0x28 record shift, per-class slots +0x274..+0x284 with first-read outcomes | FUN_005227d0, FUN_00524b70, FUN_00522980, FUN_00522280, FUN_00522a10, FUN_005236e0, FUN_00592f50, FUN_00592c80 |
| [ai-mission-planning.md](ai-mission-planning.md) | Who fills mission team and decoy lists: mission orders 0x240..0x242 into command 0x250, the player dialog's team/decoy buttons, AI planners per kind, the candidate filter, decoy cap (+0xbc + 2) / 2 | FUN_0046c3c0, FUN_004f4a00, FUN_004f4b60, FUN_0042f830, FUN_004bced0, FUN_0047b360, FUN_0047d720, FUN_00403460 |
| [object-state-flags.md](object-state-flags.md) | `+0x50` state bits (usable, created, completed, destroyed, en route, existing) and the direct-child iterator modes | FUN_004f7410, FUN_004f6b90, FUN_005131d0, FUN_00513120, FUN_004fe540 |
| [regiment-unload.md](regiment-unload.md) | Unloading a regiment by hand: drop targets, the group and command refusals (0x28 only toward another side, populated-planet rule), regiment speed GNPRTB 1, immediate same-system container change | FUN_00556390, FUN_00555920, FUN_0053d430, FUN_004f63f0, FUN_004f8630, FUN_004f6fd0 |
| [fleet-join-split.md](fleet-join-split.md) | Joining and splitting fleets: a move onto a fleet hands over its capital ships, an emptied fleet disbands, Create Fleet (0x270) fills a system's hidden spare fleet, a fleet holds only capital ships | FUN_004feca0, FUN_004ffc90, FUN_004fe630, FUN_0050be00, FUN_00512d00, FUN_00580b00, FUN_00509b40 |
| [fleet-finder.md](fleet-finder.md) | The Fleet Finder (window type 0x15, 470x330): cockpit command 0x12e or F3, side tabs, name box with prefix match, a name-sorted list of the side's known fleets or ships, Display and double click open the Sector and Fleet windows | FUN_0042a0c0, FUN_00461750, FUN_00461960, FUN_00462be0, FUN_00462770, FUN_00462a50, FUN_00429440 |
| [sector-window-placement.md](sector-window-placement.md) | Sector windows: two at most, one per column; the first takes its sector's galaxy half (width 1023), a second the free column, a third replaces the window on its half | FUN_00429ce0 |
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
| [build-delivery.md](build-delivery.md) | Build queue products, completion, per-object transit time `max(1, isqrt(d^2) / GNPRTB 5120 * speed / 100)`, arrival event 0x387; corrects the bombardment misread | FUN_0052b960, FUN_0052bee0, FUN_00514a60, FUN_00556430, FUN_0055d8c0, FUN_0057be20, FUN_004fb520 |
| [timer-scheduler.md](timer-scheduler.md) | Frame loop and setup sequence, the day scheduler, timers 0x380 to 0x394 with their handlers; refutes the 0x1f0 master-tick claim | FUN_0040a050, FUN_005136d0, FUN_0041dff0, FUN_0051df30, FUN_00586130, FUN_005862a0 |
| [community-address-remap.md](community-address-remap.md) | The community disassembly is a different REBEXE build; region shifts and 43 remapped functions | FUN_00508250, FUN_0050b310, FUN_00559fe0, FUN_0055e410 |

## Decompiled Functions (5,408 files)

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
| Space combat pipeline | 0x540000-0x560000 | 7-phase auto-resolve, per-side resolvers | ~60 |
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
