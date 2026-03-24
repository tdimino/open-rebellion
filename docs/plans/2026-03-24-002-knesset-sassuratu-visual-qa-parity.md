---
title: "Knesset Sassuratu — Visual QA + UI Parity Completion"
type: feat
status: active
date: 2026-03-24
project: open-rebellion
origin: docs/plans/2026-03-24-001-knesset-demiurge-ui-parity.md
tags: [knesset, swarm, qa, visual-testing, parity, panels, cockpit]
---

# Knesset Sassuratu — Visual QA + UI Parity Completion

*ששורתו (Sassuratu) — "the midwife" in Ugaritic. She who draws the newborn into the light and verifies it is whole. Named for the midwives of the divine assembly who attend the birth of gods in the Ba'al Cycle (KTU 1.17). As midwife, Sassuratu inspects every limb of the delivered work—each panel, each button, each pixel—and calls out what is malformed before the child is presented.*

## Purpose

Two objectives for the Open Rebellion UI after the Knesset Demiurge build sprint:

1. **Visual QA**: Launch the WASM build in a browser via agent-browser, navigate every UI panel systematically, screenshot and verify that each renders correctly and is functionally interactive.

2. **UI Parity Completion**: Fix every broken panel, missing wiring, visual bug, or incomplete feature found during QA. Close remaining non-combat UI gaps from the roadmap.

## Architecture Decisions

1. **QA-first, fix-second**: No code changes until QA produces a complete defect inventory. Prevents chasing phantoms.
2. **WASM is the test target**: All visual QA runs against the browser build. If WASM works, native works (egui renders identically). WASM also exercises the HTTP fetch data path.
3. **Automated screenshots as evidence**: Every test scenario produces a timestamped screenshot in `docs/qa/screenshots/`. Defects reference screenshots by filename.
4. **Fix in smallest scope**: Each bug fix is a single commit touching minimal files. No refactoring during fix phase.
5. **Re-verify after fix**: Every fix is re-tested by the verification dabora before the Knesset closes.

---

## Known Bugs (Pre-QA)

These are already identified from code review. QA will confirm + discover more.

### Bug 1: Incomplete Panel Mutual Exclusion

**Location**: `crates/rebellion-app/src/main.rs:405-496`

The keyboard panel toggles have two tiers of mutual exclusion that don't interlock:

- **Tier 1** (O/F/M/N): Officers, Fleets, Manufacturing, Missions — these clear each other but NOT Tier 2 or Tier 3.
- **Tier 2** (T/J): Research, Jedi — these clear Tier 1 and each other, but NOT Tier 3.
- **Tier 3** (B/D/L): Bombardment, Death Star, Loyalty — these clear everything (correct).

**Symptom**: Press B to open Bombardment, then press O. Officers panel opens but Bombardment stays open. Both panels render simultaneously.

**Fix**: All 9 panel toggles (O/F/M/N/T/J/B/D/L) must clear all 8 others when opening. Extract into a `close_all_panels()` helper.

### Bug 2: Cockpit Viewport Mouse Bleed-Through

**Location**: `crates/rebellion-render/src/cockpit.rs:210-212`

The cockpit comment says: "We can't scissor/clip macroquad draw calls to the viewport without a render target, so we just draw it across the map area." This means the galaxy map renders behind cockpit bars, and mouse clicks in the cockpit chrome region still hit the galaxy map.

**Fix**: Clamp mouse input in `draw_galaxy_map` to the `CockpitViewport` rect. If `(mouse_x, mouse_y)` is outside `[vp.x..vp.x+vp.width, vp.y..vp.y+vp.height]`, treat as no-hover / no-click.

### Bug 3: Save/Load Panel Not Wired to Cockpit Button

**Status**: Needs verification. The cockpit has a `CockpitButton::SaveLoad` variant, but the Save/Load panel (`panels/save_load.rs`) may not be toggled by the cockpit button handler in main.rs.

---

## 5 Daborot

