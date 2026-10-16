//! The AI's character tasks (module 12; `ghidra/notes/ai-order-emission.md`,
//! "Character tasks (module 12)"). Class G: a General for each system the
//! strategy controller tags, and the Emperor brought to Coruscant. Class I:
//! an Admiral, a General and a Commander for each regional record
//! (`crate::ai_regions`), carried to its flagship and given the rank there.
//!
//! port: the controller (`FUN_004eaaa0`) proposes once per six calls of
//! module 13's first phase, and module 12 consumes the proposal at its
//! next turn. The port computes the proposal at each module-12 turn
//! (`CharacterTasks::turn`).

use serde::{Deserialize, Serialize};

use crate::ai::AIAction;
use crate::ai_production::{Round, Turn};
use crate::ai_records::fleet;
use crate::ai_regions::Regions;
use crate::dat::Faction;
use crate::ids::{CharacterKey, SystemKey};
use crate::mission_planning::{Planner, EMPEROR, EN_ROUTE, NOT_SENT, ON_MISSION, PRISONER, RANKED};
use crate::officer_rank::CommandKind;
use crate::world::GameWorld;

/// Coruscant's system index: `0x90000109`, the system controller state 6
/// tags while the side holds it (`FUN_004e5870`).
const CORUSCANT: u32 = 0x109;

/// A task's kind (slot `+0x18`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskKind {
    /// Kind 1 (`FUN_0047a430`): a General at a tagged system.
    General,
    /// Kind 6 (`FUN_00479a60`): the Emperor to Coruscant, Empire only.
    Emperor,
    /// Class I kind 3 (`FUN_0047a3c0`): the record's Admiral.
    Admiral,
    /// Class I kind 4 (`FUN_0047a000`): the record's General.
    GeneralOfRecord,
    /// Class I kind 5 (`FUN_00479f70`): the record's Commander.
    Commander,
}

impl TaskKind {
    /// A class I kind from its number in a record request (`R+0xe0`,
    /// `FUN_004c86b0` -> `FUN_0042efe0`).
    const fn of_record(kind: u8) -> Option<Self> {
        match kind {
            3 => Some(Self::Admiral),
            4 => Some(Self::GeneralOfRecord),
            5 => Some(Self::Commander),
            _ => None,
        }
    }

    /// The kind's number, its record counter (`FUN_00479520`), the
    /// capability bit it asks of a character (`K+0x50`) and the rank it
    /// gives (Command `0x262`, `0x263`, `0x261`); `None` for class G.
    const fn class_i(self) -> Option<(u8, usize, u32, CommandKind)> {
        match self {
            Self::Admiral => Some((3, 0, 0x100_0000, CommandKind::Admiral)),
            Self::GeneralOfRecord => Some((4, 1, 0x400_0000, CommandKind::General)),
            Self::Commander => Some((5, 2, 0x200_0000, CommandKind::Commander)),
            Self::General | Self::Emperor => None,
        }
    }
}

/// Class I task flag `K+0x20` bit 1: a character held (`FUN_004bc6d0`).
const HELD: u8 = 0x1;
/// The flagship and staging ids are valid (`FUN_004bc5d0`).
const TARGET: u8 = 0x2;
/// Ranked aboard the flagship (`FUN_004bc6d0`, `FUN_004bca20`).
const RANKED_ABOARD: u8 = 0x4;
/// Aboard the flagship's fleet (`FUN_004bc6d0`, `FUN_004bca20`).
const ABOARD: u8 = 0x8;
/// Waiting at the staging system (`FUN_004bca20`).
const WAITING: u8 = 0x10;

/// One character task (`FUN_0042efe0`; class I `FUN_004bc290`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterTask {
    /// The id the task list assigns.
    pub id: u32,
    pub kind: TaskKind,
    /// Class G: the target system (`+0x28`); set means the valid-target
    /// flag `0x2`. Class I: the staging system (`+0x48`).
    pub target: Option<SystemKey>,
    /// The claimed character (`+0x2c`).
    pub character: Option<CharacterKey>,
    /// Flag `0x80000000` (`FUN_0042ef90`): released at its next step.
    pub dropped: bool,
    /// Class I: the regional record (`+0x54`, 0 none).
    #[serde(default)]
    pub record: u32,
    /// Class I: the record's flagship tag (`+0x28`, 0 none).
    #[serde(default)]
    pub flagship: u32,
    /// Class I: the flags `+0x20` (`HELD` ..).
    #[serde(default)]
    pub flags: u8,
}

