//! The AI mission planners' team and decoy selectors
//! (`ghidra/notes/ai-mission-planning.md`).
//!
//! The original keeps one record per character and special force
//! (`FUN_00401d20`, refreshed by `FUN_00402230`): flags at `+0x30`, the
//! missions the object may legally undertake at `+0x34`, and its eight
//! skills at `+0x40`. Each planner class queries those records
//! (`FUN_00403460`), ranks the hits, and takes the best (`FUN_004357b0`)
//! until its team, then its decoy list, is full.
//!
//! port: the original planner runs one state per tick (`FUN_004bced0`) and
//! re-enters its selectors on later cycles. The port plans in one cycle, so
//! the target system (`+0x38`, set in state 6 after the first cycle's
//! selectors) is unset while the selectors run and the nearness query
//! (`FUN_00403750`) never joins.

use std::collections::HashSet;

use crate::ids::{CharacterKey, SystemKey};
use crate::mission_detection::MissionRng;
use crate::missions::{
    member_location, member_skill, members_allowed, members_together, MissionKind, MissionMember,
};
use crate::research::TechType;
use crate::uprising::Draws;
use crate::world::{GameWorld, Skill};

// Record flags `+0x30` (`FUN_00401d20`, `FUN_00402230`).

/// `+0x50` bit 2 is clear: a character the side has not recruited.
const INACTIVE: u32 = 0x1;
/// The object is on the planner's side (`FUN_00401d20`).
const OWN_SIDE: u32 = 0x2;
/// The object is on the other side, or is a prisoner of its own
/// (`FUN_00401d20`).
const OTHER_SIDE: u32 = 0x4;
/// One of the six main characters (`FUN_004025f0`).
const MAIN_CHARACTER: u32 = 0x8;
// 0x10, injured (`+0x94` != 0): the port has no injuries.
/// The Emperor, `0x34000280`, marked in the leader branch
/// (`FUN_00401d20`).
pub(crate) const EMPEROR: u32 = 0x400;
/// En route (`+0x50` bit 4, `FUN_00402230`).
pub(crate) const EN_ROUTE: u32 = 0x20;
/// A prisoner (`+0xac` bit 0, `FUN_00401d20`).
pub(crate) const PRISONER: u32 = 0x40;
/// On a mission (role bit 7, `FUN_00402230`).
pub(crate) const ON_MISSION: u32 = 0x80;
/// On a mandatory mission (role bit 9, `FUN_00402230`).
const MANDATORY: u32 = 0x40_0000;
/// Set at construction; cleared when a planner takes the object
/// (`FUN_0047b610`, `FUN_0047b730`).
const UNASSIGNED: u32 = 0x80_0000;
/// The held officer rank's bit (`+0x96`, `FUN_00402230`): Commander
/// `0x4000000`, Admiral `0x1000000`, General `0x2000000`.
const fn officer_rank_bit(kind: crate::officer_rank::CommandKind) -> u32 {
    match kind {
        crate::officer_rank::CommandKind::None => 0,
        crate::officer_rank::CommandKind::Commander => 0x400_0000,
        crate::officer_rank::CommandKind::Admiral => 0x100_0000,
        crate::officer_rank::CommandKind::General => 0x200_0000,
    }
}
/// Reserved: a Diplomacy-capable object with diplomacy of 80 or more, or a
/// research-capable one the side needs (`FUN_00402230`).
const RESERVED: u32 = 0x800_0000;
/// A special force (DatId family `0x3c..0x3f`, `FUN_00401d20`).
const SPECIAL_FORCE: u32 = 0x1000_0000;
/// A character (`FUN_00401d20`).
const CHARACTER: u32 = 0x2000_0000;
/// One of the four leaders (`FUN_004024d0`).
const LEADER: u32 = 0x4000_0000;

/// The flags an emitted member may not carry (`FUN_0047b360`).
pub(crate) const NOT_SENT: u32 = 0x0740_00f1;
/// Any held officer rank: `0x4000000`, `0x1000000`, `0x2000000` from
/// character `+0x96` (`FUN_00402230`).
pub(crate) const RANKED: u32 = 0x0700_0000;

/// The capability bit of each of the port's mission kinds (`FUN_00402720`).
fn capability(kind: MissionKind) -> Option<u32> {
    Some(match kind {
        MissionKind::Diplomacy => 0x1,
        MissionKind::Rescue => 0x2,
        MissionKind::Sabotage => 0x4,
        MissionKind::Espionage => 0x8,
        MissionKind::Abduction => 0x10,
        MissionKind::SubdueUprising => 0x20,
        MissionKind::Assassination => 0x40,
        MissionKind::DeathStarSabotage => 0x80,
        MissionKind::InciteUprising => 0x100,
        MissionKind::Recruitment => 0x400,
        MissionKind::Research(TechType::Ship) => 0x800,
        MissionKind::Research(TechType::Troop) => 0x1000,
        MissionKind::Research(TechType::Facility) => 0x2000,
        MissionKind::Autoscrap | MissionKind::Move => return None,
    })
}

/// The port's agent kinds, in capability-bit order.
pub(crate) const KINDS: [MissionKind; 13] = [
    MissionKind::Diplomacy,
    MissionKind::Rescue,
    MissionKind::Sabotage,
    MissionKind::Espionage,
    MissionKind::Abduction,
    MissionKind::SubdueUprising,
    MissionKind::Assassination,
    MissionKind::DeathStarSabotage,
    MissionKind::InciteUprising,
    MissionKind::Recruitment,
    MissionKind::Research(TechType::Ship),
    MissionKind::Research(TechType::Troop),
    MissionKind::Research(TechType::Facility),
];

/// The three Research records (`0x53000020`, `0x53000022`, `0x53000021`;
/// capability bits `0x800`, `0x1000`, `0x2000`; `FUN_00402720`).
const RESEARCH_RECORDS: [u32; 3] = [0x5300_0020, 0x5300_0022, 0x5300_0021];

/// The six main characters (`FUN_004025f0`), as record ids.
const MAIN_CHARACTERS: [u32; 6] = [0x240, 0x241, 0x242, 0x243, 0x280, 0x281];
/// The four leaders (`FUN_004024d0`): `0x30000240`, `0x32000242`,
/// `0x34000280`, and `0x35000281`.
const LEADERS: [u32; 4] = [0x240, 0x242, 0x280, 0x281];
/// The Emperor's record id, `0x34000280` (`FUN_00401d20`).
const EMPEROR_ID: u32 = 0x280;

/// Whether `c` is one of the four leaders (`FUN_004024d0`).
pub(crate) fn is_leader(c: &crate::world::Character) -> bool {
    c.is_major && LEADERS.contains(&c.dat_id.raw())
}

