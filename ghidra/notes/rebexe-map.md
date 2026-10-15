---
title: "REBEXE Map: Every Address Range and What It Holds"
description: "An atlas of REBEXE.EXE from 0x00401000 to 0x00629000 in 22 ranges: the subsystems, classes, vtables and key functions in each, and what the port lacks per range"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [atlas, map, rebexe, functions, vtables, subsystems]
---

# REBEXE map

Built from the 22 Feature Wiki pages `executable/map-NN-<start>` (task-graph
epic `or-map-rebexe-r8t`), which account for every function and code gap in
REBEXE.EXE. Each section names what its range holds; the decompiles it cites
sit beside this note. Where `FUNCTION_INDEX.md` names a range's subsystem
differently, the sections here hold. `hyp:` marks anything a page did not
trace through code.

| Range | Holds |
|---|---|
| `0x00401000`-`0x0041a460` | Application Core and Game Session |
| `0x0041a460`-`0x0043eca0` | AI World Model, Session Glue, and Notification Classes |
| `0x0043eca0`-`0x00475890` | UI Message Factories, Order Classes, and Game Windows |
| `0x00475890`-`0x0048a3b0` | AI Mission Planner and Per-Player Interface |
| `0x0048a3b0`-`0x004ae150` | Game Messages, Agent Modules, and Missions Window |
| `0x004ae150`-`0x004bca20` | AI Regional Posture Engine |
| `0x004bca20`-`0x004cfed0` | AI Task Graph, Droid Advisors and Agent World Model |
| `0x004cfed0`-`0x004deba0` | AI Economy Module Family |
| `0x004deba0`-`0x004ed950` | AI Task Classes, Sector-Planner Strategies and Shared Bases |
| `0x004ed950`-`0x00508c53` | Character, GameObj, Fleet and Unit Classes |
| `0x00508c53`-`0x0052a380` | Star System, Game Root, Setup, Scheduler and Missions |
| `0x0052a380`-`0x00547ba0` | ManufacturingMgr, SideState, Person, Orders and Combat Phases |
| `0x00547ba0`-`0x00564240` | Mission/Story Manager, Schedulers, Deployment |
| `0x00564240`-`0x005771b0` | Class Registration — Missions, Characters, Systems, Events, Timers |
| `0x005771b0`-`0x0058ef80` | Events, Commands, Orders, Ground Assault, Bombardment |
| `0x0058ef80`-`0x005a54ee` | DAT Record Classes, Mission Bases, Tactical Engine Entry |
| `0x005a54ee`-`0x005c2990` | Tactical Battle Engine Core |
| `0x005c2990`-`0x005d895e` | Space Battle Object Layer |
| `0x005d895e`-`0x005e9bcb` | D3DRM Wrapper, Cameras, Tactical Panels |
| `0x005e9bcb`-`0x005fd7c0` | Formations, Class Library, DirectPlay, Video, Bitmap |
| `0x005fd7c0`-`0x00610b70` | Cool* Window and Control Library |
| `0x00610b70`-`0x00629000` | Sound, DirectDraw, iostream, CRT |

## 0x00401000-0x0041a460: Application Core and Game Session

Feature Wiki page `executable/map-01-401000`.

Source: the Feature Wiki page executable/map-01-401000.

468 functions. FUNCTION_INDEX.md labels the overlapping range "crt-init" but
only ~50 functions are CRT stubs; the rest are application-level game subsystems.

- **Game settings forwarders (0x401000-0x401b00).** ~40 null-guarded thunks over the
  global `DAT_006be3b8` (GameOptions singleton). Entry `FUN_00414330` dispatches app
  state (15 callers). `FUN_004011a0` posts messages 0x47a/0x405/0x46a/0x46b with game
  speed at `DAT_006b1470`. Offsets +0x18 HWND, +0x104 game mode, +0x140 display opts.
- **AI object record and queries (0x401c70-0x404040).** Record vtable `0x006584a0`
  (0x68 bytes): ctor `FUN_00401d20`, save `FUN_00402120`, refresh `FUN_00402230`.
  Record holds DatId at +0x18, skills +0x40..+0x5c. `FUN_00402720` probes mission
  legality via dummy order 0x240. `FUN_00402cf0` tests Luke (0x32000242) or Vader
  (0x35000281). `FUN_00402e80` sets special-force capability bits in +0x34.
- **Droid advisor step classes (base at 0x403dd0; steps 0x412080-0x412fb0).** Base ctor `FUN_00403dd0`
  (vtable 0x6584e8). Prototype table `DAT_006b059c[1..0x20]`. Command 4 (vtable
  0x658520, animation script player), Command 5 (vtable 0x658550, still frame).
  Step types 0x11-0x13 and 0x15 each override bind/tick/load. See droid-advisor-triggers.md.
- **Cockpit main window (0x404c30-0x406000).** Class vtable 0x658580, global
  `DAT_006b06f4`. Galaxy size screen, difficulty, game type. Alliance = side 1 (cmd
  0x65), Empire = side 2 (cmd 0x66). Side 1 passes difficulty+1, side 2 difficulty+4.
- **Game Options window (0x406950-0x408f10).** Save/load/volume/display. Vtable
  0x658600. Display option bits: 0x100 music, 0x10000000 starfield, 0x01000000 planet,
  0x00100000 pyro, 0x02000000 holocube. Slot refresh via `FUN_00408f10`.
- **BattleSession singleton (0x409230-0x40a700).** Global `DAT_006b1214`. Step state
  machine `FUN_0040a050`: strategic states 1-0xc, tactical 0x14-0x17. `FUN_0040a700`
  maps state to debug name. Snapshot files use registry InstalledLocation path.
- **Tactical import/export (0x40afa0-0x40d760).** Export vtable 0x65864c walks system
  fleets/ships/squadrons/characters/regiments. Import vtable 0x658650 decides winner
  (0=none, 1=Alliance, 2=Empire, 3=both) from `FUN_005969d0` outcome and system+0x78
  bits 6-7, then writes back ships/squads/troops/characters.
- **MultiPlayerWindow (0x40d7a0-0x411710).** Vtable 0x658788. Screens: 0x2774 main,
  0x2775 join, 0x2776 host, 0x2777 lobby. Net types: 0x10 setup, 0x11 chat, 0x12
  quit, 0x14 ready, 0x16 (unlabelled). Session app name string "Rebellion".
- **RebellionApplication (0x413000-0x416950).** 0x160-byte object, vtable 0x6588f8,
  class "STARWARS_REBELLION". State machine `FUN_00414330` (states 1-0x13). WinMain
  `FUN_004165b0`: single-instance semaphore, 640x480 display check, -L/-N/-W flags.
- **Alert dialogs (0x416b90-0x417260).** ConfirmDialog vtable 0x658950, OneButtonAlert
  vtable 0x6589a8, QuitConfirm vtable 0x658a00. Result at `DAT_006b1470`.
- **Music player (0x4173c0-0x417780).** 16-track table `PTR_s_MDATA_300_006a8478` (`audio-dispatch.md`). Registry
  MusicVolume (stored negated), MusicSwitch (stored inverted). `FUN_00417610` pinned
  play, `FUN_00417640` stop. Next-track chooser is `FUN_0041d3b0` (outside band).
- **Side controller (0x4178f0-0x4199b0).** Per-side AI/game-state object. Layout: +0
  side (Alliance 0, Empire 1), +4 flags, +0x140..+0x148 current tech ids. Id-dispatch
  helpers (0x418a60-0x419260) switch on id high byte: 0x08-0x0f characters, 0x14-0x1b
  troops, 0x30-0x3f ships, 0x40-0x7f fleets, 0x80-0x8f locations, 0x90-0x97 systems. Tech-completion hooks test Empire
  0x1400004a, Alliance 0x14000082, fighter 0x25000004.
- **Filtered collectors (0x419a00-0x41a400).** Mask-based queries over the master
  object list. Variants weight by stat, distance, or constant. `FUN_00419e40` is an
  id-to-record wrapper (78 callers). Neighbours: `FUN_004317e0`, `FUN_00431830`.
- **CRT _initterm pairs.** 7 _initterm slots (006a6004-006a6050) run prototype ctors
  then `FUN_00618c20` (atexit). `FUN_00403eb0` stores each in `DAT_006b059c[id]`.

## 0x0041a460-0x0043eca0: AI World Model, Session Glue, and Notification Classes

Feature Wiki page `executable/map-02-41a460`.

Source: the Feature Wiki page executable/map-02-41a460.

990 functions (180 cited by local notes, 810 uncited). Side convention in this range:
galaxy view +0x9c uses 1=Alliance, 2=Empire; AI records use 0=neutral, 1=Alliance,
2=Empire.

- **AI side-controller record pool (0x41a460-0x41bd90).** SysRec, SectorRec, FleetRec
  pools (CObList-style, next at +0x10). `FUN_0041a460` and `FUN_0041ac80` are
  id-guarded wrappers over `FUN_00431f40`/`FUN_00433450`. `FUN_0041b2b0` finds the
  nearest system (<2e9) with a qualifying candidate. `FUN_0041b640` resolves any id up
  the location chain to a system. RankedList (vtable 0x658a58) sorts by key with
  configurable order (1=lower-first).
- **Per-side AI census (0x41b7e0-0x41bd20).** Accumulators fed by fleet, regiment, and
  facility records during refresh state 5. Per-kind counters at +0x35c (indexed by
  vtbl+0x28) and +800 decimal, `0x320` (indexed by vtbl+0x40). `FUN_0041bde0` copies side+0x58
  (top-bar resource counters).
- **CPlayerMgr (0x435a40-0x436a10).** Singleton `DAT_006b14e4`, 0x34 bytes, ctor
  `FUN_00435bc0`. Slot 0=Alliance, slot 1=Empire. Local side is 0xc4-byte human
  controller (`FUN_004861b0`); opponent is 0x42c-byte (network, kind 3) or 0x60-byte
  (AI, kind 2). RNG state at +4 (`FUN_0041cd80`=`FUN_005f5700`), reset to 1 on
  turn-wrap in `FUN_00435f70`.
- **Strategic main frame and GID (0x421c70-0x42db70).** ~75 functions. Galaxy view
  `FUN_00422ce0` (message 0x409/0x468/0x482 handler), `SetGidMode` `FUN_00425d00`,
  `UpdateSystemMarker` `FUN_0042b330` (3,951 bytes). Cockpit buttons per side
  `FUN_00427270` (Alliance 0x12d.., Empire sets). 12-slot minimized-window rail, game
  speed menus (Alliance 0x20-0x24, Empire 0x25-0x29), message rail.
- **Multiplayer messages and movie player (0x41d1f0-0x421ba0).** Lobby message kinds
  0x10-0x16. TextScrollField class (25 functions, string "TextScrollField"). Movie
  window with MCI API, RLE delta sprite helper, alsprite.dll/emsprite.dll selection.
- **GameStep (0x41dce0-0x41e500).** Vtable 0x658a9c. 14-phase step loop in
  `FUN_0041dff0`. Phase 5 = serve one due scheduler item. Phase 6 = compare snapshots
  and flush notifications. Phase 0xc = take snapshot every 40 ticks. Phase 0xd =
  lockstep gates and cancel-order post (0x15e).
- **Notification registry (0x41e550-0x41ec50).** `FUN_0041e580` registers 97
  notification id-to-factory pairs into `DAT_006b2fd8` (capacity 0x800). Ids span
  1-0x600. `FUN_0041ec50` registers 10 multiplayer session message kinds (1, 2, 4, 5,
  0x10-0x15) into the command table via `FUN_0051f930`.
- **Agent controller and build-selection window (0x437880-0x43a200).** Player-side
  agent/advisor controller at galaxy view +0xc0 (vtable 0x658e10, 29 functions).
  Advice-topic holding, build-selection window (0x154 bytes, 210x261, bg 0x2a30).
