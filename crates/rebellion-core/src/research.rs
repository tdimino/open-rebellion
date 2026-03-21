//! Research system: tech tree progression driven by character skill assignments.
//!
//! # Design
//!
//! Follows the same stateless advance() pattern as manufacturing and missions:
//! - `ResearchState` holds all active research projects and per-faction tech levels.
//! - `ResearchSystem::advance(state, world, tick_events) -> Vec<ResearchResult>`
//! - Results are pure data; the caller applies them to `ResearchState` and logs messages.
//!
//! # Research Trees
//!
//! Three independent tech trees per faction (from `rebellion2/Faction.cs`):
//! - **Ship** (`TechType::Ship`) — driven by `ship_design` skill
//! - **Troop** (`TechType::Troop`) — driven by `troop_training` skill
//! - **Facility** (`TechType::Facility`) — driven by `facility_design` skill
//!
//! Each tree has an integer level. When a character completes a research project,
//! the corresponding tree level advances by 1. Classes with `research_order <= level`
//! become buildable by the manufacturing system.
//!
//! # Tick Cost
//!
//! The base cost in ticks for advancing from level N to N+1 is the maximum
//! `research_difficulty` among all classes at `research_order == N + 1`.
//! Minimum cost is `RESEARCH_MIN_TICKS`. If no classes are at that order, cost
//! falls back to `RESEARCH_DEFAULT_TICKS`.
//!
//! # Dispatch
//!
//! The caller creates a `ResearchProject` and inserts it via `ResearchState::dispatch`.
//! Only one project per (faction × TechType) is active at a time — a new dispatch
//! replaces the previous one.

use serde::{Deserialize, Serialize};

use crate::ids::{CharacterKey, FighterKey};
use crate::tick::TickEvent;
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Minimum ticks for any research project (prevents instantaneous unlocks).
pub const RESEARCH_MIN_TICKS: u32 = 10;

/// Fallback ticks when no classes exist at the next research order.
pub const RESEARCH_DEFAULT_TICKS: u32 = 30;

/// Maximum research level per tree (matches REBEXE.EXE's tech tier range).
pub const RESEARCH_MAX_LEVEL: u32 = 15;

// ---------------------------------------------------------------------------
// TechType
// ---------------------------------------------------------------------------

/// The three independently-progressing tech trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechType {
    /// Capital ship research — driven by `ship_design` skill.
    Ship,
    /// Troop regiment research — driven by `troop_training` skill.
    Troop,
    /// Facility / construction research — driven by `facility_design` skill.
    Facility,
}

// ---------------------------------------------------------------------------
// ResearchProject
// ---------------------------------------------------------------------------

/// An active research project assigned to a character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProject {
    /// Which tech tree is being advanced.
    pub tech_type: TechType,
    /// The character doing the research.
    pub character: CharacterKey,
    /// True = Rebel Alliance project; false = Empire.
    pub faction_is_alliance: bool,
    /// Ticks remaining until this level completes.
    pub ticks_remaining: u32,
    /// Original duration (for progress display).
    pub total_ticks: u32,
}

impl ResearchProject {
    /// Progress fraction [0.0, 1.0].
    pub fn progress(&self) -> f32 {
        if self.total_ticks == 0 {
            return 1.0;
        }
        1.0 - (self.ticks_remaining as f32 / self.total_ticks as f32)
    }
}

// ---------------------------------------------------------------------------
// ResearchLevels
// ---------------------------------------------------------------------------

/// Per-faction research levels across all three trees.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResearchLevels {
    pub ship: u32,
    pub troop: u32,
    pub facility: u32,
}

impl ResearchLevels {
    /// Return the current level for a specific tech tree.
    pub fn level(&self, tech: TechType) -> u32 {
        match tech {
            TechType::Ship     => self.ship,
            TechType::Troop    => self.troop,
            TechType::Facility => self.facility,
        }
    }

    /// Increment the level for a specific tech tree (capped at `RESEARCH_MAX_LEVEL`).
    pub fn advance(&mut self, tech: TechType) {
        let v = match tech {
            TechType::Ship     => &mut self.ship,
            TechType::Troop    => &mut self.troop,
            TechType::Facility => &mut self.facility,
        };
        *v = (*v + 1).min(RESEARCH_MAX_LEVEL);
    }
}