impl CharacterTask {
    /// Whether the controller's drop (`FUN_004e57e0`) passes it over: its
    /// valid-target flag `0x2`.
    const fn has_target(&self) -> bool {
        match self.kind {
            TaskKind::General | TaskKind::Emperor => self.target.is_some(),
            _ => self.flags & TARGET != 0,
        }
    }
}

/// One side's character tasks (context `+0xec`), the controller's counts
/// and module 12's record cursor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterTasks {
    pub tasks: Vec<CharacterTask>,
    /// Kind 1 and kind 6 tasks ever created (`+0x35c`, `+0x360`). Only the
    /// constructor, `FUN_0041b5d0` and the save codec write them, so a
    /// released task is never replaced while its count has reached the cap.
    pub created: [u32; 2],
    pub next_id: u32,
    /// `m+0x44`: the record the next request scan starts from (0 the head).
    #[serde(default)]
    pub record_cursor: u32,
    /// `m+0x4c`: the task the next turn's scan starts from (0 the head).
    #[serde(default)]
    pub task_cursor: u32,
}

impl Default for CharacterTasks {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            created: [0; 2],
            // The list counter (`FUN_005f5440`) starts at 1; 0 is no task.
            next_id: 1,
            record_cursor: 0,
            task_cursor: 0,
        }
    }
}