| # | Dabora | Role | Model | Start Condition |
|---|--------|------|-------|-----------------|
| 1 | **Bōreret** (בוררת "The Inspector") | Visual QA — screenshot and catalog every panel | sonnet | Immediately after WASM build succeeds |
| 2 | **Rophe** (רופא "The Healer") | Bug fixes — repair each defect from QA inventory | sonnet | After Bōreret completes defect inventory |
| 3 | **Yashar** (ישר "The Straightener") | Parity gaps — close remaining non-combat UI holes | sonnet | Parallel with Rophe (no file conflicts) |
| 4 | **Sopher** (סופר "The Scribe") | SEO/GEO optimization for Ghidra documentation site | sonnet | Immediately (parallel — touches only ghidra-site/) |
| 5 | **Mashbīr** (משביר "The Verifier") | Re-test — confirm every fix and parity item | sonnet | After Rophe + Yashar + Sopher complete |

### Dabora 1: Bōreret (בוררת) — The Inspector

*Etymology: from B-R-R, "to purify, select, examine" — the one who separates wheat from chaff.*

**Role**: Systematic visual QA of every panel in the WASM build using agent-browser.

**Tools**: `agent-browser open`, `agent-browser snapshot`, `agent-browser screenshot`, `agent-browser click`, `agent-browser type`, Chrome CDP as fallback.

**Pre-flight**:
1. Build WASM: `bash /Users/tomdimino/Desktop/Programming/open-rebellion/scripts/build-wasm.sh`
2. Serve: `python3 -m http.server 8080 -d /Users/tomdimino/Desktop/Programming/open-rebellion/web/` (background)
3. Open: `agent-browser open http://localhost:8080`
4. Wait for loading screen to complete (DAT files fetch)
5. Create output dir: `mkdir -p /Users/tomdimino/Desktop/Programming/open-rebellion/docs/qa/screenshots`

**Test Protocol**: For each panel, execute:
1. **Navigate**: Click the cockpit button or press the keyboard shortcut
2. **Screenshot**: Capture with timestamp filename `{panel}-{timestamp}.png`
3. **Inspect**: Use `agent-browser snapshot` to read the DOM/canvas state
4. **Interact**: Click interactive elements (buttons, tabs, lists, sliders)
5. **Record**: Log result as PASS/FAIL with description in defect inventory

**Test Scenarios** (19 panels, 47 scenarios):

#### 1. Main Menu (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 1.1 | Launch game, observe title screen | Main menu renders with "New Game", "Load Game", "Quit" | — |
| 1.2 | Click "New Game" | Transitions to Game Setup screen | click |
| 1.3 | Click "Quit" | Application closes / returns to menu | click |

#### 2. Game Setup (4 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 2.1 | Observe setup screen | Galaxy size, difficulty, faction selection visible | — |
| 2.2 | Toggle galaxy size options | All 3 sizes selectable (Standard/Large/Huge) | click |
| 2.3 | Toggle difficulty | All 3 difficulties selectable | click |
| 2.4 | Select faction + Start | Transitions to Galaxy mode with correct faction | click |

#### 3. Galaxy Map (5 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 3.1 | Observe galaxy map | Stars render with names, cockpit chrome visible | — |
| 3.2 | Scroll to zoom | Zoom 0.3x–5.0x, stars scale, no clipping | scroll |
| 3.3 | Right-drag to pan | Camera pans smoothly, no jitter | drag |
| 3.4 | Left-click a system | System info panel appears on right side | click |
| 3.5 | Click cockpit chrome area | No galaxy map interaction (viewport constraint test) | click |

#### 4. System Info Panel (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 4.1 | Select a system with fleets | Fleets section shows fleet names + counts | click |
| 4.2 | Select a system with facilities | Facilities listed with correct types | click |
| 4.3 | Observe popularity bars | Alliance/Empire popularity bars render 0-100% | — |

#### 5. Officers Panel (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 5.1 | Press O or click Officers button | Panel opens, roster visible | O |
| 5.2 | Click a character row | Detail view: skills, Force, location, assignment | click |
| 5.3 | Press O again | Panel closes | O |

#### 6. Fleets Panel (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 6.1 | Press F or click Fleets button | Fleet list with composition | F |
| 6.2 | Click a fleet | Fleet editor: ships, officers, merge button | click |
| 6.3 | Verify mutual exclusion | Officers panel (if open) closes | F |

