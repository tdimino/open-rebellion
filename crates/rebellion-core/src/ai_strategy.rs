//! AI module 13 and the strategy controller (`FUN_004bf320`,
//! `FUN_004bf490`, `FUN_004e5440`; `ghidra/notes/ai-order-emission.md`,
//! "Module 13"): the side picks a mode from its census, and the mode's
//! controller writes a plan of sectors, each with a posture kind that
//! module 8's regional records then serve.
//!
//! port: one module-13 cycle (the mode pick and the six controller states)
//! runs per round; the original spreads it over eight scheduler calls. The
//! Alliance's headquarters move (`FUN_004bf3e0`) waits on the port's
//! missing headquarters object (`or-alliance-hq-move-o9ms`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ai_production::{Dir, Round, SectorRecord};
use crate::ai_records::{fleet, sys, Marks};
use crate::dat::Faction;
use crate::ids::{SectorKey, SystemKey};
use crate::uprising::Draws;

/// `c+4` bits module 13 reads and writes.
pub mod ctx {
    /// A Headquarters Only game (`FUN_004182a0` case 1: settings word 2
    /// is 1).
    pub const HEADQUARTERS_ONLY: u32 = 0x1;
    /// Replan (`FUN_004e5530`).
    pub const REPLAN: u32 = 0x2;
    /// An own Death Star fleet (`FUN_0041ba80`).
    pub const DEATH_STAR: u32 = 0x4;
    /// An own gravity-well fleet (`FUN_0041ba80`).
    pub const GRAVITY_WELL: u32 = 0x8;
    /// An own sector holds Coruscant (`FUN_0041b7e0`).
    pub const CORUSCANT: u32 = 0x80;
    /// A sector holds a headquarters (`rec+0x2c & 0x80000000`) (`FUN_0041b7e0`).
    pub const HQ_SECTOR: u32 = 0x400;
    /// The fresh pick found the side weaker (module 6 raises its regiment
    /// targets) (`FUN_004bf490`).
    pub const WEAK: u32 = 0x80_0000;
    /// Mode 5's fresh pick (module 6 lowers them) (`FUN_004bf490`).
    pub const STRONG: u32 = 0x40_0000;
}

/// One plan entry (`FUN_004ecac0`: `+0x18` id, `+0x1c` sector, `+0x20`
/// kind, `+0x24` parameter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanEntry {
    pub id: u32,
    pub sector: SectorKey,
    pub kind: u8,
    pub param: u8,
}

/// The mode's controller (`FUN_004e52c0`: `k+8`, `k+0x10`, `k+0x14`,
/// `k+0x18`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Controller {
    /// The mode it was built for (slot 3).
    pub mode: u8,
    /// `k+0x10` bit 0: the last plan was not filled.
    pub unfilled: bool,
    /// `k+0x14`, `k+0x18`: grow or shrink the next plan by one.
    pub grow: bool,
    pub shrink: bool,
}

/// One side's module 13 (`FUN_004bf0d0`) and the context fields it owns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyState {
    /// `m+0x5c`: the mode, 0 before the first pick.
    pub mode: u8,
    /// `m+0x60`.
    pub controller: Option<Controller>,
    /// `m+0x64`: the last plan, in id order.
    pub plan: Vec<PlanEntry>,
    /// `c+4` bits 2, `0x400000`, `0x800000`.
    pub flags: u32,
    /// `c+0x28`: the Alliance's tagged headquarters system.
    pub tagged_hq: Option<SystemKey>,
}

impl Default for StrategyState {
    fn default() -> Self {
        Self {
            mode: 0,
            controller: None,
            plan: Vec::new(),
            flags: 0,
            tagged_hq: None,
        }
    }
}

