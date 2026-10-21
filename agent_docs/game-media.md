---
title: "Game Media Reference"
description: "Working inventory of visual and audio media in Star Wars Rebellion (1998)"
category: "agent-docs"
created: 2026-03-16
updated: 2026-10-19
tags: [dll, smacker, wav, audio, video]
---

# Game Media Reference

Working inventory of visual and audio media in Star Wars Rebellion (1998).
Read when working on asset extraction, upscaling, total conversion, or media playback.

## Source Files

All media lives in the game installation at `~/Desktop/Programming/star-wars-rebellion/`.

### DLL Resources (Win32 PE)

| DLL | Size | BMP Resources | BIN/Data | Content |
|-----|------|--------------|----------|---------|
| COMMON.DLL | 2.8MB | 321 | 1 | Main menu backgrounds, core UI toolkit, 215 animated cockpit/control frames |
| STRATEGY.DLL | 29MB | 1,042 | 96 | Galaxy map chrome, facility indicators, event screens, character diplomacy panels, UI chrome |
| TACTICAL.DLL | 7.5MB | 288 | 1 + 87 meshes + 397 textures + 66 WAV | Combat HUD, controls, meshes, textures, and audio |
| GOKRES.DLL | 2.9MB | 580 | — | Entity status sprites (troops, ships, fighters, characters, facilities, damage diagrams) |
| ALSPRITE.DLL | 16MB | 38 | 752 + 1,640 custom frames + 213 WAV | Alliance advisor anchors, animations, control data, and audio |
| EMSPRITE.DLL | 19MB | 34 | 753 + 2,348 custom frames + 216 WAV | Empire advisor anchors, animations, control data, and audio |
| ALBRIEF.DLL | 7.9MB | 20 | 366 + 2,684 custom frames + 17 WAV | Alliance briefing anchors, animations, control data, and audio |
| EMBRIEF.DLL | 8.2MB | 18 | 471 + 2,738 custom frames + 22 WAV | Empire briefing anchors, animations, control data, and audio |
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

Resource ID ranges per DLL: `dll-resource-catalog.md`.

Locale `1033` = English (US). All extractions are English-locale.

### Smacker Video Files (MDATA/)

15 Smacker (.SMK2) video files decoded to WebM (VP9+Opus) in `assets/references/ref-videos/`.
16 WAV soundtrack files in `MDATA/` directory.

| File | Size | Content |
|------|------|---------|
| MDATA.000 | 2.8MB | Intro logo/splash screen |
| MDATA.001 | 39MB | Main intro cinematic (Star Wars crawl + setup) |
| MDATA.003 | 16MB | Alliance new-game movie (0x68) |
| MDATA.004 | 20MB | Empire new-game movie (0x69) |
| MDATA.005 | 20MB | Credits (0x6e) |
| MDATA.101 | 6MB | System Destroyed (0x6f) |
| MDATA.102 | 19MB | Rebel HQ Destroyed (0x73) |
| MDATA.103 | 25MB | Coruscant Captured (0x74) |
| MDATA.104 | 16MB | Death Star Sabotaged (0x72) |
| MDATA.105 | 40MB | Ending: Alliance wins, Alliance viewer (code 0x10) |
| MDATA.106 | 40MB | Ending: Empire wins, Alliance viewer (code 0x11) |
| MDATA.107 | 47MB | Ending: Alliance wins, Empire viewer (code 0x13) |
| MDATA.108 | 34MB | Ending: Empire wins, Empire viewer (code 0x12) |
| MDATA.201 | 12MB | Successful X-wing trench run; Death Star destroyed |
| MDATA.202 | 8MB | Failed trench run; X-wing destroyed |

MDATA.002 is named by movie 0x67 but absent from the install. Movie ids map to
files in `crates/rebellion-app/src/movies.rs` (`file_number`, `FUN_00421ba0`).

**Codec**: SMACKW32.DLL (Smacker 2 format, RAD Game Tools).
**Resolution**: 640x324 (most), 640x480 (splash). Higher than expected for 1998.
**Decoded**: All 15 videos converted to WebM (VP9+Opus) in `assets/references/ref-videos/`. Total: 271MB.
**Command**: `ffmpeg -i MDATA.001 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus output.webm`

### WAV Soundtrack (MDATA/300-315)

16 WAV files with John Williams soundtrack excerpts:

| File | Content |
|------|---------|
| MDATA.300 | *Return of the Jedi* / Battle of Endor shuttle-menu cue (0:35) |
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
| 07-droid-advisors | 72 | 1505 | ALSPRITE + EMSPRITE.DLL; custom frames and WAV omitted |
| 08-backgrounds-screens | 83 | — | STRATEGY.DLL + Resources |
| 09-death-star-controls | 8 | — | TACTICAL.DLL |
| 10-weapon-gauges | 5 | — | TACTICAL.DLL |
| 11-alliance-briefing | 20 | 366 | ALBRIEF.DLL |
| 12-empire-briefing | 18 | 471 | EMBRIEF.DLL |
| 13-dialogs | 24 | — | REBDLOG.DLL |
| **Staged standard subset** | **448** | **2342** | Custom frames, meshes, textures, and WAV omitted |

### Standard BMP/RCDATA Extraction

Standard bitmap and RCDATA subsets are extracted to
`assets/references/ref-ui-full/`. The extractor does not yet handle custom PE
types 301, 302, or 303, so the visual extraction is not complete. See
`agent_docs/dll-resource-catalog.md` for the full inventory.

Voice lines: 285 WAV files (153 Alliance + 132 Empire) in
`ref-ui-full/voice-alliance/` and `ref-ui-full/voice-empire/`.

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
| Droid advisor frames | 72 standard anchors plus 9,410 custom advisor/briefing frames | Decode originals first | Do not infer or generate parity frames |
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