/// One AI record: `+0x30`, `+0x34`, and the skills at `+0x40`.
#[derive(Debug, Clone)]
struct AgentRecord {
    member: MissionMember,
    flags: u32,
    caps: u32,
    skills: [u32; 8],
    /// port: an earlier plan in this evaluation sent the object.
    sent: bool,
}

/// One candidate query (`FUN_00403460` and `FUN_004035d0`).
#[derive(Debug, Clone, Copy)]
struct Query {
    req_flags: u32,
    req_caps: u32,
    forbid_flags: u32,
    forbid_caps: u32,
    skill: Skill,
    min: u32,
    /// `FUN_004035d0`'s record test (`FUN_004035d0.c:40`): a record passes
    /// only while unassigned (`+0x30 & 0x800000`) or claimed with a
    /// priority (`+0x2c`) above the call's 0. Every claim the port makes
    /// has priority 0, as a class A planner's (slot `+0x20` is
    /// `FUN_005a5b60`, `return 0`; `FUN_0047b610.c:42-46`) and a character
    /// task's do, so a claimed record never passes.
    unclaimed: bool,
}

/// A planner class's selectors (`FUN_0042f830`).
struct Selectors {
    team: &'static [Query],
    team_size: usize,
    /// The decoy query and its cap; `None` for a class that takes no decoys.
    decoys: Option<(Query, DecoyCap)>,
}

/// Where a decoy selector reads its cap.
#[derive(Clone, Copy)]
enum DecoyCap {
    /// `(record.+0xbc + 2) / 2` of the target system's AI record, whose
    /// `+0xbc` is the mission kind its posture recommends (`FUN_00478000`
    /// via `FUN_004b7150`), or 0.
    Posture,
    /// Planner slot `+0x44` (`FUN_004047d0`).
    Fixed(usize),
}

const fn query(
    req_flags: u32,
    req_caps: u32,
    forbid_flags: u32,
    forbid_caps: u32,
    skill: Skill,
    min: u32,
) -> Query {
    Query {
        req_flags,
        req_caps,
        forbid_flags,
        forbid_caps,
        skill,
        min,
        unclaimed: false,
    }
}

/// `q` as a `FUN_004035d0` query, which skips a claimed record.
const fn unclaimed(q: Query) -> Query {
    Query {
        unclaimed: true,
        ..q
    }
}

/// Every decoy query takes an unassigned special force of the side with
/// espionage of 51 or more (`FUN_0047d720` and siblings).
const fn decoys(req_caps: u32, cap: DecoyCap) -> Option<(Query, DecoyCap)> {
    let q = query(0x1080_0002, req_caps, 0x0740_0051, 0, Skill::Espionage, 51);
    Some((q, cap))
}

/// The selectors of `kind`'s planner (`FUN_0042f830`).
///
/// port: the AI's subtype (side `+0x31c`) is untraced. Espionage takes the
/// class for subtypes other than 6 (the posture `FUN_004b7150` names no
/// subtype for it), and Abduction takes its default class.
fn selectors(kind: MissionKind) -> Option<Selectors> {
    use Skill::{
        Combat, Diplomacy, Espionage, FacilityDesign, Leadership, ShipDesign, TroopTraining,
    };
    const DIPLOMACY: &[Query] = &[unclaimed(query(0x2, 0x1, 0x0740_4051, 0, Diplomacy, 69))];
    const ESPIONAGE: &[Query] = &[query(0x1080_0002, 0x8, 0x0740_0051, 0, Espionage, 51)];
    const RECRUITMENT: &[Query] = &[unclaimed(query(0x2, 0x400, 0x0740_0051, 0, Leadership, 80))];
    const INCITE: &[Query] = &[unclaimed(query(0x2, 0x100, 0x4f40_0051, 0, Leadership, 49))];
    const SUBDUE: &[Query] = &[unclaimed(query(0x2, 0x20, 0x0740_0051, 0, Leadership, 49))];
    const RESCUE: &[Query] = &[unclaimed(query(
        0x2,
        0x2,
        0x4f40_0051,
        0x700_0000,
        Combat,
        51,
    ))];
    const ABDUCTION: &[Query] = &[unclaimed(query(
        0x2,
        0x10,
        0x0f40_0051,
        0x700_0000,
        Combat,
        51,
    ))];
    const ASSASSINATION: &[Query] = &[query(0x1080_0002, 0x40, 0x4740_0051, 0, Combat, 51)];
    const SABOTAGE: &[Query] = &[
        query(0x80_0002, 0x4, 0x4f40_0051, 0, Combat, 51),
        query(0x80_0002, 0x4, 0x4f40_0051, 0, Espionage, 51),
    ];
    const DS_SABOTAGE: &[Query] = &[
        unclaimed(query(0x2, 0x80, 0x4740_0051, 0, Combat, 51)),
        unclaimed(query(0x2, 0x80, 0x4740_0051, 0, Espionage, 51)),
    ];
    // FUN_0047bcd0, FUN_0047c2b0, FUN_0047c900: the side's own records
    // with skill 10 or more.
    const SHIPS: &[Query] = &[query(0x2, 0x800, 0x0740_0051, 0, ShipDesign, 10)];
    const TROOPS: &[Query] = &[query(0x2, 0x1000, 0x0740_0051, 0, TroopTraining, 10)];
    const FACILITIES: &[Query] = &[query(0x2, 0x2000, 0x0740_0051, 0, FacilityDesign, 10)];
    let (team, team_size, decoys) = match kind {
        // FUN_004815a0; FUN_0047f500 takes no decoys.
        MissionKind::Diplomacy => (DIPLOMACY, 1, None),
        // FUN_004808c0, FUN_00480aa0.
        MissionKind::Espionage => (ESPIONAGE, 2, decoys(0x8, DecoyCap::Posture)),
        // FUN_0047db40; the class has no decoy selector.
        MissionKind::Recruitment => (RECRUITMENT, 1, None),
        // FUN_0047ff90, FUN_00480110.
        MissionKind::InciteUprising => (INCITE, 1, decoys(0x100, DecoyCap::Posture)),
        // FUN_0047cee0; no decoy selector.
        MissionKind::SubdueUprising => (SUBDUE, 1, None),
        // FUN_0047e270, FUN_0047e3f0 (cap slot +0x44 = 4).
        MissionKind::Rescue => (RESCUE, 2, decoys(0x2, DecoyCap::Fixed(4))),
        // FUN_00483290, FUN_00483410 (cap 4).
        MissionKind::Abduction => (ABDUCTION, 2, decoys(0x10, DecoyCap::Fixed(4))),
        // FUN_00482ce0, FUN_00482ec0 with +0x7c = 0x40.
        MissionKind::Assassination => (ASSASSINATION, 2, decoys(0x40, DecoyCap::Posture)),
        // FUN_0047d520, FUN_0047d720.
        MissionKind::Sabotage => (SABOTAGE, 2, decoys(0x4, DecoyCap::Posture)),
        // FUN_00482180, FUN_00482330 (cap 4).
        MissionKind::DeathStarSabotage => (DS_SABOTAGE, 2, decoys(0x80, DecoyCap::Fixed(4))),
        // The Research selectors stop after one member; no decoys.
        MissionKind::Research(TechType::Ship) => (SHIPS, 1, None),
        MissionKind::Research(TechType::Troop) => (TROOPS, 1, None),
        MissionKind::Research(TechType::Facility) => (FACILITIES, 1, None),
        MissionKind::Autoscrap | MissionKind::Move => return None,
    };
    Some(Selectors {
        team,
        team_size,
        decoys,
    })
}

