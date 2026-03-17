# DLL Resource Catalog

Complete extraction of all 18 game DLLs. 5,917 total resources (2,705 BMPs + 3,212 data files).
Organized by DLL with resource ID ranges, image dimensions, and content descriptions.

## Extraction Location

All extracted to `assets/references/ref-ui-full/{dll-name}/` and `assets/references/ref-ui/{category}/`.

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

## COMMON.DLL (2.8MB) — 321 BMPs + 1 BIN

Main menu and global UI elements — buttons, sliders, panel chrome used across all screens.

| ID Range | Count | Dimensions | Content |
|----------|-------|-----------|---------|
| 10000-10999 | 101 | Various (4x4 to 47x47) | **Core UI toolkit**: scrollbar tracks, slider thumbs, checkbox sprites, radio buttons, progress bars, panel dividers, scroll arrows |
| 11000-11999 | 215 | Various (33x28 to 63x64) | **Button sprites**: every clickable button in the game across all screens (pressed/unpressed/disabled × all functions). Galaxy map buttons, fleet management, manufacturing, mission dispatch |
| 15000-15999 | 3 | 29x26, 30x26 | **Miscellaneous icons** |
| 20000-20999 | 2 | 640x480 | **Main menu backgrounds** — the title screen |

**Key assets:**
- 20001: Main menu background (640x480) — the iconic Rebellion title screen
- 11001-11215: 215 button sprites — this is the complete UI button library
- 10001-10101: Core widgets (scrollbars, checkboxes, sliders)

## TACTICAL.DLL (7.5MB) — 288 BMPs + 1 BIN

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

## ALSPRITE.DLL (16MB) — 74 BMPs + 716 BIN

Alliance advisor: C-3PO and R2-D2 animated sprites. The BIN files are animation sequence data (frame ordering, timing, coordinates).

| Content | BMP Count | Dimensions | Notes |
|---------|-----------|-----------|-------|
| C-3PO animation frames | ~62 | 67x116 | 12+ gesture/speech sequences |
| R2-D2 animation frames | ~12 | 47x69 | 2+ sequences (beep/rotate) |
| Animation control data | — | (716 BIN files) | Frame ordering, timing, coordinates |

## EMSPRITE.DLL (19MB) — 74 BMPs + 713 BIN

Empire advisor: Imperial protocol droid animated sprites. Same structure as ALSPRITE.

| Content | BMP Count | Dimensions | Notes |
|---------|-----------|-----------|-------|
| Imperial droid frames | ~74 | 67x116 | 22+ gesture/speech sequences |
| Animation control data | — | (713 BIN files) | Frame ordering, timing, coordinates |

## ALBRIEF.DLL (7.9MB) — 20 BMPs + 366 BIN

Alliance briefing screen resources. The BMP sprites are advisor frames shown during the briefing. The BIN data is likely audio + animation control for the briefing sequences.

| Content | Count | Dimensions | Notes |
|---------|-------|-----------|-------|
| C-3PO briefing sprites | 15 | 67x116 | Subset of ALSPRITE frames |
| R2-D2 briefing sprites | 5 | 47x69 | Subset of ALSPRITE frames |
| Briefing sequence data | 366 BIN | — | Audio clips, animation control, cockpit compositing data |

**Note:** The cockpit/shuttle background is NOT a single BMP. It is either composited at runtime from layered elements, or rendered as part of the Smacker video sequences (MDATA.003-005).

## EMBRIEF.DLL (8.2MB) — 18 BMPs + 471 BIN

Empire briefing screen resources. Same structure as ALBRIEF but for the Imperial bridge environment.

## REBDLOG.DLL (290KB) — 24 BMPs

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

Voice lines stored as RT_WAVE resources (resource type 302, custom). Our extraction script only handles RT_BITMAP (2) and RT_RCDATA (10) — voice extraction needs a separate tool targeting resource type 302.

## ENCYBMAP.DLL (34KB)

Encyclopedia bitmap mapping table — data-only, maps system IDs to EDATA file numbers. No visual resources.

## TEXTCOMM.DLL / TEXTTACT.DLL (35KB + 34KB)

Common and tactical text strings. String-only DLLs, no bitmap resources.

## Smacker Videos (MDATA/)

15 Smacker (.SMK2) video files in `~/Desktop/Programming/star-wars-rebellion/MDATA/`:

| File | Size | Content | Resolution |
|------|------|---------|-----------|
| MDATA.000 | 2.8MB | Intro logo/splash | ~320x200 |
| MDATA.001 | 39MB | Main intro cinematic | ~320x200 |
| MDATA.003 | 16MB | Campaign setup 1 (faction select cockpit) | ~320x200 |
| MDATA.004 | 20MB | Campaign setup 2 (difficulty/options) | ~320x200 |
| MDATA.005 | 20MB | Campaign setup 3 | ~320x200 |
| MDATA.101-108 | 230MB | 8 story event cutscenes | ~320x200 |
| MDATA.201 | 12MB | Victory sequence | ~320x200 |
| MDATA.202 | 8MB | Defeat sequence | ~320x200 |

**Codec:** Smacker 2 (RAD Game Tools). Decode via `ffmpeg -i MDATA.001 output.webm` or RAD tools.

## WAV Soundtrack (MDATA/300-315)

16 WAV files with John Williams Star Wars soundtrack excerpts. See MetasharpNet `medias/MDATA/readme.txt` for full track listing with timestamps.

## Summary Statistics

| Category | BMPs | BIN/Data | Total |
|----------|------|----------|-------|
| STRATEGY.DLL | 1,042 | 96 | 1,138 |
| GOKRES.DLL | 580 | 0 | 580 |
| COMMON.DLL | 321 | 1 | 322 |
| TACTICAL.DLL | 288 | 1 | 289 |
| ALSPRITE.DLL | 74 | 716 | 790 |
| EMSPRITE.DLL | 74 | 713 | 787 |
| ALBRIEF.DLL | 20 | 366 | 386 |
| EMBRIEF.DLL | 18 | 471 | 489 |
| REBDLOG.DLL | 24 | 0 | 24 |
| ENCYTEXT.DLL | 0 | 348 | 348 |
| TEXTSTRA.DLL | 0 | 511 | 511 |
| **Total** | **2,441** | **3,223** | **5,664** |

Plus: 15 Smacker videos (356MB), 16 WAV soundtrack (17MB), ~280 voice WAVs (in VOICEFX DLLs, not yet extracted).
