//! AI module 8, the regional records and their postures (`FUN_004cf600`,
//! `FUN_004774c0`, `FUN_004adb70`; `ghidra/notes/ai-order-emission.md`,
//! "Regional records"): one record per planned sector gathers the ships
//! and fleets its posture kind needs and runs them against the sector.

use serde::{Deserialize, Serialize};

use crate::ai::AIAction;
use crate::ai_production::{rank_pick, Dir, Round, SectorRecord, SystemRecord, Turn};
use crate::ai_records::{fleet, ship, sys, BestClasses, FleetRecord, Marks, ShipRecord};
use crate::ai_strategy::{ctx, Holder};
use crate::ids::{FleetKey, SectorKey, SystemKey};
use crate::uprising::Draws;

/// The ten counters `R+0x6c..+0x90` and the requirement `P+0x34..+0x58`,
/// by index.
pub mod counter {
    pub const ATTACK: usize = 0;
    pub const MANOEUVRE: usize = 1;
    /// `R+0x74`: troop room (`FUN_00477cb0`).
    pub const TROOP_ROOM: usize = 2;
    /// `R+0x78`: fighter room (`FUN_00477cb0`).
    pub const HANGAR: usize = 3;
    /// `R+0x7c`: fleets with a Death Star (`FUN_00477cb0`).
    pub const DEATH_STAR: usize = 4;
    /// `R+0x80`: bombardment (`FUN_00477cb0`).
    pub const BOMBARD: usize = 5;
    /// `R+0x84`: fleets with a gravity well (`FUN_00477cb0`).
    pub const GRAVITY_WELL: usize = 6;
    /// `R+0x88`: regiments (`FUN_00477cb0`).
    pub const REGIMENTS: usize = 7;
    /// `R+0x8c`: squadrons (`FUN_00477cb0`).
    pub const SQUADRONS: usize = 8;
    /// `R+0x90`: laser ships (`FUN_00477cb0`).
    pub const LASER: usize = 9;
}

/// `R+0x64` bits for counters present before the requirement
/// (`FUN_00477cb0`).
const PRESENCE: [(usize, u32); 9] = [
    (counter::ATTACK, 0x20_0000),
    (counter::MANOEUVRE, 0x40_0000),
    (counter::TROOP_ROOM, 0x80_0000),
    (counter::HANGAR, 0x100_0000),
    (counter::BOMBARD, 0x200_0000),
    (counter::REGIMENTS, 0x400_0000),
    (counter::SQUADRONS, 0x800_0000),
    (counter::LASER, 0x1000_0000),
    (counter::GRAVITY_WELL, 0x10_0000),
];
/// `R+0x64` bits for counters still short after the takers
/// (`FUN_00477e80`).
const SHORTFALL: [(usize, u32); 9] = [
    (counter::ATTACK, 0x1),
    (counter::MANOEUVRE, 0x2),
    (counter::TROOP_ROOM, 0x4),
    (counter::HANGAR, 0x8),
    (counter::BOMBARD, 0x10),
    (counter::REGIMENTS, 0x20),
    (counter::SQUADRONS, 0x40),
    (counter::LASER, 0x80),
    (counter::GRAVITY_WELL, 0x800),
];
/// The counters whose shortfall sends step 3 for ships (`+0x88` and
/// `+0x8c` aside).
const SHORT: [usize; 8] = [
    counter::ATTACK,
    counter::MANOEUVRE,
    counter::TROOP_ROOM,
    counter::HANGAR,
    counter::BOMBARD,
    counter::DEATH_STAR,
    counter::GRAVITY_WELL,
    counter::LASER,
];

/// One regional record (`FUN_004762b0`; `R+0x18..+0xe4`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionalRecord {
    /// `+0x18`.
    pub id: u32,
    /// `+0x1c`: the step, 1-6; 0 restarts.
    pub step: u8,
    /// `+0x24`: the ships it holds, by tag.
    pub ships: Vec<u32>,
    /// `+0x2c`: the fleets it holds.
    pub fleets: Vec<FleetKey>,
    /// `+0x34`.
    pub sector: Option<SectorKey>,
    /// `+0x38`: the flagship's tag (0 none).
    pub flagship: u32,
    /// `+0x3c`, `+0x40`: the staging and loading systems.
    pub staging: Option<SystemKey>,
    pub loading: Option<SystemKey>,
    /// `+0x44`: the plan entry of its sector.
    pub plan: u32,
    /// `+0x60`: the satisfaction bits the controller reads.
    pub satisfaction: u32,
    /// `+0x64`: the need bits.
    pub need: u32,
    /// `+0x68`: run the cleanup before the next step.
    pub resync: bool,
    /// `+0x6c`..`+0x90`: the ten counters, less the requirement.
    pub counters: [i64; 10],
    /// `+0x94`, `+0x98`: troop and fighter capacity before the
    /// requirement.
    pub troop_room: i64,
    pub hangar: i64,
    /// `+0xa0`..`+0xac`: the planner missions by kind (Abduction or
    /// Assassination, Espionage, Recon, Sabotage). hyp: their writer
    /// (module 4) is not traced, so they stay 0.
    pub missions: [u32; 4],
    /// `+0xbc`: the mission kind posture slot 6 recommends (0 none).
    pub mission: u16,
    /// `+0xc8`, `+0xcc`, `+0xd0`: class I tasks by kind.
    pub officers: [u32; 3],
    /// `+0xd4`: the class I tasks, in id order.
    pub tasks: Vec<RecordTask>,
    /// `+0xe0`, `+0xe4`: the class I task kind wanted and the task to drop.
    pub want: u8,
    pub drop: u32,
    /// `+0x5c`: the posture object.
    pub posture: Option<Posture>,
}

/// A class I task as its record reads it (`FUN_00478140`): the id, the
/// kind (3 Admiral, 4 General, 5 Commander) and the task flags `K+0x20`
/// (`crate::character_tasks`), which module 12 writes back each step.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordTask {
    pub id: u32,
    pub kind: u8,
    pub flags: u8,
}

/// A posture (`FUN_004ad8e0`; `P+8..+0x5c`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Posture {
    /// The kind, 1-6 (slot `+0x14`).
    pub kind: u8,
    /// `+8`: the step state.
    pub state: u8,
    /// `+0x28`: the current fleet; `+0x2c`, `+0x30`: the fleet and
    /// execution cursors.
    pub fleet: Option<FleetKey>,
    pub cursor: Option<FleetKey>,
    pub run: Option<FleetKey>,
    /// `+0x34`..`+0x58`: the requirement, which only grows.
    pub requirement: [i64; 10],
    /// Kind 2's visited systems; kind 4's target (`+0x5c`).
    pub visited: Vec<SystemKey>,
    pub target: Option<SystemKey>,
}

impl Posture {
    /// `FUN_00477630`: a fresh posture of `kind`.
    fn new(kind: u8) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }
}

/// One side's regional records (context `+0xd8`) and module 8's driver
/// (`m+0x3c`, `+0x40`, `+0x44`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Regions {
    pub records: Vec<RegionalRecord>,
    /// The list counter (`FUN_005f5440`), starting at 1.
    pub next_id: u32,
    /// `m+0x3c`: the driver state.
    pub state: u8,
    /// `m+0x40`, `m+0x44`: the current and next record.
    pub current: u32,
    pub next: u32,
}

impl Default for Regions {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            next_id: 1,
            state: 0,
            current: 0,
            next: 0,
        }
    }
}

/// A system query's six words (`FUN_0041a340`): `sys+0x28`, `+0x2c`,
/// `+0x30` required, then excluded.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Words {
    req: [u32; 3],
    excl: [u32; 3],
}

impl Words {
    pub(crate) const fn new(r28: u32, r2c: u32, r30: u32, x28: u32, x2c: u32, x30: u32) -> Self {
        Self {
            req: [r28, r2c, r30],
            excl: [x28, x2c, x30],
        }
    }

    /// `FUN_00431830`: each of `sys+0x28`, `+0x2c`, `+0x30` holds its
    /// required bits and none of its excluded ones.
    fn admits(self, r: &SystemRecord) -> bool {
        let words = [r.a, r.q, r.s];
        (0..3).all(|i| words[i] & self.req[i] == self.req[i] && words[i] & self.excl[i] == 0)
    }
}

/// What a record step reads besides the round.
struct Context {
    best: BestClasses,
    /// `c+4 & 4`: an own Death Star ship.
    death_star: bool,
    /// `c+4 & 0x1000000`: the Death Star shield class is producible
    /// (`FUN_004197b0`).
    shield_class: bool,
}

impl Context {
    fn of(round: &Round<'_>) -> Self {
        Self {
            best: BestClasses::of(round.inputs.world, round.inputs.research, round.inputs.side),
            death_star: crate::ai_strategy::Sums::of(round).flags & ctx::DEATH_STAR != 0,
            shield_class: round.inputs.side == crate::dat::Faction::Empire
                && crate::ai_production::ds_shield_available(
                    round.inputs.world,
                    round.inputs.research,
                ),
        }
    }
}

/// The calls one turn may take. A turn ends at its first order; the longest
/// without one is about 25 calls (the driver's 4, a record's 7 steps and a
/// posture's 13 states). port: the bound only stops a runaway; the original
/// calls until done.
const RECORD_CALLS: usize = 64;

/// What one driver call did.
enum Call {
    /// No record exists.
    End,
    /// The current record's cycle ended (`m+0x20`).
    Finished,
    Next,
    /// An order, and whether the record's cycle ended with it.
    Order(AIAction, bool),
}

impl Regions {
    /// The record with `id` (0 is none).
    pub fn get_mut(&mut self, id: u32) -> Option<&mut RegionalRecord> {
        if id == 0 {
            return None;
        }
        self.records.iter_mut().find(|r| r.id == id)
    }

    fn index(&self, id: u32) -> Option<usize> {
        if id == 0 {
            return None;
        }
        self.records.iter().position(|r| r.id == id)
    }