/// The context sums module 13 reads (`FUN_0041b7e0`, `FUN_0041ba80`).
#[derive(Debug, Clone, Copy, Default)]
pub struct Sums {
    /// `c+4` bits the census sets.
    pub flags: u32,
    /// `c+0x1ac`, `+0x1b0`, `+0x1b4`.
    pub own: i64,
    pub enemy: i64,
    pub neutral: i64,
    /// `c+0x1e8`, `+0x1ec`: free energy and raw-material slots (X, Y).
    pub free_energy: i64,
    pub free_raw: i64,
    /// `c+0x1fc`, `+0x200`: regiment targets and regiments present.
    pub regiment_targets: i64,
    pub regiments: i64,
    /// `c+0x204`: the record sync sum.
    pub sync: i64,
    /// `c+0x210`: the smallest enemy-sector threat (B).
    pub threat: i64,
    /// `c+0x1c8`: sectors not wholly held or populated (C).
    pub open_sectors: i64,
    /// `c+0x220`: tagged systems; `c+0x224`: own unblockaded HQ or
    /// Coruscant systems.
    pub tagged: i64,
    pub headquarters: i64,
    /// `c+0x258`, `+0x25c`: own attack figure and regiments aboard.
    pub attack: i64,
    pub regiments_aboard: i64,
    /// `c+0x284`, `+0x288`: foreign fleets and their attack figure.
    pub foreign_fleets: i64,
    pub foreign_attack: i64,
}

impl Sums {
    /// `FUN_00418500` case 5 over the round's records.
    #[must_use]
    pub fn of(round: &Round<'_>) -> Self {
        let mut s = Self::default();
        for rec in &round.sectors {
            s.own += rec.slot(3);
            s.enemy += rec.slot(4);
            s.neutral += rec.slot(5);
            s.free_energy += rec.slot(9);
            s.free_raw += rec.slot(0xa);
            s.regiment_targets += rec.slot(0xb);
            s.regiments += rec.slot(0x20);
            s.sync += rec.sync;
            s.tagged += rec.slot(0x46);
            s.headquarters += rec.slot(crate::ai_production::slot::OWN_HQ);
            if rec.b & 0x8 != 0 {
                let threat = rec.slot(crate::ai_production::slot::ENEMY_THREAT);
                if threat < s.threat || s.threat == 0 {
                    s.threat = threat;
                }
            }
            if rec.b & 0x2 == 0 {
                s.open_sectors += 1;
            }
            if rec.b & 0x4 != 0 && rec.b & 0x200 != 0 {
                s.flags |= ctx::CORUSCANT;
            }
            if rec.b & 0x8000_0000 != 0 {
                s.flags |= ctx::HQ_SECTOR;
            }
        }
        for record in round.fleets.values() {
            if record.flags & fleet::OWN != 0 {
                s.attack += record.attack_all;
                s.regiments_aboard += record.regiments;
                if record.flags & fleet::DEATH_STAR != 0 {
                    s.flags |= ctx::DEATH_STAR;
                }
                if record.flags & fleet::GRAVITY_WELL != 0 {
                    s.flags |= ctx::GRAVITY_WELL;
                }
            } else {
                s.foreign_fleets += 1;
                s.foreign_attack += record.attack_all;
            }
        }
        s
    }
}

/// A sector filter (`rec+0x24`, `+0x28`, `+0x2c` required and excluded).
#[derive(Debug, Clone, Copy, Default)]
struct Filter {
    s1: u32,
    s2: u32,
    s3: u32,
    n1: u32,
    n2: u32,
    n3: u32,
}

impl Filter {
    const fn b(s3: u32, n3: u32) -> Self {
        Self {
            s1: 0,
            s2: 0,
            s3,
            n1: 0,
            n2: 0,
            n3,
        }
    }

    const fn passes(self, r: &SectorRecord) -> bool {
        r.flags & self.s1 == self.s1
            && r.flags & self.n1 == 0
            && r.c28 & self.s2 == self.s2
            && r.c28 & self.n2 == 0
            && r.b & self.s3 == self.s3
            && r.b & self.n3 == 0
    }
}

/// A rank-sum holder (`FUN_00435790`, `FUN_004357b0`): its lists sorted
/// once, equal keys in random order (`FUN_0041c250`), each pick removing
/// the winner from the first list only.
pub(crate) struct Holder<T> {
    first: Vec<T>,
    positions: Vec<HashMap<T, usize>>,
}