#### 7. Manufacturing Panel (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 7.1 | Press M | Production queue with enqueue/cancel | M |
| 7.2 | Click "Add to Queue" | Item appears in queue | click |
| 7.3 | Click "Cancel" on queued item | Item removed from queue | click |

#### 8. Missions Panel (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 8.1 | Press N | Mission types with dispatch controls | N |
| 8.2 | Select mission + character | Probability preview renders | click |

#### 9. Research Panel (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 9.1 | Press T | 3 tech tree tabs visible | T |
| 9.2 | Click Ship/Troop/Facility tab | Tab switches, progress bars render | click |
| 9.3 | Assign character to research | Character appears in assignment slot | click |

#### 10. Jedi Panel (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 10.1 | Press J | Force-sensitive roster visible | J |
| 10.2 | Observe tier progression | XP bars, tier labels (None/Aware/Training/Experienced) | — |

#### 11. Bombardment Panel (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 11.1 | Press B | Fleet selection + damage forecast visible | B |
| 11.2 | Verify B clears other panels | All 8 other panels close | B |

#### 12. Death Star Panel (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 12.1 | Press D | Construction progress, superlaser, movement | D |
| 12.2 | Verify D clears other panels | All 8 other panels close | D |

#### 13. Loyalty Panel (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 13.1 | Press L | Per-system danger + betrayal risk visible | L |
| 13.2 | Verify L clears other panels | All 8 other panels close | L |

#### 14. Encyclopedia (3 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 14.1 | Press E | 4-tab browser opens (floating window) | E |
| 14.2 | Click each tab | Content switches (Ships/Troops/Characters/Facilities) | click |
| 14.3 | Check BMP textures | Entity images render (from BmpCache) | — |

#### 15. Cockpit Buttons (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 15.1 | Click each of 9 cockpit buttons | Corresponding panel/action triggers | click |
| 15.2 | Verify speed buttons | SpeedDown/SpeedUp change game speed | click |

#### 16. Context Menus (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 16.1 | Right-click a system (no drag) | System context menu: control, popularity, garrison | rclick |
| 16.2 | Right-click a fleet icon | Fleet context menu: composition, commander, transit | rclick |

#### 17. Status Bar (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 17.1 | Observe bottom bar | Day counter, speed controls visible | — |
| 17.2 | Click speed controls in status bar | Game speed changes | click |

#### 18. Save/Load Panel (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 18.1 | Open Save/Load (cockpit button or shortcut) | 10 slots visible with save/load buttons | click |
| 18.2 | Save to slot 1, then load from slot 1 | Round-trip preserves game state | click |

#### 19. Message Log (2 scenarios)
| # | Action | Expected | Key |
|---|--------|----------|-----|
| 19.1 | Observe message log | Events appear with color-coded categories | — |
| 19.2 | Scroll message log | Scroll works, older events visible | scroll |

**Output**: `docs/qa/defect-inventory.md` — structured table:

```markdown
| ID | Panel | Scenario | Status | Description | Screenshot | Severity |
|----|-------|----------|--------|-------------|------------|----------|
| D001 | ... | ... | FAIL | ... | filename.png | P0/P1/P2 |
```

**Ownership**: READ-ONLY on all source files. Writes only to `docs/qa/`.

---

### Dabora 2: Rophe (רופא) — The Healer

*Etymology: from R-P-ʾ, "to heal" — the physician who mends what is broken. Cognate with Ugaritic rpu, the divine healer.*

**Role**: Fix every defect cataloged by Bōreret, plus the 3 pre-identified bugs.

**Pre-identified Fixes**:

#### Fix 1: Panel Mutual Exclusion (P0)
**File**: `crates/rebellion-app/src/main.rs`
**Change**: Extract a `close_all_panels()` closure or inline block. Every keyboard toggle (O/F/M/N/T/J/B/D/L) first calls `close_all_panels()`, then opens its own panel. Also update cockpit button handler to use the same logic.

