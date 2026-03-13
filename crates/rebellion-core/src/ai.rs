//! Rule-based AI manager for the computer-controlled faction.
//!
//! The AI evaluates the game state each `AI_TICK_INTERVAL` game-days and
//! produces a list of `AIAction` recommendations. The caller applies those
//! actions to `ManufacturingState`, `MissionState`, and `GameWorld`.
//!
//! # Design
//!
//! Follows the same stateless pattern as manufacturing and mission systems:
//! - `AIState` holds per-faction persistent data (cooldowns, assignments)
//! - `AISystem::advance(state, world, mfg, missions, tick_events) -> Vec<AIAction>`
//! - Actions are recommendations only — the caller decides whether to apply them
//!
//! # Heuristics (ported from rebellion2's AIManager.cs)
//!
//! 1. **Officer assignment**: for each available (unassigned) character —
//!    - If major character AND unrecruited officers exist → recruitment mission
//!    - Else if major OR diplomacy > 60 → diplomacy mission targeting lowest-popularity system
//! 2. **Production — fighters**: if a system has idle manufacturing capacity,
//!    enqueue the best available fighter class for fleets with excess carrier slots
//! 3. **Production — facilities**: if a system has fewer than `MAX_CONSTRUCTION_YARDS`
//!    construction yards and has free slots, enqueue another
//!
//! # AI tick gating
//!
//! The AI only re-evaluates every `AI_TICK_INTERVAL` game-days. This prevents
//! thrashing and is consistent with the original game's turn-based cadence.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::ids::{
    CharacterKey, FighterKey, FleetKey, ManufacturingFacilityKey, SystemKey,
};
use crate::manufacturing::{BuildableKind, ManufacturingState};
use crate::missions::{MissionFaction, MissionKind, MissionState};
use crate::tick::TickEvent;
use crate::world::{Character, GameWorld};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// How many game-days between full AI re-evaluations.
pub const AI_TICK_INTERVAL: u64 = 7;

/// Diplomacy skill threshold above which a minor character is considered a
/// viable diplomat (rebellion2: `officer.GetSkillValue(Diplomacy) > 60`).
pub const DIPLOMACY_SKILL_THRESHOLD: u32 = 60;

/// Maximum construction yards the AI will build per system before stopping.
pub const MAX_CONSTRUCTION_YARDS: usize = 5;

/// Minimum popularity fraction below which the AI considers a system a
/// diplomacy target (systems already at high popularity are deprioritized).
pub const DIPLOMACY_TARGET_POPULARITY_CAP: f32 = 0.8;

// ---------------------------------------------------------------------------
// AiFaction
// ---------------------------------------------------------------------------

/// The faction the AI controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiFaction {
    Alliance,
    Empire,
}

impl AiFaction {
    /// Convert to the `MissionFaction` used by the mission system.
    pub fn as_mission_faction(self) -> MissionFaction {
        match self {
            AiFaction::Alliance => MissionFaction::Alliance,
            AiFaction::Empire => MissionFaction::Empire,
        }
    }

    /// Returns true if the given character belongs to this faction.
    pub fn owns_character(self, c: &Character) -> bool {
        match self {
            AiFaction::Alliance => c.is_alliance,
            AiFaction::Empire => c.is_empire,
        }
    }

    /// Returns true if a system favors this faction (above neutral popularity).
    pub fn system_popularity(self, system: &crate::world::System) -> f32 {
        match self {
            AiFaction::Alliance => system.popularity_alliance,
            AiFaction::Empire => system.popularity_empire,
        }
    }
}

// ---------------------------------------------------------------------------
// AIState
// ---------------------------------------------------------------------------

/// Persistent AI state for one controlled faction.
///
/// Tracks which characters are currently on missions and the tick of the last
/// full evaluation, so the AI doesn't re-evaluate every single day.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIState {
    /// Which faction this AI controls.
    pub faction: Option<AiFaction>,
    /// Game-day of the last full evaluation pass.
    pub last_eval_tick: u64,
    /// Characters currently dispatched on missions (not available for re-dispatch).
    pub busy_characters: HashSet<CharacterKey>,
}