impl<T: Copy + Eq + std::hash::Hash> Holder<T> {
    /// `FUN_00435790` for each list. port: equal keys take their order from
    /// the AI's stream at once rather than through the insertion sort's coin
    /// flips (`FUN_0041c250`, `rand(10) < 5`).
    pub(crate) fn new(lists: Vec<(Vec<(T, i64)>, Dir)>, rng: &mut impl Draws) -> Self {
        let mut sorted: Vec<Vec<T>> = Vec::new();
        for (items, dir) in lists {
            let mut keyed: Vec<(i64, i32, T)> = items
                .into_iter()
                .map(|(id, key)| (key, rng.draw(9), id))
                .collect();
            keyed.sort_by(|a, b| match dir {
                Dir::Largest => a.0.cmp(&b.0).then(a.1.cmp(&b.1)),
                Dir::Smallest => b.0.cmp(&a.0).then(a.1.cmp(&b.1)),
            });
            sorted.push(keyed.into_iter().map(|(_, _, id)| id).collect());
        }
        let first = sorted.first().cloned().unwrap_or_default();
        let positions = sorted
            .iter()
            .map(|list| {
                list.iter()
                    .enumerate()
                    .map(|(i, &id)| (id, i + 1))
                    .collect()
            })
            .collect();
        Self { first, positions }
    }

    /// The best remaining pick; ids absent from any list leave the first.
    pub(crate) fn pick(&mut self) -> Option<T> {
        let mut best: Option<(usize, usize)> = None;
        let mut keep = Vec::new();
        for &id in &self.first {
            let mut sum = 0;
            let mut everywhere = true;
            for list in &self.positions {
                match list.get(&id) {
                    Some(p) => sum += p,
                    None => everywhere = false,
                }
            }
            if !everywhere {
                continue;
            }
            keep.push(id);
            if best.is_none_or(|(_, s)| sum > s) {
                best = Some((keep.len() - 1, sum));
            }
        }
        self.first = keep;
        let (index, _) = best?;
        Some(self.first.remove(index))
    }
}

impl StrategyState {
    /// One module-13 cycle (`FUN_004bf390`): the mode pick, then the six
    /// controller states. `goals` is the production goal bits (`c+8`).
    pub fn cycle(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        regions: &mut crate::ai_regions::Regions,
        goals: u8,
    ) {
        let sums = Sums::of(round);
        self.pick_mode(&sums, goals);
        if self.controller.is_none() {
            // FUN_004e54f0: the old controller's plan bit leaves every
            // record.
            for record in &mut regions.records {
                record.satisfaction &= !0x800_0000;
            }
            self.controller = Some(Controller {
                mode: self.mode,
                ..Controller::default()
            });
            self.flags |= ctx::REPLAN;
        }
        // State 1 (`FUN_004e5530`).
        if self.flags & ctx::REPLAN != 0 {
            if let Some(k) = self.controller.as_mut() {
                k.unfilled = false;
            }
            self.replan(round, marks, goals);
            self.flags &= !ctx::REPLAN;
            // The plan kinds (FUN_00431790, FUN_004317b0) write the sector
            // records (FUN_004321d0 -> FUN_004184f0), so the round ends
            // after this turn.
            round.dirty = true;
        }
        self.size(round, regions);
        self.tag(round, marks);
    }

