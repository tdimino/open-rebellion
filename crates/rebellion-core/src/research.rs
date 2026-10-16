//! Research: each side's points pools and levels for the three trees
//! (`ghidra/notes/research.md`).
//!
//! Points come from the Research missions (`missions.rs`: 1-7 for each
//! member whose roll succeeds) and from timer `0x381`, which adds one point
//! for each working yard. Every write spends the pool on the next research
//! order that has a class (`FUN_00567ba0`), so one large pool can buy several
//! levels. Classes with `research_order <= level` are unlocked.

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{FighterKey, SystemKey};
use crate::mission_detection::MissionRng;
use crate::tick::TickEvent;
use crate::uprising::{Draws, UprisingState};
use crate::world::GameWorld;

/// GNPRTB 7169 and 7170: timer `0x381`'s minimum delay and spread in days
/// (side record `+0xd4`, armed by `FUN_00532350`; shipped 30 and 30).
const GNPRTB_PASSIVE_MIN: u16 = 7169;
/// GNPRTB 7170, the spread (`FUN_00532350`).
const GNPRTB_PASSIVE_SPREAD: u16 = 7170;
/// `DAT_00661a88`: the divisor of the cost scale (`FUN_0053e150`).
const COST_DIVISOR: u64 = 100;

// ---------------------------------------------------------------------------
// TechType
// ---------------------------------------------------------------------------

/// The three independently-progressing tech trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechType {
    /// Ship Design Research, `0x53000020`: ship design skill.
    Ship,
    /// Troop Training Research, `0x53000022`: troop training skill.
    Troop,
    /// Facility Design Research, `0x53000021`: facility design skill.
    Facility,
}

impl TechType {
    /// The trees in the order timer `0x381` credits them (`FUN_00530460`:
    /// side `+0x90`, `+0x94`, `+0x98`).
    pub const ALL: [TechType; 3] = [TechType::Ship, TechType::Troop, TechType::Facility];

    /// The tree's slot in the side's pools, side `+0x90`, `+0x94`, `+0x98`
    /// (`FUN_00530460`).
    pub(crate) const fn index(self) -> usize {
        match self {
            TechType::Ship => 0,
            TechType::Troop => 1,
            TechType::Facility => 2,
        }
    }

    /// The skill a Research member rolls against (`FUN_005337f0`: slot
    /// `+0x1e4` ships, `+0x1e8` troops, `+0x1ec` facilities).
    #[must_use]
    pub const fn skill(self) -> crate::world::Skill {
        match self {
            TechType::Ship => crate::world::Skill::ShipDesign,
            TechType::Troop => crate::world::Skill::TroopTraining,
            TechType::Facility => crate::world::Skill::FacilityDesign,
        }
    }

    /// The producer family whose classes make up the tree and whose yards
    /// feed it: shipyards `0x28`, training facilities `0x29`, construction
    /// yards `0x2a` (`FUN_005326f0`, `FUN_005327f0`, `FUN_005328f0`; a class
    /// belongs when its producer range `+0x24` holds the family).
    #[must_use]
    pub const fn producer_family(self) -> u8 {
        match self {
            TechType::Ship => 0x28,
            TechType::Troop => 0x29,
            TechType::Facility => 0x2a,
        }
    }
}

// ---------------------------------------------------------------------------
// ResearchLevels
// ---------------------------------------------------------------------------

/// One side's research: levels (side `+0x9c`, `+0xa0`, `+0xa4`), points by
/// tree, and timer `0x381`. The exhausted flags are the world's
/// ([`GameWorld::research_exhausted`]), beside the side's other flags the
/// mission validators read.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResearchLevels {
    pub ship: u32,
    pub troop: u32,
    pub facility: u32,
    /// Side `+0x90`, `+0x94`, `+0x98`: points not yet spent, by tree in
    /// [`TechType::ALL`] order.
    pub points: [u32; 3],
    /// The tick timer `0x381` next fires. hyp: the timer is armed on the
    /// first day and first fires one delay later.
    pub passive_due: Option<u64>,
}

impl ResearchLevels {
    /// Return the current level for a specific tech tree.
    #[must_use]
    pub fn level(&self, tech: TechType) -> u32 {
        match tech {
            TechType::Ship => self.ship,
            TechType::Troop => self.troop,
            TechType::Facility => self.facility,
        }
    }

    fn level_mut(&mut self, tech: TechType) -> &mut u32 {
        match tech {
            TechType::Ship => &mut self.ship,
            TechType::Troop => &mut self.troop,
            TechType::Facility => &mut self.facility,
        }
    }