/// One side's planners for one AI evaluation.
pub(crate) struct Planner<'a> {
    pub(crate) world: &'a GameWorld,
    records: Vec<AgentRecord>,
    rng: MissionRng,
}

impl<'a> Planner<'a> {
    /// The side's records as `FUN_00402230` would refresh them now.
    /// `exhausted` says which research trees, in the order of
    /// [`RESEARCH_RECORDS`], the side has finished
    /// ([`crate::research::ResearchSystem::tree_exhausted`]).
    ///
    /// port: the tie draws come from a stream seeded by the day and side;
    /// the original draws from its global generator (`FUN_0041cd80`).
    pub(crate) fn new(
        world: &'a GameWorld,
        busy: &HashSet<CharacterKey>,
        alliance: bool,
        tick: u64,
        exhausted: [bool; 3],
    ) -> Self {
        Self {
            world,
            records: records(world, busy, alliance, exhausted),
            rng: MissionRng::seeded(0.0, tick.wrapping_mul(2) + u64::from(alliance)),
        }
    }

    /// Fill `kind`'s team, then its decoys, as its planner's selectors do,
    /// and keep the members the order takes (`FUN_0047b360`). `None` when
    /// no team member is sent.
    ///
    /// port: a member taken by an earlier plan in this evaluation stays
    /// taken; the original planners run on separate ticks, by which time
    /// the first order has set its members on the mission.
    pub(crate) fn plan(
        &mut self,
        kind: MissionKind,
    ) -> Option<(Vec<MissionMember>, Vec<MissionMember>)> {
        self.plan_with(kind, None, None)
    }

    /// A regional Sabotage planner (`FUN_004bda60`): its staging system
    /// (`FUN_004bdcd0`, planner `+0x38`) is set before the team and decoys
    /// are picked, so the nearness query (`FUN_00403750`) joins both
    /// selections, and the decoy cap is `(sys+0xbc + 2) / 2` of the target
    /// system's record (`FUN_0047d720.c:36-53`).
    pub(crate) fn plan_sabotage(
        &mut self,
        stack: i64,
        staging: Option<SystemKey>,
    ) -> Option<(Vec<MissionMember>, Vec<MissionMember>)> {
        self.plan_with(MissionKind::Sabotage, Some(stack), staging)
    }

    fn plan_with(
        &mut self,
        kind: MissionKind,
        stack: Option<i64>,
        staging: Option<SystemKey>,
    ) -> Option<(Vec<MissionMember>, Vec<MissionMember>)> {
        let selectors = selectors(kind)?;
        let team = self.select_near(selectors.team, staging, selectors.team_size);
        // port: a plan that sends no one keeps no one. The original planner
        // holds its picks (they lost 0x800000) across ticks until it emits.
        if !team
            .iter()
            .any(|&index| self.records[index].flags & NOT_SENT == 0)
        {
            for index in team {
                self.records[index].flags |= UNASSIGNED;
            }
            return None;
        }
        let decoys = match selectors.decoys {
            // hyp: a planner with no target record reads +0xbc as 0, which
            // allows one decoy.
            Some((decoy, DecoyCap::Posture)) => {
                let cap = usize::try_from((stack.unwrap_or(0) + 2) / 2).unwrap_or(0);
                self.select_near(&[decoy], staging, cap)
            }
            Some((decoy, DecoyCap::Fixed(cap))) => self.select_near(&[decoy], staging, cap),
            None => Vec::new(),
        };
        let sent = |picked: Vec<usize>, records: &[AgentRecord]| -> Vec<MissionMember> {
            picked
                .into_iter()
                .filter(|&index| records[index].flags & NOT_SENT == 0)
                .map(|index| records[index].member)
                .collect()
        };
        // port: the original planner moves every member to the target
        // (state 7, FUN_004bd570) and emits the order once all are there,
        // so FUN_0054c200 finds them together. The port does not gather
        // them; the order keeps the members standing with its first.
        let team = sent(team, &self.records);
        let lead = team[0];
        let together = |members: Vec<MissionMember>| -> Vec<MissionMember> {
            members
                .into_iter()
                .filter(|&member| members_together(self.world, lead, member))
                .collect()
        };
        let team = together(team);
        let decoys = together(sent(decoys, &self.records));
        for record in &mut self.records {
            if team.contains(&record.member) || decoys.contains(&record.member) {
                record.sent = true;
            }
        }
        Some((team, decoys))
    }

    /// The member a Research planner's selector takes while its team is
    /// empty (`FUN_0047bcd0`, `FUN_0047c2b0`, `FUN_0047c900`): one record,
    /// claimed. `None` for any other kind, or when no record qualifies.
    ///
    /// port: the selector queries do not test `UNASSIGNED`; a member that
    /// another planner or a task holds is skipped here, as the claim test
    /// of `FUN_0047ba40` (`+0x24` is this planner's id) skips it.
    pub(crate) fn pick_research_member(&mut self, kind: MissionKind) -> Option<CharacterKey> {
        let MissionKind::Research(_) = kind else {
            return None;
        };
        let selectors = selectors(kind)?;
        let queries: Vec<Query> = selectors
            .team
            .iter()
            .map(|q| Query {
                req_flags: q.req_flags | UNASSIGNED,
                ..*q
            })
            .collect();
        let taken = self.select(&queries, 1);
        taken
            .first()
            .and_then(|&index| self.records[index].member.character())
    }

    /// The number of the side's characters capable of `kind`
    /// (`FUN_0041bc10`), the limit on its live Research planners.
    pub(crate) fn capable(&self, kind: MissionKind) -> usize {
        let Some(bit) = capability(kind) else {
            return 0;
        };
        self.records
            .iter()
            .filter(|r| {
                r.flags & OWN_SIDE != 0 && r.caps & bit != 0 && r.member.character().is_some()
            })
            .count()
    }

