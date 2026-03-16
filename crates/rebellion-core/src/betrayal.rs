//! Betrayal system: loyalty-driven faction defection.
//!
//! Characters with low loyalty may betray their faction. The probability
//! is looked up from UPRIS1TB (same table used for uprising thresholds).
//! Characters with `is_unable_to_betray = true` (Luke, Vader) are immune.
//!
//! Follows the same stateless advance() pattern as other simulation systems:
//! - `BetrayalState` holds per-character cooldown timers
//! - `BetrayalSystem::advance(state, world, tick_events, rng_rolls, loyalty_table)`
//!   returns `Vec<BetrayalEvent>`
//! - The caller provides pre-generated random rolls for determinism

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ids::CharacterKey;
use crate::tick::TickEvent;
use crate::world::{GameWorld, MstbTable};

/// How often (in ticks) the betrayal check runs per character.
pub const BETRAYAL_CHECK_INTERVAL: u64 = 50;

// ---------------------------------------------------------------------------
// BetrayalEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BetrayalEvent {
    /// A character has betrayed their faction and switched sides.
    CharacterBetrayed {
        character: CharacterKey,
        /// True if the character defected to the Alliance; false if to the Empire.
        defected_to_alliance: bool,
    },
}

// ---------------------------------------------------------------------------
// BetrayalState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BetrayalState {
    /// Last tick a betrayal check ran per character (prevents spam).
    last_check: HashMap<CharacterKey, u64>,
}

impl BetrayalState {
    pub fn new() -> Self {
        Self::default()
    }
}

// ---------------------------------------------------------------------------
// BetrayalSystem
// ---------------------------------------------------------------------------

/// Stateless system that checks for character betrayals per tick.
pub struct BetrayalSystem;

impl BetrayalSystem {
    /// Advance the betrayal system: check each character for possible defection.
    ///
    /// # RNG contract
    ///
    /// The caller provides `rng_rolls`: one `f64` roll per character that is
    /// eligible for a betrayal check this frame. Each roll is uniform [0, 1).
    /// Characters are iterated in slotmap order; rolls are consumed sequentially.
    pub fn advance(
        state: &mut BetrayalState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        rng_rolls: &[f64],
        loyalty_table: &MstbTable,
    ) -> Vec<BetrayalEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let current_tick = tick_events.last().unwrap().tick;
        let mut events = Vec::new();
        let mut roll_iter = rng_rolls.iter().copied();

        for (ck, character) in world.characters.iter() {
            // Immune characters never betray.
            if character.is_unable_to_betray {
                continue;
            }

            // Cooldown: skip if checked too recently.
            if let Some(&last) = state.last_check.get(&ck) {
                if current_tick.saturating_sub(last) < BETRAYAL_CHECK_INTERVAL {
                    continue;
                }
            }

            // Loyalty score: base - 50 (same scale as uprising).
            // Positive = loyal enough, skip.
            let loyalty_score = character.loyalty.base as i32 - 50;
            if loyalty_score >= 0 {
                continue;
            }

            // Look up betrayal probability from the table.
            let prob = loyalty_table.lookup(loyalty_score) as f64 / 100.0;

            // Consume a roll. If exhausted, default to 1.0 (no betrayal) — safe conservative fallback.
            let roll = roll_iter.next().unwrap_or(1.0);

            // Update last check regardless of outcome.
            state.last_check.insert(ck, current_tick);

            if roll < prob {
                // Defect to the opposing faction.
                let defected_to_alliance = !character.is_alliance;
                events.push(BetrayalEvent::CharacterBetrayed {
                    character: ck,
                    defected_to_alliance,
                });
            }
        }

        events
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::DatId;
    use crate::tick::TickEvent;
    use crate::world::{Character, ForceTier, GameWorld, MstbEntry, MstbTable, SkillPair};

    fn skill_pair(base: u32) -> SkillPair {
        SkillPair { base, variance: 0 }
    }