// ---------------------------------------------------------------------------
// ResearchState
// ---------------------------------------------------------------------------

/// Persistent research state for both factions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResearchState {
    /// Tech levels for the Alliance.
    pub alliance: ResearchLevels,
    /// Tech levels for the Empire.
    pub empire: ResearchLevels,
    /// Active research projects (at most 6: 2 factions × 3 trees).
    pub projects: Vec<ResearchProject>,
}

impl ResearchState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Dispatch a research project. Replaces any existing project for the same
    /// (faction × tech_type) combination.
    pub fn dispatch(&mut self, project: ResearchProject) {
        self.projects.retain(|p| {
            !(p.faction_is_alliance == project.faction_is_alliance
                && p.tech_type == project.tech_type)
        });
        self.projects.push(project);
    }

    /// Cancel a project for a given faction + tech type.
    pub fn cancel(&mut self, faction_is_alliance: bool, tech_type: TechType) {
        self.projects
            .retain(|p| !(p.faction_is_alliance == faction_is_alliance && p.tech_type == tech_type));
    }

    /// Return the current research level for a faction + tech tree.
    pub fn level(&self, faction_is_alliance: bool, tech: TechType) -> u32 {
        if faction_is_alliance {
            self.alliance.level(tech)
        } else {
            self.empire.level(tech)
        }
    }

    /// True if `class_research_order <= current level` — i.e. the class is
    /// buildable by this faction.
    pub fn is_unlocked(&self, faction_is_alliance: bool, class_research_order: u32, tech: TechType) -> bool {
        self.level(faction_is_alliance, tech) >= class_research_order
    }
}

// ---------------------------------------------------------------------------
// ResearchResult
// ---------------------------------------------------------------------------

/// A completed research project — emitted by `ResearchSystem::advance`.
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchResult {
    /// A tech tree advanced one level.
    TechUnlocked {
        faction_is_alliance: bool,
        tech_type: TechType,
        /// The new level (old level + 1).
        new_level: u32,
    },
}

// ---------------------------------------------------------------------------
// ResearchSystem
// ---------------------------------------------------------------------------

/// Stateless research tick processor.
pub struct ResearchSystem;

impl ResearchSystem {
    /// Advance all active research projects by one tick per `TickEvent`.
    ///
    /// Returns a `ResearchResult::TechUnlocked` for each project that completes.
    ///
    /// **IMPORTANT**: As of v0.6.0, this function no longer applies level-ups internally.
    /// The caller must apply each TechUnlocked result:
    /// ```ignore
    /// for result in &results {
    ///     let ResearchResult::TechUnlocked { faction_is_alliance, tech_type, .. } = result;
    ///     if *faction_is_alliance { state.alliance.advance(*tech_type); }
    ///     else { state.empire.advance(*tech_type); }
    /// }
    /// ```
    pub fn advance(
        state: &mut ResearchState,
        _world: &GameWorld,
        tick_events: &[TickEvent],
    ) -> Vec<ResearchResult> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let ticks_elapsed = tick_events.len() as u32;
        let mut results = Vec::new();

        for project in &mut state.projects {
            if project.ticks_remaining == 0 {
                continue;
            }
            project.ticks_remaining = project.ticks_remaining.saturating_sub(ticks_elapsed);

            if project.ticks_remaining == 0 {
                // Determine new level and emit result.
                let current_level = if project.faction_is_alliance {
                    state.alliance.level(project.tech_type)
                } else {
                    state.empire.level(project.tech_type)
                };

                let new_level = (current_level + 1).min(RESEARCH_MAX_LEVEL);

                results.push(ResearchResult::TechUnlocked {
                    faction_is_alliance: project.faction_is_alliance,
                    tech_type: project.tech_type,
                    new_level,
                });
            }
        }

        // Remove completed projects.
        state.projects.retain(|p| p.ticks_remaining > 0);