    /// The points the side holds for `tech`.
    #[must_use]
    pub fn points(&self, tech: TechType) -> u32 {
        self.points[tech.index()]
    }
}

// ---------------------------------------------------------------------------
// ResearchState
// ---------------------------------------------------------------------------

/// Persistent research state for both factions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResearchState {
    /// The Alliance's research.
    pub alliance: ResearchLevels,
    /// The Empire's research.
    pub empire: ResearchLevels,
}

impl ResearchState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// One side's research.
    #[must_use]
    pub fn side(&self, faction_is_alliance: bool) -> &ResearchLevels {
        if faction_is_alliance {
            &self.alliance
        } else {
            &self.empire
        }
    }

    fn side_mut(&mut self, faction_is_alliance: bool) -> &mut ResearchLevels {
        if faction_is_alliance {
            &mut self.alliance
        } else {
            &mut self.empire
        }
    }

    /// Return the current research level for a faction + tech tree.
    #[must_use]
    pub fn level(&self, faction_is_alliance: bool, tech: TechType) -> u32 {
        self.side(faction_is_alliance).level(tech)
    }

    /// True if `class_research_order <= current level` — i.e. the class is
    /// buildable by this faction.
    #[must_use]
    pub fn is_unlocked(
        &self,
        faction_is_alliance: bool,
        class_research_order: u32,
        tech: TechType,
    ) -> bool {
        self.level(faction_is_alliance, tech) >= class_research_order
    }
}

// ---------------------------------------------------------------------------
// ResearchResult
// ---------------------------------------------------------------------------

/// A level bought from a side's points.
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchResult {
    /// A tech tree reached a new level.
    TechUnlocked {
        faction_is_alliance: bool,
        tech_type: TechType,
        /// The new level: the order of the class it bought.
        new_level: u32,
    },
}

// ---------------------------------------------------------------------------
// ResearchSystem
// ---------------------------------------------------------------------------

/// Stateless research tick processor.
pub struct ResearchSystem;

impl ResearchSystem {
    /// Run timer `0x381` for each tick in `tick_events`. `rolls` holds one
    /// roll per tick (padded with 1.0 by the caller), which seeds that tick's
    /// draws.
    pub fn advance(
        state: &mut ResearchState,
        world: &mut GameWorld,
        uprisings: &UprisingState,
        tick_events: &[TickEvent],
        rolls: &[f64],
    ) -> Vec<ResearchResult> {
        let mut results = Vec::new();
        for (index, tick) in tick_events.iter().enumerate() {
            let roll = rolls.get(index).copied().unwrap_or(1.0);
            let mut draws = MissionRng::seeded(roll, tick.tick);
            let min = world
                .gnprtb
                .value(GNPRTB_PASSIVE_MIN, world.difficulty_index);
            let spread = world
                .gnprtb
                .value(GNPRTB_PASSIVE_SPREAD, world.difficulty_index);
            for faction_is_alliance in [true, false] {
                let mut delay = || u64::try_from(min + draws.draw(spread)).unwrap_or(0);
                let side = state.side_mut(faction_is_alliance);
                let due = *side.passive_due.get_or_insert_with(|| tick.tick + delay());
                if due > tick.tick {
                    continue;
                }
                side.passive_due = Some(tick.tick + delay());
                for tech in TechType::ALL {
                    let points = Self::working_yards(world, uprisings, faction_is_alliance, tech);
                    results.extend(Self::add_points(
                        world,
                        state,
                        faction_is_alliance,
                        tech,
                        points,
                    ));
                }
            }
        }
        results
    }

    /// Whether yards at `system` work: the system is neither in uprising
    /// nor blockaded (yard `+0x60` bit 0, which `FUN_0050bbb0` sets from
    /// system `+0x88 & 0x24`). Timer `0x381` counts only working yards, and
    /// the Research validator needs one (`FUN_00528100`).
    #[must_use]
    pub fn yards_work(world: &GameWorld, uprisings: &UprisingState, system: SystemKey) -> bool {
        world.systems.get(system).is_some_and(|sys| {
            !uprisings.is_uprising(system)
                && !crate::blockade::BlockadeSystem::system_is_blockaded(world, sys)
        })
    }

    /// The side's yards of `tech`'s producer family at `system` (the yards
    /// `FUN_0052bde0` accepts: the manager's kind, the side's).
    fn yards_at(
        world: &GameWorld,
        system: &crate::world::System,
        side: Faction,
        tech: TechType,
    ) -> usize {
        system
            .manufacturing_facilities
            .iter()
            .filter_map(|key| world.manufacturing_facilities.get(*key))
            .filter(|yard| {
                yard.side == side && yard.class_dat_id.family() == tech.producer_family()
            })
            .count()
    }