- **~100 UI notification classes (0x43a320-0x43ec90).** One record class per
  notification id, each with ctor/dtor/factory/id stubs. Ids and names
  (SystemRawMaterial, SmugglingPercent, Uprising, TroopRegSurplus, ShipyardRDOrder, ...,
  Destroyed) come from sender name strings. Registered by `FUN_0041e580` with
  `FUN_0051f770(id, factory)`.
- **Droid advisor tour plumbing (0x41d660-0x41dae0).** RT_RCDATA scripts from modules
  9/13 queued on agent droid (view+0x128, key 0xdead) or partner (view+0x12c, key
  0xbeef). Keyboard and mouse hooks for tour mode. `FUN_0041daf0` selects
  alsprite.dll/emsprite.dll by side.
- **Galaxy view forwarders (0x41d430-0x41db60).** Per-system window updates via
  `FUN_00604500`. Top-bar counters: raw +0x178 (tooltip 0x1510), refined +0x1ac
  (0x1511), maintenance +0x1e0 (0x1512; above 99999 shows TEXTSTRA 0x1302).

## 0x0043eca0-0x00475890: UI Message Factories, Order Classes, and Game Windows

Feature Wiki page `executable/map-03-43eca0`.

Source: the Feature Wiki page executable/map-03-43eca0.

~330 functions. FUNCTION_INDEX.md labels this "UI Dialogs / Windows", which is broadly
correct: the range contains notification class factories, order-kind classes, the
Status/System/Manufacturing/Sector windows, fleet/personnel/troop finders, the Message
Index, the Mission dialog, the droid command queue, class-profile classes, and an AI
fleet task tail.

- **UI message class factories, ids 0x101-0x500 (0x43eca0-0x440350).** One class per
  notification id. Base ctors: `FUN_00539290` (events 0x304-0x306, vtable 0x661300),
  `FUN_00539370` (0x1xx notifications), `FUN_00539440` (id 2, vtable 0x661340),
  `FUN_00539550` (0x500, vtable 0x661360). Vtable has 8 slots (stride 0x20); slot 6
  is the accept/visitor call. Payload: this+0x20 (id) and *(this+0x28) (value).
- **Events 0x303-0x306 (0x43eca0-0x43f110).** GameObjDestroyed subtypes: 0x303
  OnArrival (factory 0x43eca0), 0x304 Autoscrap (vtable 0x659798), 0x305 Sabotage
  (vtable 0x6597b8), 0x306 Assassination (vtable 0x6597d8). Slot 7 is identical
  across all three. 0x305 alone has a real slot 6 (`FUN_0043ef10`, visitor+0x170).
- **Fleet notifications 0x180-0x183.** FleetAssault (0x183), FleetBattle (0x180),
  FleetBlockade (0x181), FleetBombard (0x182). Visitor slots +0xf8..+0x104.
- **Order-kind classes (0x440410-0x441190).** 10 kinds registered by `FUN_0041ec50`
  via `FUN_0051f930`: kinds 1, 2, 4, 5, 0x10-0x15. The lobby pump `FUN_004104b0`
  handles 0x10/0x11/0x12/0x14/0x16; in-game pump `FUN_00485030` handles 2/4/0x12/0x15.
  All share the Command base (`FUN_0051f8a0`).
- **Strategic-view state saver and droid advisor player.** State saver records the
  GID mode, view flags, and system selection for restore after tactical or options.
  Droid advisor player loads RT_RCDATA scripts, queues them, and dispatches.
- **Game Menu / Status window row fillers (0x441190-0x458040).** Status window (kind
  0x1a, vtable at 0x659a70) fills rows from object vtable slots. Column kinds: name,
  location, garrison, shields, missions, construction. `FUN_0044a2e0` uses the TEXTSTRA
  version flag for the "Attached:" label string.
- **Battle Results and Alert windows.** Battle Results (vtable 0x659e38) shows outcome
  text from `FUN_005969d0`. Alert window (vtable 0x659f30) displays side-specific
  notification text.
- **GID menu and legends.** Context menu on the galaxy view, system info tooltip, and
  the legend overlays for fleet icons, system markers, and sector colors.
- **System/Manufacturing/Sector windows (0x457b40-0x45d400).** System window (kind 9),
  Manufacturing window (kind 0xa), Sector window (kind 1). Manufacturing yard list
  with build-queue items. System window shows garrison, defenses, raw/refined totals.
- **Fleet/Personnel/Troop Finders (0x45d400-0x468fb0).** Encyclopedia (type 0x19,
  alloc 0x164, `FUN_0045d400`). Fleet finder, personnel finder, troop finder each
  present a filtered object list.
- **Message Index (0x468fb0-0x46f140).** Kind 0xd window. Filter categories: All (mask
  0x79), Advice (0x82). Post Messages Silently at +0x158 (filter mask via
  `FUN_0041d230`). Body is a TextScrollField (395x80, font 4).
- **Battle Summary (0x46f140-0x474000).** Kind 8 window (470x331, `FUN_0046f140`).
  The mission dialog is the type-0x0c window at `FUN_0046a750`.
- **AI fleet task tail (0x474000-0x475890).** Tail of an AI fleet-task class that
  begins before this range. Task priority queue and force-assembly stubs.

## 0x00475890-0x0048a3b0: AI Mission Planner and Per-Player Interface

Feature Wiki page `executable/map-04-475890`.

Source: the Feature Wiki page executable/map-04-475890.

FUNCTION_INDEX.md labels the overlapping range "UI Controls" but ~70% of this range is
the AI mission planner and per-player interface controllers.

- **AI mission planner (0x475890-0x47a430).** Vtable 0x0065a3b0, 6-state step machine.
  `FUN_004774c0` is the planner's step machine; `FUN_004bced0` (map-07) drives
  the per-kind subclass ticks. Factory `FUN_00477630`
  builds per-kind subclasses. Kind ids and their classes:
  - Research 1/2/3 (basic, interdict, facility)
  - Subdue (planetary loyalty shift)
  - Recruitment (character acquisition)
  - Rescue (prisoner extraction)
  - Sabotage (facility targeting)
  - Recon (intelligence gathering)
  - Incite Uprising (populace agitation)
  - Espionage (intel theft)
  - Diplomacy (loyalty shift)
  - Death Star Sabotage
  - Assassination (character elimination)
  - Abduction (character capture)
- **Force assembly (0x478000-0x479a00).** Force-assembly helpers select fleets and
  units for mission escort. Priority ranking by distance and capability.
- **Character appointment requests.** Separate appointment-request objects hold the
  request id at +0x44 and the pending-selection id at +0x48.
- **Planner base class with per-kind subclasses.** Each subclass overrides target
  selection, character fitness evaluation, and force requirements. The Assassination
  subclass uses a different base (`FUN_004bd810`) with a different slot layout than
  other mission kinds.
- **Per-player interface classes (0x484df0-0x486f60).** Three kinds:
  - Kind 1 (0xc4 bytes, vtable 0x65ad70): local human player UI controller.
    Ctor `FUN_004861b0`. +0x24 = side, +0xc0 = advisor object (0x194 bytes).
    Advisor creation: +0x24==1 dispatches to `FUN_004c27f0` (vtable 0x65c4c8),
    +0x24==2 to `FUN_004c0710` (vtable 0x65c4a0). hyp: 1 is the Alliance, from the
    galaxy view's 1/2 convention; the page leaves the side mapping open.
  - Kind 2 (0x60 bytes, vtable 0x65aa70): AI/computer player. hyp: name.
  - Kind 3 (0x42c bytes, vtable 0x65abf0): remote/network player. Drains 14 message
    types with budget limit of 10.
- **Command dispatcher `FUN_00486fb0` (1,659 bytes).** Routes commands by id: 0x20-0x29
  game speed, 0x100/0x103 object windows, 0x110-0x112 build orders, 0x115-0x118 agent
  actions, 0x171-0x179 option toggles, 0x201/0x202/0x214 order targeting, 0x203/0x215
  rename, 0x210-0x212 build facilities/ships/troops.
- **Advisor mood selection (`FUN_004866e0`).** Computes mood from advisor stat deltas:
  d>=2 -> mood 1, d==1 -> 2, d==-1 -> 0x20, d<-1 -> 0x40, else ratio-based (r>=300 ->
  2, r>=200 -> 4, r<0x33 -> 0x10).
- **Message center / session glue.** Session message posting, per-side inbox helpers,
  and the message pump that dispatches types 2/4/0x12/0x15 (`FUN_00485030`).
- **Game speed intervals.** Speed 1 = 600 ticks, 2 = 60, 3 = 12, 4 = 4 (the source of
  the speed row in "What the port lacks").

## 0x0048a3b0-0x004ae150: Game Messages, Agent Modules, and Missions Window

Feature Wiki page `executable/map-05-48a3b0`.

Source: the Feature Wiki page executable/map-05-48a3b0.

FUNCTION_INDEX.md labels the overlapping range as straddling "UI Controls" and "Game
Logic / Turn Processing." The actual content is game message/notification class family,
per-side inbox helpers, agent build modules, the Missions window, and an AI posture
base class tail. Not a window-code range.

- **Game message/notification class family (0x48a3b0-0x499000).** One class per
  notification type, each with a factory, a kind accessor, and a visitor-accept slot.
  Per-side inbox helpers dispatch messages to the correct side's queue. `FUN_0048aa00`
  is the shared posting entry point, taking the kind id and side.
- **Han bounty message (factory case 0x1b).** Reports kind 0x1a (same as character
  health), overwriting the prototype. hyp: loading a saved kind-0x1a message builds
  a Han bounty message instead of a character-health message. This is a potential
  save-load fidelity issue.
- **Advice topics (0x499000-0x49c000).** Advice-topic holder classes for the droid
  advisor. Each topic has a trigger condition, a script id, and a priority. Topics
  span strategic advice (fleet positioning, garrison, production), tactical advice
  (battle engagement), and event-driven advice (uprising, blockade, capture).
- **Message posting.** `FUN_0048aa00` / `FUN_0048aa90` post messages; `FUN_0048a340`
  (just below the range) builds kind-8 ConfirmationMsg messages.
- **Agent build modules (0x49c6f0-0x49ec40).** Sub-objects of the player Agent (vtable
  0x658e10). Action table: one 0x2c-byte slot per reaction slot, loaded from
  GData/C3POACT.SPT (Alliance) or IMP22ACT.SPT (Empire) via `FUN_0049c6f0`. Survey
  object (Agent+0x3c) holds per-system and per-sector scan results used during Agent
  tick state 2 (`FUN_0049cb30`).
- **AgentModuleMap (Agent+0x18, vtable 0x65bc88).** Tree of sub-modules keyed by
  index: 0x14 Production, 0x15 Garrison, 0x17 Build. Factory `FUN_0049e010` creates
  sized instances (0x58/0x5c/0x58). Scheduler `FUN_0049e130`: highest priority among
  ready modules, then round-robin fallback.
- **BuildModule 0x17 (vtable 0x65bc98).** One-shot "Build Ships/Troops/Facilities"
  automation. Menu 0x110/0x111/0x112. Kind +0x34: 1=facilities, 2=ships, 3=troops.
  Order types 0x210/0x211/0x212. Yard masks 0x4000/0x1000/0x2000. Phase machine:
  phase 1 finds a yard, phase 2 creates order 0x214, phase 3 creates 0x210/0x211/0x212
  for facility family 0xa0..0xaf.
- **AgentSurvey (Agent+0x3c).** Per-system/per-sector tallies. 26-int tally per record
  (+0x60). Phase 4 (`FUN_0049cc80`) re-tallies dirty records incrementally (cursor
  +0x74, sub-state +0x7c). Queries by metric, distance, or availability feed the
  Production/Garrison/Build modules. The survey is not ported.
- **Missions window (kind 11, vtable 0x65bd10).** `FUN_0049f4b0` is slot 5.
  Character assignment, target selection, mission-type tabs. Type-7 overlay window
  with slots 9/10 at vtable offsets 0x65bcdc/0x65bce0.
