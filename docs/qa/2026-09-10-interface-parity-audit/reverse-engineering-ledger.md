---
title: "Interface Reverse-Engineering Ledger"
description: "Executable and original-resource evidence queue for closing the original interface parity audit"
category: qa
created: 2026-09-10
updated: 2026-09-11
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
| `FUN_00427270` | Original control positions, sizes, command IDs, and paired bitmap resource IDs | CMD-10 |
| `FUN_006028c0`, `FUN_00602d30`, `FUN_006030c0`, `FUN_006030f0`, and `FUN_006035f0` | Normal and captured-press paint states, pointer capture, release cancellation, dispatch, and disabled flags | CMD-10 |
| `FUN_005fca00`, `FUN_005fd170`, and `FUN_005fc140` | Strict four-edge rejection, bottom-left palette-key hit mask, natural-size paint, and control-window clipping | CMD-10 |
| `FUN_00422ce0` and freshly recovered `FUN_00429020` | `WM_COMMAND`, double-click routing, exact rail hit testing, child-window focus, and rail removal | CMD-03, CMD-04 |
| `FUN_00427010`, `FUN_00425d00`, `FUN_00426d00`, `FUN_00426e70`, `FUN_00426ee0`, `FUN_00427270`, and `FUN_0042b330` | Active 903 versus Display Off 902, GID captions, exact faction control, compact and expanded legends, native marker families, Popular Support thresholds, and floating 180×240 display-window path | CMD-02 |
| `FUN_00427270` and `FUN_00422ce0` | Nine Message Index rail controls per faction, exact 27x22 rectangles, resting and illuminated resource pairs, command IDs `0x136..0x13e`, and dispatch to `FUN_0042a240` | CMD-08 |
| `FUN_0044f670`, `FUN_00442d70`, and `FUN_0044c410` | Code-built, modeless object windows using GOKRES rather than an invented sidebar | CMD-03, OBJ-02–OBJ-14 |
| `FUN_0042d650` and freshly recovered `FUN_0042adb0` | Faction advisor/briefing DLL selection and exact advisor apertures | PRE-05, CMD-07 |

P46A implements the centered 640x480 strategic canvas, the 640x481 source
crop, both faction galaxy apertures, and one shared map, hit-test, blockade,
and advisor transform. Native tests and packaged-browser viewport checks
corroborate this shell checkpoint. UIP-B01 now implements the faction-specific
twelve-slot rail geometry and first focus, close, minimize, restore, and
eviction lifecycle. Exact active and inactive rail thumbnails plus the full
multiwindow matrix remain open. See the
[P46A evidence](evidence/2026-09-11-strategic-shell-canvas.md).

P46B restores the six primary faction controls from the recovered constructor,
paint, hit-test, capture, and `WM_COMMAND` paths. It removes the replacement
text strip, renders the exact normal and captured-press bitmap pairs, and keeps
all six destinations closed until their original windows exist. Full
A0 captures, the disabled path, destination compositions, and speed controls
remain open. P47B adds the exact faction GID control under the same native
input contract, while its destination menu remains open. See the
[P46B evidence](evidence/2026-09-11-strategic-command-controls.md).

The first UIP-B01 checkpoint also replaces the invented sidebar with recovered
235x360 sector and 226x304 detailed-system shells. Original planet pictures,
relationship title art, six tab resource families, single and double-click
routing, pointer occlusion, and rail transitions work in both faction shells.
P46D maps characters, fleet representatives, facilities, regiments, special
forces, mines, and refineries to their GOKRES miniatures. It also restores the
three-column item viewport, STRATEGY `10365` through `10369` scrollbar art,
selection, displayed labels, bounded input, and coarse current-intelligence
gating for opposing objects. Nested compositions, exact intelligence rules,
drag and command semantics, uncommon states, and the complete A0 matrix remain open. See the
[navigation evidence](evidence/2026-09-11-strategic-window-navigation.md) and
[tab-item evidence](evidence/2026-09-11-detailed-system-tab-items.md).

P49 restores the eighteen source-mapped Message Index rail resting BMPs. Both
faction rails match their original 8-bit resources pixel-for-pixel at 640x480.
The illuminated predicate and original Message Index destination remain open.
See the [rail evidence](evidence/2026-09-12-message-index-rail.md).

