//! Jedi / Force training system.
//!
//! # Design
//!
//! Follows the stateless advance() pattern:
//! - `JediState` tracks which characters are in Force training and their experience.
//! - `JediSystem::advance(state, world, tick_events, rng_rolls) -> Vec<JediEvent>`
//! - Caller applies results to `world.characters` and logs messages.
//!
//! # Force Tiers
//!
//! From `entity-system.md` §1.3 and `REBEXE.EXE` RE:
//!
//! ```text
//! entity[9] >> 6 & 3:
//!   0 = ForceTier::None     — no Force ability
//!   1 = ForceTier::Aware    — potential discovered, untrained
//!   2 = ForceTier::Training — active training underway
//!   3 = ForceTier::Experienced — full Jedi Knight / Sith Lord
//! ```
//!
//! # Lifecycle
//!
//! 1. At game start, characters with `jedi_probability > 0` roll against that probability.
//!    Those that succeed have `force_tier` set to `Aware` by `JediSystem::apply_initial_awakening`.
//! 2. A Force-aware character can be placed in Jedi training (`JediState::start_training`).
//! 3. Each tick while in training, `force_experience` increments by 1.
//! 4. When `force_experience` crosses a tier threshold, a `JediEvent::TierAdvanced` fires.
//! 5. Once `Experienced`, training is complete — the character is removed from active training.
//! 6. Once per `DETECTION_CHECK_INTERVAL` ticks, the opposing faction may detect a Jedi
//!    (`JediEvent::JediDiscovered`). Detection probability scales with force_tier.
//!
//! # RNG Contract
//!
//! The caller provides a pre-generated `rng_rolls: &[f64]` slice (one roll per active
//! trainee per tick, plus one detection roll per trainee). Rolls are consumed in order.
//! Determinism is guaranteed — same inputs produce same outputs.

use serde::{Deserialize, Serialize};

use crate::ids::CharacterKey;
use crate::tick::TickEvent;
use crate::world::{ForceTier, GameWorld};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Experience points required to advance from Aware → Training.
pub const XP_TO_TRAINING: u32 = 50;

/// Experience points required to advance from Training → Experienced.
pub const XP_TO_EXPERIENCED: u32 = 150;

/// How often (in ticks) the detection check runs for each active Jedi.
pub const DETECTION_CHECK_INTERVAL: u64 = 30;

/// Base detection probability per detection check for an Aware Jedi (0.0–1.0).
pub const DETECT_PROB_AWARE: f64 = 0.05;

/// Detection probability for a Training-tier Jedi.
pub const DETECT_PROB_TRAINING: f64 = 0.15;

/// Detection probability for an Experienced Jedi.
pub const DETECT_PROB_EXPERIENCED: f64 = 0.30;

// ---------------------------------------------------------------------------
// JediTrainingRecord
// ---------------------------------------------------------------------------

/// One character currently undergoing Force training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JediTrainingRecord {
    pub character: CharacterKey,
    /// True = Rebel Alliance faction; false = Empire.
    pub faction_is_alliance: bool,
    /// Tick on which training started (used to gate detection checks).
    pub started_tick: u64,
    /// Tick of the last detection check (prevents double-triggering).
    pub last_detection_tick: u64,
    /// Accumulated XP since training began. Stored here (not in world)
    /// so XP persists across ticks without requiring the caller to write
    /// force_experience back to the character every tick.
    pub accumulated_xp: u32,
}

// ---------------------------------------------------------------------------
// JediState
// ---------------------------------------------------------------------------

/// Persistent Jedi/Force training state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JediState {
    /// Characters currently in active Force training.
    pub training: Vec<JediTrainingRecord>,
}

impl JediState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enroll a character in Force training.
    ///
    /// Characters already in training are not re-enrolled (idempotent).
    /// The character's `force_tier` must be ≥ `Aware` before calling this.
    pub fn start_training(&mut self, character: CharacterKey, faction_is_alliance: bool, current_tick: u64) {
        if self.training.iter().any(|r| r.character == character) {
            return;
        }
        self.training.push(JediTrainingRecord {
            character,
            faction_is_alliance,
            started_tick: current_tick,
            last_detection_tick: current_tick,
            accumulated_xp: 0,
        });
    }

    /// Remove a character from training (e.g. they reached Experienced, died, or were captured).
    pub fn stop_training(&mut self, character: CharacterKey) {
        self.training.retain(|r| r.character != character);
    }

    /// True if the character is currently in training.
    pub fn is_training(&self, character: CharacterKey) -> bool {
        self.training.iter().any(|r| r.character == character)
    }
}

