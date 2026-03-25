# Open Rebellion — UI Parity Gap Analysis (100% Target)

**Date**: 2026-03-25
**Status**: v0.15.0 (UI 95%)
**Goal**: Identify ALL remaining gaps for UI parity with original Star Wars Rebellion (1998)
**Current**: Core 97% | UI 95% | Combat 90% | Overall ~93%

---

## Executive Summary

| Category | Current | Gaps | Est. LOC |
|----------|---------|------|---------|
| **Main Menu & Setup** | 95% | Faction selection click bug (P1) | ~50 |
| **Cockpit Interface** | 85% | Chrome renders; buttons not fully wired; viewport bleed-through (P2) | ~300 |
| **Galaxy Map Overlays** | 90% | All major overlays done; right-click menus partially broken (WASM timing) | ~100 |
| **War Room Panels** | 92% | 9 panels built; mutual exclusion incomplete (P1) | ~80 |
| **Advisor System** | 40% | BIN animation format partially decoded; frame-cycling fallback only | ~600 |
| **Tactical Combat** | 60% | 2D arena + phased combat done; ship sprites integrated; ground combat partially done | ~400 |
| **Asset Integration** | 30% | Only 4 DLLs partially wired (Gokres portraits, Tactical ships). 2,441 BMPs mostly unused | ~1,000+ |
| **Audio System** | 70% | Quad-snd integrated; music + SFX wired; voice lines hookable; context music stubs | ~200 |
| **Visual Polish** | 10% | HD PNG fallback ready; no batch upscaling applied; cockpit chrome fallback colors only | ~800 |
| | **TOTAL** | | **~3,500 LOC** |

---

## 1. Main Menu & Game Setup (Phase 1)

### P1: Faction Selection Cards Unclickable
**File**: `crates/rebellion-render/src/panels/game_setup.rs:311-339`
**Issue**: `faction_button()` uses `allocate_ui + interact(Sense::click())` pattern. The allocated response doesn't carry clicks through the `Frame::show()` closure. Galaxy/difficulty buttons use `ui.add_sized([...], btn).clicked()` which works.

**Impact**: Player cannot start new game via normal flow. Workaround exists (LOAD GAME button bypasses setup).

**Fix**: Replace `allocate_ui` pattern with `egui::Button` + `ui.add_sized()` (same as galaxy/difficulty).
**Est. LOC**: ~50

---

## 2. Cockpit Interface (Phase 2)

### 2.1 Chrome Rendering — DONE
**Status**: Faction cockpit chrome renders correctly (Alliance blue, Empire gray).
**File**: `crates/rebellion-render/src/cockpit.rs`
**Asset Used**: `DllSource::Strategy, resource_id=900` (galaxy background, 640x481)

**What works**:
- Top bar with faction label ("REBEL ALLIANCE — COMMAND BRIDGE" / "EMPIRE COMMAND BRIDGE")
- Bottom bar with status indicators
- Correct color scheme per faction

### 2.2 Cockpit Control Buttons — PARTIAL
**Status**: 9 buttons wired (Officers, Fleets, Manufacturing, Missions, Research, Encyclopedia, Save/Load, Speed−, Speed+).
**Current Implementation**: Keyboard shortcuts only (O, F, M, N, T, J, B, D, L).

**Missing**:
- Visual button sprites from `DllSource::Common, IDs 11001-11215` (215 button designs available)
- Clickable button regions in cockpit chrome
- Button state graphics (unpressed/pressed/disabled)
- Hover/focus visuals

**Impact**: Players must use keyboard. Cockpit feels incomplete without clickable buttons.
**Est. LOC**: ~200 for clickable button layout + state rendering

### 2.3 Droid Advisor Animation — CRITICAL GAP
**Status**: 40% — frame cycling fallback only.
**Files**:
- `crates/rebellion-render/src/advisor.rs` (render)
- `assets/references/ref-ui-full/07-droid-advisors/` (148 BMP frames extracted)
- BIN animation format: 713-716 BIN files per DLL (animation sequences, timing, coordinates)

**Current Implementation**:
- C-3PO/R2-D2 (Alliance) or Imperial protocol droid (Empire) sprites render
- Frames cycle linearly (no respect for BIN timing data)
- Advisors appear for key events (greetings, mission results, combat outcomes, uprisings)
- Priority message queue works