impl AIState {
    pub fn new(faction: AiFaction) -> Self {
        AIState {
            faction: Some(faction),
            last_eval_tick: 0,
            busy_characters: HashSet::new(),
        }
    }

    /// Returns true if enough ticks have elapsed since the last evaluation.
    pub fn should_evaluate(&self, current_tick: u64) -> bool {
        current_tick == 0 || current_tick.saturating_sub(self.last_eval_tick) >= AI_TICK_INTERVAL
    }

    /// Mark a character as busy (on a mission).
    pub fn mark_busy(&mut self, character: CharacterKey) {
        self.busy_characters.insert(character);
    }

    /// Release a character back to available pool.
    pub fn mark_available(&mut self, character: CharacterKey) {
        self.busy_characters.remove(&character);
    }

    /// Returns true if a character is currently busy.
    pub fn is_busy(&self, character: CharacterKey) -> bool {
        self.busy_characters.contains(&character)
    }
}

// ---------------------------------------------------------------------------
// AIAction
// ---------------------------------------------------------------------------

/// A recommended action for the caller to apply to the game state.
///
/// Actions are pure recommendations — the AI system never mutates state
/// directly. The caller decides whether to apply each action.
#[derive(Debug, Clone, PartialEq)]
pub enum AIAction {
    /// Dispatch a character on a mission.
    DispatchMission {
        kind: MissionKind,
        character: CharacterKey,
        target_system: SystemKey,
        /// Suggested duration roll (0..1) for `MissionState::dispatch`.
        duration_roll: f64,
    },

    /// Enqueue a unit or facility for construction at a system.
    EnqueueProduction {
        system: SystemKey,
        kind: BuildableKind,
        /// Suggested tick cost for the `QueueItem`.
        ticks: u32,
    },

    /// Move a fleet to a target system (attack or reinforce).
    MoveFleet {
        fleet: FleetKey,
        to_system: SystemKey,
        reason: FleetMoveReason,
    },
}

/// Why the AI is moving a fleet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetMoveReason {
    /// Attacking a weak or neutral enemy system.
    Attack,
    /// Reinforcing a friendly system under threat.
    Reinforce,
}

// ---------------------------------------------------------------------------
// AISystem
// ---------------------------------------------------------------------------

/// Stateless AI evaluation system.
pub struct AISystem;

impl AISystem {
    /// Evaluate the AI faction's situation and return recommended actions.
    ///
    /// Returns an empty vec if no ticks elapsed or the AI interval hasn't
    /// elapsed yet. The caller should apply each `AIAction` in order.
    ///
    /// After applying `DispatchMission` actions, the caller must also call
    /// `state.mark_busy(character)` for each dispatched character, and
    /// `state.mark_available(character)` when the corresponding
    /// `MissionResult` arrives.
    pub fn advance(
        state: &mut AIState,
        world: &GameWorld,
        mfg_state: &ManufacturingState,
        _mission_state: &MissionState,
        tick_events: &[TickEvent],
    ) -> Vec<AIAction> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let current_tick = tick_events.last().unwrap().tick;

        if !state.should_evaluate(current_tick) {
            return Vec::new();
        }

        state.last_eval_tick = current_tick;

        let faction = match state.faction {
            Some(f) => f,
            None => return Vec::new(),
        };

        let mut actions = Vec::new();

        // Run each heuristic module.
        Self::evaluate_officers(state, world, faction, &mut actions);
        Self::evaluate_production(world, mfg_state, faction, &mut actions);
        Self::evaluate_fleet_deployment(world, faction, &mut actions);