// ---------------------------------------------------------------------------
// JediEvent
// ---------------------------------------------------------------------------

/// An event emitted by `JediSystem::advance`. The caller applies the effect.
#[derive(Debug, Clone, PartialEq)]
pub enum JediEvent {
    /// A character's Force tier advanced.
    TierAdvanced {
        character: CharacterKey,
        /// The new tier after advancement.
        new_tier: ForceTier,
    },
    /// Training is complete — character has reached `Experienced`.
    TrainingComplete {
        character: CharacterKey,
    },
    /// The opposing faction detected this character's Force ability.
    JediDiscovered {
        character: CharacterKey,
        /// Faction that made the discovery (the OTHER side from the Jedi's faction).
        discovered_by_alliance: bool,
    },
}

// ---------------------------------------------------------------------------
// JediSystem
// ---------------------------------------------------------------------------

/// Stateless Jedi/Force training system.
pub struct JediSystem;

impl JediSystem {
    /// Advance all active Force training sessions by one tick per `TickEvent`.
    ///
    /// Consumes `rng_rolls` for detection checks in order:
    /// - One roll per trainee per `DETECTION_CHECK_INTERVAL` ticks.
    ///
    /// Returns `JediEvent`s for tier advancements, training completions, and detections.
    /// The caller must:
    /// 1. Update `world.characters[key].force_experience` from `TierAdvanced` events.
    /// 2. Update `world.characters[key].force_tier` from `TierAdvanced` events.
    /// 3. Update `world.characters[key].is_discovered_jedi` from `JediDiscovered` events.
    /// 4. Call `state.stop_training(key)` for each `TrainingComplete` event.
    pub fn advance(
        state: &mut JediState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        rng_rolls: &[f64],
    ) -> Vec<JediEvent> {
        if tick_events.is_empty() || state.training.is_empty() {
            return Vec::new();
        }

        let ticks_elapsed = tick_events.len() as u32;
        let current_tick = tick_events.last().unwrap().tick;
        let mut events = Vec::new();
        let mut roll_idx = 0;

        // Collect indices of records that completed training (to remove after iteration).
        let mut completed: Vec<CharacterKey> = Vec::new();

        for record in &mut state.training {
            let Some(character) = world.characters.get(record.character) else {
                // Character was removed from the world (killed/captured).
                completed.push(record.character);
                continue;
            };

            // ── Experience accumulation ───────────────────────────────────────
            // Use accumulated_xp from the training record (persists across ticks)
            // plus the character's base force_experience from game start.
            let base_xp = character.force_experience;
            record.accumulated_xp += ticks_elapsed;
            let new_xp = base_xp + record.accumulated_xp;

            // ── Tier advancement check ────────────────────────────────────────
            let old_tier = character.force_tier;
            let new_tier = Self::tier_for_xp(new_xp);

            if new_tier > old_tier {
                events.push(JediEvent::TierAdvanced {
                    character: record.character,
                    new_tier,
                });

                if new_tier == ForceTier::Experienced {
                    events.push(JediEvent::TrainingComplete {
                        character: record.character,
                    });
                    completed.push(record.character);
                }
            }

            // ── Detection check (every DETECTION_CHECK_INTERVAL ticks) ───────
            if current_tick.saturating_sub(record.last_detection_tick) >= DETECTION_CHECK_INTERVAL
                && !character.is_discovered_jedi
            {
                let detect_prob = Self::detection_probability(old_tier.max(new_tier));
                let roll = if roll_idx < rng_rolls.len() {
                    let r = rng_rolls[roll_idx];
                    roll_idx += 1;
                    r
                } else {
                    0.5 // safe fallback if caller under-supplies rolls
                };

                if roll < detect_prob {
                    events.push(JediEvent::JediDiscovered {
                        character: record.character,
                        // The opposing faction makes the discovery.
                        discovered_by_alliance: !record.faction_is_alliance,
                    });
                }

                record.last_detection_tick = current_tick;
            }
        }

        // Remove completed trainees.
        for key in completed {
            state.stop_training(key);
        }

        events
    }

