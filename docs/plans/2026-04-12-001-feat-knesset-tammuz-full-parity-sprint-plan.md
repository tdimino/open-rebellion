---
title: "Knesset Tammuz: Full Parity Sprint"
type: feat
status: completed
date: 2026-04-12
origin: docs/plans/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md
---

# Knesset Tammuz: Full Parity Sprint

*Named for **Tammuz** (תמוז), the dying-and-rising god of Mesopotamia — the final sprint that brings the game from ~98% to 100% feature parity with the original Star Wars Rebellion (1998), then ships it.*

## Overview

Complete every remaining parity gap in Open Rebellion except HD visual upscaling. This sprint consolidates the deferred work from Knesset Shamash-Bet (daborot 4–6), the mission/betrayal/death-star telemetry emissions (R6–R8, R11), the Emperor Palpatine combat modifier (R12, deferred from Shamash-Bet to Knesset Nergal), the 8 unwired story cutscene triggers (101–108), the AI dispatch validators, troop deployment AI, and release packaging.

**Current state**: v0.21.0, 447 tests, ~98% parity. 15 simulation systems, 4 story chains, 5-case palace outcomes, carbonite countdown, heritage gate, SpawnSpecialForce, betrayal telemetry. Shamash-Bet Daborot 1–3 DONE.

**Target state**: v1.0.0, ~500+ tests, 100% feature parity (excluding HD graphics). Every original game mechanic implemented, every telemetry event wired, every cutscene triggered, AI dispatch fully ported, release artifacts published.

## Problem Statement

13 discrete parity gaps remain scattered across 6 subsystems. Each is individually small (50–500 LOC), but collectively they prevent declaring 1.0. The gaps cluster into 5 natural phases that match the existing Shamash-Bet dabora structure plus two new phases for AI and release.

## Technical Approach

### Phase 1: Shamash-Bet Dabora 4 — Victory + Integrator + Golden Oracle

**Files**: `victory.rs`, `integrator.rs`, `golden_values.json`, `eval_parity.py`
**Estimated**: ~350 LOC + JSON

| Task | What | File |
|------|------|------|
| A1 | `EVT_HQ_CAPTURED` (0x128) emission from `apply_victory` before victory modal | `integrator.rs` |
| A2 | Integrator arm for `SpecialForceSpawned` — create `SpecialForceUnit` in `world.special_forces`, push key onto `system.special_forces`, in-transit fallback via `MovementOrder` destination | `integrator.rs` |
| A3 | Integrator arm for `CharacterFlagSet` — mutate named flag on named character | `integrator.rs` |
| A4 | Integrator arm for `SystemIncidentFired` — clear incident bit after emission | `integrator.rs` |
| A5 | `story_events` section in `golden_values.json` — baseline from `--seed 42 --ticks 5000 --jsonl`, lock fire counts (±10% tolerance) for 14 event types | `golden_values.json` |
| A6 | `eval_parity.py` extension to load story_events section and report pass/fail | `eval_parity.py` |

**Verification**: `cargo test --workspace` + `eval_parity.py` with new story section passes.

### Phase 2: Mission + Death Star Telemetry (Shamash-Bet R6–R8, R11)

**Files**: `missions.rs`, `death_star.rs`, `integrator.rs`
**Estimated**: ~200 LOC

| Task | What | File |
|------|------|------|
| R6 | `EVT_INFORMANT_INTEL` (0x153) emission when Espionage mission resolves with non-empty intel | `missions.rs` or `integrator.rs` |
| R7 | `EVT_SABOTEUR_DETECTED` (0x305) emission when enemy Sabotage resolves on player system | `missions.rs` or `integrator.rs` |
| R8 | `EVT_CHARACTER_HEALTH` (0x106) emission on `MissionEffect::CharacterInjured` | `missions.rs` or `integrator.rs` |
| R11 | `EVT_CHARACTER_KILLED` (0x306) emission from `cleanup_destroyed_system()` (gain `&mut Vec<GameEffect>` out-parameter) AND from assassination mission resolution. Payload uses `dat_id`/`name` (DI-H2). Uniqueness via `Character::is_killed` flag (DI-M3). | `death_star.rs`, `missions.rs`, `integrator.rs` |

**Constraint**: Character-death story events are STRICTLY next-tick reactive (Events=step 6, Death Star cleanup=step 11). Document this in tests.

### Phase 3: Emperor Palpatine Combat Check (formerly R12 / Knesset Nergal)

**Files**: `combat.rs`
**Estimated**: ~60 LOC

