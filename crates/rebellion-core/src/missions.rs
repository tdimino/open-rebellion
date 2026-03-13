//! Mission system: diplomacy and recruitment mission lifecycle.
//!
//! Missions are assigned to characters and execute over multiple game-days.
//! When a mission completes, success is determined by a quadratic probability
//! formula derived from the character's relevant skill score.
//!
//! # Design
//!
//! Follows the same stateless pattern as the manufacturing system:
//! - `MissionState` holds all active missions (across all factions, all systems)
//! - `MissionSystem::advance(state, world, tick_events, rng_rolls)` returns `Vec<MissionResult>`
//! - The caller provides pre-generated random rolls so rebellion-core stays
//!   dependency-free and fully deterministic in tests
//!
//! # Probability Formula
//!
//! Ported from rebellion2's Mission.cs (quadratic):
//! ```text
//! agent_prob = clamp(a·score² + b·score + c,  min%, max%)
//! total_prob = agent_prob · (1 − foil_prob)
//! success    = roll < total_prob
//! ```
//! Coefficients are mission-type constants defined in `MissionKind`.
//!
//! # Lifecycle
//!
//! 1. Caller creates `ActiveMission` with `ActiveMission::new(...)` and enqueues it
//!    into `MissionState`.
//! 2. Each tick the mission's `ticks_remaining` decrements.
//! 3. At zero, `MissionSystem` resolves the outcome and emits a `MissionResult`.
//! 4. Repeatable missions (diplomacy below threshold, recruitment with officers
//!    remaining) are re-queued automatically by the caller based on the result.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::ids::{CharacterKey, SystemKey};
use crate::tick::TickEvent;
use crate::world::{Character, GameWorld};

// ---------------------------------------------------------------------------
// MissionKind
// ---------------------------------------------------------------------------

/// The two mission types in Living Galaxy scope.
///
/// Additional types (sabotage, assassination, rescue, incite uprising) are
/// reserved for later phases once MISSNSD.DAT is parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionKind {
    /// Shift a system's popularity toward the sending faction.
    ///
    /// Uses the character's `diplomacy` skill.
    /// Coefficients from rebellion2 DiplomacyMission.cs.
    Diplomacy,

    /// Recruit an available character for the sending faction.
    ///
    /// Uses the character's `leadership` skill.
    /// Coefficients from rebellion2 RecruitmentMission.cs.
    Recruitment,
}

impl MissionKind {
    /// Quadratic success-probability coefficients: (a, b, c).
    ///
    /// Formula: `prob% = a·score² + b·score + c`, clamped to [min%, max%].
    pub fn coefficients(self) -> (f64, f64, f64) {
        match self {
            MissionKind::Diplomacy => (0.005558, 0.7656, 20.15),
            MissionKind::Recruitment => (-0.001748, 0.8657, 11.923),
        }
    }

    /// Minimum success probability (percent, 1–100).
    pub fn min_success_prob(self) -> f64 {
        1.0
    }

    /// Maximum success probability (percent, 1–100).
    pub fn max_success_prob(self) -> f64 {
        100.0
    }

    /// Mission duration range in game-days: (min_ticks, max_ticks).
    ///
    /// The actual duration is chosen randomly by the caller at dispatch time
    /// using `MissionKind::sample_duration(roll)`.
    pub fn tick_range(self) -> (u32, u32) {
        // Both types share the same range from rebellion2 (15–20 days).
        (15, 20)
    }

    /// Sample a concrete duration from the tick range given a uniform roll in [0,1].
    pub fn sample_duration(self, roll: f64) -> u32 {
        let (min, max) = self.tick_range();
        // Map [0,1] → [min, max] inclusive, clamped to guard against roll == 1.0.
        let raw = min + (roll * (max - min + 1) as f64).floor() as u32;
        raw.min(max)
    }
}

// ---------------------------------------------------------------------------
// MissionFaction
// ---------------------------------------------------------------------------

