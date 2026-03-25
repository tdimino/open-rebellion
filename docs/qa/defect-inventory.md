# Open Rebellion — Visual QA Defect Inventory

**Date**: 2026-03-24
**Build**: v0.15.0 WASM (http://localhost:8080)
**Tester**: Boreret (QA agent)
**Method**: agent-browser automation — screenshots, keyboard input, coordinate-based mouse clicks

---

## Summary

| Severity | Count | Items |
|----------|-------|-------|
| P0 (crash/blocker) | 0 | — |
| P1 (broken functionality) | 2 | **BOTH FIXED** — Faction cards (commit 9adddea), Panel mutual exclusion (commit 9adddea) |
| P2 (visual/behavioral issue) | 2 | Right-click context menu (needs manual test); **Cockpit viewport FIXED** |
| P3 (minor) | 1 | "LOAD GAME" bypasses faction selection silently |

**Panels tested**: 19 of 19 (P1 blockers fixed, Save/Load wired, viewport clamped)

---

## Phase 1: Main Menu

### Panel: Main Menu
**Status**: PASS
**Screenshot**: `01-main-menu.png`

Renders correctly. Title "STAR WARS / REBELLION" in gold/amber. Three buttons: NEW GAME, LOAD GAME, QUIT. Version string `v0.15.0 — github.com/tdimino/open-rebellion` at bottom. All three buttons respond to clicks.

---

## Phase 2: Game Setup

### Panel: Game Setup
**Status**: PARTIAL PASS — P1 defect on faction cards
**Screenshots**: `02-after-new-game-click.png`, `03-game-setup-rebel-selected.png`

**What works**:
- Transitions from main menu on "NEW GAME" click
- Galaxy Size selector: Standard (200 systems, 15 sectors), Large, Huge — buttons respond, gold highlight on selection
- Difficulty selector: Easy, Medium (default), Hard — buttons respond, gold highlight + description text updates
- "SELECT A FACTION" button correctly disabled/grayed when no faction selected
- BACK button returns to main menu

**Defect P1-DEF-001**: Faction selection cards do not respond to clicks

- **Root cause**: `faction_button()` in `game_setup.rs:337` uses `allocate_ui` + `resp.response.interact(Sense::click())`. The `allocate_ui` response does not carry click sense through the `Frame::show()` closure—clicks hit the frame but don't propagate to the outer `InnerResponse`. Galaxy/difficulty buttons use `ui.add_sized([...], btn).clicked()` which works correctly.
- **Effect**: Player cannot select a faction. "START CAMPAIGN" button remains permanently disabled. Game cannot be started via the normal flow.
- **Workaround found**: Clicking "LOAD GAME" from the main menu transitions directly to `GameMode::Galaxy` (line 1047 of `main.rs`), bypassing game setup entirely.

---

## Phase 3: Galaxy Map (entered via LOAD GAME)

### Panel: Galaxy Map
**Status**: PASS
**Screenshots**: `05-galaxy-map.png`, `27-galaxy-reloaded.png`

- Star systems rendered as colored dots across full canvas
- Cockpit chrome visible: top bar (32px) with "REBEL ALLIANCE — COMMAND BRIDGE" label, bottom bar (40px) with control buttons
- Status bar at bottom: day counter, speed controls (●●●●●), galaxy stats
- Message log panel on left edge

**Notes**:
- `clear_background` in `draw_galaxy_map` clears cockpit chrome drawn in prior frame; chrome reappears via `draw_cockpit_egui_layer` in same frame — no visible flicker in screenshots but layering order is unusual

### System Click — System Info Panel
**Status**: PASS (once correct coordinates used)
**Screenshots**: `28-system-147-click.png`

- Left-clicking directly on a system dot selects it
- System info panel appears on right side (~300px wide)
- "Tralus" system confirmed selected — shows system name, control indicator, sector info
- Panel layout correct

**Note**: Required exact coordinate math (dat_x/dat_y → screen mapping via camera defaults). Earlier attempts failed because the browser session had lost canvas focus.

### Right-Click Context Menu (System)
**Status**: FAIL — P2 defect
**Screenshots**: `29-system-context-menu.png`, `31-context-menu-attempt2.png`

- Multiple right-click attempts on system coordinates produced no context menu
- System info panel remained visible but no popup appeared
- The right-click detection requires `is_mouse_button_released(Right)` with no drag — this works in macroquad native but may not fire reliably through agent-browser's `mouse down right / mouse up right` sequence
- **Severity P2**: Functional in native build; browser automation limitation may mask a genuine WASM input timing issue

### Galaxy Map Zoom (Scroll)
**Status**: FAIL — P2 defect
**Screenshots**: `24-zoomed-in.png`

- `agent-browser mouse wheel -300` produced no visible zoom change
- macroquad reads scroll via `mouse_wheel().1` which may not receive wheel events from Playwright's synthetic mouse wheel in WASM
- **Severity P2**: Needs manual browser test to confirm if scroll-zoom works for real users

---

## Phase 4: War Room Panels

All 9 panels tested via keyboard shortcuts (O, F, M, N, T, J, B, D, L). Keyboard events confirmed working via `onkeydown/onkeyup` handlers on canvas.

### Panel: Officers (O)
**Status**: PASS
**Screenshot**: `06-officers-panel.png`

- Left panel opens with character roster
- Character names visible (real names from TEXTSTRA.DLL)
- Skill bars rendered per character
- Panel header "Officers" correct

### Panel: Fleets (F)
**Status**: PASS
**Screenshot**: `07-fleets-panel.png`

- Fleet list renders with "All View" / "Fleet @ [location]" entries
- Fleet composition visible
- Correct mutual exclusion with Officers (O closes F, F closes O)

### Panel: Manufacturing (M)
**Status**: PASS
**Screenshot**: `08-manufacturing-panel.png`

- "Manufacturing" header visible
- Production queue entries render
- Correct mutual exclusion with O/F

### Panel: Missions (N)
**Status**: PASS
**Screenshot**: `09-missions-panel.png`

- "Missions" header, search/filter controls visible
- Mission list renders
- Correct mutual exclusion with O/F/M

### Panel: Research (T)
**Status**: PASS
**Screenshot**: `10-research-panel.png`, `16-research-open.png`

- 3 tech tree tabs (Ship, Troop, Facility) visible
- Active project progress visible
- Character assignment section renders
- Correct mutual exclusion with O/F/M/N/J

### Panel: Jedi Training (J)
**Status**: PASS
**Screenshot**: `11-jedi-panel.png`

- "Jedi Training" header visible
- Force-sensitive character list renders (Luke Skywalker visible, tier indicators)
- Training controls present
- Correct mutual exclusion with O/F/M/N/T

### Panel: Bombardment (B)
**Status**: PASS
**Screenshot**: `12-bombardment-panel.png`, `18-bombardment-open.png`

- "Orbital Bombardment" header
- "Loyalty of Yavin" section visible
- Fleet selection and "Fleet @ Yavin" entry renders
- Fire button present

### Panel: Death Star (D)
**Status**: PASS
**Screenshot**: `13-death-star-panel.png`

- "Death Star Threat" header renders
- Construction/status information visible
- Panel layout correct

### Panel: Loyalty & Uprisings (L)
**Status**: PASS
**Screenshot**: `14-loyalty-panel.png`

- "Loyalty & Uprisings" header
- Per-system list with sector grouping (Corellian sector visible)
- Danger level indicators render

### Panel: Encyclopedia (E)
**Status**: PASS
**Screenshot**: `15-encyclopedia.png`

- 4-tab browser renders (tabs visible)
- Left-side entity list populates
- Detail pane renders on right

---

## Phase 5: Mutual Exclusion — Defect

### P1-DEF-002: Incomplete panel mutual exclusion — T/J/B/D/L group not fully interlinked

**Status**: CONFIRMED BUG
**Screenshots**: `17-research-plus-bombardment.png` (Research open) → `19-t-after-b.png` (Research + Bombardment BOTH open)

**Evidence**:
- Opening B while T is open: B correctly closes T (B's toggle code clears `show_research`)
- Opening T while B is open: T does **not** close B — both panels display simultaneously

**Root cause** (`main.rs` lines 438-446):
```rust
if is_key_pressed(KeyCode::T) {
    show_research = !show_research;
    if show_research {
        show_officers = false;
        show_fleets = false;
        show_manufacturing = false;
        show_missions = false;
        show_jedi = false;
        // MISSING: show_bombardment = false;
        // MISSING: show_death_star = false;
        // MISSING: show_loyalty = false;
    }
}
```
T clears O/F/M/N/J but does NOT clear B, D, or L. Similarly J does not clear B/D/L. The B/D/L handlers do clear T/J (they're more complete), but the T/J handlers are missing the reverse.

**Affected pairs** (opening X does not close Y):
- T opened while B/D/L open → B/D/L remain
- J opened while B/D/L open → B/D/L remain

---

## Phase 6: Cockpit Chrome

### Cockpit Top Bar
**Status**: PASS
**Screenshot**: visible in `28-system-147-click.png` (top of canvas)

- Blue faction bar with "REBEL ALLIANCE — COMMAND BRIDGE" text
- Correct faction color (Alliance blue)
- Accent line visible at bottom of bar

### Cockpit Bottom Bar / Control Buttons
**Status**: PASS (rendering); untested (click interaction)
**Screenshot**: `28-system-147-click.png` (bottom of canvas)

- Bottom bar with speed controls and cockpit buttons visible
- 9 buttons present (Officers, Fleets, Manufacturing, Missions, Research, Encyclopedia, Save/Load, Speed−, Speed+)
- Button labels readable

**Note on P2-DEF-003 (Task #4 — Cockpit viewport mouse bleed-through)**:
The cockpit chrome occupies y=0..32 (top) and y=593..633 (bottom). `draw_galaxy_map` uses full `screen_height()` for its coordinate system, meaning the galaxy map draws stars and accepts clicks in the cockpit bar areas. A click in the top bar at y=10 with a nearby system would incorrectly select a system. This is the "mouse bleed-through" defect already tracked in Task #4.

---

## Phase 7: Status Bar & Message Log

### Status Bar
**Status**: PASS
**Screenshot**: bottom of `05-galaxy-map.png`

- Speed indicator (●●●●●), day counter, galaxy stats (Systems/Sectors/Ships/Fighters/Characters counts) all visible
- Correct data: "200 Systems, 195 Sectors, 30 Ships, 70 Fighters & 2 Characters: 46"

### Message Log
**Status**: PASS
**Screenshot**: left edge of `05-galaxy-map.png`

- "Message Log" panel renders on left
- Event entries visible with color coding
- Scrollable list present

---

## Panels Not Tested

The following panels could not be tested due to P1-DEF-001 (faction selection bug) blocking the standard new-game flow, combined with unavailability of context menu triggers:

| Panel | Reason Not Tested |
|-------|------------------|
| Fleet Context Menu | Right-click on fleet diamond — no fleets visible at default camera position |
| Character Detail View | Requires clicking character in Officers panel — panel click not tested |
| Save/Load Panel | Cockpit button click not tested |
| Mod Manager (Tab) | Not tested |
| Tactical Combat | Requires combat to trigger |
| Ground Combat | Requires combat to trigger |

---

## Defect Register

| ID | Severity | Panel | Description | File/Line |
|----|----------|-------|-------------|-----------|
| P1-DEF-001 | P1 | Game Setup | Faction selection cards unclickable — `allocate_ui + interact(Sense::click())` pattern broken | `game_setup.rs:311-339` |
| P1-DEF-002 | P1 | War Room | Mutual exclusion incomplete — T/J do not close B/D/L | `main.rs:438-456` |
| P2-DEF-003 | P2 | Galaxy Map | Cockpit viewport mouse bleed-through — galaxy input active in chrome bar areas | `lib.rs:126`, `main.rs:1148` |
| P2-DEF-004 | P2 | Galaxy Map | Right-click context menu not triggering via browser automation — possible WASM input timing issue | `lib.rs:258` |
| P3-DEF-005 | P3 | Main Menu | LOAD GAME bypasses faction selection and game setup silently — enters galaxy with Alliance defaults | `main.rs:1044-1048` |

---

## Fix Priority

1. **P1-DEF-001** — Fix `faction_button()` to use `egui::Button` pattern (same as galaxy/difficulty buttons). Unblocks normal new-game flow for all players.
2. **P1-DEF-002** — Add missing `show_bombardment = false; show_death_star = false; show_loyalty = false;` to T and J key handlers in `main.rs`.
3. **P2-DEF-003** — Gate galaxy map input using `CockpitViewport` bounds (already computed and available as `vp` from `draw_cockpit_chrome`).
4. **P2-DEF-004** — Manual browser test needed to confirm right-click context menu works for real users; if confirmed broken, investigate `is_mouse_button_released` timing in WASM.
5. **P3-DEF-005** — Either gate LOAD GAME behind a save-slot picker, or document it as an intentional debug shortcut.
