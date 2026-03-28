---
title: "Knesset Sassuratu — Visual QA + UI Parity Completion"
type: feat
status: active
date: 2026-03-24
project: open-rebellion
origin: docs/plans/2026-03-24-001-knesset-demiurge-ui-parity.md
tags: [knesset, swarm, qa, visual-testing, parity, panels, cockpit, seo, geo, ghidra-site]
---

# Knesset Sassuratu — Visual QA + UI Parity Completion

*ששורתו (Sassuratu) — "the midwife" in Ugaritic. She who draws the newborn into the light and verifies it is whole. Named for the midwives of the divine assembly who attend the birth of gods in the Ba'al Cycle (KTU 1.17). As midwife, Sassuratu inspects every limb of the delivered work—each panel, each button, each pixel—and calls out what is malformed before the child is presented.*

## Purpose

Three objectives for the Open Rebellion project after the Knesset Demiurge build sprint:

1. **Visual QA**: Launch the WASM build in a browser via agent-browser, navigate every UI panel systematically, screenshot and verify that each renders correctly and is functionally interactive.

2. **UI Parity Completion**: Fix every broken panel, missing wiring, visual bug, or incomplete feature found during QA. Close remaining non-combat UI gaps from the roadmap.

3. **SEO/GEO Optimization**: Bring the Ghidra documentation site (`ghidra-site/`) to parity with the minoanmystery-astro SEO/GEO implementation—OG/Twitter cards, AI-crawler-explicit robots.txt, FAQ sections, source citations, and structured data. Based on the Scholiast research plan (`.subdaimon-output/scholiast-seo-geo-plan.md`).

## Architecture Decisions

1. **QA-first, fix-second**: No code changes until QA produces a complete defect inventory. Prevents chasing phantoms.
2. **WASM is the test target**: All visual QA runs against the browser build. If WASM works, native works (egui renders identically). WASM also exercises the HTTP fetch data path.
3. **Automated screenshots as evidence**: Every test scenario produces a timestamped screenshot in `docs/qa/screenshots/`. Defects reference screenshots by filename.
4. **Fix in smallest scope**: Each bug fix is a single commit touching minimal files. No refactoring during fix phase.
5. **Re-verify after fix**: Every fix is re-tested by the verification dabora before the Knesset closes.
6. **SEO/GEO is isolated**: The Ghidra site dabora touches only `ghidra-site/` files—zero overlap with game code. Runs fully parallel with all other daborot.

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
| 4 | **Kēryx** (κῆρυξ "The Herald") | SEO/GEO — Ghidra docs site optimization | sonnet | Immediately (parallel with all — isolated files) |
| 5 | **Mashbīr** (משביר "The Verifier") | Re-test — confirm every fix, parity item, and site deploy | sonnet | After Rophe + Yashar + Kēryx complete |

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

### Dabora 4: Kēryx (κῆρυξ) — The Herald

*Etymology: from Greek κῆρυξ (kēryx), "herald, crier" — the one who proclaims the name abroad. In the Eleusinian Mysteries, the kēryx was the sacred herald who announced the rites to the world. Here, Kēryx announces the Ghidra RE documentation to search engines and AI citation systems.*

**Role**: SEO and GEO optimization for the Ghidra documentation site at `ghidra-site/`. Based on the Scholiast research plan (`.subdaimon-output/scholiast-seo-geo-plan.md`), porting patterns from the minoanmystery-astro reference implementation.

**Source**: `.subdaimon-output/scholiast-seo-geo-plan.md` — Sections 1–8, confidence assessments, priority matrix.

**Tools**: File editing only. No build tools, no browser automation. All changes are Markdown, YAML, HTML, and plain text within `ghidra-site/`.

#### P0 Items (High Impact, Low Effort)

##### SEO-1: Expand `site_description` in `mkdocs.yml`
**File**: `ghidra-site/mkdocs.yml`
**Change**: Replace the current terse description:
```yaml
# Before:
site_description: "Reverse engineering documentation for Star Wars Rebellion (1998, LucasArts)"

# After:
site_description: "Reverse engineering documentation for Star Wars Rebellion (1998, LucasArts). 5,127 decompiled functions from REBEXE.EXE, complete combat formula derivation, 111 GNPRTB parameters mapped, C++ class hierarchy reconstructed, AI behavior analysis."
```

