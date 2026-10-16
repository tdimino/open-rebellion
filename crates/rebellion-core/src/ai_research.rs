//! The computer player's Research planners (`ghidra/notes/research.md`,
//! "The planner after its order").
//!
//! A planner is a persistent object in the side's planner list. It claims
//! one member and, every pass, re-emits Create Mission `0x240` for the
//! member's own system (`FUN_004bced0` states 1 to 8, `FUN_0047b360`). It
//! tests no yard: the mission validator refuses the order where the original
//! refuses it. A planner retires only through the controller's drop request
//! (`FUN_004e5710`, `FUN_004d2580`, `FUN_0042f690` set planner flag
//! `0x10000000`; state 1 runs `FUN_0047aff0` and releases the members).

use serde::{Deserialize, Serialize};

use crate::ai::{AIAction, AiFaction};
use crate::ai_production::{Round, Turn};
use crate::ids::CharacterKey;
use crate::mission_planning::{Planner, NOT_SENT, ON_MISSION};
use crate::missions::{member_location, MissionKind, MissionMember};
use crate::research::TechType;
use crate::world::GameWorld;

/// The record flags that make a planner let its member go
/// (`FUN_0047ba40`): unrecruited, injured, prisoner, on a mandatory
/// mission, an officer rank.
const RELEASED: u32 = 0x0740_0051;

/// One Research planner (class `0x53`, subtypes 1 to 3).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResearchPlanner {
    /// `+0x18`: the id the planner list assigns (`FUN_005f5440`).
    #[serde(default)]
    id: u32,
    tech: TechType,
    /// The claimed member (the team, record `+0x24`).
    member: Option<CharacterKey>,
    /// Planner flag `0x10000000`: the controller asked to drop it.
    retiring: bool,
}

/// One side's Research planners, in list order (side `+0xa8`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResearchPlanners {
    planners: Vec<ResearchPlanner>,
    /// The planner list's counter (`FUN_005f5440`); the last id given.
    #[serde(default)]
    next_id: u32,
    /// Module 4's `m+0x4c`: the planner the next turn's scan starts from.
    #[serde(default)]
    cursor: PlannerRef,
    /// The controller's pending add (side `+0x318`/`+0x31c`): a Research
    /// planner of this tree, made at module 4's next turn.
    #[serde(default)]
    pending_add: Option<TechType>,
    /// The controller's pending drop (side `+0x314`): the planner to flag,
    /// 0 none.
    #[serde(default)]
    pending_drop: u32,
}

/// port: the mission-target picks of `crate::ai` that stand in for the
/// original's other mission planners (`ghidra/notes/rebexe-map.md`, the
/// planners of class A and B), after the Research planners in module 4's
/// list. Their own planner records are not ported. The evaluators are the
/// port's sourceless target choice: the primary planner the side's
/// `+0x318`/`+0x31c` name is untraced. The structure around them, one
/// planner's pass per turn and at most one order, is `FUN_004d2490`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Pseudo {
    Officers,
    Espionage,
    Rescue,
    Reconnaissance,
    UprisingPrevention,
}

impl Pseudo {
    const ALL: [Self; 5] = [
        Self::Officers,
        Self::Espionage,
        Self::Rescue,
        Self::Reconnaissance,
        Self::UprisingPrevention,
    ];
}

/// A planner in module 4's list: a Research planner by id, or a
/// pseudo-planner.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
enum PlannerRef {
    /// The list's head (cursor 0).
    #[default]
    Head,
    Research(u32),
    Pseudo(Pseudo),
}