    /// `FUN_004bf490`: keep the mode while its test holds, else pick a
    /// fresh one; a changed mode rebuilds the controller.
    fn pick_mode(&mut self, s: &Sums, goals: u8) {
        #[expect(clippy::cast_precision_loss, reason = "the original compares floats")]
        let r = if s.foreign_attack > 0 {
            s.attack as f32 / s.foreign_attack as f32
        } else {
            s.attack as f32
        };
        let a = s.regiments_aboard - s.regiment_targets + s.regiments;
        let b = s.threat;
        let (x, y, own, c) = (s.free_energy, s.free_raw, s.own, s.open_sectors);
        let g = goals != 0;
        let low1 = x < own / 2 + 1 || y < own / 4 + 1;
        let low2 = x < own + 1 || y < own / 2 + 1;
        let enough = own / 2 < x && own / 4 < y;
        let keep = match self.mode {
            2 => low2 && (r < 1.5 || a <= b) && !g && c != 0,
            3 => r < 1.5 && !g && a > 0 && enough,
            4 => g,
            5 => r >= 1.5 && !g && a <= b && enough,
            6 => r >= 1.5 && !g && a > 0,
            7 => low2 && a > 0 && (r < 1.5 || a <= b) && !g && c != 0,
            _ => false,
        };
        if !keep {
            self.flags &= !(ctx::WEAK | ctx::STRONG);
            self.mode = if r < 1.5 || g {
                self.flags |= ctx::WEAK;
                if g {
                    4
                } else if low1 && c > 0 {
                    if a <= b {
                        2
                    } else {
                        7
                    }
                } else if a <= b {
                    1
                } else {
                    3
                }
            } else if b < a {
                6
            } else if low1 && c > 0 {
                2
            } else {
                self.flags |= ctx::STRONG;
                5
            };
        }
        if self.controller.is_some_and(|k| k.mode != self.mode) {
            self.controller = None;
        }
    }

    /// Slot 5 of the mode (modes 1-7: `FUN_004e62b0`, `FUN_004e7990`,
    /// `FUN_004e8940`, `FUN_004e9690`, `FUN_004ea060`, `FUN_004ead50`,
    /// `FUN_004e6c00`), the common tail and the write-back
    /// (`ai-order-emission.md`, "The controller").
    fn replan(&mut self, round: &mut Round<'_>, marks: &mut Marks, goals: u8) {
        let Some(k) = self.controller else {
            return;
        };
        let n0 = self.plan.len();
        let n = if k.grow {
            n0 + 1
        } else if k.shrink {
            n0.saturating_sub(1)
        } else {
            n0
        };
        let sticky = n0 != 0;
        let anchor = self.plan.first().map(|e| e.sector);
        let mut plan = Plan {
            entries: Vec::new(),
            n,
            sticky,
            anchor,
        };
        if n > 0 {
            match self.mode {
                2 => plan.lead_expand(round),
                7 => {
                    plan.lead_expand(round);
                    plan.lead_attack_mode7(round);
                }
                3 => plan.lead_mode3(round),
                4 => plan.lead_build_goal(round, goals),
                5 => plan.lead_mode5(round),
                6 => plan.lead_mode6(round),
                _ => {}
            }
        }
        let tail_t = if matches!(self.mode, 1 | 2 | 7) { 4 } else { 0 };
        plan.fill(
            round,
            3,
            Filter {
                s1: 0x4000_0000,
                s3: 4,
                ..Filter::default()
            },
            0x26,
            Dir::Largest,
        );
        plan.fill(round, 3, Filter::b(0x16, 8), 0x26, Dir::Largest);
        plan.fill(round, 3, Filter::b(6, 0), 0x26, Dir::Largest);
        plan.fill(round, 1, Filter::b(0x8_0000, tail_t), 4, Dir::Largest);
        if plan.entries.len() < n {
            if let Some(k) = self.controller.as_mut() {
                k.unfilled = true;
            }
        }
        if plan.entries.is_empty() {
            return;
        }
        for entry in &self.plan {
            let mark = marks.sector_mut(entry.sector);
            mark.kind = 0;
            mark.param = 0;
            mark.plan_id = 0;
            mark.exhausted = false;
        }
        for (i, entry) in plan.entries.iter_mut().enumerate() {
            entry.id = u32::try_from(i + 1).unwrap_or(u32::MAX);
            let mark = marks.sector_mut(entry.sector);
            mark.kind = entry.kind;
            mark.param = entry.param;
            mark.plan_id = entry.id;
            mark.exhausted = false;
        }
        for rec in &mut round.sectors {
            rec.set_mark(marks.sector(rec.key));
        }
        self.plan = plan.entries;
    }

