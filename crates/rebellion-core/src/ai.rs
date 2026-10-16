//! The computer player of either side.
//!
//! `AISystem::round` (`crate::ai_scheduler`) runs one side's round and
//! returns its order as an `AIAction` recommendation; the caller applies it
//! to `ManufacturingState`, `MissionState` and `GameWorld`.
//!
//! The original builds the same fourteen modules for each computer player
//! (`FUN_00485a50`) and steps one armed module per main-loop pass, the sides
//! taking turns (`FUN_0041cf20` -> `FUN_004361a0`, `FUN_00485b40`;
//! `ghidra/notes/ai-order-emission.md`, "Scheduler and executor"): module
//! 13 and its controller (`crate::ai_strategy`), module 8's regional
//! records (`crate::ai_regions`), module 9's fleet tasks
//! (`crate::fleet_tasks`), module 12's character tasks
//! (`crate::character_tasks`), the production modules
//! (`crate::ai_production`) and module 4's mission planners
//! (`crate::ai_research`, `crate::mission_planning`).

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::ai_production::{Round, SystemRecord};
use crate::ids::{CharacterKey, FleetKey, ManufacturingFacilityKey, SystemKey, TroopKey};
use crate::manufacturing::BuildableKind;
use crate::mission_planning::Planner;
use crate::missions::{MissionFaction, MissionKind, MissionState};
use crate::world::{Character, GameWorld};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

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
    #[must_use]
    pub fn as_mission_faction(self) -> MissionFaction {
        match self {
            AiFaction::Alliance => MissionFaction::Alliance,
            AiFaction::Empire => MissionFaction::Empire,
        }
    }

    /// Returns true if the given character belongs to this faction.
    #[must_use]
    pub fn owns_character(self, c: &Character) -> bool {
        match self {
            AiFaction::Alliance => c.is_alliance,
            AiFaction::Empire => c.is_empire,
        }
    }

    /// Returns true if a system favors this faction (above neutral popularity).
    #[must_use]
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

/// Persistent AI state for one side: busy characters and each module's saved
/// state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIState {
    /// Which faction this AI controls.
    pub faction: Option<AiFaction>,
    /// Characters currently dispatched on missions (not available for re-dispatch).
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_set",
        deserialize_with = "crate::serde_ordered::deserialize_hash_set"
    )]
    pub busy_characters: HashSet<CharacterKey>,
    /// The class G character tasks (module 12, `crate::character_tasks`).
    #[serde(default)]
    pub character_tasks: crate::character_tasks::CharacterTasks,
    /// The Research planners (`crate::ai_research`).
    #[serde(default)]
    pub research_planners: crate::ai_research::ResearchPlanners,
    /// The production modules (`crate::ai_production`).
    #[serde(default)]
    pub production: crate::ai_production::ProductionState,
    /// The marks the strategic modules keep (`crate::ai_records`).
    #[serde(default)]
    pub marks: crate::ai_records::Marks,
    /// Module 13 and its controller (`crate::ai_strategy`).
    #[serde(default)]
    pub strategy: crate::ai_strategy::StrategyState,
    /// Module 8's regional records (`crate::ai_regions`).
    #[serde(default)]
    pub regions: crate::ai_regions::Regions,
    /// Module 9's fleet tasks (`crate::fleet_tasks`).
    #[serde(default)]
    pub fleet_tasks: crate::fleet_tasks::FleetTasks,
    /// The round scheduler (`crate::ai_scheduler`).
    #[serde(default)]
    pub scheduler: crate::ai_scheduler::Scheduler,
}

impl AIState {
    #[must_use]
    pub fn new(faction: AiFaction) -> Self {
        AIState {
            faction: Some(faction),
            busy_characters: HashSet::new(),
            character_tasks: crate::character_tasks::CharacterTasks::default(),
            research_planners: crate::ai_research::ResearchPlanners::default(),
            production: crate::ai_production::ProductionState::default(),
            marks: crate::ai_records::Marks::default(),
            strategy: crate::ai_strategy::StrategyState::default(),
            regions: crate::ai_regions::Regions::default(),
            fleet_tasks: crate::fleet_tasks::FleetTasks::default(),
            scheduler: crate::ai_scheduler::Scheduler::default(),
        }
    }