impl CharacterTasks {
    /// The characters the tasks hold.
    pub fn held(&self) -> impl Iterator<Item = CharacterKey> + '_ {
        self.tasks.iter().filter_map(|task| task.character)
    }

    /// Whether a task holds `character`.
    #[must_use]
    pub fn holds(&self, character: CharacterKey) -> bool {
        self.held().any(|held| held == character)
    }

    /// Controller slot 7 (`FUN_004eaaa0`), the request it stores
    /// (`FUN_004e57e0`) and module 12 state 1 (`FUN_004c85b0`).
    fn propose(&mut self, world: &GameWorld, side: Faction) {
        let cap = u32::try_from(tagged(world, side).len()).unwrap_or(u32::MAX);
        let [generals, emperors] = self.created;
        let mut request = None;
        let mut drop = false;
        if generals < cap {
            request = Some(TaskKind::General);
        } else if generals > cap {
            drop = true;
        }
        // hyp: context +0xa4, copied from +0x14c (FUN_004182a0), is
        // untraced and read as 0.
        if side == Faction::Empire {
            let near = holds_in_coruscant_sector(world, side);
            if request.is_none() && near && emperors == 0 {
                request = Some(TaskKind::Emperor);
            }
            if !drop && emperors == 1 && !near {
                drop = true;
            }
        }
        // FUN_004e57e0: the first task without a valid target.
        if drop {
            if let Some(task) = self.tasks.iter_mut().find(|task| !task.has_target()) {
                task.dropped = true;
            }
        }
        if let Some(kind) = request {
            self.tasks.push(CharacterTask {
                id: self.next_id,
                kind,
                target: None,
                character: None,
                dropped: false,
                record: 0,
                flagship: 0,
                flags: 0,
            });
            self.next_id += 1;
            // FUN_0041b5d0: index 0 for kind 1, 1 for kind 6.
            self.created[usize::from(kind == TaskKind::Emperor)] += 1;
        }
    }

    /// One pass of the class G machine (`FUN_004bbb00`): release (state
    /// 1), target (state 3), character (state 2), order (state 4).
    fn step(
        &mut self,
        id: u32,
        planner: &mut Planner<'_>,
        world: &GameWorld,
        side: Faction,
        actions: &mut Vec<AIAction>,
    ) {
        let Some(at) = self.tasks.iter().position(|task| task.id == id) else {
            return;
        };
        let flags =
            |character: CharacterKey, planner: &Planner<'_>| planner.record_flags(character);
        // FUN_004bba90: not while its character is on a mission or en route.
        if let Some(character) = self.tasks[at].character {
            if flags(character, planner).is_some_and(|f| f & (ON_MISSION | EN_ROUTE) != 0) {
                return;
            }
        }
        if self.tasks[at].dropped {
            let task = self.tasks.remove(at);
            // FUN_004bbba0: free the character, and take its rank when it
            // is neither on a mission nor en route.
            if let Some(character) = task.character {
                let ranked_idle = flags(character, planner)
                    .is_some_and(|f| f & RANKED != 0 && f & (ON_MISSION | EN_ROUTE) == 0);
                if ranked_idle {
                    actions.push(AIAction::GiveCommand {
                        character,
                        kind: CommandKind::None,
                    });
                }
            }
            return;
        }
        let claimed = self.claimed(id);
        let task = &mut self.tasks[at];
        task.target = match task.kind {
            // FUN_0047a480, FUN_0047a870. hyp: the pick's sector and system
            // values (+0x26, +0x21) are untraced; the first free tagged
            // system in id order is taken.
            TaskKind::General => {
                let tags = tagged(world, side);
                task.target
                    .filter(|target| tags.contains(target))
                    .or_else(|| tags.into_iter().find(|system| !claimed.contains(system)))
            }
            // FUN_00479ad0, FUN_00479e30.
            TaskKind::Emperor => coruscant(world),
            TaskKind::Admiral | TaskKind::GeneralOfRecord | TaskKind::Commander => return,
        };
        // FUN_0047a5b0, FUN_00479c00: a prisoner, or an Emperor no longer
        // marked, is freed.
        if let Some(character) = task.character {
            let keep = flags(character, planner).is_some_and(|f| {
                f & PRISONER == 0 && (task.kind == TaskKind::General || f & EMPEROR != 0)
            });
            if !keep {
                task.character = None;
            }
        }
        if task.character.is_none() {
            let open = task.target.filter(|&target| match task.kind {
                TaskKind::General => !claimed.contains(&target),
                TaskKind::Emperor => held_by(world, target, side),
                TaskKind::Admiral | TaskKind::GeneralOfRecord | TaskKind::Commander => false,
            });
            task.target = open;
            task.character = match (task.kind, open) {
                (TaskKind::General, Some(target)) => planner.take_general(target),
                (TaskKind::Emperor, Some(_)) => planner.take_emperor(),
                _ => None,
            };
        }
        let (Some(character), Some(target)) = (task.character, task.target) else {
            return;
        };
        // FUN_004bbc50: nothing once the character holds a rank or is
        // otherwise unavailable.
        if flags(character, planner).is_none_or(|f| f & NOT_SENT != 0) {
            return;
        }
        let Some(c) = world.characters.get(character) else {
            return;
        };
        if c.current_fleet.is_none() && c.current_system == Some(target) {
            // FUN_004bbf50: Command General at the target; kind 6 only moves.
            if task.kind == TaskKind::General {
                actions.push(AIAction::GiveCommand {
                    character,
                    kind: CommandKind::General,
                });
            }
        } else {
            // FUN_004bc040.
            actions.push(AIAction::MoveCharacter {
                character,
                to_system: target,
            });
        }
    }

    /// The systems other kind 1 tasks claim (`+0x30` bit `0x20000000`):
    /// a kind 1 task's target while it holds a character (`FUN_0047a5b0`
    /// sets it on acceptance; `FUN_0047a850` and release clear it).
    fn claimed(&self, except: u32) -> Vec<SystemKey> {
        self.tasks
            .iter()
            .filter(|task| task.id != except && task.kind == TaskKind::General)
            .filter(|task| task.character.is_some())
            .filter_map(|task| task.target)
            .collect()
    }
}

