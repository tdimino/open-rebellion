---
title: "Knesset Demiurge — UI Parity Swarm"
type: feat
status: active
date: 2026-03-24
project: open-rebellion
origin: docs/plans/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md
tags: [knesset, swarm, ui, parity, cockpit, tactical, audio]
---

# Knesset Demiurge — Open Rebellion UI Parity Swarm

*Merged from Codex ExecPlan (7 milestones) + Nomos Knesset Blueprint (5 daborot, 13 tasks).*

## Purpose

Replace all remaining debug-era presentation seams so Open Rebellion feels like Star Wars Rebellion (1998) during normal play. "Parity" means matching what a player can see, click, and hear: cockpit frame, cockpit buttons, galaxy-map facility/blockade overlays, tactical combat view, faction music/voice, and HD art polish.

## Architecture Decisions (from Codex)

1. **Asset-first**: Shared BMP/WAV loader + staging script before any visual feature
2. **Immutable source refs**: `assets/references/` untouched; staged copies in `data/base/ui/` and `web/data/ui/`
3. **Tactical split**: Foundation (GameMode + session) separate from completion (combat API + HUD)
4. **HD deferred**: Original art path first, HD overrides added last
5. **Keyboard fallback**: Shortcuts preserved until cockpit buttons fully cover them

## 5 Daborot

| # | Dabora | Phases | LOC | Model | Start |
|---|--------|--------|-----|-------|-------|
| 1 | **Kaptaru** (כפתור Caphtor) | BMP cache + Cockpit (2.1-2.2) | ~450 | sonnet | Immediately |
| 2 | **Athirat** (אתרת Lady of the Sea) | Galaxy overlays (3.1, 3.2, 3.5) | ~260 | sonnet | After Kaptaru task #1 |
| 3 | **Anat** (ענת Lady of Battle) | Tactical combat (5.1-5.8) | ~2,350 | opus | After Kaptaru task #1 |
| 4 | **Shapash** (שפש Torch of the Gods) | Audio + Visual (6, 7) | ~530 | sonnet | Audio immediately; visual after Kaptaru |
| 5 | **Sassuratu** (ששורתו midwife) | Integration testing | ~200 | sonnet | After ALL |

## 13 Tasks

| # | Task | Dabora | Blocked By | Files (OWN) |
|---|------|--------|------------|-------------|
| 1 | BMP texture cache + asset staging | Kaptaru | -- | bmp_cache.rs (NEW), scripts/stage-ui-assets.py |
| 2 | Cockpit frame rendering | Kaptaru | 1 | cockpit.rs (NEW), lib.rs |
| 3 | Cockpit control buttons | Kaptaru | 2 | cockpit.rs, main.rs |
| 4 | Facility status icons on galaxy map | Athirat | 1 | lib.rs (galaxy draw fns) |
| 5 | Sector boundary polygons | Athirat | 1 | lib.rs |
| 6 | Blockade visual indicators | Athirat | 1 | lib.rs |
| 7 | Combat arena + ship sprites + placement | Anat | 1 | tactical_view.rs (NEW), main.rs (GameMode) |
| 8 | Real-time phased combat + HUD | Anat | 7 | tactical_view.rs, combat.rs |
| 9 | Ship targeting, orders, retreat | Anat | 8 | tactical_view.rs |
| 10 | Ground combat + results | Anat | 7 | ground_combat.rs (NEW), combat.rs |
| 11 | Audio (soundtrack, voice, SFX) | Shapash | NONE | audio.rs |
| 12 | Entity portraits + event screens | Shapash | 1 | encyclopedia.rs, panels/officers.rs |
| 13 | Integration testing + verify | Sassuratu | 3,6,10,11,12 | tests/integration.rs (NEW) |

## File Ownership Matrix

| File | Kaptaru | Athirat | Anat | Shapash | Sassuratu |
|------|---------|---------|------|---------|-----------|
| bmp_cache.rs (NEW) | **OWN** | read | read | read | read |
| cockpit.rs (NEW) | **OWN** | -- | -- | -- | read |
| lib.rs | **OWN→read** | **OWN** (galaxy fns) | -- | -- | read |
| tactical_view.rs (NEW) | -- | -- | **OWN** | -- | read |
| ground_combat.rs (NEW) | -- | -- | **OWN** | -- | read |
| combat.rs (core) | -- | -- | **OWN** | -- | read |
| main.rs | **OWN→read** | -- | **OWN** (GameMode) | -- | read |
| audio.rs | -- | -- | -- | **OWN** | read |
| encyclopedia.rs | -- | -- | -- | **OWN** | read |
| panels/officers.rs | -- | -- | -- | **OWN** | read |
| tests/integration.rs (NEW) | -- | -- | -- | -- | **OWN** |

## Critical Path

```
Kaptaru #1 (BMP cache, ~1h)
  ├── Kaptaru #2-3 (cockpit, ~1h) ──────────────────────────────────┐
  ├── Athirat #4-6 (overlays, ~1.5h) ─────────────────────────────┐ │
  ├── Anat #7-10 (tactical, ~4.5h) ──────────────────────────────┐│ │
  └── Shapash #12 (portraits, ~1h) ────────────────────────────┐ ││ │
Shapash #11 (audio, ~1h, NO DEPS) ───────────────────────────┐│ ││ │
                                                              ││ ││ │
                                               Sassuratu #13 ←┘└─┘└─┘
```

Wall-clock: ~7h (Kaptaru 1.5h → Anat 4.5h → Sassuratu 1h)

## Conflict Resolution

1. **main.rs**: Kaptaru commits first (cockpit wiring). Anat rebases before adding GameMode.
2. **lib.rs**: Kaptaru commits first (cockpit viewport). Athirat works only in galaxy draw fns.
3. All other files have single ownership — no conflicts.

## Acceptance Criteria

- Both factions render distinct cockpit chrome with clickable buttons
- Systems with facilities show correct icon family on galaxy map
- Blockaded systems display blockade marker
- Player-involved battles enter TacticalCombat mode (not auto-resolve)
- Background music plays, at least one voice line triggers
- HD PNG preferred when present, falls back to original BMP
- `PATH="/usr/bin:$PATH" cargo test` passes
- `bash scripts/build-wasm.sh` succeeds

## Scope Boundaries (NOT included)

- Droid Advisor (2.3): BIN format RE needed — post-Knesset
- Video playback: WebM decoder for WASM — post-Knesset
- Save/Load polish: P2
- Tech Tree Visualizer: P2
