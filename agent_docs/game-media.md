---
title: "Game Media Reference"
description: "Complete inventory of all visual and audio media in Star Wars Rebellion (1998)"
category: "agent-docs"
created: 2026-03-16
updated: 2026-03-16
---

# Game Media Reference

Complete inventory of all visual and audio media in Star Wars Rebellion (1998).
Read when working on asset extraction, upscaling, total conversion, or media playback.

## Source Files

All media lives in the game installation at `~/Desktop/Programming/star-wars-rebellion/`.

### DLL Resources (Win32 PE)

| DLL | Size | BMP Resources | BIN/Data | Content |
|-----|------|--------------|----------|---------|
| COMMON.DLL | 2.8MB | 321 | 1 | Main menu backgrounds, core UI toolkit (scrollbars, checkboxes, sliders), 215 button sprites |
| STRATEGY.DLL | 29MB | 1,042 | 96 | Galaxy map chrome, facility indicators, event screens, character diplomacy panels, UI chrome |
| TACTICAL.DLL | 7.5MB | 288 | 1 | Combat HUD, squadron buttons, weapon gauges, Death Star controls, tactical ship sprites |
| GOKRES.DLL | 2.9MB | 580 | — | Entity status sprites (troops, ships, fighters, characters, facilities, damage diagrams) |
| ALSPRITE.DLL | 16MB | 74 | 716 | Alliance advisor: C-3PO animations (12+ sequences) + R2-D2 (2+ sequences) |
| EMSPRITE.DLL | 19MB | 74 | 713 | Empire advisor: Imperial droid animations (22+ sequences) |
| ALBRIEF.DLL | 7.9MB | 20 | 366 | Alliance briefing: C-3PO/R2-D2 sprites + animation control data |
| EMBRIEF.DLL | 8.2MB | 18 | 471 | Empire briefing: Imperial droid sprites + animation control data |
| REBDLOG.DLL | 290KB | 24 | — | Dialog box UI chrome (in-game dialog frames) |
| VOICEFXA.DLL | 4.7MB | — | 153 | Alliance voice lines (WAV, extracted to ref-ui-full/voice-alliance/) |
| VOICEFXE.DLL | 4.5MB | — | 132 | Empire voice lines (WAV, extracted to ref-ui-full/voice-empire/) |
| TEXTSTRA.DLL | 147KB | — | — | Entity name strings (RT_STRING, extracted via pelite) |
| ENCYBMAP.DLL | 34KB | — | — | Encyclopedia system→EDATA index mapping |
| ENCYTEXT.DLL | 143KB | — | — | Encyclopedia text descriptions |
| TEXTCOMM.DLL | 35KB | — | — | Common UI text strings |
| TEXTTACT.DLL | 34KB | — | — | Tactical combat text strings |
| CDDERR.DLL | 44KB | — | — | Error dialog text (no bitmaps) |
| SMACKW32.DLL | 65KB | — | — | Smacker video codec (runtime library) |

### Resource ID Numbering Scheme

| Range | DLL | Content |
|-------|-----|---------|
| 900-999 | STRATEGY.DLL | Galaxy chrome, display toggles |
| 1000-2200 | TACTICAL.DLL | HUD panels, buttons, squadron controls, weapon gauges, Death Star |
| 2000-9000 | ALSPRITE/EMSPRITE.DLL | Droid advisor animated sprite frames |
| 2000-17000 | ALBRIEF/EMBRIEF.DLL | Briefing advisor sprites + animation data |
| 9000-9999 | STRATEGY.DLL | Facility/system status indicators |
| 10000-11700 | STRATEGY.DLL | Event screens, battle results, bombardment |
| 10600-11067 | REBDLOG.DLL | Dialog box UI chrome |
| 17000-17800 | GOKRES.DLL | Troop + special force entity sprites |
| 17900-18600 | GOKRES.DLL | Fighter + capital ship + major character sprites |
| 19000-19100 | GOKRES.DLL | Minor character portraits |
| 20000+ | COMMON.DLL | Main menu screens + restart buttons |

Locale `1033` = English (US). All extractions are English-locale.

### Smacker Video Files (MDATA/)

15 Smacker (.SMK2) video files + 16 WAV soundtrack files in `MDATA/` directory.

| File | Size | Content |
|------|------|---------|
| MDATA.000 | 2.8MB | Intro logo/splash screen |
| MDATA.001 | 39MB | Main intro cinematic (Star Wars crawl + setup) |
| MDATA.003 | 16MB | Campaign setup sequence 1 (cockpit/shuttle briefing) |
| MDATA.004 | 20MB | Campaign setup sequence 2 |
| MDATA.005 | 20MB | Campaign setup sequence 3 |
| MDATA.101 | 6MB | Story event cutscene 1 |
| MDATA.102 | 19MB | Story event cutscene 2 |
| MDATA.103 | 25MB | Story event cutscene 3 |
| MDATA.104 | 16MB | Story event cutscene 4 |
| MDATA.105 | 40MB | Story event cutscene 5 |
| MDATA.106 | 40MB | Story event cutscene 6 |
| MDATA.107 | 47MB | Story event cutscene 7 (largest) |
| MDATA.108 | 34MB | Story event cutscene 8 |
| MDATA.201 | 12MB | Victory sequence |
| MDATA.202 | 8MB | Defeat sequence |