impl CharacterTasks {
    /// Module 12's turn (`FUN_004c84d0`): the controller's proposal (state
    /// 1, `FUN_004c85b0`), one record request (states 2 and 3), the next
    /// runnable task from the task cursor, without wrapping (state 4,
    /// `FUN_004c8740`), and one pass of it (state 5). No runnable task left
    /// resets the cursor and finishes the turn (`:24-28`). A pass ends with
    /// its order (`FUN_004bbb00.c:31-34`, `FUN_004bc480.c:16-17, 31-34`), so
    /// no order resumes.
    ///
    /// port: the proposal is computed here, not stored by the controller.
    /// The claim of a character (`FUN_004021b0`) and the release of a task
    /// (`FUN_0041b600`) mark the round dirty (`FUN_004184f0`).
    pub(crate) fn turn(
        &mut self,
        round: &mut Round<'_>,
        regions: &mut Regions,
        planner: &mut Planner<'_>,
        side: Faction,
    ) -> Turn {
        let world = round.inputs.world;
        self.propose(world, side);
        self.take_request(regions);
        // FUN_004c8740: from the cursor's task, else the head; the task
        // runs unless FUN_004bba90 finds its character on a mission or en
        // route (record +0x30 & 0xa0).
        let from = self
            .tasks
            .iter()
            .position(|t| t.id == self.task_cursor)
            .unwrap_or(0);
        let runnable = |task: &CharacterTask| {
            task.character
                .and_then(|c| planner.record_flags(c))
                .is_none_or(|f| f & (ON_MISSION | EN_ROUTE) == 0)
        };
        let Some(at) = (from..self.tasks.len()).find(|&i| runnable(&self.tasks[i])) else {
            self.task_cursor = 0;
            return Turn::Finished;
        };
        let id = self.tasks[at].id;
        let before = self.tasks[at].character;
        self.task_cursor = self.tasks.get(at + 1).map_or(0, |t| t.id);
        let mut actions = Vec::new();
        if self.tasks[at].kind.class_i().is_some() {
            self.step_class_i(id, round, regions, planner, &mut actions);
        } else {
            self.step(id, planner, world, side, &mut actions);
        }
        match self.tasks.iter().find(|t| t.id == id) {
            None => round.dirty = true,
            Some(task) if task.character.is_some() && task.character != before => {
                round.dirty = true;
            }
            Some(_) => {}
        }
        let turn = actions
            .into_iter()
            .next()
            .map_or(Turn::Finished, |action| Turn::Order {
                action,
                resume: false,
            });
        crate::trace::record(crate::trace::TraceMask::AI, "character_task_turn", || {
            let order = match &turn {
                Turn::Order { action, .. } => Some(format!("{action:?}")),
                Turn::Spent | Turn::Finished => None,
            };
            serde_json::json!({
                "task": id,
                "next": self.task_cursor,
                "order": order,
                "dirty": round.dirty,
            })
        });
        turn
    }

