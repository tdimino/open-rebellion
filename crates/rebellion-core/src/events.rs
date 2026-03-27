//! Event system: conditional triggers and scripted actions.
//!
//! Events fire when all their conditions are satisfied. Most events are
//! one-shot (a character's story beat, a scripted reveal); some are recurring
//! (natural disasters, random bounty hunter spawns).
//!
//! # Architecture
//!
//! Follows the same stateless pattern as manufacturing and missions:
//! - `EventState` holds all defined events and the set of already-fired IDs.
//! - `EventSystem::advance(state, world, tick_events, rng_rolls)` evaluates
//!   every enabled event against the current game state and returns
//!   `Vec<FiredEvent>` — one entry per event that triggered this frame.
//! - The **caller** applies `FiredEvent::actions` to `GameWorld` / UI.
//! - `rebellion-core` stays headless: no IO, no random state, no mutation of
//!   `GameWorld` inside this module.
//!
//! # Random conditions
//!
//! `EventCondition::Random { probability }` consumes one value from the
//! caller-supplied `rng_rolls` slice (in event-definition order). If there
//! are fewer rolls than `Random` conditions the roll defaults to `1.0`
//! (never fires). Pass pre-generated rolls from a deterministic RNG (e.g.
//! `rand::Rng::gen::<f32>()`) so tests can control outcomes exactly.
//!
//! # Usage
//!
//! ```ignore
//! use rebellion_core::events::{
//!     EventCondition, EventAction, EventState, EventSystem, GameEvent,
//! };
//! use rebellion_core::tick::TickEvent;
//! use rebellion_core::world::GameWorld;
//!
//! let mut state = EventState::new();
//! // Define events at game-start or load from JSON…
//! state.define(GameEvent {
//!     id: 1,
//!     name: "tick_threshold".into(),
//!     conditions: vec![EventCondition::TickAtLeast { tick: 10 }],
//!     actions: vec![EventAction::DisplayMessage {
//!         text: "Ten days have passed.".into(),
//!     }],
//!     is_repeatable: false,
//!     enabled: true,
//! });
//!
//! let world = GameWorld::default();
//! let tick_events = vec![TickEvent { tick: 10 }];
//! let fired = EventSystem::advance(&mut state, &world, &tick_events, &[]);
//! assert_eq!(fired.len(), 1);
//! ```

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{CharacterKey, SystemKey};
use crate::tick::TickEvent;
use crate::world::{ForceTier, GameWorld};

// ---------------------------------------------------------------------------
// Ghidra RE event IDs (from annotated-functions.md)
// ---------------------------------------------------------------------------

pub const EVT_RECRUITMENT_DONE: u32 = 0x12c; // 300
pub const EVT_SYSTEM_BATTLE: u32 = 0x14d; // 333
pub const EVT_SYSTEM_BLOCKADE: u32 = 0x14e; // 334
pub const EVT_UPRISING_INCIDENT: u32 = 0x152; // 338
pub const EVT_FLEET_BATTLE: u32 = 0x180; // 384
pub const EVT_CHARACTER_FORCE: u32 = 0x1e1; // 481
pub const EVT_FORCE_TRAINING: u32 = 0x1e5; // 485
pub const EVT_DAGOBAH_COMPLETED: u32 = 0x210; // 528
pub const EVT_BOUNTY_ATTACK: u32 = 0x212; // 530
pub const EVT_FINAL_BATTLE: u32 = 0x220; // 544
pub const EVT_LUKE_DAGOBAH: u32 = 0x221; // 545
pub const EVT_GAME_OBJ_DESTROYED: u32 = 0x302; // 770
pub const EVT_TROOP_BLOCKADE_KILL: u32 = 0x340; // 832
pub const EVT_FORCE_DISCOVERED: u32 = 0x362; // 866
pub const EVT_ESPIONAGE_EXTRA: u32 = 0x370; // 880

