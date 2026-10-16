//! AI module 9, the fleet tasks (`FUN_004cf290`, record class
//! `FUN_00474070`; `ghidra/notes/ai-order-emission.md`, "Fleet tasks"):
//! one task per own unblockaded headquarters system gathers free warships
//! there until its fleet's attack figure reaches the best attack class's
//! (twice that in a Headquarters Only game, `c+4 & 1`), and lets surplus
//! ships go.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::ai::{AIAction, FleetMoveReason};
use crate::ai_production::{bit, Dir, Round, Turn};
use crate::ai_records::{fleet, ship, sys, Marks};
use crate::ai_regions::{engaged, own_riders, random_system, set_system_bits, Words};
use crate::ai_strategy::Holder;
use crate::ids::{FleetKey, SystemKey};
use crate::uprising::Draws;

/// One fleet task (`T+`, `FUN_00474070`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetTask {
    /// `+0x18`.
    pub id: u32,
    /// `+0x1c`: the step state, 0-8.
    pub state: u8,
    /// `+0x24`: the tags of the ships it claimed.
    pub ships: BTreeSet<u32>,
    /// `+0x2c`: its fleet.
    pub fleet: Option<FleetKey>,
    /// `+0x30`: the headquarters system it holds.
    pub target: Option<SystemKey>,
    /// `+0x34`: the fleet's attack figure less the threshold.
    pub need: i64,
    /// `+0x38`: revalidate before the next step.
    pub dirty: bool,
}

/// One side's fleet tasks (context `+0x11c`) and module 9's driver
/// (`m+0x3c`, `+0x40`, `+0x44`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetTasks {
    pub tasks: Vec<FleetTask>,
    /// The list counter (`FUN_005f5440`), starting at 1.
    pub next_id: u32,
    pub state: u8,
    pub current: u32,
    pub next: u32,
}

impl Default for FleetTasks {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
            state: 0,
            current: 0,
            next: 0,
        }
    }
}

impl FleetTasks {
    /// Controller slot 8 (`FUN_004e69d0`, every mode): one task more while
    /// fewer than `wanted` (`c+0x224`), else the head task less while more,
    /// unless it holds a target (`FUN_0042edc0`).
    pub fn size(&mut self, round: &mut Round<'_>, marks: &mut Marks, wanted: i64) {
        let count = i64::try_from(self.tasks.len()).unwrap_or(i64::MAX);
        if count < wanted {
            self.tasks.push(FleetTask {
                id: self.next_id,
                ..FleetTask::default()
            });
            self.next_id += 1;
        } else if count > wanted && self.tasks.first().is_some_and(|t| t.target.is_none()) {
            let mut head = self.tasks.remove(0);
            head.release(round, marks);
        }
    }