| Task | What | File |
|------|------|------|
| R12 | `FUN_00542050_is_emperor_palpatine` combat-winner modifier. Add `winner_resolution_emperor_modifier()` helper to `phase_who_won`. When Emperor Palpatine is co-located with an engagement, apply a multiplier (likely 1.5× based on Ghidra cross-reference patterns) to his faction's combat score. | `combat.rs` |

**Research needed**: Cross-reference `FUN_00542050` in decompiled C files (`ghidra/notes/`) to confirm the exact modifier value and application point. If the function is a simple identity check (`return character.is_emperor`), the modifier value comes from a GNPRTB parameter — check indices 0x1500-0x15FF.

### Phase 4: Story Cutscene Triggers (101–108)

**Files**: `video_player.rs`, `main.rs`, `story_events.rs`, `event_screen.rs`
**Estimated**: ~400 LOC

This subsumes the Shamash-Bet Dabora 5 cutscene state machine work AND the 8 unwired story cutscenes.

| Task | What | File |
|------|------|------|
| V1 | `VictoryScreenState` wired into `main.rs` — populate on victory detection, draw in `GameMode::VictoryModal` | `main.rs` |
| V2 | `GameMode::VictoryModal { outcome }` variant — egui modal over frozen galaxy, ESC/"Continue" transitions to Cutscene | `main.rs` |
| V3 | `GameMode::Cutscene { kind: CutsceneKind }` — extend existing unit variant to struct variant with `Intro`/`Victory`/`Defeat`/`Story(u32)` kinds | `main.rs` |
| V4 | Post-cutscene transitions: `Intro → MainMenu`, `Victory/Defeat → MainMenu`, `Story(n) → Galaxy` (resume gameplay) | `main.rs` |
| V5 | Save lock-out during `Cutscene` and `VictoryModal` modes | `main.rs` |
| V6–V9 | `VideoPlayer` volume routing + graceful WAV-missing fallback | `video_player.rs` |
| C1–C8 | Wire story events to cutscene triggers. Map each of the 8 story cutscenes (101–108) to their triggering event IDs. When a story event fires that maps to a cutscene file, transition to `GameMode::Cutscene { kind: Story(N) }` | `main.rs`, `story_events.rs` |

**Cutscene mapping** (requires research — cross-reference original game event handler dispatch tables):

| File | Likely Trigger | Content (from original game manual + video inspection) |
|------|---------------|-------|
| 101.webm | `EVT_LUKE_DAGOBAH` (0x221) | Luke departs for Dagobah |
| 102.webm | `EVT_DAGOBAH_COMPLETED` (0x210) | Luke completes training |
| 103.webm | `EVT_BOUNTY_ATTACK` (0x212) | Han captured by bounty hunters |
| 104.webm | `0x383` Han rescued | Palace rescue success |
| 105.webm | `EVT_FINAL_BATTLE` (0x220) | Luke vs Vader final confrontation |
| 106.webm | `0x393` Vader dispatched | Vader sets out to confront Luke |
| 107.webm | `0x396` Final Battle Imminent | Father and son confrontation |
| 108.webm | `0x397` Bounty Hunters Active | Empire dispatches bounty hunters |

**Note**: This mapping is preliminary. Confirm by inspecting the original REBEXE.EXE event handler dispatch table in Ghidra decompilation or by playing the original game with event logging enabled.

### Phase 5: Advisor BIN Decoder (Shamash-Bet Dabora 6)

**Files**: `advisor.rs`, research scripts
**Estimated**: ~300 LOC + research artifacts

| Task | What | File |
|------|------|------|
| N1–N4 | Research phase: hex corpus extraction, variant signature discovery, BIN→BMP offset discovery, research artifact | Scripts + `agent_docs/research/` |
| N5 | `parse_advisor_bin_v2` — cascading decoder for undocumented BIN header variants. Target ≥50% parse rate. | `advisor.rs` |
| N6 | `ADVISOR_BIN_BMP_OFFSET` const replacing modulo shortcut | `advisor.rs` |
| N7 | Per-faction logging update (`valid_v1`, `valid_v2`, `parse-failed`, `mapped`, `unmapped`) | `advisor.rs` |
| N8 | Tests for v2 decoder + BMP offset lookup + fallback | `advisor.rs` |

### Phase 6: AI Parity Closeout

**Files**: `ai.rs`, `tuning.rs`
**Estimated**: ~500 LOC

| Task | What | Status |
|------|------|--------|
| D1 | Port remaining 8 dispatch validators (FUN_0050ad60, 0050ad80, 0050b0b0, 0050b310, 0050b610, 0050c350 + strengthen 3 PARTIAL gates) | 5 MISSING + 5 PARTIAL → all DONE |
| D2 | `AIAction::MoveTroops` — AI troop deployment. Ground force production priority + transport to frontline systems. | MISSING → DONE |
| D3 | Death Star multi-target selection — escort, retreat, target prioritization beyond "go to enemy HQ" | Basic → Full |
| D4 | AI resource rebalancing (`FUN_00558660`) | Not yet examined |
| D5 | Reconnaissance mission variant (0x54) | Not yet implemented |

