//! Scripted story events from the original game.
//!
//! Defines the 4 story event chains as GameEvent data structures.
//! Called at game start to register events in EventState.

use crate::events::*;
use crate::ids::CharacterKey;
use crate::world::{ForceTier, GameWorld};

/// Search world.characters for a character whose name contains `needle` (case-insensitive).
fn find_character(world: &GameWorld, needle: &str) -> Option<CharacterKey> {
    let needle_lower = needle.to_lowercase();
    world
        .characters
        .iter()
        .find(|(_, c)| c.name.to_lowercase().contains(&needle_lower))
        .map(|(key, _)| key)
}

/// Register all 4 scripted story event chains into `state`.
///
/// Characters are looked up by name from `world`. If a required character
/// is not found, that chain is silently skipped (graceful degradation for
/// mods or scenarios without those characters).
pub fn define_story_events(state: &mut EventState, world: &GameWorld) {
    let luke = find_character(world, "Luke");
    let vader = find_character(world, "Vader");
    let han = find_character(world, "Han");
    let leia = find_character(world, "Leia");
    let chewbacca = find_character(world, "Chew");

    // -----------------------------------------------------------------------
    // Chain 1: Luke Dagobah Training
    // -----------------------------------------------------------------------

    if let Some(luke) = luke {
        // EVT_LUKE_DAGOBAH (0x221): Luke begins Jedi training
        state.define(GameEvent {
            id: EVT_LUKE_DAGOBAH,
            name: "Luke Dagobah Training".into(),
            conditions: vec![
                EventCondition::CharacterExists { character: luke },
                EventCondition::CharacterHasForceLevel {
                    character: luke,
                    min_tier: ForceTier::Aware,
                },
                EventCondition::TickAtLeast { tick: 50 },
            ],
            actions: vec![
                EventAction::SetMandatoryMission {
                    character: luke,
                    mandatory: true,
                },
                EventAction::StartJediTraining { character: luke },
                EventAction::DisplayMessage {
                    text: "Luke Skywalker travels to Dagobah for Jedi training...".into(),
                },
            ],
            is_repeatable: false,
            enabled: true,
        });

        // EVT_DAGOBAH_COMPLETED (0x210): Luke completes training
        state.define(GameEvent {
            id: EVT_DAGOBAH_COMPLETED,
            name: "Dagobah Training Complete".into(),
            conditions: vec![
                EventCondition::EventFired {
                    id: EVT_LUKE_DAGOBAH,
                },
                EventCondition::CharacterHasForceLevel {
                    character: luke,
                    min_tier: ForceTier::Experienced,
                },
            ],
            actions: vec![
                EventAction::SetMandatoryMission {
                    character: luke,
                    mandatory: false,
                },
                EventAction::DisplayMessage {
                    text: "Luke has completed his Jedi training on Dagobah.".into(),
                },
            ],
            is_repeatable: false,
            enabled: true,
        });

        // -------------------------------------------------------------------
        // Chain 2: Final Battle (Luke vs Vader)
        // -------------------------------------------------------------------

        if let Some(vader) = vader {
            state.define(GameEvent {
                id: EVT_FINAL_BATTLE,
                name: "The Final Battle".into(),
                conditions: vec![
                    EventCondition::EventFired {
                        id: EVT_DAGOBAH_COMPLETED,
                    },
                    EventCondition::CharacterExists { character: vader },
                    EventCondition::CharacterIsForceUser { character: vader },
                ],
                actions: vec![
                    EventAction::SetMandatoryMission {
                        character: vader,
                        mandatory: true,
                    },
                    EventAction::SetMandatoryMission {
                        character: luke,
                        mandatory: true,
                    },
                    EventAction::DisplayMessage {
                        text: "The Final Battle between Luke and Vader approaches...".into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
            });
        }

        // -------------------------------------------------------------------
        // Chain 4 (partial): Jabba's Palace — Luke senses Han
        // -------------------------------------------------------------------

        // Event 0x380: Luke senses Han's capture
        if let Some(_han) = han {
            state.define(GameEvent {
                id: 0x380,
                name: "Luke Senses Han's Capture".into(),
                conditions: vec![
                    EventCondition::EventFired {
                        id: EVT_BOUNTY_ATTACK,
                    },
                    EventCondition::CharacterExists { character: luke },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "Luke senses Han's capture through the Force...".into(),
                }],
                is_repeatable: false,
                enabled: true,
            });
        }
    }

    // -----------------------------------------------------------------------
    // Chain 3: Bounty Hunter Attack on Han Solo
    // -----------------------------------------------------------------------

    if let Some(han) = han {
        state.define(GameEvent {
            id: EVT_BOUNTY_ATTACK,
            name: "Bounty Hunter Attack".into(),
            conditions: vec![
                EventCondition::TickAtLeast { tick: 100 },
                EventCondition::CharacterExists { character: han },
                EventCondition::Random { probability: 0.15 },
            ],
            actions: vec![
                EventAction::SetMandatoryMission {
                    character: han,
                    mandatory: true,
                },
                EventAction::DisplayMessage {
                    text: "Bounty hunters have captured Han Solo!".into(),
                },
            ],
            is_repeatable: false,
            enabled: true,
        });

        // -------------------------------------------------------------------
        // Chain 4: Jabba's Palace Rescue
        // -------------------------------------------------------------------

        // Event 0x381: Leia plans rescue
        if let Some(leia) = leia {
            state.define(GameEvent {
                id: 0x381,
                name: "Leia Plans Rescue".into(),
                conditions: vec![
                    EventCondition::EventFired {
                        id: EVT_BOUNTY_ATTACK,
                    },
                    EventCondition::CharacterExists { character: leia },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "Princess Leia plans a rescue mission to Jabba's Palace...".into(),
                }],
                is_repeatable: false,
                enabled: true,
            });

            // Event 0x382: Chewbacca joins rescue team
            if let Some(chewbacca) = chewbacca {
                state.define(GameEvent {
                    id: 0x382,
                    name: "Chewbacca Joins Rescue".into(),
                    conditions: vec![
                        EventCondition::EventFired { id: 0x381 },
                        EventCondition::CharacterExists {
                            character: chewbacca,
                        },
                    ],
                    actions: vec![EventAction::DisplayMessage {
                        text: "Chewbacca accompanies the rescue team to Tatooine...".into(),
                    }],
                    is_repeatable: false,
                    enabled: true,
                });
            }

            // Event 0x383: Han rescued (requires both Luke and Leia events)
            if luke.is_some() {
                state.define(GameEvent {
                    id: 0x383,
                    name: "Han Solo Rescued".into(),
                    conditions: vec![
                        EventCondition::EventFired { id: 0x380 },
                        EventCondition::EventFired { id: 0x381 },
                    ],
                    actions: vec![
                        EventAction::SetMandatoryMission {
                            character: han,
                            mandatory: false,
                        },
                        EventAction::DisplayMessage {
                            text: "Han Solo has been rescued from Jabba's Palace!".into(),
                        },
                    ],
                    is_repeatable: false,
                    enabled: true,
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventState, EventSystem};
    use crate::tick::TickEvent;
    use crate::world::{Character, ForceTier, SkillPair};

    fn tick(n: u64) -> Vec<TickEvent> {
        vec![TickEvent { tick: n }]
    }

    fn make_character(name: &str, force_tier: ForceTier, is_alliance: bool) -> Character {
        let sp = SkillPair {
            base: 0,
            variance: 0,
        };
        Character {
            dat_id: crate::ids::DatId::new(0),
            name: name.into(),
            is_alliance,
            is_empire: !is_alliance,
            is_major: true,
            diplomacy: sp,
            espionage: sp,
            ship_design: sp,
            troop_training: sp,
            facility_design: sp,
            combat: sp,
            leadership: sp,
            loyalty: sp,
            jedi_probability: 0,
            jedi_level: sp,
            can_be_admiral: false,
            can_be_commander: false,
            can_be_general: false,
            force_tier,
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

    fn make_world_with_characters() -> GameWorld {
        let mut world = GameWorld::default();
        world
            .characters
            .insert(make_character("Luke Skywalker", ForceTier::Aware, true));
        world
            .characters
            .insert(make_character("Darth Vader", ForceTier::Experienced, false));
        world
            .characters
            .insert(make_character("Han Solo", ForceTier::None, true));
        world
            .characters
            .insert(make_character("Yoda", ForceTier::Experienced, true));
        world
            .characters
            .insert(make_character("Princess Leia", ForceTier::None, true));
        world
            .characters
            .insert(make_character("Chewbacca", ForceTier::None, true));
        world
    }

    #[test]
    fn define_story_events_empty_world_does_not_panic() {
        let world = GameWorld::default();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);
        // No characters found — no events registered
        assert!(state.events().is_empty());
    }

    #[test]
    fn define_story_events_registers_events_with_characters() {
        let world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);
        // Should have registered multiple events
        assert!(
            state.events().len() >= 4,
            "expected at least 4 story events, got {}",
            state.events().len()
        );
    }

    #[test]
    fn luke_dagobah_chain_fires_in_sequence() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Before tick 50: nothing fires (Luke Dagobah requires TickAtLeast(50))
        let fired = EventSystem::advance(&mut state, &world, &tick(49), &[]);
        let dagobah_ids: Vec<u32> = fired
            .iter()
            .filter(|f| f.event_id == EVT_LUKE_DAGOBAH)
            .map(|f| f.event_id)
            .collect();
        assert!(dagobah_ids.is_empty(), "should not fire before tick 50");

        // At tick 50: Luke Dagobah fires (Luke is Aware, exists, tick >= 50)
        let fired = EventSystem::advance(&mut state, &world, &tick(50), &[]);
        let dagobah_ids: Vec<u32> = fired
            .iter()
            .filter(|f| f.event_id == EVT_LUKE_DAGOBAH)
            .map(|f| f.event_id)
            .collect();
        assert_eq!(dagobah_ids, vec![EVT_LUKE_DAGOBAH], "Luke Dagobah should fire at tick 50");

        // Dagobah completed requires Luke to be Experienced.
        // Simulate Luke advancing to Experienced.
        let luke_key = find_character(&world, "Luke").unwrap();
        world.characters.get_mut(luke_key).unwrap().force_tier = ForceTier::Experienced;

        let fired = EventSystem::advance(&mut state, &world, &tick(51), &[]);
        let completed_ids: Vec<u32> = fired
            .iter()
            .filter(|f| f.event_id == EVT_DAGOBAH_COMPLETED)
            .map(|f| f.event_id)
            .collect();
        assert_eq!(
            completed_ids,
            vec![EVT_DAGOBAH_COMPLETED],
            "Dagobah completed should fire after Luke reaches Experienced"
        );
    }

    #[test]
    fn one_shot_story_events_do_not_refire() {
        let world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Fire Luke Dagobah at tick 50
        let fired1 = EventSystem::advance(&mut state, &world, &tick(50), &[]);
        let count1 = fired1
            .iter()
            .filter(|f| f.event_id == EVT_LUKE_DAGOBAH)
            .count();
        assert_eq!(count1, 1);

        // Try again at tick 51 — should NOT fire again (one-shot)
        let fired2 = EventSystem::advance(&mut state, &world, &tick(51), &[]);
        let count2 = fired2
            .iter()
            .filter(|f| f.event_id == EVT_LUKE_DAGOBAH)
            .count();
        assert_eq!(count2, 0, "one-shot event should not fire again");
    }
}