    /// The `+0x30` flags of `character`'s record; `None` when it has none.
    pub(crate) fn record_flags(&self, character: CharacterKey) -> Option<u32> {
        self.records
            .iter()
            .find(|r| r.member == MissionMember::Character(character))
            .map(|r| r.flags)
    }

    /// Characters held by the AI's character tasks: a task's claim clears
    /// the record's `0x800000` (`FUN_0047a5b0`, `FUN_00479c00`).
    pub(crate) fn claim(&mut self, held: impl IntoIterator<Item = CharacterKey>) {
        for character in held {
            for record in &mut self.records {
                if record.member == MissionMember::Character(character) {
                    record.flags &= !UNASSIGNED;
                }
            }
        }
    }

    /// The General a kind 1 task takes for `target` (`FUN_0047a5b0`): the
    /// best Leadership of 50 or more (`FUN_00403460`) combined with the
    /// nearest to the target (`FUN_00403750`).
    pub(crate) fn take_general(&mut self, target: SystemKey) -> Option<CharacterKey> {
        const GENERAL: Query = query(0x80_0002, 0x400_0000, 0x4f40_00f1, 0, Skill::Leadership, 50);
        let best = ranked(&self.records, GENERAL, &mut self.rng);
        let near = self.nearness(GENERAL, target);
        self.take(vec![best, near])
    }

    /// The officer a class I task takes (`FUN_004bc6d0`, `FUN_0047a050`):
    /// the capability `caps`, Leadership of 50 or more (`FUN_00403460`),
    /// combined with the nearest to `staging` (`FUN_00403750`). A General
    /// may be reserved (exclusion `0x474000f1`, else `0x4f4000f1`).
    pub(crate) fn take_officer(&mut self, caps: u32, staging: SystemKey) -> Option<CharacterKey> {
        let forbid = if caps == 0x400_0000 {
            0x4740_00f1
        } else {
            0x4f40_00f1
        };
        let q = query(0x80_0002, caps, forbid, 0, Skill::Leadership, 50);
        let best = ranked(&self.records, q, &mut self.rng);
        let near = self.nearness(q, staging);
        self.take(vec![best, near])
    }

    /// The Emperor a kind 6 task takes (`FUN_00479c00`): one query, flags
    /// `0x402` with none of `0x74000f1`, any Leadership.
    pub(crate) fn take_emperor(&mut self) -> Option<CharacterKey> {
        const EMPEROR_QUERY: Query = query(0x402, 0, 0x0740_00f1, 0, Skill::Leadership, 0);
        let only = ranked(&self.records, EMPEROR_QUERY, &mut self.rng);
        self.take(vec![only])
    }

    /// Take the best common hit (`FUN_004357b0`) and claim it.
    /// `FUN_004bc180` refuses a character standing in a system whose
    /// record `+0x28` has bit 2. hyp: that bit is untraced and read as
    /// clear, so the first pick is accepted.
    fn take(&mut self, mut lists: Vec<Vec<usize>>) -> Option<CharacterKey> {
        let best = pop_best(&mut lists)?;
        self.records[best].flags &= !UNASSIGNED;
        self.records[best].member.character()
    }

    /// `FUN_00403750`: the records that pass `q`'s masks, at a system,
    /// keyed by the squared distance (`FUN_0041b7b0`) from that system to
    /// `target`, the farthest first (order 2). hyp: the system records'
    /// `+0x34` halves are read as the map position.
    fn nearness(&mut self, q: Query, target: SystemKey) -> Vec<usize> {
        let world = self.world;
        let position = |system: SystemKey| {
            world
                .systems
                .get(system)
                .map(|s| (i64::from(s.x), i64::from(s.y)))
        };
        let Some((tx, ty)) = position(target) else {
            return Vec::new();
        };
        let distance: Vec<Option<i64>> = self
            .records
            .iter()
            .map(|record| {
                let (x, y) = position(member_location(world, record.member)?)?;
                Some((x - tx).pow(2) + (y - ty).pow(2))
            })
            .collect();
        let mut list = Vec::new();
        for (index, record) in self.records.iter().enumerate() {
            if passes(record, q) && distance[index].is_some() {
                let value_of = |i: usize| distance[i].unwrap_or(0);
                insert_sorted(&mut list, index, value_of, true, &mut self.rng);
            }
        }
        list
    }

    /// [`Self::select`], with the nearness query to `staging` joining the
    /// lists when the planner has one (`FUN_00403750`).
    fn select_near(&mut self, queries: &[Query], staging: Option<SystemKey>, size: usize) -> Vec<usize> {
        let near = match (staging, queries.first()) {
            (Some(staging), Some(&q)) => Some(self.nearness(q, staging)),
            _ => None,
        };
        self.select_lists(queries, near, size)
    }

    /// Run `queries` and take their best common hit until `size` are taken
    /// (`FUN_004357b0` in the selector loop). A taken record loses
    /// `UNASSIGNED`.
    fn select(&mut self, queries: &[Query], size: usize) -> Vec<usize> {
        self.select_lists(queries, None, size)
    }

    fn select_lists(&mut self, queries: &[Query], near: Option<Vec<usize>>, size: usize) -> Vec<usize> {
        let mut lists: Vec<Vec<usize>> = queries
            .iter()
            .map(|q| ranked(&self.records, *q, &mut self.rng))
            .collect();
        lists.extend(near);
        let mut taken = Vec::new();
        while taken.len() < size {
            let Some(best) = pop_best(&mut lists) else {
                break;
            };
            // FUN_0047ba90. port: the target system's record +0x28 bit 1 is
            // untraced and read as clear, so every candidate is eligible.
            self.records[best].flags &= !UNASSIGNED;
            taken.push(best);
        }
        taken
    }
}

/// `FUN_00403460`: the records that pass `q`, in the order of the sorted
/// container. Order flag 1 inserts each hit before the first entry with a
/// higher skill; an equal skill goes before it on a coin flip
/// (`FUN_0041c250`, `FUN_0041cd80(10) < 5`).
fn ranked(records: &[AgentRecord], q: Query, rng: &mut MissionRng) -> Vec<usize> {
    let mut list: Vec<usize> = Vec::new();
    let skill = |index: usize| i64::from(records[index].skills[q.skill as usize]);
    for (index, record) in records.iter().enumerate() {
        let hit = passes(record, q) && (i64::from(q.min)..=10_000).contains(&skill(index));
        if hit {
            insert_sorted(&mut list, index, skill, false, rng);
        }
    }
    list
}

