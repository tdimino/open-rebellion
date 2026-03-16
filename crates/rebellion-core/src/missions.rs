//! Mission system: all 9 mission types with MSTB probability lookup.
//!
//! Missions are assigned to characters and execute over multiple game-days.
//! When a mission completes, success is determined either by a quadratic
//! probability formula (fallback) or by a piecewise-linear MSTB table lookup
//! when `world.mission_tables` is populated by rebellion-data.
//!
//! # Design
//!
//! Follows the same stateless pattern as the manufacturing system:
//! - `MissionState` holds all active missions (across all factions, all systems)
//! - `MissionSystem::advance(state, world, tick_events, rng_rolls)` returns `Vec<MissionResult>`
//! - The caller provides pre-generated random rolls so rebellion-core stays
//!   dependency-free and fully deterministic in tests
//!
//! # Probability
//!
//! Two paths, in priority order:
//! 1. **MSTB lookup**: if `world.mission_tables` contains an entry for this mission kind,
//!    use `MstbTable::lookup(skill_score)` (piecewise-linear over IntTableEntry thresholds).
//! 2. **Quadratic fallback**: `clamp(a·score² + b·score + c, min%, max%)` — same as
//!    rebellion2's Mission.cs coefficients. Used when MSTB tables are not yet loaded.
//!
//! Combined: `total_prob = agent_prob · (1 − foil_prob)`.
//!
//! # Mission Types
//!
//! Nine types ported from REBEXE.EXE's 13-case dispatch (FUN_0050d5a0):
//! Diplomacy, Recruitment, Sabotage, Assassination, Espionage, Rescue, Abduction,
//! InciteUprising, Autoscrap.
//!
//! # Lifecycle
//!
//! 1. Caller creates `ActiveMission` with `ActiveMission::new(...)` and enqueues it
//!    into `MissionState`.
//! 2. Each tick the mission's `ticks_remaining` decrements.
//! 3. At zero, `MissionSystem` resolves the outcome and emits a `MissionResult`.
//! 4. Repeatable missions (diplomacy below threshold, recruitment with officers
//!    remaining) are re-queued automatically by the caller based on the result.

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::ids::{CharacterKey, SystemKey};
use crate::tick::TickEvent;
use crate::world::{Character, GameWorld, MstbTable};

// ---------------------------------------------------------------------------
// MissionKind
// ---------------------------------------------------------------------------

/// All nine mission types recognised by the REBEXE.EXE dispatch (FUN_0050d5a0).
///
/// Type codes match the original binary's 13-case switch where applicable.
/// Coefficients are ported from rebellion2's Mission.cs; they serve as
/// fallback when `GameWorld::mission_tables` is not yet populated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionKind {
    // ── Living Galaxy (implemented) ─────────────────────────────────────────

    /// Shift a system's popularity toward the sending faction.
    /// DIPLMSTB.DAT. Skill: diplomacy.
    Diplomacy,

    /// Recruit an available character for the sending faction.
    /// RCRTMSTB.DAT. Skill: leadership.
    Recruitment,

    // ── War Machine phase ────────────────────────────────────────────────────

    /// Destroy an enemy facility (type code 6 in REBEXE.EXE).
    /// SBTGMSTB.DAT. Skill: espionage.
    Sabotage,

    /// Kill an enemy character (type code 7 in REBEXE.EXE).
    /// ASSNMSTB.DAT. Skill: combat.
    Assassination,

    /// Gather intelligence on an enemy system.
    /// ESPIMSTB.DAT. Skill: espionage.
    Espionage,

    /// Free a captured character.
    /// RESCMSTB.DAT. Skill: combat.
    Rescue,

    /// Capture an enemy character.
    /// ABDCMSTB.DAT. Skill: espionage.
    Abduction,

    /// Trigger a planetary uprising.
    /// INCTMSTB.DAT. Skill: diplomacy.
    InciteUprising,

    /// Automatic scrapping of obsolete units (type code 21 / 0x15).
    /// No character required; always succeeds.
    Autoscrap,
}