    /// The game type the strategic modules read (`c+4 & 1`, set from the
    /// settings when the context is built, `FUN_004182a0`).
    pub fn set_headquarters_only(&mut self, on: bool) {
        let bit = crate::ai_strategy::ctx::HEADQUARTERS_ONLY;
        if on {
            self.strategy.flags |= bit;
        } else {
            self.strategy.flags &= !bit;
        }
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
    #[must_use]
    pub fn is_busy(&self, character: CharacterKey) -> bool {
        self.busy_characters.contains(&character)
    }

    /// Release every character a resolved mission frees. Special forces
    /// are not in the busy set.
    pub fn free_mission_members(&mut self, effects: &[crate::missions::MissionEffect]) {
        for effect in effects {
            if let crate::missions::MissionEffect::MemberAvailable { member } = effect {
                if let Some(character) = member.character() {
                    self.mark_available(character);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AIAction
// ---------------------------------------------------------------------------

/// A recommended action for the caller to apply to the game state.
///
/// Actions are pure recommendations — the AI system never mutates state
/// directly. The caller decides whether to apply each action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIAction {
    /// Dispatch a mission: the order `0x240` a planner emits
    /// (`FUN_0047b360`), with its team (`+0x2c`) and decoys (`+0x58`).
    DispatchMission {
        kind: MissionKind,
        team: Vec<crate::missions::MissionMember>,
        decoys: Vec<crate::missions::MissionMember>,
        target_system: SystemKey,
        /// Target character for character-targeted missions (Assassination, Abduction, Recruitment).
        target_character: Option<CharacterKey>,
        /// The object a Sabotage or DS Sabotage mission names.
        target_object: Option<crate::missions::MissionTarget>,
    },

    /// A production module's Build `0x210`/`0x211`/`0x212`: `count` units
    /// at `system`'s yards, after a Destination `0x214` when `destination`
    /// is set (`crate::ai_production`).
    EnqueueProduction {
        system: SystemKey,
        kind: BuildableKind,
        /// Units of work per unit: the class's refined-material cost, which
        /// the yards' cycles draw one unit at a time (`crate::stockpiles`).
        work: u32,
        count: u32,
        destination: Option<crate::manufacturing::Destination>,
    },

    /// A production module's Scrap `0x200`; a Retire `0x242` of a special
    /// force arrives as its Scrap (`crate::ai_production`, hyp).
    Scrap {
        targets: Vec<crate::scrap::ScrapTarget>,
    },

    /// Move a fleet to a target system (attack or reinforce).
    MoveFleet {
        fleet: FleetKey,
        to_system: SystemKey,
        reason: FleetMoveReason,
        /// Surface regiments to embark before departure. Empty for ordinary
        /// fleet movement. Capacity and co-location are revalidated when the
        /// action is applied.
        troops: Vec<TroopKey>,
    },

    /// Create Fleet `0x270` of a regional record (`ai_regions`): the
    /// tagged ships of one fleet form a new fleet in `to_system`, travelling
    /// there first from another system; in their own system without one.
    CreateFleet {
        fleet: FleetKey,
        ships: Vec<u32>,
        to_system: Option<SystemKey>,
    },

    /// A Move `0x201` onto another fleet of the side: a regional record's
    /// regroup moves a whole fleet (`FUN_004b0c90`, `ships` empty), a fleet
    /// task the ships of `fleet` with these tags (`FUN_00475490`).
    JoinFleet {
        fleet: FleetKey,
        ships: Vec<u32>,
        target: FleetKey,
    },

    /// A Move `0x201` of regiments onto a fleet or to a system: a regional
    /// record's loading, unloading or drop (`crate::ai_regions`).
    MoveRegiments {
        troops: Vec<TroopKey>,
        to: crate::troop_transport::RegimentTarget,
    },

    /// A Move `0x201` of a system's squadrons onto a fleet there
    /// (`FUN_004b0120`).
    MoveSquadrons {
        squadrons: Vec<crate::troop_transport::SquadronSource>,
        fleet: FleetKey,
    },

    /// A Move `0x201` of characters off a fleet into its system
    /// (`FUN_004b06b0`).
    MoveCharacters {
        characters: Vec<CharacterKey>,
        to_system: SystemKey,
    },

    /// Planetary Assault `0x234` (`FUN_004b1070`). port: the fleet's held
    /// cargo is released to the landing (`crate::troop_transport`).
    Assault { fleet: FleetKey },

    /// General Bombardment `0x222` or Target Military Facilities `0x220`
    /// (`FUN_004b1070`).
    Bombard {
        fleet: FleetKey,
        system: SystemKey,
        kind: crate::ai_regions::Strike,
    },

    /// Destroy System `0x223` by the fleet's Death Star (`FUN_004b1070`).
    DestroySystem { fleet: FleetKey, system: SystemKey },

    /// Move a character to a system: the Move `0x201` of a character task
    /// (`FUN_004bc040`).
    MoveCharacter {
        character: CharacterKey,
        to_system: SystemKey,
    },

    /// Move a character onto a fleet: a class I task's Move `0x201` onto
    /// its record's flagship (`FUN_004bca20`), whose fleet the port boards.
    MoveCharacterToFleet {
        character: CharacterKey,
        fleet: FleetKey,
    },

    /// Give or take an officer rank: the Command order `0x260`..`0x263` of
    /// a character task (`FUN_004bbf50`; `None` on release, `FUN_004bbba0`).
    GiveCommand {
        character: CharacterKey,
        kind: crate::officer_rank::CommandKind,
    },
}

impl From<crate::ai_production::Order> for AIAction {
    fn from(order: crate::ai_production::Order) -> Self {
        use crate::ai_production::Order;
        match order {
            Order::Build {
                producer,
                kind,
                work,
                count,
                destination,
            } => Self::EnqueueProduction {
                system: producer,
                kind,
                work,
                count,
                destination,
            },
            Order::Scrap(targets) => Self::Scrap { targets },
            Order::MoveShips { fleet, ships, to } => Self::CreateFleet {
                fleet,
                ships,
                to_system: Some(to),
            },
            Order::Retire(force) => Self::Scrap {
                targets: vec![crate::scrap::ScrapTarget::SpecialForce(force)],
            },
        }
    }
}

/// Why the AI is moving a fleet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// A system's name for a trace record.
fn system_label(world: &GameWorld, system: SystemKey) -> Option<String> {
    world.systems.get(system).map(|sys| sys.name.clone())
}

impl AISystem {
    // -----------------------------------------------------------------------
    // Officer heuristics
    // -----------------------------------------------------------------------

    /// Trace a candidate list the evaluators scored, best first as they
    /// walk it. The scores are each evaluator's own sort key.
    fn trace_scored(
        world: &GameWorld,
        kind: &'static str,
        scored: impl FnOnce() -> Vec<(SystemKey, f64)>,
    ) {
        crate::trace::record(crate::trace::TraceMask::AI, kind, || {
            let candidates: Vec<_> = scored()
                .into_iter()
                .map(|(system, score)| {
                    serde_json::json!({ "system": system_label(world, system), "score": score })
                })
                .collect();
            serde_json::json!({ "candidates": candidates })
        });
    }

    /// Plan `kind`'s team and decoys (`crate::mission_planning`) and push
    /// the order when the planner sends anyone, unless `actions` already
    /// holds `stop` orders. Returns whether it did.
    fn dispatch_planned(
        planner: &mut Planner<'_>,
        stop: usize,
        kind: MissionKind,
        target: (SystemKey, Option<CharacterKey>),
        target_object: Option<crate::missions::MissionTarget>,
        actions: &mut Vec<AIAction>,
    ) -> bool {
        if actions.len() >= stop {
            return false;
        }
        let planned = planner.plan(kind);
        crate::trace::record(crate::trace::TraceMask::AI, "dispatch", || {
            let world = planner.world;
            serde_json::json!({
                "kind": format!("{kind:?}"),
                "target_system": system_label(world, target.0),
                "target_character": target
                    .1
                    .and_then(|key| world.characters.get(key))
                    .map(|c| c.name.clone()),
                "chosen": planned.is_some(),
                "team": planned.as_ref().map(|(team, _)| team.len()),
                "decoys": planned.as_ref().map(|(_, decoys)| decoys.len()),
            })
        });
        let Some((team, decoys)) = planned else {
            return false;
        };
        actions.push(AIAction::DispatchMission {
            kind,
            team,
            decoys,
            target_system: target.0,
            target_character: target.1,
            target_object,
        });
        true
    }

    /// The round's record of each system, in the world's order.
    fn system_records<'r>(round: &'r Round<'_>) -> impl Iterator<Item = &'r SystemRecord> + 'r {
        round
            .inputs
            .world
            .systems
            .keys()
            .filter_map(|key| round.records.get(&key))
    }

    /// Whether the class validator at creation (`FUN_00542b60` ->
    /// `FUN_0054c590`) admits `kind` against this target: the backstop
    /// behind the queries' record bits, so no pick is refused at the order.
    fn admits(
        round: &Round<'_>,
        faction: AiFaction,
        kind: MissionKind,
        system: SystemKey,
        character: Option<CharacterKey>,
        object: Option<crate::missions::MissionTarget>,
    ) -> bool {
        let mut mission = crate::missions::ActiveMission::new(
            0,
            kind,
            faction.as_mission_faction(),
            Vec::new(),
            system,
            0,
        );
        mission.target_character = character;
        mission.target_object = object;
        crate::missions::validate(
            &mission,
            round.inputs.world,
            round.inputs.uprisings,
            crate::missions::Check::Creation,
        )
        .is_none()
    }

    /// Whether the side's Diplomacy already works `system`: the Diplomacy
    /// planner's claim on its target (`+0x2c` bit `0x20000000`,
    /// `FUN_00481720.c:76, 287`), which its query forbids.
    fn diplomacy_claimed(missions: &MissionState, faction: AiFaction, system: SystemKey) -> bool {
        missions.missions().iter().any(|m| {
            m.kind == MissionKind::Diplomacy
                && m.faction == faction.as_mission_faction()
                && m.target_system == system
        })
    }

    /// Incite, Diplomacy, and Recruitment orders.
    ///
    /// The targets are those the original's system queries take over the
    /// round's records (`FUN_00433e40`, `sys+0x2c`) and the creation
    /// validator admits ([`Self::admits`]). port: the side's choice among
    /// them (side `+0x318`, `+0x31c`) is untraced, so the port ranks them;
    /// the original planners pick the members.
    /// - Incite (`FUN_00480370.c:109`: bit `0x100`, an enemy system where
    ///   our support is above 50): our most popular first.
    /// - Diplomacy (`FUN_00481720.c:97-268`: bit `0x40`, our systems below
    ///   support 70 and neutral ones, no foreign fleet, no uprising): ours
    ///   first, then neutral, least popular first, while the planner finds a
    ///   diplomat; a system our Diplomacy works is skipped.
    /// - Recruitment (`FUN_0047dcc0.c:62, 119, 139-166`: bit 2): our most
    ///   popular calm, unblockaded system, while its recruit pool holds
    ///   anyone and the planner finds a recruiter.
    ///
    /// port: the three are separate planners in the original, each with its
    /// own module-4 turn; the port's one pseudo-planner tries Incite, then
    /// Diplomacy, then Recruitment, so with one dispatch a turn an Incite
    /// target with a team keeps the other two waiting.
    pub(crate) fn evaluate_officers(
        planner: &mut Planner<'_>,
        round: &Round<'_>,
        missions: &MissionState,
        faction: AiFaction,
        limit: usize,
        actions: &mut Vec<AIAction>,
    ) {
        let world = round.inputs.world;
        let stop = actions.len().saturating_add(limit);
        if let Some(target) = Self::find_incite_target(round, faction) {
            Self::trace_scored(world, "incite_candidate", || {
                vec![(
                    target,
                    f64::from(faction.system_popularity(&world.systems[target])),
                )]
            });
            Self::dispatch_planned(
                planner,
                stop,
                MissionKind::InciteUprising,
                (target, None),
                None,
                actions,
            );
        }

        let mut diplomacy_targets: Vec<(SystemKey, bool, f32)> = Self::system_records(round)
            .filter(|r| r.q & 0x40 != 0)
            .filter(|r| !Self::diplomacy_claimed(missions, faction, r.key))
            .filter(|r| Self::admits(round, faction, MissionKind::Diplomacy, r.key, None, None))
            .map(|r| {
                let popularity = faction.system_popularity(&world.systems[r.key]);
                (r.key, !r.own, popularity)
            })
            .collect();
        // FUN_00481720 queries our systems, then neutral ones.
        diplomacy_targets.sort_by(|a, b| {
            a.1.cmp(&b.1)
                .then(a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
        });
        Self::trace_scored(world, "diplomacy_candidates", || {
            diplomacy_targets
                .iter()
                .map(|&(key, _, popularity)| (key, f64::from(popularity)))
                .collect()
        });
        for (target, _, _) in diplomacy_targets {
            if !Self::dispatch_planned(
                planner,
                stop,
                MissionKind::Diplomacy,
                (target, None),
                None,
                actions,
            ) {
                break;
            }
        }

        let side = crate::dat::Faction::from(faction.as_mission_faction());
        let mut pool = crate::missions::recruit_pool(world, side).len();
        if let Some(base) = Self::find_friendly_system(round, faction) {
            Self::trace_scored(world, "recruitment_base", || {
                vec![(
                    base,
                    f64::from(faction.system_popularity(&world.systems[base])),
                )]
            });
            while pool > 0 && !world.recruit_pool_empty(side) {
                if !Self::dispatch_planned(
                    planner,
                    stop,
                    MissionKind::Recruitment,
                    (base, None),
                    None,
                    actions,
                ) {
                    break;
                }
                pool -= 1;
            }
        }
    }

    /// The Recruitment base: `sys+0x2c` bit 2 (`FUN_00433e40.c:845-849`), a
    /// populated system of ours with no uprising or blockade (`sys+0x28 &
    /// 3`) that is not system `0x121` (`sys+0x30 & 0x8000000`). hyp: the
    /// side object's `+0xa4` (`FUN_00433e40.c:62`, `:845`), which the bit
    /// also needs, is untraced and read as set. port: the most popular such
    /// system.
    fn find_friendly_system(round: &Round<'_>, faction: AiFaction) -> Option<SystemKey> {
        let world = round.inputs.world;
        Self::system_records(round)
            .filter(|r| r.own && r.populated)
            .filter(|r| r.a & crate::ai_production::bit::BLOCKED == 0)
            .filter(|r| r.s & crate::ai_records::sys::SPECIAL == 0)
            .filter(|r| Self::admits(round, faction, MissionKind::Recruitment, r.key, None, None))
            .max_by(|a, b| {
                faction
                    .system_popularity(&world.systems[a.key])
                    .partial_cmp(&faction.system_popularity(&world.systems[b.key]))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.key)
    }

    /// The Incite target: `sys+0x2c` bit `0x100` in an enemy system
    /// (`FUN_00480370.c:109`, `FUN_0041a340(.., 0x100, 4, ..)`), set where
    /// our support is above 50 (`FUN_00433e40.c:636-637`). port: our most
    /// popular such system.
    fn find_incite_target(round: &Round<'_>, faction: AiFaction) -> Option<SystemKey> {
        let world = round.inputs.world;
        Self::system_records(round)
            .filter(|r| r.enemy && r.q & 0x100 != 0)
            .filter(|r| {
                Self::admits(
                    round,
                    faction,
                    MissionKind::InciteUprising,
                    r.key,
                    None,
                    None,
                )
            })
            .max_by(|a, b| {
                faction
                    .system_popularity(&world.systems[a.key])
                    .partial_cmp(&faction.system_popularity(&world.systems[b.key]))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.key)
    }

    // -----------------------------------------------------------------------
    // Espionage heuristics
    // -----------------------------------------------------------------------

    /// Dispatch covert operatives on Sabotage, Assassination, Abduction, and
    /// Espionage missions against the enemy faction.
    ///
    /// port: the targets are the port's (side `+0x318` is untraced); the
    /// planner of each kind picks the members (`crate::mission_planning`).
    /// 1. **Sabotage**: enemy systems with manufacturing facilities, the most
    ///    first.
    /// 2. **Assassination**, then **Abduction**: enemy characters.
    /// 3. **Espionage**: systems with `sys+0x2c` bit `0x200` (held by the
    ///    side or its enemy, `FUN_00433e40`), the enemy's first
    ///    (`FUN_004b1bf0.c:31-52`).
    ///
    /// A kind stops once its planner finds no team.
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    pub(crate) fn evaluate_espionage(
        planner: &mut Planner<'_>,
        round: &Round<'_>,
        faction: AiFaction,
        limit: usize,
        actions: &mut Vec<AIAction>,
    ) {
        let world = round.inputs.world;
        let stop = actions.len().saturating_add(limit);

        // ── Priority 1: Sabotage enemy manufacturing systems ─────────────────
        // Score each enemy system by number of mfg facilities (proxy for value).
        // port: the planner's object choice is untraced (F-019 phase 5); the
        // AI names the system's first enemy manufacturing facility.
        let mut sabotage_targets: Vec<(SystemKey, usize, ManufacturingFacilityKey)> = world
            .systems
            .iter()
            .filter_map(|(sys_key, system)| {
                // Target systems where the enemy faction has manufacturing presence.
                let enemy_mfg: Vec<ManufacturingFacilityKey> = system
                    .manufacturing_facilities
                    .iter()
                    .copied()
                    .filter(|mfk| {
                        world
                            .manufacturing_facilities
                            .get(*mfk)
                            .is_some_and(|f| match faction {
                                AiFaction::Alliance => f.side == crate::dat::Faction::Empire,
                                AiFaction::Empire => f.side == crate::dat::Faction::Alliance,
                            })
                    })
                    .collect();
                enemy_mfg
                    .first()
                    .map(|&first| (sys_key, enemy_mfg.len(), first))
            })
            .filter(|&(system, _, facility)| {
                let target = crate::missions::MissionTarget::ManufacturingFacility(facility);
                Self::admits(
                    round,
                    faction,
                    MissionKind::Sabotage,
                    system,
                    None,
                    Some(target),
                )
            })
            .collect();

        // Highest facility count first.
        sabotage_targets.sort_by_key(|a| std::cmp::Reverse(a.1));
        Self::trace_scored(world, "sabotage_candidates", || {
            sabotage_targets
                .iter()
                .map(|&(key, facilities, _)| (key, facilities as f64))
                .collect()
        });

        for &(target_sys, _, facility) in &sabotage_targets {
            let target = Some(crate::missions::MissionTarget::ManufacturingFacility(
                facility,
            ));
            if !Self::dispatch_planned(
                planner,
                stop,
                MissionKind::Sabotage,
                (target_sys, None),
                target,
                actions,
            ) {
                break;
            }
        }

        // ── Priority 2: Assassination, then Abduction, of enemy characters ──
        // The target chooser (FUN_004bd0a0) sends the order to the target's
        // system; a character in a fleet stands at the fleet's system.
        let whereabouts = |c: &Character| {
            c.current_system.or_else(|| {
                c.current_fleet
                    .and_then(|fleet| world.fleets.get(fleet))
                    .map(|fleet| fleet.location)
            })
        };
        let enemy_major_chars: Vec<(CharacterKey, SystemKey)> = world
            .characters
            .iter()
            .filter(|(_, c)| c.is_major && !faction.owns_character(c) && !c.is_killed)
            // port: a target en route would end the mission at once (rule 5).
            .filter(|(_, c)| !c.is_captive && !c.en_route)
            .filter_map(|(key, c)| whereabouts(c).map(|system| (key, system)))
            .filter(|&(key, system)| {
                Self::admits(
                    round,
                    faction,
                    MissionKind::Assassination,
                    system,
                    Some(key),
                    None,
                )
            })
            .collect();

        // Abduction targets: ALL enemy characters, sorted by lowest combat defense
        // (weakest first) to maximize capture probability.
        let mut abduction_targets: Vec<(CharacterKey, SystemKey, u32)> = world
            .characters
            .iter()
            .filter(|(_, c)| {
                !faction.owns_character(c) && !c.is_captive && !c.is_killed && !c.en_route
            })
            .filter_map(|(key, c)| {
                let defense = c.combat.base + c.combat.variance / 2;
                whereabouts(c).map(|system| (key, system, defense))
            })
            .filter(|&(key, system, _)| {
                Self::admits(
                    round,
                    faction,
                    MissionKind::Abduction,
                    system,
                    Some(key),
                    None,
                )
            })
            .collect();
        abduction_targets.sort_by_key(|a| a.2);
        crate::trace::record(
            crate::trace::TraceMask::AI,
            "covert_character_candidates",
            || {
                let name = |key: CharacterKey| world.characters.get(key).map(|c| c.name.clone());
                serde_json::json!({
                    "assassination": enemy_major_chars
                        .iter()
                        .map(|&(key, system)| serde_json::json!({
                            "character": name(key),
                            "system": system_label(world, system),
                        }))
                        .collect::<Vec<_>>(),
                    "abduction": abduction_targets
                        .iter()
                        .map(|&(key, system, defense)| serde_json::json!({
                            "character": name(key),
                            "system": system_label(world, system),
                            "score": defense,
                        }))
                        .collect::<Vec<_>>(),
                })
            },
        );

        for &(target_char, target_sys) in &enemy_major_chars {
            let target = (target_sys, Some(target_char));
            if !Self::dispatch_planned(
                planner,
                stop,
                MissionKind::Assassination,
                target,
                None,
                actions,
            ) {
                break;
            }
        }

        for &(target_char, target_sys, _) in &abduction_targets {
            let target = (target_sys, Some(target_char));
            if !Self::dispatch_planned(planner, stop, MissionKind::Abduction, target, None, actions)
            {
                break;
            }
        }

        // ── Priority 3: Espionage ─────────────────────────────────────────
        // FUN_004b1bf0 asks FUN_0041a3a0(.., 0x200, 4): sys+0x2c bit 0x200
        // in an enemy system, else FUN_0041a460(.., 0x200, 0): in any
        // (:31-52). Bit 0x200 marks an explored system the side or its
        // enemy holds (FUN_00433e40). port: the original asks within the
        // sector of the regional record whose posture asked for Espionage
        // (module 4 states 2 and 3, not ported); the port asks galaxy-wide,
        // in the world's order.
        let mut espionage_targets: Vec<(bool, SystemKey)> = Self::system_records(round)
            .filter(|r| r.q & 0x200 != 0)
            .filter(|r| Self::admits(round, faction, MissionKind::Espionage, r.key, None, None))
            .map(|r| (!r.enemy, r.key))
            .collect();
        espionage_targets.sort_by_key(|&(not_enemy, _)| not_enemy);
        let espionage_targets: Vec<SystemKey> =
            espionage_targets.into_iter().map(|(_, key)| key).collect();
        Self::trace_scored(world, "espionage_candidates", || {
            espionage_targets.iter().map(|&key| (key, 0.0)).collect()
        });

        for &target_sys in &espionage_targets {
            if !Self::dispatch_planned(
                planner,
                stop,
                MissionKind::Espionage,
                (target_sys, None),
                None,
                actions,
            ) {
                break;
            }
        }
    }

    // -----------------------------------------------------------------------
    // Rescue heuristics
    // -----------------------------------------------------------------------

    /// Dispatch a rescue mission to free captive allies.
    ///
    /// Scans for characters held captive by the enemy; the Rescue planner
    /// (`FUN_0047e270`, `FUN_0047e3f0`) picks each team.
    pub(crate) fn evaluate_rescue(
        planner: &mut Planner<'_>,
        round: &Round<'_>,
        faction: AiFaction,
        limit: usize,
        actions: &mut Vec<AIAction>,
    ) {
        let world = round.inputs.world;
        let stop = actions.len().saturating_add(limit);
        // Find captive allies (our characters held by the enemy).
        let captives: Vec<(CharacterKey, SystemKey)> = world
            .characters
            .iter()
            .filter_map(|(key, c)| {
                // port: a captive en route would end the mission at once
                // (rule 5).
                if c.is_captive && faction.owns_character(c) && !c.en_route {
                    c.current_system.map(|sys| (key, sys))
                } else {
                    None
                }
            })
            .filter(|&(key, system)| {
                Self::admits(round, faction, MissionKind::Rescue, system, Some(key), None)
            })
            .collect();

        crate::trace::record(crate::trace::TraceMask::AI, "rescue_candidates", || {
            let candidates: Vec<_> = captives
                .iter()
                .map(|&(key, system)| {
                    serde_json::json!({
                        "character": world.characters.get(key).map(|c| c.name.clone()),
                        "system": system_label(world, system),
                    })
                })
                .collect();
            serde_json::json!({ "candidates": candidates })
        });
        // One rescue per captive while the planner finds a team.
        for (captive_key, captive_system) in captives {
            let target = (captive_system, Some(captive_key));
            if !Self::dispatch_planned(planner, stop, MissionKind::Rescue, target, None, actions) {
                break;
            }
        }
    }

    // -----------------------------------------------------------------------
    // Reconnaissance heuristics (D5)
    // -----------------------------------------------------------------------

    /// Dispatch reconnaissance missions to gather intelligence on enemy systems.
    ///
    /// This targets explored enemy-controlled systems to update our
    /// intelligence picture. The original game uses
    /// mission type 0x54 for this — we reuse `MissionKind::Espionage` since
    /// the mechanic is the same (reveal system state).
    ///
    /// Priority: enemy-controlled systems with the most fleets/facilities
    /// (highest strategic value to scout).
    pub(crate) fn evaluate_reconnaissance(
        planner: &mut Planner<'_>,
        round: &Round<'_>,
        faction: AiFaction,
        limit: usize,
        actions: &mut Vec<AIAction>,
    ) {
        let world = round.inputs.world;
        let stop = actions.len().saturating_add(limit);
        let is_alliance = matches!(faction, AiFaction::Alliance);
        let enemy_faction = if is_alliance {
            crate::dat::Faction::Empire
        } else {
            crate::dat::Faction::Alliance
        };

        // Find explored enemy systems worth scouting.
        let mut recon_targets: Vec<(SystemKey, u32)> = world
            .systems
            .iter()
            .filter_map(|(sys_key, system)| {
                if !system.control.is_controlled_by(enemy_faction) {
                    return None;
                }
                if system.is_destroyed {
                    return None;
                }
                // Only target explored systems. Score by total enemy assets.
                if system.exploration_status != crate::dat::ExplorationStatus::Explored {
                    return None;
                }
                let value = Self::system_strength(world, system, !is_alliance);
                Some((sys_key, value))
            })
            .filter(|&(system, _)| {
                Self::admits(round, faction, MissionKind::Espionage, system, None, None)
            })
            .collect();

        if recon_targets.is_empty() {
            return;
        }

        // Sort by highest value first (most important to scout).
        recon_targets.sort_by_key(|a| std::cmp::Reverse(a.1));
        Self::trace_scored(world, "reconnaissance_candidates", || {
            recon_targets
                .iter()
                .map(|&(key, strength)| (key, f64::from(strength)))
                .collect()
        });

        for (target_sys, _) in recon_targets {
            if !Self::dispatch_planned(
                planner,
                stop,
                MissionKind::Espionage,
                (target_sys, None),
                None,
                actions,
            ) {
                break;
            }
        }
    }

    /// A garrison strength score for a system: the side's live hull total,
    /// its regiment strength, and ten per facility. Higher is more heavily
    /// defended.
    ///
    /// port: ranks the reconnaissance targets above; the original's
    /// per-system recommendation (`FUN_004b7150`) is traced
    /// (`ai-mission-planning.md`) but not ported.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    fn system_strength(world: &GameWorld, sys: &crate::world::System, is_alliance: bool) -> u32 {
        // Ship hull total from friendly fleets at this system
        let ship_strength: u32 = sys
            .fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .filter(|f| f.is_alliance == is_alliance)
            .flat_map(|f| &f.capital_ships)
            .filter(|ship| ship.alive)
            .map(|ship| ship.hull_current.max(0) as u32)
            .fold(0, u32::saturating_add);

        // Troop strength from friendly ground units
        let troop_strength: u32 = sys
            .ground_units
            .iter()
            .filter_map(|&tk| world.troops.get(tk))
            .filter(|t| t.is_alliance == is_alliance)
            .map(|t| t.regiment_strength as u32)
            .fold(0, u32::saturating_add);

        // Facility count (defense + manufacturing)
        let facility_count =
            sys.defense_facilities.len() as u32 + sys.manufacturing_facilities.len() as u32;

        ship_strength
            .saturating_add(troop_strength)
            .saturating_add(facility_count.saturating_mul(10))
    }

    /// Strategic: send diplomats to low-support controlled systems to prevent uprisings.
    ///
    /// Scans systems where our popularity < 0.4 and sends the Diplomacy
    /// planner's diplomat (`FUN_004815a0`) to the two lowest, to stabilize
    /// support before an uprising can trigger (uprising threshold is
    /// popularity-dependent via UPRIS1TB). Each target is one Diplomacy's
    /// own query takes (`sys+0x2c` bit `0x40`, `FUN_00481720.c:97-268`) and
    /// the creation validator admits, and no Diplomacy of ours works it.
    pub(crate) fn evaluate_uprising_prevention(
        planner: &mut Planner<'_>,
        round: &Round<'_>,
        missions: &MissionState,
        faction: AiFaction,
        limit: usize,
        actions: &mut Vec<AIAction>,
    ) {
        let world = round.inputs.world;
        let stop = actions.len().saturating_add(limit);
        let is_alliance = matches!(faction, AiFaction::Alliance);
        let our_faction = if is_alliance {
            crate::dat::Faction::Alliance
        } else {
            crate::dat::Faction::Empire
        };

        // Find controlled systems with dangerously low support.
        let mut at_risk: Vec<SystemKey> = Vec::new();
        for (sys_key, system) in &world.systems {
            if !system.control.is_controlled_by(our_faction) {
                continue;
            }
            let candidate = round
                .records
                .get(&sys_key)
                .is_some_and(|r| r.own && r.q & 0x40 != 0);
            if !candidate
                || Self::diplomacy_claimed(missions, faction, sys_key)
                || !Self::admits(round, faction, MissionKind::Diplomacy, sys_key, None, None)
            {
                continue;
            }
            let support = if is_alliance {
                system.popularity_alliance
            } else {
                system.popularity_empire
            };
            if support < 0.4 {
                at_risk.push(sys_key);
            }
        }

        if at_risk.is_empty() {
            return;
        }

        // Sort by lowest support first (most urgent).
        at_risk.sort_by(|a, b| {
            let sup_a = if is_alliance {
                world.systems[*a].popularity_alliance
            } else {
                world.systems[*a].popularity_empire
            };
            let sup_b = if is_alliance {
                world.systems[*b].popularity_alliance
            } else {
                world.systems[*b].popularity_empire
            };
            sup_a
                .partial_cmp(&sup_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Self::trace_scored(world, "uprising_prevention_candidates", || {
            at_risk
                .iter()
                .map(|&key| {
                    let sys = &world.systems[key];
                    let support = if is_alliance {
                        sys.popularity_alliance
                    } else {
                        sys.popularity_empire
                    };
                    (key, f64::from(support))
                })
                .collect()
        });
        // Limit to 2 uprising-prevention diplomats per cycle.
        for &target in at_risk.iter().take(2) {
            if !Self::dispatch_planned(
                planner,
                stop,
                MissionKind::Diplomacy,
                (target, None),
                None,
                actions,
            ) {
                break;
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
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::{DatId, SectorKey};
    use crate::manufacturing::ManufacturingState;
    use crate::tuning::GameConfig;
    use crate::world::{
        Character, ControlKind, FighterClass, Fleet, GameWorld, Sector, SkillPair, System,
    };

    #[test]
    fn every_character_a_mission_frees_leaves_the_busy_set() {
        use crate::missions::{MissionEffect, MissionMember};
        let mut keys: slotmap::SlotMap<CharacterKey, ()> = slotmap::SlotMap::with_key();
        let (lead, second, idle) = (keys.insert(()), keys.insert(()), keys.insert(()));
        let mut units: slotmap::SlotMap<crate::ids::SpecialForceKey, ()> =
            slotmap::SlotMap::with_key();
        let mut ai = AIState::new(AiFaction::Empire);
        for character in [lead, second, idle] {
            ai.mark_busy(character);
        }

        ai.free_mission_members(&[
            MissionEffect::MemberAvailable {
                member: MissionMember::Character(lead),
            },
            MissionEffect::MemberAvailable {
                member: MissionMember::SpecialForce(units.insert(())),
            },
            MissionEffect::MemberAvailable {
                member: MissionMember::Character(second),
            },
        ]);

        assert!(!ai.is_busy(lead));
        assert!(!ai.is_busy(second));
        assert!(ai.is_busy(idle));
    }

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

    fn add_system(
        world: &mut GameWorld,
        sector: SectorKey,
        pop_alliance: f32,
        pop_empire: f32,
    ) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId(0),
            name: "Test System".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: pop_alliance,
            popularity_empire: pop_empire,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        })
    }

    fn add_character(
        world: &mut GameWorld,
        is_alliance: bool,
        is_major: bool,
        diplomacy_base: u32,
    ) -> CharacterKey {
        world.characters.insert(Character {
            name: "TestChar".into(),
            is_alliance,
            is_empire: !is_alliance,
            is_major,
            diplomacy: SkillPair {
                base: diplomacy_base,
                variance: 0,
            },
            leadership: SkillPair {
                base: 50,
                variance: 0,
            },
            can_be_commander: true,
            // A placed character belongs to its side's roster (+0x50 bit 2).
            recruited: true,
            ..Default::default()
        })
    }

    // -----------------------------------------------------------------------
    // AIState tests
    // -----------------------------------------------------------------------

    #[test]
    fn busy_character_can_be_marked_and_cleared() {
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

    // -----------------------------------------------------------------------
    // Officer heuristics
    // -----------------------------------------------------------------------

    /// The characters of the first `DispatchMission` of `kind`'s team.
    fn team_of(
        actions: &[AIAction],
        kind: MissionKind,
    ) -> Option<Vec<crate::missions::MissionMember>> {
        actions.iter().find_map(|a| match a {
            AIAction::DispatchMission { kind: k, team, .. } if *k == kind => Some(team.clone()),
            _ => None,
        })
    }

    /// One day at Fast for `state`'s side, its orders in issue order. Rate
    /// 4 gives five in-day steps (`FUN_0051dea0.c:11-19, 30`), each running
    /// one round (`AISystem::round`), which ends at the first module that
    /// finishes with an order (`FUN_00485b40.c:53-67`); module 4 runs one
    /// planner's pass a turn (`FUN_004d2490.c:26-46`). Between rounds the
    /// applier's part: the team marked busy and the order accepted. Then
    /// the day's end (`FUN_00485de0.c:7-11`). The world does not change.
    fn day_of_rounds(state: &mut AIState, world: &GameWorld) -> Vec<AIAction> {
        let manufacturing = ManufacturingState::new();
        let movement = crate::movement::MovementState::new();
        let config = GameConfig::default();
        let research = crate::research::ResearchState::new();
        let uprisings = crate::uprising::UprisingState::new();
        let transport = crate::troop_transport::TroopTransportState::new();
        let missions = crate::missions::MissionState::new();
        let rate = config.ai.rounds_per_day.fast - 1;
        let mut orders = Vec::new();
        for round_in_day in 0..=rate {
            let inputs = crate::ai_scheduler::AiInputs {
                world,
                manufacturing: &manufacturing,
                movement: &movement,
                config: &config,
                research: &research,
                uprisings: &uprisings,
                transport: &transport,
                missions: &missions,
                tick: 7,
                round_in_day,
            };
            if let Some(issued) = AISystem::round(state, &inputs) {
                if let AIAction::DispatchMission { team, .. } = &issued.action {
                    for character in team.iter().filter_map(|m| m.character()) {
                        state.mark_busy(character);
                    }
                }
                AISystem::settle(state, issued.kind, true);
                orders.push(issued.action);
            }
        }
        AISystem::end_day(state);
        orders
    }

    #[test]
    fn a_recruiter_needs_leadership_of_80_and_someone_left_to_recruit() {
        // FUN_0047db40: one member, leadership 80..=10000. FUN_0055fc80
        // recruits from the side's unrecruited minor characters.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let sys = add_system(&mut world, sector, 0.1, 0.9);
        // The Recruitment base is a system of ours (sys+0x2c bit 2,
        // FUN_00433e40.c:845-849; FUN_0047dcc0.c:62).
        world.systems[sys].control = ControlKind::Controlled(crate::dat::Faction::Empire);
        let recruiter = add_character(&mut world, false, true, 30);
        world.characters[recruiter].leadership.base = 80;
        let weaker = add_character(&mut world, false, true, 30);
        world.characters[weaker].leadership.base = 79;
        let mut state = AIState::new(AiFaction::Empire);

        assert_eq!(
            team_of(&day_of_rounds(&mut state, &world), MissionKind::Recruitment),
            None
        );

        let recruit = add_character(&mut world, false, false, 30);
        world.characters[recruit].recruited = false;
        let mut state = AIState::new(AiFaction::Empire);
        let team = team_of(&day_of_rounds(&mut state, &world), MissionKind::Recruitment);

        assert_eq!(
            team,
            Some(vec![crate::missions::MissionMember::Character(recruiter)])
        );
    }

    #[test]
    fn a_diplomat_needs_diplomacy_of_69() {
        // FUN_004815a0: one member, diplomacy 69..=10000.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _ = add_system(&mut world, sector, 0.1, 0.2);
        let _ = add_character(&mut world, false, false, 68);
        let mut state = AIState::new(AiFaction::Empire);

        assert_eq!(
            team_of(&day_of_rounds(&mut state, &world), MissionKind::Diplomacy),
            None
        );

        let diplomat = add_character(&mut world, false, false, 69);
        let mut state = AIState::new(AiFaction::Empire);
        let team = team_of(&day_of_rounds(&mut state, &world), MissionKind::Diplomacy);

        assert_eq!(
            team,
            Some(vec![crate::missions::MissionMember::Character(diplomat)])
        );
    }

    #[test]
    fn low_diplomacy_minor_not_dispatched() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let _ = add_system(&mut world, sector, 0.1, 0.2);

        // Minor with low diplomacy — below threshold
        let _ = add_character(&mut world, false, false, 30);

        let mut state = AIState::new(AiFaction::Empire);

        let actions = day_of_rounds(&mut state, &world);
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

        let actions = day_of_rounds(&mut state, &world);
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
    fn uncontrolled_system_facility_cannot_produce_for_ai() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let mfg_key =
            world
                .manufacturing_facilities
                .insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(1),
                    side: crate::dat::Faction::Empire,
                    is_shipyard: false,
                });
        let sys_key = add_system(&mut world, sector, 0.5, 0.5);
        world.systems[sys_key]
            .manufacturing_facilities
            .push(mfg_key);
        world.systems[sys_key].control = ControlKind::Uncontrolled;
        world.fighter_classes.insert(FighterClass {
            dat_id: DatId(10),
            name: "TIE Fighter".into(),
            is_empire: true,
            refined_material_cost: 20,
            overall_attack_strength: 10,
            ..FighterClass::default()
        });

        let mut state = AIState::new(AiFaction::Empire);
        let actions = day_of_rounds(&mut state, &world);

        assert!(!actions.iter().any(|action| matches!(
            action,
            AIAction::EnqueueProduction { system, .. } if *system == sys_key
        )));
    }

    // -----------------------------------------------------------------------
    // Espionage heuristics
    // -----------------------------------------------------------------------

    /// Helper: insert a character with custom espionage + combat scores.
    /// A covert operative. The covert tables read their input as the chance
    /// (row `t` holds `t`), so the chance follows the operative's skills
    /// through `member_chance`.
    fn add_spy(
        world: &mut GameWorld,
        is_alliance: bool,
        is_major: bool,
        espionage_base: u32,
        combat_base: u32,
    ) -> CharacterKey {
        for key in ["SBTGMSTB", "ESPIMSTB", "ASSNMSTB", "ABDCMSTB"] {
            let rows = (0..=100)
                .map(|threshold| crate::world::MstbEntry {
                    threshold,
                    value: threshold.unsigned_abs(),
                })
                .collect();
            world
                .mission_tables
                .insert(key.to_string(), crate::world::MstbTable::new(rows));
        }
        world.characters.insert(Character {
            name: "TestSpy".into(),
            is_alliance,
            is_empire: !is_alliance,
            is_major,
            espionage: SkillPair {
                base: espionage_base,
                variance: 0,
            },
            combat: SkillPair {
                base: combat_base,
                variance: 0,
            },
            leadership: SkillPair {
                base: 50,
                variance: 0,
            },
            can_be_commander: true,
            recruited: true,
            ..Default::default()
        })
    }

    /// A special force of `espionage` and `combat`, standing nowhere.
    fn add_commando(
        world: &mut GameWorld,
        is_alliance: bool,
        espionage: u32,
        combat: u32,
    ) -> crate::missions::MissionMember {
        let mut skills = [0; 8];
        skills[crate::world::Skill::Espionage as usize] = espionage;
        skills[crate::world::Skill::Combat as usize] = combat;
        crate::missions::MissionMember::SpecialForce(world.special_forces.insert(
            crate::world::SpecialForceUnit {
                class_dat_id: DatId(0x3c00_0002),
                is_alliance,
                skills,
                on_mission: false,
                en_route: false,
                on_hidden_mission: false,
            },
        ))
    }

    /// Whether the Empire sends a special force of `espionage` and `combat`
    /// to sabotage an Alliance manufacturing system.
    fn sabotage_dispatched(espionage: u32, combat: u32) -> bool {
        sabotage_dispatched_by(AiFaction::Empire, |world, alliance| {
            add_commando(world, alliance, espionage, combat)
        })
    }

    /// Whether `side`'s AI sends the agent `add` makes to sabotage the enemy
    /// shipyard, naming the shipyard as the target object.
    fn sabotage_dispatched_by(
        side: AiFaction,
        add: impl FnOnce(&mut GameWorld, bool) -> crate::missions::MissionMember,
    ) -> bool {
        let alliance_ai = side == AiFaction::Alliance;
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Enemy (alliance) system with a manufacturing facility — a sabotage target.
        let mfg_key =
            world
                .manufacturing_facilities
                .insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(1),
                    side: crate::dat::Faction::of_alliance(!alliance_ai), // owned by the AI's enemy
                    is_shipyard: false,
                });
        let enemy_sys = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Enemy Shipyard".into(),
            sector,
            x: 10,
            y: 10,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.1,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![mfg_key],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });

        let spy = add(&mut world, alliance_ai);

        let mut state = AIState::new(side);

        let actions = day_of_rounds(&mut state, &world);

        let sabotage = actions.iter().find(|a| {
            matches!(
                a,
                AIAction::DispatchMission {
                    kind: MissionKind::Sabotage,
                    team,
                    target_system,
                    target_object: Some(crate::missions::MissionTarget::ManufacturingFacility(yard)),
                    ..
                }
                if *team == [spy] && *target_system == enemy_sys && *yard == mfg_key
            )
        });
        sabotage.is_some()
    }

    #[test]
    fn a_sabotage_team_needs_combat_and_espionage_of_51() {
        // FUN_0047d520 queries combat 51..=10000 and espionage 51..=10000
        // and takes only hits of both (FUN_004357b0).
        assert!(sabotage_dispatched(51, 51));
        assert!(!sabotage_dispatched(50, 80));
        assert!(!sabotage_dispatched(80, 50));
    }

    #[test]
    fn an_alliance_commando_sabotages_an_empire_shipyard() {
        assert!(sabotage_dispatched_by(
            AiFaction::Alliance,
            |world, alliance| { add_commando(world, alliance, 80, 60) }
        ));
    }

    #[test]
    fn low_espionage_character_not_dispatched_on_covert_ops() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        let mfg_key =
            world
                .manufacturing_facilities
                .insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(1),
                    side: crate::dat::Faction::Alliance,
                    is_shipyard: false,
                });
        let _ = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Enemy Shipyard".into(),
            sector,
            x: 10,
            y: 10,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.1,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
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

        let actions = day_of_rounds(&mut state, &world);

        let covert = actions
            .iter()
            .filter(|a| {
                matches!(
                    a,
                    AIAction::DispatchMission {
                        kind: MissionKind::Sabotage
                            | MissionKind::Assassination
                            | MissionKind::Espionage,
                        ..
                    }
                )
            })
            .count();
        assert_eq!(
            covert, 0,
            "low-espionage character should not be dispatched on covert ops"
        );
    }