/// The flag and capability tests every query applies (`FUN_00403460`,
/// `FUN_00403750`).
fn passes(record: &AgentRecord, q: Query) -> bool {
    !record.sent
        && (!q.unclaimed || record.flags & UNASSIGNED != 0)
        && record.flags & q.req_flags == q.req_flags
        && record.flags & q.forbid_flags == 0
        && record.caps & q.req_caps == q.req_caps
        && record.caps & q.forbid_caps == 0
}

/// `FUN_005f59f0` walks from the head and inserts before the first entry
/// the comparator `FUN_0041c250` puts the new one ahead of: order 1 puts
/// the lower value first, order 2 (`descending`) the higher.
fn insert_sorted(
    list: &mut Vec<usize>,
    index: usize,
    value_of: impl Fn(usize) -> i64,
    descending: bool,
    rng: &mut MissionRng,
) {
    let value = value_of(index);
    let at = list
        .iter()
        .position(|&other| match value.cmp(&value_of(other)) {
            std::cmp::Ordering::Less => !descending,
            std::cmp::Ordering::Greater => descending,
            std::cmp::Ordering::Equal => rng.draw(9) < 5,
        })
        .unwrap_or(list.len());
    list.insert(at, index);
}

/// `FUN_004357b0`: each entry of the first list scores the sum of its
/// positions in every list; an entry missing from any list leaves the first.
/// The highest score wins, the earliest on a tie, and leaves the first list.
///
/// `FUN_0041c230` numbers positions from 1. Every remaining entry is in
/// every list, so counting from 0 shifts all scores alike.
fn pop_best(lists: &mut [Vec<usize>]) -> Option<usize> {
    let (first, rest) = lists.split_first_mut()?;
    let position = |list: &[usize], index: usize| list.iter().position(|&x| x == index);
    first.retain(|&index| rest.iter().all(|list| list.contains(&index)));
    let mut best: Option<(usize, usize)> = None;
    for (at, &index) in first.iter().enumerate() {
        let score = at
            + rest
                .iter()
                .filter_map(|list| position(list, index))
                .sum::<usize>();
        if best.is_none_or(|(_, top)| top < score) {
            best = Some((index, score));
        }
    }
    let (index, _) = best?;
    first.retain(|&x| x != index);
    Some(index)
}

/// The records of every character and special force for the side
/// (`FUN_00401d20` then `FUN_00402230`).
///
/// port: the pool's order is the original's DatId order for characters
/// (majors, then minors, by id); special forces, which the port gives no
/// DatId, follow in arena order.
fn records(
    world: &GameWorld,
    busy: &HashSet<CharacterKey>,
    alliance: bool,
    exhausted: [bool; 3],
) -> Vec<AgentRecord> {
    let mut characters: Vec<_> = world
        .characters
        .iter()
        .filter(|(_, c)| !c.is_killed)
        .collect();
    characters.sort_by_key(|(_, c)| (!c.is_major, c.dat_id.raw()));
    let mut records = Vec::new();
    for (key, c) in characters {
        let member = MissionMember::Character(key);
        let id = c.dat_id.raw();
        let mut flags = UNASSIGNED | CHARACTER;
        // FUN_00401d20: a prisoner counts on the side that holds it.
        flags |= if (c.is_alliance == alliance) != c.is_captive {
            OWN_SIDE
        } else {
            OTHER_SIDE
        };
        let mut caps = 0;
        if c.is_major && MAIN_CHARACTERS.contains(&id) {
            flags |= MAIN_CHARACTER;
        } else {
            // FUN_00401d20: capability bits 0x1000000..0x4000000 mark a
            // minor character that may be an admiral, commander, or general
            // (class record +0xa8, +0xac, +0xb0; FUN_004ed1c0..FUN_004ed200).
            for (can, bit) in [
                (c.can_be_admiral, 0x100_0000),
                (c.can_be_commander, 0x200_0000),
                (c.can_be_general, 0x400_0000),
            ] {
                if can {
                    caps |= bit;
                }
            }
        }
        // FUN_00401d20: the leader bit is set while the side controller's
        // +4 bit 0 is clear. hyp: that bit's only writer (FUN_004182a0, a
        // game setting read through FUN_004fce50) is untraced; it is read
        // as clear, its constructor value.
        if c.is_major && LEADERS.contains(&id) {
            flags |= LEADER;
            if id == EMPEROR_ID {
                flags |= EMPEROR;
            }
        }
        if c.is_captive {
            flags |= PRISONER;
        }
        if !c.recruited {
            flags |= INACTIVE;
        }
        // port: the AI's busy set holds characters its applied actions sent
        // on missions or research.
        if c.on_mission || c.on_hidden_mission || busy.contains(&key) {
            flags |= ON_MISSION;
        }
        if c.on_mandatory_mission {
            flags |= MANDATORY;
        }
        if c.en_route {
            flags |= EN_ROUTE;
        }
        flags |= officer_rank_bit(c.command_kind);
        records.push(agent_record(world, member, flags, caps, exhausted));
    }
    for (key, unit) in &world.special_forces {
        let member = MissionMember::SpecialForce(key);
        let mut flags = UNASSIGNED | SPECIAL_FORCE;
        flags |= if unit.is_alliance == alliance {
            OWN_SIDE
        } else {
            OTHER_SIDE
        };
        if unit.on_mission {
            flags |= ON_MISSION;
        }
        if unit.en_route {
            flags |= EN_ROUTE;
        }
        records.push(agent_record(world, member, flags, 0, exhausted));
    }
    records
}