    /// State 2 (`FUN_004e5560`): sync each entry's record and grow or
    /// shrink the plan by its satisfaction bits.
    fn size(&mut self, round: &Round<'_>, regions: &mut crate::ai_regions::Regions) {
        let Some(k) = self.controller.as_mut() else {
            return;
        };
        k.grow = false;
        k.shrink = false;
        let mut ready = true;
        let mut linked = Vec::new();
        for entry in &self.plan {
            let Some(rec) = round.sectors.iter().find(|s| s.key == entry.sector) else {
                continue;
            };
            match regions.get_mut(rec.mark.record) {
                Some(record) => {
                    record.set_plan(entry.id);
                    let bits = record.satisfaction;
                    if bits & 0x800_0000 == 0 || bits & 0x400_0000 == 0 {
                        ready = false;
                    }
                    linked.push(rec.mark.record);
                }
                None => ready = false,
            }
        }
        if !ready {
            return;
        }
        let mut lacks = false;
        let mut last = false;
        let mut any = false;
        let mut count = 0;
        for id in linked {
            let Some(record) = regions.get_mut(id) else {
                continue;
            };
            let bits = record.satisfaction;
            lacks |= bits & 0x8_0000 == 0;
            last = bits & 0x1_0000 != 0;
            if last {
                any = true;
                count += 1;
            }
            if bits & 0x2_0000 == 0 {
                self.flags |= ctx::REPLAN;
            }
            record.satisfaction &= !0x800_0000;
        }
        if any {
            if last && count > 1 {
                self.flags |= ctx::REPLAN;
                k.shrink = true;
            }
        } else if !lacks && !k.unfilled {
            self.flags |= ctx::REPLAN;
            k.grow = true;
        }
    }

    /// State 6 (`FUN_004e5870`): tag own Coruscant and, for the Alliance,
    /// one headquarters system.
    fn tag(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        let world = round.inputs.world;
        let coruscant = world
            .systems
            .iter()
            .find(|(_, s)| s.dat_id.raw() & 0xff_ffff == 0x109)
            .map(|(k, _)| k);
        if let Some(c) = coruscant {
            let own = round.records.get(&c).is_some_and(|r| r.s & sys::OWN != 0);
            set_tag(marks, round, c, own);
        }
        if round.inputs.side != Faction::Alliance {
            return;
        }
        if let Some(t) = self.tagged_hq {
            let keep = round
                .records
                .get(&t)
                .is_some_and(|r| r.s & sys::OWN != 0 && r.s & sys::HQ != 0);
            set_tag(marks, round, t, keep);
            if !keep {
                self.tagged_hq = None;
            }
        }
        if self.tagged_hq.is_some() {
            return;
        }
        let sector = {
            let items: Vec<(SectorKey, i64)> = round
                .sectors
                .iter()
                .filter(|s| s.b & 0x8000_0000 != 0)
                .map(|s| (s.key, s.slot(3)))
                .collect();
            Holder::new(vec![(items, Dir::Largest)], &mut round.rng).pick()
        };
        let Some(sector) = sector else {
            return;
        };
        let system = {
            let items: Vec<(SystemKey, i64)> = round
                .sectors
                .iter()
                .find(|s| s.key == sector)
                .map(|s| s.systems.clone())
                .unwrap_or_default()
                .into_iter()
                .filter_map(|k| round.records.get(&k))
                .filter(|r| r.s & sys::HQ != 0)
                .map(|r| (r.key, r.popularity))
                .collect();
            Holder::new(vec![(items, Dir::Largest)], &mut round.rng).pick()
        };
        if let Some(system) = system {
            let own = round
                .records
                .get(&system)
                .is_some_and(|r| r.s & sys::OWN != 0);
            set_tag(marks, round, system, own);
            if own {
                self.tagged_hq = Some(system);
            }
        }
    }
}