    /// Module 9's turn (`FUN_004cf2c0`): its calls until it sets done
    /// (`m+0x20`), which an order or the end of a task's cycle does
    /// (`:25-35`), or no task is left (`:17-18`). An order in mid-cycle
    /// leaves the state at 3 and raises `m+0x34` (`:28`), so the next turn
    /// resumes the task.
    pub(crate) fn turn(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Turn {
        let threshold = threshold(round);
        let mut calls = TASK_CALLS;
        let turn = match self.turn_on(round, marks, threshold, &mut calls) {
            Call::Finished(Some(action)) => Turn::Order {
                action,
                resume: false,
            },
            Call::Order(action) => Turn::Order {
                action,
                resume: true,
            },
            Call::Finished(None) | Call::End | Call::Next => Turn::Finished,
        };
        crate::trace::record(crate::trace::TraceMask::AI, "fleet_task_turn", || {
            let (order, resume) = match &turn {
                Turn::Order { action, resume } => (Some(format!("{action:?}")), *resume),
                Turn::Spent | Turn::Finished => (None, false),
            };
            serde_json::json!({
                "tasks": self.tasks.len(),
                "attack_class_threshold": threshold,
                "task": self.current,
                "order": order,
                "resume": resume,
            })
        });
        turn
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
        threshold: i64,
        calls: &mut usize,
    ) -> Call {
        while *calls > 0 {
            *calls -= 1;
            match self.call(round, marks, threshold) {
                Call::Next => {}
                call => return call,
            }
        }
        Call::Next
    }

    /// `FUN_004cf2c0`: one driver step.
    fn call(&mut self, round: &mut Round<'_>, marks: &mut Marks, threshold: i64) -> Call {
        match self.state {
            2 => {
                // FUN_004cf350: the task after the last, else the head.
                let at = self
                    .tasks
                    .iter()
                    .position(|t| t.id == self.next)
                    .or_else(|| (!self.tasks.is_empty()).then_some(0));
                let Some(at) = at else {
                    self.current = 0;
                    self.next = 0;
                    self.state = 0;
                    return Call::End;
                };
                self.current = self.tasks[at].id;
                self.next = self.tasks.get(at + 1).map_or(0, |t| t.id);
                self.state = 3;
                Call::Next
            }
            3 => {
                // FUN_004cf3b0: a missing task is done.
                let Some(task) = self.tasks.iter_mut().find(|t| t.id == self.current) else {
                    self.state = 0;
                    return Call::Finished(None);
                };
                let (order, done) = task.step(round, marks, threshold);
                if done {
                    self.state = 0;
                    Call::Finished(order)
                } else {
                    order.map_or(Call::Next, Call::Order)
                }
            }
            _ => {
                self.state = 2;
                Call::Next
            }
        }
    }
}

/// The calls one task's cycle may take: states 0-8 and the driver's two.
/// port: the bound only stops a runaway; the original calls until done.
const TASK_CALLS: usize = 16;

/// `FUN_00474b90`: the attack figure a task's fleet must reach, the best
/// attack class's (`c+0x164`), twice that in a Headquarters Only game
/// (`c+4 & 1`).
fn threshold(round: &Round<'_>) -> i64 {
    let best = crate::ai_records::BestClasses::of(
        round.inputs.world,
        round.inputs.research,
        round.inputs.side,
    )
    .attack;
    if round.ctx & crate::ai_strategy::ctx::HEADQUARTERS_ONLY != 0 {
        2 * best
    } else {
        best
    }
}

/// What one driver step gave.
enum Call {
    End,
    Next,
    /// A task's cycle ended, with the order of its last state.
    Finished(Option<AIAction>),
    Order(AIAction),
}

impl FleetTask {
    /// `FUN_00474390`: one state; the order and whether the cycle is done.
    fn step(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        threshold: i64,
    ) -> (Option<AIAction>, bool) {
        if self.dirty {
            self.revalidate(round, marks);
            self.dirty = false;
        }
        let mut order = None;
        let mut done = false;
        self.state = match self.state {
            1 => {
                if self.choose_target(round, marks) {
                    2
                } else {
                    8
                }
            }
            2 => {
                order = self.split(round);
                3
            }
            3 => {
                self.need = self
                    .fleet
                    .and_then(|f| round.fleets.get(&f))
                    .map_or(0, |f| f.attack_all)
                    - threshold;
                4
            }
            4 => {
                // State 4 always passes to 5 (`FUN_00474390` tests its own
                // argument slot, which FUN_00474c00 only sets to 1).
                order = self.balance(round, marks);
                5
            }
            5 => {
                order = self.unload_regiments(round);
                6
            }
            6 => {
                order = self.unload_characters(round);
                7
            }
            7 => {
                order = self.move_fleet(round);
                done = true;
                0
            }
            8 => {
                self.release(round, marks);
                done = true;
                0
            }
            _ => {
                self.dirty = true;
                1
            }
        };
        if order.is_some() {
            self.dirty = true;
        }
        (order, done)
    }

    /// `FUN_004747b0`: drop a fleet that is no longer the task's, then the
    /// claimed ships that left it; with no fleet, adopt a new fleet that
    /// holds one of them.
    fn revalidate(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        let keep = self
            .fleet
            .and_then(|f| round.fleets.get(&f))
            .is_some_and(|f| f.has(fleet::OWN) && f.has(fleet::TASK) && !f.ships.is_empty());
        if !keep {
            self.release_fleet(round, marks);
        }
        for tag in self.ships.clone() {
            let Some(s) = round.ships.get(&tag).filter(|s| s.flags & ship::OWN != 0) else {
                self.free_ship(round, marks, tag);
                continue;
            };
            let container = s.fleet;
            match self.fleet {
                Some(f) if f == container => {}
                Some(_) => self.free_ship(round, marks, tag),
                None => {
                    if round
                        .fleets
                        .get(&container)
                        .is_some_and(|f| f.flags & fleet::UNADOPTED != 0)
                    {
                        self.adopt_fleet(round, marks, container);
                    }
                }
            }
        }
    }