- **AI posture base class tail (0x4a0c20-0x4ae150).** Tail end of the AI posture
  engine base class. Posture state fields, force-evaluation helpers, and the order
  dispatch path. Connects to the six posture sibling classes in map-06.

## 0x004ae150-0x004bca20: AI Regional Posture Engine

Feature Wiki page `executable/map-06-4ae150`.

Source: the Feature Wiki page executable/map-06-4ae150.

FUNCTION_INDEX.md labels the overlapping range "Save/Load / Serialization", but the
range is the AI regional posture engine: six sibling classes over one base, plus AI
character-relocation tasks. Its save-load content is each class's own Load (slot 1)
and Save (slot 2). 141 functions (130 defined, 11 recovered from gaps); no strings or
resource ids.

- **Posture classes.** Base vtable `0x0065c0b0` (ctor `FUN_004ad8e0`, just below the
  range); six siblings of 22 slots each, built by the factory `FUN_00477630` for the
  kind in a regional record's `+0x30` (`FUN_004773f0`): kind 5 `0x0065c108`, kind 3
  `0x0065c160`, kind 6 `0x0065c1b8`, kind 4 `0x0065c210`, kind 2 `0x0065c268`, kind 1
  `0x0065c2c0`. Slot 5 returns the kind; `FUN_004773f0` uses it to check the posture
  still matches the record. The kinds carry numbers only: the page gives them no
  role names.
- **Object layout.** `+4` side (0 Alliance, 1 Empire), `+8` step state, `+0xc` AI
  context (per-class floors at `+0x164..+0x174`), `+0x10` regional record, `+0x14`
  flag word, `+0x18` object list, `+0x28/+0x2c/+0x30` three DatId refs, `+0x34..+0x58`
  ten need counters, `+0x5c` an extra ref in kinds 4 and 2.
- **Step machine `FUN_004adb70`** (below the range) keeps the state in `+8` and runs
  states 1..`0xe`: 1 `FUN_004af6d0` (classify), 2 slot 13 (claim secondary), 3 slot 14
  (issue moves), 4 `FUN_004b0120` (move squadrons), 5 `FUN_004b04c0`, 6 `FUN_004b06b0`
  (move ships and personnel), 7..10 slots 15..18 (try needs 1..4), `0xc`
  `FUN_004b0c90`, `0xd` `FUN_004b0f90` (staged Move), `0xe` `FUN_004b1070` (emitter).
- **Slots shared or overridden.** Slot 6 `FUN_004b7150` (recommend missions: Alliance
  `0x62` Abduction, Empire `0x63` Assassination) is common to all six; slot 7
  SuggestAction is `FUN_004b1800` (kinds 5, 3), `FUN_004b72d0` (kinds 6, 4) or
  `FUN_004b9290` (kinds 2, 1); slots 13 and 14 default to `FUN_004af8a0` and
  `FUN_004afce0` (kinds 1, 2, 3) and are overridden by kinds 4, 5, 6. Slot 20
  ComputeNeeds sets margins of 1.5, 1.2 or 1.1 times the target's strengths.
- **Need predicates and collectors.** `FUN_004ae150`, `FUN_004ae240`, `FUN_004ae320`,
  `FUN_004ae440` test a candidate against need classes 1..4; `FUN_004ae480` collects
  units by class bit, `FUN_004af1c0` collects personnel. The emitter turns class 2
  into Planetary Assault `0x234` (or Destroy System `0x223` with flag `0x10`), class 3
  into `0x223` or `0x220`/`0x222`, class 4 into a troop Move `0x201`. hyp: classes 1..4
  are fleet power, ground assault, bombardment and troop transport.
- **AI character-relocation tasks (0x4bb9f0-0x4bca20).** Classes at `0x0065c318` (G,
  ctor `FUN_004bba10`), `0x0065c358` (base, `FUN_004bbd60`) and `0x0065c380` (I,
  `FUN_004bc290`); slot 5 steps them: state 1 strips a character's command (`0x260`),
  state 4 Moves it (`0x201`) toward the target. Class I picks a replacement with the
  mission-planner selectors `FUN_00403460` / `FUN_00403750`. `FUN_004bbba0` and
  `FUN_004bc520` call `FUN_0041b600`.
- **Side.** Only `FUN_004b7150` tests the side (`this+4 != 1`); the floors are fields
  of the acting side's own AI context, so the code is the same for both sides.
  `FUN_00595c40` is not called in the range.

## 0x004bca20-0x004cfed0: AI Task Graph, Droid Advisors and Agent World Model

Feature Wiki page `executable/map-07-4bca20`.

- **Mission planners (class A, vtable 0x0065c3b0; class B "at-system", vtable 0x0065c408).** Twelve per-kind planners share base `FUN_0047aae0` (ctor `FUN_004bce70`, tick slot 11 `FUN_004bced0`). Class B (tick at `0x004bda60`, 9-state target-first) is used by Espionage, Recon, Sabotage, Assassination/Abduction-4. Key fields: `+0x18` planner id, `+0x1c` state, `+0x20` flags (0x80000000 idle, 0x60000000 abort), `+0x24` side, `+0x3c` team list, `+0x44` decoy list. Emits Move order 0x201 and mission order 0x240.
- **AI side-controller containers (vtables 0x0065c450/c460/c470).** Three lists on the side-controller object built by `FUN_004856f0` (vtable 0x0065abf0): record list at `+0x54`, active-task list at `+0x64`, task-group list. `FUN_004be840` loads each module by type tag then calls slot 1 (load) and slot 3 (set_context).
- **Task factory `FUN_004bea20`.** Builds 14 task kinds (1-14) by type id; table at `0x004beda4`. Sizes range from 0x4c (type 14) to 0x8c (type 6). All share base ctor `FUN_004e4fb0` (vtable 0x0065c868). Save/load: vtable slot 1 = load, slot 2 = save.
- **AI module 13 (vtable 0x0065c480): Alliance HQ relocation planner.** Alliance-only (side 1); Empire branch (`FUN_004bf480`) completes immediately. Picks one of 7 modes via `FUN_004bf490` using a force ratio against threshold 1.5 (float at `0x65c49c`). Issues Scrap 0x200 and Move 0x201 of entity `0x20000005` (Alliance HQ) to a target system at `+0x50`. Mode controllers (ctors `FUN_004e60e0`..`FUN_004e6a30`) drive a 6-phase machine `FUN_004e5440`.
- **Droid advisor agents (Empire vtable 0x0065c4a0, 86 slots, `IMP22ACT.SPT`; Alliance vtable 0x0065c4c8, 88 slots, `C3POACT.SPT`).** Same code, different slot layout; Alliance table is 2 slots longer. Slot-per-refusal-code tables map (kind, code) pairs to SPT slots. Mood mapper (slot 7, `FUN_004c2700`/`FUN_004c48e0`): byte `+0x170` from `FUN_00439bf0` maps to return values 1-9 (Empire) or reversed (Alliance). `hyp:` load/save thunks `FUN_004c0850`/`FUN_004c2930` appear in both vtables and may be link-time identical-COMDAT folds.
- **Sound variant picker `FUN_004c4990`.** Loads RT_RCDATA resource `id & 0xffff`; format `[u16 n][u16 v0..]`; n>1 returns a random draw via `FUN_0041cd80(n)`. 23 callers; `FUN_00488030` passes ids 0x131, 0xc9, 0xfb, 0xff.
- **Game message objects (vtables 0x0065c538/c4f0/c570).** Base class c538: 300-tick expiry (`FUN_004c4fe0`), `+0x24` flags (0x10 = unread), `+0x34` category mask, `+0x38` creation tick. Derived c4f0 adds two CStrings, an id and a `FUN_0042dbe0` member; derived c570 adds two CStrings plus an id.
- **Agent world model (Agent+0x3c, `FUN_0049c780`).** Knowledge cache with sector list (vtable 0x0065c5c0, 200-byte records of 10 systems) and system list (vtable 0x0065c5e8, 0x9c-byte records). Tick machine `FUN_0049cb30`: state 2 builds system list (families 0x90..0x97), state 3 builds sector list (families 0x80..0x8f), state 4 sweeps via `FUN_004ebe40`. Sector record `+0x24` bit 1 = all 10 systems fully explored, bit 2 = all have a second property; `+0x60..+0xc7` holds 26 stat dwords.
- **Manage Production/Garrison module base (covered by manage-automation.md).** Base `FUN_004c7d20` with vtables 0x0065c630-0x0065c670. Slot 6 (`FUN_004c7550`): sets `+0x20=1`, clears `+0x30/+0x34/+0x38`, deletes passed object.
- **AI budget modules 0xb and 0xa.** Module 0xb (`FUN_004c8820`, vtable 0x0065c6d8, 30% share, 0x88 bytes): three phases (0x3f8 move/fleet, 0x3f9 ship build, 0x3ec facility build). Issues Move 0x201, Create Fleet 0x270, Build 0x210/0x211, Destination 0x214, Scrap 0x200. Tests hard-coded Coruscant `0x90000109` in `FUN_004c94c0` (not side-gated). Module 0xa (`FUN_004cc6b0`, vtable 0x0065c6f8, 8% share, 0x84 bytes): similar pipeline.
- **CoolDragList item (vtable 0x0065c690) and sorted container (vtable 0x0065c6a8).** Subclass of `FUN_0060ba00`; slot 4 `FUN_004c7f30` walks items whose `+0x68` bit 0 (visible) is set. Comparator slot 2 sorts by `+0xc`.

## 0x004cfed0-0x004deba0: AI Economy Module Family

Feature Wiki page `executable/map-08-4cfed0`.

- **Seven AI module classes (types 5, 4, 14, 7, 3, 1, 6)** created by `FUN_004bea20`, sharing base ctor `FUN_004e4fb0` and vtable shape: VT[4] ready check `FUN_004cc990`, VT[5] step, VT[6] complete `FUN_004dd810`. Common layout: `+0x1c` state (1 = ready), `+0x20` done flag, `+0x24` budget weight percent of side `+0x184`, `+0x30` side id (1 = Alliance, 2 = Empire), `+0x38` phase, `+0x3c` sub-state.
- **Type 5: troop and facility planner (vtable 0x0065c758, weight 4, 0x88 bytes).** Ctor `FUN_004cfb20`. Phase 0x3f4 trains troops: rotates four troop classes 0x5a-0x5d (`FUN_004d0480`), finds a site (`FUN_004d01a0`), prices via `FUN_0041b230(ref, 0x29, ...)`, emits Build 0x212 (`FUN_004d15f0`). Issues Retire 0x242 for special forces (family 0x3c..0x3f) before building. Phase 0x3ec builds facilities via the template.
- **Type 4: mission launcher (vtable 0x0065c778, 0x60 bytes).** Ctor `FUN_004d2220`. Covered by ai-mission-planning.md. Builds primary planner from side `+0x318`/`+0x31c` via `FUN_0042f640`, walks secondary-request list, ticks planners through slot `+0x2c`.
- **Type 14: capital-ship auto-namer (vtable 0x0065c798, 0x4c bytes).** Ctor `FUN_004d2810`. Ready only while side `+0x264 > 0`. Walks capital ships (family 0x14..0x1b); for ships with record flags `+0x24` having 0x4000 and none of 0x8002800, issues Rename order 0x203 from six TEXTSTRA name pools. Alliance pools: 0x5200 (20 names), 0x5260 (35), 0x52c0 (16). Empire pools: 0x5100 (39), 0x5160 (34), 0x51c0 (32). Three u16 counters `+0x44`/`+0x46`/`+0x48` track usage. `hyp:` pool = ship size class (flags 0x10/0x20/0x40).
- **Type 7: ship, fighter and shipyard planner (vtable 0x0065c7b8, weight 17, 0x7c bytes).** Ctor `FUN_004d2ef0`. Phase 0x3f1 fighters: demand `FUN_004d34c0`, source `FUN_004d3610`, Move 0x201, Scrap 0x200. Phase 0x3f2 ships: site `FUN_004d3ff0`, cost against Shipyard `FUN_004d46e0`, Destination 0x214, Build 0x211. Phase 0x3ec facilities via template (token 8, scraps Shipyards 0x28).
- **Type 3: defense, resource and yard planner (vtable 0x0065c7d8, weight 9, 0x7c bytes).** Ctor `FUN_004d6b20`. Phase 0x3ee defense scrap. Phase 0x3ec resources: Coruscant check `FUN_004d76f0` (hard-coded `0x90000109`, not side-gated), Mine/Refinery choice by side `+0x1d0`/`+0x1d4`. Phase 0x3ed yards via template (token 1, scraps Construction Yards 0x2a).
- **Type 1: mine and refinery planner (vtable 0x0065c7f8, weight 4, 0x78 bytes).** Ctor `FUN_004da950`. Finds site (`FUN_004daca0`, world-model fallback `FUN_004daf10`), builds Mine `0x2c000001` or Refinery `0x2d000002` by `+0x1d0`/`+0x1d4` comparison.
- **Type 6: garrison mover (vtable 0x0065c818, weight 22, 0x8c bytes).** Ctor `FUN_004dd460`. Phase 0x3ef runs four Move 0x201 troop passes (`FUN_004ddb70`/`FUN_004de130`/`FUN_004de6f0`/`FUN_004deba0`). `FUN_004dd810` is the shared VT[6] complete of all seven classes.
- **Facility-builder template (nine functions, four clones).** Select site, claim token, scrap for room, quantity, scrap own kind, Destination 0x214, Build 0x210, bookkeeping, reorder. Parameters per class: type 5 token 4 scraps Training Facility 0x29; type 7 token 8 scraps Shipyard 0x28; type 3 token 1 scraps Construction Yard 0x2a; type 1 token 0x80 scraps Construction Yard 0x2a.
- **Rotating build token.** Side `+0x8`/`+0xc` (`FUN_004199b0`) decides which module builds next. Token bits: 1 (type 3), 4 (type 5), 8 (type 7), 0x20 (type 6), 0x80 (type 1).
- **Budget formulas.** Each module takes weight% of side `+0x184`, then a per-branch split minus already-committed spend, clamped to `+0x184 - +0x188`.