```
Before:
  if is_key_pressed(KeyCode::O) {
      show_officers = !show_officers;
      if show_officers { show_fleets = false; show_manufacturing = false; show_missions = false; }
  }

After:
  if is_key_pressed(KeyCode::O) {
      let was_open = show_officers;
      // close all
      show_officers = false; show_fleets = false; show_manufacturing = false;
      show_missions = false; show_research = false; show_jedi = false;
      show_bombardment = false; show_death_star = false; show_loyalty = false;
      show_officers = !was_open;
  }
```

#### Fix 2: Cockpit Viewport Mouse Clamping (P0)
**File**: `crates/rebellion-render/src/lib.rs` (galaxy map input handling)
**Change**: Before processing mouse hover/click in `draw_galaxy_map`, check if mouse position is within the `CockpitViewport` rect. If outside, suppress hover detection and click handling.

#### Fix 3: Save/Load Cockpit Button Wiring (P1)
**File**: `crates/rebellion-app/src/main.rs`
**Change**: Verify `CockpitButton::SaveLoad` in the cockpit button match arm opens `show_save_load`. If missing, add it.

#### Dynamic Fixes (from Bōreret inventory)
Each defect gets a fix branch. Rophe reads `docs/qa/defect-inventory.md` and addresses P0 first, then P1.

**Ownership**:

| File | Access |
|------|--------|
| `crates/rebellion-app/src/main.rs` | **OWN** (keyboard handling, panel wiring) |
| `crates/rebellion-render/src/lib.rs` | **OWN** (galaxy map input clamping) |
| `crates/rebellion-render/src/cockpit.rs` | **OWN** (button handler fixes) |
| `crates/rebellion-render/src/panels/*.rs` | **OWN** (panel rendering fixes) |
| All other render files | READ |
| All core files | READ |

---

### Dabora 3: Yashar (ישר) — The Straightener

*Etymology: from Y-Sh-R, "to be straight, upright" — the one who makes level what is crooked. Cognate with Hebrew yāshār (righteous) and the Ugaritic divine epithet.*

**Role**: Close remaining non-combat UI parity gaps from the roadmap. These are structural omissions, not bugs.

**Parity Items** (from roadmap "Remaining" section, excluding tactical combat + droid advisor):

#### Parity 1: Cockpit Viewport Constraining (P0)
Already handled by Rophe Fix 2 — Yashar skips this.

#### Parity 2: Keyboard Mutual Exclusion Fix (P0)
Already handled by Rophe Fix 1 — Yashar skips this.

#### Parity 3: Save/Load Keyboard Shortcut (P1)
**File**: `crates/rebellion-app/src/main.rs`
**Change**: Add `KeyCode::S` (or `KeyCode::F5`/`KeyCode::F9`) to toggle `show_save_load` with mutual exclusion, matching the cockpit button.

#### Parity 4: Galaxy Map Overlay Polish (P1)
**File**: `crates/rebellion-render/src/lib.rs`
**Change**: Verify facility icons, sector boundaries, and blockade indicators render correctly at all zoom levels. Adjust icon scaling if they become unreadable at zoom < 0.5x or > 3.0x.

#### Parity 5: Panel Close-on-Outside-Click (P2)
**File**: `crates/rebellion-render/src/panels/*.rs`
**Change**: When the user clicks outside a panel's egui window, close it. Standard egui floating-window behavior.

#### Parity 6: Status Bar Day Counter Format (P2)
**File**: `crates/rebellion-render/src/lib.rs` (or status bar function)
**Change**: Verify day counter displays "Day X" with correct formatting. Check speed indicator shows Paused/1x/2x/4x.

**Ownership**:

| File | Access |
|------|--------|
| `crates/rebellion-app/src/main.rs` | SHARED with Rophe (Yashar works ONLY on save/load shortcut, separate from panel toggle block) |
| `crates/rebellion-render/src/lib.rs` | READ (overlay verification — report bugs to Rophe if found) |
| `crates/rebellion-render/src/panels/*.rs` | SHARED (Yashar: close-on-outside behavior; Rophe: rendering fixes) |

**Conflict Protocol**: Yashar works on lines/functions that Rophe does NOT touch. If both need the same block, Rophe commits first, Yashar rebases.