// Phase 3 additions — from community disassembly cross-reference
pub const EVT_SUPPORT_CHANGE: u32 = 0x100; // 256 — popular support shift
pub const EVT_FLEET_ARRIVE: u32 = 0x105; // 261 — fleet arrived at system
pub const EVT_CHARACTER_HEALTH: u32 = 0x106; // 262 — character health change
pub const EVT_UNITS_DEPLOYED: u32 = 0x107; // 263 — units deployed to system
pub const EVT_HQ_CAPTURED: u32 = 0x128; // 296 — headquarters captured
pub const EVT_INFORMANT_INTEL: u32 = 0x153; // 339 — informant intelligence
pub const EVT_RESOURCE_DISCOVERY: u32 = 0x154; // 340 — resource discovery
pub const EVT_NATURAL_DISASTER: u32 = 0x155; // 341 — natural disaster
pub const EVT_MANUFACTURING_IDLE: u32 = 0x160; // 352 — manufacturing queue empty
pub const EVT_HAN_RESCUE: u32 = 0x200; // 512 — Han Solo rescue variant
pub const EVT_EMPEROR_ARRIVAL: u32 = 0x230; // 560 — Emperor arrives at system
pub const EVT_JABBA_PRISONERS: u32 = 0x231; // 561 — Jabba's prisoner events
pub const EVT_MAINTENANCE_SHORTFALL_EVENT: u32 = 0x304; // 772 — maintenance budget shortfall
pub const EVT_SABOTEUR_DETECTED: u32 = 0x305; // 773 — saboteur detected
pub const EVT_CHARACTER_KILLED: u32 = 0x306; // 774 — character killed in action
pub const EVT_TRAITOR_REVEALED: u32 = 0x361; // 865 — traitor revealed
pub const EVT_LEIA_FORCE: u32 = 0x363; // 867 — Leia-specific Force discovery (distinct from 0x362 generic)
pub const EVT_SIDE_CHANGE: u32 = 0x386; // 902 — character changes faction
pub const EVT_JABBA_CAPTURES_CHEWIE: u32 = 0x387; // 903 — Chewie captured at Jabba's palace

// ---------------------------------------------------------------------------
// EventCondition
// ---------------------------------------------------------------------------

/// A single predicate evaluated against the current game state.
///
/// `GameEvent::conditions` is evaluated with AND logic: every condition must
/// return `true` for the event to fire. For OR / NOT semantics, define
/// multiple events or extend this enum with composite variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCondition {
    /// Fires exactly once when the tick counter equals `tick`.
    TickReached {
        tick: u64,
    },

    /// Fires on every tick at or after `tick` (useful with `is_repeatable`).
    TickAtLeast {
        tick: u64,
    },

    /// Fires when `character` is aboard a fleet located at `system`.
    ///
    /// Evaluated by scanning `world.systems[system].fleets` and their
    /// `characters` lists. Returns `false` if either key is stale/invalid.
    CharacterAtSystem {
        character: CharacterKey,
        system: SystemKey,
    },

    /// Fires with the given probability (0.0–1.0) on each evaluation.
    ///
    /// Consumes one value from the `rng_rolls` slice. If no roll is available
    /// the condition always fails (conservative default).
    Random {
        /// Probability per evaluation that this condition is satisfied.
        probability: f32,
    },

    /// Fires only after event `id` has already been fired at least once.
    ///
    /// Useful for event chains (e.g., "Bounty hunter arrives after Luke event").
    EventFired {
        id: u32,
    },

    /// Character's Force tier is at least `min_tier`.
    CharacterHasForceLevel {
        character: CharacterKey,
        min_tier: ForceTier,
    },

    /// A specific faction controls a system.
    FactionControlsSystem {
        faction: Faction,
        system: SystemKey,
    },

    /// Character is a Force user (any tier above None).
    CharacterIsForceUser {
        character: CharacterKey,
    },

    /// Character exists in the world (not killed/removed).
    CharacterExists {
        character: CharacterKey,
    },

    /// Character is on a mandatory mission (unavailable for player assignment).
    CharacterOnMandatoryMission {
        character: CharacterKey,
    },

    /// A specific number of systems are controlled by a faction.
    FactionControlsNSystems {
        faction: Faction,
        min_count: usize,
    },

    /// Character's accumulated Force XP meets threshold.
    CharacterForceExperience {
        character: CharacterKey,
        min_xp: u32,
    },

    /// Character is currently held captive.
    CharacterIsCaptive {
        character: CharacterKey,
    },

    /// Character is a Jedi trainer (can teach others).
    CharacterIsJediTrainer {
        character: CharacterKey,
    },

    /// A specific event has NOT yet fired (inverse of EventFired).
    EventNotFired {
        id: u32,
    },

    /// All listed characters are at the same system (any system).
    ///
    /// Evaluated by finding any system where ALL characters are present
    /// in its fleet rosters. Returns `false` if any character doesn't exist
    /// or if no single system contains all of them.
    CharactersCoLocated {
        characters: Vec<CharacterKey>,
    },
}

// ---------------------------------------------------------------------------
// EventAction
// ---------------------------------------------------------------------------