**Codec**: SMACKW32.DLL (Smacker 2 format, RAD Game Tools).
**Resolution**: ~320x200 (original 1998 resolution).
**Conversion**: `ffmpeg -i MDATA.001 output.webm` (requires Smacker support, or use RAD tools).

### WAV Soundtrack (MDATA/300-315)

16 WAV files with John Williams soundtrack excerpts:

| File | Content |
|------|---------|
| MDATA.300 | Battle of Endor medley (0:35) |
| MDATA.302 | Main Title + approaching Death Star + Tatooine |
| MDATA.306 | Emperor Arrives / Death of Yoda / Obi-Wan Revelation |
| MDATA.307 | Attacking a Star Destroyer |
| MDATA.310 | Imperial March / Darth Vader's Theme |
| MDATA.312 | Battle of Hoth medley |
| MDATA.315 | Battle of Endor III medley |

(Full list in MetasharpNet `medias/MDATA/readme.txt`)

## Extraction Status

### Extracted (in `assets/references/ref-ui/`)

| Category | BMP | BIN | Source |
|----------|-----|-----|--------|
| 01-main-menu | 6 | — | COMMON.DLL |
| 02-galaxy-chrome | 5 | — | STRATEGY.DLL |
| 03-tactical-hud | 29 | — | TACTICAL.DLL |
| 04-buttons-controls | 33 | — | TACTICAL.DLL |
| 05-facility-indicators | 16 | — | STRATEGY.DLL |
| 06-status-panels | 129 | — | GOKRES.DLL |
| 07-droid-advisors | 74 | 1505 | ALSPRITE + EMSPRITE.DLL |
| 08-backgrounds-screens | 83 | — | STRATEGY.DLL + Resources |
| 09-death-star-controls | 8 | — | TACTICAL.DLL |
| 10-weapon-gauges | 5 | — | TACTICAL.DLL |
| 11-alliance-briefing | 20 | 366 | ALBRIEF.DLL |
| 12-empire-briefing | 18 | 471 | EMBRIEF.DLL |
| 13-dialogs | 24 | — | REBDLOG.DLL |
| **Total** | **450** | **2342** | |

### Full DLL Extraction Complete

All visual DLLs fully extracted to `assets/references/ref-ui-full/` with per-DLL INDEX.md files.
See `agent_docs/dll-resource-catalog.md` for complete per-DLL resource ID ranges and content descriptions.

**All DLLs fully extracted.** Voice lines: 285 WAV files (153 Alliance + 132 Empire) in `ref-ui-full/voice-alliance/` and `ref-ui-full/voice-empire/`.

### Extraction Tool

```bash
# Extract BMP resources from any Win32 PE DLL
uv run scripts/extract-dll-resources.py ALSPRITE.DLL --output ref-ui/07-droid-advisors/alliance/
uv run scripts/extract-dll-resources.py --all          # Extract all unextracted DLLs
uv run scripts/extract-dll-resources.py --list ALBRIEF.DLL  # List without extracting
```

## Upscaling Strategy

| Asset Type | Count | Method | Notes |
|------------|-------|--------|-------|
| Main menu / backgrounds | ~89 | Gemini edit | Benefits from generative detail |
| Droid advisor frames | 74 BMP | Vertex AI Imagen | Must be consistent, no hallucination |
| Buttons / HUD | ~75 | Vertex AI Imagen | Pixel-perfect state alignment required |
| Entity status sprites | ~129 | Vertex AI Imagen | Faithful entity recognition |
| Facility indicators | ~16 | Vertex AI Imagen | Small icons, preserve exactly |
| Event screens | ~83 | Gemini edit | Narrative scenes benefit from detail |
| Dialog chrome | ~24 | Vertex AI Imagen | Frame edges must be exact |
| Smacker videos | 15 | Frame extraction + Vertex/ESRGAN | Or re-render from scratch |

## Video Upscaling Options

| Tool | Method | Quality | Cost |
|------|--------|---------|------|
| **Frame extraction + Vertex AI** | Extract frames → upscale each → reassemble | Good, consistent | ~$0.01/frame |
| **Real-ESRGAN video** | Built-in video mode | Good, fast | Free |
| **Topaz Video AI** | Neural temporal upscaling | Best | $199 license |
| **Re-render** | Generate new cinematics via nano-banana-pro | Variable | ~$0.13/frame |
| **Skip** | Replace with static/generated screens | Practical | Free |

## Total Conversion Impact

For a Yuuzhan Vong TC, UI media splits into:

**Replace entirely:** Main menu, briefing cockpit (→ organic Vong bridge), droid advisors (→ villip/yammosk), Death Star controls (→ Worldship), victory/defeat videos
**Reskin:** Faction-colored HUD panels, buttons, squadron labels, event screens
**Keep as-is:** Weapon gauges, generic chrome, dialog frames