---

### Dabora 4: Sopher (סופר) — The Scribe

*Etymology: from S-P-R, "to count, recount, inscribe" — the royal scribe who makes the record visible to the world. In Ugaritic administration, the sōpher maintained the tablets that allowed the palace to be found.*

**Role**: Implement SEO/GEO optimization for the Ghidra documentation site (`ghidra-site/`). Runs entirely in parallel — touches NO game code files.

**Source**: `.subdaimon-output/scholiast-seo-geo-plan.md` — full implementation spec from Scholiast research.

**P0 Tasks** (must complete):

#### SEO 1: Site Description + Meta Plugin
**File**: `ghidra-site/mkdocs.yml`
**Change**: Expand `site_description` to full SEO text. Add `meta` plugin to `plugins:` list. This enables per-folder `.meta.yml` defaults.

#### SEO 2: Open Graph + Twitter Cards
**File**: `ghidra-site/overrides/main.html` (NEW)
**Change**: Create custom `main.html` template override that extends `base.html` and injects OG/Twitter meta tags in `{% block extrahead %}`. Tags: `og:type`, `og:url`, `og:title`, `og:description`, `og:image`, `twitter:card`, `twitter:title`, `twitter:description`.

#### SEO 3: robots.txt with AI Crawler Allow
**File**: `ghidra-site/docs/robots.txt` (NEW)
**Change**: Create `robots.txt` that explicitly allows 20+ AI crawlers (GPTBot, ClaudeBot, PerplexityBot, Google-Extended, etc.) with `Allow: /`. Reference `llms.txt`. Add to mkdocs.yml via `extra` or static file copy.

#### SEO 4: Social Cards Plugin
**File**: `ghidra-site/mkdocs.yml`
**Change**: Add `social` plugin with Star Wars dark theme (`background_color: "#0d0d0d"`, `color: "#ffc107"`). Requires `pip install pillow cairosvg`.

**P1 Tasks** (if time permits):

#### GEO 1: FAQ Sections
**Files**: `ghidra-site/docs/index.md`, `ghidra-site/docs/ai-functions.md`
**Change**: Add `## Frequently Asked Questions` sections with direct-answer prose. These are the highest-ROI GEO technique per the Princeton study.

#### GEO 2: Sources Sections in Scholar Docs
**Files**: `ghidra-site/docs/scholar/*.md`
**Change**: Add `## Sources` footer to each scholar doc citing the Ghidra project, REBEXE.EXE analysis, and relevant academic/community references.

#### GEO 3: Section Meta Files
**Files**: `ghidra-site/docs/scholar/.meta.yml`, `ghidra-site/docs/combat/.meta.yml`, `ghidra-site/docs/guide/.meta.yml` (all NEW)
**Change**: Per-folder default descriptions for search engines and AI crawlers.

**Ownership**:

| File | Access |
|------|--------|
| `ghidra-site/**` | **OWN** (exclusive) |
| All game code | NONE — do not read or touch |

---

### Dabora 5: Mashbīr (משביר) — The Verifier

*Etymology: from Sh-B-R, "to break" (in the hiphil: "to cause to break through, to deliver") — the one who breaks through to certainty. In Ugaritic royal inscriptions, the mashbīr is the gate-breaker who confirms the siege is over.*

**Role**: Re-run all 47 test scenarios from Bōreret's protocol after Rophe + Yashar finish. Confirm all defects are resolved. Run cargo test + WASM build to verify no regressions.

**Protocol**:
1. Rebuild WASM: `bash scripts/build-wasm.sh`
2. Re-serve: `python3 -m http.server 8080 -d web/`
3. Re-run ALL 47 scenarios from Bōreret's protocol
4. For each previously-failed scenario, screenshot the fix as evidence
5. Run `PATH="/usr/bin:$PATH" cargo test` — all 284+ tests must pass
6. Run `PATH="/usr/bin:$PATH" cargo check` — zero warnings
7. Build WASM one final time to confirm clean artifact

**Output**: `docs/qa/verification-report.md`

