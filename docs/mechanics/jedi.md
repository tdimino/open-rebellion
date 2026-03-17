---
title: "Jedi & Force Training"
description: "4-tier Force progression, XP thresholds, detection checks, initial awakening"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "jedi"
sources:
  - type: "ghidra"
    file: "ghidra/notes/entity-system.md"
  - type: "code"
    file: "crates/rebellion-core/src/jedi.rs"
related:
  - "characters"
  - "story-events"
tags: ["jedi", "force", "training", "detection"]
---

# Jedi & Force Training

Characters with Force potential can progress through four tiers of Force sensitivity, unlocking combat bonuses and triggering story events.

## What You See

- A character with `jedi_probability > 0` may awaken as Force-sensitive at game start.
- Once Aware, the character can be placed in Jedi training.
- Training accumulates XP each tick until the character reaches Experienced (full Jedi Knight / Sith Lord).
- The opposing faction can detect your Jedi, escalating the danger.

## How It Works

### Force Tiers

The 2-bit field at `entity[9] >> 6 & 3` encodes four tiers:

| Tier | Value | Meaning |
|------|-------|---------|
| None | 0 | No Force ability detected |
| Aware | 1 | Potential discovered, untrained |
| Training | 2 | Actively training in the Force |
| Experienced | 3 | Full Jedi Knight / Sith Lord |

### Initial Awakening

At game start, `JediSystem::apply_initial_awakening` rolls each character with `jedi_probability > 0` against that probability. Characters that pass have `force_tier` set to `Aware`. Characters already marked as Force users (e.g., Luke starts Aware, Vader starts Experienced) skip the roll.

### XP Accumulation

While in training (`JediState::start_training`), `force_experience` increments by 1 each tick. The `JediTrainingRecord` stores `accumulated_xp` persistently so XP is not lost between ticks.

| Threshold | XP Required | Tier Reached |
|-----------|-------------|--------------|
| Aware → Training | 50 | Training |
| Training → Experienced | 150 | Experienced |

When `force_experience` crosses a threshold, `JediEvent::TierAdvanced` fires. At Experienced, `JediEvent::TrainingComplete` fires and the character is removed from active training.

### Detection

Every `DETECTION_CHECK_INTERVAL` (30) ticks, the opposing faction rolls to detect each active Jedi. Detection probability scales with tier:

| Tier | Detection Probability |
|------|-----------------------|
| None | 0% |
| Aware | 5% |
| Training | 15% |
| Experienced | 30% |

On success, `JediEvent::JediDiscovered` fires and `is_discovered_jedi` is set. Once discovered, a Jedi is never re-checked (monotonic). The binary formula (entity-system.md §1.3) uses `FUN_0058a3f0` with three sub-conditions all required.

### Key Fields

| Field | Source | Notes |
|-------|--------|-------|
| `force_tier` | `entity[9] >> 6 & 3` | Current tier |
| `force_experience` | `entity[0x23]` (short) | Accumulated XP |
| `is_discovered_jedi` | `!(entity[0x1e] & 1)` | Initially hidden |
| `is_jedi_trainer` | `MJCHARSD.DAT` | Yoda: can train others |
| `is_known_jedi` | `MJCHARSD.DAT` | Publicly known Force user |
| `jedi_probability` | `MJCHARSD.DAT` | Awakening roll threshold (0--100) |

## Source Material

- **Ghidra RE**: `ghidra/notes/entity-system.md` §1.3 (Force/Jedi System) -- field offsets, detection formula, notification dispatchers
- **Code**: `crates/rebellion-core/src/jedi.rs` -- `JediSystem`, `JediState`, `JediTrainingRecord`
- **Event IDs**: `0x1e1` (CharacterForce), `0x1e5` (CharacterForceTraining), `0x362` (ForceUserDiscovered)

## Related

- [characters.md](characters.md) -- Character skills and Force potential fields
- [story-events.md](story-events.md) -- Luke Dagobah training chain, Force milestone events