    /// Whether `system` holds a working yard of `side` for `tech`
    /// (`FUN_00509670(target, kind)`, `FUN_00528100`), the Research
    /// validator's yard test.
    #[must_use]
    pub fn has_working_yard(
        world: &GameWorld,
        uprisings: &UprisingState,
        system: SystemKey,
        side: Faction,
        tech: TechType,
    ) -> bool {
        Self::yards_work(world, uprisings, system)
            && world
                .systems
                .get(system)
                .is_some_and(|sys| Self::yards_at(world, sys, side, tech) > 0)
    }

    /// Timer `0x381`'s count for one tree (`FUN_00530460`, `FUN_00528040`):
    /// the side's working yards of the tree's producer family in the systems
    /// it holds. hyp: every system has all three production managers; every
    /// yard instance is active (yard `+0x50` bit 0).
    #[must_use]
    pub fn working_yards(
        world: &GameWorld,
        uprisings: &UprisingState,
        faction_is_alliance: bool,
        tech: TechType,
    ) -> u32 {
        let side = Faction::of_alliance(faction_is_alliance);
        let count: usize = world
            .systems
            .iter()
            .filter(|(key, system)| {
                system.control.is_controlled_by(side) && Self::yards_work(world, uprisings, *key)
            })
            .map(|(_, system)| Self::yards_at(world, system, side, tech))
            .sum();
        u32::try_from(count).unwrap_or(u32::MAX)
    }

    /// Add `points` to the side's pool for `tech` and spend it
    /// (`FUN_00530600`, then the side's slot `+0x1e0`-`+0x1e8` and
    /// `FUN_00567ba0`): while a higher order has a class and the points cover
    /// that class's scaled cost, the level becomes the order and the cost is
    /// deducted (`FUN_0052f280`, `FUN_0052f130`). With no higher class the
    /// tree is exhausted (`FUN_0052f440`).
    pub fn add_points(
        world: &mut GameWorld,
        state: &mut ResearchState,
        faction_is_alliance: bool,
        tech: TechType,
        points: u32,
    ) -> Vec<ResearchResult> {
        let mut results = Vec::new();
        let side = state.side_mut(faction_is_alliance);
        let index = tech.index();
        let level_before = side.level(tech);
        side.points[index] = side.points[index].saturating_add(points);
        crate::trace::record(crate::trace::TraceMask::RESEARCH, "points_added", || {
            serde_json::json!({
                "faction": if faction_is_alliance { "alliance" } else { "empire" },
                "tech": format!("{tech:?}"),
                "added": points,
                "pool": side.points[index],
                "level": level_before,
            })
        });
        loop {
            let Some((order, difficulty)) =
                Self::next_order(world, faction_is_alliance, tech, side.level(tech))
            else {
                crate::trace::record(crate::trace::TraceMask::RESEARCH, "tree_exhausted", || {
                    serde_json::json!({
                        "faction": if faction_is_alliance { "alliance" } else { "empire" },
                        "tech": format!("{tech:?}"),
                        "level": side.level(tech),
                    })
                });
                world.set_research_exhausted(Faction::of_alliance(faction_is_alliance), tech);
                break;
            };
            let cost = Self::scaled_cost(world, difficulty);
            crate::trace::record(crate::trace::TraceMask::RESEARCH, "level_threshold", || {
                serde_json::json!({
                    "faction": if faction_is_alliance { "alliance" } else { "empire" },
                    "tech": format!("{tech:?}"),
                    "next_order": order,
                    "difficulty": difficulty,
                    "cost": cost,
                    "pool": side.points[index],
                    "unlocks": side.points[index] >= cost,
                })
            });
            if side.points[index] < cost {
                break;
            }
            side.points[index] -= cost;
            *side.level_mut(tech) = order;
            results.push(ResearchResult::TechUnlocked {
                faction_is_alliance,
                tech_type: tech,
                new_level: order,
            });
        }
        results
    }

    /// Credit the Research missions' points among `effects`
    /// ([`crate::missions::MissionEffect::ResearchPoints`]) to their sides'
    /// pools and spend them.
    pub fn apply_mission_points(
        world: &mut GameWorld,
        state: &mut ResearchState,
        effects: &[crate::missions::MissionEffect],
    ) -> Vec<ResearchResult> {
        let mut results = Vec::new();
        for effect in effects {
            if let crate::missions::MissionEffect::ResearchPoints {
                faction,
                tech,
                points,
            } = effect
            {
                let faction_is_alliance = *faction == crate::missions::MissionFaction::Alliance;
                results.extend(Self::add_points(
                    world,
                    state,
                    faction_is_alliance,
                    *tech,
                    *points,
                ));
            }
        }
        results
    }

