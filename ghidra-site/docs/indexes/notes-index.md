---
title: "Ghidra RE Notes—Index"
description: "Master index of 5,127 decompiled C files, 8 Jython scripts, and 14 scholar documents"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# Ghidra RE Notes—Index

**5,127 decompiled C files, 8 Jython scripts, 14 markdown docs, 6,049 lines of scholar analysis**

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
| [combat-formulas.md](combat-formulas.md) | ~200 | **Master reference**—binary overview, 111 GNPRTB mappings, confirmed functions, scripted events, Ghidra scripts |
| [COMBAT-SUMMARY.md](COMBAT-SUMMARY.md) | ~115 | Combat call chain diagram, confirmed formulas, entity type codes, implementation readiness |

## Combat Subsystem Docs

| File | Subsystem | Key Functions |
|------|-----------|---------------|
| [space-combat.md](space-combat.md) | Space combat auto-resolve—7-phase pipeline | FUN_00549910, FUN_00544030, FUN_00544130, FUN_005443f0, FUN_005444e0, FUN_005445d0 |
| [ground-combat.md](ground-combat.md) | Ground combat—troop iteration + per-unit resolution | FUN_00560d50, FUN_004ee350, FUN_005617b0 |
| [bombardment.md](bombardment.md) | Orbital bombardment—Euclidean distance formula | FUN_00556430, FUN_0055d8c0, FUN_0055d860 |

## Decompiled Functions (5,127 files)

### By Game System

| System | Address Range | Key Functions | Decompiled |
|--------|--------------|---------------|------------|
| Game init / CRT | 0x401000-0x403e90 | entry, CRT boilerplate | ~100 |
| Galaxy map rendering (GDI) | 0x422000-0x43a000 | FUN_00422ce0 (11K), FUN_00433e40 (6K) | ~50 |
| UI dialogs / windows | 0x43a000-0x470000 | FUN_0044c630 (6K), FUN_004665f0 (6K) | ~200 |
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
| FUN_0053e390 |—| Parameter → global address binder |
| FUN_0053e3e0 |—| Per-side parameter → global address binder |

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
| FUN_00509cc0 | SystemEnergy |—|—|
| FUN_00509d40 | SystemEnergyAllocated |—|—|
| FUN_00509dc0 | SystemRawMaterial |—|—|
| FUN_00509e40 | SystemRawMaterialAllocated |—|—|

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