    /// Module 8's turn (`FUN_004cf630`): its calls until it sets done
    /// (`m+0x20`), which an order or the end of a record's cycle does
    /// (`:30-42`), or no record is left (`:47-48`). An order in mid-cycle
    /// leaves the state at 4 and raises `m+0x34` (`:33`), so the next turn
    /// resumes the record.
    pub(crate) fn turn(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Turn {
        let cx = Context::of(round);
        let mut calls = RECORD_CALLS;
        match self.turn_on(round, marks, &cx, &mut calls) {
            Call::Order(action, done) => Turn::Order {
                action,
                resume: !done,
            },
            Call::End | Call::Finished | Call::Next => Turn::Finished,
        }
    }

    /// Slot 6 (`FUN_004dd810`): a refused order restarts the module.
    pub(crate) fn reject(&mut self) {
        self.state = 0;
    }

    /// The calls of one turn, at most `calls` of them: the first call that
    /// is not `Call::Next`, else `Call::Next` once the calls run out.
    fn turn_on(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        cx: &Context,
        calls: &mut usize,
    ) -> Call {
        while *calls > 0 {
            *calls -= 1;
            match self.call(round, marks, cx) {
                Call::Next => {}
                call => return call,
            }
        }
        Call::Next
    }

    /// `FUN_004cf630`: one driver step.
    fn call(&mut self, round: &mut Round<'_>, marks: &mut Marks, cx: &Context) -> Call {
        match self.state {
            1 => {
                self.create_or_delete(round, marks);
                self.state = 2;
                Call::Next
            }
            2 => {
                // FUN_004cf760: the record after the last, else the head.
                let at = self
                    .index(self.next)
                    .or_else(|| (!self.records.is_empty()).then_some(0));
                let Some(at) = at else {
                    self.current = 0;
                    self.next = 0;
                    self.state = 0;
                    return Call::End;
                };
                self.current = self.records[at].id;
                self.next = self.records.get(at + 1).map_or(0, |r| r.id);
                self.state = 3;
                Call::Next
            }
            3 => {
                if self.assign(round, marks) {
                    self.state = 4;
                    Call::Next
                } else {
                    self.state = 0;
                    Call::Finished
                }
            }
            4 => {
                // FUN_004cfac0: a missing record is done.
                let Some(at) = self.index(self.current) else {
                    self.state = 0;
                    return Call::Finished;
                };
                let (order, done) = self.records[at].step(round, marks, cx);
                if done {
                    self.state = 0;
                }
                match (order, done) {
                    (Some(order), done) => Call::Order(order, done),
                    (None, true) => Call::Finished,
                    (None, false) => Call::Next,
                }
            }
            _ => {
                self.state = 1;
                Call::Next
            }
        }
    }

    /// State 1 (`FUN_004cf700`): one record more while sectors with a
    /// kind lack one, one fewer while a record lacks its sector.
    fn create_or_delete(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        let s: i64 = round.sectors.iter().map(|r| r.sync).sum();
        if s < 0 {
            if self
                .records
                .iter()
                .any(|r| r.satisfaction & 0x400_0000 == 0)
            {
                return;
            }
            // FUN_0042f350: a new record holds no sector, so the round
            // stays clean until `assign` links it.
            let id = self.next_id;
            self.next_id += 1;
            self.records.push(RegionalRecord {
                id,
                ..RegionalRecord::default()
            });
        } else if s > 0 {
            let stale = self.records.iter().position(|r| {
                r.sector.is_none_or(|key| {
                    round
                        .sectors
                        .iter()
                        .find(|s| s.key == key)
                        .is_none_or(|s| s.sync > 0)
                })
            });
            if let Some(at) = stale {
                let mut record = self.records.remove(at);
                record.release(round, marks);
            }
        }
    }

    /// State 3 (`FUN_004cf7c0`): the current record keeps or finds its
    /// sector, then its posture matches the sector's kind.
    fn assign(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let Some(at) = self.index(self.current) else {
            return false;
        };
        let record = &mut self.records[at];
        let id = record.id;
        let valid = record.sector.is_some_and(|key| {
            sector(round, key).is_some_and(|s| s.sync == 0 && s.mark.record == id)
        });
        if !valid {
            let mut found = None;
            if let Some(old) = record.sector {
                // FUN_00419b80: the nearest sector with a kind and no
                // record.
                found = nearest_sector(round, old, |s| s.b & 0x1_0000 != 0);
                record.release(round, marks);
                if let Some(key) = found {
                    record.link(round, marks, Some(key));
                }
            }
            if found.is_none() {
                // FUN_0041a1d0: a planned sector at random, else any with
                // a kind and no record.
                let pick = random_sector(round, |s| s.b & 0x801_0000 == 0x801_0000)
                    .or_else(|| random_sector(round, |s| s.b & 0x1_0000 != 0));
                let Some(key) = pick else {
                    return false;
                };
                record.link(round, marks, Some(key));
            }
        }
        record.sync_posture(round, marks)
    }
}

impl RegionalRecord {
    /// `FUN_004769f0`: the plan entry the record serves, copied to its
    /// ships and fleets.
    pub fn set_plan(&mut self, plan: u32) {
        if self.plan != plan {
            self.plan = plan;
        }
    }

    fn set_plan_marked(&mut self, marks: &mut Marks, round: &mut Round<'_>, plan: u32) {
        if self.plan == plan {
            return;
        }
        self.plan = plan;
        for &tag in &self.ships {
            if marks.ship(tag).record == self.id {
                marks.ship_mut(tag).plan = plan;
                if let Some(s) = round.ships.get_mut(&tag) {
                    s.plan = plan;
                }
            }
        }
        for &key in &self.fleets {
            if marks.fleet(key).record == self.id {
                marks.fleet_mut(key).plan = plan;
                if let Some(f) = round.fleets.get_mut(&key) {
                    f.plan = plan;
                }
            }
        }
    }

    /// `FUN_004768b0`: leave the old sector, take `to` when no record
    /// holds it.
    fn link(&mut self, round: &mut Round<'_>, marks: &mut Marks, to: Option<SectorKey>) {
        if let Some(old) = self.sector.take() {
            if marks.sector(old).record == self.id {
                set_sector_record(round, marks, old, 0);
            }
        }
        if let Some(key) = to {
            if marks.sector(key).record == 0 {
                set_sector_record(round, marks, key, self.id);
                self.sector = Some(key);
                let plan = marks.sector(key).plan_id;
                self.set_plan_marked(marks, round, plan);
            }
        }
        if self.sector.is_none() {
            self.set_plan_marked(marks, round, 0);
        }
    }

    /// `FUN_004773f0`: the posture follows the sector's kind.
    fn sync_posture(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let Some(key) = self.sector else {
            return false;
        };
        let mark = marks.sector(key);
        let mut ok = self.posture.as_ref().is_some_and(|p| p.kind == mark.kind);
        if !ok {
            self.posture = None;
            self.satisfaction = 0;
            self.need = 0;
            self.step = 0;
            self.resync = false;
            if mark.kind != 0 {
                self.posture = Some(Posture::new(mark.kind));
                ok = true;
            }
        }
        if mark.plan_id != self.plan {
            self.set_plan_marked(marks, round, mark.plan_id);
        }
        if !ok {
            self.release(round, marks);
        }
        ok
    }

    /// `FUN_00477190`: let go of everything.
    fn release(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        self.satisfaction = 0;
        self.need = 0;
        self.posture = None;
        self.step = 0;
        if let Some(key) = self.sector.take() {
            set_sector_record(round, marks, key, 0);
        }
        self.resync = false;
        for tag in self.ships.clone() {
            self.release_ship(round, marks, tag);
        }
        for key in self.fleets.clone() {
            self.release_fleet(round, marks, key);
        }
        // FUN_0042ef90, FUN_00476de0: every class I task goes, and with it
        // its count; module 12 releases a task its record no longer lists.
        self.tasks.clear();
        self.officers = [0; 3];
        self.counters = [0; 10];
        if let Some(system) = self.loading.take() {
            // FUN_00477190.c:105-109 clears the loading claim through the
            // system record, which marks it (FUN_00433e30 -> FUN_004184f0).
            round.dirty = true;
            *marks.system_mut(system) &= !sys::LOADING;
            if let Some(r) = round.records.get_mut(&system) {
                r.s &= !sys::LOADING;
            }
        }
        self.set_plan_marked(marks, round, 0);
    }

    /// `FUN_00478e00`: hold a ship, its figures into the counters.
    fn adopt_ship(&mut self, round: &mut Round<'_>, marks: &mut Marks, tag: u32) {
        let Some(s) = round.ships.get_mut(&tag) else {
            return;
        };
        s.flags &= !ship::FREE;
        s.record = self.id;
        s.plan = self.plan;
        let m = marks.ship_mut(tag);
        m.flags &= !ship::FREE;
        m.record = self.id;
        m.plan = self.plan;
        let c = &mut self.counters;
        for (index, value, bit) in [
            (counter::ATTACK, s.attack, 0x20_0000),
            (counter::MANOEUVRE, s.manoeuvre, 0x40_0000),
            (counter::TROOP_ROOM, s.troop_room, 0x80_0000),
            (counter::HANGAR, s.hangar, 0x100_0000),
            (counter::BOMBARD, s.bombard, 0x200_0000),
        ] {
            if value > 0 {
                c[index] += value;
                self.need |= bit;
            }
        }
        if s.regiments > 0 {
            c[counter::REGIMENTS] += s.regiments;
        }
        if s.squadrons > 0 {
            c[counter::SQUADRONS] += s.squadrons;
        }
        for (flag, index) in [
            (ship::DEATH_STAR, counter::DEATH_STAR),
            (ship::GRAVITY_WELL, counter::GRAVITY_WELL),
            (ship::LASER, counter::LASER),
        ] {
            if s.flags & flag != 0 {
                c[index] += 1;
            }
        }
        self.ships.push(tag);
    }

    /// `FUN_004790d0`: let a listed ship go, its figures out of the
    /// counters.
    fn release_ship(&mut self, round: &mut Round<'_>, marks: &mut Marks, tag: u32) {
        let Some(at) = self.ships.iter().position(|&t| t == tag) else {
            return;
        };
        if marks.ship(tag).record == self.id {
            let m = marks.ship_mut(tag);
            m.flags = (m.flags | ship::FREE) & !(ship::FLAGSHIP | ship::DEMOTE);
            m.record = 0;
            m.plan = 0;
            if let Some(s) = round.ships.get_mut(&tag) {
                s.flags = (s.flags | ship::FREE) & !(ship::FLAGSHIP | ship::DEMOTE);
                s.record = 0;
                s.plan = 0;
            }
        }
        if let Some(s) = round.ships.get(&tag) {
            let c = &mut self.counters;
            c[counter::ATTACK] -= s.attack;
            c[counter::MANOEUVRE] -= s.manoeuvre;
            c[counter::TROOP_ROOM] -= s.troop_room;
            c[counter::HANGAR] -= s.hangar;
            c[counter::BOMBARD] -= s.bombard;
            c[counter::REGIMENTS] -= s.regiments;
            c[counter::SQUADRONS] -= s.squadrons;
            for (flag, index) in [
                (ship::DEATH_STAR, counter::DEATH_STAR),
                (ship::GRAVITY_WELL, counter::GRAVITY_WELL),
                (ship::LASER, counter::LASER),
            ] {
                if s.flags & flag != 0 {
                    c[index] -= 1;
                }
            }
        }
        self.ships.remove(at);
    }

    /// `FUN_00478fd0`: hold a fleet.
    fn adopt_fleet(&mut self, round: &mut Round<'_>, marks: &mut Marks, key: FleetKey) {
        let Some(f) = round.fleets.get_mut(&key) else {
            return;
        };
        f.flags &= !fleet::UNADOPTED;
        f.record = self.id;
        f.plan = self.plan;
        let m = marks.fleet_mut(key);
        m.flags &= !fleet::UNADOPTED;
        m.record = self.id;
        m.plan = self.plan;
        self.fleets.push(key);
    }

    /// `FUN_004791d0`: let a fleet go; it is free again.
    fn release_fleet(&mut self, round: &mut Round<'_>, marks: &mut Marks, key: FleetKey) {
        let m = marks.fleet_mut(key);
        m.flags |= fleet::UNADOPTED;
        m.record = 0;
        m.plan = 0;
        if let Some(f) = round.fleets.get_mut(&key) {
            f.flags |= fleet::UNADOPTED;
            f.record = 0;
            f.plan = 0;
        }
        self.fleets.retain(|&k| k != key);
    }

    /// `FUN_004774c0`: one step; the order and whether the record's cycle
    /// is done.
    fn step(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        cx: &Context,
    ) -> (Option<AIAction>, bool) {
        if self.resync {
            self.cleanup(round, marks);
            self.resync = false;
        }
        if self.posture.is_none() {
            return (None, true);
        }
        let mut done = false;
        let order = match self.step {
            1 => {
                self.step = 2;
                self.clear_and_split(round, marks)
            }
            2 => {
                self.sum_and_require(round, cx);
                self.step = 3;
                None
            }
            3 => {
                let order = self.shortfall_and_surplus(round, marks);
                self.step = 4;
                order
            }
            4 => {
                self.missions_and_tasks(round);
                self.step = 5;
                None
            }
            5 => {
                let (order, finished) = self.posture_step(round, marks, cx);
                if finished {
                    self.step = 6;
                }
                order
            }
            6 => {
                self.flagship_and_objective(round, marks);
                self.step = 1;
                self.resync = true;
                done = true;
                None
            }
            // Step 0 (a new record, or a posture change): restart without
            // ending the cycle.
            _ => {
                self.step = 1;
                self.resync = true;
                None
            }
        };
        if order.is_some() {
            self.resync = true;
        }
        (order, done)
    }

    /// `FUN_00477810`, `FUN_00477950`: drop ships and fleets no longer
    /// the record's; adopt a free fleet that holds one of its ships.
    fn cleanup(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        for tag in self.ships.clone() {
            let keep = round
                .ships
                .get(&tag)
                .is_some_and(|s| s.record == self.id && s.flags & ship::OWN != 0);
            if !keep {
                self.release_ship(round, marks, tag);
                continue;
            }
            let container = round.ships[&tag].fleet;
            if !self.fleets.contains(&container)
                && round
                    .fleets
                    .get(&container)
                    .is_some_and(|f| f.flags & fleet::UNADOPTED != 0)
            {
                self.adopt_fleet(round, marks, container);
            }
        }
        for key in self.fleets.clone() {
            let keep = round.fleets.get(&key).is_some_and(|f| {
                f.record == self.id && f.flags & fleet::OWN != 0 && !f.ships.is_empty()
            });
            if !keep {
                self.release_fleet(round, marks, key);
            }
        }
    }

