---
title: "DLL Resource Catalog"
description: "Original game resource inventory, including standard and custom PE resource types"
category: "agent-docs"
created: 2026-03-17
updated: 2026-09-10
tags: [dll, resources, bmp, sprites]
---

# DLL Resource Catalog

Working inventory of the owned English-US game modules, organized by DLL with
resource ID ranges, dimensions, and content descriptions. Earlier counts only
included standard bitmaps and RCDATA. The live PE inventory also contains
thousands of custom type-302 animation frames, type-301 meshes, type-303
textures, and embedded WAV resources.

## Extraction Location

Extracted to two locations:
- `assets/references/ref-ui-full/{dll-name}/` — STRATEGY, GOKRES, COMMON, TACTICAL, REBDLOG, ENCYTEXT, TEXTSTRA, voice DLLs
- `assets/references/ref-ui/{category}/` — ALSPRITE, EMSPRITE (07-droid-advisors), ALBRIEF (11-alliance-briefing), EMBRIEF (12-empire-briefing), and curated subsets from all DLLs

## STRATEGY.DLL (29MB) — 1,042 BMPs + 96 BIN

The largest visual DLL. Contains all strategy-layer UI: galaxy map, event screens, diplomacy, character panels.

| ID Range | Count | Dimensions | Content |
|----------|-------|-----------|---------|
| 900-999 | 7 | 37x37, 607x437, 640x481 | Galaxy map backgrounds, display toggles, full-screen overlays |
| 1000-1999 | 73 | 400x200, 400x203 | **Character diplomacy/encyclopedia panels** — same format as EDATA |
| 6000-6999 | 61 | 400x200 | **Character event screens** — dialogue scenes, mission briefings |
| 9000-9999 | 33 | 67x35 | **Facility/system indicators** — mine, shipyard, refinery status icons |
| 10000-10999 | 687 | Various (11x11 to 122x50) | **UI chrome mega-block**: buttons, icons, panel frames, scroll arrows, status indicators, faction markers. This is the bulk of the game's UI toolkit. |
| 11000-11999 | 181 | Various (108x27 to 121x49) | **Event result screens** — battle outcomes, bombardment, fleet events, assault results |

**Key assets:**
- 900: Galaxy background (640x481) — the main starfield behind the map
- 1000-1072: 73 character event panels at 400x200 — same format as encyclopedia, usable as character art
- 6208-6268: 61 character dialogue scenes at 400x200
- 10553-10555: Generic UI frames (used as panel backgrounds)
- 10757-10760: Fleet battle result screens (Alliance/Empire victory/defeat)
- 11160-11163: Ground assault/bombardment event screens

## GOKRES.DLL (2.9MB) — 580 BMPs

Entity status sprites. Every ship, character, troop, facility as it appears in info panels.

| ID Range | Count | Dimensions | Content |
|----------|-------|-----------|---------|
| 1-999 | 18 | 122x50, 126x88 | **Facility status icons** — mine, refinery, shipyard, shield, HQ |
| 1000-1999 | 57 | 122x50 | **Ship status views** — hull profile for each capital ship class |
| 2000-2999 | 61 | 80x80 | **Character portraits** — square headshots for officer panels |
| 3000-4999 | 22 | 130x65 | **Ship construction views** — in-progress build graphics |
| 5000-5999 | 27 | 122x50 | **Reactor layout diagrams** — power distribution per ship |
| 6000-6999 | 15 | 122x50 | **Reactor detail views** |
| 7000-8999 | 22 | 130x65 | **Ship damage close-ups** — detail of damaged sections |
| 9000-9999 | 8 | 122x50 | **System-specific status** |
| 10000-10999 | 30 | 122x50 | **Ship damage overview** — full hull showing damage locations |
| 16000-16999 | 13 | 61x25 | **Facility mini-icons** — small icons for fleet/system panels |
| 17000-17999 | 25 | 61x25, 66x25 | **Troop + special force mini-icons** |
| 18000-18999 | 41 | 61x25, 66x25 | **Fighter + capital ship mini-icons** |
| 19000-19999 | 76 | 61x25, 73x48 | **Character mini-icons** — small portraits for lists |
| 21000-22999 | 42 | 61x25, 66x25 | **Ship weapon arc diagrams** |
| 23000-26999 | 60 | 61x25, 66x25, 73x48 | **Ship damage detail icons** |
| 27000+ | 62 | 61x25 | **Additional entity icons** |

**Key assets:**
- 2112-2200: 61 character portraits at 80x80 — the officer headshots shown in character panels
- 1856-1932: Ship status views (122x50) — hull profiles for info panel
- 10048-10126: Ship damage diagrams (122x50) — shows damage locations on hull
- 16385-17216: Facility icons (61x25) — compact icons for panel lists

**Verified fleet miniature mapping (F-010B):**

| DAT record index | Faction/type | GOKRES resource IDs |
|---:|---|---:|
| 1–4 | Alliance fighters | 17984–17987 |
| 5–8 | Empire fighters | 18048–18051 |
| 64–78 | Alliance capital ships | 18240–18254 |
| 128–142 | Empire capital ships | 18304–18318 |