## 0x004deba0-0x004ed950: AI Task Classes, Sector-Planner Strategies and Shared Bases

Feature Wiki page `executable/map-09-4deba0`.

- **AI task class 6 (vtable 0x0065c818, ctor `FUN_004dd460`, 0x8c bytes).** Economy+garrison task; three-phase cycle: 0x3ec facility build/scrap, 0x3ef move units, 0x3f0 troop training. Emits Scrap 0x200, Move 0x201, Build facility 0x210, Build troops 0x212, Destination 0x214. Budget share `+0x24` = 22 (0x16). Five source-picker strategies (`FUN_004e0ef0`/`FUN_004e1060`/`FUN_004e11a0`/`FUN_004e12e0`/`FUN_004e1430`) run in rotated order selected by `+0x78` (1..4). Troop budget = 80% of share (`FUN_004e19c0`), facility budget = 20% (`FUN_004e1a20`).
- **AI task class 2 (vtable 0x0065c838, ctor `FUN_004e1e50`, 0x80 bytes).** `hyp:` facility planner, twin of class 6. Budget share `+0x24` = 3. Phases: 0x3ea mine, 0x3eb refinery (spec `0x2c000001` / `0x2d000002` set in `FUN_004e25c0` case 9), 0x3ec yard path. Mine path uses system `+0x40`, rec bit 0x80; refinery uses `+0x44`, rec bit 0x100. Side `+0x2c` fields `+0x1d0`/`+0x1d4` decide mine vs refinery (same logic as player-side `FUN_004c6f00`).
- **Twin comparison.** Type-2 and type-6 share template code that differs only in object offsets, claim-bit masks (0x1000000 vs 0x400000), query constants and ctx+8 request bits (0x40 vs 0x20). `FUN_004e3a40` vs `FUN_004e0170` (both 1333 bytes) differ in 2 lines; `FUN_004e4300` vs `FUN_004e0c90` (both 314 bytes) differ in one offset.
- **Seven sector-planner strategy classes (0x30-byte objects).** Selected by switch at `FUN_004bf7c0` on `agent+0x5c` (values 1..7). Vtable slot `+0x0c` returns the class id (1..7). Six-state machine `FUN_004e5440` drives them: state 2 scans map objects for flags (0x80000, 0x10000, 0x20000), state 3 calls vt[6] (`FUN_004e8290`, mission priority table), state 4 calls vt[7] (`FUN_004eaaa0`, proposal kind 1 or 6), state 5 calls vt[8] (derived execute), state 6 finalizes.
- **Strategy ready-ratio and force-ratio selection.** Each class reads a float 0.9 from its vtable data (e.g. `_DAT_0065c8e4`). `FUN_004bf490` picks the strategy using a force ratio against 1.5: ratio < 1.5 and threat -> mode 4; low stock -> mode 2 or 7; else modes 1, 3, 5, 6 by resource and threat conditions.
- **Shared evaluators.** `FUN_004e8290`: mission priority table (Recruitment 0x55, Research 0x53 sub 1-3, Diplomacy, Incite, Subdue, Espionage, Sabotage, Assassination, DS Sabotage when `this+4==1`). `FUN_004eaaa0`: proposal kind 1 or 6; kind 6 only when `this+4==2`.
- **Strategy base (vtable 0x0065c898, ctor `FUN_004e52c0`).** Fields: `+4` side id (1 Alliance, 2 Empire), `+8` state-machine phase, `+0xc` controller, `+0x1c..0x2b` merged candidate-list helper (`FUN_004355d0`). `FUN_004e5b10` maps role bits to facility types 0x2b/0x2c/0x2d.
- **StreamList (vtable 0x0065c858, secondary at +0xc 0x0065c860).** Multiple-inheritance class; load `FUN_004e4e20` reads header, count, `+0x28` flag, then per entry: id -> `FUN_0051f730` (object factory by type id), vt[1](stream), Add.
- **Agent module base (vtable 0x0065c868, base of 0x0065ca30).** Ctor `FUN_004e4fb0(typeId, owner)`: 14 ctor callers, 43 dtor callers. Load `FUN_004e5070`, Save `FUN_004e50c0`.
- **Character vtable slots.** `FUN_004ed060` (gap): Force rank text by `+0x8c` thresholds (<10 None, <20 Novice, <80 Trainee, <100 Jedi Student, <120 Jedi Knight, else Jedi Master; resource ids 0x8749, 0x8750-0x8754). Order eligibility: `FUN_004ed560` (Mission 0x240: injured -> reason 3, prisoner -> reason 2), `FUN_004ed750` (rank requirement 0..3 maps to any officer / commander / admiral / general; unmet -> reason 4).
- **Per-system strategic summary `FUN_004ebe40`.** Called 10 times per group from `FUN_004c6770`. Side branches: param_2 == 1 -> ours=1, enemy=2; param_2 == 2 -> ours=2, enemy=1. Numbers are 1-based side ids, not the 0/1 convention.

## 0x004ed950-0x00508c53: Character, GameObj, Fleet and Unit Classes

Feature Wiki page `executable/map-10-4ed950`.

- **Character class (0x004ed950-0x004f2d00).** Shorts `+0x7c..+0x9a` (16 skill/stat fields), dwords `+0x9c..+0xa8` (4 id fields), flags `+0xac`. Every setter clamps to 0..0x7fff via `FUN_0053e0f0`, notifies both side views through `FUN_00539fd0(1)`/`(2)`, then calls a class hook. Save layout: 16 shorts, 4 dwords (`FUN_004ece90` each), flags dword (`FUN_004ef7e0` load, `FUN_004ef940` save). Flag bits on `+0xac`: 0x01 (alive/eligible), 0x02 (same-side check via `(this+0x24 ^ other+0x24) & 0xc0`), 0x04 (lookup from `+0x8c`), 0x10 / 0x20 / 0x40 / 0x80 / 0x100 / 0x200 each with dedicated setter and notifier pair.
- **Character notification senders (0x004f1c80-0x004f2400).** 29 functions including `CharacterInjuryNotif` (event 0x1e2), `CharacterCommandKindNotif` (0x1e3), `CharacterEncounterKeyNotif` (0x360). Each validates then registers the event for both side views.
- **Object lookup helpers (0x004f2f60-0x004f5c40).** Id-to-object finders filtered by type-id range: 0x08-0x0f (fleets), 0x14-0x1c (capital ships + fighters), 0x1c-0x1f (fighters), 0x20-0x21, 0x30-0x3f (characters), 0xa0-0xaf (managers). Three intrusive pointer lists (vtables 0x0065d030, 0x0065d040, 0x0065d1c8) with 0x20/0x1c-byte nodes. Five thin side-object classes (vtables 0x0065d068..0x0065d210).
- **SideViewedObj / GameObj (0x004f5ea0-0x004fcc70).** Entity base with per-side view (`this+0x24 >> 4 & 3`, views 1/2 = the two sides, view 0 = global). Reparent/move validation (`FUN_004faf50`, `FUN_004fb310`), visibility (`FUN_004f9a60`), status text (`FUN_004f6030`, resource 0x4da1). Flag-bit change notifiers on `+0xac`/`+0x58` bits 7..9 call vtable slots `+0x3dc..+0x3e4`. Game context globals: `DAT_006b2ae0`/`006b2af4`/`006b2af8`.
- **Fleet class (0x004fd670-0x004ffdb0).** Children iterated with `FUN_004fcd00`; ship ids 0x14-0x1b. Same-owner test: `(elem+0x24 ^ this+0x24) & 0xc0`. `FUN_004ff040` checks all ships same side and few. `FUN_004ff690` validates ships can perform action 5. `FUN_004fddd0` is the fleet-level action validator (pushes status pairs via `FUN_00520580`).
- **CapitalShip class (0x004ffdd0-0x00502bd0).** Class data at `+0x6c`/`+0x70`, shield nibble `+0x64`. `FUN_00501510` recharge step. `FUN_00500960` and `FUN_00500a10` are the A and B action validators. `FUN_005008a0` tries to notify status. Status pairs pushed: `{1,-1}`, `{1,1}`, `{1,0x17}`.
- **FighterSquadron (0x00502cd0-0x00503930).** `FUN_005036f0` tests "full and not ready". `FUN_00503370` adds count. Class data at vtable 0x0065e640 helpers.
- **SpecialForce (0x00503970-0x00503fe0).** Register class, create, create2.
- **Regiment (0x00504180-0x00504b30).** `FUN_00504790` queries flag-not-bit-3. Guarded operations A and B.
- **Entity finder family (0x00505010-0x00507130).** 46 functions; range-filtered: 0x14-0x1c, 0x30-0x3c, 0x14-0x20 and others.
- **Static-init/atexit thunks** scattered through gaps (`DAT_006b2948..006b2b78`).

## 0x00508c53-0x0052a380: Star System, Game Root, Setup, Scheduler and Missions

Feature Wiki page `executable/map-11-508c53`.