    /// Apply initial Force awakenings at game start.
    ///
    /// For each character with `jedi_probability > 0`, rolls against that probability
    /// using the provided rng rolls. Characters that pass have their `force_tier` set
    /// to `Aware`. Returns the keys of newly awakened characters.
    ///
    /// Caller must update `world.characters[key].force_tier = ForceTier::Aware`
    /// for each returned key.
    pub fn apply_initial_awakening(
        world: &GameWorld,
        rng_rolls: &[f64],
    ) -> Vec<CharacterKey> {
        let mut awakened = Vec::new();
        let mut roll_idx = 0;

        for (key, character) in world.characters.iter() {
            if character.jedi_probability == 0 {
                continue;
            }
            if character.force_tier != ForceTier::None {
                continue; // already awakened (e.g. Luke)
            }

            let threshold = character.jedi_probability as f64 / 100.0;
            let roll = if roll_idx < rng_rolls.len() {
                let r = rng_rolls[roll_idx];
                roll_idx += 1;
                r
            } else {
                0.5
            };

            if roll < threshold {
                awakened.push(key);
            }
        }

        awakened
    }

    /// Determine the Force tier for a given experience value.
    fn tier_for_xp(xp: u32) -> ForceTier {
        if xp >= XP_TO_EXPERIENCED {
            ForceTier::Experienced
        } else if xp >= XP_TO_TRAINING {
            ForceTier::Training
        } else if xp > 0 {
            ForceTier::Aware
        } else {
            ForceTier::None
        }
    }