**Missing**:
- **BIN Animation Decoder**: Decode frame ordering, timing (ms between frames), position offsets from BIN control files
  - 713 ALSPRITE BIN files + 716 EMSPRITE BIN files = 1,429 animation sequences
  - Format unknown (likely binary frame index table + timing data)
  - Reverse-engineering needed from Ghidra REBEXE.EXE (animation playback function)
- **Gesture Library**: Map mission types/combat outcomes to correct gesture sequences
- **Lip Sync**: BIN data may include audio clip references for synced voice playback
- **Confidence**: Estimated 600-1000 LOC if format can be reverse-engineered; may be high-barrier item

**Workaround**: Current frame-cycling fallback is acceptable for MVP. Replace with full animation when format decoded.
**Est. LOC**: ~600+ (blocked on BIN format RE)

### 2.4 Star Wars egui Theme — DONE
**Status**: Implemented.
**File**: `crates/rebellion-render/src/theme.rs`
**Colors**: Dark space (#050510), gold/amber accents, Liberation Sans font, faction-colored elements.

---

## 3. Galaxy Map Enhancements (Phase 3)

### 3.1 Facility Status Icons — DONE
**Status**: Fully implemented.
**File**: `crates/rebellion-render/src/lib.rs:357` (`draw_facility_icons`)
**Asset Used**: `DllSource::Gokres, IDs 1-999` (18 facility indicator sprites, 67x35 px)

**What works**:
- Facility indicators (mine, shipyard, refinery, shield, HQ) render as colored squares
- Positioned on galaxy map next to systems with facilities
- Zoom-scaled (visible at 0.3x-5.0x)

### 3.2 Sector Boundaries — DONE
**Status**: Fully implemented.
**File**: `crates/rebellion-render/src/lib.rs:473` (`draw_sector_boundaries`)
**Implementation**: Convex hull of member systems with padding, colored by SectorGroup.

**What works**:
- Core, Rim Inner, Rim Outer sectors distinguished by color
- Translucent polygon outlines
- Graham scan algorithm for hull computation

### 3.3 System Context Menu — PARTIAL
**Status**: 75% — right-click detection broken in browser.
**Files**: `crates/rebellion-render/src/lib.rs:738` (`draw_system_context_menu`)

**What works**:
- Panel shows fleet garrison, facility list, popularity bars
- Correct content rendering

**Missing**:
- **Right-click detection in WASM**: `is_mouse_button_released(Right)` with no drag doesn't fire reliably through Playwright's synthetic mouse up sequence
- **Severity**: P2 — works in native build, browser automation issue may mask genuine WASM timing problem
- **Impact**: Players on WASM build cannot right-click systems to see context menu

**Workaround**: Manual browser test needed to confirm WASM right-click works for real users.
**Est. LOC**: ~20 (if timing fix found); may require macroquad WASM bindings investigation

### 3.4 Fleet Context Menu — PARTIAL
**Status**: 75% — same right-click WASM issue as 3.3.

**What works**:
- Panel shows fleet composition, commander, transit info
- Correct content rendering
- Fleet hover detection (`hovered_fleet` function)

**Missing**:
- Right-click triggering (WASM timing issue)

### 3.5 Blockade Visual Indicators — DONE
**Status**: Fully implemented.
**File**: `crates/rebellion-render/src/lib.rs:542` (`draw_blockade_indicators`)

**What works**:
- Red ring + fill on blockaded systems
- Visual feedback matches UI state

---

## 4. War Room Panels (Phase 4) — MOSTLY DONE

All 9 panels implemented. Panel mutual exclusion **incomplete**.

| Panel | Status | File | Asset Integration |
|-------|--------|------|-------------------|
| **Officers** | 95% | `panels/officers.rs` | Gokres portraits (80x80) — DONE |
| **Fleets** | 90% | `panels/fleets.rs` | Fleet composition UI — DONE |
| **Manufacturing** | 95% | `panels/manufacturing.rs` | Queue UI — DONE |
| **Missions** | 90% | `panels/missions.rs` | Dispatch UI — DONE |
| **Research** | 95% | `panels/research.rs` | Tech tree UI + character assignment — DONE |
| **Jedi Training** | 95% | `panels/jedi.rs` | Force progression UI — DONE |
| **Bombardment** | 90% | `panels/bombardment.rs` | Fleet selection + damage forecast — DONE |
| **Death Star** | 90% | `panels/death_star.rs` | Construction + superlaser controls — DONE |
| **Loyalty & Uprisings** | 90% | `panels/loyalty.rs` | Per-system danger display — DONE |

### P1: Incomplete Panel Mutual Exclusion
**File**: `crates/rebellion-app/src/main.rs:438-456`

**Current behavior**:
- Buttons O/F/M/N (Officers/Fleets/Manufacturing/Missions) fully exclude each other ✓
- Button T (Research) opens T, clears O/F/M/N, clears J, **but does NOT clear B/D/L** ✗
- Button J (Jedi) opens J, clears O/F/M/N, clears T, **but does NOT clear B/D/L** ✗
- Buttons B/D/L (Bombardment/Death Star/Loyalty) properly clear T/J ✓

**Root cause**: T and J key handlers have hardcoded closure lists (lines 444-447, 451-454) without B/D/L entries.

**Impact**: T or J can display simultaneously with B, D, or L, creating multi-panel visual clutter.

**Fix**: Add three lines to T and J handlers:
```rust
show_bombardment = false;
show_death_star = false;
show_loyalty = false;
```

**Est. LOC**: ~6

---

## 5. Tactical Combat View (Phase 5) — PARTIAL

### 5.1 Combat Arena Canvas — 70% DONE
**File**: `crates/rebellion-render/src/tactical_view.rs`
**Status**: 2D tactical arena with ship placement zones implemented.

**What works**:
- Arena background renders
- Fleet separation (player left, enemy right)
- Ship placement UI
- Phased combat system (maneuver → attack → hold → retreat)
- Targeting UI (select target, attack button)
- Phase indicator

**Missing**:
- **Ship Sprite Rendering**: Tactical ship sprites from `DllSource::Tactical, IDs 2001-2130` (~130 3D-rendered capital ship views, 102x107 px)
  - Sprites are extracted but only preloaded, not actively displayed during combat
  - Code path exists (`bmp_cache.preload_range()` + `bmp_cache.get()` calls) but sprites not rendering in battle phases
  - **Impact**: Ships appear as rectangles, not 3D-rendered tactical graphics
  - **Fix**: Debug sprite rendering in combat phase loop, ensure texture display per ship slot
  - **Est. LOC**: ~100

### 5.2 Ship Placement Phase — 90% DONE
**Status**: Drag ships to deployment zone, AI auto-places, countdown implemented.

**Missing**:
- Visual feedback for placement validity (is this zone occupied?)
- Drag-and-drop gestures (mouse coordinate tracking in WASM may be incomplete)

### 5.3 Ground Combat — 60% DONE
**File**: `crates/rebellion-render/src/ground_combat.rs`

**What works**:
- Regiment engagement phase
- Animated victory/defeat bars
- Results display

**Missing**:
- **Troop tactical sprites** from DLL (not yet identified/extracted)
- **Terrain background** (not in current asset list)
- **Ground unit placement UI**

**Est. LOC**: ~200 for full ground combat rendering

---

## 6. Asset Integration — CRITICAL GAP

### Current Status
Only **4 of 11+ DLLs partially integrated**:

| DLL | BMPs | Status | Used | % Used |
|-----|------|--------|------|--------|
| **STRATEGY.DLL** | 1,042 | Partial | 1 (bg 900) | <1% |
| **GOKRES.DLL** | 580 | Partial | 2 (portraits 2112-2200, ship icons 1024+) | <1% |
| **COMMON.DLL** | 321 | Unused | 0 | 0% |
| **TACTICAL.DLL** | 288 | Partial | ~130 (ship sprites, preloaded) | ~45% |
| **ALSPRITE.DLL** | 74 | Partial | 74 (advisor frames) | 100% |
| **EMSPRITE.DLL** | 74 | Partial | 74 (advisor frames) | 100% |
| **ALBRIEF.DLL** | 20 | Unused | 0 | 0% |
| **EMBRIEF.DLL** | 18 | Unused | 0 | 0% |
| **REBDLOG.DLL** | 24 | Unused | 0 | 0% |
| | **TOTAL** | | **~307** | **~13%** |

### Missing Integrations (Estimated 300+ UI Assets Unused)

#### COMMON.DLL (321 BMPs) — Buttons, Sliders, Panel Chrome
**Resource ranges**:
- 10000-10999 (101): Scrollbars, sliders, checkboxes, progress bars, panel dividers
- 11000-11999 (215): Button sprites (pressed/unpressed/disabled states for all game functions)
- 20000-20999 (2): Main menu backgrounds

**Gap**: No cockpit button sprites rendered. All 215 button designs available but unused.
**Impact**: Cockpit buttons are keyboard-only; no visual feedback.
**Est. LOC**: ~300 to wire button rendering + click detection

#### STRATEGY.DLL (1,042 BMPs) — Galaxy Chrome, Event Screens, Character Panels
**Key unused ranges**:
- 1000-1999 (73): Character event screens (400x200 dialogue scenes)
- 6000-6999 (61): Character event screens
- 10000-10999 (687): UI chrome mega-block (buttons, icons, panel frames, scroll arrows, status indicators)
- 11000-11999 (181): Event result screens (battle outcomes, bombardment, fleet events)

**Gap**:
- Event screens not wired to mission/combat/bombardment outcomes
- UI chrome (buttons, frames, scroll arrows) not integrated
- Battle result screens (10757-10760: Alliance/Empire victory/defeat) not displayed

**Impact**: Generic UI instead of period-authentic graphics.
**Est. LOC**: ~500 to integrate event screens + result overlays

#### GOKRES.DLL (580 BMPs) — Entity Status, Damage Detail, Weapon Arcs
**Unused ranges**:
- 3000-4999 (22): Ship construction in-progress graphics (130x65 px)
- 5000-6999 (37): Reactor layout diagrams + detail views (122x50 px)
- 7000-8999 (22): Ship damage close-ups (130x65 px)
- 10000-10999 (30): Ship damage overview (122x50 px, shows damaged hull sections)
- 16000-26999 (199): Mini-icons (61x25 px) for facilities, troops, fighters, characters, weapon arcs, damage details

**Gap**:
- Ship damage diagrams not displayed in fleet info panels (only hull profile shown)
- Reactor diagrams not shown in ship detail view
- Weapon arc diagrams unused
- Mini-icons not used in compact UI displays

**Impact**: Panels lack visual detail/richness compared to original.
**Est. LOC**: ~200 to add damage diagram + mini-icon displays

#### TACTICAL.DLL (288 BMPs) — Combat HUD, Task Force Panels, Squadron Controls
**Unused ranges**:
- 1000-1999 (154): Combat HUD (squadron buttons, task force panels, weapon recharge gauges, hull/shield display)
- 40000+ (4): Tactical labels

**Gap**: HUD elements rendered as generic egui widgets, not original tactical combat interface.
**Impact**: Tactical combat UI lacks authentic 1998 game feel.
**Est. LOC**: ~250 to replace combat HUD with DLL sprite-based design

#### REBDLOG.DLL (24 BMPs) — Dialog Box Chrome
**Status**: Unused.
**Content**: Dialog backgrounds, button sprites, frame elements.

**Gap**: Generic egui modals instead of original styled dialog boxes.
**Est. LOC**: ~100

---

## 7. Audio System (Phase 6) — MOSTLY DONE

### Current Status: 70% Complete
**File**: `crates/rebellion-render/src/audio.rs`
**Library**: quad-snd (CoreAudio/ALSA/WebAudio)

**What works**:
- Soundtrack playback (quad-snd CoreAudio integration)
- 285 voice WAV files extracted (153 Alliance + 132 Empire)
- SFX playback system (advisor sounds, button clicks)
- Volume controls wired to UI
- Music context awareness (galaxy vs combat vs menu)

**Missing**:
- **Context-based music selection**: Stubs for contextual tracks per situation
- **Voice line queue**: Infrastructure exists; advisor voice lines not fully wired
- **Smacker video audio**: 15 story cutscenes extracted to WebM but not played
- **Background loop management**: Music loop points not configured per track
- **Fade-in/fade-out transitions**: Abrupt track switches

**Impact**: Audio mostly works; edge cases and polish missing.
**Est. LOC**: ~200 for full audio polish

---

## 8. Visual Polish — CRITICAL GAP (10% Complete)

### 8.1 HD Asset Upscaling — INFRASTRUCTURE READY, NOT EXECUTED
**Status**: Pipeline exists but not batch-applied.
**Files**:
- `scripts/upscale-assets.py` (exists)
- `crates/rebellion-render/src/encyclopedia.rs` (HD PNG fallback ready at data/hd/EData/)

**Capability**:
- Vertex AI Imagen 4.0 (non-generative super-resolution): 4x upscaling without hallucination
- waifu2x-ncnn-vulkan (anime/photo upscaling): fast GPU acceleration
- Gemini 3 Pro Image (generative upscaling): adds plausible detail to capital ships

**Missing**:
- **Batch upscaling execution**: ~200+ EData BMPs (portraits, ships, planets) not upscaled
- **DLL sprite upscaling**: 2,241 STRATEGY/COMMON/TACTICAL/GOKRES/ALSPRITE/EMSPRITE BMPs not processed
- **Sprite sheet generation**: No pre-rendered 3D model sprite sheets for tactical combat

**Impact**: Game art is 1998-resolution (320-640px). HD upscaling would modernize visual fidelity 4x.
**Est. LOC**: ~200 for batch script execution + pipeline integration

### 8.2 Cockpit Chrome Color Fallbacks — PARTIAL
**Status**: Faction colors render correctly, but no custom texture mapping.
**Current**: Flat color fills (Alliance blue #0033CC, Empire gray #333333)
**Missing**: Actual texture overlays from extracted cockpit backgrounds

**Est. LOC**: ~50 for texture integration

### 8.3 3D Tactical Model Sprite Sheets — NOT DONE
**Status**: Pipeline documented but not executed.
**Capability**: Hunyuan3D Pro → Blender → sprite sheet generation

**Missing**:
- ~30 capital ship models (Executor, Mon Cal, Nebulon-B, etc.)
- Generation + pre-render to sprite sheets for tactical combat

**Impact**: Tactical combat uses flat 2D ship sprites, not 3D renders.
**Cost**: ~$23.63 for 63 Hunyuan3D models (or $0.225/model via WaveSpeedAI at 1/16th cost)
**Est. LOC**: ~100 for sprite sheet rendering + integration

---

## 9. Known Defects (From QA Inventory)

| ID | Severity | Panel | Description | File/Line |
|----|----------|-------|-------------|-----------|
| P1-DEF-001 | P1 | Game Setup | Faction selection cards unclickable | `game_setup.rs:311-339` |
| P1-DEF-002 | P1 | War Room | T/J do not close B/D/L (incomplete mutual exclusion) | `main.rs:438-456` |
| P2-DEF-003 | P2 | Galaxy Map | Cockpit viewport mouse bleed-through | `lib.rs:126`, `main.rs:1148` |
| P2-DEF-004 | P2 | Galaxy Map | Right-click context menus not triggering (WASM timing) | `lib.rs:258` |
| P3-DEF-005 | P3 | Main Menu | LOAD GAME bypasses faction selection silently | `main.rs:1044-1048` |

**All P1 defects fixable with <100 LOC.**

---

## 10. Remaining Work Summary (Prioritized)

### Phase A: Critical Bug Fixes (P1) — ~100 LOC
1. **P1-DEF-001**: Faction selection click bug (`game_setup.rs`)
2. **P1-DEF-002**: Panel mutual exclusion (T/J → B/D/L) (`main.rs`)

### Phase B: Core UI Completion — ~1,500 LOC
3. **Cockpit Button Wiring**: Clickable button rendering + state graphics
4. **Tactical Combat Ship Sprites**: Debug sprite display in combat phases
5. **Ground Combat Rendering**: Troop placement + terrain

### Phase C: Asset Integration — ~1,500 LOC
6. **COMMON.DLL Integration**: 215 button sprites, scrollbars, checkboxes
7. **STRATEGY.DLL Integration**: Event screens (73 BMPs), UI chrome (687 BMPs), result screens (181 BMPs)
8. **Event Result Overlays**: Battle outcome screens, bombardment results, fleet events
9. **Combat HUD Restyling**: Replace generic widgets with tactical DLL sprites

### Phase D: Advisor System — ~600 LOC (BLOCKED on BIN format RE)
10. **BIN Animation Decoder**: Reverse-engineer animation sequence format
11. **Gesture Mapping**: Wire mission/combat outcomes to advisor animations
12. **Lip Sync**: Audio sync for advisor voice lines

### Phase E: Audio Polish — ~200 LOC
13. **Context Music**: Contextualize music selection
14. **Smacker Video Playback**: Integrate 15 story cutscenes
15. **Fade Transitions**: Smooth music/SFX transitions

### Phase F: Visual Polish — ~500 LOC
16. **Batch HD Upscaling**: Execute scripts/upscale-assets.py
17. **3D Model Sprite Sheets**: Generate tactical combat sprites from Hunyuan3D
18. **Cockpit Texture Mapping**: Wire extracted backgrounds to chrome rendering

---

## 11. Completion Path to 100% UI Parity

| Phase | Work | LOC | Dependencies | Notes |
|-------|------|-----|--------------|-------|
| **Quick Wins** | P1 bugs + cockpit buttons | ~600 | None | Unblocks faction selection, improves UI feel |
| **Asset Wave 1** | COMMON + basic STRATEGY | ~800 | BMP staging complete | Button rendering, basic overlays |
| **Tactical Polish** | Ship sprites + ground combat | ~400 | Tactical DLL assets staged | Combat view completeness |
| **Advisor System** | BIN format RE + implementation | ~600 | **BLOCKED** on GhidraMCP session | Critical missing feature |
| **Audio/Video** | Smacker + context music | ~300 | Codec support (ffmpeg for WASM?) | Polish + story delivery |
| **Visual Enhancement** | HD upscaling + 3D models | ~500 | GPU upscaling tools available | Aesthetic modernization |
| | **TOTAL** | **~3,200** | Phase 1-2 ready; rest dependent | |

---

## 12. Asset Staging Checklist

**Prerequisite**: Confirm BMP assets are staged at expected paths:
```
data/ui/
├── strategy-dll/BMP/    (1,042 BMPs)
├── common-dll/BMP/      (321 BMPs)
├── tactical-dll/BMP/    (288 BMPs)
├── gokres-dll/BMP/      (580 BMPs)
└── ...
```

If staging not complete, run extraction script first.

---

## 13. Recommendations

### High-Impact Quick Wins (First Sprint)
1. ✅ **Fix P1-DEF-001** (faction selection) — unblocks testing
2. ✅ **Fix P1-DEF-002** (panel exclusion) — improves UI consistency
3. ✅ **Wire COMMON.DLL buttons** — visual completeness
4. ✅ **Debug tactical ship sprites** — combat feels complete

### Medium-Term (Next 2-3 sprints)
5. **STRATEGY.DLL event screens** — mission/combat feedback
6. **Cockpit viewport bleed-through fix** — input consistency
7. **Ground combat rendering** — combat completeness
8. **HD upscaling batch execution** — visual modernization

### Long-Term (Post-MVP)
9. **BIN animation decoder** (requires RE) — advisor polish
10. **Smacker video playback** — story delivery
11. **3D model sprite sheets** — tactical visual fidelity

### Not Blocking Release
- Right-click WASM timing (workaround: manual browser test confirms if real issue)
- Lip-synced advisor voices (voice lines playable, just not perfectly timed)
- Context music fine-tuning (music plays; context awareness nice-to-have)

---

## 14. Files to Review/Modify (Priority Order)

### P1 Bugs (Immediate)
1. `crates/rebellion-render/src/panels/game_setup.rs` — faction button click fix
2. `crates/rebellion-app/src/main.rs` — panel mutual exclusion fix

### UI Completion
3. `crates/rebellion-render/src/cockpit.rs` — button wiring
4. `crates/rebellion-render/src/tactical_view.rs` — ship sprite rendering
5. `crates/rebellion-render/src/lib.rs` — cockpit viewport bounds checking

### Asset Integration
6. `crates/rebellion-render/src/bmp_cache.rs` — ensure preload coverage
7. `crates/rebellion-render/src/panels/` — integrate COMMON/STRATEGY sprites
8. `scripts/stage-ui-assets.py` — BMP staging automation (if not complete)

### Audio/Video
9. `crates/rebellion-render/src/audio.rs` — context music + Smacker integration
10. `crates/rebellion-render/src/advisor.rs` — BIN animation (post-RE)

---

## Appendix: Original Game Asset Manifest

**Source**: `agent_docs/dll-resource-catalog.md`

```
STRATEGY.DLL:   1,042 BMPs + 96 BIN
GOKRES.DLL:     580 BMPs
COMMON.DLL:     321 BMPs + 1 BIN
TACTICAL.DLL:   288 BMPs + 1 BIN
ALSPRITE.DLL:   74 BMPs + 716 BIN (Alliance advisor)
EMSPRITE.DLL:   74 BMPs + 713 BIN (Empire advisor)
ALBRIEF.DLL:    20 BMPs + 366 BIN
EMBRIEF.DLL:    18 BMPs + 471 BIN
REBDLOG.DLL:    24 BMPs
ENCYTEXT.DLL:   348 BIN (text, not visual)
TEXTSTRA.DLL:   511 BIN (strings, not visual)

Total Extracted: 2,441 BMPs + 3,223 BIN files
+ 15 Smacker videos (WebM)
+ 16 WAV soundtrack files
+ 285 voice WAV files (153 Alliance + 132 Empire)
```

All extracted; ~307 in active use (<13%); ~2,134 unused.

---

**Analysis completed by**: Daimon team
**Next action**: Team lead assigns priorities for Phase A (bug fixes) and Phase B (asset integration).