```markdown
## Verification Report — Knesset Sassuratu

| Defect ID | Status | Fix Commit | Re-test Result | Screenshot |
|-----------|--------|------------|----------------|------------|
| D001 | FIXED | abc1234 | PASS | fix-d001.png |
```

**Ownership**: READ-ONLY on all source files. Writes only to `docs/qa/`.

---

## File Ownership Matrix

| File | Bōreret | Rophe | Yashar | Sopher | Mashbīr |
|------|---------|-------|--------|--------|---------|
| `crates/rebellion-app/src/main.rs` | read | **OWN** (panel toggles, wiring) | write (save shortcut only) | — | read |
| `crates/rebellion-render/src/lib.rs` | read | **OWN** (mouse clamping) | read | — | read |
| `crates/rebellion-render/src/cockpit.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/panels/*.rs` | read | **OWN** (rendering) | write (close behavior) | — | read |
| `crates/rebellion-render/src/encyclopedia.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/message_log.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/*.rs` (other) | read | read | read | — | read |
| `crates/rebellion-core/src/**` | read | read | read | — | read |
| `ghidra-site/**` | — | — | — | **OWN** (exclusive) | read |
| `docs/qa/**` | **OWN** (create) | read | read | — | **OWN** (verify) |
| `scripts/build-wasm.sh` | exec | exec | — | — | exec |

---

## Dependency Chain

```
Phase 1: BUILD
  bash scripts/build-wasm.sh
  python3 -m http.server 8080 -d web/   (background)
      │
      ├──────────────────────────────────────────┐
      ▼                                          ▼
Phase 2: QA (Bōreret)                   Phase 2s: SEO/GEO (Sopher)
  47 test scenarios                        ghidra-site/ optimization
  → defect-inventory.md                    OG tags, robots.txt, FAQ, meta
      │                                          │
      ├──────────────────────────┐               │
      ▼                          ▼               │
Phase 3a: FIX (Rophe)     Phase 3b: PARITY      │
  Pre-identified bugs +     (Yashar)             │
  QA defect inventory       Non-combat UI gaps   │
      │                          │               │
      └────────────┬─────────────┘───────────────┘
                   ▼
Phase 4: VERIFY (Mashbīr)
  Re-run all 47 scenarios
  Verify ghidra-site OG tags + robots.txt
  cargo test + cargo check
  WASM rebuild
      │
      ▼
Phase 5: CLOSE
  Final report in docs/qa/verification-report.md
  Update roadmap.md: mark "UI Rebuild" remaining items
  Update CLAUDE.md version string if warranted
```

Wall-clock estimate: Bōreret ~1h + Sopher ~1h (parallel) → Rophe/Yashar ~2h parallel → Mashbīr ~1h = **~4h total**

---

## Acceptance Criteria

1. All 47 test scenarios PASS (verified by Mashbir)
2. All pre-identified bugs (mutual exclusion, viewport clamping, save/load wiring) are fixed
3. All P0 defects from QA inventory are resolved
4. `PATH="/usr/bin:$PATH" cargo test` passes (284+ tests, zero failures)
5. `PATH="/usr/bin:$PATH" cargo check` has zero warnings
6. `bash scripts/build-wasm.sh` produces a clean WASM artifact
7. Panel mutual exclusion is complete: any panel toggle closes all 8 others
8. Mouse input in cockpit chrome area does not interact with galaxy map
9. Every cockpit button opens its corresponding panel
10. Ghidra site has OG/Twitter meta tags in `overrides/main.html`
11. `robots.txt` exists with AI crawler explicit allow
12. `meta` plugin enabled with per-folder `.meta.yml` files
13. FAQ sections added to `index.md` and `ai-functions.md`

## Scope Boundaries (NOT included)

- **Tactical space combat QA**: Already built and tested by Knesset Demiurge
- **Droid Advisor System**: Requires BIN format RE — post-Knesset
- **Video playback**: WebM decoder — post-Knesset
- **HD asset bulk upscaling**: Visual polish — post-Knesset
- **Mod Manager QA**: Low priority, Tab toggle only
- **Native-only testing**: WASM is the canonical test target; egui parity assumed