##### SEO-2: Create `overrides/main.html` with OG + Twitter Meta Tags
**Files**: `ghidra-site/overrides/main.html` (NEW), `ghidra-site/mkdocs.yml` (add `custom_dir: overrides`)
**Change**: Inject Open Graph and Twitter Card meta tags for social sharing previews. Template:

```html
{% extends "base.html" %}

{% block extrahead %}
  {# Open Graph #}
  <meta property="og:type" content="website" />
  <meta property="og:url" content="{{ page.canonical_url }}" />
  <meta property="og:title" content="{{ page.title }} — Open Rebellion RE" />
  <meta property="og:description" content="{{ page.meta.description | default(config.site_description) }}" />
  <meta property="og:image" content="{{ config.site_url }}assets/social-card.png" />
  <meta property="og:site_name" content="{{ config.site_name }}" />

  {# Twitter Card #}
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content="{{ page.title }} — Open Rebellion RE" />
  <meta name="twitter:description" content="{{ page.meta.description | default(config.site_description) }}" />
  <meta name="twitter:image" content="{{ config.site_url }}assets/social-card.png" />

  {# JSON-LD Structured Data #}
  <script type="application/ld+json">
  {
    "@context": "https://schema.org",
    "@graph": [
      {
        "@type": "WebSite",
        "@id": "{{ config.site_url }}#website",
        "name": "{{ config.site_name }}",
        "url": "{{ config.site_url }}",
        "description": "{{ config.site_description }}",
        "author": {
          "@type": "Person",
          "name": "Tom di Mino",
          "url": "https://www.minoanmystery.org",
          "sameAs": ["https://github.com/tdimino"]
        }
      },
      {
        "@type": "SoftwareSourceCode",
        "@id": "{{ config.site_url }}#project",
        "name": "Open Rebellion",
        "codeRepository": "https://github.com/tdimino/open-rebellion",
        "programmingLanguage": ["Rust", "C++"],
        "about": "Rust reimplementation of Star Wars Rebellion (1998), informed by exhaustive Ghidra decompilation of REBEXE.EXE"
      }
    ]
  }
  </script>
{% endblock %}
```

Also add `custom_dir: overrides` under `theme:` in `mkdocs.yml`.

##### SEO-3: Enable `meta` Plugin
**File**: `ghidra-site/mkdocs.yml`
**Change**: Add `- meta` to the `plugins:` block. This enables per-folder `.meta.yml` defaults and per-page front matter descriptions.

##### SEO-4: Create `docs/robots.txt` with AI Crawler Explicit Allow
**Files**: `ghidra-site/docs/robots.txt` (NEW), `ghidra-site/mkdocs.yml` (add to extra files)
**Content**:

```
User-agent: *
Allow: /

# AI Crawlers — Allow all content
User-agent: GPTBot
User-agent: ChatGPT-User
User-agent: Claude-Web
User-agent: ClaudeBot
User-agent: Anthropic-AI
User-agent: PerplexityBot
User-agent: Google-Extended
User-agent: Amazonbot
User-agent: OAI-SearchBot
User-agent: YouBot
User-agent: PetalBot
Allow: /

# LLMs.txt Discovery
LLMs-Txt: https://tdimino.github.io/open-rebellion/llms.txt

# Sitemap
Sitemap: https://tdimino.github.io/open-rebellion/sitemap.xml
```

Register as extra file in `mkdocs.yml` (note: mkdocs-material does not have `extra_files` — use a `hooks/` copy script or place in `docs/` root for automatic inclusion).

#### P1 Items (High Impact, Medium Effort)

##### GEO-1: Add FAQ Sections to `index.md` and `ai-functions.md`
**Files**: `ghidra-site/docs/index.md`, `ghidra-site/docs/ai-functions.md`
**Change**: Append `## Frequently Asked Questions` sections with question/answer pairs. These serve dual purpose: GEO citation signals (Princeton study: FAQ structure is high-ROI for AI citation) and potential FAQPage schema.org structured data.