/// Fill in the skills (slots `+0x1dc..+0x1f8`), the mission capabilities
/// (`FUN_00402720`), and the reserved bit.
fn agent_record(
    world: &GameWorld,
    member: MissionMember,
    flags: u32,
    officer_caps: u32,
    exhausted: [bool; 3],
) -> AgentRecord {
    let mut skills = [0; 8];
    for skill in Skill::ALL {
        skills[skill as usize] = member_skill(world, member, skill).unwrap_or(0);
    }
    let mut caps = officer_caps;
    for kind in KINDS {
        let Some(bit) = capability(kind) else {
            continue;
        };
        // port: a kind with no MISSNSD record admits any members, as at
        // creation. FUN_005420d0 also runs the class validator's creation
        // checks (FUN_0054c440), so only a major gets Recruitment.
        let legal = kind
            .record_id()
            .and_then(|id| world.mission_record(id))
            .is_none_or(|record| members_allowed(world, record.members, [member]))
            && crate::missions::creation_refusal(world, kind, [member]).is_none();
        if legal {
            caps |= bit;
        }
    }
    let mut flags = flags;
    // FUN_00402230: an object legal for a Research record is reserved while
    // the side controller's matching +4 bit (0x2000000, 0x4000000,
    // 0x8000000) is clear. FUN_004198a0, FUN_00419870 and FUN_004197b0 set
    // a bit when the tree's level reaches its highest order, and nothing
    // clears it. port: with no Research record there is no research to
    // reserve for.
    let research = RESEARCH_RECORDS.iter().zip(exhausted).any(|(&raw, done)| {
        !done
            && world
                .mission_record(crate::ids::DatId::new(raw))
                .is_some_and(|record| members_allowed(world, record.members, [member]))
    });
    // FUN_00402230: a Diplomacy-capable object with diplomacy above 79.
    if research || (caps & 0x1 != 0 && skills[Skill::Diplomacy as usize] > 79) {
        flags |= RESERVED;
    }
    AgentRecord {
        member,
        flags,
        caps,
        skills,
        sent: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::SystemKey;
    use crate::world::{Character, MissionMemberRules, MissionRecord, SkillPair, SpecialForceUnit};

    fn pair(base: u32) -> SkillPair {
        SkillPair { base, variance: 0 }
    }

    /// A world with no MISSNSD records: every member is legal, and nobody is
    /// reserved for research.
    fn world() -> GameWorld {
        GameWorld::default()
    }

    /// The shipped Research records: characters of both sides, no special
    /// forces.
    fn research_records() -> Vec<MissionRecord> {
        RESEARCH_RECORDS
            .iter()
            .map(|&raw| MissionRecord {
                dat_id: crate::ids::DatId::new(raw),
                timer_min_days: 10,
                timer_spread_days: 10,
                repeats: true,
                hidden: false,
                detection_phases: false,
                can_resign: false,
                return_policy: 1,
                rules: crate::world::MissionTargetRules::default(),
                members: MissionMemberRules {
                    alliance: true,
                    empire: true,
                    special_force_mask: 0,
                    character_mask: 0x1_0000,
                },
            })
            .collect()
    }

    /// A recruited Alliance minor character.
    fn agent(
        world: &mut GameWorld,
        dat_id: u32,
        edit: impl FnOnce(&mut Character),
    ) -> MissionMember {
        let mut c = Character {
            dat_id: crate::ids::DatId::new(dat_id),
            name: "Agent".into(),
            is_alliance: true,
            recruited: true,
            ..Default::default()
        };
        edit(&mut c);
        MissionMember::Character(world.characters.insert(c))
    }

    fn unit(world: &mut GameWorld, espionage: u32, combat: u32) -> MissionMember {
        let mut skills = [0; 8];
        skills[Skill::Espionage as usize] = espionage;
        skills[Skill::Combat as usize] = combat;
        MissionMember::SpecialForce(world.special_forces.insert(SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x3c00_0002),
            is_alliance: true,
            skills,
            on_mission: false,
            en_route: false,
            on_hidden_mission: false,
        }))
    }

    fn planner(world: &GameWorld) -> Planner<'_> {
        Planner::new(world, &HashSet::new(), true, 1, [false; 3])
    }

    #[test]
    fn a_diplomat_is_the_one_with_the_highest_diplomacy() {
        // FUN_00403460 sorts ascending (order 1, FUN_0041c250) and
        // FUN_004357b0 takes the highest position.
        let mut world = world();
        let _ = agent(&mut world, 1, |c| c.diplomacy = pair(70));
        let best = agent(&mut world, 2, |c| c.diplomacy = pair(79));
        let _ = agent(&mut world, 3, |c| c.diplomacy = pair(75));

        let plan = planner(&world).plan(MissionKind::Diplomacy);

        assert_eq!(plan, Some((vec![best], vec![])));
    }

    #[test]
    fn a_candidate_missing_from_any_query_is_dropped_and_positions_add_up() {
        // FUN_004357b0 sums each first-list entry's positions in every list
        // and drops entries some list lacks. Combat ranks b 1, c 2, a 3 and
        // espionage a 1, c 2, b 3, so all three score 4; the earliest in the
        // combat list, b, wins, then c. d lacks espionage 51.
        let mut world = world();
        let a = unit(&mut world, 60, 90);
        let b = unit(&mut world, 90, 60);
        let c = unit(&mut world, 80, 80);
        let _d = unit(&mut world, 50, 99);

        let (team, _) = planner(&world).plan(MissionKind::Sabotage).unwrap();

        assert_eq!(team, vec![b, c]);
        assert!(!team.contains(&a));
    }

    #[test]
    fn sabotage_decoys_are_special_forces_left_after_the_team() {
        // FUN_0047d720 queries 0x10800002: special forces still unassigned
        // (a team member lost 0x800000 in FUN_0047b610). hyp: a planner
        // with no target record reads sys+0xbc as 0, one decoy.
        let mut world = world();
        let first = unit(&mut world, 90, 90);
        let second = unit(&mut world, 80, 80);
        let third = unit(&mut world, 70, 70);
        let _ = unit(&mut world, 60, 60);

        let plan = planner(&world).plan(MissionKind::Sabotage);

        assert_eq!(plan, Some((vec![first, second], vec![third])));
    }

    #[test]
    fn a_sabotage_takes_half_its_targets_defence_plus_one_in_decoys() {
        // FUN_0047d720.c:36-53 caps the decoys at (sys+0xbc + 2) / 2 of
        // the target system's record.
        let mut world = world();
        for skill in (60..=99).rev().step_by(5) {
            let _ = unit(&mut world, skill, skill);
        }

        for (stack, cap) in [(0, 1), (1, 1), (4, 3), (9, 5)] {
            let (team, decoys) = planner(&world).plan_sabotage(stack, None).unwrap();

            assert_eq!(team.len(), 2);
            assert_eq!(decoys.len(), cap, "stack {stack}");
        }
    }

    #[test]
    fn a_sabotage_planner_takes_agents_near_its_staging_system_first() {
        // FUN_004bdcd0 sets the staging system before FUN_0047d520 picks
        // the team, so the nearness query (FUN_00403750, smallest squared
        // distance last) joins the combat and espionage lists. All three
        // tie without it and the first in the combat list, the weakest
        // fighter, leads; nearness lifts the agent at the staging system.
        let mut world = world();
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId::new(0),
            name: "Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let at = |world: &mut GameWorld, x| {
            world.systems.insert(crate::world::System {
                sector,
                x,
                ..Default::default()
            })
        };
        let staging = at(&mut world, 0);
        let far = at(&mut world, 90);
        let placed = |world: &mut GameWorld, system, espionage, combat| {
            let member = unit(world, espionage, combat);
            if let MissionMember::SpecialForce(key) = member {
                world.systems[system].special_forces.push(key);
            }
            member
        };
        let near = placed(&mut world, staging, 60, 99);
        let weakest = placed(&mut world, far, 99, 60);
        let _ = placed(&mut world, far, 80, 80);

        let lead = |staging| planner(&world).plan_sabotage(0, staging).unwrap().0[0];

        assert_eq!(lead(None), weakest);
        assert_eq!(lead(Some(staging)), near);
    }

    #[test]
    fn a_rescue_takes_up_to_four_decoys() {
        // FUN_0047e3f0 caps the decoys at slot +0x44, FUN_004047d0 = 4.
        let mut world = world();
        let units: Vec<_> = (0..7).map(|i| unit(&mut world, 90 - i, 90 - i)).collect();

        let (team, decoys) = planner(&world).plan(MissionKind::Rescue).unwrap();

        assert_eq!(team, units[..2].to_vec());
        assert_eq!(decoys, units[2..6].to_vec());
    }

    #[test]
    fn a_best_candidate_already_on_a_mission_is_picked_but_not_sent() {
        // The Diplomacy query does not forbid 0x80, but FUN_0047b360 sends
        // no member with any of 0x74000f1, so the order has no team.
        let mut world = world();
        let _ = agent(&mut world, 1, |c| {
            c.diplomacy = pair(90);
            c.on_mission = true;
        });
        let _ = agent(&mut world, 2, |c| c.diplomacy = pair(70));

        assert_eq!(planner(&world).plan(MissionKind::Diplomacy), None);
    }

    #[test]
    fn a_character_the_side_has_not_recruited_is_never_picked() {
        // FUN_00402230 sets 0x1 while +0x50 bit 2 is clear; every query
        // forbids it.
        let mut world = world();
        let _ = agent(&mut world, 1, |c| {
            c.diplomacy = pair(90);
            c.recruited = false;
        });

        assert_eq!(planner(&world).plan(MissionKind::Diplomacy), None);
    }

    #[test]
    fn a_minor_character_fit_to_command_is_kept_off_rescues() {
        // FUN_0047e270 forbids capability bits 0x7000000, which
        // FUN_00401d20 sets for a minor character that may be an admiral,
        // commander, or general.
        let mut world = world();
        let _ = agent(&mut world, 0x300, |c| {
            c.combat = pair(90);
            c.can_be_general = true;
        });
        let fighter = agent(&mut world, 0x301, |c| c.combat = pair(60));

        let plan = planner(&world).plan(MissionKind::Rescue);

        assert_eq!(plan, Some((vec![fighter], vec![])));
    }

    #[test]
    fn a_character_free_for_research_is_reserved_from_sabotage() {
        // FUN_00402230 reserves an object legal for a Research record while
        // the side's research bit is clear. hyp: the bit is read as clear.
        let mut world = world();
        let _ = agent(&mut world, 1, |c| {
            c.combat = pair(90);
            c.espionage = pair(90);
        });
        assert!(planner(&world).plan(MissionKind::Sabotage).is_some());

        world.mission_records = research_records();

        assert_eq!(planner(&world).plan(MissionKind::Sabotage), None);
    }

    #[test]
    fn an_order_keeps_only_the_members_standing_with_its_first() {
        // FUN_0054c200 refuses a member away from the first free one. port:
        // the planner's gathering move (state 7) is not ported, so the
        // second pick, elsewhere, is left out.
        let mut world = world();
        let mut places: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let (here, there) = (places.insert(()), places.insert(()));
        let first = agent(&mut world, 1, |c| {
            c.combat = pair(90);
            c.current_system = Some(here);
        });
        let _away = agent(&mut world, 2, |c| {
            c.combat = pair(80);
            c.current_system = Some(there);
        });
        let _third = agent(&mut world, 3, |c| {
            c.combat = pair(70);
            c.current_system = Some(here);
        });

        let plan = planner(&world).plan(MissionKind::Rescue);

        assert_eq!(plan, Some((vec![first], vec![])));
    }

    #[test]
    fn a_busy_best_candidate_still_takes_a_team_slot() {
        // No query forbids 0x80, so the Recruitment planner picks the busy
        // leader and FUN_0047b360 drops him; the failed plan frees him, and
        // the DS Sabotage planner picks him again before the next best.
        let mut world = world();
        let skilled = |c: &mut Character| {
            c.combat = pair(95);
            c.espionage = pair(95);
            c.leadership = pair(90);
            c.on_mission = true;
        };
        let _busy = agent(&mut world, 1, skilled);
        let second = agent(&mut world, 2, |c| {
            c.combat = pair(80);
            c.espionage = pair(80);
        });
        let _third = agent(&mut world, 3, |c| {
            c.combat = pair(70);
            c.espionage = pair(70);
        });
        let mut planner = planner(&world);

        assert_eq!(planner.plan(MissionKind::Recruitment), None);
        assert_eq!(
            planner.plan(MissionKind::DeathStarSabotage),
            Some((vec![second], vec![]))
        );
    }

    #[test]
    fn a_character_en_route_takes_a_team_slot_but_is_not_sent() {
        // 0x20 (FUN_00402230) is in no query's forbid mask, only in
        // FUN_0047b360's.
        let mut world = world();
        let travelling = agent(&mut world, 1, |c| {
            c.combat = pair(95);
            c.espionage = pair(95);
        });
        let second = agent(&mut world, 2, |c| {
            c.combat = pair(80);
            c.espionage = pair(80);
        });
        let _third = agent(&mut world, 3, |c| {
            c.combat = pair(70);
            c.espionage = pair(70);
        });
        if let MissionMember::Character(key) = travelling {
            world.characters[key].en_route = true;
        }
        let mut planner = Planner::new(&world, &HashSet::new(), true, 1, [false; 3]);

        assert_eq!(
            planner.plan(MissionKind::DeathStarSabotage),
            Some((vec![second], vec![]))
        );
    }

    #[test]
    fn a_busy_or_travelling_special_force_takes_a_slot_but_is_not_sent() {
        let mut world = world();
        let busy = unit(&mut world, 95, 95);
        if let MissionMember::SpecialForce(key) = busy {
            world.special_forces[key].on_mission = true;
        }
        let second = unit(&mut world, 80, 80);
        let third = unit(&mut world, 70, 70);

        assert_eq!(
            planner(&world).plan(MissionKind::Sabotage),
            Some((vec![second], vec![third]))
        );

        if let MissionMember::SpecialForce(key) = busy {
            world.special_forces[key].on_mission = false;
        }
        if let MissionMember::SpecialForce(key) = busy {
            world.special_forces[key].en_route = true;
        }
        let mut planner = Planner::new(&world, &HashSet::new(), true, 1, [false; 3]);

        assert_eq!(
            planner.plan(MissionKind::Sabotage),
            Some((vec![second], vec![third]))
        );
    }

    #[test]
    fn equal_skills_are_ordered_by_a_coin_flip() {
        // FUN_0041c250: an equal entry goes ahead on FUN_0041cd80(10) < 5,
        // half the time. Over 1000 seeded evaluations the first-listed of
        // two equal diplomats is picked about 500 times.
        let mut world = world();
        let first = agent(&mut world, 1, |c| c.diplomacy = pair(80));
        let _ = agent(&mut world, 2, |c| c.diplomacy = pair(80));

        let picked = (0..1000)
            .filter(|&tick| {
                let mut planner = Planner::new(&world, &HashSet::new(), true, tick, [false; 3]);
                planner.plan(MissionKind::Diplomacy) == Some((vec![first], vec![]))
            })
            .count();

        assert!((450..=550).contains(&picked), "picked {picked} of 1000");
    }

    #[test]
    fn positions_are_added_not_multiplied() {
        // FUN_00435980 adds positions. Combat ranks a, b, c, d and espionage
        // d, c, b, a give every candidate the same sum, so the earliest in
        // the combat list, a, wins; a product would pick b.
        let mut world = world();
        let a = unit(&mut world, 90, 60);
        let _b = unit(&mut world, 80, 70);
        let _c = unit(&mut world, 70, 80);
        let _d = unit(&mut world, 60, 90);

        let (team, _) = planner(&world).plan(MissionKind::Sabotage).unwrap();

        assert_eq!(team[0], a);
    }

    #[test]
    fn a_major_other_than_the_six_mains_fit_to_command_is_kept_off_rescues() {
        // FUN_00401d20 gives officer capability bits to any character but
        // the six mains (FUN_004025f0); FUN_0047e270 forbids them.
        let mut world = world();
        let _general = agent(&mut world, 0x2c0, |c| {
            c.is_major = true;
            c.combat = pair(90);
            c.can_be_general = true;
        });
        let main = agent(&mut world, 0x241, |c| {
            c.is_major = true;
            c.combat = pair(80);
            c.can_be_general = true;
        });

        assert_eq!(
            planner(&world).plan(MissionKind::Rescue),
            Some((vec![main], vec![]))
        );
    }

    #[test]
    fn a_main_character_who_is_not_a_leader_may_sabotage() {
        // 0x31000241 is main (FUN_004025f0) but not a leader (FUN_004024d0).
        let mut world = world();
        let main = agent(&mut world, 0x241, |c| {
            c.is_major = true;
            c.combat = pair(90);
            c.espionage = pair(90);
        });

        assert_eq!(
            planner(&world).plan(MissionKind::Sabotage),
            Some((vec![main], vec![]))
        );
    }

    /// The shipped Diplomacy record: characters of both sides only.
    fn diplomacy_record() -> MissionRecord {
        MissionRecord {
            dat_id: crate::ids::DatId::new(0x5100_0010),
            timer_min_days: 5,
            timer_spread_days: 10,
            repeats: true,
            hidden: false,
            detection_phases: true,
            can_resign: true,
            return_policy: 1,
            rules: crate::world::MissionTargetRules::default(),
            members: MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0,
                character_mask: 0x1_0000,
            },
        }
    }

    #[test]
    fn a_diplomat_of_80_is_reserved_from_sabotage() {
        // FUN_00402230 reserves a Diplomacy-capable object with diplomacy
        // above 79, and FUN_0047d520 forbids reserved objects.
        let saboteur = |diplomacy| {
            move |c: &mut Character| {
                c.diplomacy = pair(diplomacy);
                c.combat = pair(90);
                c.espionage = pair(90);
            }
        };
        let mut world = world();
        world.mission_records = vec![diplomacy_record()];
        let _reserved = agent(&mut world, 1, saboteur(80));
        assert_eq!(planner(&world).plan(MissionKind::Sabotage), None);

        let free = agent(&mut world, 2, saboteur(79));
        assert_eq!(
            planner(&world).plan(MissionKind::Sabotage),
            Some((vec![free], vec![]))
        );
    }

    #[test]
    fn a_special_force_the_diplomacy_record_refuses_is_never_reserved_for_diplomacy() {
        // The shipped Diplomacy record takes no special force (+0x48 = 0),
        // so a SPECFCSD class of mask 0x2 is not Diplomacy-capable, whatever
        // its diplomacy.
        let mut world = world();
        world.mission_records = vec![diplomacy_record()];
        world.special_force_classes.insert(
            crate::ids::DatId::new(0x3c00_0002),
            crate::world::SpecialForceClassDef {
                skills: [pair(0); 8],
                mission_mask: 0x2,
            },
        );
        let commando = unit(&mut world, 90, 90);
        if let MissionMember::SpecialForce(key) = commando {
            world.special_forces[key].skills[Skill::Diplomacy as usize] = 90;
        }

        assert_eq!(
            planner(&world).plan(MissionKind::Sabotage),
            Some((vec![commando], vec![]))
        );
    }

    #[test]
    fn a_plan_with_no_team_takes_no_decoys() {
        // port: an Incite planner with no leader of 49 sends nothing, so its
        // would-be decoy is still free for the Sabotage team.
        let mut world = world();
        let commando = unit(&mut world, 80, 80);
        let mut planner = planner(&world);

        assert_eq!(planner.plan(MissionKind::InciteUprising), None);
        assert_eq!(
            planner.plan(MissionKind::Sabotage),
            Some((vec![commando], vec![]))
        );
    }

    #[test]
    fn a_member_one_plan_sends_is_not_sent_by_the_next() {
        // port: the original's second planner would run after the first
        // order set its member on the mission.
        let mut world = world();
        let first = agent(&mut world, 1, |c| c.diplomacy = pair(90));
        let second = agent(&mut world, 2, |c| c.diplomacy = pair(80));
        let mut planner = planner(&world);

        assert_eq!(
            planner.plan(MissionKind::Diplomacy),
            Some((vec![first], vec![]))
        );
        assert_eq!(
            planner.plan(MissionKind::Diplomacy),
            Some((vec![second], vec![]))
        );
        assert_eq!(planner.plan(MissionKind::Diplomacy), None);
    }

    #[test]
    fn a_leader_is_kept_off_sabotage_but_may_negotiate() {
        // FUN_00401d20 sets 0x40000000 for the four leaders
        // (FUN_004024d0) while the side's +4 bit 0 is clear; FUN_0047d520
        // forbids it and FUN_004815a0 does not.
        let mut world = world();
        let leader = agent(&mut world, 0x240, |c| {
            c.is_major = true;
            c.diplomacy = pair(90);
            c.combat = pair(90);
            c.espionage = pair(90);
        });
        let mut planner = planner(&world);

        assert_eq!(planner.plan(MissionKind::Sabotage), None);
        assert_eq!(
            planner.plan(MissionKind::Diplomacy),
            Some((vec![leader], vec![]))
        );
    }
}
