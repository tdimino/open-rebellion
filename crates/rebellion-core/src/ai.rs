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
use crate::dat::ExplorationStatus;
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

/// Espionage skill threshold above which a character is considered a viable
/// covert operative. Characters below this threshold are not sent on
/// Sabotage/Assassination/Espionage missions.
pub const ESPIONAGE_SKILL_THRESHOLD: u32 = 50;

/// Minimum expected success probability (0.0–1.0) the AI requires before
/// dispatching a covert mission. Prevents wasting characters on impossible ops.
pub const COVERT_MIN_SUCCESS_PROB: f64 = 0.30;

/// Maximum number of new covert missions the AI will queue per evaluation pass.
/// Prevents spamming every available operative on espionage each tick interval.
pub const MAX_COVERT_OPS_PER_EVAL: usize = 3;

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
        Self::evaluate_espionage(state, world, faction, &mut actions);
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
    // Espionage heuristics
    // -----------------------------------------------------------------------

    /// Dispatch covert operatives on Sabotage, Assassination, and Espionage
    /// missions against the enemy faction.
    ///
    /// Priority order (each pass picks the best available character for the
    /// best available target):
    /// 1. **Sabotage** — enemy systems with manufacturing facilities. High-value
    ///    targets (more mfg facilities) first. Skill: espionage.
    /// 2. **Assassination** — enemy major characters. Most dangerous first
    ///    (highest combined skill). Skill: combat.
    /// 3. **Espionage** (intelligence) — unexplored enemy systems. Skill: espionage.
    ///
    /// Only characters with skill ≥ `ESPIONAGE_SKILL_THRESHOLD` are considered.
    /// A target is skipped if expected success probability < `COVERT_MIN_SUCCESS_PROB`.
    /// At most `MAX_COVERT_OPS_PER_EVAL` covert missions are queued per pass.
    fn evaluate_espionage(
        state: &AIState,
        world: &GameWorld,
        faction: AiFaction,
        actions: &mut Vec<AIAction>,
    ) {
        // Collect available covert operatives sorted by espionage skill (desc).
        let mut operatives: Vec<(CharacterKey, u32)> = world
            .characters
            .iter()
            .filter_map(|(key, c)| {
                if !faction.owns_character(c) || state.is_busy(key) || !c.can_be_commander {
                    return None;
                }
                let esp = c.espionage.base + c.espionage.variance / 2;
                if esp >= ESPIONAGE_SKILL_THRESHOLD {
                    Some((key, esp))
                } else {
                    None
                }
            })
            .collect();

        if operatives.is_empty() {
            return;
        }

        // Sort descending by espionage score so best ops go to highest-value targets.
        operatives.sort_by(|a, b| b.1.cmp(&a.1));

        let mut ops_queued = 0;
        let mut op_idx = 0;

        // ── Priority 1: Sabotage enemy manufacturing systems ─────────────────
        // Score each enemy system by number of mfg facilities (proxy for value).
        let mut sabotage_targets: Vec<(SystemKey, usize)> = world
            .systems
            .iter()
            .filter_map(|(sys_key, system)| {
                // Target systems where the enemy faction has manufacturing presence.
                let enemy_mfg = system
                    .manufacturing_facilities
                    .iter()
                    .filter(|mfk| {
                        world
                            .manufacturing_facilities
                            .get(**mfk)
                            .map(|f| match faction {
                                AiFaction::Alliance => !f.is_alliance, // enemy = empire
                                AiFaction::Empire   =>  f.is_alliance, // enemy = alliance
                            })
                            .unwrap_or(false)
                    })
                    .count();
                if enemy_mfg > 0 {
                    Some((sys_key, enemy_mfg))
                } else {
                    None
                }
            })
            .collect();

        // Highest facility count first.
        sabotage_targets.sort_by(|a, b| b.1.cmp(&a.1));

        for (target_sys, _) in &sabotage_targets {
            if ops_queued >= MAX_COVERT_OPS_PER_EVAL || op_idx >= operatives.len() {
                break;
            }
            let (char_key, esp_score) = operatives[op_idx];
            if !Self::expected_success(world, MissionKind::Sabotage, esp_score) {
                op_idx += 1;
                continue;
            }
            actions.push(AIAction::DispatchMission {
                kind: MissionKind::Sabotage,
                character: char_key,
                target_system: *target_sys,
                duration_roll: 0.5,
            });
            op_idx += 1;
            ops_queued += 1;
        }

        // ── Priority 2: Assassination of dangerous enemy major characters ─────
        // Score each enemy major character by total skill.
        let enemy_major_chars: Vec<CharacterKey> = world
            .characters
            .iter()
            .filter_map(|(key, c)| {
                if !c.is_major || faction.owns_character(c) {
                    return None;
                }
                Some(key)
            })
            .collect();

        // For assassination we need a system to target — use any enemy system as the
        // "location" proxy (the actual character is tracked by CharacterKey in effects).
        // Pick the best-populated enemy system as target anchor.
        let assassination_base = world
            .systems
            .iter()
            .filter(|(_, s)| {
                match faction {
                    AiFaction::Alliance => s.popularity_empire   > 0.3,
                    AiFaction::Empire   => s.popularity_alliance > 0.3,
                }
            })
            .max_by(|(_, a), (_, b)| {
                let pop_a = match faction {
                    AiFaction::Alliance => a.popularity_empire,
                    AiFaction::Empire   => a.popularity_alliance,
                };
                let pop_b = match faction {
                    AiFaction::Alliance => b.popularity_empire,
                    AiFaction::Empire   => b.popularity_alliance,
                };
                pop_a.partial_cmp(&pop_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k);

        if let Some(target_sys) = assassination_base {
            for _ in &enemy_major_chars {
                if ops_queued >= MAX_COVERT_OPS_PER_EVAL || op_idx >= operatives.len() {
                    break;
                }
                let (char_key, _) = operatives[op_idx];
                let char = &world.characters[char_key];
                let combat_score = char.combat.base + char.combat.variance / 2;
                if !Self::expected_success(world, MissionKind::Assassination, combat_score) {
                    op_idx += 1;
                    continue;
                }
                actions.push(AIAction::DispatchMission {
                    kind: MissionKind::Assassination,
                    character: char_key,
                    target_system: target_sys,
                    duration_roll: 0.5,
                });
                op_idx += 1;
                ops_queued += 1;
            }
        }

        // ── Priority 3: Intelligence gathering on unexplored enemy systems ────
        let unexplored_targets: Vec<SystemKey> = world
            .systems
            .iter()
            .filter_map(|(sys_key, system)| {
                if system.exploration_status == ExplorationStatus::Unexplored {
                    Some(sys_key)
                } else {
                    None
                }
            })
            .collect();

        for target_sys in &unexplored_targets {
            if ops_queued >= MAX_COVERT_OPS_PER_EVAL || op_idx >= operatives.len() {
                break;
            }
            let (char_key, esp_score) = operatives[op_idx];
            if !Self::expected_success(world, MissionKind::Espionage, esp_score) {
                op_idx += 1;
                continue;
            }
            actions.push(AIAction::DispatchMission {
                kind: MissionKind::Espionage,
                character: char_key,
                target_system: *target_sys,
                duration_roll: 0.5,
            });
            op_idx += 1;
            ops_queued += 1;
        }
    }

    /// Estimate whether a mission is worth dispatching given the operative's
    /// skill score.
    ///
    /// Uses the MSTB table if loaded; falls back to the quadratic formula.
    /// Returns true if expected success probability ≥ `COVERT_MIN_SUCCESS_PROB`.
    fn expected_success(world: &GameWorld, kind: MissionKind, skill_score: u32) -> bool {
        let prob_pct: f64 = if let Some(key) = kind.mstb_key() {
            if let Some(table) = world.mission_tables.get(key) {
                table.lookup(skill_score as i32) as f64
            } else {
                // MSTB not loaded — quadratic fallback.
                let (a, b, c) = kind.coefficients();
                let s = skill_score as f64;
                (a * s * s + b * s + c).clamp(kind.min_success_prob(), kind.max_success_prob())
            }
        } else {
            // Autoscrap and others without tables always succeed.
            100.0
        };

        prob_pct / 100.0 >= COVERT_MIN_SUCCESS_PROB
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
        Character, FighterClass, Fleet, ForceTier, GameWorld, Sector, SkillPair, System,
    };
    use crate::dat::SectorGroup;
    use crate::manufacturing::ManufacturingState;
    use crate::missions::MissionState;
    use crate::tick::TickEvent;

    // -----------------------------------------------------------------------
    // World builder helpers
    // -----------------------------------------------------------------------

    fn empty_world() -> GameWorld {
        GameWorld::default()
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
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
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
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
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
            overall_attack_strength: 10,
            bombardment_defense: 0,
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
            has_death_star: false,
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

    // -----------------------------------------------------------------------
    // Espionage heuristics
    // -----------------------------------------------------------------------

    /// Helper: insert a character with custom espionage + combat scores.
    fn add_spy(
        world: &mut GameWorld,
        is_alliance: bool,
        is_major: bool,
        espionage_base: u32,
        combat_base: u32,
    ) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId(0),
            name: "TestSpy".into(),
            is_alliance,
            is_empire: !is_alliance,
            is_major,
            diplomacy: zero_skills(),
            espionage: SkillPair { base: espionage_base, variance: 0 },
            ship_design: zero_skills(),
            troop_training: zero_skills(),
            facility_design: zero_skills(),
            combat: SkillPair { base: combat_base, variance: 0 },
            leadership: SkillPair { base: 50, variance: 0 },
            loyalty: zero_skills(),
            jedi_probability: 0,
            jedi_level: zero_skills(),
            can_be_admiral: false,
            can_be_commander: true,
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
        })
    }

    #[test]
    fn high_espionage_character_dispatched_on_sabotage() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Enemy (alliance) system with a manufacturing facility — a sabotage target.
        let mfg_key = world.manufacturing_facilities.insert(
            crate::world::ManufacturingFacilityInstance {
                class_dat_id: DatId(1),
                is_alliance: true, // alliance-owned → enemy from Empire's perspective
            },
        );
        let enemy_sys = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Enemy Shipyard".into(),
            sector,
            x: 10,
            y: 10,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.1,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![mfg_key],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
        });

        // Empire spy with high espionage — above threshold.
        let spy = add_spy(&mut world, false, false, 80, 50);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let sabotage = actions.iter().find(|a| matches!(
            a,
            AIAction::DispatchMission {
                kind: MissionKind::Sabotage,
                character,
                target_system,
                ..
            }
            if *character == spy && *target_system == enemy_sys
        ));
        assert!(sabotage.is_some(), "expected sabotage mission against enemy shipyard");
    }

    #[test]
    fn low_espionage_character_not_dispatched_on_covert_ops() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        let mfg_key = world.manufacturing_facilities.insert(
            crate::world::ManufacturingFacilityInstance {
                class_dat_id: DatId(1),
                is_alliance: true,
            },
        );
        let _ = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Enemy Shipyard".into(),
            sector,
            x: 10,
            y: 10,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.1,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![mfg_key],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
        });

        // Empire spy with low espionage — below threshold.
        let _ = add_spy(&mut world, false, false, 20, 20);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let covert = actions.iter().filter(|a| matches!(
            a,
            AIAction::DispatchMission {
                kind: MissionKind::Sabotage
                    | MissionKind::Assassination
                    | MissionKind::Espionage,
                ..
            }
        )).count();
        assert_eq!(covert, 0, "low-espionage character should not be dispatched on covert ops");
    }

    #[test]
    fn ai_dispatches_espionage_on_unexplored_system() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Unexplored system — worth investigating.
        let unexplored = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Dark System".into(),
            sector,
            x: 50,
            y: 50,
            exploration_status: crate::dat::ExplorationStatus::Unexplored,
            popularity_alliance: 0.0,
            popularity_empire: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
        });

        // Alliance spy.
        let spy = add_spy(&mut world, true, false, 75, 40);

        let mut state = AIState::new(AiFaction::Alliance);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let intel = actions.iter().find(|a| matches!(
            a,
            AIAction::DispatchMission {
                kind: MissionKind::Espionage,
                character,
                target_system,
                ..
            }
            if *character == spy && *target_system == unexplored
        ));
        assert!(intel.is_some(), "expected intelligence mission on unexplored system");
    }

    #[test]
    fn covert_ops_capped_at_max_per_eval() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Create 5 enemy manufacturing systems — more than MAX_COVERT_OPS_PER_EVAL.
        for i in 0..5u32 {
            let mfg_key = world.manufacturing_facilities.insert(
                crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(i),
                    is_alliance: true,
                },
            );
            world.systems.insert(System {
                dat_id: DatId(i),
                name: format!("Enemy System {}", i),
                sector,
                x: (i * 10) as u16,
                y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.7,
                popularity_empire: 0.1,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                defense_facilities: vec![],
                manufacturing_facilities: vec![mfg_key],
                production_facilities: vec![],
                is_headquarters: false,
                is_destroyed: false,
                controlling_faction: None,
            });
        }

        // 5 Empire spies — all highly skilled.
        for _ in 0..5 {
            add_spy(&mut world, false, false, 80, 60);
        }

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &ticks(7));

        let covert_count = actions.iter().filter(|a| matches!(
            a,
            AIAction::DispatchMission {
                kind: MissionKind::Sabotage
                    | MissionKind::Assassination
                    | MissionKind::Espionage,
                ..
            }
        )).count();

        assert!(
            covert_count <= MAX_COVERT_OPS_PER_EVAL,
            "expected at most {} covert ops, got {}",
            MAX_COVERT_OPS_PER_EVAL,
            covert_count,
        );
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
