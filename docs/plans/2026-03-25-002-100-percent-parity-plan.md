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
| Core | 97% | 100% | ~1,400 |
| Combat | 90% | 100% | ~1,330 |
| UI | 95% | 100% | ~1,000 (code) + batch upscaling |
| **Total** | ~93% | 100% | **~3,700 LOC** |

## 4 Waves (3 sequential Knessets + 1 background batch)

### Wave 1: Combat Formula Parity (5 agents, ~600 LOC)
No Ghidra needed — decompiled C already exists.
- Shield absorption phase (FUN_00544130)
- Weapon fire per-arc (FUN_00543b60, 12 arc fields already promoted)
- Fighter combat subsystem (launch/recall/engagement/losses)
- Ground combat formula (FUN_00560d50) + difficulty modifiers
- Wire 74 unused GNPRTB combat parameters

### Wave 2: AI + Core Gaps (4 agents, ~800 LOC)
Depends on Wave 1 combat formulas.
- 14 missing dispatch validators (FUN_00508250)
- Troop deployment AI (AIAction::MoveTroops)
- Faction deployment budgets + Death Star AI
- Mission/event/character edge cases

### Wave 3: BMP Integration + Upscaling (4 agents + background, ~800 LOC)
Fully parallel with Waves 1-2.
- Cockpit button sprites (215 from COMMON.DLL)
- Event screen wiring (73 from STRATEGY.DLL)
- Tactical HUD elements (154 from TACTICAL.DLL)
- Ship status views + mini-icons (GOKRES.DLL)
- Background: waifu2x batch upscale all 2,231 BMPs to data/hd/

### Wave 4: Advisor + Polish + Validation (4 agents, ~1,500 LOC)
After Waves 1-3.
- BIN animation decoder for advisor
- GOKRES diagram integration
- Dead code cleanup + doc refresh
- Integration testing + autoresearch validation

## Dependency Chain
```
Wave 1 + Wave 3 (parallel) → Wave 2 → Wave 4
```

## Gap Analysis Sources
- `.subdaimon-output/core-parity-gaps-complete-audit-2026-03-25.md`
- `.subdaimon-output/combat-parity-gap-analysis.md`
- `.subdaimon-output/UI_PARITY_GAP_ANALYSIS.md`