**Approach**: D1 is mechanical porting from Ghidra decompilation. D2 is the largest — analyze the original troop deployment logic in `FUN_00508660` handlers for family bytes 0x10-0x13 and port the loop+scoring pattern. D3–D5 are smaller additions.

### Phase 7: Release Packaging

**Files**: CI config, scripts, packaging
**Estimated**: ~200 LOC

| Task | What |
|------|------|
| P1 | itch.io web release — automated WASM build + butler upload |
| P2 | macOS release — `.app` bundle via `cargo-bundle` or manual packaging |
| P3 | GitHub Releases — CI artifact publishing with changelog |
| P4 | Code hygiene — 27 LOC dead code, 5 stale doc comments, test boilerplate reduction |

## Merge Order

```
Phase 1 (Victory+Golden) ──┬─→ Phase 2 (Mission telemetry)
                           ├─→ Phase 3 (Emperor combat)
                           └─→ Phase 4 (Cutscenes)
                                  └─→ Phase 5 (Advisor BIN)

Phase 6 (AI parity) — independent, can run parallel to Phases 1–5

Phase 7 (Release) — after all other phases
```

Phases 2, 3, and 4 can run in parallel after Phase 1 lands (Phase 1 establishes the golden oracle baseline). Phase 5 gates on Phase 4 (both touch `main.rs` state machine). Phase 6 is independent of all story/cutscene work. Phase 7 is the final gate.

## Acceptance Criteria

### Functional Requirements

- [ ] All 14 golden-value event types pass `eval_parity.py` with ±10% tolerance
- [ ] 8 story cutscenes (101–108) play when their corresponding events fire
- [ ] VictoryModal → Cutscene → MainMenu transition chain works
- [ ] Emperor Palpatine combat modifier applies when co-located
- [ ] All 18 AI dispatch validators DONE or FAITHFUL
- [ ] AI deploys troops to frontline systems
- [ ] Death Star AI uses escort, retreat, multi-target selection
- [ ] `EVT_CHARACTER_KILLED` fires from both Death Star cleanup and assassination missions
- [ ] `EVT_INFORMANT_INTEL`, `EVT_SABOTEUR_DETECTED`, `EVT_CHARACTER_HEALTH` fire from missions

### Quality Gates

- [ ] All tests pass (target: 500+)
- [ ] Zero warnings
- [ ] `eval_parity.py` full pass (all sections)
- [ ] Save/load round-trip works across all new features (save v8 compatibility)
- [ ] WASM build compiles and runs (cutscenes gracefully skip)

## Dependencies & Risks

| Risk | Mitigation |
|------|-----------|
| Cutscene→event mapping unknown | Cross-reference Ghidra decompilation; worst case play original game to verify |
| Advisor BIN format intractable | Phase 5 research may yield <50% parse rate; accept graceful fallback (legacy cycling) |
| AI troop deployment is complex | Start with simplified scoring; augment if campaigns are degenerate |
| Emperor combat modifier value unknown | Default to 1.5×; validate via campaign sim quality scores |

## Scope Boundaries

**In scope**: Everything listed above.

**Out of scope** (deferred):
- HD visual upscaling (separate pipeline, not parity)
- Per-character `MissionKind::PalaceRescue` mission types (gameplay enhancement, not parity)
- Multiplayer / 178 `net_notify_*` message types (Knesset Marduk)
- Addon 1: Create Your Own Character (Demiurgos)
- Advisor BIN Ghidra RE session (only if Phase 5 hex research fails)

## Sources & References

### Origin

- **Origin document:** [docs/plans/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md](docs/plans/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md) — Phases 4–6 carried forward, R6–R8/R11/R12 unshipped tasks, cutscene wiring from Alternative 4 (now in scope).

### Internal References

- AI parity tracker: `agent_docs/systems/ai-parity-tracker.md`
- Community disassembly cross-reference: `docs/reports/2026-03-26-community-disassembly-cross-reference.md`
- Game media inventory: `agent_docs/game-media.md:60-86`
- Ghidra RE notes: `ghidra/notes/` (7 scholar docs, 5,127 decompiled functions)
- Decoded cutscenes: `assets/references/ref-videos/` (15 WebM files including 101–108)
- Story events system: `agent_docs/systems/story-events.md`
- Roadmap: `agent_docs/roadmap.md`