    /// `FUN_004c8630`: from the cursor, the first record with a request or
    /// a cancel, without wrapping; then `FUN_004c86b0`: the cancelled task
    /// is dropped and leaves its record (`FUN_0042ef90`, `FUN_00476de0`),
    /// the requested task is made and joins it (`FUN_00476d10`).
    fn take_request(&mut self, regions: &mut Regions) {
        let from = regions
            .records
            .iter()
            .position(|r| r.id == self.record_cursor)
            .unwrap_or(0);
        let Some(at) = (from..regions.records.len())
            .find(|&i| regions.records[i].want != 0 || regions.records[i].drop != 0)
        else {
            self.record_cursor = 0;
            return;
        };
        self.record_cursor = regions.records.get(at + 1).map_or(0, |r| r.id);
        let record = &mut regions.records[at];
        if record.drop != 0 {
            let id = std::mem::take(&mut record.drop);
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.dropped = true;
                if let Some((_, slot, _, _)) = task.kind.class_i() {
                    record.officers[slot] = record.officers[slot].saturating_sub(1);
                }
                task.record = 0;
            }
            record.tasks.retain(|t| t.id != id);
        }
        if let Some(kind) = TaskKind::of_record(std::mem::take(&mut record.want)) {
            let Some((number, slot, _, _)) = kind.class_i() else {
                return;
            };
            let id = self.next_id;
            self.next_id += 1;
            self.tasks.push(CharacterTask {
                id,
                kind,
                target: None,
                character: None,
                dropped: false,
                record: record.id,
                flagship: 0,
                flags: 0,
            });
            record.tasks.push(crate::ai_regions::RecordTask {
                id,
                kind: number,
                flags: 0,
            });
            record.officers[slot] += 1;
        }
    }

    /// One pass of the class I machine (`FUN_004bc480`): release (state
    /// 1), the record's staging and flagship (state 3), the character
    /// (state 2), the order (state 4).
    fn step_class_i(
        &mut self,
        id: u32,
        round: &Round<'_>,
        regions: &mut Regions,
        planner: &mut Planner<'_>,
        actions: &mut Vec<AIAction>,
    ) {
        let Some(at) = self.tasks.iter().position(|task| task.id == id) else {
            return;
        };
        let world = round.inputs.world;
        let flags = |c: CharacterKey, planner: &Planner<'_>| planner.record_flags(c);
        // FUN_004bba90: not while its character is on a mission or en route.
        if let Some(c) = self.tasks[at].character {
            if flags(c, planner).is_some_and(|f| f & (ON_MISSION | EN_ROUTE) != 0) {
                return;
            }
        }
        // FUN_00477190 drops every task of a record it clears.
        let listed = regions
            .records
            .iter()
            .any(|r| r.id == self.tasks[at].record && r.tasks.iter().any(|t| t.id == id));
        if self.tasks[at].dropped || !listed {
            let task = self.tasks.remove(at);
            // FUN_004bc520: free the character, and take its rank when it
            // is neither on a mission nor en route.
            if let Some(c) = task.character {
                let ranked_idle = flags(c, planner)
                    .is_some_and(|f| f & RANKED != 0 && f & (ON_MISSION | EN_ROUTE) == 0);
                if ranked_idle {
                    actions.push(AIAction::GiveCommand {
                        character: c,
                        kind: CommandKind::None,
                    });
                }
            }
            return;
        }
        let Some((_, _, caps, rank)) = self.tasks[at].kind.class_i() else {
            return;
        };
        let Some(record) = regions
            .records
            .iter_mut()
            .find(|r| r.id == self.tasks[at].record)
        else {
            return;
        };
        let task = &mut self.tasks[at];
        // FUN_004bc5d0: the record's staging system and flagship.
        task.target = record.staging;
        task.flagship = record.flagship;
        task.flags &= !TARGET;
        if task.flagship != 0 && task.target.is_some() {
            task.flags |= TARGET;
        }
        // FUN_004bc6d0, FUN_0047a050: keep the character unless a prisoner.
        task.flags &= !(HELD | RANKED_ABOARD | ABOARD | WAITING);
        let flagship_fleet = round
            .ships
            .get(&task.flagship)
            .filter(|_| task.flagship != 0)
            .map(|s| s.fleet);
        if let Some(c) = task.character {
            match flags(c, planner) {
                Some(f) if f & PRISONER == 0 => {
                    task.flags |= HELD;
                    let aboard = flagship_fleet.is_some()
                        && world.characters.get(c).and_then(|c| c.current_fleet) == flagship_fleet;
                    if aboard {
                        task.flags |= ABOARD;
                        if f & RANKED != 0 {
                            task.flags |= RANKED_ABOARD;
                        }
                    }
                }
                Some(_) => task.character = None,
                None => {}
            }
        }
        if task.flags & HELD == 0 {
            // List B ranks by nearness to the staging system, else to the
            // sector's first system (FUN_00432120).
            let near = task.target.or_else(|| {
                record
                    .sector
                    .and_then(|k| world.sectors.get(k))
                    .and_then(|s| s.systems.first().copied())
            });
            task.character = near.and_then(|near| planner.take_officer(caps, near));
            if task.character.is_some() {
                task.flags |= HELD;
            }
        }
        if task.flags & HELD != 0 {
            if let Some(order) = issue(task, round, flagship_fleet, rank, planner) {
                actions.push(order);
            }
        }
        if let Some(t) = record.tasks.iter_mut().find(|t| t.id == id) {
            t.flags = task.flags;
        }
    }
}