Questions for `index.md`:
- What is REBEXE.EXE?
- How many functions were decompiled?
- What is the GNPRTB table?
- What tools were used for reverse engineering?
- How does the combat system work?

Questions for `ai-functions.md`:
- How does the AI evaluate the galaxy?
- How does fleet deployment work?
- What is the garrison strength formula?
- How does the AI choose attack targets?

##### GEO-2: Add `## Sources` Sections to Scholar Docs
**Files**: All 9 files in `ghidra-site/docs/scholar/*.md`
**Change**: Append a `## Sources` section to each scholar document citing:
1. Ghidra RE session date and function count
2. TheArchitect2018 wiki cross-reference
3. REBEXE.EXE binary details (version, MD5 if known)
4. Any related community or academic sources

Example footer:
```markdown
## Sources

1. REBEXE.EXE decompilation — Ghidra 11.x, 2026-03-23, 5,127 functions >100 bytes
2. TheArchitect2018, *Star Wars Rebellion Wiki* — cross-referenced 2026-03-23
3. Star Wars Rebellion v2.0 (1998, LucasArts/Coolhand Interactive) — Windows PE 32-bit
```

##### GEO-3: Rewrite Section Intros with Quantitative Claims
**Files**: `ghidra-site/docs/index.md`, `ghidra-site/docs/scholar/annotated-functions.md`, `ghidra-site/docs/combat/space-combat.md`
**Change**: Ensure the first 2 paragraphs of each major page lead with specific numbers rather than building context. AI citation tools prefer prose with statistics over tables for extraction.

Before: "In this document, we will examine how space combat works in the original game."
After: "Space combat in Star Wars Rebellion resolves through a 7-phase pipeline. Phase 1 computes fleet power ratios from GNPRTB[0x05] and GNPRTB[0x06]. The full pipeline processes 111 tunable parameters across 5,127 decompiled functions."

##### GEO-4: Create `docs/llms.txt`
**File**: `ghidra-site/docs/llms.txt` (NEW)
**Change**: Manual LLM-friendly site index (per minoanmystery-astro pattern):

```markdown
# Open Rebellion — Ghidra RE Documentation

> Exhaustive reverse engineering of Star Wars Rebellion (1998, LucasArts). 5,127 decompiled functions, full combat formula derivation, C++ class hierarchy reconstruction, AI behavior analysis.

## Sections

- [Overview](https://tdimino.github.io/open-rebellion/): Index, what was decompiled, key findings
- [AI Functions](https://tdimino.github.io/open-rebellion/ai-functions/): Galaxy evaluation, fleet deployment, targeting
- [Space Combat Pipeline](https://tdimino.github.io/open-rebellion/combat/space-combat/): 7-phase pipeline, GNPRTB params
- [Scholar Documents](https://tdimino.github.io/open-rebellion/scholar/annotated-functions/): Full annotated function index
- [Rust Implementation Guide](https://tdimino.github.io/open-rebellion/scholar/rust-implementation-guide/): Mapping C++ to Rust
- [Function Index (5,127)](https://tdimino.github.io/open-rebellion/indexes/function-index/): Complete decompiled function list
- [Game Guide](https://tdimino.github.io/open-rebellion/guide/scenario/): Player-facing mechanics documentation
```

#### P2 Items (Deferred — NOT in Knesset scope unless time permits)

- `social` plugin with Pillow/CairoSVG for auto-generated social cards (requires CI env check)
- `.meta.yml` files per section (`scholar/`, `combat/`, `guide/`)
- `FAQPage` JSON-LD schema for FAQ-bearing pages
- `mkdocs-llmstxt-md` plugin (automated `llms.txt` generation)
- Explicit sitemap plugin config with `changefreq`/`priority`

**Ownership**:

