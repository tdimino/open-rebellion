# UI Element Reference Collection

316 UI element files extracted from Star Wars Rebellion (1998) via MetasharpNet editor.
Organized by function for upscaling pipeline and total conversion modding.

## Upscaling Strategy

| Method | Use For | Why |
|--------|---------|-----|
| **Vertex AI Imagen 4.0** (default) | Buttons, gauges, HUD, status panels, indicators | Faithful — no hallucination |
| **Gemini edit** (generative) | Main menu, C-3PO, galaxy backgrounds, event scenes | Benefits from invented detail |

## Categories

### 01 — Main Menu (6 files)
*COMMON.DLL. Title screen background + restart button states.*
- Main screen background (640x480) — the iconic Star Wars Rebellion title
- Restart button: unpressed, pressed, disabled states
- **TC note:** Replace entirely with new faction menu art

### 02 — Galaxy Chrome (5 files)
*STRATEGY.DLL. Galaxy map toggle buttons + generic UI frames.*
- Galaxy display on/off toggles
- Generic strategy UI frame elements
- **TC note:** Keep as-is (faction-neutral chrome)

### 03 — Tactical HUD (29 files)
*TACTICAL.DLL. Combat interface panels and controls.*
- Task force panels: Alliance + Empire variants
- Fighter squadron panels: Alliance + Empire
- Squadron assignment buttons (4 colors × pressed/unpressed/unassigned)
- Ship highlight/dim toggles (per-faction)
- Right panel hull/shield display
- Mission HUD overlays (attack/recover/death star per faction)
- **TC note:** Reskin faction colors (NR blue → keep, Vong → organic green/brown)

### 04 — Buttons & Controls (33 files)
*TACTICAL.DLL. All interactive button sprites with states.*
- Maneuvers/tactics: unpressed, pressed
- Missions: unpressed, pressed
- Withdraw from battle: unpressed, pressed
- Recover units: Empire/Alliance × unpressed/pressed/disabled
- Attack capital ships: Empire/Alliance × unpressed/pressed
- Attack fighters: Empire/Alliance × unpressed/pressed
- Attack Death Star: unpressed/pressed/disabled
- **TC note:** "Attack Death Star" → "Attack Worldship" for Vong TC

### 05 — Facility Indicators (16 files)
*STRATEGY.DLL. Galaxy map facility status icons.*
- Mine: active, Empire construction, Alliance construction, enroute
- Orbital Shipyard: active, Empire/Alliance construction, enroute
- Advanced Shipyard: active, Empire/Alliance construction, enroute
- Refinery: active, Empire/Alliance construction, enroute
- **TC note:** Replace industrial icons with organic Vong equivalents for Vong faction

### 06 — Status Panels (129 files)
*GOKRES.DLL. Unit icons for info panels and fleet management.*
- All 10 troop types (5 Alliance + 5 Empire)
- All 9 special forces (4 Alliance + 5 Empire)
- All 8 fighter types (4 Alliance + 4 Empire)
- All 30 capital ship types (15 Alliance + 15 Empire)
- All 60+ character portraits (31 Alliance + 30 Empire)
- All 11 facility status icons + tab icons
- **TC note:** All entity-specific — full replacement for TC

### 07 — Droid Advisors (2 files)
*Resources. C-3PO advisor images.*
- C-3PO portrait (400x200) — the in-game advisor character
- C-3PO icon (16x16) — toolbar/panel icon
- **TC note:** Replace with Vong equivalent (yammosk tentacle? villip face?)

### 08 — Backgrounds & Screens (83 files)
*STRATEGY.DLL + Resources. Event screens, mission results, galaxy backgrounds.*
- Galaxy background images (640x480, 768x788)
- Mission event result screens (bounty hunters defeated, fleet defeated, etc.)
- Character diplomacy screens
- Battle outcome screens
- **TC note:** Event screens need Vong-themed replacements

### 09 — Death Star Controls (8 files)
*TACTICAL.DLL. Death Star weapon system HUD.*
- Laser ready/fired/loading states
- Laser charge gauge
- Attack Death Star button states
- **TC note:** Replace with Worldship controls for Vong TC

### 10 — Weapon Gauges (5 files)
*TACTICAL.DLL. Weapon recharge progress indicators.*
- 0%, 25%, 50%, 75%, 100% charge states
- **TC note:** Keep as-is (universal combat mechanic)