    /// Detection probability per check interval for a given tier.
    fn detection_probability(tier: ForceTier) -> f64 {
        match tier {
            ForceTier::None       => 0.0,
            ForceTier::Aware      => DETECT_PROB_AWARE,
            ForceTier::Training   => DETECT_PROB_TRAINING,
            ForceTier::Experienced => DETECT_PROB_EXPERIENCED,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::DatId;
    use crate::world::{Character, ForceTier, GameWorld, SkillPair};
    use crate::tick::TickEvent;

    fn ticks(n: u64) -> Vec<TickEvent> {
        (1..=n).map(|t| TickEvent { tick: t }).collect()
    }

    fn add_jedi_character(world: &mut GameWorld, jedi_probability: u32, force_xp: u32, force_tier: ForceTier) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId(0),
            name: "Test Jedi".into(),
            is_alliance: true,
            is_empire: false,
            is_major: true,
            diplomacy: SkillPair { base: 50, variance: 0 },
            espionage: SkillPair { base: 50, variance: 0 },
            ship_design: SkillPair { base: 50, variance: 0 },
            troop_training: SkillPair { base: 50, variance: 0 },
            facility_design: SkillPair { base: 50, variance: 0 },
            combat: SkillPair { base: 70, variance: 0 },
            leadership: SkillPair { base: 60, variance: 0 },
            loyalty: SkillPair { base: 80, variance: 0 },
            jedi_probability,
            jedi_level: SkillPair { base: 5, variance: 0 },
            can_be_admiral: false,
            can_be_commander: true,
            can_be_general: false,
            force_tier,
            force_experience: force_xp,
            is_discovered_jedi: false,
        })
    }

    #[test]
    fn no_ticks_returns_empty() {
        let world = GameWorld::default();
        let mut state = JediState::new();
        let events = JediSystem::advance(&mut state, &world, &[], &[]);
        assert!(events.is_empty());
    }

    #[test]
    fn no_trainees_returns_empty() {
        let world = GameWorld::default();
        let mut state = JediState::new();
        let events = JediSystem::advance(&mut state, &world, &ticks(10), &[0.5]);
        assert!(events.is_empty());
    }

    #[test]
    fn experience_accumulates_and_tier_advances() {
        let mut world = GameWorld::default();
        let key = add_jedi_character(&mut world, 100, 0, ForceTier::Aware);

        let mut state = JediState::new();
        state.start_training(key, true, 0);

        // Advance 50 ticks — should reach Training threshold (XP_TO_TRAINING = 50).
        // But force_experience on the character starts at 0 — the system reads it
        // and computes new_xp. We need to simulate the caller applying the XP update.
        // In the real loop the caller updates world.characters[key].force_experience.
        // For the test, we advance in steps and manually apply.

        let events = JediSystem::advance(&mut state, &world, &ticks(50), &[0.99]);

        let tier_event = events.iter().find(|e| matches!(e, JediEvent::TierAdvanced { new_tier: ForceTier::Training, .. }));
        assert!(tier_event.is_some(), "should advance to Training after 50 XP");
    }

    #[test]
    fn training_completes_at_experienced() {
        let mut world = GameWorld::default();
        // Start at 140 XP — 10 more will push past XP_TO_EXPERIENCED=150.
        let key = add_jedi_character(&mut world, 100, 140, ForceTier::Training);

        let mut state = JediState::new();
        state.start_training(key, true, 0);

        let events = JediSystem::advance(&mut state, &world, &ticks(10), &[0.99]);

        let completed = events.iter().any(|e| matches!(e, JediEvent::TrainingComplete { .. }));
        assert!(completed, "should emit TrainingComplete when reaching Experienced");

        // Character removed from training.
        assert!(!state.is_training(key), "completed trainees should be removed");
    }

    #[test]
    fn detection_triggers_on_low_roll() {
        let mut world = GameWorld::default();
        let key = add_jedi_character(&mut world, 100, XP_TO_TRAINING, ForceTier::Training);

        let mut state = JediState::new();
        state.start_training(key, true, 0);

        // Advance DETECTION_CHECK_INTERVAL ticks so detection triggers.
        // Low roll (0.0) < DETECT_PROB_TRAINING = 0.15 → detection fires.
        let events = JediSystem::advance(
            &mut state,
            &world,
            &ticks(DETECTION_CHECK_INTERVAL),
            &[0.01],
        );

        let detected = events.iter().any(|e| matches!(e, JediEvent::JediDiscovered { .. }));
        assert!(detected, "detection should fire on low roll");
    }

    #[test]
    fn detection_does_not_trigger_on_high_roll() {
        let mut world = GameWorld::default();
        let key = add_jedi_character(&mut world, 100, XP_TO_TRAINING, ForceTier::Training);

        let mut state = JediState::new();
        state.start_training(key, true, 0);

        // High roll (0.99) > DETECT_PROB_TRAINING = 0.15 → no detection.
        let events = JediSystem::advance(
            &mut state,
            &world,
            &ticks(DETECTION_CHECK_INTERVAL),
            &[0.99],
        );

        let detected = events.iter().any(|e| matches!(e, JediEvent::JediDiscovered { .. }));
        assert!(!detected, "detection should not fire on high roll");
    }

    #[test]
    fn already_discovered_jedi_not_re_detected() {
        let mut world = GameWorld::default();
        let key = add_jedi_character(&mut world, 100, XP_TO_TRAINING, ForceTier::Training);
        // Mark as already discovered.
        world.characters[key].is_discovered_jedi = true;

        let mut state = JediState::new();
        state.start_training(key, true, 0);

        // Even with low roll, no detection event should fire.
        let events = JediSystem::advance(
            &mut state,
            &world,
            &ticks(DETECTION_CHECK_INTERVAL),
            &[0.0],
        );

        let detected = events.iter().any(|e| matches!(e, JediEvent::JediDiscovered { .. }));
        assert!(!detected, "already-discovered Jedi should not trigger detection again");
    }

    #[test]
    fn initial_awakening_respects_probability() {
        let mut world = GameWorld::default();

        // High-probability Jedi (50%).
        let high_key = add_jedi_character(&mut world, 50, 0, ForceTier::None);
        // Zero-probability character.
        let none_key = add_jedi_character(&mut world, 0, 0, ForceTier::None);

        // Roll 0.3 < 0.5 → awakened; second character has jedi_probability=0 → skipped.
        let awakened = JediSystem::apply_initial_awakening(&world, &[0.3, 0.3]);

        assert!(awakened.contains(&high_key), "high-probability character should awaken");
        assert!(!awakened.contains(&none_key), "zero-probability character should not awaken");
    }

    #[test]
    fn start_training_is_idempotent() {
        let mut world = GameWorld::default();
        let key = add_jedi_character(&mut world, 50, 0, ForceTier::Aware);
        let mut state = JediState::new();

        state.start_training(key, true, 0);
        state.start_training(key, true, 5);
        assert_eq!(state.training.len(), 1, "duplicate start_training should not append");
    }

    #[test]
    fn stop_training_removes_record() {
        let mut world = GameWorld::default();
        let key = add_jedi_character(&mut world, 50, 0, ForceTier::Aware);
        let mut state = JediState::new();

        state.start_training(key, true, 0);
        assert!(state.is_training(key));
        state.stop_training(key);
        assert!(!state.is_training(key));
    }

    #[test]
    fn tier_for_xp_boundaries() {
        assert_eq!(JediSystem::tier_for_xp(0), ForceTier::None);
        assert_eq!(JediSystem::tier_for_xp(1), ForceTier::Aware);
        assert_eq!(JediSystem::tier_for_xp(XP_TO_TRAINING - 1), ForceTier::Aware);
        assert_eq!(JediSystem::tier_for_xp(XP_TO_TRAINING), ForceTier::Training);
        assert_eq!(JediSystem::tier_for_xp(XP_TO_EXPERIENCED - 1), ForceTier::Training);
        assert_eq!(JediSystem::tier_for_xp(XP_TO_EXPERIENCED), ForceTier::Experienced);
        assert_eq!(JediSystem::tier_for_xp(u32::MAX), ForceTier::Experienced);
    }
}