/// Set or clear a system's tag (`sys+0x30 & 0x10000000`) in the marks and
/// the round's record.
fn set_tag(marks: &mut Marks, round: &mut Round<'_>, system: SystemKey, on: bool) {
    let mark = marks.system_mut(system);
    if on {
        *mark |= sys::TAGGED;
    } else {
        *mark &= !sys::TAGGED;
    }
    if let Some(r) = round.records.get_mut(&system) {
        if on {
            r.s |= sys::TAGGED;
        } else {
            r.s &= !sys::TAGGED;
        }
    }
}

/// A plan under construction.
struct Plan {
    entries: Vec<PlanEntry>,
    n: usize,
    sticky: bool,
    anchor: Option<SectorKey>,
}

impl Plan {
    fn full(&self) -> bool {
        self.entries.len() >= self.n
    }

    fn holds(&self, sector: SectorKey) -> bool {
        self.entries.iter().any(|e| e.sector == sector)
    }

    fn add(&mut self, sector: SectorKey, kind: u8, param: u8) {
        self.entries.push(PlanEntry {
            id: 0,
            sector,
            kind,
            param,
        });
    }

    /// `FUN_00419a00`: the sectors the filter passes, keyed by a slot.
    fn q(round: &Round<'_>, f: Filter, key: u32, dir: Dir) -> (Vec<(SectorKey, i64)>, Dir) {
        (
            round
                .sectors
                .iter()
                .filter(|s| f.passes(s))
                .map(|s| (s.key, s.slot(key)))
                .collect(),
            dir,
        )
    }

    /// `FUN_00419b80`: keyed by the squared distance from the anchor
    /// sector, the nearest first.
    fn near(
        round: &Round<'_>,
        anchor: SectorKey,
        f: Filter,
    ) -> Option<(Vec<(SectorKey, i64)>, Dir)> {
        let a = round.sectors.iter().find(|s| s.key == anchor)?;
        let (ax, ay) = (a.x, a.y);
        Some((
            round
                .sectors
                .iter()
                .filter(|s| f.passes(s))
                .map(|s| (s.key, (s.x - ax).pow(2) + (s.y - ay).pow(2)))
                .collect(),
            Dir::Smallest,
        ))
    }

    /// `FUN_00419e90`: the last plan's sectors first, by entry id.
    fn sticky(round: &Round<'_>, f: Filter) -> (Vec<(SectorKey, i64)>, Dir) {
        (
            round
                .sectors
                .iter()
                .filter(|s| f.passes(s))
                .map(|s| {
                    let id = i64::from(s.mark.plan_id);
                    (s.key, if id == 0 { 100 } else { id })
                })
                .collect(),
            Dir::Smallest,
        )
    }

    /// `FUN_0041a060`: keyed by the non-own systems' mean staleness
    /// (`FUN_004317e0`).
    fn qs(round: &Round<'_>, f: Filter, dir: Dir) -> (Vec<(SectorKey, i64)>, Dir) {
        let day = i64::try_from(round.inputs.tick).unwrap_or(i64::MAX);
        (
            round
                .sectors
                .iter()
                .filter(|s| f.passes(s))
                .map(|s| {
                    let total: i64 = s
                        .systems
                        .iter()
                        .filter_map(|k| round.records.get(k))
                        .filter(|r| r.s & sys::OWN == 0 && r.s & sys::HELD != 0)
                        .map(|r| day - r.last_seen)
                        .sum();
                    let count = s.slot(5) + s.slot(4);
                    (
                        s.key,
                        if total > 0 && count > 0 {
                            total / count
                        } else {
                            total
                        },
                    )
                })
                .collect(),
            dir,
        )
    }

    /// The lists of one pick: the query, the near list when an anchor
    /// exists and `near` is given, and the sticky list twice.
    fn lists(
        &self,
        round: &Round<'_>,
        main: (Vec<(SectorKey, i64)>, Dir),
        near: Option<Filter>,
        sticky: Option<Filter>,
    ) -> Vec<(Vec<(SectorKey, i64)>, Dir)> {
        let mut lists = vec![main];
        if let (Some(f), Some(anchor)) = (near, self.anchor) {
            if let Some(l) = Self::near(round, anchor, f) {
                lists.push(l);
            }
        }
        if let (Some(f), true) = (sticky, self.sticky) {
            lists.push(Self::sticky(round, f));
            lists.push(Self::sticky(round, f));
        }
        lists
    }