- **Star system class (families 0x90..0x98, vtable 0x0065e640, 0x00508c60-0x00515480).** State in `+0x88` flag word; setters fire change hooks in vtable slots `+0x220..+0x258` which fire notification slots `+0x25c..+0x2e4` (e.g. `SystemEnergyNotif`). Side branches via `this+0x24` bits 6-7: 1 Alliance, 2 Empire, 3 contested, 0 neutral. View 2 shows `100 - loyalty` in `FUN_00511b90`. Notification event ids: Loyalty 0x140, ProductionModifier 0x146, Smuggling 0x145, TroopRegWithdrawPercent 0x147, Bombard 0x14f, Assault 0x150. Spare fleet creation at `FUN_00512d00` (type 0x8000004).
- **Object iterator (vtable 0x0065e92c/e928).** Layout: `+4` owner key, `+8`/`+0xc` type range [lo,hi), `+0x10` filter mask, `+0x14` side-filter flag, `+0x18` side, `+0x1c` cursor. API: First (`FUN_00513120`), Next (`FUN_005130d0`), Count, HasAny, Reset. SystemIterator range 0x90..0x98.
- **Game root object (type 0xf1, vtable 0x0065e938, 0x1b0 bytes).** Three instances per game (master + side views 1 and 2). Stores singleton handles at `+0xb0..+0xdc`. Parameter table loader `FUN_00513f70`: ids 1, 0x200-0x205, 0x600-0x606, 0x800-0x805, 0xa00-0xa21, 0xc00-0xc0f, 0xe00-0xe05, 0x1400-0x1420, 0x1800-0x182e, 0x1c00-0x1c02, 0x1e00-0x1e57 (includes GNPRTB 7691-7720).
- **Victory and support-shift code (0x00515970-0x00519470).** Victory flag word at singleton `+0x74`. Support-shift rules in `FUN_0055de50` family.
- **New-game setup and seeding (0x00519740-0x0051d8d0).** Stage counter drives 27 stages (0x1b start, 0x1a end). Stage 8 looks up class 0xf1000001. Stage 9 creates singleton handles (sides 1/2 per key). Stage 0xa creates sectors (family 0x80, filtered by galaxy size at `+0x5c`). Stage 0x10 places Alliance HQ (`FUN_0051a680`, key at `+0x154`, not fixed) and Empire Coruscant (`FUN_0051a870`, hard-coded `0x90000109`). Stage 0x12 seeds starting forces (CMUNYVTB 0x50, CMUNHQTB 0x51, etc.). Stage 0xc (`FUN_0051a280` → `FUN_0051bc70`) creates the systems. Stage 0x14 creates characters from families 0x30 and 0x38 (`FUN_00562520`: side 1 if only class `+0x40` is set, 2 if only `+0x44`; a class with both or neither is not created). Stage 0x15 (`FUN_0055f7b0`) places the named characters: 0x34000280 (Palpatine) at Coruscant; 0x35000281 (Vader) and 0x38000380-0x38000383, 0x3800038c into a random system or fleet ship of their side (`FUN_00562710`); 0x30000240 (Mon Mothma) at the HQ's container (`FUN_00506110()+0x1c`); 0x32000242 (Luke), 0x31000241 (Leia), 0x33000243 (Han), 0x38000341, 0x38000343, 0x38000344 at Yavin (`maintenance-and-vader-start.md`).
- **Scheduler, order and mission machinery (0x0051d8f0-0x00525f30).** Scheduler singleton `DAT_006b2fcc` (vtable 0x0065ecf4, 0xc4 bytes): `+0x10` day, `+0xa4` queue, timer queues at `+0xa8..+0xb0`. Order base vtable 0x0065ed28. Mission object (vtable 0x0065edf8) handles mission state, member iteration and detection.
- **Tail classes (0x00525fb0-0x0052a360).** Mission member iterators, sector object, Alliance HQ facility class (family 0x20, ctor `FUN_005267c0`, vtable 0x0065f280), defense-facility classes, production manager (vtable 0x0065fbf8) with queue, completion, field sync/notifiers.
- **Event and timer handler classes.** Object-event handlers at 0x00514510-0x00515480 fire from vtable slot `+0x20`. `FUN_00514a60` ("system battle orchestrator") triggers space combat `FUN_005457f0` and travel start `FUN_00556430` when an object arrives in a container. `hyp:` event ids by factory adjacency: 0x300, 0x301, 0x302, 0x307, 0x308 (confirmed), 0x318, 0x31d, 0x320.

## 0x0052a380-0x00547ba0: ManufacturingMgr, SideState, Person, Orders and Combat Phases

Feature Wiki page `executable/map-12-52a380`.

- **ManufacturingMgr (families 0xa0-0xaf, vtable 0x0065fbf8/0x006629a0).** Queue at `+0x54`, Remaining `+0x58`, Completed `+0x5c`, Overflow `+0x60`, SeedKey `+0x64`, Required `+0x68`, TotalRequired `+0x6c`, Reserved `+0x70`, ProductKey `+0x74`, DeploymentKey `+0x78`, TargetKey `+0x7c`, ProductName string `+0x80`. Recompute chain: `FUN_0052a710` -> `FUN_0052a9c0`/`FUN_0052a7d0`/`FUN_0052aac0`. Eleven sync functions compare to source object, each calling a notifier at vtable slot `+0x230..+0x258`. Events: Remaining 0x160, Completed 0x161, Required 0x162, Reserved 0x163, SeedKey 0x400, ProductKey 0x320, DeploymentKey 0x321, ProductName 0x520.
- **Facility families.** Shipyard 0x28 (ctor `FUN_0053a670`, vtable 0x00660490), Training 0x29, Construction Yard 0x2a, Mine 0x2c, Refinery 0x2d. Common yard base class with thunk slots to `FUN_0053ab50`/`FUN_0053abf0`/`FUN_0053ac60`/`FUN_0053b0f0`.
- **SideState class (vtable 0x660af8, 0xdc bytes, key family 0xf3, one per side).** Maintenance and mine/refinery capacity pairs, raw and refined stockpiles with waiter queues, R&D order and done flags. Events 0x120-0x12c. Timers 0x381/0x382. Side key: `FUN_004f3dd0` builds `0xf3000000 | (side != 1 ? 0x10 : 0xf)`, so game side 1 (key 0xf) = Alliance, 2 (key 0x10) = Empire; object `+0x24` bits 6-7 hold 1 (0x40) Alliance, 2 (0x80) Empire.
- **Person base class (0x005336b0-0x00535c90).** Eight skill shorts `+0x58..+0x66`, four keys, role bits `+0x78`. View-sync observers per field.
- **Order classes 0x210-0x216.** Build facility 0x210, Build ship 0x211, Build troops 0x212, Stop 0x213, Destination 0x214, Rename 0x215, Reserved 0x216. Personnel and facility order lists.
- **Move orders 0x201/0x202/0x204.** Order construction and dispatch.
- **ProcFacil update handlers.** Facility/ship class records, the registry singletons.
- **GNPRTB binding block (0x0053e0a0-0x0053e450).** Derived formulas at 0x0053e900-0x0053ed00 feed constants the port should confirm against `data/base`.
- **Stream helpers.** `FUN_005f4d90` reads 4 bytes (Load), `FUN_005f4db0` writes 4 bytes (Save), `FUN_005f60f0` feeds a CRC-32 accumulator (table at `0x6bdf20`). The third pass of every Save/Load/third trio is a checksum pass (`hyp:` multiplayer or save validation).
- **Event and timer factories (0x100-0x621 events, 0x300-0x396 timers).** Factory registration via `FUN_0051ef80`.
- **Story-check chain (0x00544030-0x00544fc0).** `FUN_00549910` runs ten MissionMgr story checks in order, each setting a bit of the story word `+0x58`: `FUN_005442f0` HanCapturedAtPalace, `FUN_00544da0` FinalBattleReady, `FUN_00544030` LukeDagobah, `FUN_00544130` LukePalace, `FUN_005443f0` LeiaPalace, `FUN_005444e0` ChewbaccaPalace, `FUN_005445d0` DarthPickup, `FUN_00544a20` DarthToLukeFinalBattle, `FUN_00544be0` DarthToEmperorFinalBattle, `FUN_00544fc0` FinalBattle. None applies damage (`space-combat.md`).
- **Mission creation helpers.** `FUN_005420d0` lists candidate records (non-hidden MISSNSD). `FUN_005422f0` preflight filters. `FUN_005429e0` creates for popup menu. `FUN_00542b60` creates for order 0x250. `FUN_005438a0` checks named majors: Luke 0x32000242, Leia 0x31000241, Han 0x33000243, Chewie 0x38000343, Vader 0x35000281, Emperor 0x34000280 (gates the story-check chain).
- **Entity event hooks (0x00545180-0x00547ba0).** Deployed 0x308, destroy/add/reparent, arrival-leg end 0x306, mission state change. Timers 0x392/0x393. Notification senders for message ids 0x41000001-0x44000004.

## 0x00547ba0-0x00564240: Mission/Story Manager, Schedulers, Deployment

Feature Wiki page `executable/map-13-547ba0`.

Source: the Feature Wiki page executable/map-13-547ba0.

- **MissionMgr** (vtable `0x661c08`, family 0xf8/0xf9, size 0x74). Side story manager. Object offset +0x58 holds a story-flag dword with named bits: LukeDagobahRequired (bit 0), LukeDagobah (bit 2), LukePalace (bit 5), HanCapturedAtPalace (bit 6), LeiaPalace (bit 7), ChewbaccaPalace (bit 8), DarthPickup (bit 9), DarthToLukeFinalBattle (bit 10), DarthToEmperorFinalBattle (bit 11), FinalBattleReady (bit 12), FinalBattle (bit 13), BountyHuntersActive (bit 14). Twelve broadcasters at `0x54a140`-`0x54a770` propagate flags, twelve notif slots at `0x54b770`-`0x54ba50` emit named strings.
- **MissionMgr timers** 0x392 and 0x393 (vtables `0x661f48`, `0x661f80`). 0x392 arms the Dagobah-required delay; 0x393 fires on BountyHuntersActive. Target type range {0xf8, 0xf9}.
- **Scheduler queues** (three concrete queues at offsets +0xa8, +0xac, +0xb0 of the scheduler object). TimerQueue (`0x661fd8`, ordered by item+0x40 due day), FIFO (`0x661ff8`, append-only), PriorityQueue (`0x662018`, sorted by slot +0x28 result). All share a tree container base (`FUN_005f6170`); run path `FUN_0051df30` pops heads, items with slot +0x20 true are reinserted.
- **CmdBase hierarchy** (vtable `0x662038`). Base for serialisable commands, events and timers. Layout: +0x18 int (default 1), +0x1c int (default 0), +0x20 context sub-object (28 bytes, vptr `0x65d42c`), +0x3c DatId key. Derived layers: ScheduledItem (+0x40 due day), ObjectCmd (+0x40 payload), CmdPlain (no new fields).
- **Mission phase hooks** (`0x547ba0`-`0x549580`). Entry via `FUN_00546ea0`; executes opponent mapping (`0x547f60`), deployment checks, and the prisoner location chain (`0x548da0`-`0x549850`).
- **Mission rule table** (entries at MissionMgr+0x70, count at +0x6c). 12-byte entries {DatId, fnA, fnB}, one per MISSNSD record. Lookup `FUN_0054c720` by DatId family 0x40..0x7f.
- **Command 0x250** (vtable `0x661e28`, size 0x74, mission create command). Slots: validate -> `FUN_005429e0`, execute -> actor slot +0x1bc.
- **Id -> factory registry** (table at `DAT_006b92f0`, stride 8, cap 0x3ff). `FUN_0054f140` registers pairs; 73 pairs for events 0x300+ and timers 0x380+.
- **Type-range table** (`DAT_006bb2f0`, 9 x 8-byte {lo,hi} entries). Indices map to unit families: [0x14,0x18), [0x18,0x1c), [0x1c,0x20), etc. Used by room-node builders.
- **Room result classes**. RoomResultX (vptr `0x6620dc`, 0x1c bytes), RoomResultZ (vptr `0x6620e4`), RoomLimitY (vptr `0x6620e0`, 0xc bytes). The candidate comparator `FUN_0054fbb0` implements "no refusal beats a refusal."

## 0x00564240-0x005771b0: Class Registration — Missions, Characters, Systems, Events, Timers

Feature Wiki page `executable/map-14-564240`.

Source: the Feature Wiki page executable/map-14-564240.