| File | Access |
|------|--------|
| `ghidra-site/mkdocs.yml` | **OWN** |
| `ghidra-site/overrides/` (NEW dir) | **OWN** |
| `ghidra-site/overrides/main.html` (NEW) | **OWN** |
| `ghidra-site/docs/robots.txt` (NEW) | **OWN** |
| `ghidra-site/docs/llms.txt` (NEW) | **OWN** |
| `ghidra-site/docs/index.md` | **OWN** (append FAQ + rewrite intro) |
| `ghidra-site/docs/ai-functions.md` | **OWN** (append FAQ + rewrite intro) |
| `ghidra-site/docs/scholar/*.md` | **OWN** (append Sources sections) |
| `ghidra-site/docs/combat/space-combat.md` | **OWN** (rewrite intro) |
| All `crates/` files | NONE (no access) |
| All `docs/qa/` files | NONE (no access) |

**Zero file overlap with Daborot 1–3 and 5.** Kēryx is fully isolated.

---

### Dabora 5: Mashbīr (משביר) — The Verifier

*Etymology: from Sh-B-R, "to break" (in the hiphil: "to cause to break through, to deliver") — the one who breaks through to certainty. In Ugaritic royal inscriptions, the mashbīr is the gate-breaker who confirms the siege is over.*

**Role**: Re-run all 47 test scenarios from Bōreret's protocol after Rophe + Yashar finish. Confirm all defects are resolved. Run cargo test + WASM build to verify no regressions. Verify Kēryx's SEO/GEO changes with `mkdocs build`.

**Protocol**:
1. Rebuild WASM: `bash scripts/build-wasm.sh`
2. Re-serve: `python3 -m http.server 8080 -d web/`
3. Re-run ALL 47 scenarios from Bōreret's protocol
4. For each previously-failed scenario, screenshot the fix as evidence
5. Run `PATH="/usr/bin:$PATH" cargo test` — all 284+ tests must pass
6. Run `PATH="/usr/bin:$PATH" cargo check` — zero warnings
7. Build WASM one final time to confirm clean artifact
8. Verify Ghidra site: `cd ghidra-site && mkdocs build --strict` — zero warnings
9. Spot-check OG meta tags in built HTML: `grep "og:description" ghidra-site/site/index.html`
10. Verify `robots.txt` present in `ghidra-site/site/robots.txt`
11. Verify `llms.txt` present in `ghidra-site/site/llms.txt`

**Output**: `docs/qa/verification-report.md`

```markdown
## Verification Report — Knesset Sassuratu

### Game UI Verification
| Defect ID | Status | Fix Commit | Re-test Result | Screenshot |
|-----------|--------|------------|----------------|------------|
| D001 | FIXED | abc1234 | PASS | fix-d001.png |

### Ghidra Site SEO/GEO Verification
| Item | Status | Evidence |
|------|--------|----------|
| OG meta tags in index.html | PASS/FAIL | grep output |
| robots.txt present | PASS/FAIL | file check |
| llms.txt present | PASS/FAIL | file check |
| mkdocs build --strict | PASS/FAIL | exit code |
| FAQ sections in index.md | PASS/FAIL | visual check |
| Sources in scholar docs | PASS/FAIL | grep count |
```

**Ownership**: READ-ONLY on all source files. Writes only to `docs/qa/`.

---

## File Ownership Matrix

| File | Bōreret | Rophe | Yashar | Kēryx | Mashbīr |
|------|---------|-------|--------|-------|---------|
| `crates/rebellion-app/src/main.rs` | read | **OWN** | write (save shortcut) | — | read |
| `crates/rebellion-render/src/lib.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/cockpit.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/panels/*.rs` | read | **OWN** | write (close behavior) | — | read |
| `crates/rebellion-render/src/encyclopedia.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/message_log.rs` | read | **OWN** | read | — | read |
| `crates/rebellion-render/src/*.rs` (other) | read | read | read | — | read |
| `crates/rebellion-core/src/**` | read | read | read | — | read |
| `ghidra-site/mkdocs.yml` | — | — | — | **OWN** | read |
| `ghidra-site/overrides/**` | — | — | — | **OWN** | read |
| `ghidra-site/docs/*.md` | — | — | — | **OWN** | read |
| `ghidra-site/docs/scholar/*.md` | — | — | — | **OWN** | read |
| `ghidra-site/docs/combat/*.md` | — | — | — | **OWN** | read |
| `ghidra-site/docs/robots.txt` | — | — | — | **OWN** | read |
| `ghidra-site/docs/llms.txt` | — | — | — | **OWN** | read |
| `docs/qa/**` | **OWN** | read | read | — | **OWN** |
| `scripts/build-wasm.sh` | exec | exec | — | — | exec |