        results
    }

    /// Compute the tick cost to advance a tech tree from its current level to
    /// the next level.
    ///
    /// Uses the maximum `research_difficulty` among all classes at
    /// `research_order == current_level + 1`. Falls back to `RESEARCH_DEFAULT_TICKS`
    /// if no classes exist at that order.
    ///
    /// Currently only `TechType::Ship` has DAT-backed difficulty values
    /// (from `CapitalShipClass::research_difficulty`). Troop and Facility trees
    /// fall back to `RESEARCH_DEFAULT_TICKS` until those class types expose
    /// `research_difficulty` in the world model.
    pub fn ticks_for_next_level(
        world: &GameWorld,
        faction_is_alliance: bool,
        tech_type: TechType,
        current_level: u32,
    ) -> u32 {
        let target_order = current_level + 1;

        let max_difficulty = match tech_type {
            TechType::Ship => world
                .capital_ship_classes
                .values()
                .filter(|c| {
                    c.research_order == target_order
                        && if faction_is_alliance { c.is_alliance } else { c.is_empire }
                })
                .map(|c| c.research_difficulty)
                .max(),
            TechType::Troop | TechType::Facility => None,
        };

        max_difficulty
            .unwrap_or(RESEARCH_DEFAULT_TICKS)
            .max(RESEARCH_MIN_TICKS)
    }

    /// True if a capital ship class is available for this faction at the given
    /// research level.
    pub fn ship_class_is_available(
        world: &GameWorld,
        state: &ResearchState,
        faction_is_alliance: bool,
        class_key: crate::ids::CapitalShipKey,
    ) -> bool {
        let Some(class) = world.capital_ship_classes.get(class_key) else {
            return false;
        };
        let faction_ok = if faction_is_alliance { class.is_alliance } else { class.is_empire };
        if !faction_ok {
            return false;
        }
        state.is_unlocked(faction_is_alliance, class.research_order, TechType::Ship)
    }

    /// True if a fighter class is available at the current research level.
    ///
    /// Fighter classes use `research_order` the same way as capital ships.
    pub fn fighter_class_is_available(
        world: &GameWorld,
        state: &ResearchState,
        faction_is_alliance: bool,
        class_key: FighterKey,
    ) -> bool {
        let Some(class) = world.fighter_classes.get(class_key) else {
            return false;
        };
        let faction_ok = if faction_is_alliance { class.is_alliance } else { class.is_empire };
        if !faction_ok {
            return false;
        }
        // Fighter classes carry research_order implicitly via refined_material_cost ordering.
        // Until FighterClass gains a research_order field, treat all fighters as available
        // at any level (consistent with original game where fighters unlock with ships).
        let _ = state;
        true
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::ControlKind;
    use crate::ids::{CharacterKey, DatId};
    use crate::world::{CapitalShipClass, Character, GameWorld, SkillPair};
    use crate::tick::TickEvent;

    fn ticks(n: u32) -> Vec<TickEvent> {
        (1..=n as u64).map(|t| TickEvent { tick: t }).collect()
    }

    fn make_char_key(world: &mut GameWorld) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId(0),
            name: "Researcher".into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: SkillPair { base: 0, variance: 0 },
            espionage: SkillPair { base: 0, variance: 0 },
            ship_design: SkillPair { base: 60, variance: 0 },
            troop_training: SkillPair { base: 50, variance: 0 },
            facility_design: SkillPair { base: 50, variance: 0 },
            combat: SkillPair { base: 0, variance: 0 },
            leadership: SkillPair { base: 30, variance: 0 },
            loyalty: SkillPair { base: 70, variance: 0 },
            jedi_probability: 0,
            jedi_level: SkillPair { base: 0, variance: 0 },
            can_be_admiral: false,
            can_be_commander: true,
            can_be_general: false,
            force_tier: crate::world::ForceTier::None,
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

    fn add_ship_class(world: &mut GameWorld, research_order: u32, research_difficulty: u32, is_alliance: bool) {
        world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId(0),
            name: "TestShip".into(),
            is_alliance,
            is_empire: !is_alliance,
            refined_material_cost: 100,
            maintenance_cost: 10,
            research_order,
            research_difficulty,
            hull: 100,
            shield_strength: 50,
            sub_light_engine: 5,
            maneuverability: 5,
            hyperdrive: 2,
            fighter_capacity: 0,
            troop_capacity: 0,
            detection: 3,
            turbolaser_fore: 2,
            turbolaser_aft: 1,
            turbolaser_port: 1,
            turbolaser_starboard: 1,
            ion_cannon_fore: 0,
            ion_cannon_aft: 0,
            ion_cannon_port: 0,
            ion_cannon_starboard: 0,
            laser_cannon_fore: 0,
            laser_cannon_aft: 0,
            laser_cannon_port: 0,
            laser_cannon_starboard: 0,
            shield_recharge_rate: 5,
            damage_control: 3,
            bombardment_modifier: 10,
        });
    }

    #[test]
    fn no_ticks_returns_empty() {
        let world = GameWorld::default();
        let mut state = ResearchState::new();
        let results = ResearchSystem::advance(&mut state, &world, &[]);
        assert!(results.is_empty());
    }

    #[test]
    fn project_completes_after_ticks() {
        let mut world = GameWorld::default();
        let mut state = ResearchState::new();
        let char_key = make_char_key(&mut world);

        state.dispatch(ResearchProject {
            tech_type: TechType::Ship,
            character: char_key,
            faction_is_alliance: true,
            ticks_remaining: 20,
            total_ticks: 20,
        });

        // 10 ticks — not done yet.
        let r = ResearchSystem::advance(&mut state, &world, &ticks(10));
        assert!(r.is_empty(), "should not complete after 10 of 20 ticks");
        assert_eq!(state.projects[0].ticks_remaining, 10);

        // 10 more — completes.
        let r = ResearchSystem::advance(&mut state, &world, &ticks(10));
        assert_eq!(r.len(), 1);
        assert_eq!(
            r[0],
            ResearchResult::TechUnlocked {
                faction_is_alliance: true,
                tech_type: TechType::Ship,
                new_level: 1,
            }
        );

        // advance() no longer auto-applies level-ups — caller must apply.
        assert_eq!(state.alliance.ship, 0, "advance() should NOT mutate state");
        // Caller applies:
        for result in &r {
            let ResearchResult::TechUnlocked { faction_is_alliance, tech_type, .. } = result;
            if *faction_is_alliance { state.alliance.advance(*tech_type); }
            else { state.empire.advance(*tech_type); }
        }
        assert_eq!(state.alliance.ship, 1);
        // Project removed.
        assert!(state.projects.is_empty());
    }

    #[test]
    fn dispatch_replaces_existing_project() {
        let mut world = GameWorld::default();
        let mut state = ResearchState::new();
        let char_key = make_char_key(&mut world);

        state.dispatch(ResearchProject {
            tech_type: TechType::Ship,
            character: char_key,
            faction_is_alliance: false,
            ticks_remaining: 50,
            total_ticks: 50,
        });
        assert_eq!(state.projects.len(), 1);

        // Replace with a faster project.
        state.dispatch(ResearchProject {
            tech_type: TechType::Ship,
            character: char_key,
            faction_is_alliance: false,
            ticks_remaining: 5,
            total_ticks: 5,
        });
        assert_eq!(state.projects.len(), 1, "should replace, not append");
        assert_eq!(state.projects[0].ticks_remaining, 5);
    }

    #[test]
    fn cancel_removes_project() {
        let mut world = GameWorld::default();
        let mut state = ResearchState::new();
        let char_key = make_char_key(&mut world);

        state.dispatch(ResearchProject {
            tech_type: TechType::Troop,
            character: char_key,
            faction_is_alliance: true,
            ticks_remaining: 30,
            total_ticks: 30,
        });
        assert_eq!(state.projects.len(), 1);

        state.cancel(true, TechType::Troop);
        assert!(state.projects.is_empty());
    }

    #[test]
    fn ticks_for_next_level_uses_max_difficulty() {
        let mut world = GameWorld::default();

        // Two alliance ships at research_order = 1 with different difficulties.
        add_ship_class(&mut world, 1, 20, true);
        add_ship_class(&mut world, 1, 40, true);

        let cost = ResearchSystem::ticks_for_next_level(&world, true, TechType::Ship, 0);
        assert_eq!(cost, 40, "should use the highest research_difficulty");
    }

    #[test]
    fn ticks_for_next_level_fallback_when_no_classes() {
        let world = GameWorld::default();
        let cost = ResearchSystem::ticks_for_next_level(&world, true, TechType::Ship, 5);
        assert_eq!(cost, RESEARCH_DEFAULT_TICKS);
    }

    #[test]
    fn is_unlocked_respects_current_level() {
        let world = GameWorld::default();
        let mut state = ResearchState::new();

        // Level 0 — order-0 classes are available (pre-game units), order-1 are not.
        assert!(state.is_unlocked(true, 0, TechType::Ship), "order 0 always unlocked");
        assert!(!state.is_unlocked(true, 1, TechType::Ship), "order 1 needs level ≥ 1");

        state.alliance.advance(TechType::Ship);
        assert!(state.is_unlocked(true, 1, TechType::Ship), "order 1 unlocked after advance");
        assert!(!state.is_unlocked(true, 2, TechType::Ship), "order 2 still locked");
    }

    #[test]
    fn two_factions_advance_independently() {
        let mut world = GameWorld::default();
        let mut state = ResearchState::new();
        let char_key = make_char_key(&mut world);

        state.dispatch(ResearchProject {
            tech_type: TechType::Facility,
            character: char_key,
            faction_is_alliance: true,
            ticks_remaining: 5,
            total_ticks: 5,
        });
        state.dispatch(ResearchProject {
            tech_type: TechType::Facility,
            character: char_key,
            faction_is_alliance: false,
            ticks_remaining: 10,
            total_ticks: 10,
        });

        let r = ResearchSystem::advance(&mut state, &world, &ticks(5));
        // Only alliance completes.
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], ResearchResult::TechUnlocked {
            faction_is_alliance: true,
            tech_type: TechType::Facility,
            new_level: 1,
        });
        // Caller applies results.
        for result in &r {
            let ResearchResult::TechUnlocked { faction_is_alliance, tech_type, .. } = result;
            if *faction_is_alliance { state.alliance.advance(*tech_type); }
            else { state.empire.advance(*tech_type); }
        }
        assert_eq!(state.alliance.facility, 1);
        assert_eq!(state.empire.facility, 0);

        // Empire completes after 5 more ticks.
        let r2 = ResearchSystem::advance(&mut state, &world, &ticks(5));
        assert_eq!(r2.len(), 1);
        for result in &r2 {
            let ResearchResult::TechUnlocked { faction_is_alliance, tech_type, .. } = result;
            if *faction_is_alliance { state.alliance.advance(*tech_type); }
            else { state.empire.advance(*tech_type); }
        }
        assert_eq!(state.empire.facility, 1);
    }

    #[test]
    fn research_level_capped_at_max() {
        let mut levels = ResearchLevels::default();
        for _ in 0..=RESEARCH_MAX_LEVEL + 5 {
            levels.advance(TechType::Ship);
        }
        assert_eq!(levels.ship, RESEARCH_MAX_LEVEL);
    }

    #[test]
    fn ship_class_availability_respects_research_order() {
        let mut world = GameWorld::default();
        add_ship_class(&mut world, 3, 20, true); // needs level 3

        let class_key = world.capital_ship_classes.keys().next().unwrap();
        let mut state = ResearchState::new(); // level 0

        assert!(!ResearchSystem::ship_class_is_available(&world, &state, true, class_key));

        state.alliance.ship = 3;
        assert!(ResearchSystem::ship_class_is_available(&world, &state, true, class_key));
    }

    #[test]
    fn advance_does_not_mutate_ship_level() {
        let mut world = GameWorld::default();
        let mut state = ResearchState::new();
        let char_key = make_char_key(&mut world);

        state.dispatch(ResearchProject {
            tech_type: TechType::Ship,
            character: char_key,
            faction_is_alliance: true,
            ticks_remaining: 5,
            total_ticks: 5,
        });

        let r = ResearchSystem::advance(&mut state, &world, &ticks(5));
        assert_eq!(r.len(), 1);
        // advance() must NOT auto-apply level-ups.
        assert_eq!(state.alliance.ship, 0, "advance() should not mutate state internally");
    }

    #[test]
    fn advance_returns_correct_new_level() {
        let mut world = GameWorld::default();
        let mut state = ResearchState::new();
        let char_key = make_char_key(&mut world);

        // Pre-advance alliance ship to level 3.
        state.alliance.ship = 3;

        state.dispatch(ResearchProject {
            tech_type: TechType::Ship,
            character: char_key,
            faction_is_alliance: true,
            ticks_remaining: 5,
            total_ticks: 5,
        });

        let r = ResearchSystem::advance(&mut state, &world, &ticks(5));
        assert_eq!(r.len(), 1);
        assert_eq!(
            r[0],
            ResearchResult::TechUnlocked {
                faction_is_alliance: true,
                tech_type: TechType::Ship,
                new_level: 4,
            }
        );
    }
}