    /// Step 1 (`FUN_00477a40`): the first fleet flagged for a split gives
    /// its damaged ships (`0x1000`, `0x800000`, set by
    /// `ai_records::ship_record`) to a Create Fleet in its system.
    fn clear_and_split(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Option<AIAction> {
        self.satisfaction = 0;
        self.need = 0;
        let found = self.fleets.iter().copied().find(|key| {
            round
                .fleets
                .get(key)
                .is_some_and(|f| f.flags & 0x3000 != 0 && f.flags & 0xf000_0802 == 0)
        })?;
        let (location, tags) = {
            let f = &round.fleets[&found];
            (f.location, f.ships.clone())
        };
        let mut team = Vec::new();
        for tag in tags {
            let flags = round.ships.get(&tag).map_or(0, |s| s.flags);
            if flags & 0x80_1000 != 0 && flags & ship::EN_ROUTE == 0 {
                self.release_ship(round, marks, tag);
                team.push(tag);
            }
        }
        (!team.is_empty()).then_some(AIAction::CreateFleet {
            fleet: found,
            ships: team,
            to_system: Some(location),
        })
    }

    /// Step 2 (`FUN_00477cb0`, `FUN_004adaf0`): the fleets' sums less the
    /// posture's requirement.
    fn sum_and_require(&mut self, round: &mut Round<'_>, cx: &Context) {
        let mut c = [0i64; 10];
        for f in self.fleets.iter().filter_map(|k| round.fleets.get(k)) {
            c[counter::ATTACK] += f.attack_all;
            c[counter::MANOEUVRE] += f.manoeuvre_all;
            c[counter::TROOP_ROOM] += f.troop_room;
            c[counter::HANGAR] += f.hangar;
            c[counter::BOMBARD] += f.bombard;
            c[counter::REGIMENTS] += f.regiments;
            c[counter::SQUADRONS] += f.squadrons;
            c[counter::LASER] += f.laser_ships;
            if f.flags & fleet::DEATH_STAR != 0 {
                c[counter::DEATH_STAR] += 1;
            }
            if f.flags & fleet::GRAVITY_WELL != 0 {
                c[counter::GRAVITY_WELL] += 1;
            }
        }
        for (index, bit) in PRESENCE {
            if c[index] > 0 {
                self.need |= bit;
            }
        }
        self.troop_room = c[counter::TROOP_ROOM];
        self.hangar = c[counter::HANGAR];
        if let (Some(key), Some(p)) = (self.sector, self.posture.as_mut()) {
            if p.kind == 4 {
                p.target = kind4_target(round, key);
            }
            if let Some(sec) = sector(round, key) {
                let target = p.target.and_then(|t| round.records.get(&t));
                requirement(
                    p.kind,
                    &mut p.requirement,
                    sec,
                    cx,
                    target,
                    self.troop_room,
                    self.hangar,
                );
            }
            for (value, required) in c.iter_mut().zip(p.requirement) {
                *value -= required;
            }
        }
        self.counters = c;
    }

    fn short(&self) -> bool {
        SHORT.iter().any(|&i| self.counters[i] < 0)
    }

    /// Step 3 (`FUN_00477e80`): ships for a shortfall, from free fleets
    /// first, then from records of later plan entries; else one surplus
    /// ship let go.
    fn shortfall_and_surplus(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
    ) -> Option<AIAction> {
        let short = self.short();
        let mut order = None;
        if short {
            order = self.take_free(round, marks);
            if order.is_none() && self.short() {
                order = self.poach(round, marks);
            }
            for (index, bit) in SHORTFALL {
                if self.counters[index] < 0 {
                    self.need |= bit;
                }
            }
        }
        if order.is_none() && SHORT.iter().any(|&i| self.counters[i] > 0) {
            order = self.let_go(round, marks);
        }
        order
    }

    /// The matching rule both takers share (`FUN_004782a0`,
    /// `FUN_00478690`): the ship serves the first short counter it has an
    /// attribute for.
    fn serves(&self, s: &ShipRecord) -> bool {
        let c = &self.counters;
        (c[counter::ATTACK] < 0 && s.flags & ship::COMBAT != 0)
            || (c[counter::MANOEUVRE] < 0 && s.flags & 0x102 != 0)
            || (c[counter::LASER] < 0 && s.flags & ship::LASER != 0)
            || (c[counter::TROOP_ROOM] < 0 && s.flags & 0xc != 0)
            || (c[counter::HANGAR] < 0 && s.hangar > 0)
            || (c[counter::BOMBARD] < 0 && s.bombard > 0)
            || (c[counter::DEATH_STAR] < 0 && s.flags & ship::DEATH_STAR != 0)
            || (c[counter::GRAVITY_WELL] < 0 && s.flags & ship::GRAVITY_WELL != 0)
    }

    /// Adopt the ships of `source` that `eligible` admits while they serve
    /// a shortfall; a Create Fleet of them.
    fn take_from(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        source: FleetKey,
        eligible: u32,
        never_flagship: bool,
    ) -> Option<AIAction> {
        let tags = round.fleets.get(&source)?.ships.clone();
        let mut team = Vec::new();
        for tag in tags {
            let Some(s) = round.ships.get(&tag) else {
                continue;
            };
            if s.flags & eligible != 0 || !self.serves(s) {
                continue;
            }
            if never_flagship && s.flags & ship::FLAGSHIP != 0 {
                marks.ship_mut(tag).flags |= ship::DEMOTE;
                if let Some(s) = round.ships.get_mut(&tag) {
                    s.flags |= ship::DEMOTE;
                }
                continue;
            }
            self.adopt_ship(round, marks, tag);
            team.push(tag);
        }
        if team.is_empty() {
            return None;
        }
        let to_system = self.destination(round, source);
        Some(AIAction::CreateFleet {
            fleet: source,
            ships: team,
            to_system,
        })
    }

    /// `FUN_004782a0`: the nearest free fleet with a ship that serves.
    fn take_free(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Option<AIAction> {
        let sector_key = self.sector?;
        let (sx, sy) = {
            let s = sector(round, sector_key)?;
            (s.x, s.y)
        };
        let world = round.inputs.world;
        let items: Vec<(FleetKey, i64)> = world
            .fleets
            .keys()
            .filter_map(|k| round.fleets.get(&k))
            .filter(|f| f.has(0x8005) && f.flags & fleet::EN_ROUTE == 0)
            .filter_map(|f| {
                let at = world.systems.get(f.location)?;
                Some((
                    f.key,
                    (i64::from(at.x) - sx).pow(2) + (i64::from(at.y) - sy).pow(2),
                ))
            })
            .collect();
        let mut holder = Holder::new(vec![(items, Dir::Smallest)], &mut round.rng);
        while let Some(source) = holder.pick() {
            let order = self.take_free_ships(round, marks, source);
            if order.is_some() {
                return order;
            }
        }
        None
    }

    /// `FUN_004782a0`'s pass over one fleet: free ships with none of
    /// `0xc03800` that serve a shortfall; a Create Fleet of them.
    fn take_free_ships(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        source: FleetKey,
    ) -> Option<AIAction> {
        let tags = round.fleets.get(&source)?.ships.clone();
        let mut team = Vec::new();
        for tag in tags {
            let Some(s) = round.ships.get(&tag) else {
                continue;
            };
            // Free ships only, none en route, unfinished, crewed or
            // flagged 0x800000.
            if s.flags & ship::FREE == 0 || s.flags & 0xc0_3800 != 0 || !self.serves(s) {
                continue;
            }
            self.adopt_ship(round, marks, tag);
            team.push(tag);
        }
        if team.is_empty() {
            return None;
        }
        let to_system = self.destination(round, source);
        Some(AIAction::CreateFleet {
            fleet: source,
            ships: team,
            to_system,
        })
    }

    /// `FUN_00478690`: ships from a fleet of a later plan entry's record;
    /// its flagship is asked to stand down instead.
    fn poach(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Option<AIAction> {
        let world = round.inputs.world;
        let items: Vec<(FleetKey, i64)> = world
            .fleets
            .keys()
            .filter_map(|k| round.fleets.get(&k))
            .filter(|f| f.has(0x8001) && f.flags & 0xf000_0006 == 0 && f.plan > self.plan)
            .map(|f| (f.key, i64::from(f.plan)))
            .collect();
        let mut holder = Holder::new(vec![(items, Dir::Largest)], &mut round.rng);
        while let Some(source) = holder.pick() {
            let order = self.take_from(round, marks, source, 0xc2_3800, true);
            if order.is_some() {
                return order;
            }
        }
        None
    }

    /// `FUN_00478aa0`: the first listed ship the surplus can spare leaves
    /// in a Create Fleet.
    fn let_go(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Option<AIAction> {
        let c = self.counters;
        let others = self.ships.len() > 1;
        let mut chosen = None;
        for &tag in &self.ships {
            let Some(s) = round.ships.get(&tag) else {
                continue;
            };
            if s.flags & 0x40_2800 != 0 || engaged(round, s) {
                continue;
            }
            let spare = s.attack <= c[counter::ATTACK]
                && s.manoeuvre <= c[counter::MANOEUVRE]
                && !(s.flags & ship::LASER != 0 && c[counter::LASER] < 1)
                && !(s.flags & 0xc != 0 && c[counter::TROOP_ROOM] < s.troop_room)
                && !(s.hangar > 0 && c[counter::HANGAR] < s.hangar)
                && !(s.bombard > 0 && c[counter::BOMBARD] < s.bombard)
                && !(c[counter::DEATH_STAR] < 1 && s.flags & ship::DEATH_STAR != 0)
                && !(c[counter::GRAVITY_WELL] < 1 && s.flags & ship::GRAVITY_WELL != 0);
            if !spare {
                continue;
            }
            if tag == self.flagship && others {
                marks.ship_mut(tag).flags |= ship::DEMOTE;
                continue;
            }
            chosen = Some((tag, s.fleet));
            break;
        }
        let (tag, source) = chosen?;
        self.release_ship(round, marks, tag);
        let to_system = self.destination(round, source);
        Some(AIAction::CreateFleet {
            fleet: source,
            ships: vec![tag],
            to_system,
        })
    }

    /// `FUN_00479220`: a Create Fleet's system: the loading system, an own
    /// headquarters of the sector, the own system of the sector nearest
    /// the source fleet, else one of the nearest other sector.
    fn destination(&self, round: &mut Round<'_>, source: FleetKey) -> Option<SystemKey> {
        if let Some(l) = self.loading {
            if round.records.get(&l).is_some_and(|r| r.a & 0x2 == 0) {
                return Some(l);
            }
        }
        let key = self.sector?;
        let own_free = Words::new(0, 0, sys::OWN, 0x2, 0, 0);
        if let Some(s) = pick_system(
            round,
            key,
            Words::new(0x8000_0000, 0, sys::OWN, 0x2, 0, 0),
            7,
            Dir::Smallest,
        ) {
            return Some(s);
        }
        let from = round.fleets.get(&source).map(|f| f.location);
        if let Some(from) = from {
            if let Some(s) = nearest_system(round, key, from, own_free) {
                return Some(s);
            }
        }
        nearest_other_sector_system(round, key, own_free)
    }

    /// Step 4 (`FUN_00478000`, `FUN_00478140`): the recommended mission,
    /// the class I task the record wants and the bits they set.
    fn missions_and_tasks(&mut self, round: &Round<'_>) {
        let Some(kind) = self.posture.as_ref().map(|p| p.kind) else {
            return;
        };
        let b = self
            .sector
            .and_then(|k| sector(round, k))
            .map_or(0, |s| s.b);
        // FUN_004b7150 (posture slot 6, every kind).
        let [abduct, espionage, recon, sabotage] = self.missions;
        self.mission = if recon < 2 && b & 0x18 != 0 {
            0x54
        } else if espionage == 0 {
            0x52
        } else if sabotage == 0 && b & 0x8 != 0 {
            0x69
        } else if abduct == 0 && b & 0x4_0008 != 0 {
            if round.inputs.side == crate::dat::Faction::Alliance {
                0x62
            } else {
                0x63
            }
        } else {
            0
        };
        if self.mission != 0 {
            self.need |= 0x100;
        }
        // FUN_00478140: the class I tasks' bits (module 12), then slot 7.
        self.want = 0;
        self.drop = 0;
        self.satisfaction &= !0x700_0000;
        for t in &self.tasks {
            self.satisfaction |= 0x200_0000;
            self.need |= 0x4000_0000;
            if t.flags & 0x10 != 0 {
                self.satisfaction |= 0x400_0000;
            }
            if t.flags & 0x8 != 0 {
                if t.flags & 0x4 == 0 {
                    self.satisfaction |= 0x100_0000;
                } else if t.kind == 4 {
                    self.satisfaction |= 0x20_0000;
                }
            }
            self.need |= if t.flags & 0x1 != 0 {
                0x8000_0000
            } else {
                0x400
            };
        }
        let kind3 = self.officers[0] < 1 && !self.fleets.is_empty();
        let kind5 = self.officers[2] < 1 && self.hangar > 0 && b & 0x4_0008 != 0;
        let kind4 = self.officers[1] < 1 && self.troop_room > 0 && b & 0x8 != 0;
        self.want = match kind {
            4 | 6 if kind4 => 4,
            _ if kind3 => 3,
            _ if kind5 => 5,
            _ => 0,
        };
        if self.want != 0 {
            self.need |= 0x200;
        }
        // FUN_004add40: the first task the posture would cancel.
        let has_sector = self.sector.and_then(|k| sector(round, k)).is_some();
        let [admirals, generals, commanders] = self.officers;
        let cancel = |t: &RecordTask| match t.kind {
            // FUN_004b1650.
            3 => admirals >= 2 || (admirals == 1 && self.fleets.is_empty()),
            // FUN_004b16d0.
            5 => commanders > 1 || (commanders > 0 && self.hangar == 0),
            // Slot +0x4c: FUN_004b8b30 (kinds 4, 6), FUN_004b1690.
            _ if matches!(kind, 4 | 6) => has_sector && generals > 1,
            _ => has_sector && generals > 0,
        };
        self.drop = self.tasks.iter().find(|t| cancel(t)).map_or(0, |t| t.id);
    }

    /// Step 6: the flagship (`FUN_00479560`), the staging system
    /// (`FUN_004799d0`), the objective (posture slot 8) and the
    /// satisfaction (`FUN_00479a00`).
    fn flagship_and_objective(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        self.choose_flagship(round, marks);
        self.choose_staging(round);
        self.objective(round, marks);
        let need = self.need;
        if need & 0x4ff == 0 {
            self.satisfaction |= 0x1c_0000;
        } else if need & 0x9fe0_0000 == 0 {
            self.satisfaction |= 0x1_0000;
        } else {
            self.satisfaction |= 0x10_0000;
            if need & 0x1f == 0 {
                self.satisfaction |= 0x8_0000;
            }
        }
    }

    fn mark_flagship(&self, round: &mut Round<'_>, marks: &mut Marks, tag: u32, on: bool) {
        let m = marks.ship_mut(tag);
        if on {
            m.flags |= ship::FLAGSHIP;
        } else {
            m.flags &= !ship::FLAGSHIP;
        }
        if let Some(s) = round.ships.get_mut(&tag) {
            if on {
                s.flags |= ship::FLAGSHIP;
            } else {
                s.flags &= !ship::FLAGSHIP;
            }
        }
    }

    /// `FUN_00479560`: with a class I task the record keeps the ship with
    /// the best shield figure as its flagship.
    fn choose_flagship(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        if self.satisfaction & 0x200_0000 == 0 {
            if self.flagship != 0 && self.ships.contains(&self.flagship) {
                self.mark_flagship(round, marks, self.flagship, false);
            }
            self.flagship = 0;
        } else {
            let f = self.flagship;
            let mut valid = false;
            let current = round.ships.get(&f).cloned();
            if let Some(s) = current.filter(|s| {
                self.ships.contains(&f) && s.record == self.id && s.flags & ship::FLAGSHIP != 0
            }) {
                if s.flags & ship::DEMOTE != 0 {
                    self.mark_flagship(round, marks, f, false);
                } else {
                    valid = true;
                    if s.flags & ship::EN_ROUTE == 0 && self.fleets.contains(&s.fleet) {
                        let candidate = best_ship(round, s.fleet, 0x4001, 0x280_3800)
                            .or_else(|| best_ship(round, s.fleet, 0x4000, 0x280_3800));
                        if let Some((tag, shield)) = candidate {
                            if s.shield < shield || s.flags & ship::COMBAT == 0 {
                                self.mark_flagship(round, marks, tag, true);
                                self.mark_flagship(round, marks, f, false);
                                self.flagship = tag;
                            }
                        }
                    }
                }
            }
            if !valid {
                self.flagship = 0;
                let mut best: Option<(u32, i64)> = None;
                for &key in &self.fleets {
                    if let Some((tag, shield)) = best_ship(round, key, 0x4000, 0x280_3800) {
                        if shield > 0 && best.is_none_or(|(_, b)| shield > b) {
                            best = Some((tag, shield));
                        }
                    }
                }
                if let Some((tag, _)) = best {
                    self.mark_flagship(round, marks, tag, true);
                    self.flagship = tag;
                }
            }
        }
        for &tag in &self.ships {
            marks.ship_mut(tag).flags &= !ship::DEMOTE;
            if let Some(s) = round.ships.get_mut(&tag) {
                s.flags &= !ship::DEMOTE;
            }
        }
    }

    /// `FUN_004799d0`, posture slot 4 (`FUN_004adde0`): with a class I task,
    /// an own unblockaded staging system in the sector.
    fn choose_staging(&mut self, round: &mut Round<'_>) {
        if self.satisfaction & 0x200_0000 == 0 {
            self.staging = None;
            return;
        }
        let fit = |round: &Round<'_>, s: SystemKey| {
            round
                .records
                .get(&s)
                .is_some_and(|r| r.own && r.a & bit_blocked() == 0)
        };
        if let Some(l) = self.loading {
            if self.staging != Some(l) && fit(round, l) {
                self.staging = Some(l);
            }
        }
        // The anchor stays the staging system even when lost or blockaded.
        if self
            .staging
            .is_some_and(|s| Some(s) != self.loading && !fit(round, s))
        {
            self.staging = None;
        }
        if self.staging.is_some() {
            return;
        }
        let Some(key) = self.sector else {
            return;
        };
        let own = |r28| Words::new(r28, 0, sys::OWN, bit_blocked(), 0, 0);
        self.staging = random_system(round, key, own(0x8000_0000))
            .or_else(|| random_system(round, key, own(0x3e0_0000)))
            .or_else(|| random_system(round, key, own(0)))
            .or_else(|| nearest_other_sector_system(round, key, own(0)));
    }

    /// Posture slot 8: whether the sector still holds what the kind came
    /// for.
    fn objective(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        self.satisfaction |= 0x400_0000;
        let Some(key) = self.sector else {
            return;
        };
        let Some(b) = sector(round, key).map(|s| s.b) else {
            return;
        };
        self.satisfaction |= 0x800_0000;
        let Some(p) = self.posture.as_mut() else {
            return;
        };
        let holds = match p.kind {
            1 => b & 0x8_0000 != 0,
            2 => {
                if b & 0x4_0000 != 0 {
                    true
                } else {
                    // FUN_004b9310: an unvisited held system, oldest
                    // intel first.
                    let w = Words::new(0, 0, sys::HELD, 0, 0, 0x70_0201);
                    let day = i64::try_from(round.inputs.tick).unwrap_or(i64::MAX);
                    let mut items: Vec<(SystemKey, i64)> = systems_of(round, key)
                        .into_iter()
                        .filter_map(|s| round.records.get(&s))
                        .filter(|r| w.admits(r))
                        .map(|r| (r.key, if r.own { 0 } else { day - r.last_seen }))
                        .collect();
                    items.retain(|(s, _)| !p.visited.contains(s));
                    if items.is_empty() {
                        let mark = marks.sector_mut(key);
                        mark.exhausted = true;
                        let mark = *mark;
                        if let Some(s) = round.sectors.iter_mut().find(|s| s.key == key) {
                            s.set_mark(mark);
                        }
                        false
                    } else {
                        true
                    }
                }
            }
            3 => b & 0x4 != 0,
            4 => p
                .target
                .and_then(|t| round.records.get(&t))
                .is_some_and(|r| r.s & sys::OWN == 0),
            5 => b & 0x2 == 0 || b & 0x8 != 0,
            6 => {
                b & 0x8 != 0
                    && b & 0x400 != 0
                    && (b & 0x40_0000 != 0 || self.satisfaction & 0x20_0000 != 0)
            }
            _ => false,
        };
        if holds {
            self.satisfaction |= 0x2_0000;
        }
    }
}

/// Posture slot 20 (`FUN_004bb510`, `FUN_004b9fe0`, `FUN_004b4920`,
/// `FUN_004b8b70`, `FUN_004b3300`, `FUN_004b6a60`): the requirement, each
/// field raised to its new value and never lowered.
fn requirement(
    kind: u8,
    r: &mut [i64; 10],
    sec: &SectorRecord,
    cx: &Context,
    target: Option<&SystemRecord>,
    troop_room: i64,
    hangar: i64,
) {
    use crate::ai_production::slot;
    let attack = sec.slot(slot::FLEET_ATTACK);
    let manoeuvre = sec.slot(slot::FLEET_MANOEUVRE);
    let carried = sec.slot(slot::FLEET_SQUADRONS);
    let hq = sec.slot(slot::HQ);
    // rec+0x134, +0x138, +0x13c: the enemy's worst ground threat, most
    // squadrons at a system and best squadron manoeuvre.
    let threat = sec.slot(0x31);
    let squadrons = sec.slot(0x32);
    let squadron_manoeuvre = sec.slot(0x33);
    let shield = sec.slot(slot::WEAKEST_SHIELD);
    let b = &cx.best;
    let up = |field: &mut i64, value: i64| *field = (*field).max(value);
    let fighters = squadrons + carried;
    match kind {
        1 => {
            up(&mut r[counter::ATTACK], attack * 12 / 10);
            up(
                &mut r[counter::MANOEUVRE],
                (manoeuvre * 12 / 10).max(b.manoeuvre),
            );
            up(&mut r[counter::HANGAR], fighters * 12 / 10);
            r[counter::TROOP_ROOM] = 0;
            r[counter::DEATH_STAR] = 0;
            up(&mut r[counter::BOMBARD], (shield * 11 / 10).max(b.bombard));
            r[counter::REGIMENTS] = 0;
        }
        2 => {
            up(&mut r[counter::ATTACK], (attack * 3 / 2).max(b.attack));
            up(
                &mut r[counter::MANOEUVRE],
                (squadron_manoeuvre * 3 / 2 + manoeuvre * 3 / 2).max(b.manoeuvre),
            );
            up(&mut r[counter::HANGAR], (fighters * 3 / 2).max(b.hangar));
            r[counter::TROOP_ROOM] = 0;
            r[counter::DEATH_STAR] = 0;
            r[counter::REGIMENTS] = 0;
        }
        3 => {
            up(&mut r[counter::ATTACK], (attack * 12 / 10).max(b.attack));
            up(
                &mut r[counter::MANOEUVRE],
                (manoeuvre * 12 / 10).max(b.manoeuvre),
            );
            up(&mut r[counter::HANGAR], fighters * 12 / 10);
            r[counter::TROOP_ROOM] = 0;
            r[counter::DEATH_STAR] = 0;
            r[counter::REGIMENTS] = 0;
        }
        4 => {
            let at_target = target.map_or(0, |t| t.squadron_manoeuvre * 3 / 2);
            up(&mut r[counter::ATTACK], (attack * 3 / 2).max(b.attack));
            up(
                &mut r[counter::MANOEUVRE],
                (manoeuvre * 3 / 2 + at_target).max(b.manoeuvre),
            );
            let hangar_need = (fighters * 11 / 10).max(b.hangar);
            up(&mut r[counter::HANGAR], hangar_need);
            // UNSURE: without a target the decompile reuses the fighter
            // figure (`ai-order-emission.md`, "Regional records", Open 6).
            up(
                &mut r[counter::TROOP_ROOM],
                target.map_or(hangar_need, |t| t.threat),
            );
            if cx.death_star && hq > 0 {
                r[counter::DEATH_STAR] = 1;
            }
            r[counter::REGIMENTS] = troop_room;
        }
        5 => {
            up(&mut r[counter::ATTACK], attack * 3 / 2);
            let m = manoeuvre * 3 / 2;
            up(
                &mut r[counter::MANOEUVRE],
                if sec.b & 0x8 != 0 {
                    m.max(b.manoeuvre)
                } else {
                    m
                },
            );
            up(&mut r[counter::HANGAR], fighters * 12 / 10);
            up(&mut r[counter::TROOP_ROOM], 1);
            if cx.death_star && hq > 0 {
                r[counter::DEATH_STAR] = 1;
            }
            r[counter::REGIMENTS] = troop_room;
        }
        6 => {
            up(&mut r[counter::ATTACK], (attack * 3 / 2).max(b.attack));
            up(
                &mut r[counter::MANOEUVRE],
                (manoeuvre * 3 / 2 + squadron_manoeuvre * 3 / 2).max(b.manoeuvre),
            );
            up(&mut r[counter::HANGAR], (fighters * 3 / 2).max(b.hangar));
            up(&mut r[counter::TROOP_ROOM], threat.max(b.troop_room));
            if cx.death_star && hq > 0 {
                r[counter::DEATH_STAR] = 1;
            }
            r[counter::REGIMENTS] = troop_room;
        }
        _ => return,
    }
    if cx.shield_class {
        r[counter::GRAVITY_WELL] = 1;
    }
    r[counter::SQUADRONS] = hangar;
    r[counter::LASER] = 1;
}

/// `sys+0x28 & 3`: an uprising or a blockade.
const fn bit_blocked() -> u32 {
    crate::ai_production::bit::BLOCKED
}

/// The side's characters aboard `fleet`, prisoners aside. port: the port
/// keeps no character's ship, so these are every rider the original's
/// ships with `ship+0x24 & 0x400000` hold.
pub(crate) fn own_riders(
    world: &crate::world::GameWorld,
    fleet: FleetKey,
    side: crate::dat::Faction,
) -> Vec<crate::ids::CharacterKey> {
    let side_is_alliance = side == crate::dat::Faction::Alliance;
    world.fleets.get(fleet).map_or_else(Vec::new, |f| {
        f.characters
            .iter()
            .copied()
            .filter(|c| {
                world
                    .characters
                    .get(*c)
                    .is_some_and(|c| c.is_alliance == side_is_alliance && c.captured_by.is_none())
            })
            .collect()
    })
}

/// `FUN_00484510`: the ship's fleet carries a need, or stands in an enemy
/// system.
pub(crate) fn engaged(round: &Round<'_>, s: &ShipRecord) -> bool {
    round.fleets.get(&s.fleet).is_some_and(|f: &FleetRecord| {
        f.flags & fleet::NEEDS != 0
            || round
                .records
                .get(&f.location)
                .is_some_and(|r| r.s & sys::ENEMY != 0)
    })
}

/// `FUN_0041ac80` with key 3 (`ship+0x50`), largest first: the fleet's
/// ship with the best shield figure, and that figure.
fn best_ship(round: &mut Round<'_>, key: FleetKey, req: u32, excl: u32) -> Option<(u32, i64)> {
    let items: Vec<(u32, i64)> = round
        .fleets
        .get(&key)?
        .ships
        .iter()
        .filter_map(|t| round.ships.get(t))
        .filter(|s| s.flags & req == req && s.flags & excl == 0)
        .map(|s| (s.tag, s.shield))
        .collect();
    let tag = rank_pick(&[(items.clone(), Dir::Largest)], &mut round.rng)?;
    items.into_iter().find(|&(t, _)| t == tag)
}

fn sector<'r>(round: &'r Round<'_>, key: SectorKey) -> Option<&'r SectorRecord> {
    round.sectors.iter().find(|s| s.key == key)
}

fn systems_of(round: &Round<'_>, key: SectorKey) -> Vec<SystemKey> {
    sector(round, key)
        .map(|s| s.systems.clone())
        .unwrap_or_default()
}

/// `FUN_004317d0`: the sector's record, in the marks and the round. The
/// write marks the side's records changed (`FUN_004321d0` →
/// `FUN_004184f0`), so `link` (`FUN_004768b0`) and `release`
/// (`FUN_00477190`, also a removed record's, `FUN_0042f3d0`) leave the
/// round dirty.
fn set_sector_record(round: &mut Round<'_>, marks: &mut Marks, key: SectorKey, id: u32) {
    round.dirty = true;
    let mark = marks.sector_mut(key);
    mark.record = id;
    let mark = *mark;
    if let Some(s) = round.sectors.iter_mut().find(|s| s.key == key) {
        s.set_mark(mark);
    }
}

/// `FUN_00419b80`: the sector nearest `from` that `admits`, by sector
/// position.
fn nearest_sector(
    round: &mut Round<'_>,
    from: SectorKey,
    admits: impl Fn(&SectorRecord) -> bool,
) -> Option<SectorKey> {
    let (ox, oy) = sector(round, from).map(|s| (s.x, s.y))?;
    let items: Vec<(SectorKey, i64)> = round
        .sectors
        .iter()
        .filter(|s| admits(s))
        .map(|s| (s.key, (s.x - ox).pow(2) + (s.y - oy).pow(2)))
        .collect();
    rank_pick(&[(items, Dir::Smallest)], &mut round.rng)
}

/// `FUN_0041a1d0`: a sector that `admits`, keyed at random (`rand(20)`).
fn random_sector(
    round: &mut Round<'_>,
    admits: impl Fn(&SectorRecord) -> bool,
) -> Option<SectorKey> {
    let keys: Vec<SectorKey> = round
        .sectors
        .iter()
        .filter(|s| admits(s))
        .map(|s| s.key)
        .collect();
    let items: Vec<(SectorKey, i64)> = keys
        .into_iter()
        .map(|k| (k, i64::from(round.rng.draw(19))))
        .collect();
    rank_pick(&[(items, Dir::Smallest)], &mut round.rng)
}

/// `FUN_0041a340`: the best system of the sector the words admit.
fn pick_system(
    round: &mut Round<'_>,
    key: SectorKey,
    w: Words,
    slot: u32,
    dir: Dir,
) -> Option<SystemKey> {
    let items: Vec<(SystemKey, i64)> = systems_of(round, key)
        .into_iter()
        .filter_map(|s| round.records.get(&s))
        .filter(|r| w.admits(r))
        .map(|r| (r.key, r.slot(slot)))
        .collect();
    rank_pick(&[(items, dir)], &mut round.rng)
}

/// `FUN_0041a460`: a system of the sector the words admit, keyed at
/// random (`rand(10)`).
pub(crate) fn random_system(round: &mut Round<'_>, key: SectorKey, w: Words) -> Option<SystemKey> {
    let admitted: Vec<SystemKey> = systems_of(round, key)
        .into_iter()
        .filter(|s| round.records.get(s).is_some_and(|r| w.admits(r)))
        .collect();
    let items: Vec<(SystemKey, i64)> = admitted
        .into_iter()
        .map(|s| (s, i64::from(round.rng.draw(9))))
        .collect();
    rank_pick(&[(items, Dir::Largest)], &mut round.rng)
}

/// `FUN_0041a400`: the system of the sector the words admit nearest
/// `near`'s, by system position.
fn nearest_system(
    round: &mut Round<'_>,
    key: SectorKey,
    near: SystemKey,
    w: Words,
) -> Option<SystemKey> {
    let world = round.inputs.world;
    let at = world.systems.get(near)?;
    let (x, y) = (i64::from(at.x), i64::from(at.y));
    let items: Vec<(SystemKey, i64)> = systems_of(round, key)
        .into_iter()
        .filter_map(|s| round.records.get(&s))
        .filter(|r| w.admits(r))
        .filter_map(|r| {
            let s = world.systems.get(r.key)?;
            Some((
                r.key,
                (i64::from(s.x) - x).pow(2) + (i64::from(s.y) - y).pow(2),
            ))
        })
        .collect();
    rank_pick(&[(items, Dir::Smallest)], &mut round.rng)
}

/// `FUN_0041b2b0`: a random admitted system of the nearest other sector
/// that has one.
fn nearest_other_sector_system(
    round: &mut Round<'_>,
    key: SectorKey,
    w: Words,
) -> Option<SystemKey> {
    let (ox, oy) = sector(round, key).map(|s| (s.x, s.y))?;
    let mut best: Option<(i64, SectorKey)> = None;
    for s in &round.sectors {
        if s.key == key {
            continue;
        }
        let has = s
            .systems
            .iter()
            .any(|k| round.records.get(k).is_some_and(|r| w.admits(r)));
        if !has {
            continue;
        }
        let d = (s.x - ox).pow(2) + (s.y - oy).pow(2);
        if best.is_none_or(|(b, _)| d < b) {
            best = Some((d, s.key));
        }
    }
    let (_, other) = best?;
    random_system(round, other, w)
}

// ---------------------------------------------------------------------------
// The posture engine (`FUN_004adb70` and the kind slots)
// ---------------------------------------------------------------------------

/// `sys+0x30` claim and `fleet+0x38` need of each target slot.
const ENGAGE: (u32, u32) = (sys::ENGAGE_CLAIM, fleet::ENGAGE);
const ASSAULT: (u32, u32) = (sys::ASSAULT_CLAIM, fleet::ASSAULT);
const BOMBARD: (u32, u32) = (sys::BOMBARD_CLAIM, fleet::BOMBARD);
const DROP: (u32, u32) = (sys::DROP_CLAIM, fleet::DROP);

/// A bombardment the target test allows (`FUN_004ae320`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strike {
    /// General Bombardment `0x222`.
    General,
    /// Target Military Facilities `0x220`.
    Military,
}

impl RegionalRecord {
    /// Record step 5 (`FUN_004adb70`): one posture state; the order and
    /// whether the posture's pass is done.
    fn posture_step(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        cx: &Context,
    ) -> (Option<AIAction>, bool) {
        let Some(mut p) = self.posture.take() else {
            return (None, true);
        };
        let mut order = None;
        let mut finished = false;
        p.state = match p.state {
            1 => {
                self.rebuild_claims(round, marks);
                2
            }
            2 => {
                if self.loading_system(p.kind, round, marks) {
                    3
                } else {
                    5
                }
            }
            3 => {
                order = self.regiments(p.kind, round);
                4
            }
            4 => {
                order = self.load_squadrons(round);
                5
            }
            5 => {
                if self.next_fleet(&mut p, round) {
                    6
                } else {
                    14
                }
            }
            6 => {
                let (moved, o) = self.flagship_fleet(&p, round);
                order = o;
                if moved {
                    14
                } else {
                    7
                }
            }
            7 => {
                if self.engage_target(&mut p, round, marks) {
                    13
                } else {
                    8
                }
            }
            8 => {
                if self.assault_target(&p, round, marks) {
                    13
                } else {
                    10
                }
            }
            10 => {
                if p.kind == 5 && self.drop_target(&p, round, marks) {
                    13
                } else {
                    9
                }
            }
            9 => {
                if self.bombard_target(&p, round, marks) {
                    13
                } else {
                    12
                }
            }
            12 => {
                order = self.regroup(&mut p, round, cx);
                trace_regroup(round, p.fleet, order.as_ref());
                14
            }
            13 => {
                // FUN_004b0f90: the fleet to its target.
                order = p.fleet.and_then(|f| {
                    let target = round.fleets.get(&f)?.target?;
                    Some(AIAction::MoveFleet {
                        fleet: f,
                        to_system: target,
                        reason: crate::ai::FleetMoveReason::Attack,
                        troops: Vec::new(),
                    })
                });
                14
            }
            14 => {
                let (o, exhausted) = self.execute(&mut p, round, marks);
                order = o;
                if exhausted {
                    finished = true;
                    0
                } else {
                    14
                }
            }
            _ => 1,
        };
        self.posture = Some(p);
        (order, finished)
    }

    /// `FUN_004af6d0`: the sector's fleet claims, rebuilt from the needs of
    /// the record's fleets.
    fn rebuild_claims(&self, round: &mut Round<'_>, marks: &mut Marks) {
        let Some(key) = self.sector else {
            return;
        };
        for s in systems_of(round, key) {
            set_system_bits(round, marks, s, sys::FLEET_CLAIMS | 0x80_0000, false);
        }
        for f in &self.fleets {
            let Some(r) = round.fleets.get(f) else {
                continue;
            };
            let Some(target) = r.target else {
                continue;
            };
            let bit = if r.flags & fleet::DROP != 0 {
                sys::DROP_CLAIM
            } else if r.flags & fleet::ENGAGE != 0 {
                sys::ENGAGE_CLAIM
            } else if r.flags & fleet::ASSAULT != 0 {
                sys::ASSAULT_CLAIM
            } else if r.flags & fleet::BOMBARD != 0 {
                sys::BOMBARD_CLAIM
            } else {
                continue;
            };
            set_system_bits(round, marks, target, bit, true);
        }
    }

    /// Posture slot 13 (`FUN_004af8a0`, `FUN_004b8070`, `FUN_004b24b0`,
    /// `FUN_004b5b20`): keep or pick the loading system and its claim.
    fn loading_system(&mut self, kind: u8, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let hq_sector = self
            .sector
            .and_then(|k| sector(round, k))
            .is_some_and(|s| s.flags & 0x4000_0000 != 0);
        if let Some(l) = self.loading {
            let keep = round.records.get(&l).is_some_and(|r| {
                r.own && r.a & 0x2 == 0 && (kind > 3 || !(hq_sector && r.a & 0x8000_0000 == 0))
            });
            let s = round.records.get(&l).map_or(0, |r| r.s);
            if keep {
                let claim = match kind {
                    1..=3 => None,
                    5 if s & 0x2_0000 == 0 => Some((0x1_0000, 0x6_0000)),
                    4 if s & 0x1_0000 == 0 => Some((0, 0x5_0000)),
                    6 if s & 0x1_0000 == 0 => Some((0x2_0000, 0x5_0000)),
                    _ => Some((0, 0)),
                };
                // Each system-word write marks the record (posture slot 13:
                // FUN_004af8a0, FUN_004b24b0, FUN_004b5b20, FUN_004b8070,
                // through FUN_00433e30 -> FUN_004321d0 -> FUN_004184f0); a
                // kept system is written only behind its guard.
                match claim {
                    None => {
                        if s & 0x3_0000 != 0 {
                            round.dirty = true;
                        }
                        set_system_bits(round, marks, l, 0x3_0000, false);
                    }
                    Some((0, 0)) => {}
                    Some((clear, set)) => {
                        round.dirty = true;
                        set_system_bits(round, marks, l, clear, false);
                        set_system_bits(round, marks, l, set, true);
                    }
                }
                return true;
            }
            let clear = if kind == 4 { 0x5_0000 } else { sys::LOADING };
            // A lost loading system's claim is cleared (posture slot 13).
            round.dirty = true;
            set_system_bits(round, marks, l, clear, false);
            self.loading = None;
        }
        let Some(key) = self.sector else {
            return false;
        };
        let slot = if kind <= 3 { 0x26 } else { 0x13 };
        let found = pick_system(
            round,
            key,
            Words::new(0x8000_0000, 0, sys::OWN, 0x2, 0, 0x4_0000),
            slot,
            Dir::Largest,
        )
        .or_else(|| {
            pick_system(
                round,
                key,
                Words::new(0, 0, sys::OWN, 0x2, 0, 0x4_0000),
                slot,
                Dir::Largest,
            )
        })
        .or_else(|| {
            nearest_other_sector_system(round, key, Words::new(0, 0, sys::OWN, 0x2, 0, 0x4_0000))
        });
        let Some(l) = found else {
            return false;
        };
        let (clear, set) = match kind {
            1..=3 => (0x3_0000, 0x4_0000),
            5 => (0x1_0000, 0x6_0000),
            4 => (0, 0x5_0000),
            _ => (0x2_0000, 0x5_0000),
        };
        // A new pick claims its system (posture slot 13).
        round.dirty = true;
        set_system_bits(round, marks, l, clear, false);
        set_system_bits(round, marks, l, set, true);
        self.loading = Some(l);
        true
    }

    /// Posture slot 14: kinds 1-3 unload a fleet's regiments in an own
    /// system (`FUN_004afce0`); kinds 4-6 load the loading system's
    /// surplus regiments (`FUN_004b8470`, `FUN_004b28c0`, `FUN_004b5f30`).
    fn regiments(&self, kind: u8, round: &mut Round<'_>) -> Option<AIAction> {
        let world = round.inputs.world;
        let transport = round.inputs.transport;
        if kind <= 3 {
            let fleets = self.ranked_fleets(round, 0x4_0001, 0xf000_0002, |f| f.regiments_active);
            let f = fleets.into_iter().find(|f| {
                round
                    .fleets
                    .get(f)
                    .and_then(|r| round.records.get(&r.location))
                    .is_some_and(|r| r.s & sys::OWN != 0)
            })?;
            let troops = transport.cargo(f).to_vec();
            let location = round.fleets.get(&f)?.location;
            return (!troops.is_empty()).then_some(AIAction::MoveRegiments {
                troops,
                to: crate::troop_transport::RegimentTarget::System(location),
            });
        }
        let l = self.loading?;
        let surplus = round.records.get(&l)?.regiment_surplus;
        if surplus <= 0 {
            return None;
        }
        let fleets = self.ranked_fleets(round, fleet::OWN, fleet::EN_ROUTE, |f| f.free_troop_room);
        let f = fleets.into_iter().find(|f| {
            round
                .fleets
                .get(f)
                .is_some_and(|r| r.location == l && r.free_troop_room >= 1)
        })?;
        let room = round.fleets.get(&f)?.free_troop_room;
        let n = usize::try_from(room.min(surplus)).unwrap_or(0);
        let b = self
            .sector
            .and_then(|k| sector(round, k))
            .map_or(0, |s| s.b);
        let by_attack = kind != 5 || b & 0x8 != 0;
        let side_is_alliance = round.inputs.side == crate::dat::Faction::Alliance;
        let mut own: Vec<(crate::ids::TroopKey, i64)> = world.systems[l]
            .ground_units
            .iter()
            .filter_map(|&t| {
                let troop = world.troops.get(t)?;
                if troop.is_alliance != side_is_alliance {
                    return None;
                }
                let class = world.troop_classes.get(&troop.class_dat_id)?;
                let value = if by_attack {
                    class.attack_strength
                } else {
                    class.defense_strength
                };
                Some((t, i64::from(value)))
            })
            .collect();
        // The largest first, the earlier on a tie.
        own.sort_by(|a, b| b.1.cmp(&a.1));
        let troops: Vec<_> = own.into_iter().take(n).map(|(t, _)| t).collect();
        (!troops.is_empty()).then_some(AIAction::MoveRegiments {
            troops,
            to: crate::troop_transport::RegimentTarget::Fleet(f),
        })
    }

    /// `FUN_004b0120`: the loading system's own squadrons board the
    /// record's fleet there with the most free hangar room.
    fn load_squadrons(&self, round: &mut Round<'_>) -> Option<AIAction> {
        let l = self.loading?;
        // sys+0xac: the side's active squadrons there.
        let available = round.records.get(&l)?.slot(0x19);
        if available <= 0 {
            return None;
        }
        let fleets = self.ranked_fleets(round, fleet::OWN, fleet::EN_ROUTE, |f| f.free_hangar);
        let f = fleets
            .into_iter()
            .find(|f| round.fleets.get(f).is_some_and(|r| r.location == l))?;
        let room = round.fleets.get(&f)?.free_hangar;
        if room < 1 {
            return None;
        }
        let mut n = room.min(available);
        let world = round.inputs.world;
        let side_is_alliance = round.inputs.side == crate::dat::Faction::Alliance;
        let mut squadrons = Vec::new();
        for entry in world.systems[l]
            .fighters
            .iter()
            .filter(|e| e.is_alliance == side_is_alliance)
        {
            for _ in 0..entry.count {
                if n == 0 {
                    break;
                }
                squadrons.push(crate::troop_transport::SquadronSource::System {
                    system: l,
                    class: entry.class,
                    is_alliance: side_is_alliance,
                });
                n -= 1;
            }
        }
        (!squadrons.is_empty()).then_some(AIAction::MoveSquadrons {
            squadrons,
            fleet: f,
        })
    }

    /// `FUN_0041acd0`: the record's fleets with `req` and without `excl`,
    /// best key first.
    fn ranked_fleets(
        &self,
        round: &mut Round<'_>,
        req: u32,
        excl: u32,
        key: impl Fn(&FleetRecord) -> i64,
    ) -> Vec<FleetKey> {
        let items: Vec<(FleetKey, i64)> = self
            .fleets
            .iter()
            .filter_map(|k| round.fleets.get(k))
            .filter(|f| f.record == self.id && f.has(req) && f.flags & excl == 0)
            .map(|f| (f.key, key(f)))
            .collect();
        let mut holder = Holder::new(vec![(items, Dir::Largest)], &mut round.rng);
        std::iter::from_fn(|| holder.pick()).collect()
    }

    /// `FUN_004b04c0`: the next fleet free for a target, from the cursor
    /// round the list.
    fn next_fleet(&self, p: &mut Posture, round: &Round<'_>) -> bool {
        let n = self.fleets.len();
        let start = p
            .cursor
            .and_then(|c| self.fleets.iter().position(|&f| f == c))
            .unwrap_or(0);
        for i in 0..n {
            let at = (start + i) % n;
            let key = self.fleets[at];
            if round
                .fleets
                .get(&key)
                .is_some_and(|f| f.flags & 0xf000_3802 == 0)
            {
                p.fleet = Some(key);
                p.cursor = self.fleets.get(at + 1).copied();
                return true;
            }
        }
        p.fleet = None;
        false
    }

    /// `FUN_004b06b0`: the flagship's fleet goes to the staging system;
    /// another fleet sets its riders down in an own system.
    fn flagship_fleet(&self, p: &Posture, round: &Round<'_>) -> (bool, Option<AIAction>) {
        let Some(f) = p.fleet else {
            return (false, None);
        };
        let Some(r) = round.fleets.get(&f) else {
            return (false, None);
        };
        if self.flagship != 0 && r.ships.contains(&self.flagship) {
            if self.satisfaction & 0x100_0000 != 0 {
                return (true, None);
            }
            if self.satisfaction & 0x400_0000 != 0 {
                if let Some(s) = self
                    .staging
                    .filter(|s| round.records.get(s).is_some_and(|r| r.a & 0x2 == 0))
                {
                    return (
                        true,
                        Some(AIAction::MoveFleet {
                            fleet: f,
                            to_system: s,
                            reason: crate::ai::FleetMoveReason::Reinforce,
                            troops: Vec::new(),
                        }),
                    );
                }
            }
            return (false, None);
        }
        let home_own = round
            .records
            .get(&r.location)
            .is_some_and(|s| s.s & sys::OWN != 0);
        if r.flags & fleet::OWN_CHARACTER == 0 || r.flags & fleet::EN_ROUTE != 0 || !home_own {
            return (false, None);
        }
        // A rider sets down unless it holds one of this record's tasks
        // (character +0x28, FUN_00476e50).
        let characters: Vec<_> = own_riders(round.inputs.world, f, round.inputs.side)
            .into_iter()
            .filter(|c| {
                round
                    .task_of
                    .get(c)
                    .is_none_or(|id| !self.tasks.iter().any(|t| t.id == *id))
            })
            .collect();
        (
            false,
            (!characters.is_empty()).then_some(AIAction::MoveCharacters {
                characters,
                to_system: r.location,
            }),
        )
    }

    /// Set a fleet's target and need, and claim the system.
    fn aim(
        round: &mut Round<'_>,
        marks: &mut Marks,
        f: FleetKey,
        target: SystemKey,
        (claim, need): (u32, u32),
    ) {
        set_system_bits(round, marks, target, claim, true);
        set_fleet_need(round, marks, f, need, Some(target));
    }

    /// Posture slot 15: an enemy fleet or squadrons the fleet can beat
    /// (`FUN_004bafe0`, `FUN_004b9c90`, `FUN_004b4560`, `FUN_004b8880`,
    /// `FUN_004b2d70`, `FUN_004b6340`).
    fn engage_target(&self, p: &mut Posture, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let (Some(f), Some(key)) = (p.fleet, self.sector) else {
            return false;
        };
        let Some(r) = round.fleets.get(&f).cloned() else {
            return false;
        };
        if r.attack <= 0 {
            return false;
        }
        if p.kind == 4 {
            if let Some(t) = p.target {
                let fits = round
                    .records
                    .get(&t)
                    .is_some_and(|s| s.s & sys::FOREIGN_FLEET != 0 && beats(&r, s));
                if fits {
                    Self::aim(round, marks, f, t, ENGAGE);
                    return true;
                }
            }
        }
        let fleets = Words::new(0, 0, sys::FOREIGN_FLEET, 0, 0, sys::ENGAGE_CLAIM);
        let mut found = first_passing(round, key, fleets, 0x27, Dir::Largest, |s| beats(&r, s));
        if found.is_none() && matches!(p.kind, 1 | 2) && r.manoeuvre > 0 {
            let squadrons = Words::new(0, 0x40_0000, sys::ENEMY, 0, 0, sys::ENGAGE_CLAIM);
            found = first_passing(round, key, squadrons, 0x1b, Dir::Largest, |s| beats(&r, s));
        }
        let Some(t) = found else {
            return false;
        };
        if p.kind == 2 {
            p.visited.push(t);
        }
        Self::aim(round, marks, f, t, ENGAGE);
        true
    }

    /// Posture slot 16: an assault (`FUN_004b8ab0` kind 4,
    /// `FUN_004b6510` kind 6).
    fn assault_target(&self, p: &Posture, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let (Some(f), Some(key)) = (p.fleet, self.sector) else {
            return false;
        };
        let Some(r) = round.fleets.get(&f).cloned() else {
            return false;
        };
        if r.regiments_active <= 0 {
            return false;
        }
        let best = round_best(round);
        let found = match p.kind {
            4 => p.target.filter(|t| {
                round
                    .records
                    .get(t)
                    .is_some_and(|s| can_assault(&r, s, best.manoeuvre))
            }),
            6 => {
                let w = Words::new(0, 0, sys::ENEMY, 0, 0, 0x30_0200);
                let hq = self
                    .sector
                    .and_then(|k| sector(round, k))
                    .map_or(0, |s| s.slot(crate::ai_production::slot::HQ));
                let ranked = if hq > 0 {
                    ranked_systems(round, key, w, &[(0x32, Dir::Largest)])
                } else {
                    let both = Words::new(0, 0, sys::ENEMY | sys::HELD, 0, 0, 0x30_0200);
                    ranked_two(
                        round,
                        key,
                        (w, 0x21, Dir::Largest),
                        (both, 0x14, Dir::Smallest),
                    )
                };
                ranked.into_iter().find(|t| {
                    round
                        .records
                        .get(t)
                        .is_some_and(|s| can_assault(&r, s, best.manoeuvre))
                })
            }
            _ => None,
        };
        let Some(t) = found else {
            return false;
        };
        Self::aim(round, marks, f, t, ASSAULT);
        true
    }

    /// Posture slot 18, kind 5 (`FUN_004b2f40`): a neutral unpopulated
    /// system to drop regiments on.
    fn drop_target(&self, p: &Posture, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let (Some(f), Some(key)) = (p.fleet, self.sector) else {
            return false;
        };
        let Some(r) = round.fleets.get(&f).cloned() else {
            return false;
        };
        if r.regiments_active <= 0 {
            return false;
        }
        let w = Words::new(0, 0, sys::NEUTRAL, 0, 0, 0x10_0208);
        let Some(t) = first_passing(round, key, w, 0xf, Dir::Largest, |s| can_drop(&r, s)) else {
            return false;
        };
        Self::aim(round, marks, f, t, DROP);
        true
    }

    /// Posture slot 17 (`FUN_004bb300` kind 1, `FUN_004b4730` kind 3,
    /// `FUN_004b3110` kind 5, `FUN_004b6870` kind 6).
    fn bombard_target(&self, p: &Posture, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        let (Some(f), Some(key)) = (p.fleet, self.sector) else {
            return false;
        };
        let Some(r) = round.fleets.get(&f).cloned() else {
            return false;
        };
        if r.bombard <= 0 {
            return false;
        }
        let enemy = Words::new(0, 0, sys::ENEMY, 0, 0, 0x70_0200);
        let held = Words::new(0, 0, sys::ENEMY | sys::HELD, 0, 0, 0x70_0200);
        let ranked = match p.kind {
            1 => ranked_lists(
                round,
                key,
                &[
                    (enemy, 8, Dir::Smallest),
                    (enemy, 0x21, Dir::Largest),
                    (enemy, 0, Dir::Largest),
                ],
            ),
            3 => ranked_lists(
                round,
                key,
                &[(enemy, 8, Dir::Smallest), (enemy, 0x10, Dir::Largest)],
            ),
            5 | 6 => ranked_lists(
                round,
                key,
                &[(held, 8, Dir::Smallest), (held, 0x21, Dir::Largest)],
            ),
            _ => return false,
        };
        let best = round_best(round);
        let found = ranked.into_iter().find(|t| {
            round
                .records
                .get(t)
                .is_some_and(|s| strike(&r, s, sector(round, key), best.manoeuvre).is_some())
        });
        let Some(t) = found else {
            return false;
        };
        Self::aim(round, marks, f, t, BOMBARD);
        true
    }

    /// `FUN_004b0c90`: with no target, the fleet joins the nearest other
    /// fleet of the record, else goes to the rally system (slot 21).
    fn regroup(&self, p: &mut Posture, round: &mut Round<'_>, cx: &Context) -> Option<AIAction> {
        let f = p.fleet?;
        let r = round.fleets.get(&f)?.clone();
        let world = round.inputs.world;
        if self.fleets.len() > 1 {
            let at = world.systems.get(r.location)?;
            let (x, y) = (i64::from(at.x), i64::from(at.y));
            let items: Vec<(FleetKey, i64)> = self
                .fleets
                .iter()
                .filter(|&&k| k != f)
                .filter_map(|k| round.fleets.get(k))
                .filter(|o| o.has(fleet::OWN) && o.flags & 0xf000_0002 == 0)
                .filter(|o| {
                    round
                        .records
                        .get(&o.location)
                        .is_some_and(|s| s.s & sys::FOREIGN_FLEET == 0)
                })
                .filter_map(|o| {
                    let s = world.systems.get(o.location)?;
                    Some((
                        o.key,
                        (i64::from(s.x) - x).pow(2) + (i64::from(s.y) - y).pow(2),
                    ))
                })
                .collect();
            if let Some(target) = rank_pick(&[(items, Dir::Smallest)], &mut round.rng) {
                return Some(AIAction::JoinFleet {
                    fleet: f,
                    ships: Vec::new(),
                    target,
                });
            }
        }
        let to = self.rally(p, &r, round, cx)?;
        (to != r.location).then_some(AIAction::MoveFleet {
            fleet: f,
            to_system: to,
            reason: crate::ai::FleetMoveReason::Reinforce,
            troops: Vec::new(),
        })
    }

    /// Posture slot 21 (`FUN_004bb630`, `FUN_004ba0d0`, `FUN_004b4a20`,
    /// `FUN_004b8ca0`, `FUN_004b3400`, `FUN_004b6b70`): a lone fleet
    /// scouts; else the loading system, an own headquarters, a role
    /// system, or out of an enemy system.
    fn rally(
        &self,
        p: &mut Posture,
        r: &FleetRecord,
        round: &mut Round<'_>,
        cx: &Context,
    ) -> Option<SystemKey> {
        let key = self.sector?;
        let unclaimed = 0x70_0201;
        if self.fleets.len() == 1 && r.flags & 0xa00 == 0 {
            let strong = r.attack >= cx.best.attack && r.manoeuvre >= cx.best.manoeuvre;
            let scout = match p.kind {
                1 => oldest_intel(round, key, Words::new(0, 0, 0, 0, 0, unclaimed), &[]),
                2 if strong => {
                    let pick = oldest_intel(
                        round,
                        key,
                        Words::new(0, 0, sys::HELD, 0, 0, unclaimed),
                        &p.visited,
                    );
                    if let Some(s) = pick {
                        p.visited.push(s);
                    }
                    pick
                }
                6 if strong => random_system(round, key, Words::new(0, 0, 0, 0, 0, unclaimed)),
                5 if self.loading.is_none() => {
                    random_system(round, key, Words::new(0, 0, 0, 0, 0x800_0000, unclaimed))
                }
                _ => None,
            };
            if scout.is_some() {
                return scout;
            }
        }
        if let Some(l) = self
            .loading
            .filter(|l| round.records.get(l).is_some_and(|s| s.a & 0x2 == 0))
        {
            return Some(l);
        }
        if let Some(s) = pick_system(
            round,
            key,
            Words::new(0x8000_0000, 0, sys::OWN, 0x2, 0, 0),
            7,
            Dir::Smallest,
        ) {
            return Some(s);
        }
        if let Some(s) = pick_system(
            round,
            key,
            Words::new(0x3e0_0000, 0, sys::OWN, 0x2, 0, 0),
            0x21,
            Dir::Largest,
        ) {
            return Some(s);
        }
        let in_enemy = round
            .records
            .get(&r.location)
            .is_some_and(|s| s.s & sys::ENEMY != 0);
        if in_enemy {
            return nearest_system(
                round,
                key,
                r.location,
                Words::new(0, 0, 0, 0, 0, sys::ENEMY | sys::FOREIGN_FLEET),
            );
        }
        None
    }

    /// `FUN_004b1070`: the next arrived fleet with a need acts on it; the
    /// pass is done when the list runs out.
    fn execute(
        &self,
        p: &mut Posture,
        round: &mut Round<'_>,
        marks: &mut Marks,
    ) -> (Option<AIAction>, bool) {
        let start = p
            .run
            .and_then(|c| self.fleets.iter().position(|&f| f == c))
            .unwrap_or(0);
        let best = round_best(round);
        for at in start..self.fleets.len() {
            let f = self.fleets[at];
            let Some(r) = round.fleets.get(&f).cloned() else {
                continue;
            };
            if r.flags & fleet::NEEDS == 0 {
                continue;
            }
            if r.flags & fleet::EN_ROUTE != 0 {
                trace_execute(round, f, &r, "en_route", None);
                continue;
            }
            let target = r
                .target
                .filter(|t| r.location == *t && round.records.contains_key(t));
            let Some(t) = target else {
                set_fleet_need(round, marks, f, 0, None);
                trace_execute(round, f, &r, "cleared_away", None);
                continue;
            };
            let s = round.records[&t].clone();
            let sec = round.inputs.world.systems.get(t).map(|v| v.sector);
            let strike_kind = strike(&r, &s, sec.and_then(|k| sector(round, k)), best.manoeuvre);
            let destroy = r.flags & fleet::DEATH_STAR != 0 && s.slot(0x32) > 0;
            let order = if r.flags & fleet::ENGAGE != 0 {
                if !beats(&r, &s) {
                    set_fleet_need(round, marks, f, 0, None);
                }
                None
            } else if r.flags & fleet::ASSAULT != 0 {
                if destroy {
                    Some(AIAction::DestroySystem {
                        fleet: f,
                        system: t,
                    })
                } else if can_assault(&r, &s, best.manoeuvre) {
                    Some(AIAction::Assault { fleet: f })
                } else if let Some(kind) = strike_kind {
                    Some(AIAction::Bombard {
                        fleet: f,
                        system: t,
                        kind,
                    })
                } else {
                    set_fleet_need(round, marks, f, 0, None);
                    None
                }
            } else if r.flags & fleet::BOMBARD != 0 {
                set_fleet_need(round, marks, f, 0, None);
                if destroy {
                    Some(AIAction::DestroySystem {
                        fleet: f,
                        system: t,
                    })
                } else {
                    strike_kind.map(|kind| AIAction::Bombard {
                        fleet: f,
                        system: t,
                        kind,
                    })
                }
            } else if can_drop(&r, &s) {
                // FUN_004b1440: the first regiment aboard goes down.
                round
                    .inputs
                    .transport
                    .cargo(f)
                    .first()
                    .map(|&troop| AIAction::MoveRegiments {
                        troops: vec![troop],
                        to: crate::troop_transport::RegimentTarget::System(t),
                    })
            } else {
                set_fleet_need(round, marks, f, 0, None);
                None
            };
            let outcome = match &order {
                Some(_) => "order",
                None if round
                    .fleets
                    .get(&f)
                    .is_some_and(|n| n.flags & fleet::NEEDS == 0) =>
                {
                    "cleared"
                }
                None => "kept",
            };
            trace_execute(round, f, &r, outcome, order.as_ref());
            if order.is_some() {
                p.run = self.fleets.get(at + 1).copied();
                return (order, false);
            }
        }
        p.run = None;
        (None, true)
    }
}

/// The names of the need bits set in `flags`.
fn need_names(flags: u32) -> Vec<&'static str> {
    [
        (fleet::ENGAGE, "engage"),
        (fleet::ASSAULT, "assault"),
        (fleet::BOMBARD, "bombard"),
        (fleet::DROP, "drop"),
    ]
    .into_iter()
    .filter(|(bit, _)| flags & bit != 0)
    .map(|(_, name)| name)
    .collect()
}

/// port: tooling. One `posture_execute` record per fleet `execute` visits
/// with a need: `before` is the record read at entry, the need after is
/// the round's record now.
fn trace_execute(
    round: &Round<'_>,
    f: FleetKey,
    before: &FleetRecord,
    outcome: &'static str,
    order: Option<&AIAction>,
) {
    crate::trace::record(crate::trace::TraceMask::AI, "posture_execute", || {
        let world = round.inputs.world;
        let after = round.fleets.get(&f).map_or(0, |n| n.flags & fleet::NEEDS);
        let order = order.map(|o| match o {
            AIAction::Assault { .. } => "Assault",
            AIAction::Bombard { .. } => "Bombard",
            AIAction::DestroySystem { .. } => "DestroySystem",
            AIAction::MoveRegiments { .. } => "MoveRegiments",
            _ => "Other",
        });
        serde_json::json!({
            "faction": format!("{:?}", round.inputs.side),
            "fleet": world.fleet_name(f),
            "location": world.systems.get(before.location).map(|s| s.name.clone()),
            "target": before
                .target
                .and_then(|t| world.systems.get(t))
                .map(|s| s.name.clone()),
            "need_before": need_names(before.flags & fleet::NEEDS),
            "need_after": need_names(after),
            "outcome": outcome,
            "order": order,
        })
    });
}

/// port: tooling. One `posture_regroup` record per regroup step.
fn trace_regroup(round: &Round<'_>, fleet: Option<FleetKey>, order: Option<&AIAction>) {
    crate::trace::record(crate::trace::TraceMask::AI, "posture_regroup", || {
        let world = round.inputs.world;
        let (action, join, to) = match order {
            Some(AIAction::JoinFleet { target, .. }) => {
                ("join", world.fleet_name(*target).map(str::to_owned), None)
            }
            Some(AIAction::MoveFleet { to_system, .. }) => (
                "move",
                None,
                world.systems.get(*to_system).map(|s| s.name.clone()),
            ),
            _ => ("none", None, None),
        };
        serde_json::json!({
            "faction": format!("{:?}", round.inputs.side),
            "fleet": fleet.and_then(|f| world.fleet_name(f)),
            "action": action,
            "join_target": join,
            "move_to": to,
        })
    });
}

/// `FUN_004ae150`: the fleet can beat the fleets and squadrons there.
fn beats(f: &FleetRecord, s: &SystemRecord) -> bool {
    let (mut a, mut m, mut q) = (0, 0, 0);
    if s.s & sys::FOREIGN_FLEET != 0 {
        a += s.slot(0x27);
        m += s.slot(0x28);
        q += s.slot(0x2b);
    }
    if s.s & sys::ENEMY != 0 {
        a += s.slot(0x1a);
        m += s.slot(0x1b);
        q += s.slot(0x17);
    }
    if a <= 0 || a * 12 / 10 >= f.attack {
        return false;
    }
    if q > 0 {
        if f.manoeuvre <= m {
            return false;
        }
        if f.flags & fleet::DEATH_STAR != 0 || s.s & sys::FOREIGN_DEATH_STAR != 0 {
            return f.squadrons_active > q;
        }
        if f.laser_ships < 1 {
            return f.squadrons_active >= q;
        }
    }
    true
}

/// The manoeuvre a fleet must reach to act at a system: none where it
/// stands, else the squadrons' or the side's best class's
/// (`FUN_004ae240`, `FUN_004ae320`: `max(sys+0xb4, c+0x168)`).
fn guard(f: &FleetRecord, s: &SystemRecord, best_manoeuvre: i64) -> i64 {
    if f.location == s.key {
        0
    } else {
        s.slot(0x1b).max(best_manoeuvre)
    }
}

/// `FUN_004ae240`: the fleet's regiments can take the system.
fn can_assault(f: &FleetRecord, s: &SystemRecord, best_manoeuvre: i64) -> bool {
    s.s & 0x101 == 0
        && s.s & sys::HELD != 0
        && guard(f, s, best_manoeuvre) <= f.manoeuvre
        && s.slot(0x34) <= f.regiments_active
        && (s.slot(0x17) <= f.squadrons_active || f.laser_ships > 0)
}

/// `FUN_004ae320`: the bombardment the fleet can make there.
fn strike(
    f: &FleetRecord,
    s: &SystemRecord,
    sec: Option<&SectorRecord>,
    best_manoeuvre: i64,
) -> Option<Strike> {
    use crate::ai_production::slot;
    let sec = sec?;
    if s.s & sys::ENEMY == 0
        || guard(f, s, best_manoeuvre) > f.manoeuvre
        || f.bombard <= s.slot(8)
        || s.slot(0xb) != 0
    {
        return None;
    }
    let d = (sec.slot(slot::HELD) - sec.slot(slot::ENEMY) + 1) / 2;
    let regiments = s.slot(0x12);
    if d < 1 && (regiments > 0 || s.slot(0x21) > 0) {
        Some(Strike::General)
    } else if d >= 1 && s.slot(7) + regiments > d && regiments > 0 {
        Some(Strike::Military)
    } else {
        None
    }
}

/// `FUN_004ae440`: regiments may land on the neutral unpopulated system.
fn can_drop(f: &FleetRecord, s: &SystemRecord) -> bool {
    s.s & sys::POPULATED == 0
        && s.s & sys::NEUTRAL != 0
        && s.s & sys::FOREIGN_FLEET == 0
        && f.regiments_active > 0
}

/// The side's best class figures for the round.
fn round_best(round: &Round<'_>) -> BestClasses {
    BestClasses::of(round.inputs.world, round.inputs.research, round.inputs.side)
}

/// `sys+0x30` bits in the marks and the round.
pub(crate) fn set_system_bits(
    round: &mut Round<'_>,
    marks: &mut Marks,
    key: SystemKey,
    bits: u32,
    on: bool,
) {
    let m = marks.system_mut(key);
    if on {
        *m |= bits & sys::MARKS;
    } else {
        *m &= !bits;
    }
    if let Some(r) = round.records.get_mut(&key) {
        if on {
            r.s |= bits;
        } else {
            r.s &= !bits;
        }
    }
}

/// A fleet's need bits and target, in the marks and the round.
fn set_fleet_need(
    round: &mut Round<'_>,
    marks: &mut Marks,
    key: FleetKey,
    need: u32,
    target: Option<SystemKey>,
) {
    let m = marks.fleet_mut(key);
    m.flags = (m.flags & !fleet::NEEDS) | need;
    m.target = target;
    if let Some(r) = round.fleets.get_mut(&key) {
        r.flags = (r.flags & !fleet::NEEDS) | need;
        r.target = target;
    }
}

/// `FUN_0041a340` read in rank order.
fn ranked_systems(
    round: &mut Round<'_>,
    key: SectorKey,
    w: Words,
    keys: &[(u32, Dir)],
) -> Vec<SystemKey> {
    let lists: Vec<(Words, u32, Dir)> = keys.iter().map(|&(k, d)| (w, k, d)).collect();
    ranked_lists(round, key, &lists)
}

fn ranked_two(
    round: &mut Round<'_>,
    key: SectorKey,
    a: (Words, u32, Dir),
    b: (Words, u32, Dir),
) -> Vec<SystemKey> {
    ranked_lists(round, key, &[a, b])
}

/// Several system lists of the sector, rank-summed, best first.
fn ranked_lists(
    round: &mut Round<'_>,
    key: SectorKey,
    lists: &[(Words, u32, Dir)],
) -> Vec<SystemKey> {
    let systems = systems_of(round, key);
    let lists: Vec<(Vec<(SystemKey, i64)>, Dir)> = lists
        .iter()
        .map(|&(w, slot, dir)| {
            (
                systems
                    .iter()
                    .filter_map(|s| round.records.get(s))
                    .filter(|r| w.admits(r))
                    .map(|r| (r.key, r.slot(slot)))
                    .collect(),
                dir,
            )
        })
        .collect();
    let mut holder = Holder::new(lists, &mut round.rng);
    std::iter::from_fn(|| holder.pick()).collect()
}

/// The first system in rank order the test passes.
fn first_passing(
    round: &mut Round<'_>,
    key: SectorKey,
    w: Words,
    slot: u32,
    dir: Dir,
    test: impl Fn(&SystemRecord) -> bool,
) -> Option<SystemKey> {
    ranked_systems(round, key, w, &[(slot, dir)])
        .into_iter()
        .find(|s| round.records.get(s).is_some_and(&test))
}

/// `FUN_0041a3a0`: the admitted system longest unseen (an own one counts
/// as seen today), `skip` aside.
fn oldest_intel(
    round: &mut Round<'_>,
    key: SectorKey,
    w: Words,
    skip: &[SystemKey],
) -> Option<SystemKey> {
    let day = i64::try_from(round.inputs.tick).unwrap_or(i64::MAX);
    let items: Vec<(SystemKey, i64)> = systems_of(round, key)
        .into_iter()
        .filter(|s| !skip.contains(s))
        .filter_map(|s| round.records.get(&s))
        .filter(|r| w.admits(r))
        .map(|r| (r.key, if r.own { 0 } else { day - r.last_seen }))
        .collect();
    rank_pick(&[(items, Dir::Largest)], &mut round.rng)
}

/// `FUN_004b8f80`: kind 4's target, a held system not the side's with
/// at most one shield generator, fewest yards of the plan's facility and
/// least regiment need first, that has such a yard.
fn kind4_target(round: &mut Round<'_>, key: SectorKey) -> Option<SystemKey> {
    let param = sector(round, key).map_or(0, |s| s.mark.param);
    let yards = match param {
        1 => 4,
        2 => 5,
        3 => 6,
        _ => 0,
    };
    let w = Words::new(0, 0, sys::HELD, 0, 0, 0x101);
    let ranked = ranked_lists(
        round,
        key,
        &[(w, yards, Dir::Smallest), (w, 0x14, Dir::Smallest)],
    );
    ranked
        .into_iter()
        .find(|s| round.records.get(s).is_some_and(|r| r.slot(yards) >= 1))
}