    /// `FUN_00475fe0`: hold a fleet.
    fn adopt_fleet(&mut self, round: &mut Round<'_>, marks: &mut Marks, key: FleetKey) {
        self.fleet = Some(key);
        let m = marks.fleet_mut(key);
        m.flags = (m.flags & !fleet::UNADOPTED) | fleet::TASK;
        m.record = 0;
        m.plan = 0;
        if let Some(f) = round.fleets.get_mut(&key) {
            f.flags = (f.flags & !fleet::UNADOPTED) | fleet::TASK;
            f.record = 0;
            f.plan = 0;
        }
    }

    /// `FUN_00476080`: let the fleet go; it is new again.
    fn release_fleet(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        let Some(key) = self.fleet.take() else {
            return;
        };
        if let Some(f) = round.fleets.get_mut(&key) {
            f.flags = (f.flags & !fleet::TASK) | fleet::UNADOPTED;
            f.record = 0;
            f.plan = 0;
            let m = marks.fleet_mut(key);
            m.flags = (m.flags & !fleet::TASK) | fleet::UNADOPTED;
            m.record = 0;
            m.plan = 0;
        }
    }

    /// `FUN_00475e70`: claim a ship.
    fn claim_ship(&mut self, round: &mut Round<'_>, marks: &mut Marks, tag: u32) {
        let Some(s) = round.ships.get_mut(&tag) else {
            return;
        };
        s.flags = (s.flags & !ship::FREE) | ship::TASK;
        s.record = 0;
        s.plan = 0;
        if s.attack > 0 {
            self.need += s.attack;
        }
        let m = marks.ship_mut(tag);
        m.flags = (m.flags & !ship::FREE) | ship::TASK;
        m.record = 0;
        m.plan = 0;
        self.ships.insert(tag);
    }

    /// `FUN_00475f60`: let a claimed ship go; a ship no regional record
    /// holds is free again.
    fn free_ship(&mut self, round: &mut Round<'_>, marks: &mut Marks, tag: u32) {
        if !self.ships.remove(&tag) {
            return;
        }
        let Some(s) = round.ships.get_mut(&tag) else {
            return;
        };
        if s.record == 0 {
            let clear = ship::TASK | ship::DEMOTE | ship::FLAGSHIP;
            s.flags = (s.flags & !clear) | ship::FREE;
            s.plan = 0;
            let m = marks.ship_mut(tag);
            m.flags = (m.flags & !clear) | ship::FREE;
            m.record = 0;
            m.plan = 0;
        }
        self.need -= s.attack;
    }

    /// State 8 (`FUN_004742b0`): every ship, the fleet and the target go.
    fn release(&mut self, round: &mut Round<'_>, marks: &mut Marks) {
        for tag in self.ships.clone() {
            self.free_ship(round, marks, tag);
        }
        self.release_fleet(round, marks);
        if let Some(t) = self.target.filter(|t| round.records.contains_key(t)) {
            set_system_bits(round, marks, t, sys::TASK_CLAIM, false);
            self.target = None;
        }
    }

