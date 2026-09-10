---
title: "Interface Reverse-Engineering Ledger"
description: "Executable and original-resource evidence queue for closing the original interface parity audit"
category: qa
created: 2026-09-10
updated: 2026-09-10
tags: [qa, interface, ghidra, resources, bitmap, wasm]
---

# Interface Reverse-Engineering Ledger

This ledger connects the [surface ledger](surface-ledger.json) to the original
executable and game resources. It tracks questions that static analysis can
answer without duplicating the screenshot inventory. The machine-readable queue
is in [reverse-engineering-ledger.json](reverse-engineering-ledger.json).

## Current answer

Ghidra and the owned original files can recover substantially more of the
missing interface contract. They can establish exact rectangles, control and
resource IDs, button state selection, command routing, window constructors,
display predicates, animation action tables, embedded audio, tactical event
routing, multiplayer timers, and error paths.

They cannot by themselves prove final palette output, compositing, animation
cadence, live two-peer behavior, or browser equality. Those cells still require
lossless captures from the original executable followed by native and browser
comparison.

## Proven strategic foundation

| Evidence | Static result | Surfaces |
|---|---|---|
| `REBEXE.EXE` `FUN_00421c70` | Faction-specific galaxy apertures and all twelve window-reference rectangles | CMD-01, CMD-04 |
| `FUN_00427270` | Original control positions, sizes, command IDs, and paired bitmap resource IDs | CMD-03, CMD-05, CMD-09 |
| `FUN_006028c0` and `FUN_00602d30` | Mouse capture, down/up command dispatch, invalidation, disabled/state flags, and state-dependent bitmap paint paths | CMD-05 |
| `FUN_00422ce0` and freshly recovered `FUN_00429020` | `WM_COMMAND`, double-click routing, exact rail hit testing, child-window focus, and rail removal | CMD-03, CMD-04 |
| `FUN_00425d00` and freshly recovered `FUN_00426d00` | GID mode/caption branches and the original floating 180×240 display window path | CMD-02 |
| `FUN_0044f670`, `FUN_00442d70`, and `FUN_0044c410` | Code-built, modeless object windows using GOKRES rather than an invented sidebar | CMD-03, OBJ-02–OBJ-14 |
| `FUN_0042d650` and freshly recovered `FUN_0042adb0` | Faction advisor/briefing DLL selection and exact advisor apertures | PRE-05, CMD-07 |

The read-only Ghidra pass also recovered six formerly empty high-priority UI
exports and the `CoolStrobeButton` paint/input path. The text export is not a
complete interface corpus: of 4,934 canonical `FUN_????????.c` files, 2,790 are
zero-byte placeholders. New interface work must query the saved Ghidra project
when a required export is empty.

## Original resource truth

The current browser pack contains 52 game-data entries, 2,231 standard BMPs,
and five audio files. The owned installation contains important additional
families that are not staged or packed:

| Original module | Additional authentic content currently omitted |
|---|---|
| ALSPRITE | 752 BIN controls, 1,640 type-302 animation frames, 213 WAVs |
| EMSPRITE | 753 BIN controls, 2,348 type-302 animation frames, 216 WAVs |
| ALBRIEF | 366 BIN controls, 2,684 type-302 frames, 17 WAVs |
| EMBRIEF | 471 BIN controls, 2,738 type-302 frames, 22 WAVs |
| TACTICAL | 87 type-301 DirectX meshes, 397 type-303 textures, 66 WAVs |
| STRATEGY | 96 RCDATA resources and 66 WAVs beyond its staged BMPs |
| REBDLOG and TEXTCOMM | Original dialog chrome, text, templates, and accelerators |
| EData and ENCYTEXT | 187 400×200 entity images and 348 descriptions |
| MDATA | 15 Smacker films and the remaining original music cues |

The droid path is now concrete. `C3POACT.SPT` and `IMP22ACT.SPT` map logical
actions to BIN resource IDs, which in turn reference standard bitmap anchors and
type-302 frame runs. The current sorted-filename thirds and modulo frame mapping
in `advisor.rs` are provisional and cannot pass parity.

## Work queue

| Package | Scope | Status | Next proof |
|---|---|---|---|
| RE-STR-01 | Shell, apertures, and reference rail | static-proven | Implement exact faction geometry and compare A0 captures |
| RE-STR-02 | Cockpit controls, states, input, and command routing | static-partial | Name every command/resource pair and instantiate hit-probe cells |
| RE-GID-01 | Filters, legends, marker rules, pan, zoom, and selection | static-partial | Resolve TEXTSTRA captions and paint-resource branches |
| RE-OBJ-01 | System, sector, and object-window constructors | static-partial | Map each object type to tabs, resources, states, and return paths |
| RE-ADV-01 | Type-302 advisor and briefing frame decoding | static-partial | Decode one anchor-plus-delta run byte-for-byte |
| RE-ADV-02 | SPT/BIN/FDT action semantics, cadence, and sound | static-partial | Replace inferred priority thirds with authored action mappings |
| RE-PACK-01 | Complete native/WASM resource transport | untriaged | Version the pack for arbitrary resources, films, and EData |
| RE-MSN-01 | Create Mission and Mission Status composites | untriaged | Trace constructors, legal-target predicates, and outcome routing |
| RE-TAC-01 | Tactical loader and event-handler registry | static-proven | Connect event slots to controls and observable state transitions |
| RE-TAC-02 | Tactical control geometry and resource-state selection | static-partial | Trace control constructors and type-301/303 loaders |
| RE-DS-01 | Strategic Destroy System and sabotage paths | static-partial | Resolve confirmation, report, and family `0x34` predicates |
| RE-DS-02 | Tactical Death Star and trench-run routing | static-partial | Prove result-to-`MDATA.201/.202` selection and callbacks |
| RE-END-01 | Campaign endings, skip, return, restart, and failure | static-partial | Recover the complete terminal media matrix |
| RE-NET-01 | Original multiplayer screens and controls | static-partial | Finish template 10100–10103 geometry and provider/host/join routing |
| RE-NET-02 | Two-peer sync, chat, pause, saves, departure, and errors | runtime-needed | Run an original two-peer fixture and compare protocol traces |
| RE-A0-01 | Lossless original-executable baselines | runtime-needed | Capture every required surface-state cell at native 640×480 |

## Immediate implementation order

1. Decode and test the type-302 advisor frame format against known anchors.
2. Trace SPT to BIN to frame and WAV selection for both factions.
3. Extend staging and the runtime pack with typed resources, EData, dialogs,
   voices, and media without weakening deterministic manifests.
4. Rebuild the strategic shell from the recovered rectangles and original
   controls, then replace the sidebar with managed original object windows.
5. Resolve the full GID mapping before drawing any replacement marker or legend.
6. Use original-runtime capture only for the remaining dynamic proof boundary.

No static discovery marks a surface complete. It closes only the corresponding
evidence fields; the original, native, and browser acceptance cells remain in
the main audit.