impl ResearchPlanners {
    /// The characters the planners claim.
    pub fn held(&self) -> impl Iterator<Item = CharacterKey> + '_ {
        self.planners.iter().filter_map(|p| p.member)
    }

    /// Module 4's turn (`FUN_004d2490`): the controller's request (state
    /// 1, `FUN_004d2580`), the next planner from the cursor, without
    /// wrapping (state 5, `FUN_004d2730`), and one pass of it (state 6,
    /// `FUN_004d27b0`). The list is the Research planners, then the
    /// pseudo-planners, whose pass `pseudo` runs over the round's census. A
    /// cursor past the list's end is 0, the head (`FUN_004d2730.c:9-12,
    /// 32-34`).
    ///
    /// Creating a planner (`FUN_0042f640`), releasing a dropped one
    /// (`FUN_0047aff0` -> `FUN_0041b530`) and a selector's claim of a member
    /// (`FUN_004021b0`) mark the round dirty (`FUN_004184f0`). A planner
    /// whose mission is in flight does not run (slot `+0x28`,
    /// `FUN_004bda40`: planner flag `0x80000000`, set by `FUN_0047adf0` while
    /// its mission exists and cleared by `FUN_0047b950` at its end). A
    /// one-member planner's order ends its pass (`FUN_004bced0.c:50-56`), so
    /// no order resumes. port: states 2 and 3 (`FUN_004d2610`,
    /// `FUN_004d2690`), the regional records' Recon, Espionage, Sabotage and
    /// Abduction planners, are traced and not ported.
    pub(crate) fn turn<'w>(
        &mut self,
        round: &mut Round<'_>,
        planner: &mut Planner<'w>,
        faction: AiFaction,
        mut pseudo: impl FnMut(Pseudo, &mut Planner<'w>, &Round<'_>) -> Option<AIAction>,
    ) -> Turn {
        let (world, uprisings) = (round.inputs.world, round.inputs.uprisings);
        planner.claim(self.held());
        // State 1 (FUN_004d2580.c:11-27): the controller's pending drop
        // flags its planner (FUN_0042f690), its pending add makes one
        // (FUN_0042f640, dirty); both are consumed.
        let drop = std::mem::take(&mut self.pending_drop);
        if let Some(dropped) = self.planners.iter_mut().find(|p| drop != 0 && p.id == drop) {
            dropped.retiring = true;
        }
        if let Some(tech) = self.pending_add.take() {
            self.next_id += 1;
            self.planners.push(ResearchPlanner {
                id: self.next_id,
                tech,
                member: None,
                retiring: false,
            });
            round.dirty = true;
        }
        let list: Vec<PlannerRef> = self
            .planners
            .iter()
            .map(|p| PlannerRef::Research(p.id))
            .chain(Pseudo::ALL.map(PlannerRef::Pseudo))
            .collect();
        // FUN_004d2730: from the cursor's planner, else the head, the first
        // that runs, without wrapping; none left resets the cursor and the
        // turn finishes (:16-34). A Research planner whose member is on a
        // mission stands for one in flight; the pseudo-planners always run.
        let runs = |entry: &PlannerRef| match entry {
            PlannerRef::Research(id) => self
                .planners
                .iter()
                .find(|p| p.id == *id)
                .and_then(|p| p.member)
                .and_then(|member| planner.record_flags(member))
                .is_none_or(|flags| flags & ON_MISSION == 0),
            PlannerRef::Pseudo(_) | PlannerRef::Head => true,
        };
        let from = list.iter().position(|&p| p == self.cursor).unwrap_or(0);
        let Some(at) = (from..list.len()).find(|&i| runs(&list[i])) else {
            self.cursor = PlannerRef::Head;
            return Turn::Finished;
        };
        self.cursor = list.get(at + 1).copied().unwrap_or_default();
        let mut actions = Vec::new();
        match list[at] {
            PlannerRef::Research(id) => {
                let Some(i) = self.planners.iter().position(|p| p.id == id) else {
                    return Turn::Finished;
                };
                if self.planners[i].retiring {
                    // State 1 of the planner (FUN_0047aff0) releases its
                    // members and leaves the list.
                    self.planners.remove(i);
                    round.dirty = true;
                } else {
                    let before = self.planners[i].member;
                    self.planners[i].step(planner, world, uprisings, faction, &mut actions);
                    let after = self.planners[i].member;
                    if after.is_some() && after != before {
                        round.dirty = true;
                    }
                }
            }
            PlannerRef::Pseudo(kind) => actions.extend(pseudo(kind, planner, round)),
            PlannerRef::Head => {}
        }
        let turn = actions
            .into_iter()
            .next()
            .map_or(Turn::Finished, |action| Turn::Order {
                action,
                resume: false,
            });
        crate::trace::record(crate::trace::TraceMask::AI, "mission_planner_turn", || {
            let order = match &turn {
                Turn::Order { action, .. } => Some(format!("{action:?}")),
                Turn::Spent | Turn::Finished => None,
            };
            serde_json::json!({
                "planner": format!("{:?}", list[at]),
                "next": format!("{:?}", self.cursor),
                "order": order,
                "dirty": round.dirty,
            })
        });
        turn
    }

    /// The controller's Research requests (`FUN_004e8290`): for ships, then
    /// troops, then facilities, add a planner while the tree's controller
    /// bit is clear and the live count is below the limit (the side's
    /// characters capable of the tree, `FUN_0041bc10`); drop one when the
    /// count exceeds the limit, or the bit is set and the count is above 0.
    /// `exhausted` is the tree's controller bit (`0x2000000` ships,
    /// `0x4000000` troops, `0x8000000` facilities).
    ///
    /// Module 13's controller makes the request once a cycle (`FUN_004e5440`
    /// -> `FUN_004e5710`); it waits as side `+0x318`/`+0x31c` and `+0x314`
    /// until module 4's next turn consumes it. Only the first add and the
    /// first drop of `FUN_004e8290`'s order go, and Recruitment's comes
    /// before Research's (`:16-23`). hyp: its test reads side `+0xa4`, which
    /// is untraced and read as 0 (as `crate::character_tasks`), and the port
    /// keeps no Recruitment planners (`c+0x320` = 0), so it never fires. A
    /// new add is stored only when there is one (`FUN_004e5710.c:24-27`).
    /// `FUN_004e5710` drops the unflagged planner of the tree with the
    /// lowest value of its slot `+0x18` (strict comparison from 2000000, so
    /// the first in list order on a tie); that slot is untraced and every
    /// planner is read as equal, so the oldest is dropped.
    pub(crate) fn request(&mut self, planner: &Planner<'_>, exhausted: [bool; 3]) {
        let mut add = None;
        let mut drop = None;
        for tech in TechType::ALL {
            let live = self.planners.iter().filter(|p| p.tech == tech).count();
            let limit = planner.capable(MissionKind::Research(tech));
            let done = exhausted[tech.index()];
            crate::trace::record(crate::trace::TraceMask::AI, "research_tree", || {
                serde_json::json!({
                    "tech": format!("{tech:?}"),
                    "exhausted": done,
                    "live": live,
                    "capable": limit,
                })
            });
            if add.is_none() && !done && live < limit {
                add = Some(tech);
            } else if drop.is_none() && (live > limit || (done && live > 0)) {
                drop = Some(tech);
            }
        }
        if let Some(tech) = drop {
            self.pending_drop = self
                .planners
                .iter()
                .find(|p| p.tech == tech && !p.retiring)
                .map_or(0, |p| p.id);
        }
        if add.is_some() {
            self.pending_add = add;
        }
    }
}