- **22 mission classes** (families 0x41-0x44, 0x51-0x58, 0x61-0x65, 0x69, 0x6a, 0x71-0x73). Each block has the same boilerplate: constructor, copy constructor, destructor, scalar-deleting destructor, clone, registry factory, GetFamily, Notif-key init, static initialisers. Behaviour slots: +0x274 success chance (MSTB table lookup), +0x27c per-member roll (`FUN_0053e2f0(chance)`), +0x280 world effect, +0x284 shared `FUN_00592c80`.
- **Per-mission behaviour** (all read from decompiles). Subdue 0x57: ends revolts, adds support. Incite 0x56: starts uprisings. Sabotage 0x69 / DS Sabotage 0x6a: world effect destroys target via slot +0xac. Rescue 0x61: releases captive (+0x210). Abduction 0x62: captures (+0x20c). Assassination 0x63: kills (+0x2ec) if +0x50 bit 3 allows. Palace 0x64: releases target and all prisoners at +0x74; failure captures the team. Recon 0x54: no roll, sets observation level 8. Research 0x53: adds RdPoints to side research record. Diplomacy 0x51: adds system support on result 3. Recruitment 0x55: picks a minor character (family 0x38..0x3c) for the side. Jedi Training 0x58: teacher at +0xa8, level checks and RemoveMember override.
- **Hero validators**. Palace: Luke 0x32, Leia 0x31, Chewbacca 0x38 only; rejects Han (error 0x30). Vacation/Dagobah: family 0x32 only (error 7). Pickup: family 0x35 only (error 0xc). Bounty: rejects Han as first target (error 0x30).
- **7 character classes** (families 0x30-0x35, 0x38). Object size 0xb4. Luke and Han keep a replicated flag word at +0xb0. Flag trios: authoritative setter (guarded by `FUN_0053a000`), per-side applier, Save/Load helpers. Story notifs: LukeKnowsHeritage (event 0x211), LukeDagobahCompleted (event 0x210), HanBountyAttack (event 0x212), LukeFinalBattleReady, LeiaKnowsHeritage, HanCapturedByBountyHunters, DagobahMissionFirstTrainingDay.
- **5 event classes** (0x30f, 0x310, 0x311, 0x31b, 0x319) plus the tail slots of 0x309 (its factory lies before the range). 0x54-byte objects with 12-slot vtable. Execute targets `FUN_00515c00`-`FUN_00515ef0`; 0x319 calls `FUN_0050d5a0(0xb, ...)` on system lookup.
- **8 timer classes** (0x383-0x386, 0x38d-0x390). Base `FUN_00585fe0`, 0x4c bytes. 0x383-0x386 target systems 0x90..0x98 (slot +0x28 shared stub `FUN_00566320`). 0x38d uprising, 0x38e informant (same target range). 0x38f disaster, 0x390 resource (target 0xf9..0xfa, stub `FUN_00566670`).
- **Core system** (family 0x90, vtable `0x663698`). Overrides: Populate (`FUN_00566d40`, special-cases +0x18 == 0x109), facility seeder (`FUN_00566de0`, loops +0x5c times), revolt flag at +0x88 bit 2. **Rim system** (family 0x92, vtable `0x664098`): seeder only when +0x88 bit 0 (populated). **Special system** (family 0x98, vtable `0x669260`): story systems 0x98000442 (Vacation) and 0x98000481 (Palace, Pickup).
- **Container 0xf2** (vtable `0x6639b8`). A non-system container.
- **SeedTableMgr** (singleton `DAT_006bb7a4`). Loads 17 SeedKey tables (resource type 0x642, ids 0x6a4-0x6b5 except 0x6b1). SeedKey format: high byte = table family, low 24 bits = entry id.
- **Table registry** (`FUN_00569300`). 17 family-range to parser registrations (0x08-0x10, 0x10-0x14, ... 0x90-0x98). Five entry type names: IntTableEntry, SideParamTableEntry, SeedFamilyTableEntry, SeedTableEntry, GeneralParamTableEntry.
- **Serialisation helpers**. ParamBlocks (20+3x10 ints), CharGrid (w x h strings), polymorphic ObjList, StrArray10, ObjList2, KeyTree (5-dword keys), Rec5Int.

## 0x005771b0-0x0058ef80: Events, Commands, Orders, Ground Assault, Bombardment

Feature Wiki page `executable/map-15-5771b0`.

Source: the Feature Wiki page executable/map-15-5771b0.

- **TeamGroup** (vtable `0x669260`, 0x58 bytes). Mission team/group object. `FUN_005775a0` validates children as characters (family 0x30..0x3f), all off-mission (+0x78 bit 7 clear) and same side. `FUN_005771b0` checks order-kind acceptance against a static set.
- **Material waiter events** 0x370 / 0x371 (vtables `0x669420` / `0x669450`, 0x40 bytes). Raw waiter fire `FUN_00577a20` pops from side raw queue (+0x88) and feeds a refinery in state 1. Refined waiter does the same for yards. These id numbers also exist as UI notifications in a separate registry; the overlap is by address, not role.
- **Side/facility timers** 0x381, 0x382, 0x394 (vtables `0x6694b8`, `0x669480`, `0x6694f0`). 0x382 overdrawn maintenance: locks one random completed object when side +0x58 < +0x70/+0x74. 0x381 side maintenance: sums `FUN_00528040` across all same-side systems. 0x394 facility cycle: one build-progress tick when +0x58 == 2.
- **Fleet/ship commands** 0x200 Scrap, 0x201 Move, 0x202 Confirmed Move, 0x205 (no UI label). Move command (`FUN_00578d00`): resolves object and destination, checks blockade (`FUN_005152e0`), speed, side. Execute (`FUN_00578f30`) sets transit bit 5 and changes container.
- **Keyed events** 0x300-0x320. Each 0x54-byte object with always-ok slots 6/7 and a slot-8 fire handler. 0x300 ObjChanged (recomputes maintenance), 0x301 ObjDestinationAdded, 0x302 ObjSideChange, 0x306 AutoroutingEnded, 0x308 ObjDeployed, 0x320 transit start (fires `FUN_00514a60`). Also 0x31a, 0x30a CharStatusChanged, 0x317, 0x313.
- **Product commands** 0x210-0x216 (Build, Stop, Destination, Rename, Reserved). **Bombard commands** 0x220/0x221. **Mission commands** 0x240/0x242. Dead command 0x3f0 (passes id 1 to `FUN_0053fd60` which only accepts 0x600-0x621).
- **Order classes**. Planetary Assault 0x234, Bombardment 0x220-0x223, Create Fleet 0x270, Rank 0x260-0x263.
- **MissionCheck object** and serial helpers for tally lists.
- **GNPRTB entry record** (0x44 bytes, eight difficulty values). Factory `FUN_00585640` allocates 0x44 bytes then calls `FUN_005855f0`.
- **Timer base** (vtable `0x66a230`, `FUN_00585fe0`). Slot +0x20 `FUN_005862a0`: fires when the target record counter is unchanged, then reschedules. Slot 4 runs CRC checksum via `FUN_005f60f0`.
- **Mission target classes** and decoy functors.
- **Ground assault resolution** (`0x58c120`-`0x58c8b0`). **Bombardment resolution** (`0x58d770`-`0x58e320`).
- **System distance table** and **table registry** (33 table ids). **SeedList**.

## 0x0058ef80-0x005a54ee: DAT Record Classes, Mission Bases, Tactical Engine Entry

Feature Wiki page `executable/map-16-58ef80`.

Source: the Feature Wiki page executable/map-16-58ef80.

- **DAT table container** (vtable `0x66a980`, 0x30 bytes). Slots: create_instance, load list, save list, size. Load/save gated by `FUN_00569240`/`FUN_00569250`/`FUN_005691f0`. Entry list at +0x2c.
- **DAT entry base** (vtable `0x66a9d0`). Common base for parameter-table entry classes. Layout: +0x18 id, +0x1c field2 (default 1), +0x20 threshold (default 0), +0x24 value/packed.
- **DAT entry type registry** (table `DAT_006bbe60`, stride 0x2c, limit 10). Five registered types: IntTableEntry (`FUN_0058fb90`), SideParamTableEntry (`FUN_005858c0`), SeedFamilyTableEntry (`FUN_0058f460`), SeedTableEntry (`FUN_0058f1a0`), GeneralParamTableEntry (`FUN_00585640`).
- **Per-entity DAT record classes**: basic (families 0x08-0x10, 0xf0-0xff, 0x98-0xa0, 0xa0-0xb0), troop (0x10-0x14), system (0x90-0x98), special force (0x3c-0x40), sector (0x80-0x90), facilities (0x28-0x2c), mission (0x40-0x80), characters (0x30-0x38), fighter (0x1c-0x20), capital ship (0x14-0x1c). Each has the same 8-method shape: ctor, scalar-deleting dtor, dtor, factory, save, load, size, text parse.
- **AgentMission base** (vtable `0x66abc0`, ctor `FUN_005923a0`). Shared handlers for MSTB missions plus Reconnaissance, Research, Palace, Bounty. `FUN_00592aa0` is the +0x1bc handler. `FUN_00592f50` is +0x1f4 (phase-10 roll). `FUN_00592c80` is +0x284 (end). `FUN_00593320` wraps +0x274 (success chance).
- **Four intermediate mission bases**. (1) Transit-mission base (vtable `0x66b0e0`): for Move 0x41, Return 0x42, Autorouting 0x43, Adrift 0x44; +0x1f4 -> base `FUN_00524b70`, not AgentMission. (2) Sabotage-mission base (vtable `0x66ae50`): for Sabotage 0x69, DS Sabotage 0x6a. (3) Location-mission base (vtable `0x66b378`): for Pickup 0x73, Vacation 0x72, Dagobah 0x71; validator requires target family 0x98..0x9f. (4) Character-mission base (vtable `0x66a558`) for Rescue 0x61, Abduction 0x62, Assassination 0x63; further derived classes are not mapped.
- **Character base class** with story notif and flag-word helpers.
- **Tactical engine free-function API** (`0x595b10`-`0x595f60`). Wrappers that fetch the engine singleton then call a method, bridging strategic code (`FUN_0040b0e0`, `FUN_0040b8a0`, `FUN_0040bcd0`) to tactical code.
- **Tactical engine singleton** (vtable `0x66bae8`, `DAT_006bc248`, 0x6b8 bytes). Created by `FUN_00597530`/`FUN_00597610`. Offset +0x668 selects windowed (TacticalMgr at `DAT_006bc4a0`) vs headless (TacticalBattle at `DAT_006bc4a8`, 0x274 bytes). Constructor is 9 KB with ship-type and unit-state name tables.
- **Tactical window** (vtable `0x66bb00`, `DAT_006bc45c`). Hosts map view (+0x108), right panel (+0x10c), optional popup (+0x110).
- **Tactical UI controller** (`DAT_006bc468`, 0x188 bytes). Input arrives as three queued event kinds (select, command, click). Mode objects (0x54 bytes): idle (0), non-ship (0xd), own ship (0xe), foreign ship (0xf), plus group modes.
- **Tactical input/selection state machine** (`0x5a1f30`-`0x5a54ee`). States for one unit (vtable `0x66bce8`), free selection (`0x66bc98`), control groups A (`0x66bc48`) and B (`0x66bbf8`). Handlers dispatch tactical events 0x30-0x3d through singleton vtable +0x28.

## 0x005a54ee-0x005c2990: Tactical Battle Engine Core

Feature Wiki page `executable/map-17-5a54ee`.

Source: the Feature Wiki page executable/map-17-5a54ee.

