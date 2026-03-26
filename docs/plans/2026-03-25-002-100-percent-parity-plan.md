---
title: "100% Parity Plan — Core + Combat + UI"
type: feat
status: active
date: 2026-03-25
project: open-rebellion
tags: [parity, combat, ai, seeding, assets, upscaling]
---

# Open Rebellion: 100% Parity Plan

*Synthesized from 3 parallel gap analyses (core: 24 gaps, combat: 11 gaps, UI: 13 categories) + unified planner.*

## Current → Target

| Dimension | Current | Target | Gap LOC |
|-----------|---------|--------|---------|
| Core | 97% | 100% | ~800 (Wave 2 remaining) |
| Combat | 97% | 100% | ~200 (review fixes) |
| UI | 97% | 100% | ~800 (Wave 4 remaining) + upscaling |
| **Total** | ~97% | 100% | **~1,800 LOC** |

## 4 Waves (3 sequential Knessets + 1 background batch)

### Wave 1: Combat Formula Parity — DONE (Knesset Ma'at, 2026-03-25)
- [x] Shield absorption phase (Phase 4) — pending damage pipeline, ion 2x, recharge (+8 tests)
- [x] Fighter combat — launch/dogfight/recall from carriers, per-weapon stats (+6 tests)
- [x] Ground combat — per-unit resolution with TroopClassDef + facility bonus (+6 tests)
- [x] Difficulty modifiers wired into space + ground combat
- [ ] **REVIEW FIX NEEDED**: Shield absorption `raw_consumed` double-rounding — hull overflow slightly under-damages (combat.rs:391-405)
- [ ] **REVIEW FIX NEEDED**: `recall_fighters` overwrites unlaunched (grounded) squadrons to 0 — silent fighter loss
- [ ] **REVIEW FIX NEEDED**: Hardcoded `difficulty: 2` in simulation.rs:305 — ground combat ignores actual game difficulty
- [ ] Wire 74 unused GNPRTB combat parameters (deferred — most are informational)
- [ ] Tactical view fighter damage bypasses shields (inconsistent with auto-resolve path)
- [ ] Tactical view weapon type determined by hull_max bracket, not actual weapon stats

### Wave 2: AI + Core Gaps (~800 LOC) — NEXT
- 14 missing dispatch validators (FUN_00508250)
- Troop deployment AI (AIAction::MoveTroops)
- Faction deployment budgets + Death Star AI
- Mission/event/character edge cases

### Wave 3: BMP Integration + Upscaling — PARTIALLY DONE (Knesset Ma'at)
- [x] Cockpit button sprites (9 buttons, 3-state from COMMON.DLL)
- [x] Event screen overlay (61 STRATEGY.DLL BMPs, timer + click-dismiss)
- [x] Tactical HUD panels + weapon gauge + hull/shield display (TACTICAL.DLL)
- [x] GOKRES mini-icons in fleet panel detail view
- [ ] **REVIEW FIX NEEDED**: Event screen blocks egui mouse but NOT keyboard shortcuts — added guard for galaxy shortcuts but Escape path needs verification
- [ ] **REVIEW FIX NEEDED**: `ButtonSprite::disabled` field is dead code (cockpit.rs:62)
- [ ] **REVIEW FIX NEEDED**: `egui::Frame::none()` deprecated in tactical_view.rs:1046
- [ ] **REVIEW FIX NEEDED**: TroopClassDef fallback (10/10) silently masks missing TROOPSD.DAT — add warning log
- [ ] HD upscaling: separate ExecPlan at `docs/plans/2026-03-25-003-upscale-all-dll-bmps-execplan.md` (PBRify + Vertex + Gemini, NOT waifu2x for game textures)

### Wave 4: Advisor + Polish + Validation (~1,500 LOC)
After review fixes resolved.
- BIN animation decoder for advisor (replace frame-cycling fallback)
- GOKRES diagram integration (reactor layouts, weapon arcs, damage views)
- Dead code cleanup + doc refresh
- Integration testing + autoresearch validation (20-iteration campaign balance check)

## Dependency Chain
```
Wave 1 DONE + Wave 3 DONE → Review Fixes → Wave 2 → Wave 4
```

## Review Findings Register (from Knesset Ma'at reviewers)

| # | Severity | File | Finding | Status |
|---|----------|------|---------|--------|
| 1 | BUG | combat.rs:391-405 | Shield `raw_consumed` double-rounding — hull overflow under-damages | TODO |
| 2 | MEDIUM | combat.rs:657-663 | `recall_fighters` overwrites unlaunched squads to 0 — silent fighter loss | TODO |
| 3 | MEDIUM | simulation.rs:305 | Hardcoded `difficulty: 2` for ground combat — ignores actual game difficulty | TODO |
| 4 | MEDIUM | combat.rs:848-852 | TroopClassDef fallback (10/10) silently masks missing TROOPSD.DAT | TODO |
| 5 | HIGH | main.rs:406-476 | Event screen keyboard blocking — FIXED (added `!event_screen_state.is_active()` guard) | FIXED |
| 6 | CONCERN | tactical_view.rs:603 | Fighter damage bypasses shields in tactical view (inconsistent with auto-resolve) | TODO |
| 7 | CONCERN | tactical_view.rs:547 | Weapon type from hull_max bracket, not actual stats (cosmetic only) | TODO |
| 8 | MINOR | cockpit.rs:62 | `ButtonSprite::disabled` dead code | TODO |
| 9 | MINOR | tactical_view.rs:1046 | `Frame::none()` deprecated → use `Frame::NONE` | TODO |
| 10 | CONCERN | combat.rs:855 | `atk_class_defense` fetched and discarded in O(n²) inner loop | TODO |

## Gap Analysis Sources
- `.subdaimon-output/core-parity-gaps-complete-audit-2026-03-25.md`
- `.subdaimon-output/combat-parity-gap-analysis.md`
- `.subdaimon-output/UI_PARITY_GAP_ANALYSIS.md`