/// State 4 (`FUN_004bca20`): the rank aboard the flagship's fleet, else
/// the way there: onto that fleet in its system, else to the staging
/// system, else wait there.
fn issue(
    task: &mut CharacterTask,
    round: &Round<'_>,
    flagship_fleet: Option<crate::ids::FleetKey>,
    rank: CommandKind,
    planner: &Planner<'_>,
) -> Option<AIAction> {
    let character = task.character?;
    if planner.record_flags(character)? & NOT_SENT != 0 {
        return None;
    }
    let c = round.inputs.world.characters.get(character)?;
    let staging = task.target;
    // FUN_004bc040 sends nothing toward a null staging system.
    let to_staging = || {
        staging.map(|to_system| AIAction::MoveCharacter {
            character,
            to_system,
        })
    };
    let Some(flagship_fleet) = flagship_fleet else {
        // B: no flagship.
        let staging = staging?;
        if c.current_fleet.is_none() && c.current_system == Some(staging) {
            task.flags |= WAITING;
            return None;
        }
        return to_staging();
    };
    // A: aboard the flagship's fleet.
    if c.current_fleet == Some(flagship_fleet) {
        task.flags |= RANKED_ABOARD;
        return Some(AIAction::GiveCommand {
            character,
            kind: rank,
        });
    }
    let f = round.fleets.get(&flagship_fleet)?;
    let board = |task: &mut CharacterTask| {
        (f.flags & fleet::EN_ROUTE == 0).then(|| {
            task.flags |= ABOARD;
            AIAction::MoveCharacterToFleet {
                character,
                fleet: flagship_fleet,
            }
        })
    };
    // C: aboard another fleet, or standing in a system.
    if let Some(own) = c.current_fleet {
        let o = round.fleets.get(&own)?;
        if o.flags & fleet::EN_ROUTE != 0 {
            return None;
        }
        if o.location != f.location {
            return to_staging();
        }
        return board(task);
    }
    let here = c.current_system?;
    if here != f.location {
        if staging == Some(here) {
            task.flags |= WAITING;
            return None;
        }
        return to_staging();
    }
    board(task)
}

fn coruscant(world: &GameWorld) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find_map(|(key, s)| (s.dat_id.index() == CORUSCANT).then_some(key))
}

/// Whether `side` holds `system` (record `+0x30` bit 1).
fn held_by(world: &GameWorld, system: SystemKey, side: Faction) -> bool {
    world
        .systems
        .get(system)
        .is_some_and(|s| s.control.is_controlled_by(side))
}

/// The systems the controller tags for `side` (`+0x30` bit `0x10000000`,
/// state 6 `FUN_004e5870`), in id order: Coruscant while the side holds
/// it, and for the Alliance the held system with its headquarters.
fn tagged(world: &GameWorld, side: Faction) -> Vec<SystemKey> {
    let mut systems: Vec<_> = world
        .systems
        .iter()
        .filter(|(_, s)| {
            let coruscant = s.dat_id.index() == CORUSCANT;
            let hq = side == Faction::Alliance && s.is_headquarters && !coruscant;
            (coruscant || hq) && s.control.is_controlled_by(side)
        })
        .map(|(key, s)| (s.dat_id.raw(), key))
        .collect();
    systems.sort_unstable();
    systems.into_iter().map(|(_, key)| key).collect()
}

/// Context flag `+4` bit `0x80` (`FUN_0041b7e0`): the side holds a system
/// in Coruscant's sector.
fn holds_in_coruscant_sector(world: &GameWorld, side: Faction) -> bool {
    let Some(sector) = coruscant(world).and_then(|key| world.systems.get(key)) else {
        return false;
    };
    world.sectors.get(sector.sector).is_some_and(|sector| {
        sector
            .systems
            .iter()
            .any(|&system| held_by(world, system, side))
    })
}