These are record-order mappings, not offsets from the compound `DatId`. Commit
`4589d2e` maps all 38 classes and restricts palette-blue transparency to the
known GOKRES miniature blocks. Astra-medium R9 browser evidence is retained in
`docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-fleet-miniatures.md`.

## COMMON.DLL (2.8MB) — 321 BMPs + 1 BIN

Main menu and global UI elements — buttons, sliders, panel chrome used across all screens.

| ID Range | Count | Dimensions | Content |
|----------|-------|-----------|---------|
| 10000-10999 | 101 | Various (4x4 to 47x47) | **Core UI toolkit**: scrollbar tracks, slider thumbs, checkbox sprites, radio buttons, progress bars, panel dividers, scroll arrows |
| 11000-11999 | 215 | Various (33x28 to 63x64) | **Animated cockpit/control sequences**: faction emblems, spacecraft, planets, starfields, insignia, and disc controls. These are grouped animations, not one three-frame triplet per logical command. |
| 15000-15999 | 3 | 29x26, 30x26 | **Miscellaneous icons** |
| 20000-20999 | 2 | 640x480 | **Main menu backgrounds** — the title screen |

**Key assets:**
- 20001: Main menu background (640x480) — the iconic Rebellion title screen
- 11001-11275 (with gaps): 215 animated cockpit/control frames. `FUN_00405560`
  uses bases including 11001, 11031, 11061, 11091, 11121, 11151, 11181, and
  11241; the logical command mapping still requires handler-level validation.
- 10001-10101: Core widgets (scrollbars, checkboxes, sliders)

## TACTICAL.DLL (7.5MB) — 288 BMPs + 1 BIN + 87 meshes + 397 textures + 66 WAV

Tactical combat view — HUD, ship/fighter sprites, squadron controls, weapon systems.

| ID Range | Count | Dimensions | Content |
|----------|-------|-----------|---------|
| 1000-1999 | 154 | Various (8x8 to 149x236) | **Combat HUD**: task force panels, squadron buttons (4 colors × 3 states), ship highlight/dim toggles, withdraw/recover/attack buttons, Death Star controls, weapon recharge gauges, hull/shield display, mission objective overlays |
| 2000-2999 | 130 | ~102x107 to 126x108 | **Ship tactical sprites**: 3D-rendered capital ship views for the tactical combat display. Multiple orientations per ship class. |
| 40000+ | 4 | 58x22 | **Misc tactical labels** |

**Key assets:**
- 1001-1037: Task force + squadron panel system (faction-colored)
- 1021-1024: Death Star laser controls + gauge
- 1105-1108: Maneuvers/tactics/missions buttons
- 1170-1195: Recover/attack buttons (per-faction × 3 states)
- 1206-1210: Weapon recharge gauge (0-100% in 5 steps)
- 1302: Hull integrity + shield strength panel
- 2001-2130: ~130 tactical ship sprites (the 3D renders used in combat view)

Custom type 301 contains 87 binary DirectX `.x` meshes. Custom type 303
contains 397 texture payloads, including named tactical textures and numeric
16×16 through 256×256 textures. The current extractor and browser pack omit
both custom types and all tactical WAV resources.

## ALSPRITE.DLL (16MB) — 38 BMPs + 752 BIN + 1,640 type-302 frames + 213 WAV

Alliance advisor resources. Standard BMPs are animation anchors; custom type
302 contains the missing 67×116 C-3PO and 47×69 R2-D2 frame runs. BIN resources
hold the linked animation control data.

| Content | Count | Dimensions | Notes |
|---------|-----------|-----------|-------|
| Standard anchor BMPs | 38 | 67×116, 47×69 | Native loader's current visual subset |
| Custom animation frames | 1,640 | 1,617 at 67×116; 23 at 47×69 | Type 302; currently ignored |
| Animation control data | 752 | — | RCDATA/BIN |
| Embedded audio | 213 | — | RIFF/WAVE |

## EMSPRITE.DLL (19MB) — 34 BMPs + 753 BIN + 2,348 type-302 frames + 216 WAV

Empire advisor resources. They use the same anchor, custom-frame, control, and
audio structure as ALSPRITE.

| Content | Count | Dimensions | Notes |
|---------|-----------|-----------|-------|
| Standard anchor BMPs | 34 | 106×133, 101×79 | Native loader's current visual subset |
| Custom animation frames | 2,348 | 2,333 at 106×133; 15 at 101×79 | Type 302; currently ignored |
| Animation control data | 753 | — | RCDATA/BIN |
| Embedded audio | 216 | — | RIFF/WAVE |

## ALBRIEF.DLL (7.9MB) — 20 BMPs + 366 BIN + 2,684 type-302 frames + 17 WAV

Alliance briefing screen resources. The BMP sprites are advisor frames shown during the briefing. The BIN data is likely audio + animation control for the briefing sequences.