/// Which faction is sending the mission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionFaction {
    Alliance,
    Empire,
}

// ---------------------------------------------------------------------------
// ActiveMission
// ---------------------------------------------------------------------------

/// A mission currently in progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveMission {
    /// Unique identifier within `MissionState` (sequential, never reused).
    pub id: u64,
    /// What kind of mission this is.
    pub kind: MissionKind,
    /// Which faction dispatched this mission.
    pub faction: MissionFaction,
    /// The character conducting the mission.
    pub character: CharacterKey,
    /// The target system.
    pub target_system: SystemKey,
    /// Game-days remaining until execution.
    pub ticks_remaining: u32,
    /// Original duration (for progress display).
    pub total_ticks: u32,
}

impl ActiveMission {
    /// Create a new mission ready for insertion into `MissionState`.
    pub fn new(
        id: u64,
        kind: MissionKind,
        faction: MissionFaction,
        character: CharacterKey,
        target_system: SystemKey,
        duration: u32,
    ) -> Self {
        ActiveMission {
            id,
            kind,
            faction,
            character,
            target_system,
            ticks_remaining: duration,
            total_ticks: duration,
        }
    }

    /// Progress fraction in [0.0, 1.0].
    pub fn progress_fraction(&self) -> f32 {
        if self.total_ticks == 0 {
            return 1.0;
        }
        1.0 - (self.ticks_remaining as f32 / self.total_ticks as f32)
    }
}

// ---------------------------------------------------------------------------
// MissionState
// ---------------------------------------------------------------------------

/// All active missions across the galaxy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MissionState {
    missions: VecDeque<ActiveMission>,
    next_id: u64,
}

impl MissionState {
    pub fn new() -> Self {
        MissionState {
            missions: VecDeque::new(),
            next_id: 0,
        }
    }

    /// Dispatch a new mission and return its assigned id.
    ///
    /// `duration_roll` — uniform [0,1) used to sample mission duration.
    pub fn dispatch(
        &mut self,
        kind: MissionKind,
        faction: MissionFaction,
        character: CharacterKey,
        target_system: SystemKey,
        duration_roll: f64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let duration = kind.sample_duration(duration_roll);
        self.missions.push_back(ActiveMission::new(
            id,
            kind,
            faction,
            character,
            target_system,
            duration,
        ));
        id
    }

    /// Cancel a mission by id. Returns the mission if found, None otherwise.
    pub fn cancel(&mut self, id: u64) -> Option<ActiveMission> {
        if let Some(pos) = self.missions.iter().position(|m| m.id == id) {
            self.missions.remove(pos)
        } else {
            None
        }
    }

    /// All active missions (read-only).
    pub fn missions(&self) -> &VecDeque<ActiveMission> {
        &self.missions
    }

    pub fn len(&self) -> usize {
        self.missions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.missions.is_empty()
    }
}

// ---------------------------------------------------------------------------
// MissionEffect
// ---------------------------------------------------------------------------

/// What actually changed in the game world as a result of a mission.
#[derive(Debug, Clone, PartialEq)]
pub enum MissionEffect {
    /// System popularity shifted toward the faction by `delta` (0.0–1.0 scale).
    PopularityShifted {
        system: SystemKey,
        faction: MissionFaction,
        /// Amount added to the faction's popularity (rebellion2 uses +1/100 per success).
        delta: f32,
    },
    /// A character was recruited to the faction (placed at the target system).
    CharacterRecruited {
        system: SystemKey,
        /// The character who conducted the recruitment.
        recruiter: CharacterKey,
        faction: MissionFaction,
    },
}

// ---------------------------------------------------------------------------
// MissionOutcome / MissionResult
// ---------------------------------------------------------------------------

/// Whether a mission succeeded, failed, or was foiled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionOutcome {
    Success,
    Failure,
    /// Enemy counter-intelligence detected and blocked the mission.
    Foiled,
}