---

## Dependency Chain

```
Phase 1: BUILD
  bash scripts/build-wasm.sh
  python3 -m http.server 8080 -d web/   (background)
      │
      ├────────────────────────────────────────────────────────────┐
      ▼                                                            ▼
Phase 2: QA (Bōreret)                              Phase 2': SEO/GEO (Kēryx)
  47 test scenarios → defect-inventory.md             ghidra-site/ optimization
      │                                               (FULLY PARALLEL — no deps)
      ├──────────────────────────┐                         │
      ▼                          ▼                         │
Phase 3a: FIX (Rophe)     Phase 3b: PARITY (Yashar)       │
  Pre-identified bugs +         Non-combat UI gaps         │
  QA defect inventory           (no file conflicts)        │
      │                          │                         │
      └────────────┬─────────────┘─────────────────────────┘
                   ▼
Phase 4: VERIFY (Mashbīr)
  Re-run all 47 scenarios
  cargo test + cargo check
  WASM rebuild
  mkdocs build --strict
  OG/robots/llms verification
      │
      ▼
Phase 5: CLOSE
  Final report in docs/qa/verification-report.md
  Update roadmap.md: mark "UI Rebuild" remaining items
  Update CLAUDE.md version string if warranted
```

Wall-clock estimate: Bōreret ~1h ∥ Kēryx ~1.5h → Rophe/Yashar ~2h parallel → Mashbīr ~1h = **~4h total** (Kēryx overlaps with QA phase)

---

## Acceptance Criteria

### Game UI (Daborot 1–3, 5)
1. All 47 test scenarios PASS (verified by Mashbīr)
2. All pre-identified bugs (mutual exclusion, viewport clamping, save/load wiring) are fixed
3. All P0 defects from QA inventory are resolved
4. `PATH="/usr/bin:$PATH" cargo test` passes (284+ tests, zero failures)
5. `PATH="/usr/bin:$PATH" cargo check` has zero warnings
6. `bash scripts/build-wasm.sh` produces a clean WASM artifact
7. Panel mutual exclusion is complete: any panel toggle closes all 8 others
8. Mouse input in cockpit chrome area does not interact with galaxy map
9. Every cockpit button opens its corresponding panel

### Ghidra Site SEO/GEO (Dabora 4)
10. `mkdocs build --strict` passes with zero warnings
11. Built `index.html` contains `og:title`, `og:description`, `og:image`, `twitter:card` meta tags
12. Built `index.html` contains JSON-LD `WebSite` + `SoftwareSourceCode` structured data
13. `robots.txt` present in built site root with AI crawler `Allow: /` directives
14. `llms.txt` present in built site root with section index
15. `index.md` and `ai-functions.md` each have a `## Frequently Asked Questions` section
16. All 9 `scholar/*.md` files have a `## Sources` section
17. Major page intros lead with quantitative claims (5,127 functions, 111 parameters, 7-phase pipeline)

## Scope Boundaries (NOT included)

- **Tactical space combat QA**: Already built and tested by Knesset Demiurge
- **Droid Advisor System**: Requires BIN format RE — post-Knesset
- **Video playback**: WebM decoder — post-Knesset
- **HD asset bulk upscaling**: Visual polish — post-Knesset
- **Mod Manager QA**: Low priority, Tab toggle only
- **Native-only testing**: WASM is the canonical test target; egui parity assumed
- **`social` plugin** (auto-generated social cards): Requires Pillow/CairoSVG in CI — P2
- **Per-section `.meta.yml`**: Low-effort but low-impact — P2
- **`FAQPage` JSON-LD schema**: Per-page injection requires custom hook — P2
- **`mkdocs-llmstxt-md` plugin**: Young plugin (1 contributor) — manual `llms.txt` preferred
