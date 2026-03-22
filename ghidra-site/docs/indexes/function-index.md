---
title: "Function Index"
description: "Auto-generated catalog of all 5,151 decompiled C files from REBEXE.EXE, classified by game system"
category: "ghidra"
created: 2026-03-21
updated: 2026-03-21
---

# Function Index — 5,151 Decompiled Functions

Auto-generated catalog of all decompiled C files from REBEXE.EXE Ghidra analysis.
Address ranges map to game systems identified during the Ghidra RE campaign.
See `ai-parity-tracker.md` for implementation status of key AI functions.

## Summary by Game System

| System | Count | Size Range | Key Range |
|--------|-------|-----------|-----------|
| CRT / Game Init | 247 | 0K-14K | 0x004013a0-0x00421c70 |
| Galaxy Map Rendering (GDI) | 154 | 0K-54K | 0x00422270-0x00439fb0 |
| UI Dialogs / Windows | 295 | 0K-29K | 0x0043a0b0-0x0046f360 |
| UI Controls | 248 | 0K-12K | 0x004710f0-0x0048f790 |
| Game Logic / Turn Processing | 79 | 0K-56K | 0x00490340-0x0049fef0 |
| Save/Load / Serialization | 201 | 0K-18K | 0x004a0400-0x004bfe30 |
| Multimedia / Sound | 242 | 0K-5K | 0x004c05e0-0x004dffd0 |
| Entity Base / Destruction | 87 | 0K-15K | 0x004e0170-0x004edfb0 |
| Character System | 137 | 0K-0K | 0x004ee030-0x004f3dd0 |
| Game Object / Fleet Events | 202 | 0K-7K | 0x004f4000-0x004fffe0 |
| Capital Ship / Validation | 229 | 0K-12K | 0x00500050-0x0050ffe0 |
| System Control / Economy | 443 | 0K-13K | 0x00510080-0x0052fff0 |
| Side / Victory / Recruitment | 248 | 0K-11K | 0x005302c0-0x0053ff90 |
| Space Combat Pipeline | 377 | 0K-12K | 0x005400f0-0x0055ff60 |
| Bombardment / Ground Combat | 233 | 0K-2K | 0x00560200-0x0056ffe0 |
| Mission Manager / DAT Loaders | 459 | 0K-6K | 0x00570080-0x0058ff10 |
| Ship Database / Registry | 115 | 0K-38K | 0x00590040-0x0059fd40 |
| Tactical Combat | 101 | 0K-22K | 0x005a0240-0x005afe40 |
| Unknown / Misc | 308 | 0K-17K | 0x005b0280-0x005ef1d0 |
| Networking / UI Controls | 271 | 0K-20K | 0x005f0c90-0x0060eed0 |
| CRT / Runtime Library | 475 | 0K-56K | 0x00610b70-0x00628ff0 |

**Total: 5,151 functions across 21 systems**

## Top 20 Largest Functions

| File | Address | Size | System |
|------|---------|------|--------|
| FUN_004927c0_9k.c | 0x00000000 | 58,096 bytes | Unclassified |
| FUN_004927c0.c | 0x004927c0 | 58,096 bytes | Game Logic / Turn Processing |
| FUN_00422ce0_mega_11k.c | 0x00000000 | 55,566 bytes | Unclassified |
| FUN_00422ce0.c | 0x00422ce0 | 55,566 bytes | Galaxy Map Rendering (GDI) |
| FUN_00597610_ship_db.c | 0x00000000 | 39,096 bytes | Unclassified |
| FUN_00597610.c | 0x00597610 | 39,096 bytes | Ship Database / Registry |
| FUN_00433e40_6k.c | 0x00000000 | 36,696 bytes | Unclassified |
| FUN_00433e40.c | 0x00433e40 | 36,696 bytes | Galaxy Map Rendering (GDI) |
| FUN_0046f360_7k.c | 0x00000000 | 30,269 bytes | Unclassified |
| FUN_0046f360.c | 0x0046f360 | 30,269 bytes | UI Dialogs / Windows |
| FUN_0046a9c0_7k.c | 0x00000000 | 30,078 bytes | Unclassified |
| FUN_0046a9c0.c | 0x0046a9c0 | 30,078 bytes | UI Dialogs / Windows |
| FUN_0044c630_6k.c | 0x00000000 | 29,632 bytes | Unclassified |
| FUN_0044c630.c | 0x0044c630 | 29,632 bytes | UI Dialogs / Windows |
| FUN_004665f0_6k.c | 0x00000000 | 27,200 bytes | Unclassified |
| FUN_004665f0.c | 0x004665f0 | 27,200 bytes | UI Dialogs / Windows |
| FUN_00443130.c | 0x00443130 | 23,604 bytes | UI Dialogs / Windows |
| FUN_00427270.c | 0x00427270 | 23,477 bytes | Galaxy Map Rendering (GDI) |
| FUN_005ab650.c | 0x005ab650 | 22,913 bytes | Tactical Combat |
| FUN_00455060.c | 0x00455060 | 21,513 bytes | UI Dialogs / Windows |

## Annotated Functions (from scholar docs)

See annotated-functions.md for 50+ event IDs, struct layouts, and renamed variables.
See ai-behavior-analysis.md for the 6-function AI pipeline.
See combat-formulas.md for 111 GNPRTB parameter mappings.
See ai-parity-tracker.md for implementation status of each AI function.