/// The result emitted when a mission resolves.
#[derive(Debug, Clone)]
pub struct MissionResult {
    /// The id of the mission that resolved.
    pub mission_id: u64,
    /// The game-day on which the mission resolved.
    pub tick: u64,
    pub kind: MissionKind,
    pub faction: MissionFaction,
    pub character: CharacterKey,
    pub target_system: SystemKey,
    pub outcome: MissionOutcome,
    /// World-state changes to apply (only non-empty on Success).
    pub effects: Vec<MissionEffect>,
}

// ---------------------------------------------------------------------------
// Probability helpers (pure functions, testable)
// ---------------------------------------------------------------------------

/// Evaluate the quadratic success-probability formula.
///
/// Returns a probability in percent [0, 100].
pub fn quadratic_prob(score: f64, a: f64, b: f64, c: f64) -> f64 {
    a * score * score + b * score + c
}

/// Clamp a probability to the mission's configured [min%, max%] range.
pub fn clamp_prob(p: f64, min: f64, max: f64) -> f64 {
    p.max(min).min(max)
}

/// Combined success probability: `agent_prob% * (1 - foil_prob%)`.
///
/// Both inputs are in percent [0, 100]; output is in percent [0, 100].
pub fn total_success_prob(agent_prob_pct: f64, foil_prob_pct: f64) -> f64 {
    let agent = agent_prob_pct / 100.0;
    let foil = foil_prob_pct / 100.0;
    agent * (1.0 - foil) * 100.0
}

/// Foil probability from defense score.
///
/// Coefficients from Mission.cs FoilProbability: -0.001999·d² + 0.8879·d + 84.61.
/// Returns 0.0 if the mission is in a friendly system (no counter-intel threat).
pub fn foil_prob(defense_score: f64, own_system: bool) -> f64 {
    if own_system {
        return 0.0;
    }
    quadratic_prob(defense_score, -0.001999, 0.8879, 84.61).max(0.0).min(100.0)
}

// ---------------------------------------------------------------------------
// MissionSystem
// ---------------------------------------------------------------------------

/// Stateless system that advances missions per tick and resolves completions.
///
/// # RNG contract
///
/// The caller provides `rolls`: one `f64` roll per mission that completes this
/// frame, in the order they resolve. Each roll is uniform [0, 1). If more
/// missions complete than rolls provided, remaining missions use `0.5`
/// (deterministic fallback — only happens if caller under-provides).
///
/// For production use, generate rolls with `rand::random::<f64>()` per mission.
/// For tests, pass explicit values to exercise specific outcomes.
pub struct MissionSystem;

impl MissionSystem {
    /// Advance all active missions by the ticks in `tick_events`.
    ///
    /// Missions that complete are resolved using the provided `rolls` (one per
    /// completing mission). Returns `Vec<MissionResult>` — one per completed
    /// mission, empty if none resolved this frame.
    ///
    /// **Important**: The caller is responsible for applying `MissionResult::effects`
    /// to `GameWorld` (popularity shifts, character faction assignment, etc.).
    pub fn advance(
        state: &mut MissionState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        rolls: &[f64],
    ) -> Vec<MissionResult> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let tick_count = tick_events.len() as u32;
        let final_tick = tick_events.last().unwrap().tick;
        let mut roll_iter = rolls.iter().copied();
        let mut results = Vec::new();

        for mission in state.missions.iter_mut() {
            mission.ticks_remaining = mission.ticks_remaining.saturating_sub(tick_count);
        }

        // Drain completed missions.
        // We iterate by draining from the front of the deque (FIFO order).
        let mut remaining = VecDeque::new();
        while let Some(mission) = state.missions.pop_front() {
            if mission.ticks_remaining == 0 {
                let roll = roll_iter.next().unwrap_or(0.5);
                let result =
                    Self::resolve_mission(&mission, world, final_tick, roll);
                results.push(result);
            } else {
                remaining.push_back(mission);
            }
        }
        state.missions = remaining;

