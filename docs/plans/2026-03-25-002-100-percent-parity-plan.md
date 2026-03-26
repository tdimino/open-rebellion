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
| Combat | 99% | 100% | ~200 (review fixes DONE) |
| UI | 97% | 100% | ~800 (Wave 4 remaining) + upscaling |
| **Total** | ~97% | 100% | **~1,600 LOC** |

## 4 Waves (3 sequential Knessets + 1 background batch)

### Wave 1: Combat Formula Parity — DONE (Knesset Ma'at, 2026-03-25)
- [x] Shield absorption phase (Phase 4) — pending damage pipeline, ion 2x, recharge (+8 tests)
- [x] Fighter combat — launch/dogfight/recall from carriers, per-weapon stats (+6 tests)
- [x] Ground combat — per-unit resolution with TroopClassDef + facility bonus (+6 tests)
- [x] Difficulty modifiers wired into space + ground combat
- [x] **REVIEW FIX**: Shield absorption `raw_consumed` double-rounding — replaced f64 double-truncation with single i64 integer division
- [x] **REVIEW FIX**: `recall_fighters` overwrites unlaunched (grounded) squadrons to 0 — added `originally_launched` parameter to preserve grounded squads
- [x] **REVIEW FIX**: Hardcoded `difficulty: 2` — added `difficulty_index: u8` to GameWorld, threaded through all combat/bombardment calls
- [ ] Wire 74 unused GNPRTB combat parameters (deferred — most are informational)
- [x] **REVIEW FIX**: Tactical view fighter damage bypasses shields — added shield absorption consistent with auto-resolve
- [x] **REVIEW FIX**: Tactical view weapon type from hull_max bracket — now uses actual turbolaser/ion/laser stats from CapitalShipClass

### Wave 2: AI + Core Gaps (~800 LOC) — NEXT
- 14 missing dispatch validators (FUN_00508250)
- Troop deployment AI (AIAction::MoveTroops)
- Faction deployment budgets + Death Star AI
- Mission/event/character edge cases

### Wave 3: BMP Integration + Upscaling — REVIEW FIXES DONE (Knesset Ma'at)
- [x] Cockpit button sprites (9 buttons, 3-state from COMMON.DLL)
- [x] Event screen overlay (61 STRATEGY.DLL BMPs, timer + click-dismiss)
- [x] Tactical HUD panels + weapon gauge + hull/shield display (TACTICAL.DLL)
- [x] GOKRES mini-icons in fleet panel detail view
- [x] Event screen keyboard blocking — FIXED (added `!event_screen_state.is_active()` guard)
- [x] **REVIEW FIX**: `ButtonSprite::disabled` dead code — removed field
- [x] **REVIEW FIX**: `egui::Frame::none()` deprecated — replaced with `Frame::NONE`
- [x] **REVIEW FIX**: TroopClassDef fallback (10/10) — added `eprintln!` warning when TROOPSD.DAT class missing
- [ ] HD upscaling: separate ExecPlan at `docs/plans/2026-03-25-003-upscale-all-dll-bmps-execplan.md` (PBRify + Vertex + Gemini, NOT waifu2x for game textures)

### Wave 4: Advisor + Polish + Validation (~1,500 LOC)
After review fixes resolved.
- BIN animation decoder for advisor (replace frame-cycling fallback)
- GOKRES diagram integration (reactor layouts, weapon arcs, damage views)
- Dead code cleanup + doc refresh
- Integration testing + autoresearch validation (20-iteration campaign balance check)

## Dependency Chain
```
Wave 1 DONE + Wave 3 DONE → Review Fixes DONE → Wave 2 → Wave 4
```

## Review Findings Register (from Knesset Ma'at reviewers)

| # | Severity | File | Finding | Status |
|---|----------|------|---------|--------|
| 1 | BUG | combat.rs:388-405 | Shield `raw_consumed` double-rounding — replaced with i64 integer division | FIXED |
| 2 | MEDIUM | combat.rs:652-680 | `recall_fighters` overwrites unlaunched squads — added `originally_launched` param | FIXED |
| 3 | MEDIUM | simulation.rs + main.rs | Hardcoded `difficulty: 2` — added `GameWorld.difficulty_index`, threaded everywhere | FIXED |
| 4 | MEDIUM | combat.rs:853-860 | TroopClassDef fallback silently masks missing TROOPSD.DAT — added eprintln! warning | FIXED |
| 5 | HIGH | main.rs:406-476 | Event screen keyboard blocking — added `!event_screen_state.is_active()` guard | FIXED |
| 6 | CONCERN | tactical_view.rs:596-630 | Fighter damage bypasses shields — added shield absorption to tactical fighters | FIXED |
| 7 | CONCERN | tactical_view.rs:547-562 | Weapon type from hull_max bracket — now uses actual weapon stats from class data | FIXED |
| 8 | MINOR | cockpit.rs:56-63 | `ButtonSprite::disabled` dead code — removed field | FIXED |
| 9 | MINOR | tactical_view.rs:1046 | `Frame::none()` deprecated → `Frame::NONE` | FIXED |
| 10 | CONCERN | combat.rs:855 | `atk_class_defense` fetched and discarded in O(n²) loop — removed | FIXED |

## Gap Analysis Sources
- `.subdaimon-output/core-parity-gaps-complete-audit-2026-03-25.md`
- `.subdaimon-output/combat-parity-gap-analysis.md`
- `.subdaimon-output/UI_PARITY_GAP_ANALYSIS.md`