- **TacticalGame** (vtable `0x66c2d0`, singleton `DAT_006bc4a0`, accessor `FUN_005aaf90`). Constructor `FUN_005a7500` registers about 90 message names (SHIP_ADD, SHIP_REMOVE, NAV_*, SHIPGROUP_*, DEATHSTAR_*, ALLIANCE/EMPIRE_START/END_TURN, SYSTEM_*, SHIP_FIRELASERCANNON, SHIP_TURBOLASER, SHIP_IONCANNON, SHIP_TORPEDO, SHIP_TAKE_*_HIT; strings at `0x6ab724`-`0x6abd40`). Destructor `FUN_005a88a0`. Installs 7 CShipGroupHandler (`0x66bdd0`), nav handler (`0x66bdb8`), event handlers.
- **Object manager** (vtable `0x66bed8`, `DAT_006bc470`, 0x48 bytes). Four hash maps, per-side id counters (Alliance starts at 0, Empire at 1,000,000). Lookup `FUN_005a6340` is used by hundreds of call sites (node offset 0xd4). +0x44 holds the Death Star object.
- **Cue/event objects** (vtables `0x66bb58`-`0x66be80`). 0x54-byte objects with 0x50-byte vtables.
- **Per-tick game loop**. `FUN_005a96b0` drains the tick-stamped message queue, switching on type 0x03-0x4a (switch table at `0x5a9bc4`). `FUN_005aacb0` runs the battle state machine (this+0x8b4, states 1..5). `FUN_005a9ec0` detects end of battle and picks the result cue (Alliance local 0x96-0x99, Empire local 0x118-0x11b). `FUN_005ab0e0`/`FUN_005ab650` set up collision radii, sounds and side layout.
- **TacShip base** (vtable `0x66c560`). 32 message executors (hit, repair, damage, tractor beam, jump). Offsets: +0x168 owner side, +0xec object id, +0x178 current target id, +0x170 unit states. Fire control: `FUN_005b3a40` target scan, `FUN_005b3f10` fire.
- **TacCapitalShip** (vtable `0x66c140`). Derived from TacShip.
- **TacFighterSquadron** (vtable `0x66c028`). Fighter group building creates red/blue/green/gold groups.
- **Weapon-arc records** (0x74 bytes, 4 per ship at this+0x17c).
- **Hangar launch/dock manager** (vtable `0x66c698`). Carrier launch and dock operations.
- **Death Star object**. Tactical Death Star with DEATHSTAR_UPDATE, DEATHSTAR_FIRE, DEATHSTAR_WITHDRAW messages.
- **Tactical sound manager** (`DAT_006bc4c8`, 0x1420 bytes, ids below 0x13d). Destruction-voice cue handler.
- **Save/load record walkers**. Records of 100 and 0x4c bytes, list-of-record archive routines.
- **132 static initialisers**. Fill 4-byte message-id records (0xad81-0xff0b) and float squares. Listed in .data tables at `0x6a6400`-`0x6a660c`.
- **TacBattleController** (vtable `0x66c7c8`, `DAT_006bc4a0`). Manager field walkers, advisor speech tick, commander rating and maneuver factor.
- **TacNetBattleController** (vtable `0x66c880` = `0x66c7c8` + 0xb8). Network lockstep twin. Slot 0x94 sends message 0x4b instead of writing locally.
- **Tac3DWindow** (vtable `0x66c960`, `DAT_006bc700`, 640x480 8bpp). Flip/present routine `FUN_005c25b0`.

## 0x005c2990-0x005d895e: Space Battle Object Layer

Feature Wiki page `executable/map-18-5c2990`.

Per-faction battle AI, task force lifecycle, order execution, and side-specific
logic. 382 Ghidra functions + 102 recovered gap functions + 262 initializer
bodies. About 5.7 KB is panel code; the rest is the TacticalSide / TaskForce
subsystem.

- **TacticalSide** (base vtable `0x0066cc48`): per-faction battle AI root.
  Ctor `FUN_005c7cb0` (0xb4 bytes; Alliance vtable `0x0066c428`, Empire
  `0x0066c3f8`, created in `FUN_005af5f0`). Side layout: +0x8 control mode, +0x3c strength,
  +0x40 timer, +0x58/+0x64 ship lists (`0x0066bdd0`), +0x70 task forces
  (`0x0066c390`), +0x7c/+0x80/+0x84 retreat timer/flag/state. Empire gets
  Death Star bonus strength; Alliance has order 6 (trench run).
- **TaskForce** (base vtable `0x0066cd28`, +0x1c sub-vtable `0x0066cd10`;
  derived: capital `0x0066bf70`, hyp: fighter group `0x0066bee0`). Ctor
  `FUN_005c9db0`. Manages stance, range mode, formation anchor, lead id,
  and per-unit order dispatch.
- **Order executors**: 11 order codes (1-11), each with its own executor
  vtable. Code 6 = Death Star executor (vtable `0x0066d0a8`), Alliance only,
  120000 ms window.
- **Side branching**: Empire-specific voice IDs and Death Star bonus in side
  ctor; Alliance-only order code 6 and distinct message IDs for trench run.
- **Keyed node lists** (`0x0066bdd0`, `0x0066bdb8`): generic list containers
  for ship and nav-point groups. Base vtable `0x0066caf0`; key at node+0x18,
  next at +0x10.
- **Steering and waypoint construction**: `FUN_005cc300` (`TaskForce::Update`,
  vtable `+0x14`; `tactical-engagement-ai.md`), waypoint approach with speed
  table and decel ramp. Message tables for steering
  state are in this range.
- **Retreat / stalemate logic**: retreat timer at side +0x7c, flag +0x80,
  state +0x84. Stalemate detected when both sides retreat simultaneously.
- **Formation visual base** (`FUN_005ea030` references): lists `0x0066bdd0`
  used in formation anchoring.
- **TacEvent dispatch**: per-tick event loop calls order executors via vtable
  slots. `FUN_005c2c80` is the main tick driver (also calls
  `TacSceneEffect_tick`).
- **TEXTTACT string initializers**: 2 tables of 131 TEXTTACT string-key
  static init bodies (same 131 ids at different globals); map-19 holds six more.

## 0x005d895e-0x005e9bcb: D3DRM Wrapper, Cameras, Tactical Panels

Feature Wiki page `executable/map-19-5d895e`.

Retained-mode 3D wrappers, camera system, scene effects, and the 7 right-hand
tactical command panels. 92 Ghidra functions + 829 recovered + 800 static-init
thunks. The static-init bodies are 6 blocks of 131 TEXTTACT string-key
initializers (same 131 ids at different globals).

- **RmObject** (vtables `0x0066d320` at `+0` and `0x0066d310` at `+0x1c`):
  retained-mode wrapper base. AddVisual/DeleteVisual, frame management,
  position/orientation queries. Ctor `FUN_005d8980` (creates its frame through
  IDirect3DRM slot `0x10`); every COM call is checked by `FUN_005d8b00`.
- **RmFramedVisual**: framed visual with mesh, wraps D3DRM mesh builder and
  frame. Derived from RmObject.
- **RmCamera**: D3DRM camera wrapper. Viewport creation, projection, clip
  planes.
- **TacOrbitCamera**: orbit camera around selected object (index 9). Smooth
  zoom, azimuth/elevation, auto-track. hyp: main battle view camera.
- **TacChaseCamera**: chase camera following a selected ship and its target.
  No port counterpart.
- **TacSceneEffect** (vtables `0x0066d500`/`0x0066d4f0`): timed visual
  effect. Ctor `FUN_005da150` builds an 8-vertex box of half-width 0.005
  with green (0,1,0) colour; `FUN_005da830` fades by elapsed time. hyp:
  Death Star superlaser beam (strings 0xada1/0xadae/0xdd86). Created by
  `FUN_005d59e0` via vtable dispatch.
- **7 right-hand command panels** (type codes):
  - `0x15` Idle seal (no-ship-selected)
  - `0x16` Twin capital-ship panel (ctor `FUN_005e6b90`; `FUN_005e7030` creates its controls)
  - `0x17` Fighter-group panel (ctor `FUN_005e1170`)
  - `0x18` Capital-ship panel
  - `0x19` Missions panel
  - `0x1a` Maneuvers & Tactics panel
  - `0x1b` Battle Options panel
- **Light rig**: `FUN_005d4d10` sets ambient/directional/spot lights,
  palette realization, material and filtering state.

## 0x005e9bcb-0x005fd7c0: Formations, Class Library, DirectPlay, Video, Bitmap

Feature Wiki page `executable/map-20-5e9bcb`.

Four distinct slices: tactical formations, a foundation class library,
multimedia/comms, and bitmap containers. 519 Ghidra functions + 420 recovered.

- **Formation base** (vtable `0x0066d8e0`): abstract formation with anchor,
  facing, member list. Subclasses: sphere, line, B (hyp: block). Mover and
  steering helper at formation level.
- **Surround / Stand Off tactics**: formation-level tactics that orbit or
  hold distance from a target. Target policies, range band logic.
- **Keyed list items, AVL tree, iterator**: generic container classes shared
  across the codebase. AVL tree with balance factors, in-order iteration.
- **Projectile mesh**: 3D mesh for projectile rendering in the battle view.
- **3D_WINDOW containers**: scene-graph window objects holding D3DRM frames
  and visuals.
- **Withdraw confirm panel** (TACTICAL string 1310): modal dialog presented
  when a player requests withdrawal from battle.
- **Str class** (vtable `0x0066dca8`): dynamic string with StrCursor.
  Reference-counted, copy-on-write.
- **NameValueList parser**: INI-style key=value parser. Used for registry
  reads and game configuration.
- **Lehmer RNG** (multiplier `0xa18e`): the original game's pseudorandom
  number generator. Seeded from system time.
- **IdTree**: integer-keyed tree. Used for object registries.
- **OrderedList, SerialValue, CRC-32** (polynomial `0x04C11DB7`): ordered
  collection, serial counter, and CRC used for multiplayer state hashing.
- **VideoManager**: manages SmackerWindow and AviWindow playback. Smacker
  for cutscenes, AVI for intro sequences.
- **CommMgr** (DirectPlay `IDirectPlay2`): session/transport layer for
  multiplayer. Session enumeration, join, host, send, receive.
- **Packet, buffer, ack classes**: reliable packet framing over DirectPlay
  with acknowledgment and retransmit.
- **GameWindowObject**: base game window with message pump integration.
- **HiResTimer**: `QueryPerformanceCounter`-based timer.
- **DIB Bitmap class**: device-independent bitmap wrapper for blitting and
  palette operations.

## 0x005fd7c0-0x00610b70: Cool* Window and Control Library

Feature Wiki page `executable/map-21-5fd7c0`.

Shared window and control library for the entire game UI. 407 Ghidra
functions + 50 recovered, all of them the windowed control toolkit; none is
networking.
No side-branching logic in any function.

- **CoolBitmap**: RLE8 blitting with palette remap. Mode-1 (standard) and
  mode-2 (8-byte stub that draws nothing, `FUN_005fe8d0`).
- **GAME_WINDOW_OBJECT**: base for all game windows. Associates object with
  HWND via window-long slot 0.
- **CoolWindow base** (WndProc `FUN_00600310`): `CreateWindowExA` via
  `FUN_005ffce0`, message dispatch to virtual slot +0x14. Records move,
  size, and focus changes. `SC_*` system-command filter policy.
- **CoolStrobeButton** (ctor `FUN_00602150`): animated button with 4 state
  bitmaps (+0x94 normal, +0x98 pressed, +0x9c disabled, +0xa0 selected).
  Sound on click (+0xdc sound id, +0xe0 embedded sound object vtable
  `0x00658518`). Repeat-fire via WM_TIMER (+0xfc rate, +0x104 threshold).
- **Checkbox**: toggle variant of CoolStrobeButton.
- **Slider**: horizontal track with draggable thumb.
- **CoolStringField**: text input field with type-ahead.
- **CoolGameWindow** (ctor `FUN_00606380`, vtable `0x0066e060`): game-level
  window with drag support, policy bits (+0xb8), background bitmap, and
  focus save/restore. `FUN_006073b0` saves the previous focus HWND at
  +0x10c; dtor `FUN_00606580` restores it.
- **CoolDragList**: draggable item list with reorder support.
- **ScrollBar** (vertical and horizontal): vertical `FUN_0060aa70`, horizontal
  `FUN_00610900`. hyp: `FUN_00610900` guards on both axes where the vertical
  twin tests one (bug in original).
- **CoolSelectionBoxClass**: multi-select list with keyboard navigation.
- **CoolTabControl**: tab bar managing child panels. Sets +0xa4/+0x10c/+0x110
  on each child during tab-switch bitmap rebuild.