    /// A class's research cost in points (`FUN_0055bf80` → `FUN_0053e190`):
    /// its `research_difficulty` (class `+0x54`) times side `+0xc0` / 100,
    /// truncated. `+0xc0` is the number of systems the galaxy was created
    /// with (`FUN_005305c0` from `FUN_00519d00`): 100, 150 or 200.
    #[must_use]
    pub fn scaled_cost(world: &GameWorld, difficulty: u32) -> u32 {
        let systems = u64::try_from(world.systems.len()).unwrap_or(u64::MAX);
        u32::try_from(u64::from(difficulty) * systems / COST_DIVISOR).unwrap_or(u32::MAX)
    }

    /// The next order above `level` that has a class in the side's tree, and
    /// that class's `research_difficulty` (`FUN_00567ba0`, `FUN_00567a90`).
    /// The tree's classes are those of the producer family that serve the
    /// side and cost refined material (`FUN_00567880`, `FUN_0053b860`).
    /// hyp: among several classes at one order the first, by class id, is
    /// the one priced.
    #[must_use]
    pub fn next_order(
        world: &GameWorld,
        faction_is_alliance: bool,
        tech: TechType,
        level: u32,
    ) -> Option<(u32, u32)> {
        let serves = |alliance: bool, empire: bool| {
            if faction_is_alliance {
                alliance
            } else {
                empire
            }
        };
        // (order, class id, difficulty) of each class in the tree.
        let classes: Vec<(u32, u32, u32)> = match tech {
            TechType::Ship => world
                .capital_ship_classes
                .values()
                .filter(|c| serves(c.is_alliance, c.is_empire) && c.refined_material_cost != 0)
                .map(|c| {
                    (
                        c.research_order,
                        0x1400_0000 | c.dat_id.index(),
                        c.research_difficulty,
                    )
                })
                .chain(
                    world
                        .fighter_classes
                        .values()
                        .filter(|c| {
                            serves(c.is_alliance, c.is_empire) && c.refined_material_cost != 0
                        })
                        .map(|c| {
                            (
                                c.research_order,
                                0x1c00_0000 | c.dat_id.index(),
                                c.research_difficulty,
                            )
                        }),
                )
                .collect(),
            TechType::Troop | TechType::Facility => {
                let families: &[std::ops::RangeInclusive<u8>] = if tech == TechType::Troop {
                    &[0x10..=0x13, 0x30..=0x3f]
                } else {
                    &[0x22..=0x2d]
                };
                world
                    .buildable_classes
                    .iter()
                    .filter(|(id, c)| {
                        families.iter().any(|range| range.contains(&id.family()))
                            && serves(c.is_alliance, c.is_empire)
                            && c.refined_material_cost != 0
                    })
                    .map(|(id, c)| (c.research_order, id.raw(), c.research_difficulty))
                    .collect()
            }
        };
        classes
            .into_iter()
            .filter(|(order, _, _)| *order > level)
            .min_by_key(|(order, id, _)| (*order, *id))
            .map(|(order, _, difficulty)| (order, difficulty))
    }

    /// Whether the side has finished `tech`'s tree: its level has reached
    /// the highest research order (class `+0x24`) among the tree's classes,
    /// as `FUN_0042dd00` takes it: the side's capital ship and fighter
    /// classes for ships, its regiment classes (families `0x10`-`0x13`) for
    /// troops, and every facility class (families `0x22`-`0x2b`) for
    /// facilities.
    ///
    /// port: the original latches a bit when a level-change event
    /// (`FUN_004198a0`, `FUN_00419870`, `FUN_004197b0`) brings the level to
    /// that order; the port reads the level each time. Levels start at 0
    /// and only rise (`add_points` sets the next order's level), so a tree
    /// whose highest order is 0 is never reached by an event and stays
    /// unfinished.
    #[must_use]
    pub fn tree_exhausted(
        world: &GameWorld,
        state: &ResearchState,
        faction_is_alliance: bool,
        tech: TechType,
    ) -> bool {
        let side = |alliance: bool, empire: bool| {
            if faction_is_alliance {
                alliance
            } else {
                empire
            }
        };
        let buildable = |families: std::ops::RangeInclusive<u8>, sided: bool| {
            world
                .buildable_classes
                .iter()
                .filter(move |(id, c)| {
                    families.contains(&id.family()) && (!sided || side(c.is_alliance, c.is_empire))
                })
                .map(|(_, c)| c.research_order)
                .max()
        };
        let highest = match tech {
            TechType::Ship => world
                .capital_ship_classes
                .values()
                .filter(|c| side(c.is_alliance, c.is_empire))
                .map(|c| c.research_order)
                .chain(
                    world
                        .fighter_classes
                        .values()
                        .filter(|c| side(c.is_alliance, c.is_empire))
                        .map(|c| c.research_order),
                )
                .max(),
            TechType::Troop => buildable(0x10..=0x13, true),
            TechType::Facility => buildable(0x22..=0x2b, false),
        };
        highest.is_some_and(|order| order > 0 && state.level(faction_is_alliance, tech) >= order)
    }