/// An effect to apply when an event fires.
///
/// Actions are data — the caller (rebellion-app or a test harness) interprets
/// and applies them to `GameWorld`, the UI, and any other state. This keeps
/// `rebellion-core` free of rendering and IO dependencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventAction {
    /// Shift a system's Alliance/Empire popularity values.
    ///
    /// Deltas are clamped to [0.0, 1.0] by the caller after application.
    ShiftPopularity {
        system: SystemKey,
        /// Delta to apply to `popularity_alliance` (negative = decrease).
        alliance_delta: f32,
        /// Delta to apply to `popularity_empire` (negative = decrease).
        empire_delta: f32,
    },

    /// Emit a message to the player's message log.
    DisplayMessage {
        text: String,
    },

    /// Adjust a character's base skill scores.
    ///
    /// Deltas are signed: positive boosts, negative penalties.
    /// The caller is responsible for clamping final values to valid ranges.
    ModifyCharacterSkill {
        character: CharacterKey,
        /// Skill field to modify. See `SkillField` enum.
        skill: SkillField,
        /// Change to apply to the `base` of the chosen skill pair.
        base_delta: i32,
    },

    /// Mark a character as belonging to a faction and place them at a system.
    ///
    /// Used for events that reveal or relocate a character (e.g., Yoda on Dagobah).
    /// The caller creates or retrieves the `CharacterKey` from `GameWorld`.
    RelocateCharacter {
        character: CharacterKey,
        destination: SystemKey,
    },

    /// Set a character's mandatory mission flag.
    SetMandatoryMission {
        character: CharacterKey,
        mandatory: bool,
    },

    /// Change a character's Force tier.
    ModifyForceTier {
        character: CharacterKey,
        new_tier: ForceTier,
    },

    /// Remove a character from the game.
    RemoveCharacter {
        character: CharacterKey,
    },

    /// Start Jedi training for a character.
    StartJediTraining {
        character: CharacterKey,
    },

    /// Transfer a character to a system and optionally change faction.
    TransferCharacter {
        character: CharacterKey,
        destination: SystemKey,
        new_faction: Option<Faction>,
    },

    /// Fire another event by ID (for chaining story beats).
    TriggerEvent {
        event_id: u32,
    },

    /// Add Force experience points to a character.
    AccumulateForceExperience {
        character: CharacterKey,
        amount: u32,
    },

    /// Put a character into captive/carbonite state.
    CaptureCharacter {
        character: CharacterKey,
        captor_faction: Faction,
    },

    /// Mark a character as in carbonite (can't be assigned to fleets or missions).
    SetCarboniteState {
        character: CharacterKey,
        frozen: bool,
    },
}

/// Which skill pair `ModifyCharacterSkill` targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillField {
    Diplomacy,
    Espionage,
    ShipDesign,
    TroopTraining,
    FacilityDesign,
    Combat,
    Leadership,
    Loyalty,
    JediLevel,
}

// ---------------------------------------------------------------------------
// GameEvent
// ---------------------------------------------------------------------------

/// A scripted game event: a set of conditions plus a set of actions.
///
/// All conditions must be true simultaneously for the event to fire.
/// One-shot events (`is_repeatable = false`) are disabled after firing
/// and will not fire again even if conditions are re-satisfied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    /// Unique event identifier. Used for `EventFired` conditions and
    /// cross-references from mod data.
    pub id: u32,

    /// Human-readable name (for logs and mod tooling).
    pub name: String,

    /// Conditions evaluated with AND logic. Empty conditions list always fires.
    pub conditions: Vec<EventCondition>,

    /// Actions to perform when the event fires. Passed back to caller via
    /// `FiredEvent::actions`.
    pub actions: Vec<EventAction>,

    /// If `true`, the event can fire on multiple ticks as long as conditions
    /// remain satisfied. If `false`, it fires at most once and is then
    /// disabled.
    pub is_repeatable: bool,

    /// Whether this event participates in evaluation. Set to `false` to
    /// disable without removing (used after one-shot events fire, or to
    /// support conditional mod enable/disable).
    pub enabled: bool,
}

// ---------------------------------------------------------------------------
// EventState
// ---------------------------------------------------------------------------

/// All defined events plus the set of already-fired one-shot IDs.
///
/// Lives alongside `GameWorld` and `ManufacturingState` — not inside either.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventState {
    events: Vec<GameEvent>,
    /// IDs of one-shot events that have already fired.
    fired_ids: HashSet<u32>,
}

impl EventState {
    pub fn new() -> Self {
        EventState {
            events: Vec::new(),
            fired_ids: HashSet::new(),
        }
    }

    /// Add an event to the state. Replaces any existing event with the same id.
    pub fn define(&mut self, event: GameEvent) {
        if let Some(existing) = self.events.iter_mut().find(|e| e.id == event.id) {
            *existing = event;
        } else {
            self.events.push(event);
        }
    }