    /// State 1 (`FUN_004744b0`): keep the target while it is an own
    /// unblockaded headquarters system, else claim one in a sector, the
    /// sectors and their systems in random order.
    fn choose_target(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> bool {
        if let Some(t) = self.target {
            let Some(r) = round.records.get(&t) else {
                return false;
            };
            if r.s & sys::OWN != 0 && r.a & bit::HQ != 0 && r.a & bit::BLOCKADE == 0 {
                set_system_bits(round, marks, t, sys::TASK_CLAIM, true);
                return true;
            }
            set_system_bits(round, marks, t, sys::TASK_CLAIM, false);
            self.target = None;
        }
        // FUN_0041a1d0: sectors holding an own headquarters system, keyed
        // at random (`rand(20)`).
        let keyed: Vec<_> = round
            .sectors
            .iter()
            .filter(|s| s.flags & 0x4000_0000 != 0)
            .map(|s| s.key)
            .collect();
        let items = keyed
            .into_iter()
            .map(|k| (k, i64::from(round.rng.draw(19))))
            .collect();
        let mut sectors = Holder::new(vec![(items, Dir::Largest)], &mut round.rng);
        let words = Words::new(bit::HQ, 0, sys::OWN, bit::BLOCKADE, 0, sys::TASK_CLAIM);
        while let Some(sector) = sectors.pick() {
            if let Some(t) = random_system(round, sector, words) {
                self.target = Some(t);
                set_system_bits(round, marks, t, sys::TASK_CLAIM, true);
                return true;
            }
        }
        false
    }

    /// State 2 (`FUN_004749b0`): the fleet's damaged ships (`0x1000`,
    /// `0x800000`, set by `ai_records::ship_record`) leave it as a fleet in
    /// its system.
    fn split(&self, round: &Round<'_>) -> Option<AIAction> {
        let key = self.fleet?;
        let f = round.fleets.get(&key)?;
        if f.flags & 0x3000 == 0 || f.flags & 0xf000_0802 != 0 {
            return None;
        }
        let team: Vec<u32> = f
            .ships
            .iter()
            .copied()
            .filter(|t| {
                round
                    .ships
                    .get(t)
                    .is_some_and(|s| s.flags & 0x80_1000 != 0 && s.flags & ship::EN_ROUTE == 0)
            })
            .collect();
        (!team.is_empty()).then_some(AIAction::CreateFleet {
            fleet: key,
            ships: team,
            to_system: Some(f.location),
        })
    }

    /// State 4 (`FUN_00474c00`): short, gather free warships, the nearest
    /// new fleet's first, then those of the regional records' fleets; over,
    /// let one ship go.
    fn balance(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Option<AIAction> {
        if self
            .fleet
            .and_then(|f| round.fleets.get(&f))
            .is_some_and(|f| f.flags & fleet::EN_ROUTE != 0)
        {
            return None;
        }
        if self.need < 0 {
            if let Some(order) = self.gather(round, marks, false) {
                return Some(order);
            }
            if let Some(order) = self.gather(round, marks, true) {
                return Some(order);
            }
        }
        if self.need > 0 {
            return self.peel(round, marks);
        }
        None
    }

    /// `FUN_00475490` (new fleets nearest the target, `0x8005` and not
    /// `0xa`) and `FUN_00475890` (fleets of regional records by their
    /// plan entry, `0x8001` and not `0xf000000e`): the first candidate
    /// fleet with a free warship gives its free warships until the need is
    /// met.
    fn gather(
        &mut self,
        round: &mut Round<'_>,
        marks: &mut Marks,
        from_records: bool,
    ) -> Option<AIAction> {
        // The fleets in slot order, as the original walks its list.
        let world = round.inputs.world;
        let fleets = || world.fleets.keys().filter_map(|k| round.fleets.get(&k));
        let items: Vec<(FleetKey, i64)> = if from_records {
            fleets()
                .filter(|f| {
                    f.plan > 0
                        && f.has(fleet::OWN | fleet::READY)
                        && f.flags
                            & (0xf000_0000 | fleet::EN_ROUTE | fleet::UNADOPTED | fleet::TASK)
                            == 0
                })
                .map(|f| (f.key, i64::from(f.plan)))
                .collect()
        } else {
            let target = self.target?;
            let at = world.systems.get(target)?;
            let (x, y) = (i64::from(at.x), i64::from(at.y));
            fleets()
                .filter(|f| {
                    f.has(fleet::OWN | fleet::UNADOPTED | fleet::READY)
                        && f.flags & (fleet::EN_ROUTE | fleet::TASK) == 0
                })
                .filter_map(|f| {
                    let s = world.systems.get(f.location)?;
                    Some((
                        f.key,
                        (i64::from(s.x) - x).pow(2) + (i64::from(s.y) - y).pow(2),
                    ))
                })
                .collect()
        };
        let dir = if from_records {
            Dir::Largest
        } else {
            Dir::Smallest
        };
        let mut candidates = Holder::new(vec![(items, dir)], &mut round.rng);
        while let Some(source) = candidates.pick() {
            let tags = round.fleets.get(&source)?.ships.clone();
            let mut team = Vec::new();
            for tag in tags {
                if self.need >= 0 {
                    break;
                }
                let takes = round.ships.get(&tag).is_some_and(|s| {
                    s.flags & ship::FREE != 0
                        && s.flags & 0x80_3800 == 0
                        && s.flags & ship::COMBAT != 0
                });
                if takes {
                    self.claim_ship(round, marks, tag);
                    team.push(tag);
                }
            }
            if team.is_empty() {
                continue;
            }
            return Some(match self.fleet {
                Some(target) => AIAction::JoinFleet {
                    fleet: source,
                    ships: team,
                    target,
                },
                None => AIAction::CreateFleet {
                    fleet: source,
                    ships: team,
                    to_system: self.target,
                },
            });
        }
        None
    }

    /// `FUN_00475c70`: the first claimed ship, in id order, whose attack
    /// the surplus covers leaves the task fleet as a fleet of its own.
    fn peel(&mut self, round: &mut Round<'_>, marks: &mut Marks) -> Option<AIAction> {
        let tag = self.ships.iter().copied().find(|t| {
            round.ships.get(t).is_some_and(|s| {
                s.flags & (ship::EN_ROUTE | ship::INCOMPLETE) == 0
                    && !engaged(round, s)
                    && self.need >= s.attack
            })
        })?;
        let source = round.ships.get(&tag)?.fleet;
        self.free_ship(round, marks, tag);
        Some(AIAction::CreateFleet {
            fleet: source,
            ships: vec![tag],
            to_system: self.target,
        })
    }

    /// State 5 (`FUN_00474ce0`): in an own system the fleet's regiments
    /// disembark, those of the ship with the most first.
    fn unload_regiments(&self, round: &mut Round<'_>) -> Option<AIAction> {
        let key = self.fleet?;
        let f = round.fleets.get(&key)?;
        if f.regiments_active <= 0
            || !round
                .records
                .get(&f.location)
                .is_some_and(|r| r.s & sys::OWN != 0)
        {
            return None;
        }
        let location = f.location;
        let items: Vec<(u32, i64)> = f
            .ships
            .iter()
            .filter_map(|t| round.ships.get(t))
            .filter(|s| s.flags & ship::OWN != 0 && s.flags & (ship::EN_ROUTE | ship::FREE) == 0)
            .map(|s| (s.tag, s.regiments_active))
            .collect();
        let first = f.ships.first().copied();
        let mut ships = Holder::new(vec![(items, Dir::Largest)], &mut round.rng);
        let transport = round.inputs.transport;
        let mut troops = Vec::new();
        while let Some(tag) = ships.pick() {
            // A regiment held on the fleet rides its first ship
            // (`crate::ai_records`).
            troops.extend(transport.cargo(key).iter().copied().filter(|t| {
                let carrier = transport.carrier(*t);
                carrier == tag || (carrier == 0 && first == Some(tag))
            }));
        }
        (!troops.is_empty()).then_some(AIAction::MoveRegiments {
            troops,
            to: crate::troop_transport::RegimentTarget::System(location),
        })
    }

    /// State 6 (`FUN_00474ff0`): in an own system the side's characters
    /// aboard set down.
    fn unload_characters(&self, round: &Round<'_>) -> Option<AIAction> {
        let key = self.fleet?;
        let f = round.fleets.get(&key)?;
        if f.flags & fleet::OWN_CHARACTER == 0
            || f.flags & fleet::EN_ROUTE != 0
            || !round
                .records
                .get(&f.location)
                .is_some_and(|r| r.s & sys::OWN != 0)
        {
            return None;
        }
        let characters = own_riders(round.inputs.world, key, round.inputs.side);
        (!characters.is_empty()).then_some(AIAction::MoveCharacters {
            characters,
            to_system: f.location,
        })
    }

    /// State 7 (`FUN_00475330`): the fleet goes to the target.
    fn move_fleet(&self, round: &Round<'_>) -> Option<AIAction> {
        let target = self.target?;
        if round
            .records
            .get(&target)
            .is_none_or(|r| r.a & bit::BLOCKADE != 0)
        {
            return None;
        }
        let key = self.fleet?;
        let f = round.fleets.get(&key)?;
        (f.location != target && f.flags & fleet::EN_ROUTE == 0).then_some(AIAction::MoveFleet {
            fleet: key,
            to_system: target,
            reason: FleetMoveReason::Reinforce,
            troops: Vec::new(),
        })
    }
}