    /// True if a capital ship class is available for this faction at the given
    /// research level.
    #[must_use]
    pub fn ship_class_is_available(
        world: &GameWorld,
        state: &ResearchState,
        faction_is_alliance: bool,
        class_key: crate::ids::CapitalShipKey,
    ) -> bool {
        let Some(class) = world.capital_ship_classes.get(class_key) else {
            return false;
        };
        let faction_ok = if faction_is_alliance {
            class.is_alliance
        } else {
            class.is_empire
        };
        if !faction_ok {
            return false;
        }
        state.is_unlocked(faction_is_alliance, class.research_order, TechType::Ship)
    }

    /// True if a fighter class is available at the current Ship research level.
    #[must_use]
    pub fn fighter_class_is_available(
        world: &GameWorld,
        state: &ResearchState,
        faction_is_alliance: bool,
        class_key: FighterKey,
    ) -> bool {
        let Some(class) = world.fighter_classes.get(class_key) else {
            return false;
        };
        let faction_ok = if faction_is_alliance {
            class.is_alliance
        } else {
            class.is_empire
        };
        if !faction_ok {
            return false;
        }
        // FIGHTSD.DAT carries its own research_order; fighters come from
        // shipyards, so they advance with the Ship tree (SideShipyardRdOrder).
        state.is_unlocked(faction_is_alliance, class.research_order, TechType::Ship)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{CapitalShipClass, GameWorld};

    fn add_ship_class(
        world: &mut GameWorld,
        research_order: u32,
        research_difficulty: u32,
        is_alliance: bool,
    ) {
        world.capital_ship_classes.insert(CapitalShipClass {
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
            detection: 3,
            turbolaser_fore: 2,
            turbolaser_aft: 1,
            turbolaser_port: 1,
            turbolaser_starboard: 1,
            shield_recharge_rate: 5,
            damage_control: 3,
            bombardment_modifier: 10,
            ..CapitalShipClass::default()
        });
    }

    #[test]
    fn is_unlocked_respects_current_level() {
        let mut state = ResearchState::new();

        // Level 0 — order-0 classes are available (pre-game units), order-1 are not.
        assert!(
            state.is_unlocked(true, 0, TechType::Ship),
            "order 0 always unlocked"
        );
        assert!(
            !state.is_unlocked(true, 1, TechType::Ship),
            "order 1 needs level ≥ 1"
        );

        state.alliance.ship = 1;
        assert!(
            state.is_unlocked(true, 1, TechType::Ship),
            "order 1 unlocked at level 1"
        );
        assert!(
            !state.is_unlocked(true, 2, TechType::Ship),
            "order 2 still locked"
        );
    }

    #[test]
    fn ship_class_availability_respects_research_order() {
        let mut world = GameWorld::default();
        add_ship_class(&mut world, 3, 20, true); // needs level 3

        let class_key = world.capital_ship_classes.keys().next().unwrap();
        let mut state = ResearchState::new(); // level 0

        assert!(!ResearchSystem::ship_class_is_available(
            &world, &state, true, class_key
        ));

        state.alliance.ship = 3;
        assert!(ResearchSystem::ship_class_is_available(
            &world, &state, true, class_key
        ));
    }

    #[test]
    fn a_fighter_unlocks_when_ship_research_reaches_its_order() {
        // FIGHTSD.DAT: the B-wing carries research_order 5.
        let mut world = GameWorld::default();
        let class_key = world.fighter_classes.insert(crate::world::FighterClass {
            is_alliance: true,
            research_order: 5,
            ..Default::default()
        });
        let mut state = ResearchState::new();
        state.alliance.ship = 4;
        state.alliance.troop = 5;
        state.alliance.facility = 5;

        assert!(!ResearchSystem::fighter_class_is_available(
            &world, &state, true, class_key
        ));
        assert!(!ResearchSystem::fighter_class_is_available(
            &world, &state, false, class_key
        ));

        state.alliance.ship = 5;
        assert!(ResearchSystem::fighter_class_is_available(
            &world, &state, true, class_key
        ));
    }
}