    /// Whether event `id` has ever been fired.
    pub fn has_fired(&self, id: u32) -> bool {
        self.fired_ids.contains(&id)
    }

    /// All events (read-only, for inspection or serialization).
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
}

// ---------------------------------------------------------------------------
// FiredEvent
// ---------------------------------------------------------------------------

/// Emitted when a `GameEvent` fires.
///
/// The caller reads `actions` and applies them to game state, UI, and logs.
#[derive(Debug, Clone)]
pub struct FiredEvent {
    /// The ID of the event that fired.
    pub event_id: u32,
    /// Name of the event (for log messages).
    pub event_name: String,
    /// The tick on which this event fired.
    pub tick: u64,
    /// The actions to execute. Cloned from the event definition.
    pub actions: Vec<EventAction>,
}

// ---------------------------------------------------------------------------
// EventSystem
// ---------------------------------------------------------------------------

/// Stateless system that evaluates events each tick.
///
/// Call `advance` once per frame, passing the `Vec<TickEvent>` from
/// `GameClock::advance` and pre-generated random rolls.
pub struct EventSystem;

impl EventSystem {
    /// Evaluate all enabled events against the current game state.
    ///
    /// Returns a `Vec<FiredEvent>` — one entry per event that triggered.
    /// Events are evaluated once per `advance` call (not once per tick),
    /// so rapid ticks within a single frame don't cause double-fires.
    ///
    /// # Arguments
    ///
    /// - `state`: mutable event state — fired one-shot events are disabled
    /// - `world`: current `GameWorld` for world-query conditions
    /// - `tick_events`: ticks that elapsed this frame (from `GameClock::advance`)
    /// - `rng_rolls`: pre-generated `f32` rolls in `[0.0, 1.0)` for `Random`
    ///   conditions. Consumed in event-definition order, one per `Random`
    ///   condition encountered.
    pub fn advance(
        state: &mut EventState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        rng_rolls: &[f32],
    ) -> Vec<FiredEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        // The current tick is the last tick that completed this frame.
        let current_tick = tick_events.last().unwrap().tick;

        let mut fired = Vec::new();
        let mut rng_cursor = 0usize;

        // We need to update fired_ids inline so that EventFired conditions on
        // later events in the same advance() call can chain off earlier ones.
        // Borrow splitting: collect (index, conditions_met) first, then mutate.
        let n = state.events.len();
        for i in 0..n {
            if !state.events[i].enabled {
                continue;
            }

            let conditions_met = evaluate_conditions(
                &state.events[i].conditions,
                world,
                current_tick,
                &state.fired_ids,
                rng_rolls,
                &mut rng_cursor,
            );

            if conditions_met {
                let ev = &mut state.events[i];
                let is_repeatable = ev.is_repeatable;
                fired.push(FiredEvent {
                    event_id: ev.id,
                    event_name: ev.name.clone(),
                    tick: current_tick,
                    actions: ev.actions.clone(),
                });

                if !is_repeatable {
                    ev.enabled = false;
                    // Record immediately so later events in this pass see it.
                    let id = ev.id;
                    state.fired_ids.insert(id);
                }
            }
        }

        fired
    }
}

// ---------------------------------------------------------------------------
// Condition evaluation (pure function, no mutation)
// ---------------------------------------------------------------------------

fn evaluate_conditions(
    conditions: &[EventCondition],
    world: &GameWorld,
    current_tick: u64,
    fired_ids: &HashSet<u32>,
    rng_rolls: &[f32],
    rng_cursor: &mut usize,
) -> bool {
    for condition in conditions {
        let met = evaluate_condition(
            condition,
            world,
            current_tick,
            fired_ids,
            rng_rolls,
            rng_cursor,
        );
        if !met {
            return false; // AND logic: one failure short-circuits
        }
    }
    true // empty condition list always fires
}