    /// One pick from fresh lists.
    fn pick(
        &self,
        round: &mut Round<'_>,
        lists: Vec<(Vec<(SectorKey, i64)>, Dir)>,
    ) -> Option<SectorKey> {
        Holder::new(lists, &mut round.rng).pick()
    }

    /// The common tail's stage: picks from one holder until the plan is
    /// full or the holder is empty.
    fn fill(&mut self, round: &mut Round<'_>, kind: u8, f: Filter, key: u32, dir: Dir) {
        if self.full() {
            return;
        }
        let lists = self.lists(round, Self::q(round, f, key, dir), None, Some(f));
        let mut holder = Holder::new(lists, &mut round.rng);
        while !self.full() {
            let Some(sector) = holder.pick() else {
                break;
            };
            if !self.holds(sector) {
                self.add(sector, kind, 0);
            }
        }
    }

    /// Modes 2 and 7 (`FUN_004e7990`): kind 5 at a sector with an unheld
    /// system, fewest enemy systems, nearest the last lead.
    fn lead_expand(&mut self, round: &mut Round<'_>) {
        let f = Filter::b(0, 2);
        let lists = self.lists(round, Self::q(round, f, 4, Dir::Smallest), Some(f), Some(f));
        if let Some(s) = self.pick(round, lists) {
            self.add(s, 5, 0);
        }
    }

    /// Mode 7's kind-6 entry (`FUN_004e6c00`).
    fn lead_attack_mode7(&mut self, round: &mut Round<'_>) {
        if self.full() {
            return;
        }
        let f = Filter::b(0x408, 0x4_0000);
        let lists = self.lists(round, Self::q(round, f, 0xc, Dir::Smallest), None, Some(f));
        let mut holder = Holder::new(lists, &mut round.rng);
        let mut pick = holder.pick();
        if pick.is_none() {
            let f = Filter::b(0x408, 0);
            let lists = self.lists(round, Self::q(round, f, 0x48, Dir::Smallest), None, Some(f));
            holder = Holder::new(lists, &mut round.rng);
            pick = holder.pick();
        }
        while let Some(s) = pick {
            if !self.holds(s) {
                self.add(s, 6, 0);
                return;
            }
            pick = holder.pick();
        }
    }

    /// Mode 3 (`FUN_004e8940`): attack, else expand.
    fn lead_mode3(&mut self, round: &mut Round<'_>) {
        let f = Filter::b(0x408, 0x4_0000);
        let lists = self.lists(
            round,
            Self::q(round, f, 0xc, Dir::Smallest),
            Some(f),
            Some(f),
        );
        let mut pick = self.pick(round, lists);
        if pick.is_none() {
            let f = Filter::b(0x408, 0);
            let lists = self.lists(round, Self::q(round, f, 0x48, Dir::Smallest), None, Some(f));
            pick = self.pick(round, lists);
        }
        let mut kind = 6;
        if pick.is_none() {
            kind = 5;
            let f = Filter::b(0, 2);
            let lists = self.lists(round, Self::q(round, f, 3, Dir::Largest), Some(f), Some(f));
            pick = self.pick(round, lists);
        }
        if let Some(s) = pick {
            self.add(s, kind, 0);
        }
    }

    /// Mode 5 (`FUN_004ea060`): hunt a foreign fleet.
    fn lead_mode5(&mut self, round: &mut Round<'_>) {
        let f = Filter::b(0x4_0000, 0);
        let lists = self.lists(
            round,
            Self::q(round, f, 0x48, Dir::Largest),
            Some(f),
            Some(f),
        );
        let mut pick = self.pick(round, lists);
        if pick.is_none() {
            let f = Filter::b(8, 0x2000_0000);
            let lists = self.lists(round, Self::qs(round, f, Dir::Largest), Some(f), Some(f));
            pick = self.pick(round, lists);
        }
        if let Some(s) = pick {
            self.add(s, 2, 0);
        }
    }