impl MissionKind {
    /// DAT file stem for this kind's *MSTB probability table.
    ///
    /// This is the key used to look up `world.mission_tables`. `None` for
    /// `Autoscrap` (no probability table — it always succeeds).
    pub fn mstb_key(self) -> Option<&'static str> {
        match self {
            MissionKind::Diplomacy     => Some("DIPLMSTB"),
            MissionKind::Recruitment   => Some("RCRTMSTB"),
            MissionKind::Sabotage      => Some("SBTGMSTB"),
            MissionKind::Assassination => Some("ASSNMSTB"),
            MissionKind::Espionage     => Some("ESPIMSTB"),
            MissionKind::Rescue        => Some("RESCMSTB"),
            MissionKind::Abduction     => Some("ABDCMSTB"),
            MissionKind::InciteUprising=> Some("INCTMSTB"),
            MissionKind::Autoscrap     => None,
        }
    }

    /// Quadratic success-probability fallback coefficients: (a, b, c).
    ///
    /// Formula: `prob% = a·score² + b·score + c`, clamped to [min%, max%].
    ///
    /// Diplomacy and Recruitment are from rebellion2 Mission.cs.
    /// Other types use placeholder coefficients fit to approximate MSTB curves.
    /// Once `world.mission_tables` is populated, these are never used.
    pub fn coefficients(self) -> (f64, f64, f64) {
        match self {
            MissionKind::Diplomacy     => ( 0.005558, 0.7656, 20.15),
            MissionKind::Recruitment   => (-0.001748, 0.8657, 11.923),
            MissionKind::Sabotage      => (-0.002,    0.75,   15.0),
            MissionKind::Assassination => (-0.003,    0.80,   10.0),
            MissionKind::Espionage     => (-0.002,    0.78,   12.0),
            MissionKind::Rescue        => (-0.002,    0.72,   10.0),
            MissionKind::Abduction     => (-0.002,    0.70,    8.0),
            MissionKind::InciteUprising=> (-0.003,    0.65,   18.0),
            MissionKind::Autoscrap     => ( 0.0,      0.0,  100.0), // always succeeds
        }
    }

    /// Extract the relevant skill score from a character for this mission type.
    pub fn skill_score(self, character: &Character) -> u32 {
        let pair = match self {
            MissionKind::Diplomacy      => character.diplomacy,
            MissionKind::Recruitment    => character.leadership,
            MissionKind::Sabotage       => character.espionage,
            MissionKind::Assassination  => character.combat,
            MissionKind::Espionage      => character.espionage,
            MissionKind::Rescue         => character.combat,
            MissionKind::Abduction      => character.espionage,
            MissionKind::InciteUprising => character.diplomacy,
            // Autoscrap has no character; callers guard against passing None.
            MissionKind::Autoscrap      => return 100,
        };
        // Expected value: base + half variance (variance resolved at scenario start).
        pair.base + pair.variance / 2
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
    /// Drawn from MISSNSD.DAT base_duration. War Machine types use longer
    /// ranges reflecting the original game's espionage timescales.
    pub fn tick_range(self) -> (u32, u32) {
        match self {
            MissionKind::Diplomacy | MissionKind::Recruitment => (15, 20),
            MissionKind::Sabotage                             => (20, 30),
            MissionKind::Assassination                        => (25, 35),
            MissionKind::Espionage                            => (15, 25),
            MissionKind::Rescue                               => (20, 30),
            MissionKind::Abduction                            => (25, 35),
            MissionKind::InciteUprising                       => (20, 30),
            MissionKind::Autoscrap                            => (1,  1),
        }
    }

    /// Sample a concrete duration from the tick range given a uniform roll in [0,1).
    pub fn sample_duration(self, roll: f64) -> u32 {
        let (min, max) = self.tick_range();
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
    /// The target character (for assassination, abduction, rescue). None for area missions.
    pub target_character: Option<CharacterKey>,
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
            target_character: None,
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

    /// Dispatch with mission-state guard.
    ///
    /// Returns `None` if the character is already on a mission or has a
    /// mandatory mission assignment (prevents double-dispatch).
    pub fn dispatch_guarded(
        &mut self,
        kind: MissionKind,
        faction: MissionFaction,
        character: CharacterKey,
        target_system: SystemKey,
        duration_roll: f64,
        world: &GameWorld,
    ) -> Option<u64> {
        if let Some(c) = world.characters.get(character) {
            if c.on_mission || c.on_mandatory_mission {
                return None;
            }
        }
        Some(self.dispatch(kind, faction, character, target_system, duration_roll))
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
///
/// The caller in `main.rs` is responsible for applying these effects to
/// `GameWorld`. `MissionSystem` is stateless — it only produces the events.
#[derive(Debug, Clone, PartialEq)]
pub enum MissionEffect {
    // ── Living Galaxy ────────────────────────────────────────────────────────

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

    // ── War Machine ──────────────────────────────────────────────────────────

    /// A facility was sabotaged — reduce its remaining production ticks.
    ///
    /// The caller applies `ticks_lost` to the appropriate facility at
    /// `system.manufacturing_facilities[facility_index]` or
    /// `system.defense_facilities[facility_index]`.
    FacilitySabotaged {
        system: SystemKey,
        /// Index into the system's facility list (manufacturing or defense).
        facility_index: usize,
        /// Production ticks lost due to sabotage.
        ticks_lost: u32,
    },

    /// A character was killed — remove from the game.
    CharacterKilled {
        character: CharacterKey,
        faction: MissionFaction,
    },

    /// A character was captured by the opposing faction.
    ///
    /// The captured character is held at `at_system` until rescued or the game ends.
    CharacterCaptured {
        character: CharacterKey,
        /// Which faction now holds the character.
        captured_by: MissionFaction,
        at_system: SystemKey,
    },

    /// A captured character was successfully rescued.
    CharacterRescued {
        character: CharacterKey,
        /// Faction the character was returned to.
        returned_to: MissionFaction,
        at_system: SystemKey,
    },

    /// Intelligence gathered — reveal fog of war over the target system.
    SystemIntelligenceGathered {
        system: SystemKey,
        /// Faction that gained the intelligence.
        faction: MissionFaction,
    },

    /// An uprising was incited — shift popularity against the controlling faction.
    UprisingStarted {
        system: SystemKey,
        /// Popularity delta applied against the controlling faction (positive = shift toward other side).
        popularity_delta: f32,
    },

    // ── Character availability tracking ─────────────────────────────────────

    /// Character has been assigned to a mission.
    CharacterBusy { character: CharacterKey },
    /// Character has completed/been freed from a mission.
    CharacterAvailable { character: CharacterKey },

    // ── Decoy / Escape ──────────────────────────────────────────────────────

    /// A decoy intercepted the mission — no real effect.
    DecoyTriggered {
        system: SystemKey,
        decoy_character: CharacterKey,
    },
    /// A captured character escaped on their own.
    CharacterEscaped {
        character: CharacterKey,
        escaped_to_alliance: bool,
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
                let mut result =
                    Self::resolve_mission(&mission, world, final_tick, roll);
                // Emit CharacterAvailable: the character is freed from this mission.
                result.effects.push(MissionEffect::CharacterAvailable {
                    character: mission.character,
                });
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
            Self::determine_outcome(mission, character, tick, roll, &world.mission_tables);

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
    ///
    /// Uses MSTB table lookup when available in `world.mission_tables`,
    /// falls back to quadratic coefficients otherwise.
    fn determine_outcome(
        mission: &ActiveMission,
        character: Option<&Character>,
        _tick: u64,
        roll: f64,
        mission_tables: &HashMap<String, MstbTable>, // from world.mission_tables
    ) -> (MissionOutcome, Vec<MissionEffect>) {
        // Autoscrap always succeeds — no character or probability check needed.
        if mission.kind == MissionKind::Autoscrap {
            return (MissionOutcome::Success, Self::build_effects(mission));
        }

        let skill_score: u32 = character
            .map(|c| mission.kind.skill_score(c))
            .unwrap_or(0);

        // Priority 1: MSTB table lookup (piecewise-linear over IntTableEntry data).
        // Priority 2: quadratic fallback (rebellion2 Mission.cs coefficients).
        let agent_prob = if let Some(key) = mission.kind.mstb_key() {
            if let Some(table) = mission_tables.get(key) {
                // world::MstbTable::lookup takes a signed i32 skill delta and returns u32 (0-100).
                let raw = table.lookup(skill_score as i32) as f64;
                clamp_prob(raw, mission.kind.min_success_prob(), mission.kind.max_success_prob())
            } else {
                // Table not loaded yet — fall back to quadratic.
                let (a, b, c) = mission.kind.coefficients();
                let raw = quadratic_prob(skill_score as f64, a, b, c);
                clamp_prob(raw, mission.kind.min_success_prob(), mission.kind.max_success_prob())
            }
        } else {
            100.0 // No MSTB key → always succeeds (only Autoscrap, handled above)
        };

        // Foil probability: 0 until counter-espionage defense score is tracked.
        // This is consistent with rebellion2's own-system behavior and is a safe
        // stub until the defense score system is implemented.
        let foil = foil_prob(0.0, true);
        let success_prob = total_success_prob(agent_prob, foil);

        if roll * 100.0 <= success_prob {
            let effects = Self::build_effects(mission);
            (MissionOutcome::Success, effects)
        } else {
            // No foil detection until counter-intelligence is implemented.
            (MissionOutcome::Failure, Vec::new())
        }
    }

    /// Build the world-state effects for a successful mission.
    ///
    /// Conservative defaults for War Machine types: the caller is expected to
    /// apply these effects to `GameWorld` and may override the values based on
    /// additional game state (e.g. facility selection for Sabotage).
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
            MissionKind::Sabotage => {
                // Target facility index 0 as default — callers should select the
                // specific facility based on game state before dispatching.
                vec![MissionEffect::FacilitySabotaged {
                    system: mission.target_system,
                    facility_index: 0,
                    // ~10 ticks of production lost per successful sabotage.
                    ticks_lost: 10,
                }]
            }
            MissionKind::Assassination => {
                if let Some(target) = mission.target_character {
                    vec![MissionEffect::CharacterKilled {
                        character: target,
                        faction: mission.faction,
                    }]
                } else {
                    vec![]  // No target specified — no effect
                }
            }
            MissionKind::Espionage => {
                vec![MissionEffect::SystemIntelligenceGathered {
                    system: mission.target_system,
                    faction: mission.faction,
                }]
            }
            MissionKind::Rescue => {
                if let Some(target) = mission.target_character {
                    vec![MissionEffect::CharacterRescued {
                        character: target,
                        returned_to: mission.faction,
                        at_system: mission.target_system,
                    }]
                } else {
                    vec![]
                }
            }
            MissionKind::Abduction => {
                if let Some(target) = mission.target_character {
                    let captured_by = mission.faction;
                    vec![MissionEffect::CharacterCaptured {
                        character: target,
                        captured_by,
                        at_system: mission.target_system,
                    }]
                } else {
                    vec![]
                }
            }
            MissionKind::InciteUprising => {
                vec![MissionEffect::UprisingStarted {
                    system: mission.target_system,
                    // +0.05 popularity delta: more impactful than diplomacy (+0.01).
                    popularity_delta: 0.05,
                }]
            }
            MissionKind::Autoscrap => {
                // Autoscrap has no world-state effects — the actual unit removal
                // is handled outside the mission system.
                Vec::new()
            }
        }
    }

    /// Check if a decoy intercepts a mission at the target system.
    ///
    /// If a defending-faction character with espionage skill is present at the
    /// target system, look up FDECOYTB to determine decoy probability.
    /// Returns `Some(DecoyTriggered)` if the decoy succeeds, consuming one roll.
    pub fn check_decoy(
        mission: &ActiveMission,
        world: &GameWorld,
        roll: f64,
    ) -> Option<MissionEffect> {
        let table = world.mission_tables.get("FDECOYTB")?;

        // Find a defending character at the target system with espionage skill.
        let defending_alliance = mission.faction == MissionFaction::Empire;
        let defender = world.characters.iter().find(|(_, c)| {
            c.is_alliance == defending_alliance
                && c.current_system == Some(mission.target_system)
                && c.espionage.base > 0
        });

        let (decoy_key, decoy_char) = defender?;
        let prob = table.lookup(decoy_char.espionage.base as i32) as f64 / 100.0;

        if roll < prob {
            Some(MissionEffect::DecoyTriggered {
                system: mission.target_system,
                decoy_character: decoy_key,
            })
        } else {
            None
        }
    }

    /// Check for captured characters escaping on their own.
    ///
    /// For each character held by the opposing faction, look up ESCAPETB
    /// and roll against the escape probability. Returns one `CharacterEscaped`
    /// effect per successful escape.
    ///
    /// Stub for v0.5.0 — the actual per-tick check will be wired by the main loop.
    pub fn check_escapes(
        world: &GameWorld,
        rolls: &[f64],
    ) -> Vec<MissionEffect> {
        let table = match world.mission_tables.get("ESCAPETB") {
            Some(t) => t,
            None => return Vec::new(),
        };

        let effects = Vec::new();
        let _roll_iter = rolls.iter().copied();

        // Stub: full escape-per-tick check will be wired by the main loop in v0.5.0.
        // Iterate characters, check capture state, look up ESCAPETB, roll against probability.
        let _ = (world, table);

        effects
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{CharacterKey, SystemKey};
    use crate::world::{Character, ForceTier, GameWorld, SkillPair};

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
        GameWorld::default()
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
            current_system: None,
            current_fleet: None,
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
        // Only effect should be CharacterAvailable (no mission-specific effects on failure).
        assert_eq!(results[0].effects.len(), 1);
        assert!(matches!(
            &results[0].effects[0],
            MissionEffect::CharacterAvailable { .. }
        ));
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
        // PopularityShifted + CharacterAvailable
        assert!(results[0].effects.len() >= 2);
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
            current_system: None,
            current_fleet: None,
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

    // --- New mission kind tests ---

    #[test]
    fn all_new_kinds_have_tick_ranges_in_bounds() {
        let kinds = [
            MissionKind::Sabotage,
            MissionKind::Assassination,
            MissionKind::Espionage,
            MissionKind::Rescue,
            MissionKind::Abduction,
            MissionKind::InciteUprising,
            MissionKind::Autoscrap,
        ];
        for kind in kinds {
            for roll in [0.0_f64, 0.5, 0.99] {
                let d = kind.sample_duration(roll);
                let (min, max) = kind.tick_range();
                assert!(
                    d >= min && d <= max,
                    "{kind:?}: duration {d} out of [{min}, {max}] at roll {roll}"
                );
            }
        }
    }

    #[test]
    fn autoscrap_always_succeeds() {
        // Autoscrap has no character in world — use a mock key from an empty slotmap.
        let (system, character) = mock_keys();
        let world = minimal_world();
        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::Autoscrap, MissionFaction::Empire, character, system, 1,
        ));
        state.next_id = 1;

        // roll = 1.0 → Autoscrap ignores the roll and always succeeds.
        let results = MissionSystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], &[1.0]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].outcome, MissionOutcome::Success);
    }

    #[test]
    fn sabotage_success_emits_facility_sabotaged() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let zero = skill_pair(0);
        let character = world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Spy".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: zero,
            espionage: skill_pair(80), // high espionage
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
            current_system: None,
            current_fleet: None,
        });

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::Sabotage, MissionFaction::Alliance, character, system, 1,
        ));
        state.next_id = 1;

        let results = MissionSystem::advance(
            &mut state, &world, &[TickEvent { tick: 1 }], &[0.0], // guaranteed success
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success);
        assert!(results[0].effects.iter().any(|e| matches!(e, MissionEffect::FacilitySabotaged { .. })));
    }

    #[test]
    fn incite_uprising_success_emits_uprising_started() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let zero = skill_pair(0);
        let character = world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Agitator".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: skill_pair(70),
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
            current_system: None,
            current_fleet: None,
        });

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::InciteUprising, MissionFaction::Alliance, character, system, 1,
        ));
        state.next_id = 1;

        let results = MissionSystem::advance(
            &mut state, &world, &[TickEvent { tick: 1 }], &[0.0],
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success);
        assert!(results[0].effects.iter().any(|e| matches!(e, MissionEffect::UprisingStarted { .. })));
    }

    #[test]
    fn mstb_table_lookup_used_when_populated() {
        use crate::world::{MstbEntry, MstbTable};

        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let zero = skill_pair(0);
        let character = world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Diplomat".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: skill_pair(50), // score = 50 → threshold delta = 0 → table midpoint
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
            current_system: None,
            current_fleet: None,
        });

        // Install a minimal DIPLMSTB table: two entries, threshold 0 = value 99.
        // With score=50 → delta=0 → table lookup returns 99 → guaranteed success.
        let table = MstbTable::new(vec![
            MstbEntry { threshold: -50, value: 1 },
            MstbEntry { threshold:   0, value: 99 },
            MstbEntry { threshold:  50, value: 100 },
        ]);
        world.mission_tables.insert("DIPLMSTB".to_string(), table);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::Diplomacy, MissionFaction::Alliance, character, system, 1,
        ));
        state.next_id = 1;

        // roll = 0.98 → 98% < 99% success → should succeed via MSTB table
        let results = MissionSystem::advance(
            &mut state, &world, &[TickEvent { tick: 1 }], &[0.98],
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success,
            "MSTB table lookup should yield ~99% success at skill=50");
    }

    #[test]
    fn espionage_success_emits_intelligence_gathered() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let zero = skill_pair(0);
        let character = world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Intel".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: zero,
            espionage: skill_pair(75),
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
            current_system: None,
            current_fleet: None,
        });

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::Espionage, MissionFaction::Alliance, character, system, 1,
        ));
        state.next_id = 1;

        let results = MissionSystem::advance(
            &mut state, &world, &[TickEvent { tick: 1 }], &[0.0],
        );
        assert_eq!(results[0].outcome, MissionOutcome::Success);
        assert!(results[0].effects.iter().any(|e| matches!(
            e, MissionEffect::SystemIntelligenceGathered { .. }
        )));
    }

    #[test]
    fn mstb_key_matches_expected_dat_stems() {
        assert_eq!(MissionKind::Diplomacy.mstb_key(),      Some("DIPLMSTB"));
        assert_eq!(MissionKind::Recruitment.mstb_key(),    Some("RCRTMSTB"));
        assert_eq!(MissionKind::Sabotage.mstb_key(),       Some("SBTGMSTB"));
        assert_eq!(MissionKind::Assassination.mstb_key(),  Some("ASSNMSTB"));
        assert_eq!(MissionKind::Espionage.mstb_key(),      Some("ESPIMSTB"));
        assert_eq!(MissionKind::Rescue.mstb_key(),         Some("RESCMSTB"));
        assert_eq!(MissionKind::Abduction.mstb_key(),      Some("ABDCMSTB"));
        assert_eq!(MissionKind::InciteUprising.mstb_key(), Some("INCTMSTB"));
        assert_eq!(MissionKind::Autoscrap.mstb_key(),      None);
    }

    // --- Character availability tracking tests ---

    #[test]
    fn character_available_emitted_on_mission_completion() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 80);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::new(
            0, MissionKind::Diplomacy, MissionFaction::Alliance, character, system, 1,
        ));
        state.next_id = 1;

        let results = MissionSystem::advance(
            &mut state, &world, &[TickEvent { tick: 1 }], &[0.0],
        );
        assert_eq!(results.len(), 1);
        // The last effect should be CharacterAvailable
        assert!(results[0].effects.iter().any(|e| matches!(
            e, MissionEffect::CharacterAvailable { character: c } if *c == character
        )));
    }

    #[test]
    fn dispatch_guarded_blocks_mandatory_mission() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let zero = skill_pair(0);
        let character = world.characters.insert(Character {
            dat_id: crate::ids::DatId(0),
            name: "Mandatory".into(),
            is_alliance: true,
            is_empire: false,
            is_major: true,
            diplomacy: skill_pair(80),
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
            on_mandatory_mission: true, // <-- mandatory
            current_system: None,
            current_fleet: None,
        });

        let mut state = MissionState::new();
        let result = state.dispatch_guarded(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            0.5,
            &world,
        );
        assert!(result.is_none(), "mandatory mission character should be blocked");
        assert!(state.is_empty());
    }
}