fn evaluate_condition(
    condition: &EventCondition,
    world: &GameWorld,
    current_tick: u64,
    fired_ids: &HashSet<u32>,
    rng_rolls: &[f32],
    rng_cursor: &mut usize,
) -> bool {
    match condition {
        EventCondition::TickReached { tick } => current_tick == *tick,

        EventCondition::TickAtLeast { tick } => current_tick >= *tick,

        EventCondition::CharacterAtSystem { character, system } => {
            character_is_at_system(world, *character, *system)
        }

        EventCondition::Random { probability } => {
            let roll = rng_rolls.get(*rng_cursor).copied().unwrap_or(1.0);
            *rng_cursor += 1;
            roll < *probability
        }

        EventCondition::EventFired { id } => fired_ids.contains(id),

        EventCondition::CharacterHasForceLevel {
            character,
            min_tier,
        } => world
            .characters
            .get(*character)
            .map(|c| c.force_tier >= *min_tier)
            .unwrap_or(false),

        EventCondition::FactionControlsSystem { faction, system } => world
            .systems
            .get(*system)
            .map(|s| s.control.is_controlled_by(*faction))
            .unwrap_or(false),

        EventCondition::CharacterIsForceUser { character } => world
            .characters
            .get(*character)
            .map(|c| c.force_tier > ForceTier::None)
            .unwrap_or(false),

        EventCondition::CharacterExists { character } => {
            world.characters.contains_key(*character)
        }

        EventCondition::CharacterOnMandatoryMission { character } => world
            .characters
            .get(*character)
            .map(|c| c.on_mandatory_mission)
            .unwrap_or(false),

        EventCondition::FactionControlsNSystems {
            faction,
            min_count,
        } => {
            let count = world
                .systems
                .values()
                .filter(|s| s.control.is_controlled_by(*faction))
                .count();
            count >= *min_count
        }

        EventCondition::CharacterForceExperience {
            character,
            min_xp,
        } => world
            .characters
            .get(*character)
            .map(|c| c.force_experience >= *min_xp)
            .unwrap_or(false),

        EventCondition::CharacterIsCaptive { character } => world
            .characters
            .get(*character)
            .map(|c| c.is_captive)
            .unwrap_or(false),

        EventCondition::CharacterIsJediTrainer { character } => world
            .characters
            .get(*character)
            .map(|c| c.is_jedi_trainer)
            .unwrap_or(false),

        EventCondition::EventNotFired { id } => !fired_ids.contains(id),

        EventCondition::CharactersCoLocated { characters } => {
            if characters.len() < 2 {
                return true; // vacuously true for 0-1 characters
            }
            // Check if all characters exist
            if characters.iter().any(|c| !world.characters.contains_key(*c)) {
                return false;
            }
            // Find any system where all characters are present.
            // A character is "at" a system if:
            //   (a) they are in any fleet's character roster at that system, OR
            //   (b) their current_system field points to that system
            world.systems.iter().any(|(sys_key, sys)| {
                characters.iter().all(|ch| {
                    // Path (a): fleet roster check
                    let in_fleet = sys.fleets.iter().any(|fk| {
                        world.fleets.get(*fk).map_or(false, |f| f.characters.contains(ch))
                    });
                    if in_fleet { return true; }
                    // Path (b): current_system fallback
                    world.characters.get(*ch)
                        .map_or(false, |c| c.current_system == Some(sys_key))
                })
            })
        }
    }
}