P47A identifies STRATEGY 900 and 901 as the faction shells, with 902 and 903 as
the bright and dim galaxy resources. P47B proves 902 belongs to Display Off and
903 to every active GID mode, then restores the default Popular Support caption,
exact faction GID control, compact 10168 legend, native 10146 through 10158
marker families, and support-size thresholds. It also establishes that the
nine tall side controls are Message Index categories, not GID filters. P48
maps the code-built GID command tree. P50 restores the STRATEGY 10100 through
10107 menu-frame tiles with instant display and source-pixel browser checks.
P51 withholds an unsupported hover wash and verifies frame pixels still visible
beneath the detailed system window.
The menu interior, typography, exact geometry, expanded 180x240 legend,
remaining modes and overlays, and exact map input remain open. See the
[galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md) and
[Popular Support evidence](evidence/2026-09-11-popular-support-gid.md), plus the
[P50 frame record](evidence/2026-09-12-gid-menu-frame.md) and
[P51 overlap record](evidence/2026-09-12-gid-hover-and-occlusion.md).

The read-only Ghidra pass also recovered six formerly empty high-priority UI
exports and the `CoolStrobeButton` paint/input path. The text export is not a
complete interface corpus: of 4,934 canonical `FUN_????????.c` files, 2,790 are
zero-byte placeholders. New interface work must query the saved Ghidra project
when a required export is empty.

## Original resource truth

Runtime pack v2 contains 52 game-data entries, 2,303 standard BMPs, all 3,988
ALSPRITE and EMSPRITE type-302 frames, and five audio files. The owned
installation contains important additional families that are not yet staged or
packed:

| Original module | Additional authentic content currently omitted |
|---|---|
| ALSPRITE | 752 BIN controls and 213 WAVs; its 38 BMPs and 1,640 type-302 frames are staged |
| EMSPRITE | 753 BIN controls and 216 WAVs; its 34 BMPs and 2,348 type-302 frames are staged |
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

The queue contains 27 bounded packages. Every one of the 43 required surface
families links to at least one package with named sources, a retrieval method,
and a next proof in the [machine-readable ledger](reverse-engineering-ledger.json).

| Package | Scope | Status | Next proof |
|---|---|---|---|
| RE-FE-01 | Boot, introduction, and credits routing | static-partial | Map media selection and every completion, failure, skip, and return callback |
| RE-MENU-01 | Shuttle controls and destinations | static-proven | Capture remaining original interaction and edge-probe states |
| RE-OPT-01 | Unified options, save, load, and delete | untriaged | Recover its constructor, controls, resources, persistence, and confirmations |
| RE-STR-01 | Shell, apertures, and reference rail | static-proven; shell and first rail lifecycle runtime-corroborated | Replace provisional rail thumbnails, exercise the full multiwindow matrix, then compare against A0 captures |
| RE-STR-02 | Cockpit controls, states, input, and command routing | static-partial; primary controls implemented | Capture the six primary controls in A0, implement original destinations, then recover speed and facility-indicator predicates |
| RE-GID-01 | Filters, legends, marker rules, pan, zoom, and selection | static-partial; default GID and source frame runtime-corroborated | Recover menu interior and geometry, expanded legend, remaining filter predicates and overlays, and exact map input |
| RE-OVR-01 | Galaxy Overview | untriaged | Recover geometry, category formulas, resources, and destinations |
| RE-MSG-01 | Messages, Agent menus, alerts, and reports | static-partial; rail controls recovered | Decompile `FUN_0042a240`, map unread states and the original index window, then join reports, audio, and navigation |
| RE-ENC-01 | Encyclopedia index and topics | static-partial | Complete ENCYBMAP, ENCYTEXT, EData, and navigation mappings |
| RE-OBJ-01 | System, sector, and object-window constructors | static-partial; sector and system shells runtime-corroborated | Complete system item compositions and commands, then map the remaining object families |
| RE-ADV-01 | Type-302 advisor and briefing frame decoding | runtime-corroborated | Extend the verified advisor decoder/transport to briefings and compare with A0 captures |
| RE-ADV-02 | SPT/BIN/FDT action semantics, cadence, and sound | static-partial | Replace inferred priority thirds with authored action mappings |
| RE-PACK-01 | Complete native/WASM resource transport | implementation-needed | Version the pack for arbitrary resources, films, and EData |
| RE-MSN-01 | Create Mission and Mission Status composites | untriaged | Trace constructors, legal-target predicates, and outcome routing |
| RE-EVT-01 | Strategic events and authored reports | static-partial | Resolve every common, faction, and rare event variant |
| RE-BAT-01 | Battle Alert, strategic reports, and results | untriaged | Recover constructors, choices, force tabs, results, and return routing |
| RE-GND-01 | Original ground-assault presentation | static-partial | Prove report-only flow and remove the invented live-combat route |
| RE-TAC-01 | Tactical loader and event-handler registry | static-proven | Connect event slots to controls and observable state transitions |
| RE-TAC-02 | Tactical control geometry and resource-state selection | static-partial; complete corpus staged and decoded, one pair browser-rendered | Add resources 2561 and 2562, then trace original entity, camera, palette, and LOD rules |
| RE-TAC-03 | Tactical battle-results composition | untriaged | Connect result construction to application and strategic return |
| RE-DS-01 | Strategic Destroy System and sabotage paths | static-partial | Resolve confirmation, report, and family `0x34` predicates |
| RE-DS-02 | Tactical Death Star and trench-run routing | static-partial | Prove result-to-`MDATA.201/.202` selection and callbacks |
| RE-END-01 | Campaign endings, skip, return, restart, and failure | static-partial | Recover the complete terminal media matrix |
| RE-NET-01 | Original multiplayer screens and controls | static-partial | Finish template 10100–10103 geometry and provider/host/join routing |
| RE-NET-02 | Two-peer sync, chat, pause, saves, departure, and errors | runtime-needed | Run an original two-peer fixture and compare protocol traces |
| RE-EXT-01 | Remove visible replacement dashboards | implementation-needed | Preserve every action inside original paths with zero replacement pixels |
| RE-A0-01 | Lossless original-executable baselines | runtime-needed | Capture every required surface-state cell at native 640×480 |