    /// Mode 6 (`FUN_004ead50`): attack, else screen, else expand.
    fn lead_mode6(&mut self, round: &mut Round<'_>) {
        let mut kind = 6;
        let mut pick = None;
        if round.inputs.side == Faction::Empire && round_flags(round) & ctx::HQ_SECTOR != 0 {
            let f = Filter::b(0x8000_0408, 0);
            pick = self.pick(round, vec![Self::q(round, f, 0x4b, Dir::Largest)]);
        }
        if pick.is_none() {
            let f = Filter::b(0x408, 0);
            let lists = self.lists(
                round,
                Self::q(round, f, 0x4b, Dir::Largest),
                Some(f),
                Some(f),
            );
            pick = self.pick(round, lists);
        }
        if pick.is_none() {
            let f = Filter::b(0x408, 0);
            let lists = self.lists(
                round,
                Self::q(round, f, 0x30, Dir::Largest),
                Some(Filter::b(8, 0)),
                Some(f),
            );
            pick = self.pick(round, lists);
        }
        if pick.is_none() {
            kind = 1;
            let f = Filter::b(0x8_0100, 0);
            let lists = self.lists(round, Self::q(round, f, 4, Dir::Largest), Some(f), Some(f));
            pick = self.pick(round, lists);
        }
        if pick.is_none() {
            kind = 5;
            let f = Filter::b(0, 2);
            let lists = self.lists(round, Self::q(round, f, 3, Dir::Largest), Some(f), Some(f));
            pick = self.pick(round, lists);
        }
        if let Some(s) = pick {
            self.add(s, kind, 0);
        }
    }

    /// Mode 4 (`FUN_004e5b10`): the sector holding an enemy facility of
    /// the highest pending goal's category, with a system worth taking.
    fn lead_build_goal(&mut self, round: &mut Round<'_>, goals: u8) {
        for b in [0x80_u8, 0x40, 0x20, 0x10, 8, 4, 2, 1] {
            if goals & b == 0 {
                continue;
            }
            let (slot, system_key, cat) = match b {
                1 | 0x40 | 0x80 => (0x2d, 6, 3),
                2 | 8 | 0x10 => (0x2b, 4, 1),
                _ => (0x2c, 5, 2),
            };
            let f = Filter::b(2, 0);
            let lists = self.lists(
                round,
                Self::q(round, f, 0xc, Dir::Smallest),
                Some(f),
                Some(f),
            );
            let mut holder = Holder::new(lists, &mut round.rng);
            while let Some(sector) = holder.pick() {
                let Some(rec) = round.sectors.iter().find(|s| s.key == sector) else {
                    continue;
                };
                if rec.slot(slot) <= 0 {
                    continue;
                }
                let candidates: Vec<&crate::ai_production::SystemRecord> = rec
                    .systems
                    .iter()
                    .filter_map(|k| round.records.get(k))
                    .filter(|r| r.s & sys::HELD != 0 && r.s & 0x101 == 0)
                    .collect();
                let by_kind: Vec<(SystemKey, i64)> = candidates
                    .iter()
                    .map(|r| (r.key, r.slot(system_key)))
                    .collect();
                let by_need: Vec<(SystemKey, i64)> =
                    candidates.iter().map(|r| (r.key, r.slot(0x14))).collect();
                let mut systems = Holder::new(
                    vec![(by_kind, Dir::Smallest), (by_need, Dir::Smallest)],
                    &mut round.rng,
                );
                let mut accepted = false;
                while let Some(s) = systems.pick() {
                    if round
                        .records
                        .get(&s)
                        .is_some_and(|r| r.slot(system_key) >= 1)
                    {
                        accepted = true;
                        break;
                    }
                }
                if accepted {
                    if !self.holds(sector) {
                        self.add(sector, 4, cat);
                    }
                    return;
                }
            }
        }
    }
}

/// The census flags of the round (`c+4`).
fn round_flags(round: &Round<'_>) -> u32 {
    Sums::of(round).flags
}