/// Returns true when `character` is in any fleet orbiting `system`.
fn character_is_at_system(world: &GameWorld, character: CharacterKey, system: SystemKey) -> bool {
    let sys = match world.systems.get(system) {
        Some(s) => s,
        None => return false,
    };
    for fleet_key in &sys.fleets {
        if let Some(fleet) = world.fleets.get(*fleet_key) {
            if fleet.characters.contains(&character) {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::ControlKind;
    use crate::tick::TickEvent;
    use crate::world::{Character, ForceTier, SkillPair};

    fn make_world() -> GameWorld {
        GameWorld::default()
    }

    /// Create a test character with sensible defaults. Override fields after creation.
    fn make_character(name: &str, force_tier: ForceTier) -> Character {
        let sp = SkillPair { base: 0, variance: 0 };
        Character {
            dat_id: crate::ids::DatId::new(0),
            name: name.into(),
            is_alliance: true,
            is_empire: false,
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

    fn tick(n: u64) -> Vec<TickEvent> {
        vec![TickEvent { tick: n }]
    }

    fn event(id: u32, conditions: Vec<EventCondition>, actions: Vec<EventAction>) -> GameEvent {
        GameEvent {
            id,
            name: format!("event_{}", id),
            conditions,
            actions,
            is_repeatable: false,
            enabled: true,
        }
    }

    fn repeatable_event(
        id: u32,
        conditions: Vec<EventCondition>,
        actions: Vec<EventAction>,
    ) -> GameEvent {
        GameEvent {
            id,
            name: format!("repeat_{}", id),
            conditions,
            actions,
            is_repeatable: true,
            enabled: true,
        }
    }

    // --- Basic firing ---

    #[test]
    fn empty_conditions_always_fires() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(1, vec![], vec![]));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].event_id, 1);
    }

    #[test]
    fn no_ticks_no_fires() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(1, vec![], vec![]));

        let fired = EventSystem::advance(&mut state, &world, &[], &[]);
        assert!(fired.is_empty());
    }

    // --- TickReached ---

    #[test]
    fn tick_reached_fires_exactly_on_target() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::TickReached { tick: 5 }],
            vec![],
        ));

        assert!(EventSystem::advance(&mut state, &world, &tick(4), &[]).is_empty());
        let fired = EventSystem::advance(&mut state, &world, &tick(5), &[]);
        assert_eq!(fired.len(), 1);
        // After firing (one-shot), tick 5 again should not re-fire.
        assert!(EventSystem::advance(&mut state, &world, &tick(5), &[]).is_empty());
    }

    #[test]
    fn tick_reached_does_not_fire_after_target() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::TickReached { tick: 5 }],
            vec![],
        ));

        // Jump directly to tick 6 — already past target, never fires.
        let fired = EventSystem::advance(&mut state, &world, &tick(6), &[]);
        assert!(fired.is_empty());
    }

    // --- TickAtLeast ---

    #[test]
    fn tick_at_least_fires_at_and_after_threshold() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::TickAtLeast { tick: 3 }],
            vec![],
        ));

        assert!(EventSystem::advance(&mut state, &world, &tick(2), &[]).is_empty());
        let fired = EventSystem::advance(&mut state, &world, &tick(3), &[]);
        assert_eq!(fired.len(), 1);
    }

    // --- One-shot vs repeatable ---

    #[test]
    fn one_shot_fires_only_once() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(1, vec![], vec![]));

        EventSystem::advance(&mut state, &world, &tick(1), &[]);
        let second = EventSystem::advance(&mut state, &world, &tick(2), &[]);
        assert!(second.is_empty(), "one-shot should not fire again");
    }

    #[test]
    fn repeatable_fires_every_tick() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(repeatable_event(1, vec![], vec![]));

        let f1 = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        let f2 = EventSystem::advance(&mut state, &world, &tick(2), &[]);
        let f3 = EventSystem::advance(&mut state, &world, &tick(3), &[]);

        assert_eq!(f1.len(), 1);
        assert_eq!(f2.len(), 1);
        assert_eq!(f3.len(), 1);
    }

    // --- AND logic ---

    #[test]
    fn and_logic_requires_all_conditions() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(
            1,
            vec![
                EventCondition::TickAtLeast { tick: 5 },
                EventCondition::TickAtLeast { tick: 10 },
            ],
            vec![],
        ));

        // Tick 7: first condition met, second not.
        assert!(EventSystem::advance(&mut state, &world, &tick(7), &[]).is_empty());
        // Tick 10: both met.
        let fired = EventSystem::advance(&mut state, &world, &tick(10), &[]);
        assert_eq!(fired.len(), 1);
    }

    // --- Random condition ---

    #[test]
    fn random_fires_when_roll_below_probability() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(repeatable_event(
            1,
            vec![EventCondition::Random { probability: 0.5 }],
            vec![],
        ));

        // Roll 0.3 < 0.5 → fires.
        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[0.3]);
        assert_eq!(fired.len(), 1);

        // Roll 0.7 >= 0.5 → does not fire.
        let not_fired = EventSystem::advance(&mut state, &world, &tick(2), &[0.7]);
        assert!(not_fired.is_empty());
    }

    #[test]
    fn random_no_roll_available_does_not_fire() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(repeatable_event(
            1,
            vec![EventCondition::Random { probability: 1.0 }],
            vec![],
        ));

        // No rolls provided — defaults to 1.0 which is not < 1.0 → no fire.
        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty());
    }

    // --- EventFired condition ---

    #[test]
    fn event_fired_condition_chains_events() {
        let world = make_world();
        let mut state = EventState::new();

        // Event 1: fires at tick 5.
        state.define(event(
            1,
            vec![EventCondition::TickReached { tick: 5 }],
            vec![],
        ));

        // Event 2: fires only after event 1 has fired.
        state.define(event(
            2,
            vec![EventCondition::EventFired { id: 1 }],
            vec![],
        ));

        // Tick 4: neither fires.
        assert!(EventSystem::advance(&mut state, &world, &tick(4), &[]).is_empty());

        // Tick 5: event 1 fires; event 2 also evaluates in the same call.
        // Event 1 fires, its id is added to fired_ids, then event 2 is checked.
        let fired = EventSystem::advance(&mut state, &world, &tick(5), &[]);
        // Event 1 fires. Event 2 checks EventFired(1) — which was just added
        // to fired_ids in this same advance call.
        let ids: Vec<u32> = fired.iter().map(|f| f.event_id).collect();
        assert!(ids.contains(&1), "event 1 should fire");
        assert!(ids.contains(&2), "event 2 should chain-fire after event 1");
    }

    // --- Actions carried through ---

    #[test]
    fn fired_event_carries_actions() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(
            1,
            vec![],
            vec![EventAction::DisplayMessage {
                text: "hello".into(),
            }],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1);
        match &fired[0].actions[0] {
            EventAction::DisplayMessage { text } => assert_eq!(text, "hello"),
            _ => panic!("wrong action"),
        }
    }

    // --- Tick number in FiredEvent ---

    #[test]
    fn fired_event_carries_correct_tick() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(event(1, vec![], vec![]));

        let fired = EventSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 3 }, TickEvent { tick: 4 }],
            &[],
        );
        assert_eq!(fired[0].tick, 4); // last tick in the batch
    }

    // --- Disabled event ---

    #[test]
    fn disabled_event_never_fires() {
        let world = make_world();
        let mut state = EventState::new();
        state.define(GameEvent {
            id: 1,
            name: "disabled".into(),
            conditions: vec![],
            actions: vec![],
            is_repeatable: true,
            enabled: false,
        });

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty());
    }

    // --- New condition variants ---

    #[test]
    fn character_has_force_level_true_when_tier_matches() {
        let mut world = make_world();
        let key = world
            .characters
            .insert(make_character("Luke", ForceTier::Training));

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharacterHasForceLevel {
                character: key,
                min_tier: ForceTier::Aware,
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "Training >= Aware should fire");
    }

    #[test]
    fn character_has_force_level_false_when_tier_too_low() {
        let mut world = make_world();
        let key = world
            .characters
            .insert(make_character("Han", ForceTier::None));

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharacterHasForceLevel {
                character: key,
                min_tier: ForceTier::Aware,
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty(), "None < Aware should not fire");
    }

    #[test]
    fn faction_controls_system_evaluates_correctly() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Test Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0),
            name: "Coruscant".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.3,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::FactionControlsSystem {
                faction: Faction::Empire,
                system: sys_key,
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "Empire controls the system");
    }

    #[test]
    fn character_exists_false_for_removed_character() {
        let mut world = make_world();
        let key = world
            .characters
            .insert(make_character("Doomed", ForceTier::None));

        // Remove the character
        world.characters.remove(key);

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharacterExists { character: key }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty(), "removed character should not exist");
    }

    #[test]
    fn faction_controls_n_systems_counts_correctly() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });

        let make_sys = |world: &mut GameWorld, name: &str, faction: Option<Faction>| {
            world.systems.insert(crate::world::System {
                dat_id: crate::ids::DatId::new(0),
                name: name.into(),
                sector: sector_key,
                x: 0,
                y: 0,
                exploration_status: ExplorationStatus::Explored,
                popularity_alliance: 0.5,
                popularity_empire: 0.5,
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                defense_facilities: vec![],
                manufacturing_facilities: vec![],
                production_facilities: vec![],
                is_headquarters: false,
                is_destroyed: false,
                control: faction.map(ControlKind::Controlled).unwrap_or(ControlKind::Uncontrolled),
            })
        };

        make_sys(&mut world, "Sys1", Some(Faction::Alliance));
        make_sys(&mut world, "Sys2", Some(Faction::Alliance));
        make_sys(&mut world, "Sys3", Some(Faction::Empire));

        // Require 2 Alliance systems — should fire
        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::FactionControlsNSystems {
                faction: Faction::Alliance,
                min_count: 2,
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "Alliance controls 2 systems, threshold is 2");

        // Require 3 Alliance systems — should NOT fire
        let mut state2 = EventState::new();
        state2.define(event(
            2,
            vec![EventCondition::FactionControlsNSystems {
                faction: Faction::Alliance,
                min_count: 3,
            }],
            vec![],
        ));

        let fired2 = EventSystem::advance(&mut state2, &world, &tick(1), &[]);
        assert!(fired2.is_empty(), "Alliance only controls 2, threshold is 3");
    }

    #[test]
    fn character_is_force_user_true_for_aware_tier() {
        let mut world = make_world();
        let key = world
            .characters
            .insert(make_character("Leia", ForceTier::Aware));

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharacterIsForceUser { character: key }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "Aware > None, so is a Force user");
    }

    // --- CharactersCoLocated condition ---

    #[test]
    fn characters_co_located_true_when_at_same_system() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Test".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0),
            name: "Endor".into(),
            sector: sector_key, x: 0, y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 0, raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        let luke = world.characters.insert(make_character("Luke", ForceTier::Experienced));
        let vader = world.characters.insert(make_character("Vader", ForceTier::Experienced));
        let emperor = world.characters.insert(make_character("Emperor", ForceTier::Experienced));

        // Put all three in a fleet at the same system
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: sys_key,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![luke, vader, emperor],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[sys_key].fleets.push(fleet);

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharactersCoLocated {
                characters: vec![luke, vader, emperor],
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "All 3 characters co-located should fire");
    }

    #[test]
    fn characters_co_located_false_when_at_different_systems() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Test".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        let sys1 = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0),
            name: "Endor".into(),
            sector: sector_key, x: 0, y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 0, raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });
        let sys2 = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(1),
            name: "Coruscant".into(),
            sector: sector_key, x: 100, y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 0, raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        let luke = world.characters.insert(make_character("Luke", ForceTier::Experienced));
        let vader = world.characters.insert(make_character("Vader", ForceTier::Experienced));

        // Luke at sys1, Vader at sys2
        let fleet1 = world.fleets.insert(crate::world::Fleet {
            location: sys1,
            capital_ships: vec![], fighters: vec![],
            characters: vec![luke], is_alliance: true, has_death_star: false,
        });
        world.systems[sys1].fleets.push(fleet1);

        let fleet2 = world.fleets.insert(crate::world::Fleet {
            location: sys2,
            capital_ships: vec![], fighters: vec![],
            characters: vec![vader], is_alliance: false, has_death_star: false,
        });
        world.systems[sys2].fleets.push(fleet2);

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharactersCoLocated {
                characters: vec![luke, vader],
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty(), "Characters at different systems should not be co-located");
    }

    #[test]
    fn characters_co_located_empty_or_single_is_vacuously_true() {
        let mut world = make_world();
        let luke = world.characters.insert(make_character("Luke", ForceTier::Aware));

        let mut state = EventState::new();

        // Empty characters list — vacuously true
        state.define(event(
            1,
            vec![EventCondition::CharactersCoLocated { characters: vec![] }],
            vec![],
        ));
        // Single character — also vacuously true
        state.define(event(
            2,
            vec![EventCondition::CharactersCoLocated { characters: vec![luke] }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 2, "Empty and single co-location should both be vacuously true");
    }

    #[test]
    fn characters_co_located_false_for_nonexistent_character() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let luke = world.characters.insert(make_character("Luke", ForceTier::Experienced));
        let vader = world.characters.insert(make_character("Vader", ForceTier::Experienced));

        // Remove Vader — his key is now stale
        world.characters.remove(vader);

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharactersCoLocated {
                characters: vec![luke, vader],
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert!(fired.is_empty(), "Co-location should fail for non-existent character");
    }

    #[test]
    fn characters_co_located_across_different_fleets_at_same_system() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Test".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0),
            name: "Endor".into(),
            sector: sector_key, x: 0, y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 0, raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        let luke = world.characters.insert(make_character("Luke", ForceTier::Experienced));
        let vader = world.characters.insert(make_character("Vader", ForceTier::Experienced));

        // Luke in Alliance fleet, Vader in Empire fleet — both at same system
        let fleet1 = world.fleets.insert(crate::world::Fleet {
            location: sys_key,
            capital_ships: vec![], fighters: vec![],
            characters: vec![luke], is_alliance: true, has_death_star: false,
        });
        let fleet2 = world.fleets.insert(crate::world::Fleet {
            location: sys_key,
            capital_ships: vec![], fighters: vec![],
            characters: vec![vader], is_alliance: false, has_death_star: false,
        });
        world.systems[sys_key].fleets.push(fleet1);
        world.systems[sys_key].fleets.push(fleet2);

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharactersCoLocated {
                characters: vec![luke, vader],
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "Characters in different fleets at same system should be co-located");
    }

    #[test]
    fn characters_co_located_via_current_system() {
        use crate::dat::{ExplorationStatus, Faction};

        let mut world = make_world();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Test".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0),
            name: "Endor".into(),
            sector: sector_key, x: 0, y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 0, raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        // Luke in a fleet, Emperor via current_system only (no fleet)
        let luke = world.characters.insert(make_character("Luke", ForceTier::Experienced));
        let mut emperor = make_character("Emperor", ForceTier::Experienced);
        emperor.current_system = Some(sys_key);
        let emperor_key = world.characters.insert(emperor);

        let fleet = world.fleets.insert(crate::world::Fleet {
            location: sys_key,
            capital_ships: vec![], fighters: vec![],
            characters: vec![luke], is_alliance: true, has_death_star: false,
        });
        world.systems[sys_key].fleets.push(fleet);

        let mut state = EventState::new();
        state.define(event(
            1,
            vec![EventCondition::CharactersCoLocated {
                characters: vec![luke, emperor_key],
            }],
            vec![],
        ));

        let fired = EventSystem::advance(&mut state, &world, &tick(1), &[]);
        assert_eq!(fired.len(), 1, "Character with current_system should count as co-located");
    }
}