        results
    }

    /// Resolve a single completed mission into a `MissionResult`.
    fn resolve_mission(
        mission: &ActiveMission,
        world: &GameWorld,
        tick: u64,
        roll: f64,
    ) -> MissionResult {
        let character = world.characters.get(mission.character);
        let (outcome, effects) =
            Self::determine_outcome(mission, character, tick, roll);

        MissionResult {
            mission_id: mission.id,
            tick,
            kind: mission.kind,
            faction: mission.faction,
            character: mission.character,
            target_system: mission.target_system,
            outcome,
            effects,
        }
    }

    /// Compute outcome and effects given a pre-rolled value.
    fn determine_outcome(
        mission: &ActiveMission,
        character: Option<&Character>,
        _tick: u64,
        roll: f64,
    ) -> (MissionOutcome, Vec<MissionEffect>) {
        // Skill score: base + half variance (expected value; variance resolved at scenario start).
        let skill_score = character
            .map(|c| {
                let pair = match mission.kind {
                    MissionKind::Diplomacy => c.diplomacy,
                    MissionKind::Recruitment => c.leadership,
                };
                // Use base + half variance as the expected skill.
                // Full variance resolution happens at game start (not stored here yet).
                pair.base as f64 + pair.variance as f64 * 0.5
            })
            .unwrap_or(0.0);

        let (a, b, c) = mission.kind.coefficients();
        let raw_prob = quadratic_prob(skill_score, a, b, c);
        let agent_prob =
            clamp_prob(raw_prob, mission.kind.min_success_prob(), mission.kind.max_success_prob());

        // For Living Galaxy scope, foil probability is 0 (no defense score system yet).
        // This is consistent with rebellion2's own-system behavior and is a safe stub
        // until espionage counters are implemented.
        let foil = foil_prob(0.0, true);
        let success_prob = total_success_prob(agent_prob, foil);

        let roll_pct = roll * 100.0;

        if roll_pct <= success_prob {
            let effects = Self::build_effects(mission);
            (MissionOutcome::Success, effects)
        } else {
            // No foil detection for Living Galaxy scope (no counter-espionage yet).
            (MissionOutcome::Failure, Vec::new())
        }
    }

    /// Build the world-state effects for a successful mission.
    fn build_effects(mission: &ActiveMission) -> Vec<MissionEffect> {
        match mission.kind {
            MissionKind::Diplomacy => {
                vec![MissionEffect::PopularityShifted {
                    system: mission.target_system,
                    faction: mission.faction,
                    // rebellion2 uses +1 on a 0–100 integer scale → +0.01 on our f32 [0,1]
                    delta: 0.01,
                }]
            }
            MissionKind::Recruitment => {
                vec![MissionEffect::CharacterRecruited {
                    system: mission.target_system,
                    recruiter: mission.character,
                    faction: mission.faction,
                }]
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
    use crate::ids::{CharacterKey, SystemKey};
    use crate::world::{Character, GameWorld, SkillPair};

    // --- Probability formula tests ---

    #[test]
    fn quadratic_prob_at_zero_score_equals_constant() {
        let (a, b, c) = MissionKind::Diplomacy.coefficients();
        let p = quadratic_prob(0.0, a, b, c);
        assert!((p - c).abs() < 1e-9, "expected {c}, got {p}");
    }

    #[test]
    fn quadratic_prob_diplomacy_at_max_skill() {
        // Max skill score is 100 (u32 base) — should be near 100%.
        let (a, b, c) = MissionKind::Diplomacy.coefficients();
        let p = quadratic_prob(100.0, a, b, c);
        let clamped = clamp_prob(p, 1.0, 100.0);
        assert!(clamped >= 90.0, "expected high probability at max skill, got {clamped}");
    }

    #[test]
    fn quadratic_prob_recruitment_at_zero_skill() {
        let (a, b, c) = MissionKind::Recruitment.coefficients();
        let p = quadratic_prob(0.0, a, b, c);
        let clamped = clamp_prob(p, 1.0, 100.0);
        // c = 11.923 — low but above min
        assert!(clamped >= 1.0 && clamped <= 100.0);
    }

    #[test]
    fn total_success_prob_no_foil() {
        // With 0% foil, total == agent prob.
        let total = total_success_prob(75.0, 0.0);
        assert!((total - 75.0).abs() < 1e-9);
    }

    #[test]
    fn total_success_prob_full_foil() {
        // With 100% foil, total == 0%.
        let total = total_success_prob(75.0, 100.0);
        assert!(total.abs() < 1e-9);
    }

    #[test]
    fn foil_prob_own_system_is_zero() {
        assert_eq!(foil_prob(99.0, true), 0.0);
    }

    #[test]
    fn foil_prob_enemy_system_increases_with_defense() {
        let low = foil_prob(0.0, false);
        let high = foil_prob(50.0, false);
        assert!(high > low, "expected foil to increase with defense score");
    }

    #[test]
    fn sample_duration_stays_in_range() {
        for i in 0..=10 {
            let roll = i as f64 / 10.0;
            let d = MissionKind::Diplomacy.sample_duration(roll);
            let (min, max) = MissionKind::Diplomacy.tick_range();
            assert!(d >= min && d <= max, "duration {d} out of [{min}, {max}]");
        }
    }

    // --- MissionState tests ---

    fn mock_keys() -> (SystemKey, CharacterKey) {
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let mut chr_sm: slotmap::SlotMap<CharacterKey, ()> = slotmap::SlotMap::with_key();
        (sys_sm.insert(()), chr_sm.insert(()))
    }

    #[test]
    fn dispatch_creates_mission_with_valid_duration() {
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        let id = state.dispatch(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            0.5,
        );
        assert_eq!(state.len(), 1);
        let m = &state.missions()[0];
        assert_eq!(m.id, id);
        let (min, max) = MissionKind::Diplomacy.tick_range();
        assert!(m.total_ticks >= min && m.total_ticks <= max);
    }

    #[test]
    fn cancel_removes_mission() {
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        let id = state.dispatch(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            0.5,
        );
        let removed = state.cancel(id);
        assert!(removed.is_some());
        assert!(state.is_empty());
    }

    // --- MissionSystem::advance tests ---

    fn minimal_world() -> GameWorld {
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

    fn skill_pair(base: u32) -> SkillPair {
        SkillPair { base, variance: 0 }
    }

    fn character_with_diplomacy(world: &mut GameWorld, diplomacy: u32) -> CharacterKey {
        let zero = skill_pair(0);
        world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Test".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: skill_pair(diplomacy),
            espionage: zero,
            ship_design: zero,
            troop_training: zero,
            facility_design: zero,
            combat: zero,
            leadership: zero,
            loyalty: zero,
            jedi_probability: 0,
            jedi_level: zero,
            can_be_admiral: false,
            can_be_commander: true,
            can_be_general: false,
        })
    }

    #[test]
    fn no_tick_events_no_results() {
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        state.dispatch(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            0.5,
        );
        let world = minimal_world();
        let results = MissionSystem::advance(&mut state, &world, &[], &[]);
        assert!(results.is_empty());
        assert_eq!(state.len(), 1);
    }

    #[test]
    fn mission_decrements_each_tick() {
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        // Dispatch with fixed duration of 15 ticks (roll=0.0 → min_ticks)
        state.dispatch(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            0.0,
        );
        let initial = state.missions()[0].ticks_remaining;
        let world = minimal_world();
        MissionSystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], &[0.99]);
        // Mission should still be active (15 ticks remaining at dispatch)
        if initial > 1 {
            assert_eq!(state.len(), 1);
            assert_eq!(state.missions()[0].ticks_remaining, initial - 1);
        }
    }

    #[test]
    fn mission_resolves_at_zero_ticks() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 80);

        let mut state = MissionState::new();
        // Manually insert a mission with 1 tick remaining
        state.missions.push_back(ActiveMission::new(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            1, // will resolve on next tick
        ));
        state.next_id = 1;

        // Roll = 0.01 → 1% of 100 → very likely to succeed with high-skill character
        let results = MissionSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 1 }],
            &[0.01],
        );
        assert_eq!(results.len(), 1);
        assert!(state.is_empty());
        assert_eq!(results[0].kind, MissionKind::Diplomacy);
    }

    #[test]
    fn guaranteed_success_with_roll_zero() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 50);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            1,
        ));
        state.next_id = 1;

        // roll = 0.0 → 0% of 100 → succeeds as long as success_prob > 0
        let results = MissionSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 1 }],
            &[0.0],
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success);
        assert!(!results[0].effects.is_empty());
    }

    #[test]
    fn guaranteed_failure_with_roll_one() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 50);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            1,
        ));
        state.next_id = 1;

        // roll = 1.0 → 100% of 100 → fails since success_prob <= 100
        let results = MissionSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 1 }],
            &[1.0],
        );
        assert_eq!(results[0].outcome, MissionOutcome::Failure);
        assert!(results[0].effects.is_empty());
    }

    #[test]
    fn diplomacy_success_emits_popularity_effect() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 90);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            1,
        ));
        state.next_id = 1;

        let results = MissionSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 1 }],
            &[0.0], // guaranteed success
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success);
        assert_eq!(results[0].effects.len(), 1);
        match &results[0].effects[0] {
            MissionEffect::PopularityShifted { faction, delta, .. } => {
                assert_eq!(*faction, MissionFaction::Alliance);
                assert!((delta - 0.01).abs() < 1e-6);
            }
            _ => panic!("expected PopularityShifted"),
        }
    }

    #[test]
    fn recruitment_success_emits_recruit_effect() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let zero = skill_pair(0);
        let character = world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Leader".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: zero,
            espionage: zero,
            ship_design: zero,
            troop_training: zero,
            facility_design: zero,
            combat: zero,
            leadership: skill_pair(80),
            loyalty: zero,
            jedi_probability: 0,
            jedi_level: zero,
            can_be_admiral: false,
            can_be_commander: true,
            can_be_general: false,
        });

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0,
            MissionKind::Recruitment,
            MissionFaction::Empire,
            character,
            system,
            1,
        ));
        state.next_id = 1;

        let results = MissionSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 5 }],
            &[0.0], // guaranteed success
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success);
        match &results[0].effects[0] {
            MissionEffect::CharacterRecruited { faction, .. } => {
                assert_eq!(*faction, MissionFaction::Empire);
            }
            _ => panic!("expected CharacterRecruited"),
        }
    }

    #[test]
    fn multiple_missions_resolve_independently() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let sys_a = sys_sm.insert(());
        let sys_b = sys_sm.insert(());
        let char_a = character_with_diplomacy(&mut world, 70);
        let char_b = character_with_diplomacy(&mut world, 70);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::Diplomacy, MissionFaction::Alliance, char_a, sys_a, 1,
        ));
        state.missions.push_back(ActiveMission::new(
            1, MissionKind::Diplomacy, MissionFaction::Empire, char_b, sys_b, 3,
        ));
        state.next_id = 2;

        let results = MissionSystem::advance(
            &mut state,
            &world,
            &[TickEvent { tick: 1 }],
            &[0.0], // first mission succeeds
        );
        // Only mission 0 completes (1 tick); mission 1 has 2 ticks left
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].mission_id, 0);
        assert_eq!(state.len(), 1);
        assert_eq!(state.missions()[0].ticks_remaining, 2);
    }
}
