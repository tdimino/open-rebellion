# Jedi Training System

`jedi.rs` — 4-tier Force progression with accumulated XP and detection checks.

## Types

| Type | Purpose |
|------|---------|
| `JediState` | `training: Vec<JediTrainingRecord>` |
| `JediTrainingRecord` | `{ character, faction_is_alliance, accumulated_xp, started_tick, last_detection_tick }` |
| `JediEvent` | TierAdvanced / TrainingComplete / JediDiscovered |
| `ForceTier` | None → Aware → Training → Experienced |

## API

```rust
// Each tick: advance all trainees
let events = JediSystem::advance(&mut jedi_state, &world, &tick_events, &rng_rolls);

// Game start: roll for Force awakening
let awakened = JediSystem::apply_initial_awakening(&world, &rng_rolls);

// Enroll/unenroll:
state.start_training(character, faction_is_alliance, current_tick);
state.stop_training(character);
```

## Force Tiers and XP Thresholds

| Tier | XP Threshold | Meaning |
|------|-------------|---------|
| `None` | 0 | No Force ability |
| `Aware` | > 0 | Potential discovered |
| `Training` | ≥ 50 (`XP_TO_TRAINING`) | Active training underway |
| `Experienced` | ≥ 150 (`XP_TO_EXPERIENCED`) | Full Jedi Knight / Sith Lord |

## XP Persistence

XP accumulates in `JediTrainingRecord.accumulated_xp` (stored in `JediState`, serialized in `SaveState`). This avoids the caller needing to write `force_experience` back to `world.characters` every tick. Total XP = `character.force_experience` (base) + `record.accumulated_xp`.

When a tier advances, the caller sets `character.force_tier` and `character.force_experience` to the threshold value.

## Detection

Every `DETECTION_CHECK_INTERVAL = 30` ticks, the opposing faction may detect a Jedi. Probability scales with tier:
- Aware: 5%
- Training: 15%
- Experienced: 30%

Already-discovered Jedi (`is_discovered_jedi = true`) are never re-detected.

## Initial Awakening

At game start, characters with `jedi_probability > 0` roll against that probability. Those that pass get `force_tier = Aware`. Characters already awakened (e.g. Luke) are skipped.

## Source

- `entity-system.md §1.3` — `entity[9] >> 6 & 3` tier encoding
- REBEXE.EXE RE — Force tier bit layout