impl ResearchPlanner {
    /// One pass of `FUN_004bced0`: validate the member (state 2), select
    /// one while the team is empty (state 3), find the target (state 6)
    /// and emit the order (state 8).
    fn step(
        &mut self,
        planner: &mut Planner<'_>,
        world: &GameWorld,
        uprisings: &crate::uprising::UprisingState,
        faction: AiFaction,
        actions: &mut Vec<AIAction>,
    ) {
        let kind = MissionKind::Research(self.tech);
        // FUN_0047b110, FUN_0047ba40: a member whose record is gone or
        // carries a released flag leaves the team. A member on the mission
        // or in transit stays (its flags are not tested).
        // hyp: FUN_0047ba90's system test (+0x28 bit 2) reads as clear
        // (mission_planning.rs, `take`).
        if let Some(member) = self.member {
            if planner
                .record_flags(member)
                .is_none_or(|flags| flags & RELEASED != 0)
            {
                self.member = None;
            }
        }
        // State 3: the selector runs only while the team is empty.
        if self.member.is_none() {
            self.member = planner.pick_research_member(kind);
        }
        let Some(member) = self.member else {
            return;
        };
        // State 6, FUN_004bd0a0: the target is the member's own system, the
        // side's and not blockaded; no yard test.
        let side = faction.as_mission_faction();
        let Some(target) = member_location(world, MissionMember::Character(member)) else {
            return;
        };
        let target_ok = world.systems.get(target).is_some_and(|system| {
            system.control.is_controlled_by(side.into())
                && !crate::blockade::BlockadeSystem::system_is_blockaded(world, system)
        });
        if !target_ok {
            return;
        }
        // State 8, FUN_0047b360: the order is built every pass. A member on
        // the mission (0x80) or in transit (0x20) stays out of its team, and
        // the empty order is refused (hyp: FUN_0051ff30), so none goes out.
        if planner
            .record_flags(member)
            .is_none_or(|flags| flags & NOT_SENT != 0)
        {
            return;
        }
        // FUN_0056c510 at creation: a target with no working yard, or an
        // exhausted tree, refuses the order before any mission exists.
        if let Some(refusal) = crate::missions::research_target_refusal(
            world,
            uprisings,
            self.tech,
            side.into(),
            target,
        ) {
            crate::trace::record(crate::trace::TraceMask::AI, "research_refused", || {
                serde_json::json!({
                    "tech": format!("{:?}", self.tech),
                    "target_system": world.systems.get(target).map(|s| s.name.clone()),
                    "status": refusal.status(),
                })
            });
            return;
        }
        actions.push(AIAction::DispatchMission {
            kind,
            team: vec![MissionMember::Character(member)],
            decoys: Vec::new(),
            target_system: target,
            target_character: None,
            target_object: None,
        });
    }
}
