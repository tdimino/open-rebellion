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

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::ids::{
    CapitalShipKey, CharacterKey, FighterKey, FleetKey, ManufacturingFacilityKey, SystemKey,
};
use crate::tuning::GameConfig;
use crate::manufacturing::{BuildableKind, ManufacturingState};
use crate::missions::{MissionFaction, MissionKind, MissionState};
use crate::research::{ResearchProject, ResearchState, ResearchSystem, TechType};
use crate::dat::ExplorationStatus;
use crate::tick::TickEvent;
use crate::world::{Character, ControlKind, GameWorld};

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
// GalaxyState — strategic categorization of all systems
// ---------------------------------------------------------------------------

/// Snapshot of the galaxy's strategic state from one faction's perspective.
#[derive(Debug, Default)]
struct GalaxyState {
    /// Systems we control (not contested).
    our_controlled: Vec<SystemKey>,
    /// Our controlled systems with no friendly fleet present.
    our_undefended: Vec<SystemKey>,
    /// Our HQ system.
    our_hq: Option<SystemKey>,
    /// Systems controlled by the enemy (sorted by weakness for targeting).
    enemy_controlled: Vec<SystemKey>,
    /// Enemy HQ system.
    enemy_hq: Option<SystemKey>,
    /// Our systems with enemy fleets present.
    contested: Vec<SystemKey>,
    /// Neutral/unclaimed systems.
    unoccupied: Vec<SystemKey>,

    /// Fraction of controlled systems that are ours: our / (our + enemy).
    /// 0.0 = we control nothing, 1.0 = we control everything.
    /// Used by FUN_0053e190 ratio scoring to scale aggression.
    control_ratio: f64,

    /// Aggression level derived from control_ratio.
    /// 0.0 = fully defensive (hunker down), 1.0 = fully offensive (all-out attack).
    /// Interpolated: 10% control → 0.2, 50% → 0.5, 90% → 0.8.
    aggression: f64,
}

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
    /// Systems where combat recently occurred — deprioritized for attack targeting.
    /// Maps SystemKey → tick of last battle. Decays over ~100 ticks.
    #[serde(default)]
    pub battle_cooldowns: HashMap<SystemKey, u64>,
}

impl AIState {
    pub fn new(faction: AiFaction) -> Self {
        AIState {
            faction: Some(faction),
            last_eval_tick: 0,
            busy_characters: HashSet::new(),
            battle_cooldowns: HashMap::new(),
        }
    }