    fn default_test_character(name: &str, loyalty_base: u32) -> Character {
        let zero = skill_pair(0);
        Character {
            dat_id: DatId::new(0),
            name: name.into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: zero,
            espionage: zero,
            ship_design: zero,
            troop_training: zero,
            facility_design: zero,
            combat: zero,
            leadership: zero,
            loyalty: skill_pair(loyalty_base),
            jedi_probability: 0,
            jedi_level: zero,
            can_be_admiral: false,
            can_be_commander: false,
            can_be_general: false,
            force_tier: ForceTier::None,
            force_experience: 0,
            is_discovered_jedi: false,
            is_unable_to_betray: false,
            is_jedi_trainer: false,
            is_known_jedi: false,
            hyperdrive_modifier: 0,
            enhanced_loyalty: 0,
            on_mission: false,
            on_hidden_mission: false,
            on_mandatory_mission: false,
            captured_by: None,
            capture_tick: None,
            is_captive: false,
            current_system: None,
            current_fleet: None,
        }
    }

    /// Simple table: any negative score → 80% betrayal chance.
    fn test_loyalty_table() -> MstbTable {
        MstbTable::new(vec![
            MstbEntry { threshold: -100, value: 80 },
            MstbEntry { threshold: -1, value: 80 },
            MstbEntry { threshold: 0, value: 0 },
            MstbEntry { threshold: 100, value: 0 },
        ])
    }

    fn ticks(n: u64) -> Vec<TickEvent> {
        (1..=n).map(|t| TickEvent { tick: t }).collect()
    }

    #[test]
    fn no_events_without_ticks() {
        let mut state = BetrayalState::new();
        let world = GameWorld::default();
        let table = test_loyalty_table();
        let events = BetrayalSystem::advance(&mut state, &world, &[], &[0.0], &table);
        assert!(events.is_empty());
    }

    #[test]
    fn high_loyalty_no_betrayal() {
        let mut state = BetrayalState::new();
        let mut world = GameWorld::default();
        // Loyalty 80 → score = 80 - 50 = 30 → >= 0 → skip
        world.characters.insert(default_test_character("Loyal", 80));
        let table = test_loyalty_table();
        let events = BetrayalSystem::advance(
            &mut state, &world, &ticks(1), &[0.0], &table,
        );
        assert!(events.is_empty());
    }

    #[test]
    fn low_loyalty_low_roll_triggers_betrayal() {
        let mut state = BetrayalState::new();
        let mut world = GameWorld::default();
        // Loyalty 10 → score = 10 - 50 = -40 → table says 80% → roll 0.1 < 0.8 → betrays
        let ck = world.characters.insert(default_test_character("Disloyal", 10));
        let table = test_loyalty_table();
        let events = BetrayalSystem::advance(
            &mut state, &world, &ticks(1), &[0.1], &table,
        );
        assert_eq!(events.len(), 1);
        match &events[0] {
            BetrayalEvent::CharacterBetrayed { character, defected_to_alliance } => {
                assert_eq!(*character, ck);
                // is_alliance=true → defects to !alliance = false (Empire)
                assert!(!defected_to_alliance);
            }
        }
    }

    #[test]
    fn is_unable_to_betray_blocks_betrayal() {
        let mut state = BetrayalState::new();
        let mut world = GameWorld::default();
        let mut c = default_test_character("Luke", 10); // low loyalty but immune
        c.is_unable_to_betray = true;
        world.characters.insert(c);
        let table = test_loyalty_table();
        let events = BetrayalSystem::advance(
            &mut state, &world, &ticks(1), &[0.0], &table,
        );
        assert!(events.is_empty());
    }

    #[test]
    fn cooldown_prevents_recheck() {
        let mut state = BetrayalState::new();
        let mut world = GameWorld::default();
        // Loyalty 10 → will trigger if roll is low enough
        world.characters.insert(default_test_character("Traitor", 10));
        let table = test_loyalty_table();

        // First check at tick 1 — roll 0.9 > 0.8 → no betrayal, but last_check updated
        let events = BetrayalSystem::advance(
            &mut state, &world, &ticks(1), &[0.9], &table,
        );
        assert!(events.is_empty());

        // Second check at tick 2 — within cooldown (50 ticks), should skip
        let tick2 = vec![TickEvent { tick: 2 }];
        let events = BetrayalSystem::advance(
            &mut state, &world, &tick2, &[0.0], &table,
        );
        assert!(events.is_empty());

        // Third check at tick 51 — past cooldown, should check again
        let tick51 = vec![TickEvent { tick: 51 }];
        let events = BetrayalSystem::advance(
            &mut state, &world, &tick51, &[0.1], &table,
        );
        assert_eq!(events.len(), 1);
    }
}
