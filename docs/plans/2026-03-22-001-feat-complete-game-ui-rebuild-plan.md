---
title: "Complete Game UI Rebuild — Launch to Victory"
type: feat
status: active
date: 2026-03-22
project: open-rebellion
tags: [ui, game-rebuild, cockpit, combat, audio, panels, parity]
---

# Complete Game UI Rebuild — Launch to Victory

## Context

Open Rebellion has a 95% complete simulation engine running in both native and browser, but the player experience is a debug UI — generic egui panels with no game art, no audio, no cockpit interface, no tactical combat view. The original Star Wars Rebellion (1998) had a rich cockpit interface, animated droid advisors, 2,441 bitmap assets, 285 voice lines, 15 cinematics, and a complete tactical combat view. All assets are extracted and cataloged. The simulation-to-UI gap analysis shows 13 systems fully simulated but only 4 with proper player control.

**Current parity: Core 95% | UI 80% (debug) | Combat 60% | Overall ~80%**
**Target: Core 95% | UI 95% | Combat 90% | Overall ~93%**

## Asset Inventory (Already Extracted)

| Source | BMPs | Audio | Video | Status |
|--------|------|-------|-------|--------|
| COMMON.DLL | 321 | — | — | Main menu, buttons, UI toolkit |
| STRATEGY.DLL | 1,042 | — | — | Galaxy chrome, event screens, character panels |
| TACTICAL.DLL | 288 | — | — | Combat HUD, ship sprites, Death Star controls |
| GOKRES.DLL | 580 | — | — | Portraits, ship status, troop/fighter icons |
| ALSPRITE/EMSPRITE | 148 | — | — | Droid advisor animation frames |
| ALBRIEF/EMBRIEF | 38 | — | — | Briefing sprites |
| VOICEFXA/VOICEFXE | — | 285 WAVs | — | Alliance + Empire voice lines |
| Smacker videos | — | — | 15 WebMs | Intro, story events, victory/defeat |
| Soundtrack | — | 16 WAVs | — | John Williams arrangements |
| **Total** | **2,441** | **301** | **15** | **All extracted, none rendered** |

---

## Phase 1: Main Menu & Game Setup (Pre-Game)

### 1.1 Main Menu Screen — P0
Render the title screen (COMMON.DLL 20001, 640x480) with New Game / Load Game / Options / Quit buttons.

**Files**: `rebellion-render/src/main_menu.rs` (new), `rebellion-app/src/main.rs`
**Assets**: 1 background BMP + 4 button sprites (3 states each)
**LOC**: ~150

### 1.2 New Game Setup — P0
Galaxy size selection (Standard/Large/Huge), difficulty (Easy/Medium/Hard), faction selection (replaces current faction_select modal), optional starting bonuses. Mirrors MDATA.003-005 campaign setup flow.

**Files**: `rebellion-render/src/panels/game_setup.rs` (new)
**LOC**: ~300

### 1.3 Intro Cinematic Playback — P1
Play MDATA.001 (intro) and MDATA.003-005 (campaign setup) WebM videos. Requires WebM decoder for WASM (native can shell to ffplay).

**Files**: `rebellion-render/src/video_player.rs` (new)
**LOC**: ~200

### 1.4 TEXTSTRA String Extraction — P0
Extract all entity names from TEXTSTRA.DLL to JSON so "System 12345" becomes "Coruscant." Already partially done (Task 1.3 from WASM plan).

**Files**: `tools/dat-dumper/src/main.rs`, `scripts/build-wasm.sh`
**LOC**: ~80

---

## Phase 2: Cockpit Interface (Main Game Chrome)

### 2.1 Cockpit Frame — P0
The original game wraps the galaxy map in a faction-specific cockpit frame (Alliance: Rebel base cockpit, Empire: Star Destroyer bridge). Load STRATEGY.DLL backgrounds (640x481) and render as a border around the galaxy map canvas.

**Files**: `rebellion-render/src/cockpit.rs` (new)
**Assets**: 2 cockpit backgrounds (Alliance + Empire) + frame chrome elements
**LOC**: ~200

### 2.2 Cockpit Control Buttons — P0
Replace egui keyboard shortcuts with clickable cockpit buttons from COMMON.DLL (215 button sprites). Officers (O), Fleets (F), Manufacturing (M), Missions (N), Research (R), Encyclopedia (E), Save/Load, Game Speed. Each button has unpressed/pressed/disabled BMP states.

**Files**: `rebellion-render/src/cockpit.rs`
**Assets**: ~40 button sprites from COMMON.DLL (subset of 215)
**LOC**: ~250

### 2.3 Droid Advisor System — P1
Animated C-3PO + R2-D2 (Alliance) or Imperial protocol droid (Empire). 148 animation frames + 1,429 BIN control files. Decode BIN frame ordering/timing format. Advisors speak during events, mission results, and combat outcomes.