    /// Returns true if enough ticks have elapsed since the last evaluation.
    pub fn should_evaluate(&self, current_tick: u64, tick_interval: u64) -> bool {
        current_tick == 0 || current_tick.saturating_sub(self.last_eval_tick) >= tick_interval
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

    /// Assign a character to research a tech tree.
    DispatchResearch {
        character: CharacterKey,
        tech_type: TechType,
        ticks: u32,
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
        movement: &crate::movement::MovementState,
        tick_events: &[TickEvent],
        config: &GameConfig,
        research_state: &ResearchState,
    ) -> Vec<AIAction> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let current_tick = tick_events.last().unwrap().tick;

        if !state.should_evaluate(current_tick, config.ai.tick_interval) {
            return Vec::new();
        }

        state.last_eval_tick = current_tick;

        let faction = match state.faction {
            Some(f) => f,
            None => return Vec::new(),
        };

        let mut actions = Vec::new();

        // Run each heuristic module.
        Self::evaluate_officers(state, world, faction, config, &mut actions);
        Self::evaluate_espionage(state, world, faction, config, &mut actions);
        Self::evaluate_research(state, world, research_state, faction, &mut actions);
        Self::evaluate_production(world, mfg_state, faction, config, &mut actions);
        Self::evaluate_fleet_deployment(state, world, movement, faction, current_tick, config, &mut actions);

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
        config: &GameConfig,
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
        let diplomacy_target = Self::find_diplomacy_target(world, faction, config.ai.diplomacy_target_popularity_cap);

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

            // Role-based character assignment:
            // 1. Jedi-capable characters → Jedi training priority (if not yet trained)
            // 2. High diplomacy → diplomacy missions
            // 3. High espionage → espionage handled by evaluate_espionage
            // 4. Major characters with unrecruited allies → recruitment
            // 5. Fleet admirals (can_be_admiral + high combat) → assigned to fleets in fleet_deployment

            let diplomacy_score =
                character.diplomacy.base + character.diplomacy.variance / 2;
            let combat_score =
                character.combat.base + character.combat.variance / 2;

            // Jedi-potential characters should not be wasted on diplomacy
            // (they'll train via the Jedi system automatically).
            if character.jedi_probability > 50 && character.force_tier == crate::world::ForceTier::None {
                // Skip — let them be available for Jedi training events.
                continue;
            }

            // High-diplomacy characters: send on diplomacy missions.
            if diplomacy_score > config.ai.diplomacy_skill_threshold {
                if let Some(target) = diplomacy_target {
                    actions.push(AIAction::DispatchMission {
                        kind: MissionKind::Diplomacy,
                        character: char_key,
                        target_system: target,
                        duration_roll: 0.5,
                    });
                    continue;
                }
            }

            // Major characters with unrecruited allies → recruitment.
            if character.is_major && unrecruited_count > 0 {
                if let Some(base_system) = Self::find_friendly_system(world, faction) {
                    actions.push(AIAction::DispatchMission {
                        kind: MissionKind::Recruitment,
                        character: char_key,
                        target_system: base_system,
                        duration_roll: 0.5,
                    });
                    continue;
                }
            }

            // Remaining major characters with decent diplomacy → diplomacy fallback.
            if character.is_major && diplomacy_score > 30 {
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
    /// Only returns systems below the popularity cap — systems
    /// already firmly ours are not worth spending characters on.
    fn find_diplomacy_target(world: &GameWorld, faction: AiFaction, cap: f32) -> Option<SystemKey> {
        world
            .systems
            .iter()
            .filter(|(_, s)| faction.system_popularity(s) < cap)
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
        config: &GameConfig,
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
                if esp >= config.ai.espionage_skill_threshold {
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
            if ops_queued >= config.ai.max_covert_ops_per_eval || op_idx >= operatives.len() {
                break;
            }
            let (char_key, esp_score) = operatives[op_idx];
            if !Self::expected_success(world, MissionKind::Sabotage, esp_score, config.ai.covert_min_success_prob) {
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
                    AiFaction::Alliance => s.popularity_empire   > config.ai.covert_target_popularity_threshold,
                    AiFaction::Empire   => s.popularity_alliance > config.ai.covert_target_popularity_threshold,
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
                if ops_queued >= config.ai.max_covert_ops_per_eval || op_idx >= operatives.len() {
                    break;
                }
                let (char_key, _) = operatives[op_idx];
                let char = &world.characters[char_key];
                let combat_score = char.combat.base + char.combat.variance / 2;
                if !Self::expected_success(world, MissionKind::Assassination, combat_score, config.ai.covert_min_success_prob) {
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
            if ops_queued >= config.ai.max_covert_ops_per_eval || op_idx >= operatives.len() {
                break;
            }
            let (char_key, esp_score) = operatives[op_idx];
            if !Self::expected_success(world, MissionKind::Espionage, esp_score, config.ai.covert_min_success_prob) {
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
    /// Returns true if expected success probability ≥ the configured minimum.
    fn expected_success(world: &GameWorld, kind: MissionKind, skill_score: u32, min_prob: f64) -> bool {
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

        prob_pct / 100.0 >= min_prob
    }

    // -----------------------------------------------------------------------
    // Research heuristics
    // -----------------------------------------------------------------------

    /// Assign idle characters with research skills to tech tree advancement.
    ///
    /// For each tech tree (Ship, Troop, Facility), if no project is active for
    /// this faction, find the best available character and dispatch them.
    /// Characters are matched by their primary research skill:
    /// - Ship: `ship_design`
    /// - Troop: `troop_training`
    /// - Facility: `facility_design`
    fn evaluate_research(
        state: &AIState,
        world: &GameWorld,
        research_state: &ResearchState,
        faction: AiFaction,
        actions: &mut Vec<AIAction>,
    ) {
        let is_alliance = matches!(faction, AiFaction::Alliance);

        for &tech in &[TechType::Ship, TechType::Troop, TechType::Facility] {
            // Skip if there's already an active project for this faction + tree.
            let has_active = research_state.projects.iter().any(|p| {
                p.faction_is_alliance == is_alliance && p.tech_type == tech
            });
            if has_active {
                continue;
            }

            // Find the best available character for this tree.
            let best = world
                .characters
                .iter()
                .filter(|(key, c)| {
                    faction.owns_character(c)
                        && c.can_be_commander
                        && !state.is_busy(*key)
                })
                .filter_map(|(key, c)| {
                    let skill = match tech {
                        TechType::Ship => c.ship_design.base + c.ship_design.variance / 2,
                        TechType::Troop => c.troop_training.base + c.troop_training.variance / 2,
                        TechType::Facility => c.facility_design.base + c.facility_design.variance / 2,
                    };
                    // Only consider characters with meaningful skill (>= 30).
                    if skill >= 30 { Some((key, skill)) } else { None }
                })
                .max_by_key(|&(_, skill)| skill);

            if let Some((char_key, _)) = best {
                let current_level = research_state.level(is_alliance, tech);
                let ticks = ResearchSystem::ticks_for_next_level(world, is_alliance, tech, current_level);
                actions.push(AIAction::DispatchResearch {
                    character: char_key,
                    tech_type: tech,
                    ticks,
                });
            }
        }
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
        config: &GameConfig,
        actions: &mut Vec<AIAction>,
    ) {
        let best_fighter = Self::best_fighter_class(world, faction);
        let best_capship = Self::best_capital_ship_class(world, faction);

        // Count existing fleet assets to decide what to build next.
        let is_alliance = matches!(faction, AiFaction::Alliance);
        let our_capship_count: usize = world
            .fleets
            .values()
            .filter(|f| f.is_alliance == is_alliance)
            .flat_map(|f| &f.capital_ships)
            .map(|e| e.count as usize)
            .sum();
        let our_fighter_count: usize = world
            .fleets
            .values()
            .filter(|f| f.is_alliance == is_alliance)
            .flat_map(|f| &f.fighters)
            .map(|e| e.count as usize)
            .sum();

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
            let queue_len = queue.map(|q| q.len()).unwrap_or(0);

            // Allow up to 3 items in queue (don't just wait for empty).
            if queue_len >= 3 {
                continue;
            }

            // Production priority: capital ships first (they create fleets),
            // then fighters (fill carrier slots), then construction yards.
            // Capital ships are the bottleneck — without them, no fleets.
            if our_capship_count < config.production.capship_threshold {
                if let Some((capship_key, capship_class)) = best_capship {
                    let ticks = capship_class.refined_material_cost.max(20);
                    actions.push(AIAction::EnqueueProduction {
                        system: sys_key,
                        kind: BuildableKind::CapitalShip(capship_key),
                        ticks,
                    });
                    continue;
                }
            }

            // Build fighters: either to fill carrier capacity, or as primary
            // combat units when no capital ship class is available.
            let needs_fighters = if best_capship.is_some() {
                our_fighter_count < our_capship_count * config.production.fighter_ratio
            } else {
                true // no capships available, fighters are our only option
            };
            if needs_fighters {
                if let Some((fighter_key, fighter_class)) = best_fighter {
                    let ticks = fighter_class.refined_material_cost.max(5);
                    actions.push(AIAction::EnqueueProduction {
                        system: sys_key,
                        kind: BuildableKind::Fighter(fighter_key),
                        ticks,
                    });
                    continue;
                }
            }

            // Build more construction yards if below cap.
            let yard_count = system.manufacturing_facilities.len();
            if yard_count < config.ai.max_construction_yards {
                if let Some(mfg_key) =
                    Self::find_manufacturing_facility_class(world, faction)
                {
                    actions.push(AIAction::EnqueueProduction {
                        system: sys_key,
                        kind: BuildableKind::ManufacturingFacility(mfg_key),
                        ticks: 30,
                    });
                    continue;
                }
            }

            // Default: build more capital ships.
            if let Some((capship_key, capship_class)) = best_capship {
                let ticks = capship_class.refined_material_cost.max(20);
                actions.push(AIAction::EnqueueProduction {
                    system: sys_key,
                    kind: BuildableKind::CapitalShip(capship_key),
                    ticks,
                });
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

    /// Select the most advanced capital ship class available for this faction.
    /// Prefers higher hull (stronger ships) as the tiebreaker.
    fn best_capital_ship_class(
        world: &GameWorld,
        faction: AiFaction,
    ) -> Option<(CapitalShipKey, &crate::world::CapitalShipClass)> {
        world
            .capital_ship_classes
            .iter()
            .filter(|(_, cs)| match faction {
                AiFaction::Alliance => cs.is_alliance,
                AiFaction::Empire => cs.is_empire,
            })
            .max_by_key(|(_, cs)| cs.hull)
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
    /// Compute a garrison strength score for a system.
    /// Counts ships (hull total), troop regiments, and defense facilities.
    /// Higher = more heavily defended.
    fn system_strength(world: &GameWorld, sys: &crate::world::System, is_alliance: bool) -> u32 {
        // Ship hull total from friendly fleets at this system
        let ship_strength: u32 = sys
            .fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .filter(|f| f.is_alliance == is_alliance)
            .flat_map(|f| &f.capital_ships)
            .filter_map(|entry| world.capital_ship_classes.get(entry.class))
            .map(|c| c.hull * 1) // each ship hull contributes
            .sum();

        // Troop strength from friendly ground units
        let troop_strength: u32 = sys
            .ground_units
            .iter()
            .filter_map(|&tk| world.troops.get(tk))
            .filter(|t| t.is_alliance == is_alliance)
            .map(|t| t.regiment_strength as u32)
            .sum();

        // Facility count (defense + manufacturing)
        let facility_count = sys.defense_facilities.len() as u32
            + sys.manufacturing_facilities.len() as u32;

        ship_strength + troop_strength + facility_count * 10
    }

    /// Categorize all systems into strategic buckets for fleet deployment.
    fn evaluate_galaxy_state(
        world: &GameWorld,
        faction: AiFaction,
    ) -> GalaxyState {
        use crate::dat::Faction;
        let our_faction = match faction {
            AiFaction::Alliance => Faction::Alliance,
            AiFaction::Empire => Faction::Empire,
        };
        let enemy_faction = match faction {
            AiFaction::Alliance => Faction::Empire,
            AiFaction::Empire => Faction::Alliance,
        };
        let is_alliance = matches!(faction, AiFaction::Alliance);

        let mut state = GalaxyState::default();
        for (key, sys) in world.systems.iter() {
            let has_our_fleet = sys.fleets.iter().any(|&fk|
                world.fleets.get(fk).map(|f| f.is_alliance == is_alliance).unwrap_or(false));
            let has_enemy_fleet = sys.fleets.iter().any(|&fk|
                world.fleets.get(fk).map(|f| f.is_alliance != is_alliance).unwrap_or(false));

            match sys.control {
                ControlKind::Controlled(f) if f == our_faction => {
                    if has_enemy_fleet {
                        state.contested.push(key);
                    } else if sys.is_headquarters {
                        state.our_hq = Some(key);
                        state.our_controlled.push(key);
                    } else {
                        state.our_controlled.push(key);
                        if !has_our_fleet {
                            state.our_undefended.push(key);
                        }
                    }
                }
                ControlKind::Controlled(f) if f == enemy_faction => {
                    if sys.is_headquarters {
                        state.enemy_hq = Some(key);
                    }
                    state.enemy_controlled.push(key);
                }
                _ => {
                    state.unoccupied.push(key);
                }
            }
        }

        // Sort attack targets by weakness (lowest enemy garrison first)
        state.enemy_controlled.sort_by_key(|&k| {
            world.systems.get(k)
                .map(|s| Self::system_strength(world, s, !is_alliance))
                .unwrap_or(u32::MAX)
        });

        // ── FUN_0053e190 port: ratio-based aggression scaling ──
        let our = state.our_controlled.len() as f64;
        let enemy = state.enemy_controlled.len() as f64;
        let total = our + enemy;
        state.control_ratio = if total > 0.0 { our / total } else { 0.5 };

        // Aggression curve: weak → defensive, dominant → offensive.
        // Linear interpolation with clamped floor/ceiling.
        // 0% control → 0.1 aggression, 50% → 0.5, 100% → 0.9
        state.aggression = (state.control_ratio * 0.8 + 0.1).clamp(0.1, 0.9);

        state
    }

    /// Score an enemy system as an attack target for a specific fleet.
    /// Higher score = better target. Considers weakness, proximity,
    /// deconfliction (avoid piling), and battle freshness (avoid stagnation).
    fn score_attack_target(
        world: &GameWorld,
        fleet_location: SystemKey,
        target: SystemKey,
        is_alliance: bool,
        targeted_counts: &HashMap<SystemKey, usize>,
        battle_cooldowns: &HashMap<SystemKey, u64>,
        current_tick: u64,
        config: &GameConfig,
    ) -> f64 {
        let target_sys = match world.systems.get(target) {
            Some(s) => s,
            None => return 0.0,
        };
        let fleet_sys = match world.systems.get(fleet_location) {
            Some(s) => s,
            None => return 0.0,
        };

        // Weakness: inverse of enemy garrison strength
        let strength = Self::system_strength(world, target_sys, !is_alliance) as f64;
        let weakness = 1.0 / (1.0 + strength);

        // Proximity: inverse of Euclidean distance
        let dx = target_sys.x as f64 - fleet_sys.x as f64;
        let dy = target_sys.y as f64 - fleet_sys.y as f64;
        let distance = (dx * dx + dy * dy).sqrt();
        let proximity = 1.0 / (1.0 + distance / config.ai.proximity_divisor);

        // AUGMENTATION: Deconfliction — avoid piling multiple fleets on same target
        let pile_count = *targeted_counts.get(&target).unwrap_or(&0) as f64;
        let deconfliction = 1.0 / (1.0 + pile_count);

        // AUGMENTATION: Battle freshness — deprioritize recently-fought systems
        let freshness = match battle_cooldowns.get(&target) {
            Some(&last_tick) => {
                let elapsed = current_tick.saturating_sub(last_tick) as f64;
                (elapsed / config.ai.battle_cooldown_ticks).min(1.0)
            }
            None => 1.0,
        };

        weakness * config.ai.weight_weakness
            + proximity * config.ai.weight_proximity
            + deconfliction * config.ai.weight_deconfliction
            + freshness * config.ai.weight_freshness
    }

    /// Two-pass fleet deployment — ports the original game's distributed targeting.
    ///
    /// Pass 1: Assign each fleet its own target using scoring function.
    ///   - HQ garrison first, then per-fleet attack targeting.
    /// Pass 2: Redistribute any idle fleets (no valid target in pass 1).
    fn evaluate_fleet_deployment(
        state: &AIState,
        world: &GameWorld,
        movement: &crate::movement::MovementState,
        faction: AiFaction,
        current_tick: u64,
        config: &GameConfig,
        actions: &mut Vec<AIAction>,
    ) {
        let is_alliance = matches!(faction, AiFaction::Alliance);
        let galaxy = Self::evaluate_galaxy_state(world, faction);

        if galaxy.enemy_controlled.is_empty() {
            return; // No enemy targets — nothing to attack
        }

        // Build transient deconfliction map from active movement orders.
        let mut targeted_counts: HashMap<SystemKey, usize> = HashMap::new();
        for (_, order) in movement.orders() {
            if let Some(f) = world.fleets.get(order.fleet) {
                if f.is_alliance == is_alliance {
                    *targeted_counts.entry(order.destination).or_default() += 1;
                }
            }
        }

        // Check if HQ already has a fleet stationed.
        let mut hq_defended = false;
        if let Some(hq) = galaxy.our_hq {
            if let Some(sys) = world.systems.get(hq) {
                hq_defended = sys.fleets.iter().any(|&fk| {
                    world.fleets.get(fk)
                        .map(|f| f.is_alliance == is_alliance)
                        .unwrap_or(false)
                });
            }
        }

        // Collect our idle fleets (not in combat, not already assigned).
        let mut idle_fleets: Vec<(FleetKey, SystemKey)> = Vec::new();
        for (fleet_key, fleet) in world.fleets.iter() {
            let is_ours = if is_alliance { fleet.is_alliance } else { !fleet.is_alliance };
            if !is_ours { continue; }

            // Skip fleets currently in combat (enemy present at their location).
            let in_combat = world.systems.get(fleet.location)
                .map(|s| s.fleets.iter().any(|&fk| {
                    world.fleets.get(fk)
                        .map(|f| f.is_alliance != fleet.is_alliance)
                        .unwrap_or(false)
                }))
                .unwrap_or(false);
            if in_combat { continue; }

            idle_fleets.push((fleet_key, fleet.location));
        }

        // ── Pass 1: Assign each fleet its best target ───────────────

        let mut pass2_idle: Vec<(FleetKey, SystemKey)> = Vec::new();

        for (fleet_key, fleet_location) in &idle_fleets {
            let fleet = match world.fleets.get(*fleet_key) { Some(f) => f, None => continue };

            // Death Star exemption: always target enemy HQ
            if fleet.has_death_star {
                if let Some(hq) = galaxy.enemy_hq {
                    if *fleet_location != hq {
                        actions.push(AIAction::MoveFleet {
                            fleet: *fleet_key,
                            to_system: hq,
                            reason: FleetMoveReason::Attack,
                        });
                        *targeted_counts.entry(hq).or_default() += 1;
                    }
                }
                continue;
            }

            // HQ garrison: first available fleet
            if !hq_defended {
                if let Some(hq) = galaxy.our_hq {
                    if *fleet_location != hq {
                        actions.push(AIAction::MoveFleet {
                            fleet: *fleet_key,
                            to_system: hq,
                            reason: FleetMoveReason::Reinforce,
                        });
                    }
                    hq_defended = true;
                    continue;
                }
            }

            // Per-fleet attack targeting: score all enemy systems, pick best.
            // Cap simultaneous attack fronts scaled by aggression.
            // Weak faction (aggression=0.2): 1 front. Dominant (0.8): up to config max.
            let aggression_fronts = (galaxy.aggression * config.ai.max_attack_fronts as f64).ceil() as usize;
            let max_fronts = idle_fleets.len().min(aggression_fronts.max(1));
            let distinct_targets = targeted_counts.values().filter(|&&v| v > 0).count();

            // Faction asymmetry: Empire biases toward enemy HQ.
            let candidates: Vec<SystemKey> = if matches!(faction, AiFaction::Empire) && galaxy.enemy_hq.is_some() {
                // Empire: enemy HQ always in candidate list + weakest systems
                let mut c = galaxy.enemy_controlled.clone();
                if let Some(hq) = galaxy.enemy_hq {
                    if !c.contains(&hq) { c.insert(0, hq); }
                }
                c
            } else {
                galaxy.enemy_controlled.clone()
            };

            if candidates.is_empty() {
                pass2_idle.push((*fleet_key, *fleet_location));
                continue;
            }

            // Score each candidate and pick the best
            let best = candidates.iter()
                .map(|&target| {
                    let score = Self::score_attack_target(
                        world, *fleet_location, target, is_alliance,
                        &targeted_counts, &state.battle_cooldowns, current_tick, config,
                    );
                    (target, score)
                })
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            if let Some((target, _score)) = best {
                if target != *fleet_location {
                    // Only open a new front if we haven't hit the cap,
                    // or if this target is already being attacked.
                    let is_new_front = targeted_counts.get(&target).copied().unwrap_or(0) == 0;
                    if !is_new_front || distinct_targets < max_fronts {
                        actions.push(AIAction::MoveFleet {
                            fleet: *fleet_key,
                            to_system: target,
                            reason: FleetMoveReason::Attack,
                        });
                        *targeted_counts.entry(target).or_default() += 1;
                        continue;
                    }
                }
            }

            pass2_idle.push((*fleet_key, *fleet_location));
        }

        // ── Pass 2: Redistribute idle fleets ────────────────────────
        // Behavior scaled by aggression:
        // - High aggression: pile onto existing attack targets (reinforce the assault)
        // - Low aggression: reinforce undefended friendly systems (hunker down)
        for (fleet_key, fleet_location) in pass2_idle {
            if galaxy.aggression > 0.5 {
                // Offensive: reinforce the most-targeted enemy system (pile onto attack)
                let best_attack = targeted_counts.iter()
                    .filter(|(_, &count)| count > 0)
                    .max_by_key(|(_, &count)| count)
                    .map(|(&sys, _)| sys);
                if let Some(target) = best_attack {
                    if target != fleet_location {
                        actions.push(AIAction::MoveFleet {
                            fleet: fleet_key,
                            to_system: target,
                            reason: FleetMoveReason::Attack,
                        });
                        *targeted_counts.entry(target).or_default() += 1;
                        continue;
                    }
                }
            }
            // Defensive fallback: distribute across undefended systems proportionally.
            // FUN_005385f0 port: original distributes quotient per system + remainder
            // to the weakest. We round-robin across undefended, then fall back to all controlled.
            let candidates = if !galaxy.our_undefended.is_empty() {
                &galaxy.our_undefended
            } else if !galaxy.our_controlled.is_empty() {
                &galaxy.our_controlled
            } else {
                continue;
            };
            // Pick the candidate with fewest incoming reinforcements (distribute evenly).
            let target = candidates.iter()
                .copied()
                .filter(|&sys| sys != fleet_location)
                .min_by_key(|sys| targeted_counts.get(sys).copied().unwrap_or(0))
                .or_else(|| candidates.first().copied());
            if let Some(t) = target {
                if t != fleet_location {
                    actions.push(AIAction::MoveFleet {
                        fleet: fleet_key,
                        to_system: t,
                        reason: FleetMoveReason::Reinforce,
                    });
                    *targeted_counts.entry(t).or_default() += 1;
                }
            }
        }
    }

    /// Record that combat occurred at a system (called by simulation after combat).
    pub fn record_battle(state: &mut AIState, system: SystemKey, tick: u64) {
        state.battle_cooldowns.insert(system, tick);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{DatId, SectorKey};
    use crate::tuning::GameConfig;
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
            control: ControlKind::Uncontrolled,
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
        assert!(state.should_evaluate(0, AI_TICK_INTERVAL));
    }

    #[test]
    fn should_not_evaluate_before_interval() {
        let mut state = AIState::new(AiFaction::Empire);
        state.last_eval_tick = 0;
        // 3 days elapsed — interval is 7
        assert!(!state.should_evaluate(3, AI_TICK_INTERVAL));
    }

    #[test]
    fn should_evaluate_after_interval() {
        let mut state = AIState::new(AiFaction::Empire);
        state.last_eval_tick = 0;
        assert!(state.should_evaluate(7, AI_TICK_INTERVAL));
        assert!(state.should_evaluate(14, AI_TICK_INTERVAL));
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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &[], &GameConfig::default(), &crate::research::ResearchState::new());
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
            AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &[TickEvent { tick: 8 }], &GameConfig::default(), &crate::research::ResearchState::new());
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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());
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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());
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
            control: ControlKind::Uncontrolled,
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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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

        // Empire fleet at a safe friendly system (Empire-controlled)
        let home_sys = add_system(&mut world, sector, 0.1, 0.9);
        world.systems[home_sys].control = ControlKind::Controlled(crate::dat::Faction::Empire);
        // Enemy system controlled by Alliance (attack target)
        let target_sys = add_system(&mut world, sector, 0.6, 0.15);
        world.systems[target_sys].control = ControlKind::Controlled(crate::dat::Faction::Alliance);

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

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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
            control: ControlKind::Uncontrolled,
        });

        // Empire spy with high espionage — above threshold.
        let spy = add_spy(&mut world, false, false, 80, 50);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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
            control: ControlKind::Uncontrolled,
        });

        // Empire spy with low espionage — below threshold.
        let _ = add_spy(&mut world, false, false, 20, 20);

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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
            control: ControlKind::Uncontrolled,
        });

        // Alliance spy.
        let spy = add_spy(&mut world, true, false, 75, 40);

        let mut state = AIState::new(AiFaction::Alliance);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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
                control: ControlKind::Uncontrolled,
            });
        }

        // 5 Empire spies — all highly skilled.
        for _ in 0..5 {
            add_spy(&mut world, false, false, 80, 60);
        }

        let mut state = AIState::new(AiFaction::Empire);
        let mfg = ManufacturingState::new();
        let missions = MissionState::new();

        let actions = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &ticks(7), &GameConfig::default(), &crate::research::ResearchState::new());

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
        let _first = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &[TickEvent { tick: 7 }], &GameConfig::default(), &crate::research::ResearchState::new());
        assert_eq!(state.last_eval_tick, 7);

        // Tick 10 — only 3 days elapsed, should skip
        let second = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &[TickEvent { tick: 10 }], &GameConfig::default(), &crate::research::ResearchState::new());
        assert!(second.is_empty(), "expected no actions before interval elapses");
        assert_eq!(state.last_eval_tick, 7); // unchanged

        // Tick 14 — 7 days elapsed, should evaluate again
        let third = AISystem::advance(&mut state, &world, &mfg, &missions, &crate::movement::MovementState::new(), &[TickEvent { tick: 14 }], &GameConfig::default(), &crate::research::ResearchState::new());
        assert_eq!(state.last_eval_tick, 14);
        let _ = third; // just checking it ran
    }
}