    #[test]
    fn ai_dispatches_espionage_on_an_enemy_system() {
        // FUN_004b1bf0.c:31-52: an explored enemy system (sys+0x2c bit
        // 0x200, FUN_00433e40) is an Espionage target.
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        let enemy = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Dark System".into(),
            sector,
            x: 50,
            y: 50,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.0,
            popularity_empire: 0.0,
            is_populated: false,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        });

        // FUN_004808c0 takes special forces only.
        let spy = add_commando(&mut world, true, 75, 40);

        let mut state = AIState::new(AiFaction::Alliance);

        let actions = day_of_rounds(&mut state, &world);

        let intel = actions.iter().find(|a| {
            matches!(
                a,
                AIAction::DispatchMission {
                    kind: MissionKind::Espionage,
                    team,
                    target_system,
                    ..
                }
                if *team == [spy] && *target_system == enemy
            )
        });
        assert!(
            intel.is_some(),
            "expected an espionage mission on the enemy system"
        );
    }

    #[test]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    fn a_day_at_fast_sends_at_most_one_covert_op() {
        // A round issues at most one order (FUN_00485b40.c:53-67), and module
        // 4 runs one planner's pass a turn (FUN_004d2490.c:26-46): in five
        // rounds the planner list reaches the Espionage pseudo-planner once.
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Five enemy manufacturing systems: more targets than turns.
        for i in 0..5u32 {
            let mfg_key = world.manufacturing_facilities.insert(
                crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(i),
                    side: crate::dat::Faction::Alliance,
                    is_shipyard: false,
                },
            );
            world.systems.insert(System {
                dat_id: DatId(i),
                name: format!("Enemy System {i}"),
                sector,
                x: (i * 10) as u16,
                y: 0,
                exploration_status: crate::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.7,
                popularity_empire: 0.1,
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                espionage_rating: 0.0,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                fighters: Vec::new(),
                defense_facilities: vec![],
                manufacturing_facilities: vec![mfg_key],
                production_facilities: vec![],
                is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Uncontrolled,
            });
        }

        // 10 Empire special forces — all highly skilled.
        for _ in 0..10 {
            add_commando(&mut world, false, 80, 60);
        }

        let mut state = AIState::new(AiFaction::Empire);

        let actions = day_of_rounds(&mut state, &world);

        let covert_count = actions
            .iter()
            .filter(|a| {
                matches!(
                    a,
                    AIAction::DispatchMission {
                        kind: MissionKind::Sabotage
                            | MissionKind::Assassination
                            | MissionKind::Espionage,
                        ..
                    }
                )
            })
            .count();

        assert!(
            covert_count <= 1,
            "expected at most 1 covert op, got {covert_count}",
        );
    }

    // -----------------------------------------------------------------------
    // D5: Reconnaissance test
    // -----------------------------------------------------------------------

    #[test]
    fn reconnaissance_targets_explored_enemy_systems() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);

        // Explored enemy system — a recon target.
        let enemy_sys = world.systems.insert(System {
            dat_id: DatId(0),
            name: "Enemy Base".into(),
            sector,
            x: 100,
            y: 100,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.1,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        // Empire special force for recon.
        let spy = add_commando(&mut world, false, 70, 40);

        let mut planner = Planner::new(&world, &HashSet::new(), false, 7, [false; 3]);
        let mut actions = Vec::new();
        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_reconnaissance(
                &mut planner,
                round,
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let recon = actions.iter().find(|a| {
            matches!(
                a,
                AIAction::DispatchMission {
                    kind: MissionKind::Espionage,
                    team,
                    target_system,
                    ..
                }
                if *team == [spy] && *target_system == enemy_sys
            )
        });
        assert!(
            recon.is_some(),
            "expected reconnaissance mission on explored enemy system"
        );
    }

    // -----------------------------------------------------------------------
    // Planned missions (F-019 phase 5a)
    // -----------------------------------------------------------------------

    use crate::missions::MissionMember;

    fn planner_for(world: &GameWorld, faction: AiFaction) -> Planner<'_> {
        Planner::new(
            world,
            &HashSet::new(),
            faction == AiFaction::Alliance,
            7,
            [false; 3],
        )
    }

    /// `f` over a round of `faction`'s AI on `world`, every other state
    /// empty.
    fn with_round<R>(world: &GameWorld, faction: AiFaction, f: impl FnOnce(&Round<'_>) -> R) -> R {
        let manufacturing = ManufacturingState::new();
        let transport = crate::troop_transport::TroopTransportState::new();
        let uprisings = crate::uprising::UprisingState::new();
        let research = crate::research::ResearchState::new();
        let movement = crate::movement::MovementState::new();
        let marks = crate::ai_records::Marks::default();
        let inputs = crate::ai_production::RoundInputs {
            world,
            manufacturing: &manufacturing,
            transport: &transport,
            uprisings: &uprisings,
            research: &research,
            movement: &movement,
            marks: &marks,
            side: faction.as_mission_faction().into(),
            tick: 7,
            round_in_day: 0,
        };
        let round = Round::new(&crate::ai_production::ProductionState::default(), &inputs);
        f(&round)
    }

    /// Each `kind` order as (team, target system, target character).
    fn orders(
        actions: &[AIAction],
        kind: MissionKind,
    ) -> Vec<(Vec<MissionMember>, SystemKey, Option<CharacterKey>)> {
        actions
            .iter()
            .filter_map(|a| match a {
                AIAction::DispatchMission {
                    kind: k,
                    team,
                    target_system,
                    target_character,
                    ..
                } if *k == kind => Some((team.clone(), *target_system, *target_character)),
                _ => None,
            })
            .collect()
    }

    /// `n` special forces of `combat` with no espionage, so none is a decoy.
    fn fighters(world: &mut GameWorld, is_alliance: bool, n: usize) -> Vec<MissionMember> {
        (0..n)
            .map(|_| add_commando(world, is_alliance, 0, 90))
            .collect()
    }

    fn at(world: &mut GameWorld, character: CharacterKey, system: SystemKey) {
        world.characters[character].current_system = Some(system);
    }

    #[test]
    fn each_diplomat_goes_to_another_system_least_popular_first() {
        // A populated neutral system with no foreign fleet is a Diplomacy
        // target at any support (sys+0x2c bit 0x40, FUN_00433e40.c:617-620);
        // port: the least popular goes first. FUN_004815a0 picks the best
        // diplomat for each.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let warmer = add_system(&mut world, sector, 0.1, 0.3);
        let coldest = add_system(&mut world, sector, 0.1, 0.1);
        let warmest = add_system(&mut world, sector, 0.1, 0.8);
        let best = add_character(&mut world, false, false, 90);
        let next = add_character(&mut world, false, false, 80);
        let third = add_character(&mut world, false, false, 70);
        let mut planner = planner_for(&world, AiFaction::Empire);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_officers(
                &mut planner,
                round,
                &MissionState::new(),
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let character = MissionMember::Character;
        assert_eq!(
            orders(&actions, MissionKind::Diplomacy),
            vec![
                (vec![character(best)], coldest, None),
                (vec![character(next)], warmer, None),
                (vec![character(third)], warmest, None),
            ]
        );
    }

    #[test]
    fn recruitment_sends_one_recruiter_per_character_left_to_recruit() {
        // FUN_0055fc80 recruits from the side's pool; once the side has
        // recruited its last (FUN_0052f590), no Recruitment goes out.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let base = add_system(&mut world, sector, 0.1, 0.9);
        // The Recruitment base is a system of ours (sys+0x2c bit 2,
        // FUN_00433e40.c:845-849; FUN_0047dcc0.c:62).
        world.systems[base].control = ControlKind::Controlled(crate::dat::Faction::Empire);
        for _ in 0..3 {
            let recruiter = add_character(&mut world, false, true, 30);
            world.characters[recruiter].leadership.base = 90;
        }
        for _ in 0..2 {
            let recruit = add_character(&mut world, false, false, 30);
            world.characters[recruit].recruited = false;
        }
        let run = |world: &GameWorld| {
            let mut planner = planner_for(world, AiFaction::Empire);
            let mut actions = Vec::new();
            with_round(world, AiFaction::Empire, |round| {
                AISystem::evaluate_officers(
                    &mut planner,
                    round,
                    &MissionState::new(),
                    AiFaction::Empire,
                    usize::MAX,
                    &mut actions,
                );
            });
            orders(&actions, MissionKind::Recruitment).len()
        };

        assert_eq!(run(&world), 2);

        world.set_recruit_pool_empty(crate::dat::Faction::Empire);

        assert_eq!(run(&world), 0);
    }

    #[test]
    fn a_sabotage_goes_to_each_enemy_yard() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let mut yards = Vec::new();
        for _ in 0..2 {
            let system = add_system(&mut world, sector, 0.8, 0.1);
            let yard = world.manufacturing_facilities.insert(
                crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(1),
                    side: crate::dat::Faction::Alliance,
                    is_shipyard: false,
                },
            );
            world.systems[system].manufacturing_facilities.push(yard);
            yards.push(system);
        }
        for _ in 0..4 {
            add_commando(&mut world, false, 0, 90);
        }
        for member in world.special_forces.values_mut() {
            member.skills[crate::world::Skill::Espionage as usize] = 90;
        }
        let mut planner = planner_for(&world, AiFaction::Empire);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_espionage(
                &mut planner,
                round,
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let targets: Vec<SystemKey> = orders(&actions, MissionKind::Sabotage)
            .into_iter()
            .map(|(_, system, _)| system)
            .collect();
        assert_eq!(targets.len(), 2);
        assert!(yards.iter().all(|yard| targets.contains(yard)));
    }

    #[test]
    fn an_assassination_goes_where_each_free_enemy_major_stands() {
        // FUN_004bd0a0 sends the order to the target's system; a character
        // in a fleet stands at the fleet's system. The original caps no
        // pass: each of the four free majors gets a team while teams last.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let here = add_system(&mut world, sector, 0.8, 0.1);
        let docked = add_system(&mut world, sector, 0.8, 0.1);
        let fleet = world.fleets.insert(Fleet {
            location: docked,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let target = add_character(&mut world, true, true, 30);
        at(&mut world, target, here);
        let aboard = add_character(&mut world, true, true, 30);
        world.characters[aboard].current_fleet = Some(fleet);
        for (captive, killed) in [(true, false), (false, true)] {
            let spared = add_character(&mut world, true, true, 30);
            at(&mut world, spared, here);
            world.characters[spared].is_captive = captive;
            world.characters[spared].is_killed = killed;
        }
        let own = add_character(&mut world, false, true, 30);
        at(&mut world, own, here);
        for _ in 0..2 {
            let other = add_character(&mut world, true, true, 30);
            at(&mut world, other, here);
        }
        fighters(&mut world, false, 10);
        let mut planner = planner_for(&world, AiFaction::Empire);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_espionage(
                &mut planner,
                round,
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let sent = orders(&actions, MissionKind::Assassination);
        assert_eq!(sent.len(), 4);
        assert!(sent.iter().any(|(_, _, c)| *c == Some(target)));
        assert!(sent.iter().any(|(_, _, c)| *c == Some(aboard)));
        for (_, system, character) in &sent {
            let character = character.unwrap();
            assert!(character != own && !world.characters[character].is_captive);
            assert!(!world.characters[character].is_killed);
            let expected = if character == aboard { docked } else { here };
            assert_eq!(*system, expected);
        }
    }

    #[test]
    fn abduction_takes_the_weakest_enemy_first() {
        // port: the port orders abduction targets by combat, base plus half
        // the variance, weakest first. The original caps no pass, so each
        // free enemy gets a team while teams last.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let here = add_system(&mut world, sector, 0.8, 0.1);
        let enemy = |world: &mut GameWorld, base, variance| {
            let key = add_character(world, true, false, 30);
            world.characters[key].combat = SkillPair { base, variance };
            at(world, key, here);
            key
        };
        let middle = enemy(&mut world, 20, 6); // 23
        let weakest = enemy(&mut world, 22, 0);
        let strongest = enemy(&mut world, 24, 0);
        let toughest = enemy(&mut world, 30, 0);
        let prisoner = enemy(&mut world, 1, 0);
        world.characters[prisoner].is_captive = true;
        let own = add_character(&mut world, false, false, 30);
        at(&mut world, own, here);
        fighters(&mut world, false, 8);
        let mut planner = planner_for(&world, AiFaction::Empire);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_espionage(
                &mut planner,
                round,
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let targets: Vec<_> = orders(&actions, MissionKind::Abduction)
            .into_iter()
            .map(|(_, _, character)| character.unwrap())
            .collect();
        assert_eq!(targets, vec![weakest, middle, strongest, toughest]);
    }

    #[test]
    fn espionage_goes_to_enemy_systems_then_our_own() {
        // FUN_004b1bf0.c:31-52: an enemy system with sys+0x2c bit 0x200
        // first, then any system with it; FUN_00433e40 sets the bit on an
        // explored system the side or its enemy holds, so neither a neutral
        // nor an unexplored system is a target.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let held = |world: &mut GameWorld, control| {
            let system = add_system(world, sector, 0.5, 0.5);
            world.systems[system].control = control;
            system
        };
        let ours = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Alliance),
        );
        let first_enemy = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Empire),
        );
        let _neutral = held(&mut world, ControlKind::Uncontrolled);
        let unexplored = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Empire),
        );
        world.systems[unexplored].exploration_status = ExplorationStatus::Unexplored;
        let second_enemy = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Empire),
        );
        for _ in 0..10 {
            add_commando(&mut world, true, 80, 0);
        }
        let mut planner = planner_for(&world, AiFaction::Alliance);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Alliance, |round| {
            AISystem::evaluate_espionage(
                &mut planner,
                round,
                AiFaction::Alliance,
                usize::MAX,
                &mut actions,
            );
        });

        let targets: Vec<_> = orders(&actions, MissionKind::Espionage)
            .into_iter()
            .map(|(_, system, _)| system)
            .collect();
        assert_eq!(targets, vec![first_enemy, second_enemy, ours]);
    }

    #[test]
    fn each_captive_ally_gets_a_rescue_at_its_prison() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let first_prison = add_system(&mut world, sector, 0.1, 0.8);
        let second_prison = add_system(&mut world, sector, 0.1, 0.8);
        let captive = |world: &mut GameWorld, is_alliance, system| {
            let key = add_character(world, is_alliance, false, 30);
            world.characters[key].is_captive = true;
            at(world, key, system);
            key
        };
        let first = captive(&mut world, true, first_prison);
        let second = captive(&mut world, true, second_prison);
        let _enemy_prisoner = captive(&mut world, false, first_prison);
        let free = add_character(&mut world, true, false, 30);
        at(&mut world, free, first_prison);
        fighters(&mut world, true, 6);
        let mut planner = planner_for(&world, AiFaction::Alliance);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Alliance, |round| {
            AISystem::evaluate_rescue(
                &mut planner,
                round,
                AiFaction::Alliance,
                usize::MAX,
                &mut actions,
            );
        });

        let sent: Vec<_> = orders(&actions, MissionKind::Rescue)
            .into_iter()
            .map(|(_, system, character)| (system, character.unwrap()))
            .collect();
        assert_eq!(sent.len(), 2);
        assert!(sent.contains(&(first_prison, first)));
        assert!(sent.contains(&(second_prison, second)));
    }

    #[test]
    fn reconnaissance_scouts_only_explored_systems_the_enemy_holds() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let held = |world: &mut GameWorld, control| {
            let system = add_system(world, sector, 0.8, 0.1);
            world.systems[system].control = control;
            system
        };
        let first = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Alliance),
        );
        let second = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Alliance),
        );
        let _neutral = held(&mut world, ControlKind::Uncontrolled);
        let _own = held(
            &mut world,
            ControlKind::Controlled(crate::dat::Faction::Empire),
        );
        for _ in 0..6 {
            add_commando(&mut world, false, 80, 0);
        }
        let mut planner = planner_for(&world, AiFaction::Empire);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_reconnaissance(
                &mut planner,
                round,
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let mut targets: Vec<_> = orders(&actions, MissionKind::Espionage)
            .into_iter()
            .map(|(_, system, _)| system)
            .collect();
        targets.sort();
        let mut expected = vec![first, second];
        expected.sort();
        assert_eq!(targets, expected);
    }

    #[test]
    fn uprising_prevention_sends_diplomats_to_our_systems_below_40_percent() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let ours = |world: &mut GameWorld, support| {
            let system = add_system(world, sector, support, 0.5);
            world.systems[system].control = ControlKind::Controlled(crate::dat::Faction::Alliance);
            system
        };
        let lowest = ours(&mut world, 0.2);
        let low = ours(&mut world, 0.3);
        let _steady = ours(&mut world, 0.4);
        let _lower = ours(&mut world, 0.35);
        let enemy = add_system(&mut world, sector, 0.1, 0.5);
        world.systems[enemy].control = ControlKind::Controlled(crate::dat::Faction::Empire);
        for _ in 0..3 {
            add_character(&mut world, true, false, 90);
        }
        let mut planner = planner_for(&world, AiFaction::Alliance);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Alliance, |round| {
            AISystem::evaluate_uprising_prevention(
                &mut planner,
                round,
                &MissionState::new(),
                AiFaction::Alliance,
                usize::MAX,
                &mut actions,
            );
        });

        let targets: Vec<_> = orders(&actions, MissionKind::Diplomacy)
            .into_iter()
            .map(|(_, system, _)| system)
            .collect();
        assert_eq!(targets, vec![lowest, low]);
    }

    #[test]
    fn uprising_prevention_skips_a_system_at_exactly_40_percent() {
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let steady = add_system(&mut world, sector, 0.4, 0.5);
        world.systems[steady].control = ControlKind::Controlled(crate::dat::Faction::Alliance);
        add_character(&mut world, true, false, 90);
        let mut planner = planner_for(&world, AiFaction::Alliance);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Alliance, |round| {
            AISystem::evaluate_uprising_prevention(
                &mut planner,
                round,
                &MissionState::new(),
                AiFaction::Alliance,
                usize::MAX,
                &mut actions,
            );
        });

        assert!(orders(&actions, MissionKind::Diplomacy).is_empty());
    }

    #[test]
    fn reconnaissance_scouts_the_enemy_s_strongest_systems_first() {
        // port: recon ranks enemy-held systems by the enemy's own assets;
        // our regiments there do not count.
        let mut world = empty_world();
        let sector = add_sector(&mut world);
        let regiment = |world: &mut GameWorld, system, is_alliance| {
            let troop = world.troops.insert(crate::world::TroopUnit {
                class_dat_id: DatId(0),
                is_alliance,
                regiment_strength: 50,
            });
            world.systems[system].ground_units.push(troop);
        };
        let held = |world: &mut GameWorld| {
            let system = add_system(world, sector, 0.8, 0.1);
            world.systems[system].control = ControlKind::Controlled(crate::dat::Faction::Alliance);
            system
        };
        let garrisoned = held(&mut world);
        let empty = held(&mut world);
        let occupied = held(&mut world);
        regiment(&mut world, garrisoned, true);
        regiment(&mut world, occupied, false);
        for _ in 0..4 {
            add_commando(&mut world, false, 80, 0);
        }
        let mut planner = planner_for(&world, AiFaction::Empire);
        let mut actions = Vec::new();

        with_round(&world, AiFaction::Empire, |round| {
            AISystem::evaluate_reconnaissance(
                &mut planner,
                round,
                AiFaction::Empire,
                usize::MAX,
                &mut actions,
            );
        });

        let targets: Vec<_> = orders(&actions, MissionKind::Espionage)
            .into_iter()
            .map(|(_, system, _)| system)
            .collect();
        assert_eq!(targets, vec![garrisoned, empty]);
    }
}
