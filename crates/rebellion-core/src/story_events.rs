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
    let emperor = find_character(world, "Palpatine")
        .or_else(|| find_character(world, "Emperor"));

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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
                system_tag: SystemTag::Story,
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
                system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
                system_tag: SystemTag::Story,
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
                system_tag: SystemTag::Story,
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
                system_tag: SystemTag::Story,
            });

            // Final Battle ready (0x396) — requires either path.
            // "father and son" narration reveals heritage; SetHeritageKnown
            // flips Luke's flag so the render layer picks the correct BMP.
            state.define(GameEvent {
                id: 0x396,
                name: "Final Battle Imminent".into(),
                conditions: vec![
                    // We use EventFired(0x394) as the primary gate.
                    // If 0x395 fires first, 0x394 can still fire later to gate this.
                    EventCondition::EventFired { id: 0x394 },
                ],
                actions: vec![
                    EventAction::SetHeritageKnown { character: luke },
                    EventAction::DisplayMessage {
                        text: "The final confrontation between father and son is imminent..."
                            .into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });

            // EVT_FINAL_BATTLE (0x220) — requires chain completion + co-location
            {
                let mut final_conditions = vec![
                    EventCondition::EventFired { id: 0x396 },
                    EventCondition::CharacterExists { character: luke },
                    EventCondition::CharacterExists { character: vader },
                ];
                let mut final_actions = vec![
                    EventAction::SetMandatoryMission {
                        character: vader,
                        mandatory: true,
                    },
                    EventAction::SetMandatoryMission {
                        character: luke,
                        mandatory: true,
                    },
                ];

                // Emperor Palpatine co-location requirement (Phase 3)
                if let Some(emperor) = emperor {
                    final_conditions.push(EventCondition::CharacterExists {
                        character: emperor,
                    });
                    final_conditions.push(EventCondition::CharactersCoLocated {
                        characters: vec![luke, vader, emperor],
                    });
                    final_actions.push(EventAction::SetMandatoryMission {
                        character: emperor,
                        mandatory: true,
                    });
                    final_actions.push(EventAction::DisplayMessage {
                        text: "Luke, Vader, and the Emperor are together. The Final Battle begins..."
                            .into(),
                    });
                } else {
                    final_actions.push(EventAction::DisplayMessage {
                        text: "The Final Battle between Luke and Vader approaches..."
                            .into(),
                    });
                }

                state.define(GameEvent {
                    id: EVT_FINAL_BATTLE,
                    name: "The Final Battle".into(),
                    conditions: final_conditions,
                    actions: final_actions,
                    is_repeatable: false,
                    enabled: true,
                    system_tag: SystemTag::Story,
                });
            }
        }

        // -------------------------------------------------------------------
        // Leia Force Discovery (0x362) — Leia discovers Force sensitivity
        // Requires Luke's Dagobah training to be complete first
        // -------------------------------------------------------------------
        if let Some(leia) = leia {
            state.define(GameEvent {
                id: EVT_LEIA_FORCE,
                name: "Leia Discovers the Force".into(),
                conditions: vec![
                    EventCondition::CharacterExists { character: leia },
                    // Luke must have completed training first
                    EventCondition::EventFired {
                        id: EVT_DAGOBAH_COMPLETED,
                    },
                    EventCondition::TickAtLeast { tick: 80 },
                ],
                actions: vec![
                    EventAction::ModifyForceTier {
                        character: leia,
                        new_tier: ForceTier::Aware,
                    },
                    EventAction::DisplayMessage {
                        text: "Princess Leia has discovered her connection to the Force!"
                            .into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });
        }

        // -------------------------------------------------------------------
        // Emperor Arrival (0x230) — Emperor travels to the front
        // -------------------------------------------------------------------
        if let Some(emperor) = emperor {
            state.define(GameEvent {
                id: EVT_EMPEROR_ARRIVAL,
                name: "Emperor Arrives".into(),
                conditions: vec![
                    EventCondition::CharacterExists { character: emperor },
                    EventCondition::TickAtLeast { tick: 90 },
                    EventCondition::FactionControlsNSystems {
                        faction: crate::dat::Faction::Empire,
                        min_count: 100,
                    },
                ],
                actions: vec![
                    EventAction::DisplayMessage {
                        text: "The Emperor has arrived to oversee operations personally."
                            .into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });
        }

        // -------------------------------------------------------------------
        // Chain 4: Jabba's Palace — 5-case outcome switch
        //
        // Cases:
        //   1. escape_self    — Han escapes on his own (0x384, low probability)
        //   2. escape_rescue  — Rescue team frees Han (existing 0x383 → 0x39A)
        //   3. captured_luke  — Luke captured at palace (0x399)
        //   4. jabba_captures_leia   — Leia captured during rescue (0x385)
        //   5. jabba_captures_chewie — Chewie captured during rescue (0x386)
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
                system_tag: SystemTag::Story,
            });

            // Case 1: Han self-escape (0x384) — low probability, after time in carbonite
            state.define(GameEvent {
                id: 0x384,
                name: "Han Escapes Carbonite".into(),
                conditions: vec![
                    EventCondition::EventFired {
                        id: EVT_BOUNTY_ATTACK,
                    },
                    EventCondition::TickAtLeast { tick: 135 },
                    EventCondition::EventNotFired { id: 0x383 }, // not already rescued
                    EventCondition::EventNotFired { id: 0x399 }, // not already captured
                    EventCondition::Random { probability: 0.10 },
                ],
                actions: vec![
                    EventAction::SetCarboniteState {
                        character: _han,
                        frozen: false,
                    },
                    EventAction::SetMandatoryMission {
                        character: _han,
                        mandatory: false,
                    },
                    EventAction::DisplayMessage {
                        text: "Han Solo has managed to escape from Jabba's Palace on his own!"
                            .into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });

            // Case 3: Luke captured at palace (0x399) — if rescue hasn't happened
            // Random gate (0.20) makes this non-deterministic so other outcomes
            // (Leia captured at 108, Chewie captured at 110) can preempt it.
            state.define(GameEvent {
                id: 0x399,
                name: "Luke Captured at Palace".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x380 },
                    EventCondition::TickAtLeast { tick: 115 },
                    EventCondition::EventNotFired { id: 0x383 }, // Han not rescued yet
                    EventCondition::EventNotFired { id: 0x39A }, // rescue not completed
                    EventCondition::EventNotFired { id: 0x384 }, // don't capture after self-escape
                    EventCondition::Random { probability: 0.20 },
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
                system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
        });

        // EVT_BOUNTY_ATTACK (0x212) — now requires BountyHuntersActive gate.
        // SF-#7: CharacterAssignedToFleet ensures integrator can resolve a
        // landing system for SpawnSpecialForce (Han must be on a fleet).
        state.define(GameEvent {
            id: EVT_BOUNTY_ATTACK,
            name: "Bounty Hunter Attack".into(),
            conditions: vec![
                EventCondition::EventFired { id: 0x397 },
                EventCondition::TickAtLeast { tick: 100 },
                EventCondition::CharacterExists { character: han },
                EventCondition::CharacterAssignedToFleet { character: han },
                EventCondition::Random { probability: 0.15 },
            ],
            actions: vec![
                EventAction::SpawnSpecialForce { at_character: han },
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
                system_tag: SystemTag::Story,
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
                    system_tag: SystemTag::Story,
                });
            }

            // Event 0x383: Han rescued (requires both Luke and Leia events)
            // Guards: rescue can't succeed if Luke/Leia/Chewie already captured
            if luke.is_some() {
                state.define(GameEvent {
                    id: 0x383,
                    name: "Han Solo Rescued".into(),
                    conditions: vec![
                        EventCondition::EventFired { id: 0x380 },
                        EventCondition::EventFired { id: 0x381 },
                        EventCondition::EventNotFired { id: 0x399 }, // Luke not captured
                        EventCondition::EventNotFired { id: 0x385 }, // Leia not captured
                        EventCondition::EventNotFired { id: EVT_JABBA_CAPTURES_CHEWIE }, // Chewie not captured
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
                    system_tag: SystemTag::Story,
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
                    system_tag: SystemTag::Story,
                });

                // EVT_HAN_RESCUE (0x200) — telemetry twin of 0x383.
                // Fires same-tick via EventFired chaining. No actions (silent);
                // exists so the original game's distinct event ID emits to telemetry.
                state.define(GameEvent {
                    id: EVT_HAN_RESCUE,
                    name: "Han Rescue".into(),
                    conditions: vec![EventCondition::EventFired { id: 0x383 }],
                    actions: vec![],
                    is_repeatable: false,
                    enabled: true,
                    system_tag: SystemTag::Story,
                });
            }

            // Case 4: Jabba captures Leia during rescue attempt (0x385)
            // Tick 108 — earlier than Luke capture (115) so this outcome can preempt it
            state.define(GameEvent {
                id: 0x385,
                name: "Leia Captured by Jabba".into(),
                conditions: vec![
                    EventCondition::EventFired { id: 0x381 }, // Leia planned rescue
                    EventCondition::TickAtLeast { tick: 108 },
                    EventCondition::EventNotFired { id: 0x383 }, // rescue hasn't succeeded
                    EventCondition::EventNotFired { id: 0x384 }, // Han hasn't self-escaped
                    EventCondition::Random { probability: 0.15 },
                ],
                actions: vec![
                    EventAction::CaptureCharacter {
                        character: leia,
                        captor_faction: crate::dat::Faction::Empire,
                    },
                    EventAction::DisplayMessage {
                        text: "Princess Leia has been captured by Jabba during the rescue attempt!"
                            .into(),
                    },
                ],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });

            // Case 5: Jabba captures Chewbacca during rescue attempt (0x386)
            if let Some(chewbacca) = chewbacca {
                state.define(GameEvent {
                    id: EVT_JABBA_CAPTURES_CHEWIE,
                    name: "Chewbacca Captured by Jabba".into(),
                    conditions: vec![
                        EventCondition::EventFired { id: 0x382 }, // Chewie joined rescue
                        EventCondition::TickAtLeast { tick: 110 },
                        EventCondition::EventNotFired { id: 0x383 }, // rescue hasn't succeeded
                        EventCondition::EventNotFired { id: 0x384 }, // Han hasn't self-escaped
                        EventCondition::EventNotFired { id: 0x385 }, // Leia not already captured
                        EventCondition::Random { probability: 0.12 },
                    ],
                    actions: vec![
                        EventAction::CaptureCharacter {
                            character: chewbacca,
                            captor_faction: crate::dat::Faction::Empire,
                        },
                        EventAction::DisplayMessage {
                            text: "Chewbacca has been captured by Jabba's guards!".into(),
                        },
                    ],
                    is_repeatable: false,
                    enabled: true,
                    system_tag: SystemTag::Story,
                });
            }
        }

        // EVT_JABBA_PRISONERS (0x231) — consolidator for any palace capture.
        // Three OR-branch variants: one per capture ID (0x385 Leia, 0x387 Chewie,
        // 0x399 Luke). Uses define_or_branch() to register multiple entries with
        // the same event ID. Self-guard ensures exactly one fires even if two
        // captures land in the same tick.
        for &trigger_id in &[0x385, EVT_JABBA_CAPTURES_CHEWIE, 0x399] {
            state.define_or_branch(GameEvent {
                id: EVT_JABBA_PRISONERS,
                name: "Jabba's Prisoners".into(),
                conditions: vec![
                    EventCondition::EventFired { id: trigger_id },
                    EventCondition::EventNotFired { id: EVT_JABBA_PRISONERS },
                ],
                actions: vec![],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });
        }

        // -------------------------------------------------------------------
        // Carbonite escape countdown — 5 tick-gated events track elapsed time
        // without rescue. Each chains off its predecessor. If self-escape
        // (0x384) or rescue (0x383) fires at any point, EventNotFired guards
        // halt the chain. After all 5 stages pass, EVT_HAN_PERMANENT_FREEZE
        // fires as the terminal state.
        // -------------------------------------------------------------------
        const CARBONITE_STAGES: [(u32, u32, u64); 5] = [
            (EVT_HAN_CARBONITE_FAIL_1, 0x398,                  145),
            (EVT_HAN_CARBONITE_FAIL_2, EVT_HAN_CARBONITE_FAIL_1, 160),
            (EVT_HAN_CARBONITE_FAIL_3, EVT_HAN_CARBONITE_FAIL_2, 175),
            (EVT_HAN_CARBONITE_FAIL_4, EVT_HAN_CARBONITE_FAIL_3, 190),
            (EVT_HAN_CARBONITE_FAIL_5, EVT_HAN_CARBONITE_FAIL_4, 205),
        ];
        for (i, &(id, predecessor, tick_min)) in CARBONITE_STAGES.iter().enumerate() {
            state.define(GameEvent {
                id,
                name: format!("Carbonite Escape Fails ({})", i + 1),
                conditions: vec![
                    EventCondition::EventFired { id: predecessor },
                    EventCondition::TickAtLeast { tick: tick_min },
                    EventCondition::EventNotFired { id: 0x383 },
                    EventCondition::EventNotFired { id: 0x384 },
                ],
                actions: vec![],
                is_repeatable: false,
                enabled: true,
                system_tag: SystemTag::Story,
            });
        }

        // EVT_HAN_PERMANENT_FREEZE (0x39B) — terminal state.
        // Han has been frozen too long; rescue is no longer possible.
        state.define(GameEvent {
            id: EVT_HAN_PERMANENT_FREEZE,
            name: "Han Solo Permanently Frozen".into(),
            conditions: vec![
                EventCondition::EventFired { id: EVT_HAN_CARBONITE_FAIL_5 },
                EventCondition::CharacterIsCaptive { character: han },
                EventCondition::EventNotFired { id: 0x383 }, // rescue not completed
            ],
            actions: vec![EventAction::DisplayMessage {
                text: "Han Solo's carbonite prison has become permanent. He can no longer be rescued.".into(),
            }],
            is_repeatable: false,
            enabled: true,
            system_tag: SystemTag::Story,
        });
    }

    // -----------------------------------------------------------------------
    // System notification events (non-character-specific)
    // Registered from original game event IDs found in community disassembly.
    // These fire based on game state conditions, not specific characters.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // NOTE: 8 random notification events REMOVED (Phase 3b, Knesset Shamash).
    //
    // The original game generates incidents (uprising 0x152, informant 0x153,
    // disaster 0x154, resource 0x155) via state-transition logic in the economy
    // tick (FUN_0050a970), NOT random per-tick probability. Bits 16-19 of the
    // system's field_0x88 are compared against neutral galaxy state; incidents
    // fire only when these bits differ (state change).
    //
    // Maintenance shortfall fires every GNPRTB[7694]=30 ticks via event timer.
    // Saboteur/traitor are triggered by mission/loyalty systems, not random events.
    //
    // These will be wired into the economy tick in Phase 3b implementation.
    // -----------------------------------------------------------------------
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
    use crate::world::{Character, Fleet, ForceTier, SkillPair};

    fn tick(n: u64) -> Vec<TickEvent> {
        vec![TickEvent { tick: n }]
    }

    fn make_character(name: &str, force_tier: ForceTier, is_alliance: bool) -> Character {
        Character {
            name: name.into(),
            is_alliance,
            is_empire: !is_alliance,
            is_major: true,
            force_tier,
            ..Default::default()
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

    /// Assign Han Solo to a fleet so `CharacterAssignedToFleet` passes.
    /// Creates a system and fleet if the world doesn't already have one.
    fn assign_han_to_fleet(world: &mut GameWorld) {
        let han_key = find_character(world, "Han").expect("Han must exist");
        let sys_key = if let Some((k, _)) = world.systems.iter().next() {
            k
        } else {
            let sector_key = world.sectors.insert(crate::world::Sector {
                dat_id: crate::ids::DatId::new(0),
                name: "HanSector".into(),
                group: crate::dat::SectorGroup::Core,
                x: 0, y: 0, systems: vec![],
            });
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(0),
                name: "HanSystem".into(),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.5, popularity_empire: 0.5,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Uncontrolled,
            })
        };
        let fleet_key = world.fleets.insert(Fleet {
            location: sys_key,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![han_key],
            is_alliance: true,
            has_death_star: false,
        });
        world.characters.get_mut(han_key).unwrap().current_fleet = Some(fleet_key);
    }

    #[test]
    fn define_story_events_empty_world_does_not_panic() {
        let world = GameWorld::default();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);
        // No characters found — no story events registered
        // (notification events removed in Phase 3b — they belong in economy tick)
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
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
            system_tag: SystemTag::Story,
        });
        state2.define(GameEvent {
            id: 3,
            name: "not_fired_should_fail".into(),
            conditions: vec![EventCondition::EventNotFired { id: 99 }],
            actions: vec![],
            is_repeatable: false,
            enabled: true,
            system_tag: SystemTag::Story,
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
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                espionage_rating: 0.0,
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

        assign_han_to_fleet(&mut world);
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
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                espionage_rating: 0.0,
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

        assign_han_to_fleet(&mut world);
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

        // EVT_HAN_RESCUE (0x200) — telemetry twin chains off 0x383 same pass
        assert!(
            fired2.iter().any(|f| f.event_id == EVT_HAN_RESCUE),
            "EVT_HAN_RESCUE should chain-fire same pass as 0x383"
        );

        // EVT_HAN_RESCUE is one-shot — must not re-fire on subsequent ticks
        let fired3 = EventSystem::advance(&mut state, &world, &tick(102), &[]);
        assert!(
            !fired3.iter().any(|f| f.event_id == EVT_HAN_RESCUE),
            "EVT_HAN_RESCUE should not fire again (is_repeatable: false)"
        );

        // Luke capture (0x399) should NOT fire after rescue (0x39A) completed
        let fired4 = EventSystem::advance(&mut state, &world, &tick(115), &[]);
        assert!(
            !fired4.iter().any(|f| f.event_id == 0x399),
            "Luke should NOT be captured after palace rescue complete (0x39A guards 0x399)"
        );
    }

    // -----------------------------------------------------------------------
    // Phase 3 tests: Jabba 5-case outcomes, Emperor, Leia Force
    // -----------------------------------------------------------------------

    #[test]
    fn han_self_escape_fires_with_random_roll() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();

        // Need Empire systems for bounty hunter gate
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }

        assign_han_to_fleet(&mut world);
        define_story_events(&mut state, &world);

        // Fire bounty gate + attack
        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]); // bounty attack fires

        // Self-escape (0x384) needs tick >= 135, no rescue, no capture, random < 0.10
        let fired = EventSystem::advance(&mut state, &world, &tick(135), &[0.01, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x384),
            "Han self-escape should fire at tick 135 with low roll. Events: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn evt_jabba_prisoners_fires_on_luke_capture() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();

        // 60 Empire systems for bounty hunter gate
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }
        assign_han_to_fleet(&mut world);
        define_story_events(&mut state, &world);

        // Fire bounty gate (80) + attack (100)
        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);

        // Skip tick 101 (so 0x380 hasn't fired yet), jump to 115.
        // At tick 115: 0x380 fires, then 0x399 (before 0x383 in source order)
        // fires if random passes, blocking rescue.
        // rolls: [0]=0.05 for 0x399 (< 0.20), [1]=0.99 for 0x385, [2]=0.99 for 0x387
        let fired = EventSystem::advance(&mut state, &world, &tick(115), &[0.05, 0.99, 0.99]);

        assert!(
            fired.iter().any(|f| f.event_id == 0x399),
            "Luke should be captured at tick 115. Fired: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );
        assert!(
            fired.iter().any(|f| f.event_id == EVT_JABBA_PRISONERS),
            "EVT_JABBA_PRISONERS should chain-fire same pass as 0x399"
        );
        // Self-guard: only one consolidator fires even though 3 variants are registered
        assert_eq!(
            fired.iter().filter(|f| f.event_id == EVT_JABBA_PRISONERS).count(),
            1,
            "Self-guard should prevent duplicate consolidator fires"
        );
    }

    #[test]
    fn evt_jabba_prisoners_self_guard_prevents_refire() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();

        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }
        assign_han_to_fleet(&mut world);
        define_story_events(&mut state, &world);

        // Fire bounty + capture Luke
        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);
        let fired = EventSystem::advance(&mut state, &world, &tick(115), &[0.05, 0.99, 0.99]);
        assert!(fired.iter().any(|f| f.event_id == EVT_JABBA_PRISONERS));

        // Next tick — consolidator must not re-fire
        // Provide rolls for remaining enabled Random events (0x385, 0x387)
        let fired2 = EventSystem::advance(&mut state, &world, &tick(116), &[0.99, 0.99]);
        assert!(
            !fired2.iter().any(|f| f.event_id == EVT_JABBA_PRISONERS),
            "EVT_JABBA_PRISONERS should not fire again (self-guard + is_repeatable: false)"
        );
    }

    #[test]
    fn evt_jabba_prisoners_fires_on_leia_capture_only() {
        // Regression: define_or_branch required so the 0x385 trigger variant
        // survives alongside the 0x399 variant (define() replaces same-ID).
        // World: Han + Leia only (no Luke, so rescue 0x383 can't fire —
        // it requires luke.is_some()). Leia capture can proceed.
        let mut world = GameWorld::default();
        world.characters.insert(make_character("Han Solo", ForceTier::None, true));
        world.characters.insert(make_character("Princess Leia", ForceTier::None, true));

        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }
        assign_han_to_fleet(&mut world);
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Fire bounty gate (80) + attack (100)
        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);

        // At tick 108: Leia captured (0x385, roll 0.05 < 0.15).
        // No Luke → no 0x380/0x383 rescue chain.
        let fired = EventSystem::advance(&mut state, &world, &tick(108), &[0.05]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x385),
            "Leia should be captured. Events: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );
        assert!(
            fired.iter().any(|f| f.event_id == EVT_JABBA_PRISONERS),
            "EVT_JABBA_PRISONERS should fire on Leia capture alone (regression: define_or_branch)"
        );
    }

    #[test]
    fn evt_jabba_prisoners_two_captures_same_tick_one_consolidator() {
        // When two captures fire in the same tick, exactly one consolidator fires.
        // At tick 115: 0x399 (Luke, source-order before rescue) fires first,
        // blocking 0x383. Then 0x385 (Leia) also fires (rescue blocked).
        // Self-guard ensures only 1 EVT_JABBA_PRISONERS despite 2 captures.
        let mut world = make_world_with_characters();
        let mut state = EventState::new();

        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }
        assign_han_to_fleet(&mut world);
        define_story_events(&mut state, &world);

        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);

        // rolls: 0x399 passes (0.05 < 0.20), 0x385 passes (0.05 < 0.15),
        // 0x387 fails (0.99 > 0.12)
        let fired = EventSystem::advance(&mut state, &world, &tick(115), &[0.05, 0.05, 0.99]);

        let captures: Vec<_> = fired.iter()
            .filter(|f| f.event_id == 0x399 || f.event_id == 0x385)
            .collect();
        assert!(
            captures.len() >= 2,
            "Both Luke (0x399) and Leia (0x385) should be captured. Fired: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );
        assert_eq!(
            fired.iter().filter(|f| f.event_id == EVT_JABBA_PRISONERS).count(),
            1,
            "Self-guard: exactly one consolidator despite two captures"
        );
    }

    // -----------------------------------------------------------------------
    // R3: EVT_HAN_PERMANENT_FREEZE (0x39B) — carbonite countdown tests
    // -----------------------------------------------------------------------

    /// Helper: create a world with 60 Empire systems and run bounty gate + attack.
    /// Returns the world and state ready for fail-chain testing.
    fn setup_bounty_chain(include_luke: bool) -> (GameWorld, EventState) {
        let mut world = GameWorld::default();
        let mut han = make_character("Han Solo", ForceTier::None, true);
        han.is_captive = false;
        world.characters.insert(han);
        if include_luke {
            world.characters.insert(make_character("Luke Skywalker", ForceTier::Aware, true));
            world.characters.insert(make_character("Princess Leia", ForceTier::None, true));
            world.characters.insert(make_character("Chewbacca", ForceTier::None, true));
        }
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }
        assign_han_to_fleet(&mut world);
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Fire bounty gate (80) + attack (100) → 0x398 carbonite chains
        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);

        // Simulate integrator: mark Han as captive (SetCarboniteState sets is_captive)
        let han_key = find_character(&world, "Han").unwrap();
        world.characters.get_mut(han_key).unwrap().is_captive = true;

        (world, state)
    }

    #[test]
    fn evt_permanent_freeze_fires_after_countdown() {
        // Without Luke, 0x384 (self-escape) isn't defined and rescue can't succeed.
        // The fail chain fires deterministically. At tick 205, all 5 fail stages
        // chain in a single pass (all tick thresholds met), then 0x39B fires.
        let (world, mut state) = setup_bounty_chain(false);

        // At tick 205: entire fail chain + permanent freeze chains in one pass
        let fired = EventSystem::advance(&mut state, &world, &tick(205), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_HAN_CARBONITE_FAIL_1),
            "FAIL_1 should fire. Events: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );
        assert!(
            fired.iter().any(|f| f.event_id == EVT_HAN_CARBONITE_FAIL_5),
            "FAIL_5 should fire"
        );
        assert!(
            fired.iter().any(|f| f.event_id == EVT_HAN_PERMANENT_FREEZE),
            "EVT_HAN_PERMANENT_FREEZE should chain off FAIL_5"
        );
    }

    #[test]
    fn evt_permanent_freeze_blocked_by_self_escape() {
        // Han + Luke (no Leia, so rescue 0x383 can't fire — it's inside the
        // Leia block). Self-escape (0x384) lives in the Luke+Han block.
        let mut world = GameWorld::default();
        world.characters.insert(make_character("Han Solo", ForceTier::None, true));
        world.characters.insert(make_character("Luke Skywalker", ForceTier::Aware, true));
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..60 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }
        assign_han_to_fleet(&mut world);
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Fire bounty gate + attack
        EventSystem::advance(&mut state, &world, &tick(80), &[]);
        EventSystem::advance(&mut state, &world, &tick(100), &[0.05]);

        // At tick 135: 0x380 fires first, then 0x384 (self-escape, roll 0.01 < 0.10).
        // 0x399 also evaluable but its Random(0.20) gets roll 0.99 → fails.
        let fired = EventSystem::advance(&mut state, &world, &tick(135), &[0.01, 0.99]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x384),
            "Han self-escape should fire. Events: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );

        // At tick 205: fail chain should NOT fire (self-escape already fired)
        let fired2 = EventSystem::advance(&mut state, &world, &tick(205), &[]);
        assert!(
            !fired2.iter().any(|f| f.event_id == EVT_HAN_PERMANENT_FREEZE),
            "Permanent freeze should be blocked by self-escape"
        );
    }

    #[test]
    fn evt_permanent_freeze_blocked_by_rescue() {
        // With Luke + Leia, rescue (0x383) can fire. Once it does,
        // EventNotFired(0x383) guards halt the fail chain.
        let (world, mut state) = setup_bounty_chain(true);

        // tick 101: 0x380 fires, then 0x383 chains (rescue succeeds)
        let fired = EventSystem::advance(&mut state, &world, &tick(101), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == 0x383),
            "Rescue should fire. Events: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );

        // At tick 205: fail chain should NOT fire (rescue already fired)
        let fired2 = EventSystem::advance(&mut state, &world, &tick(205), &[]);
        assert!(
            !fired2.iter().any(|f| f.event_id == EVT_HAN_PERMANENT_FREEZE),
            "Permanent freeze should be blocked by rescue"
        );
    }

    #[test]
    fn evt_permanent_freeze_does_not_refire() {
        let (world, mut state) = setup_bounty_chain(false);

        // Fire the full chain at tick 205
        let fired = EventSystem::advance(&mut state, &world, &tick(205), &[]);
        assert!(fired.iter().any(|f| f.event_id == EVT_HAN_PERMANENT_FREEZE));

        // Next tick: should not fire again (is_repeatable: false)
        let fired2 = EventSystem::advance(&mut state, &world, &tick(206), &[]);
        assert!(
            !fired2.iter().any(|f| f.event_id == EVT_HAN_PERMANENT_FREEZE),
            "Permanent freeze should not fire again (one-shot)"
        );
    }

    #[test]
    fn leia_force_discovery_fires_after_dagobah() {
        let mut world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Advance through Dagobah prerequisite and training
        EventSystem::advance(&mut state, &world, &tick(30), &[]);
        EventSystem::advance(&mut state, &world, &tick(50), &[]);

        // Complete Dagobah by making Luke Experienced
        let luke_key = find_character(&world, "Luke").unwrap();
        world.characters.get_mut(luke_key).unwrap().force_tier = ForceTier::Experienced;

        // Dagobah completed fires
        EventSystem::advance(&mut state, &world, &tick(60), &[]);

        // Leia Force discovery (0x362) fires at tick 80 after Dagobah complete
        let fired = EventSystem::advance(&mut state, &world, &tick(80), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_LEIA_FORCE),
            "Leia Force discovery should fire after Dagobah completion. Events: {:?}",
            fired.iter().map(|f| (f.event_id, &f.event_name)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn leia_force_does_not_fire_before_dagobah() {
        let world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // At tick 80 without Dagobah completion — should NOT fire
        let fired = EventSystem::advance(&mut state, &world, &tick(80), &[]);
        assert!(
            !fired.iter().any(|f| f.event_id == EVT_LEIA_FORCE),
            "Leia Force should NOT fire before Dagobah completion"
        );
    }

    // notification_events_fire_with_random_rolls — REMOVED (Phase 3b)
    // notification_events_need_correct_tick — REMOVED (Phase 3b)
    // Notification events moved to economy tick state-transition model.

    #[test]
    fn emperor_palpatine_adds_emperor_specific_events() {
        let mut world = make_world_with_characters();
        world.characters.insert(make_character("Emperor Palpatine", ForceTier::Experienced, false));

        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        // Verify Emperor Arrival event (0x230) is registered
        assert!(
            state.events().iter().any(|e| e.id == EVT_EMPEROR_ARRIVAL),
            "EVT_EMPEROR_ARRIVAL should be registered when Emperor exists"
        );

        // Verify Final Battle event (0x220) has CharactersCoLocated condition
        let final_battle = state.events().iter().find(|e| e.id == EVT_FINAL_BATTLE);
        assert!(final_battle.is_some(), "Final Battle should be registered");
        let fb = final_battle.unwrap();
        let has_co_loc = fb.conditions.iter().any(|c| matches!(c, EventCondition::CharactersCoLocated { .. }));
        assert!(has_co_loc, "Final Battle should have CharactersCoLocated when Emperor present");
    }

    #[test]
    fn emperor_arrival_fires_with_empire_dominance() {
        let mut world = make_world_with_characters();
        world.characters.insert(make_character("Emperor Palpatine", ForceTier::Experienced, false));

        // Need 100+ Empire systems
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        for i in 0..110 {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(i),
                name: format!("System_{}", i),
                sector: sector_key, x: 0, y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.3, popularity_empire: 0.7,
                is_populated: true, total_energy: 0, raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![], ground_units: vec![], special_forces: vec![],
                defense_facilities: vec![], manufacturing_facilities: vec![],
                production_facilities: vec![], is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(crate::dat::Faction::Empire),
            });
        }

        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        let fired = EventSystem::advance(&mut state, &world, &tick(90), &[]);
        assert!(
            fired.iter().any(|f| f.event_id == EVT_EMPEROR_ARRIVAL),
            "Emperor Arrival should fire at tick 90 with 110 Empire systems"
        );
    }

    /// CI guard: notification events must NEVER use EventCondition::Random.
    ///
    /// The original game generates these on state transitions (bit-difference
    /// comparison against galaxy state), not random per-tick probability.
    /// See the Phase 3b (Knesset Shamash) comment above define_story_events.
    #[test]
    fn notification_events_never_use_random() {
        let banned_ids: &[u32] = &[
            EVT_SUPPORT_CHANGE,            // 0x100
            EVT_INFORMANT_INTEL,           // 0x153
            EVT_NATURAL_DISASTER,          // 0x154
            EVT_RESOURCE_DISCOVERY,        // 0x155
            EVT_MANUFACTURING_IDLE,        // 0x160
            EVT_MAINTENANCE_SHORTFALL_EVENT, // 0x304
            EVT_SABOTEUR_DETECTED,         // 0x305
            EVT_TRAITOR_REVEALED,          // 0x361
        ];

        let world = make_world_with_characters();
        let mut state = EventState::new();
        define_story_events(&mut state, &world);

        for event in state.events() {
            if banned_ids.contains(&event.id) {
                for cond in &event.conditions {
                    assert!(
                        !matches!(cond, EventCondition::Random { .. }),
                        "Event 0x{:x} ({}) must not use EventCondition::Random — \
                         notification events fire on state transitions, not random rolls. \
                         See story_events.rs Phase 3b comment.",
                        event.id,
                        event.name
                    );
                }
            }
        }
    }
}