        actions
    }

    // -----------------------------------------------------------------------
    // Officer heuristics
    // -----------------------------------------------------------------------

    /// For each available character, decide whether to dispatch a mission.
    ///
    /// Priority order (from AIManager.cs):
    /// 1. Major characters → recruitment if unrecruited officers exist
    /// 2. Major characters or high-diplomacy minors → diplomacy on low-popularity systems
    fn evaluate_officers(
        state: &AIState,
        world: &GameWorld,
        faction: AiFaction,
        actions: &mut Vec<AIAction>,
    ) {
        // Count unrecruited characters for this faction (can_be_commander but
        // not yet flagged as belonging to the faction — proxy: opposite faction flag).
        let unrecruited_count = world
            .characters
            .values()
            .filter(|c| c.can_be_commander && !faction.owns_character(c))
            .count();

        // Find the best diplomacy target: lowest-popularity system for this faction,
        // below the popularity cap.
        let diplomacy_target = Self::find_diplomacy_target(world, faction);

        for (char_key, character) in world.characters.iter() {
            if !faction.owns_character(character) {
                continue;
            }
            if !character.can_be_commander {
                continue;
            }
            if state.is_busy(char_key) {
                continue;
            }

            let diplomacy_score =
                character.diplomacy.base + character.diplomacy.variance / 2;

            if character.is_major && unrecruited_count > 0 {
                // Find a friendly system to use as recruitment base.
                if let Some(base_system) = Self::find_friendly_system(world, faction) {
                    actions.push(AIAction::DispatchMission {
                        kind: MissionKind::Recruitment,
                        character: char_key,
                        target_system: base_system,
                        duration_roll: 0.5,
                    });
                }
            } else if character.is_major || diplomacy_score > DIPLOMACY_SKILL_THRESHOLD {
                if let Some(target) = diplomacy_target {
                    actions.push(AIAction::DispatchMission {
                        kind: MissionKind::Diplomacy,
                        character: char_key,
                        target_system: target,
                        duration_roll: 0.5,
                    });
                }
            }
        }
    }

    /// Find the system with the lowest AI faction popularity (a good diplomacy target).
    ///
    /// Only returns systems below `DIPLOMACY_TARGET_POPULARITY_CAP` — systems
    /// already firmly ours are not worth spending characters on.
    fn find_diplomacy_target(world: &GameWorld, faction: AiFaction) -> Option<SystemKey> {
        world
            .systems
            .iter()
            .filter(|(_, s)| faction.system_popularity(s) < DIPLOMACY_TARGET_POPULARITY_CAP)
            .min_by(|(_, a), (_, b)| {
                faction
                    .system_popularity(a)
                    .partial_cmp(&faction.system_popularity(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k)
    }

    /// Find a system that the AI faction has relatively high popularity in —
    /// used as a recruitment base.
    fn find_friendly_system(world: &GameWorld, faction: AiFaction) -> Option<SystemKey> {
        world
            .systems
            .iter()
            .max_by(|(_, a), (_, b)| {
                faction
                    .system_popularity(a)
                    .partial_cmp(&faction.system_popularity(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k)
    }

    // -----------------------------------------------------------------------
    // Production heuristics
    // -----------------------------------------------------------------------

    /// For systems with idle manufacturing capacity, enqueue appropriate units.
    ///
    /// - Systems with no active queue → enqueue best available fighter class
    /// - Systems with few construction yards → enqueue a manufacturing facility
    fn evaluate_production(
        world: &GameWorld,
        mfg_state: &ManufacturingState,
        faction: AiFaction,
        actions: &mut Vec<AIAction>,
    ) {
        // Find the best fighter class for this faction (highest research_order = most advanced).
        let best_fighter = Self::best_fighter_class(world, faction);

        for (sys_key, system) in world.systems.iter() {
            // Only act on systems where this faction has manufacturing facilities.
            let has_mfg = system
                .manufacturing_facilities
                .iter()
                .any(|mfk| {
                    world
                        .manufacturing_facilities
                        .get(*mfk)
                        .map(|f| match faction {
                            AiFaction::Alliance => f.is_alliance,
                            AiFaction::Empire => !f.is_alliance,
                        })
                        .unwrap_or(false)
                });

            if !has_mfg {
                continue;
            }

            let queue = mfg_state.queue(sys_key);
            let queue_empty = queue.map(|q| q.is_empty()).unwrap_or(true);

            if queue_empty {
                // Enqueue fighters if we have a class to build.
                if let Some((fighter_key, fighter_class)) = best_fighter {
                    // Cost in ticks: scale from refined_material_cost (stub: 1 material = 1 tick).
                    let ticks = fighter_class.refined_material_cost.max(5);
                    actions.push(AIAction::EnqueueProduction {
                        system: sys_key,
                        kind: BuildableKind::Fighter(fighter_key),
                        ticks,
                    });
                }

                // Also consider building more construction yards if below cap.
                let yard_count = system.manufacturing_facilities.len();
                if yard_count < MAX_CONSTRUCTION_YARDS {
                    if let Some(mfg_key) =
                        Self::find_manufacturing_facility_class(world, faction)
                    {
                        actions.push(AIAction::EnqueueProduction {
                            system: sys_key,
                            kind: BuildableKind::ManufacturingFacility(mfg_key),
                            ticks: 30, // facilities take ~30 days
                        });
                    }
                }
            }
        }
    }

    /// Select the most advanced fighter class available for this faction.
    fn best_fighter_class(
        world: &GameWorld,
        faction: AiFaction,
    ) -> Option<(FighterKey, &crate::world::FighterClass)> {
        world
            .fighter_classes
            .iter()
            .filter(|(_, fc)| match faction {
                AiFaction::Alliance => fc.is_alliance,
                AiFaction::Empire => fc.is_empire,
            })
            .max_by_key(|(_, fc)| fc.refined_material_cost)
    }

    /// Find a manufacturing facility key to use as a class reference for facility construction.
    ///
    /// In Living Galaxy scope we just pick any existing faction-owned facility
    /// as the template for "build another like this".
    fn find_manufacturing_facility_class(
        world: &GameWorld,
        faction: AiFaction,
    ) -> Option<ManufacturingFacilityKey> {
        world
            .manufacturing_facilities
            .iter()
            .find(|(_, f)| match faction {
                AiFaction::Alliance => f.is_alliance,
                AiFaction::Empire => !f.is_alliance,
            })
            .map(|(k, _)| k)
    }

    // -----------------------------------------------------------------------
    // Fleet deployment heuristics
    // -----------------------------------------------------------------------

    /// Move idle fleets toward high-value targets.
    ///
    /// - Fleets not already in a contested system → attack the enemy's weakest system
    /// - Friendly systems with no fleet and high popularity → reinforce
    fn evaluate_fleet_deployment(
        world: &GameWorld,
        faction: AiFaction,
        actions: &mut Vec<AIAction>,
    ) {
        // Find the weakest enemy system (lowest enemy popularity = easiest conquest).
        let attack_target = world
            .systems
            .iter()
            .filter(|(_, s)| {
                // Target systems where the enemy has some presence but we're weak.
                let enemy_pop = match faction {
                    AiFaction::Alliance => s.popularity_empire,
                    AiFaction::Empire => s.popularity_alliance,
                };
                let our_pop = faction.system_popularity(s);
                enemy_pop > 0.1 && our_pop < 0.4
            })
            .min_by(|(_, a), (_, b)| {
                let pop_a = match faction {
                    AiFaction::Alliance => a.popularity_empire,
                    AiFaction::Empire => a.popularity_alliance,
                };
                let pop_b = match faction {
                    AiFaction::Alliance => b.popularity_empire,
                    AiFaction::Empire => b.popularity_alliance,
                };
                pop_a.partial_cmp(&pop_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k);

        // Find friendly systems with high popularity but no fleet (need reinforcement).
        let reinforce_target = world
            .systems
            .iter()
            .filter(|(_, s)| {
                faction.system_popularity(s) > 0.6 && s.fleets.is_empty()
            })
            .max_by(|(_, a), (_, b)| {
                faction
                    .system_popularity(a)
                    .partial_cmp(&faction.system_popularity(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k);

        for (fleet_key, fleet) in world.fleets.iter() {
            // Only command our own fleets.
            let is_ours = match faction {
                AiFaction::Alliance => fleet.is_alliance,
                AiFaction::Empire => !fleet.is_alliance,
            };
            if !is_ours {
                continue;
            }

            // Check if this fleet's current system needs reinforcement.
            let current_needs_defense = world
                .systems
                .get(fleet.location)
                .map(|s| {
                    let enemy_pop = match faction {
                        AiFaction::Alliance => s.popularity_empire,
                        AiFaction::Empire => s.popularity_alliance,
                    };
                    enemy_pop > 0.5
                })
                .unwrap_or(false);

            if current_needs_defense {
                // Fleet stays in place — don't generate a move action.
                continue;
            }

            // If there's a reinforce target that isn't the current location, prefer it.
            if let Some(target) = reinforce_target {
                if target != fleet.location {
                    actions.push(AIAction::MoveFleet {
                        fleet: fleet_key,
                        to_system: target,
                        reason: FleetMoveReason::Reinforce,
                    });
                    continue;
                }
            }

            // Otherwise attack the weakest enemy system.
            if let Some(target) = attack_target {
                if target != fleet.location {
                    actions.push(AIAction::MoveFleet {
                        fleet: fleet_key,
                        to_system: target,
                        reason: FleetMoveReason::Attack,
                    });
                }
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
    use crate::ids::{DatId, SectorKey};
    use crate::world::{
        Character, FighterClass, Fleet, GameWorld, Sector, SkillPair, System,
    };
    use crate::dat::SectorGroup;
    use crate::manufacturing::ManufacturingState;
    use crate::missions::MissionState;
    use crate::tick::TickEvent;

    // -----------------------------------------------------------------------
    // World builder helpers
    // -----------------------------------------------------------------------

    fn empty_world() -> GameWorld {
        GameWorld {
            systems: slotmap::SlotMap::with_key(),
            sectors: slotmap::SlotMap::with_key(),
            capital_ship_classes: slotmap::SlotMap::with_key(),
            fighter_classes: slotmap::SlotMap::with_key(),
            characters: slotmap::SlotMap::with_key(),
            fleets: slotmap::SlotMap::with_key(),
            troops: slotmap::SlotMap::with_key(),
            special_forces: slotmap::SlotMap::with_key(),
            defense_facilities: slotmap::SlotMap::with_key(),
            manufacturing_facilities: slotmap::SlotMap::with_key(),
            production_facilities: slotmap::SlotMap::with_key(),
        }
    }

    fn add_sector(world: &mut GameWorld) -> SectorKey {
        world.sectors.insert(Sector {
            dat_id: DatId(0),
            name: "Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        })
    }

    fn add_system(world: &mut GameWorld, sector: SectorKey, pop_alliance: f32, pop_empire: f32) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId(0),
            name: "Test System".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: pop_alliance,
            popularity_empire: pop_empire,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
        })
    }

    fn zero_skills() -> SkillPair {
        SkillPair { base: 0, variance: 0 }
    }

    fn add_character(
        world: &mut GameWorld,
        is_alliance: bool,
        is_major: bool,
        diplomacy_base: u32,
    ) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId(0),
            name: "TestChar".into(),
            is_alliance,
            is_empire: !is_alliance,
            is_major,
            diplomacy: SkillPair { base: diplomacy_base, variance: 0 },
            espionage: zero_skills(),
            ship_design: zero_skills(),
            troop_training: zero_skills(),
            facility_design: zero_skills(),
            combat: zero_skills(),
            leadership: SkillPair { base: 50, variance: 0 },
            loyalty: zero_skills(),
            jedi_probability: 0,
            jedi_level: zero_skills(),
            can_be_admiral: false,
            can_be_commander: true,
            can_be_general: false,
        })
    }

    fn ticks(n: u64) -> Vec<TickEvent> {
        (1..=n).map(|t| TickEvent { tick: t }).collect()
    }

    // -----------------------------------------------------------------------
    // AIState tests
    // -----------------------------------------------------------------------

    #[test]
    fn should_evaluate_at_tick_zero() {
        let state = AIState::new(AiFaction::Empire);
        assert!(state.should_evaluate(0));
    }

    #[test]
    fn should_not_evaluate_before_interval() {
        let mut state = AIState::new(AiFaction::Empire);
        state.last_eval_tick = 0;
        // 3 days elapsed — interval is 7
        assert!(!state.should_evaluate(3));
    }

    #[test]
    fn should_evaluate_after_interval() {
        let mut state = AIState::new(AiFaction::Empire);
        state.last_eval_tick = 0;
        assert!(state.should_evaluate(7));
        assert!(state.should_evaluate(14));
    }

    #[test]
    fn busy_character_tracking() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _ = add_system(&mut world, sector, 0.5, 0.5);
        let char_key = add_character(&mut world, true, false, 30);

        let mut state = AIState::new(AiFaction::Alliance);
        assert!(!state.is_busy(char_key));
        state.mark_busy(char_key);
        assert!(state.is_busy(char_key));
        state.mark_available(char_key);
        assert!(!state.is_busy(char_key));
    }

    // -----------------------------------------------------------------------
    // No-op conditions
    // -----------------------------------------------------------------------

    #[test]
    fn no_ticks_returns_empty() {
        let world = empty_world();
        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &[]);
        assert!(actions.is_empty());
    }

    #[test]
    fn before_interval_returns_empty() {
        let world = empty_world();
        let mut state = AIState::new(AiFaction::Empire);
        state.last_eval_tick = 5; // already evaluated 5 ticks ago, interval=7

        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        // 3 ticks elapsed since last eval (5+3=8... wait, current_tick = 8 > 5+7=12? No)
        // last_eval=5, current=8, diff=3 < 7 → should not evaluate
        let actions =
            AISystem::advance(&mut state, &world, &mfg, &missions, &[TickEvent { tick: 8 }]);
        assert!(actions.is_empty());
    }

    // -----------------------------------------------------------------------
    // Officer heuristics
    // -----------------------------------------------------------------------

    #[test]
    fn major_character_dispatched_on_recruitment_when_unrecruited_exist() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _sys = add_system(&mut world, sector, 0.9, 0.1); // friendly high-pop system

        // Major empire character — can be commander
        let major = add_character(&mut world, false, true, 30);
        // An unrecruited Alliance character (enemy faction = "unrecruited" from Empire perspective)
        let _ = add_character(&mut world, true, false, 30);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let recruitment = actions.iter().find(|a| matches!(
            a,
            AIAction::DispatchMission { kind: MissionKind::Recruitment, character, .. }
            if *character == major
        ));
        assert!(recruitment.is_some(), "expected recruitment mission for major character");
    }

    #[test]
    fn high_diplomacy_minor_dispatched_on_diplomacy() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        // Low-popularity empire system — a good diplomacy target
        let _ = add_system(&mut world, sector, 0.1, 0.2);

        // Minor empire character with high diplomacy
        let diplomat = add_character(&mut world, false, false, 80);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let diplomacy = actions.iter().find(|a| matches!(
            a,
            AIAction::DispatchMission { kind: MissionKind::Diplomacy, character, .. }
            if *character == diplomat
        ));
        assert!(diplomacy.is_some(), "expected diplomacy mission for high-skill minor");
    }

    #[test]
    fn low_diplomacy_minor_not_dispatched() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _ = add_system(&mut world, sector, 0.1, 0.2);

        // Minor with low diplomacy — below threshold
        let _ = add_character(&mut world, false, false, 30);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));
        let mission_count = actions
            .iter()
            .filter(|a| matches!(a, AIAction::DispatchMission { .. }))
            .count();
        assert_eq!(mission_count, 0);
    }

    #[test]
    fn busy_character_not_redispatched() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _ = add_system(&mut world, sector, 0.1, 0.2);

        let diplomat = add_character(&mut world, false, false, 80);

        let mut state = AIState::new(AiFaction::Empire);
        state.mark_busy(diplomat);

        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));
        let mission_count = actions
            .iter()
            .filter(|a| matches!(a, AIAction::DispatchMission { .. }))
            .count();
        assert_eq!(mission_count, 0);
    }

    // -----------------------------------------------------------------------
    // Production heuristics
    // -----------------------------------------------------------------------

    #[test]
    fn idle_system_with_mfg_gets_fighter_production() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Add an empire manufacturing facility
        let mfg_key = world.manufacturing_facilities.insert(
            crate::world::ManufacturingFacilityInstance {
                class_dat_id: DatId(1),
                is_alliance: false, // empire
            },
        );

        let sys_key = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Coruscant".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.9,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![mfg_key],
            production_facilities: vec![],
        });

        // Add a TIE fighter class
        let _ = world.fighter_classes.insert(FighterClass {
            dat_id: DatId(10),
            name: "TIE Fighter".into(),
            is_alliance: false,
            is_empire: true,
            refined_material_cost: 20,
            maintenance_cost: 2,
            squadron_size: 6,
            torpedoes: 0,
        });

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new(); // empty queue
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let has_fighter_enqueue = actions.iter().any(|a| matches!(
            a,
            AIAction::EnqueueProduction { system, kind: BuildableKind::Fighter(_), .. }
            if *system == sys_key
        ));
        assert!(has_fighter_enqueue, "expected fighter production at empire system");
    }

    // -----------------------------------------------------------------------
    // Fleet deployment heuristics
    // -----------------------------------------------------------------------

    #[test]
    fn fleet_directed_to_attack_weak_enemy_system() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Empire fleet at a safe friendly system
        let home_sys = add_system(&mut world, sector, 0.1, 0.9);
        // Weak enemy system (low empire presence, some alliance)
        let target_sys = add_system(&mut world, sector, 0.6, 0.15);

        let fleet_key = world.fleets.insert(Fleet {
            location: home_sys,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: false, // empire fleet
        });

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let fleet_move = actions.iter().find(|a| matches!(
            a,
            AIAction::MoveFleet { fleet, to_system, reason: FleetMoveReason::Attack }
            if *fleet == fleet_key && *to_system == target_sys
        ));
        assert!(fleet_move.is_some(), "expected fleet to be directed at weak enemy system");
    }

    #[test]
    fn interval_gates_repeated_evaluation() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _ = add_system(&mut world, sector, 0.5, 0.5);

        let mut state = AIState::new(AiFaction::Alliance);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        // First evaluation at tick 7
        let _first = AISystem::advance(&mut state, &world, &mfg, &missions, &[TickEvent { tick: 7 }]);
        assert_eq!(state.last_eval_tick, 7);

        // Tick 10 — only 3 days elapsed, should skip
        let second = AISystem::advance(&mut state, &world, &mfg, &missions, &[TickEvent { tick: 10 }]);
        assert!(second.is_empty(), "expected no actions before interval elapses");
        assert_eq!(state.last_eval_tick, 7); // unchanged

        // Tick 14 — 7 days elapsed, should evaluate again
        let third = AISystem::advance(&mut state, &world, &mfg, &missions, &[TickEvent { tick: 14 }]);
        assert_eq!(state.last_eval_tick, 14);
        let _ = third; // just checking it ran
    }
}