- **CoolwinStatusBar**: progress meter (hyp: loading screens).
- **Modal mouse hook**: `WH_MOUSE` hook for modal dialog mouse-swallowing.
  Installed/removed per dialog lifecycle.
- **Tooltips**: tooltip window created and positioned on hover.

## 0x00610b70-0x00629000: Sound, DirectDraw, iostream, CRT

Feature Wiki page `executable/map-22-610b70`.

Sound engine, display initialization, a pre-STL iostream library, and VC6
CRT startup. 572 Ghidra functions + 84 recovered.

- **Sound library** (DirectSound mixer): WAVE resource streaming, event
  dispatch thread, keyed descriptors (vtable `0x00658518`, 6 dwords, keyed
  by resource id + DLL module). API surface: `SndInit`, `SndShutdown`, `SndPlayResource`
  (`FUN_00610c30`), `SndPlayFile`, `SndStop`, `SndStopInstance`, `SndSetVolume`,
  `SndSetMasterVolume`, `SndRefreshState`. Callers, ids and modules: `audio-dispatch.md`.
  Multiple simultaneous sounds via DirectSoundBuffer pool.
- **MCI CD audio**: `mciSendCommand` for CD device management: open, close (0x804),
  stop (0x808) and status (0x814). `MCI_PLAY` and volume control are not in this range.
- **Music**: 16 WAV files `MDATA.300`-`315` streamed from `<install dir>\MDATA\`
  by a 0..15 track index (table `0x006a8478`, `FUN_00417520` in map-01); the
  advice agent, Battle Alert and battle result window choose the track
  (`audio-dispatch.md`). Not CD audio.
- **Smacker glue**: hooks Smacker video playback into the sound mixer for
  synchronized audio during cutscenes.
- **DirectDraw wrapper**: global at `0x006b1460`. Loads `cdderr.dll` for
  error strings. Display mode: 640x480x8bpp. Surface creation, palette
  management, flip chain.
- **Game message 0x280** (set game speed): vtable `0x0066f488`, stores
  speed word at target+0xc4. Dispatched through the standard game-message
  system.
- **Pre-STL Microsoft iostream**: `ios`, `streambuf`, `strstreambuf`,
  `filebuf`, `istream`, `ostream`. Linked from the VC6 runtime, not
  custom. Used for file I/O in the save system and configuration parsing.
- **VC6 CRT**: `WinMainCRTStartup` (`entry`, `0x00619d80`), `operator new`
  (`FUN_00618b70`, 1343 callers), `operator delete` (`FUN_00618b60`,
  3430 callers).
- **rand (MSVC LCG)**: standard MSVC linear congruential generator.
  Separate from the Lehmer RNG in map-20.
- **`__ftol`** (float-to-long truncation): 47 call sites. Truncates toward
  zero (C99 default), relevant for parity since Rust `as i32` matches this
  behavior for normal values but differs on NaN/infinity.
- **Pentium FDIV workaround**: runtime detection of the 1994 Pentium
  floating-point division bug, with software fallback.

## What the port lacks, by range

### map-01-401000

- BattleSession state machine drives the strategic-to-tactical-and-back flow; the
  headless path (abstract flag +0xc=1) differs from the played path (+0xc=0).
- Side convention: Alliance=side 1, Empire=side 2 in the cockpit and lobby; side
  controller uses Alliance=0, Empire=1. Convert at the boundary.

### map-02-41a460

- Session RNG (`FUN_0041cd80` = `FUN_005f5700`, state at CPlayerMgr+4) is needed for
  AI coin flips; it resets to 1 on turn-wrap.
- The 97 notification ids registered by `FUN_0041e580` define the id-to-handler map
  the port needs for its Rust notification enum.
- GID modes are partially reconciled in cockpit.rs but the full mode table in
  `FUN_00425d00` has not been ported.

### map-03-43eca0

- The notification id-to-handler map (97 ids from `FUN_0041e580`) defines which
  handler to invoke for each change. The port issues free-text messages instead of
  implementing the per-class event handlers; porting the visitor-slot dispatch would
  give parity.
- Fleet notification ids 0x180-0x183 and their visitor offsets are needed for
  blockade/bombardment/battle result popups.

### map-04-475890

- Game speed intervals (1:600, 2:60, 3:12, 4:4 ticks) belong to the per-player
  interface range and must match game_speed.rs.
- mission_planning.rs and ai.rs cite many functions from this range. The per-kind
  planner subclasses define target selection logic the port needs.
- The Assassination planner's different base class (`FUN_004bd810`) means it cannot
  share the same slot-offset assumptions as other mission kinds.

### map-05-48a3b0

- The port issues free-text messages rather than implementing the per-class event
  handlers. message_log.rs, message_index.rs, and main.rs are all relevant.
- The AgentSurvey tally system is not ported. agent_automation.rs ports
  Production/Garrison decisions but lacks the underlying per-system/per-sector data.
- The Han bounty/kind-0x1a collision is a save-load fidelity concern.

### map-06-4ae150

- The port has no posture engine. ai.rs is rule-based, and mission_planning.rs cites
  `FUN_004b7150` (the slot-6 mission recommender) while noting that posture is not
  ported. Porting the six posture classes would give the AI its regional behaviour.
- Order codes Move 0x201, 0x220, 0x222, 0x223, 0x234 and Command: None 0x260 are the
  posture engine's vocabulary; the port must support all of them for posture parity.
- The character-relocation tasks (0x4bb9f0-0x4bca20) strip a character's command and
  Move it for the mission planners; the port has no equivalent.

### map-07-4bca20

- The AI budget modules (0xa, 0xb, module 13) and the task container/factory (`FUN_004bea20`) are not in the port. A save-compatible reimplementation needs the load/save order of each module, the 14 object sizes in the factory table, and the three container layouts.
- The advisor slot-per-refusal-code tables (Empire 86 slots, Alliance 88) are needed by `crates/rebellion-render/src/advisor_script.rs` for correct clip selection; the 4-byte slot-offset shift between sides must be checked.
- The agent knowledge cache (Agent+0x3c) sector summaries are what `agent_automation.rs` garrison and production selection consults.
- Message 300-tick expiry (`FUN_004c4fe0`) and category masks are visible behaviour for `crates/rebellion-render/src/message_log.rs`.

### map-08-4cfed0

- None of the 14-type AI module family is ported. `crates/rebellion-core/src/ai.rs` `evaluate_production` picks a best fighter and capital ship from fleet counts; the original instead splits side funds by fixed weights and rotates a work token.
- `crates/rebellion-core/src/fleet_name_bank.rs` and `fleet-names.md` need a note about the original's Rename 0x203 pass.
- `crates/rebellion-core/src/mission_planning.rs` already follows ai-mission-planning.md for the type 4 module.

### map-09-4deba0

- The port's AI mission planning lives in `crates/rebellion-core/src/mission_planning.rs` and `missions.rs`; nothing in `crates/` reimplements the type-2 or type-6 task phases or the sector-planner strategies. If the port wants original-style AI facility and troop building, the needed pieces are the task phases and orders (type 2 and 6), the strategy ready-ratio and 1.5 force-ratio selection, and the mission priority table at `FUN_004e8290`.

### map-10-4ed950

- Character field setters clamp to 0..0x7fff and notify both sides' views before the class hook; the port must reproduce this pattern.
- Character save layout (16 shorts, 4 dwords, flags) overlaps `ghidra/notes/capital-ship-save-layout.md` and `entity-system.md`.
- Fleet legality (`FUN_004ff040`, `FUN_004ff690`, `FUN_004fddd0`), capital-ship action validation (`FUN_00500960`, `FUN_00500a10`), shield recharge nibble, and type-id range filters must be reproduced.
- Most of the range is plumbing the port does not need to copy directly (scalar-deleting destructors, SEH unwind helpers, `FUN_00520580` status pairs); a Rust port keeps the information as ordinary state-change events.

### map-11-508c53

- Worth porting from uncovered code: the `+0x88` bit-to-hook-to-event table, the support-shift rules, the victory flag word, setup stage 0x10 capacity top-up, the stage 0x14 side rule for characters (compare `seeds.rs`), and the per-side view replica/loyalty inversion.
- The port's system, mission and seeding state sits in `crates/rebellion-core/src/` (missions.rs, economy.rs, seeds.rs).

### map-12-52a380

- `crates/rebellion-core/src/stockpiles.rs`, `manufacturing.rs` and `build_selection.rs` already implement the SideState stockpile/maintenance rules and ManufacturingMgr queue. The ProductKey/DeploymentKey naming above should be checked if the port's save layout follows build-delivery.md.
- `missions.rs` and `movement.rs` cover mission-create helpers and move orders. The entity hook fan-out (deployed 0x308, arrival 0x306) is only partly mirrored (`scrap.rs`, `uprising.rs`).
- Not modelled: SideState `+0x90..+0xc4` (R&D points, order/done flags, events 0x125-0x12c), the Person view-sync observers, the GNPRTB binding block and derived formulas.
- The combat phase drivers belong to `tactical_view.rs` and the space-combat notes.

### map-13-547ba0

The room-result algebra (`FUN_0054fca0`, `FUN_0054fd90`, `FUN_0054fc40`) and the three queue kinds (timer-ordered, FIFO, priority) are architectural patterns the port must replicate for the deployment and scheduling systems.

### map-14-564240

The port's `missions.rs` rejects Sabotage targets with two error codes, but the other 21 mission classes' error codes and the TEXTSTRA message text behind them are not replicated. Each mission's +0x274/+0x27c/+0x280 slot addresses are catalogued and can be matched against the port's mission handlers.

### map-15-5771b0

The CRC pass in slot 4 (`FUN_005f60f0`) is hyp: the lockstep state checksum; if so, every field serialised in Load/Save must also appear in the Checksum slot, or multiplayer desyncs. The port should replicate this three-stream discipline for any multiplayer mode.

### map-16-58ef80

The four intermediate mission bases (transit, sabotage, location, character) define which missions share behaviour and which override. The port's mission type hierarchy should mirror these four inheritance paths. The tactical engine's windowed-vs-headless branch at +0x668 is directly relevant to the port's headless battle runner.

### map-17-5a54ee

Side convention is 0 = Alliance, 1 = Empire throughout the tactical engine (engine+0x9ec local side, unit+0x168 owner, group+0x50 owner). The port's `tactical_view.rs` already references `FUN_005a7490` (side facing back-off) and `FUN_005a8a70` (random range). The 90-name message table has no port mirror; the battle controller's windowed-vs-headless split (TacBattleController vs TacNetBattleController) maps to the port's headless tactical runner.

### map-18-5c2990

Retreat/stalemate logic, waypoint construction, and steering message tables
are not represented in the port. The 11 order executor vtable dispatch
pattern has no port counterpart.

### map-19-5d895e

TacChaseCamera has no port counterpart. TacSceneEffect (hyp: superlaser)
has no port counterpart.

### map-20-5e9bcb

Formation code is sourced by `crates/rebellion-render/src/tactical_view.rs`
constants. The Lehmer RNG (multiplier `0xa18e`) is original behavior that the
port must match for deterministic replay. CRC-32 is multiplayer state hashing
(not needed for single-player port parity).

### map-21-5fd7c0

The port reimplements these controls per-screen rather than sharing a single
toolkit. Modal mouse-swallowing, `SC_*` system-command policy filter, drag
modes, type-ahead, and repeat-fire button behavior may be absent or
incomplete. The horizontal scroll-bar axis bug (if confirmed) may affect
the port's scroll behavior.

### map-22-610b70

Sound addressing matches the port's `audio.rs` resource tables. `__ftol`
truncation semantics at 47 call sites are a parity concern: Rust `as i32`
truncates toward zero for normal floats (matching `__ftol`), but saturates
on overflow/NaN where x87 `FISTP` gives the integer indefinite `0x80000000`. Any
call site that can receive out-of-range floats needs explicit parity handling.
