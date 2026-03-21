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

/// Register all scripted story event chains into `state`.
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
    let yoda = find_character(world, "Yoda");

    // -----------------------------------------------------------------------
    // Force milestone events (Task 3)
    // -----------------------------------------------------------------------

    if let Some(luke) = luke {
        // Force Awareness milestone — fires when Luke reaches Aware
        state.define(GameEvent {
            id: EVT_CHARACTER_FORCE,
            name: "Force Awakening".into(),
            conditions: vec![
                EventCondition::CharacterHasForceLevel {
                    character: luke,
                    min_tier: ForceTier::Aware,
                },
                // is_repeatable: false handles one-shot — no self-ref EventNotFired needed
            ],
            actions: vec![EventAction::DisplayMessage {
                text: "A disturbance in the Force... Luke's potential has been noticed."
                    .into(),
            }],
            is_repeatable: false,
            enabled: true,
        });

        // Force Training milestone — fires when Luke reaches Training
        state.define(GameEvent {
            id: EVT_FORCE_TRAINING,
            name: "Force Training Milestone".into(),
            conditions: vec![
                EventCondition::CharacterHasForceLevel {
                    character: luke,
                    min_tier: ForceTier::Training,
                },
                // is_repeatable: false handles one-shot
            ],
            actions: vec![EventAction::DisplayMessage {
                text: "Luke's Jedi training progresses.".into(),
            }],
            is_repeatable: false,
            enabled: true,
        });

        // -------------------------------------------------------------------
        // Chain 1: Luke Dagobah Training (with enrichment — Task 4)
        // -------------------------------------------------------------------

        // Dagobah prerequisite gate (0x390)
        if let Some(yoda) = yoda {
            state.define(GameEvent {
                id: 0x390,
                name: "Dagobah Calls".into(),
                conditions: vec![
                    EventCondition::CharacterHasForceLevel {
                        character: luke,
                        min_tier: ForceTier::Aware,
                    },
                    EventCondition::TickAtLeast { tick: 30 },
                    EventCondition::CharacterExists { character: yoda },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "Yoda senses Luke's awakening... Dagobah calls.".into(),
                }],
                is_repeatable: false,
                enabled: true,
            });

            // Yoda teacher assignment (0x392)
            state.define(GameEvent {
                id: 0x392,
                name: "Yoda Agrees to Train Luke".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x390 },
                    EventCondition::CharacterIsJediTrainer { character: yoda },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "Yoda agrees to train Luke in the ways of the Force.".into(),
                }],
                is_repeatable: false,
                enabled: true,
            });
        }

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

        // First training day (0x391) — fires after Dagobah starts
        state.define(GameEvent {
            id: 0x391,
            name: "First Training Day".into(),
            conditions: vec![
                EventCondition::EventFired {
                    id: EVT_LUKE_DAGOBAH,
                },
                EventCondition::TickAtLeast { tick: 55 },
            ],
            actions: vec![
                EventAction::AccumulateForceExperience {
                    character: luke,
                    amount: 10,
                },
                EventAction::DisplayMessage {
                    text: "Luke's first day of training on Dagobah begins.".into(),
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
        // Chain 2: Final Battle — multi-phase (Task 5)
        // -------------------------------------------------------------------

        if let Some(vader) = vader {
            // Vader dispatched (0x393)
            state.define(GameEvent {
                id: 0x393,
                name: "Vader Dispatched".into(),
                conditions: vec![
                    EventCondition::EventFired {
                        id: EVT_DAGOBAH_COMPLETED,
                    },
                    EventCondition::CharacterExists { character: vader },
                ],
                actions: vec![
                    EventAction::SetMandatoryMission {
                        character: vader,
                        mandatory: true,
                    },
                    EventAction::DisplayMessage {
                        text: "Darth Vader senses Luke's completed training. He sets out to confront his son.".into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
            });

            // Vader en route (0x394)
            state.define(GameEvent {
                id: 0x394,
                name: "Vader En Route".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x393 },
                    EventCondition::TickAtLeast { tick: 70 },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "Vader's fleet approaches Luke's location...".into(),
                }],
                is_repeatable: false,
                enabled: true,
            });

            // Vader reports to Emperor (0x395) — alternate branch
            state.define(GameEvent {
                id: 0x395,
                name: "Vader Reports to Emperor".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x393 },
                    EventCondition::FactionControlsNSystems {
                        faction: crate::dat::Faction::Empire,
                        min_count: 150,
                    },
                    EventCondition::EventNotFired { id: 0x394 },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "Vader reports to the Emperor before confronting Luke.".into(),
                }],
                is_repeatable: false,
                enabled: true,
            });

            // Final Battle ready (0x396) — requires either path
            state.define(GameEvent {
                id: 0x396,
                name: "Final Battle Imminent".into(),
                conditions: vec![
                    // We use EventFired(0x394) as the primary gate.
                    // If 0x395 fires first, 0x394 can still fire later to gate this.
                    EventCondition::EventFired { id: 0x394 },
                ],
                actions: vec![EventAction::DisplayMessage {
                    text: "The final confrontation between father and son is imminent..."
                        .into(),
                }],
                is_repeatable: false,
                enabled: true,
            });

            // EVT_FINAL_BATTLE (0x220) — requires chain completion
            state.define(GameEvent {
                id: EVT_FINAL_BATTLE,
                name: "The Final Battle".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x396 },
                    EventCondition::CharacterExists { character: luke },
                    EventCondition::CharacterExists { character: vader },
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
                        text: "The Final Battle between Luke and Vader approaches..."
                            .into(),
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

            // Palace capture Luke (0x399) — Luke captured at palace (Task 7)
            state.define(GameEvent {
                id: 0x399,
                name: "Luke Captured at Palace".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x380 },
                    EventCondition::TickAtLeast { tick: 115 },
                    EventCondition::EventNotFired { id: 0x39A }, // don't capture after rescue
                ],
                actions: vec![
                    EventAction::CaptureCharacter {
                        character: luke,
                        captor_faction: crate::dat::Faction::Empire,
                    },
                    EventAction::DisplayMessage {
                        text: "Luke has been captured at Jabba's Palace!".into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
            });
        }
    }

    // -----------------------------------------------------------------------
    // Chain 3: Bounty Hunter Attack on Han Solo (with enrichment — Task 6)
    // -----------------------------------------------------------------------

    if let Some(han) = han {
        // Bounty Hunters Active gate (0x397)
        state.define(GameEvent {
            id: 0x397,
            name: "Bounty Hunters Active".into(),
            conditions: vec![
                EventCondition::TickAtLeast { tick: 80 },
                EventCondition::FactionControlsNSystems {
                    faction: crate::dat::Faction::Empire,
                    min_count: 50,
                },
                EventCondition::CharacterExists { character: han },
            ],
            actions: vec![EventAction::DisplayMessage {
                text: "The Empire has dispatched bounty hunters across the galaxy..."
                    .into(),
            }],
            is_repeatable: false,
            enabled: true,
        });

        // EVT_BOUNTY_ATTACK (0x212) — now requires BountyHuntersActive gate
        state.define(GameEvent {
            id: EVT_BOUNTY_ATTACK,
            name: "Bounty Hunter Attack".into(),
            conditions: vec![
                EventCondition::EventFired { id: 0x397 },
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

        // Han Carbonite aftermath (0x398)
        state.define(GameEvent {
            id: 0x398,
            name: "Han Solo Frozen in Carbonite".into(),
            conditions: vec![EventCondition::EventFired {
                id: EVT_BOUNTY_ATTACK,
            }],
            actions: vec![
                EventAction::SetCarboniteState {
                    character: han,
                    frozen: true,
                },
                EventAction::DisplayMessage {
                    text: "Han Solo has been frozen in carbonite.".into(),
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
                    text: "Princess Leia plans a rescue mission to Jabba's Palace..."
                        .into(),
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
                        text: "Chewbacca accompanies the rescue team to Tatooine..."
                            .into(),
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

                // Palace rescue complete (0x39A) — all rescued, unfreeze Han (Task 7)
                state.define(GameEvent {
                    id: 0x39A,
                    name: "Palace Rescue Complete".into(),
                    conditions: vec![
                        EventCondition::EventFired { id: 0x383 },
                        // is_repeatable: false handles one-shot
                    ],
                    actions: vec![
                        EventAction::SetCarboniteState {
                            character: han,
                            frozen: false,
                        },
                        EventAction::DisplayMessage {
                            text: "The heroes escape Jabba's Palace!".into(),
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
    use crate::world::ControlKind;
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
        {
            let mut yoda = make_character("Yoda", ForceTier::Experienced, true);
            yoda.is_jedi_trainer = true;
            world.characters.insert(yoda);
        }
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
        // Should have registered many events (4 original chains + enrichments)
        assert!(
            state.events().len() >= 10,
            "expected at least 10 story events, got {}",
            state.events().len()
        );
    }

    #[test]
    fn luke_dagobah_chain_fires_in_sequence() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Dagobah prerequisite gate fires at tick 30 (Yoda exists, Luke Aware)
        let fired = EventSystem::advance(&mut state, &world, &tick(30), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x390),
            "Dagobah prerequisite gate should fire at tick 30"
        );

        // Yoda teacher assignment should also chain-fire (0x392)
        assert!(
            fired.iter().any(|f| f.event_id == 0x392),
            "Yoda teacher assignment should chain-fire"
        );

        // Before tick 50: Luke Dagobah should NOT fire
        let fired = EventSystem::advance(&mut state, &world, &tick(49), &[]);
        assert!(
            !fired.iter().any(|f| f.event_id == EVT_LUKE_DAGOBAH),
            "should not fire before tick 50"
        );

        // At tick 50: Luke Dagobah fires (Luke is Aware, exists, tick >= 50)
        let fired = EventSystem::advance(&mut state, &world, &tick(50), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_LUKE_DAGOBAH),
            "Luke Dagobah should fire at tick 50"
        );

        // First training day fires at tick 55
        let fired = EventSystem::advance(&mut state, &world, &tick(55), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x391),
            "First training day should fire at tick 55"
        );

        // Dagobah completed requires Luke to be Experienced
        let luke_key = find_character(&world, "Luke").unwrap();
        world.characters.get_mut(luke_key).unwrap().force_tier = ForceTier::Experienced;

        let fired = EventSystem::advance(&mut state, &world, &tick(56), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_DAGOBAH_COMPLETED),
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

    // --- Task 8: New condition variant tests ---

    #[test]
    fn character_force_experience_evaluates_at_threshold() {
        let mut world = GameWorld::default();
        let luke = world
            .characters
            .insert(make_character("Luke", ForceTier::Aware, true));

        // Set force_experience to 25
        world.characters.get_mut(luke).unwrap().force_experience = 25;

        let mut state = EventState::new();
        state.define(GameEvent {
            id: 1,
            name: "xp_test".into(),
            conditions: vec![EventCondition::CharacterForceExperience {
                character: luke,
                min_xp: 20,
            }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "25 >= 20 should fire");

        // Reset with higher threshold
        let mut state2 = EventState::new();
        state2.define(GameEvent {
            id: 2,
            name: "xp_test_fail".into(),
            conditions: vec![EventCondition::CharacterForceExperience {
                character: luke,
                min_xp: 30,
            }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });

        let fired2 = EventSystem::advance(&mut state2, &world, &tick(1), &[]);
        assert!(fired2.is_empty(), "25 < 30 should not fire");
    }

    #[test]
    fn character_is_captive_evaluates_correctly() {
        let mut world = GameWorld::default();
        let han = world
            .characters
            .insert(make_character("Han Solo", ForceTier::None, true));

        let mut state = EventState::new();
        state.define(GameEvent {
            id: 1,
            name: "captive_test".into(),
            conditions: vec![EventCondition::CharacterIsCaptive { character: han }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });

        // Han not captive — should not fire
        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty(), "Han is not captive");

        // Make Han captive
        world.characters.get_mut(han).unwrap().is_captive = true;
        let mut state2 = EventState::new();
        state2.define(GameEvent {
            id: 2,
            name: "captive_test_true".into(),
            conditions: vec![EventCondition::CharacterIsCaptive { character: han }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });

        let fired2 = EventSystem::advance(&mut state2, &world, &tick(1), &[]);
        assert_eq!(fired2.len(), 1, "Han is captive, should fire");
    }

    #[test]
    fn event_not_fired_returns_true_when_unfired() {
        let world = GameWorld::default();
        let mut state = EventState::new();

        // Event 1: fires immediately. Event 2: requires event 99 NOT fired.
        state.define(GameEvent {
            id: 2,
            name: "not_fired_test".into(),
            conditions: vec![EventCondition::EventNotFired { id: 99 }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "event 99 never fired, so EventNotFired should be true");

        // Now define and fire event 99, then check EventNotFired again
        let mut state2 = EventState::new();
        state2.define(GameEvent {
            id: 99,
            name: "prereq".into(),
            conditions: vec![],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });
        state2.define(GameEvent {
            id: 3,
            name: "not_fired_should_fail".into(),
            conditions: vec![EventCondition::EventNotFired { id: 99 }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
        });

        let fired2 = EventSystem::advance(&mut state2, &world, &tick(1), &[]);
        // Event 99 fires first, then event 3 checks EventNotFired(99) — should fail
        assert!(
            !fired2.iter().any(|f| f.event_id == 3),
            "event 99 fired in same advance, so EventNotFired(99) should be false"
        );
    }

    #[test]
    fn force_milestone_events_fire_at_correct_tiers() {
        let world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Luke starts as Aware — Force Awakening milestone should fire
        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_CHARACTER_FORCE),
            "Force Awakening should fire for Aware Luke"
        );

        // Force Training milestone requires Training tier — should NOT fire yet
        assert!(
            !fired.iter().any(|f| f.event_id == EVT_FORCE_TRAINING),
            "Force Training should not fire while Luke is only Aware"
        );
    }

    #[test]
    fn final_battle_chain_requires_dagobah_completion() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Advance through Dagobah prerequisite (tick 30) and training (tick 50)
        EventSystem::advance(&mut state, &world, &tick(30), &[]);
        EventSystem::advance(&mut state, &world, &tick(50), &[]);

        // Final Battle should NOT fire yet — Dagobah not completed
        let fired = EventSystem::advance(&mut state, &world, &tick(60), &[]);
        assert!(
            !fired.iter().any(|f| f.event_id == EVT_FINAL_BATTLE),
            "Final Battle should not fire before Dagobah completion"
        );

        // Complete Dagobah by making Luke Experienced
        let luke_key = find_character(&world, "Luke").unwrap();
        world.characters.get_mut(luke_key).unwrap().force_tier = ForceTier::Experienced;

        // Dagobah completed + Vader dispatched should chain
        let fired = EventSystem::advance(&mut state, &world, &tick(61), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_DAGOBAH_COMPLETED),
            "Dagobah completed should fire"
        );
        assert!(
            fired.iter().any(|f| f.event_id == 0x393),
            "Vader dispatched should chain off Dagobah completion"
        );

        // Vader en route at tick 70
        let fired = EventSystem::advance(&mut state, &world, &tick(70), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x394),
            "Vader en route should fire at tick 70"
        );

        // Final Battle ready (0x396) and then Final Battle (0x220) should chain
        assert!(
            fired.iter().any(|f| f.event_id == 0x396),
            "Final Battle Imminent should fire"
        );
        assert!(
            fired.iter().any(|f| f.event_id == EVT_FINAL_BATTLE),
            "The Final Battle should fire after full chain"
        );
    }

    #[test]
    fn bounty_hunters_require_active_gate() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();

        // Need 50+ Empire systems for the gate
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key,
                x: 0,
                y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3,
                popularity_empire: 0.7,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                defense_facilities: vec![],
                manufacturing_facilities: vec![],
                production_facilities: vec![],
                is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }

        define_story_events(&mut state, &world);

        // Bounty Attack should NOT fire at tick 100 without gate (gate fires at 80)
        // First, tick 79 — gate should not fire yet
        let fired = EventSystem::advance(&mut state, &world, &tick(79), &[]);
        assert!(
            !fired.iter().any(|f| f.event_id == 0x397),
            "Bounty Hunters Active gate should not fire before tick 80"
        );

        // Tick 80 — gate fires
        let fired = EventSystem::advance(&mut state, &world, &tick(80), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x397),
            "Bounty Hunters Active gate should fire at tick 80"
        );

        // Tick 100 with rng roll — Bounty Attack fires (gate already fired)
        let fired = EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_BOUNTY_ATTACK),
            "Bounty Attack should fire after gate, at tick 100 with successful roll"
        );
    }

    #[test]
    fn palace_events_chain_correctly() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();

        // Need Empire systems for bounty hunter gate
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key,
                x: 0,
                y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3,
                popularity_empire: 0.7,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                defense_facilities: vec![],
                manufacturing_facilities: vec![],
                production_facilities: vec![],
                is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }

        define_story_events(&mut state, &world);

        // Fire bounty hunter gate (tick 80)
        EventSystem::advance(&mut state, &world, &tick(80), &[]);

        // Fire bounty attack (tick 100 with low roll)
        let fired = EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_BOUNTY_ATTACK),
            "Bounty Attack should fire"
        );
        // Han Carbonite (0x398) chains off bounty attack (defined after it)
        assert!(
            fired.iter().any(|f| f.event_id == 0x398),
            "Han Carbonite should chain off Bounty Attack"
        );

        // Events defined AFTER EVT_BOUNTY_ATTACK chain in the same advance pass:
        // 0x381 (Leia plans rescue) — sees EventFired(EVT_BOUNTY_ATTACK) because
        // it's evaluated after the bounty attack in event-definition order.
        assert!(
            fired.iter().any(|f| f.event_id == 0x381),
            "Leia plans rescue should chain-fire same pass (defined after bounty attack)"
        );

        // 0x382 (Chewbacca joins) chains off 0x381
        assert!(
            fired.iter().any(|f| f.event_id == 0x382),
            "Chewbacca joins rescue should chain-fire"
        );

        // Events defined BEFORE EVT_BOUNTY_ATTACK fire on the next tick:
        // 0x380 (Luke senses Han) was evaluated before bounty attack fired.
        let fired2 = EventSystem::advance(&mut state, &world, &tick(101), &[]);
        assert!(
            fired2.iter().any(|f| f.event_id == 0x380),
            "Luke senses Han should fire on next tick"
        );

        // 0x383 (Han rescued) requires BOTH 0x380 + 0x381 — 0x380 just fired,
        // 0x381 fired at tick 100. Both are true, so 0x383 chains in same pass.
        assert!(
            fired2.iter().any(|f| f.event_id == 0x383),
            "Han rescued should fire after 0x380 fires"
        );

        // 0x39A (Palace rescue complete) chains off 0x383 in the same pass
        // (defined right after 0x383 in event list)
        assert!(
            fired2.iter().any(|f| f.event_id == 0x39A),
            "Palace rescue complete should chain-fire same pass as 0x383"
        );

        // Luke capture (0x399) should NOT fire after rescue (0x39A) completed
        let fired4 = EventSystem::advance(&mut state, &world, &tick(115), &[]);
        assert!(
            !fired4.iter().any(|f| f.event_id == 0x399),
            "Luke should NOT be captured after palace rescue complete (0x39A guards 0x399)"
        );
    }
}