**Files**: `rebellion-render/src/advisor.rs` (new), `rebellion-core/src/advisor_data.rs` (new)
**Assets**: 148 BMP frames + BIN animation data
**LOC**: ~400

### 2.4 Star Wars egui Theme — P0
Custom `egui::Style`: dark space background (#050510), gold/amber accents matching original UI chrome, monospace military font, faction-colored elements. Applied globally.

**Files**: `rebellion-render/src/theme.rs` (new)
**LOC**: ~150

---

## Phase 3: Galaxy Map Enhancements

### 3.1 Facility Status Icons — P0
Render 33 facility indicator sprites (STRATEGY.DLL IDs 9000-9999) on the galaxy map next to systems that have facilities. Mine, Shipyard, Advanced Shipyard, Refinery — 4 states each (active, Alliance building, Empire building, enroute).

**Files**: `rebellion-render/src/lib.rs` (galaxy map draw)
**Assets**: 33 facility icons (67x35 px)
**LOC**: ~100

### 3.2 Sector Boundaries — P1
Translucent polygon outlines for each sector region (convex hull of member systems + padding).

**Files**: `rebellion-render/src/lib.rs`
**LOC**: ~120

### 3.3 System Context Menu — P0
Right-click a system to see: fleet garrison, facility list, popularity bars, available actions (build, move fleet here, send diplomat, etc.). Currently only left-click shows a minimal text panel.

**Files**: `rebellion-render/src/lib.rs` (expand draw_system_info_panel)
**LOC**: ~200

### 3.4 Fleet Context Menu — P0
Right-click a fleet diamond on the map: show fleet composition, commander, movement order, available actions (move, split, merge, assign character).

**Files**: `rebellion-render/src/fleet_movement.rs` (expand)
**LOC**: ~150

### 3.5 Blockade Visual Indicators — P1
Show blockade icon on systems where enemy fleet blocks manufacturing. Currently only logged.

**Files**: `rebellion-render/src/lib.rs`
**LOC**: ~40

---

## Phase 4: Player Control Panels (Missing UI for Existing Systems)

### 4.1 Research Panel — P0
Assign characters to Ship/Troop/Facility research trees. Show progress bars, current level, next unlock, character skill match. Simulation exists (v0.13.0+), UI missing.

**Files**: `rebellion-render/src/panels/research.rs` (new)
**PanelActions**: `DispatchResearch { character, tech_type }`, `CancelResearch { tech_type }`
**LOC**: ~250

### 4.2 Jedi Training Panel — P0
Show Force-aware characters, training progress, detection risk per tier. Start/stop training. Simulation exists, zero UI.

**Files**: `rebellion-render/src/panels/jedi.rs` (new)
**PanelActions**: `StartTraining(character)`, `StopTraining(character)`
**LOC**: ~200

### 4.3 Fleet Composition Editor — P0
Split fleet (select ships to new fleet), merge fleets (same location), assign character as commander/admiral/general. Currently fleets panel is read-only.

**Files**: `rebellion-render/src/panels/fleets.rs` (expand)
**PanelActions**: `SplitFleet { fleet, ships }`, `MergeFleets { fleet_a, fleet_b }`, `AssignCharacter { character, fleet }`
**LOC**: ~350

### 4.4 Bombardment Targeting — P1
Select fleet, select target system, forecast damage (facilities destroyed, popularity shift), confirm fire. Currently auto-triggers.

**Files**: `rebellion-render/src/panels/bombardment.rs` (new)
**PanelActions**: `OrderBombardment { fleet, system }`
**LOC**: ~150

### 4.5 Death Star Control Panel — P1
Empire only: construction progress tracker, fire order interface, target selection. Alliance: interception planner.

**Files**: `rebellion-render/src/panels/death_star.rs` (new)
**PanelActions**: `FireDeathStar { system }`, `MoveDeathStar { system }`
**LOC**: ~200

### 4.6 Loyalty & Uprising Dashboard — P1
Per-system loyalty view, uprising risk forecast, betrayal risk per character. Sorted by danger level.

**Files**: `rebellion-render/src/panels/loyalty.rs` (new)
**LOC**: ~200

### 4.7 Expanded System Detail Panel — P0
Replace text-only counts with: fleet list by name, facility list with types, popularity bars (blue/red), ground unit roster, defense breakdown. Character portraits from GOKRES.DLL.

**Files**: `rebellion-render/src/lib.rs` (expand draw_system_info_panel)
**Assets**: 61 character portraits (80x80), 57 ship status views (122x50)
**LOC**: ~200

### 4.8 Character Detail View — P0
Full character stats: backstory (TEXTSTRA), current location, mission, fleet assignment, Force tier progression bar, combat rating, portrait.

**Files**: `rebellion-render/src/panels/officers.rs` (expand)
**Assets**: 61 character portraits
**LOC**: ~120

---

## Phase 5: Tactical Combat View

### 5.1 Combat Arena Canvas — P0
New macroquad render module: 2D tactical battlefield with ship placement zones. Ships rendered using TACTICAL.DLL sprites (130 ship sprites, 102x107px). Replaces auto-resolve when player's fleet is involved.

**Files**: `rebellion-render/src/tactical_view.rs` (new)
**Assets**: 130 tactical ship sprites from TACTICAL.DLL
**LOC**: ~400

### 5.2 Ship Placement Phase — P0
Pre-combat: drag ships to deployment zone positions. AI auto-places using formation templates. Countdown to combat start.

**LOC**: ~200

### 5.3 Real-Time Phased Combat — P0
Tick-by-tick combat using existing 7-phase pipeline. Weapon fire animations (muzzle flash), shield flicker, hull breach effects. Add `CombatSystem::step()` for per-tick resolution.

**Files**: `rebellion-core/src/combat.rs` (add step()), `tactical_view.rs`
**LOC**: ~500

### 5.4 Combat HUD — P0
Task force panels (TACTICAL.DLL 1000-1037), squadron buttons (4 colors × 3 states), weapon recharge gauge (5 stages), hull/shield display panel.

**Assets**: 29 HUD elements + 33 buttons + 5 gauges + 8 Death Star controls
**LOC**: ~300

### 5.5 Ship Targeting & Orders — P0
Click enemy ship to focus fire. Targeting lines. Retreat button (ships charge hyperdrive for 3 ticks). Fighter launch/recall from carriers.

**LOC**: ~250

### 5.6 Ground Combat Interface — P1
Troop deployment from fleet to system. Ground combat resolution (decode from Ghidra RE). Casualty display.

**Files**: `rebellion-core/src/combat.rs` (ground resolve), `rebellion-render/src/ground_combat.rs` (new)
**LOC**: ~400

### 5.7 Combat Results Modal — P0
Detailed battle report: per-ship damage, fighter losses, ground casualties. Event screen art from STRATEGY.DLL (181 event result screens, 400x200).

**Assets**: 181 battle outcome screens
**LOC**: ~150

### 5.8 GameMode State Machine — P0
`GameMode` enum (`MainMenu`, `GameSetup`, `Galaxy`, `TacticalCombat`, `GroundCombat`). Single state machine in main.rs controls which view renders.

**Files**: `rebellion-app/src/main.rs`
**LOC**: ~150

---

## Phase 6: Audio Integration

### 6.1 Soundtrack Playback — P1
Load 16 WAV tracks, play context-aware background music. Alliance themes during Alliance actions, Imperial March during Empire. quad-snd already supports this.

**Files**: `rebellion-app/src/audio.rs`
**LOC**: ~80

### 6.2 Voice Line Playback — P1
Play 285 WAV voice clips triggered by game events. Droid advisor voices during briefings, combat callouts, system control announcements.

**Files**: `rebellion-app/src/audio.rs`, `rebellion-render/src/advisor.rs`
**LOC**: ~120

### 6.3 UI Sound Effects — P1
Button clicks, fleet departure/arrival, build complete, mission success/fail, combat sounds. Most already wired via `SfxKind` enum, just need WAV loading.

**Files**: `rebellion-app/src/audio.rs`
**LOC**: ~60

### 6.4 WASM WebAudio — P1
Remove cfg(not(wasm32)) gate on audio init. quad-snd supports WebAudio. Serve sound files via HTTP alongside DAT files.

**LOC**: ~40

---

## Phase 7: Encyclopedia & Data Visualization

### 7.1 Entity Portraits in Panels — P0
Load 580 GOKRES.DLL sprites into texture cache. Render character portraits (80x80) in Officers panel, ship hull views (122x50) in Fleets panel, troop/fighter icons in system detail.

**Files**: `rebellion-render/src/encyclopedia.rs` (expand texture loading)
**Assets**: 580 entity sprites
**LOC**: ~150

### 7.2 Event Narrative Screens — P1
Display 181 event result screens (400x200) from STRATEGY.DLL when missions complete, events fire, or battles resolve. Currently text-only in message log.

**Assets**: 181 event screens + 73 character panels + 61 dialogue scenes
**LOC**: ~120

### 7.3 Research Tech Tree Visualizer — P2
Graph showing Ship/Troop/Facility tree prerequisites, current level, next unlock. Visual alternative to the research panel.

**LOC**: ~250

---

## Phase 8: Save/Load & Polish

### 8.1 WASM Save via localStorage — P1
Browser save/load using base64-encoded bincode in localStorage.

**LOC**: ~120

### 8.2 In-Game Help Overlay — P2
Dismissible panel with keyboard shortcuts, panel descriptions, gameplay basics. H key toggle.

**LOC**: ~100

### 8.3 HD Asset Upscaling — P2
Bulk upscale 450 curated BMPs using Vertex AI Imagen 4.0 (faithful) for icons and Gemini (generative) for backgrounds. Improves visual quality from 1998 640x480 to modern resolutions.

**LOC**: ~30 (script), **Cost**: ~$50-100 API

---

## Summary

| Phase | Tasks | P0 | LOC | Focus |
|-------|-------|-----|-----|-------|
| 1. Main Menu & Setup | 4 | 3 | ~730 | Pre-game flow |
| 2. Cockpit Interface | 4 | 2 | ~1,000 | Game chrome |
| 3. Galaxy Map | 5 | 3 | ~610 | Map interaction |
| 4. Player Control Panels | 8 | 4 | ~1,670 | Missing UI for existing systems |
| 5. Tactical Combat | 8 | 6 | ~2,350 | Combat view |
| 6. Audio | 4 | 0 | ~300 | Sound integration |
| 7. Encyclopedia & Visuals | 3 | 1 | ~520 | Asset rendering |
| 8. Save/Load & Polish | 3 | 0 | ~250 | Quality of life |
| **Total** | **39** | **19** | **~7,430** | **Full game rebuild** |

## Execution Order

1. **Phase 1** (main menu) + **Phase 2.4** (theme) → first impression
2. **Phase 4.1-4.2** (research + jedi panels) → player control of existing systems
3. **Phase 3.3-3.4** (context menus) + **Phase 4.7-4.8** (expanded panels) → information density
4. **Phase 2.1-2.2** (cockpit + buttons) → game chrome
5. **Phase 5.1-5.5, 5.7-5.8** (tactical combat) → the big feature
6. **Phase 4.3** (fleet editor) + **Phase 3.1** (facility icons) → strategic depth
7. **Phase 6** (audio) + **Phase 7** (visuals) → polish
8. **Phase 5.6** (ground combat) + **Phase 4.4-4.6** (bombardment, death star, loyalty) → completeness

## Parity Targets

| Component | Current | After Phase 2 | After Phase 4 | After Phase 5 | After All |
|-----------|---------|--------------|--------------|--------------|-----------|
| Core Simulation | 95% | 95% | 95% | 95% | 95% |
| UI / Game Chrome | 30% | 55% | 75% | 85% | 95% |
| Space Combat | 60% | 60% | 60% | 90% | 90% |
| Ground Combat | 20% | 20% | 20% | 50% | 70% |
| Audio | 10% | 10% | 10% | 10% | 80% |
| **Overall** | **~50%** | **~55%** | **~65%** | **~75%** | **~90%** |

## Critical Files

| File | Changes |
|------|---------|
| `crates/rebellion-app/src/main.rs` | GameMode state machine, loading flow, audio wiring |
| `crates/rebellion-render/src/lib.rs` | Galaxy map enhancements, system detail expansion |
| `crates/rebellion-render/src/cockpit.rs` | NEW — cockpit frame + control buttons |
| `crates/rebellion-render/src/tactical_view.rs` | NEW — tactical combat arena |
| `crates/rebellion-render/src/theme.rs` | NEW — Star Wars egui theme |
| `crates/rebellion-render/src/main_menu.rs` | NEW — title screen + menu |
| `crates/rebellion-render/src/advisor.rs` | NEW — droid advisor animation |
| `crates/rebellion-render/src/panels/research.rs` | NEW — research dispatch |
| `crates/rebellion-render/src/panels/jedi.rs` | NEW — Jedi training |
| `crates/rebellion-render/src/panels/bombardment.rs` | NEW — bombardment targeting |
| `crates/rebellion-render/src/panels/death_star.rs` | NEW — Death Star controls |
| `crates/rebellion-render/src/panels/loyalty.rs` | NEW — loyalty/uprising dashboard |
| `crates/rebellion-render/src/panels/game_setup.rs` | NEW — new game configuration |
| `crates/rebellion-core/src/combat.rs` | Add step() for tick-by-tick combat |

## Verification

1. Launch game → main menu screen with title art renders
2. New Game → setup screen with difficulty/faction/galaxy size
3. Choose faction → cockpit interface with galaxy map inside frame
4. Click system → expanded detail panel with portraits and facility icons
5. Press R → research panel shows tech trees with character assignment
6. Fleet meets enemy → tactical combat view opens with ship sprites
7. Combat resolves → detailed results modal with event art
8. Background music plays throughout
9. All of the above works in both native macOS and browser (WASM)