| Content | Count | Dimensions | Notes |
|---------|-------|-----------|-------|
| C-3PO briefing sprites | 15 | 67x116 | Subset of ALSPRITE frames |
| R2-D2 briefing sprites | 5 | 47x69 | Subset of ALSPRITE frames |
| Briefing sequence data | 366 BIN | — | Animation control data |
| Custom animation frames | 2,684 | 67×116, 47×69 | Type 302; currently ignored |
| Embedded audio | 17 | — | RIFF/WAVE |

**Note:** The cockpit/shuttle background is NOT a single BMP. It is either composited at runtime from layered elements, or rendered as part of the Smacker video sequences (MDATA.003-005).

## EMBRIEF.DLL (8.2MB) — 18 BMPs + 471 BIN + 2,738 type-302 frames + 22 WAV

Empire briefing screen resources. Same structure as ALBRIEF but for the
Imperial bridge environment. All are currently absent from the browser pack.

## REBDLOG.DLL (290KB) — 24 BMPs + 5 string bundles

In-game dialog box UI chrome.

| ID Range | Count | Dimensions | Content |
|----------|-------|-----------|---------|
| 1528 | 1 | varies | Dialog background |
| 10600-10627 | 17 | varies | Dialog button sprites (OK, Cancel, etc.) |
| 11061-11067 | 7 | varies | Dialog frame elements |

## ENCYTEXT.DLL (143KB) — 348 BIN

Encyclopedia text descriptions — RT_RCDATA resources containing the text shown in the encyclopedia viewer for each entity. Not visual assets.

## TEXTSTRA.DLL (147KB) — 511 BIN

Entity name strings — RT_STRING resources. Already extracted via pelite in the dat-dumper crate. 511 string bundles containing all ship names, character names, system names, etc.

## VOICEFXA.DLL / VOICEFXE.DLL (4.7MB + 4.5MB)

Voice lines stored as RT_WAVE resources (resource type 302, custom). Extracted via custom pefile script — 285 WAV files (153 Alliance + 132 Empire) in `ref-ui-full/voice-alliance/` and `ref-ui-full/voice-empire/`.

## ENCYBMAP.DLL (34KB)

Encyclopedia bitmap mapping table — data-only, maps system IDs to EDATA file numbers. No visual resources.

## TEXTCOMM.DLL / TEXTTACT.DLL (35KB + 34KB)

Common and tactical text resources. TEXTCOMM also contains six dialog templates
and an accelerator; the strategic multiplayer shell uses these alongside
code-built controls. Neither DLL contains the full rendered interface as a
standalone screenshot.

## Smacker Videos (MDATA/) — DECODED TO WebM

15 Smacker (.SMK2) video files decoded to VP9+Opus WebM in `assets/references/ref-videos/`.
Originals in `~/Desktop/Programming/star-wars-rebellion/MDATA/`:

| File | Size | Content | Resolution |
|------|------|---------|-----------|
| MDATA.000 | 2.8MB | Intro logo/splash | 640×480 |
| MDATA.001 | 39MB | Main intro cinematic | 640×324 |
| MDATA.003 | 16MB | Campaign setup 1 | 640×324 |
| MDATA.004 | 20MB | Campaign setup 2 | 640×324 |
| MDATA.005 | 20MB | Campaign setup 3 | 640×324 |
| MDATA.101-108 | 230MB | 8 story event cutscenes | 640×324 |
| MDATA.201 | 12MB | Successful X-wing trench run; Death Star destroyed | 640×324 |
| MDATA.202 | 8MB | Failed trench run; X-wing destroyed | 640×324 |

**Codec:** Smacker 2 (RAD Game Tools). Decode via `ffmpeg -i MDATA.001 output.webm` or RAD tools.

## WAV Soundtrack (MDATA/300-315)

16 WAV files with John Williams Star Wars soundtrack excerpts. See MetasharpNet `medias/MDATA/readme.txt` for full track listing with timestamps.

## Corrected Resource Summary

| Module | Standard BMP | RCDATA/BIN | Custom visual | Embedded WAV |
|---|---:|---:|---:|---:|
| STRATEGY | 1,042 | 96 | 0 | 66 |
| GOKRES | 580 | 0 | 0 | 0 |
| COMMON | 321 | 1 | 0 | 5 |
| TACTICAL | 288 | 1 | 484 | 66 |
| ALSPRITE | 38 | 752 | 1,640 | 213 |
| EMSPRITE | 34 | 753 | 2,348 | 216 |
| ALBRIEF | 20 | 366 | 2,684 | 17 |
| EMBRIEF | 18 | 471 | 2,738 | 22 |
| REBDLOG | 24 | 5 | 0 | 0 |
| ENCYTEXT | 0 | 348 | 0 | 0 |
| TEXTSTRA | 0 | 511 | 0 | 0 |

“Custom visual” combines the 87 type-301 meshes and 397 type-303 textures in
TACTICAL with type-302 advisor/briefing frames. It is not interchangeable with
standard BMP count. The installation also supplies 187 EData images, 15
Smacker videos, 16 music WAVs, and 285 faction voice WAVs. See the
[interface reverse-engineering ledger](../docs/qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md)
for pack coverage and the active decode queue.