Space battle is an explicit full mode, not a single panel. `TAC-01` through
`TAC-07` currently define 106 baseline cells covering battle entry, both
faction HUDs, capital ships, fighters, assignment, selection, targeting,
damage, camera and navigation, maneuvers, tactics, missions, recovery,
withdrawal, simulation and observation, Death Star controls, trench-run
routing, results, and strategic return. `EVT-02` owns Battle Alert entry;
`TAC-08` remains the separate strategic ground-assault report flow.

P54 now supplies the raw-resource boundary for `RE-TAC-02`: 87 type-301
binary X meshes and 397 type-303 texture or palette resources reproduce from
the owned `TACTICAL.DLL` with exact identities and hashes. The
[P54 evidence](evidence/2026-09-12-tactical-3d-staging.md) records the inventory
and one embedded mesh-to-texture edge. P55 decodes all of them into a
deterministic runtime store and verifies the mesh corpus against Assimp 6.0.5;
see its [evidence](evidence/2026-09-12-tactical-3d-runtime-pack.md). P56
packages and visibly renders exact pair `2560/1033` plus
`SDESTI52.BMP/1033`; see its
[evidence](evidence/2026-09-13-tactical-3d-render-proof.md). Complete browser
transport, semantic joins, palette activation, camera and LOD rules, and
visual acceptance remain open.

## Immediate implementation order

1. Trace SPT to BIN to frame and WAV selection for both factions on top of the
   verified type-302 advisor idle-frame decoder and transport.
2. Extend the decoder and transport to ALBRIEF and EMBRIEF.
3. Extend staging and the runtime pack with EData, dialogs,
   voices, and media without weakening deterministic manifests.
4. Complete the first recovered sector, system, and reference-rail checkpoint
   with exact item compositions, commands, thumbnails, and uncommon states,
   then continue through the remaining managed original object windows.
5. Continue the full GID mapping without replacement art: recover the code-built
   menu, expanded legend, remaining filter predicates and overlays, and exact
   map input.
6. Recover and implement the complete `TAC-01` through `TAC-07` space-battle
   path, including type-301/303 resources and Death Star controls.
7. Use original-runtime capture only for the remaining dynamic proof boundary.

No static discovery marks a surface complete. It closes only the corresponding
evidence fields; the original, native, and browser acceptance cells remain in
the main audit.
