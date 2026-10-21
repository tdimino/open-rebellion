//! The sector-wide support shifts (`ghidra/notes/support-shifts.md`).
//!
//! A shift is a record (`FUN_00562ae0`): a system family range, a gate side
//! and limit, a direction side and an amount. Posting it to a sector
//! (`FUN_00518ac0` -> `FUN_00518b20` -> `FUN_00518be0`) tests each system's
//! family, its populated bit, `+0x88` bit 3 and the gate side's support
//! against the limit, and queues event `0x373` with the system and the
//! amount, negated unless the direction is the Alliance. The event fires in
//! the same frame step's phase 5 (`FUN_0051df30` serves the event queue
//! before returning), through `FUN_00562fb0` -> `FUN_0050c9f0` on the
//! Alliance's support, each fire clamped on its own.
//!
//! The port queues the events on `GameWorld::support_events` and fires them
//! ([`fire`]) at the end of each operation that posts: a strike, the
//! economy's control pass, an occupation, the blockade events, the missions
//! and the uprising timers. So a gate reads support before any shift of the
//! same operation applies, and the shifts apply in posting order, as in the
//! original's step.
//!
//! Every change of support runs the setter's hook (`FUN_00509c30` -> slot
//! `+0x1d0`, `FUN_005109f0`), whose `FUN_0050b610` rebuilds the control
//! word at once ([`support_changed`]); a new holder raises event `0x31b`,
//! which joins the same queue ahead of the shifts still queued, so a shift
//! that turns a system posts that change's own shift in the same step.
//!
//! The shifts posted here: a control change (`FUN_00517aa0`,
//! `FUN_0055de50`) and Coruscant's own (`FUN_00517be0`, `FUN_0055e010`), a
//! blockade's end (`FUN_00517ee0`, `FUN_0055dbf0`) and the Alliance HQ's
//! removal (`FUN_00517cd0`, `FUN_0055e1f0`). The bombardment's finalize posts
//! its own records (`bombardment::finalize`).

use std::ops::Range;

use crate::blockade::{BlockadeState, UnitLoss};
use crate::dat::Faction;
use crate::ids::{CapitalShipKey, DatId, FighterKey, SectorKey, SystemKey};
use crate::world::{ControlKind, GameWorld};

/// The default system family range of a record (`FUN_00562ba0`,
/// `FUN_0055de00`): families `0x90`-`0x97`, every system.
pub const ALL_SYSTEMS: Range<u8> = 0x90..0x98;
/// Coruscant's system index (system `+0x18`, `FUN_00517aa0`).
pub const CORUSCANT: u32 = 0x109;

/// GNPRTB `0x1407` (`DAT_006bb5d0`, `FUN_0055cb60.c:12`): the HQ removal's
/// amount (`FUN_0055e1f0`).
const GNPRTB_HQ_AMOUNT: u16 = 0x1407;
/// GNPRTB `0x1408` (`DAT_006bb6b8`, `FUN_0055cb60.c:13`): its gate limit.
const GNPRTB_HQ_LIMIT: u16 = 0x1408;
/// GNPRTB `0x1420` (`DAT_006bb5f0`, `FUN_0055cb60.c:29`): the blockade
/// shift's divisor (`FUN_0055dbf0`).
const GNPRTB_BLOCKADE_DIVISOR: u16 = 0x1420;
/// SDPRTB `0x1410` (`DAT_006bb5e0`/`DAT_006bb604`, `FUN_0055cb60.c:44-45`):
/// the blockade shift's gate limit, by gate side.
const SDPRTB_BLOCKADE_LIMIT: u16 = 0x1410;
/// SDPRTB `0x1411` (`DAT_006bb6ec`/`DAT_006bb690`, `:46-47`): its cap, by
/// direction side.
const SDPRTB_BLOCKADE_CAP: u16 = 0x1411;
/// SDPRTB `0x1412` (`DAT_006bb6fc`/`DAT_006bb62c`, `:48-49`): the stronger
/// side's weight base.
const SDPRTB_STRONG_BASE: u16 = 0x1412;
/// SDPRTB `0x1413` (`DAT_006bb6f0`/`DAT_006bb6e0`, `:50-51`): its slope.
const SDPRTB_STRONG_SLOPE: u16 = 0x1413;
/// SDPRTB `0x1414` (`DAT_006bb6c8`/`DAT_006bb634`, `:52-53`): the weaker
/// side's weight base.
const SDPRTB_WEAK_BASE: u16 = 0x1414;
/// SDPRTB `0x1415` (`DAT_006bb678`/`DAT_006bb5dc`, `:54-55`): its slope.
const SDPRTB_WEAK_SLOPE: u16 = 0x1415;
/// SDPRTB `0x1420` (`DAT_006bb660`/`DAT_006bb67c`, `:56-57`): a control
/// change's amount per side involved (`FUN_0055de50`).
const SDPRTB_CONTROL_AMOUNT: u16 = 0x1420;
/// SDPRTB `0x1421` (`DAT_006bb6e4`/`DAT_006bb608`, `:58-59`): its gate
/// limit, by the new side.
const SDPRTB_CONTROL_LIMIT: u16 = 0x1421;
/// SDPRTB `0x1422` (`DAT_006bb668`/`DAT_006bb60c`, `:60-61`): Coruscant's
/// amount per side involved (`FUN_0055e010`).
const SDPRTB_CORUSCANT_AMOUNT: u16 = 0x1422;
/// SDPRTB `0x1423` (`DAT_006bb684`/`DAT_006bb5d4`, `:62-63`): its limit.
const SDPRTB_CORUSCANT_LIMIT: u16 = 0x1423;
/// SDPRTB `0x1440`/`0x1441` (`:70-73`): a change whose new word alone has
/// troop bits, amount and limit.
const SDPRTB_TROOPS_ARRIVE: (u16, u16) = (0x1440, 0x1441);
/// SDPRTB `0x1442`/`0x1443` (`:74-77`): one whose old word alone has them.
const SDPRTB_TROOPS_LEAVE: (u16, u16) = (0x1442, 0x1443);
/// SDPRTB `0x1444`/`0x1445` (`:78-81`): one whose words both have them.
const SDPRTB_TROOPS_STAY: (u16, u16) = (0x1444, 0x1445);

/// A shift record (`FUN_00562ae0`). Its owner filter (`[0]`) is 0 in every
/// record ported here, so the port keeps none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftRecord {
    /// `[1]`, `[2]`: the system families shifted.
    pub families: Range<u8>,
    /// `[3]`: the side whose support the gate reads.
    pub gate_side: Faction,
    /// `[4]`: a system shifts while that support is below it.
    pub gate_limit: i32,
    /// `[5]`: the side the shift favours when the amount is positive.
    pub direction: Faction,
    /// `[6]`: the amount; 0 posts nothing.
    pub amount: i32,
}

/// The operation that posted a shift. port: telemetry only; the original's
/// event `0x373` carries no cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftCause {
    /// A control change (`FUN_00517aa0`).
    ControlChange,
    /// Coruscant changing hands (`FUN_00517be0`).
    Coruscant,
    /// A blockade's end (`FUN_00517ee0`).
    BlockadeEnd,
    /// The Alliance HQ's removal (`FUN_00517cd0`).
    Headquarters,
    /// A bombardment's sector or galaxy shift (`FUN_0058e320`).
    Bombardment,
}

/// One posted event `0x373`: `amount` points to the Alliance's support at
/// `system` (`FUN_00562fb0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostedShift {
    pub system: SystemKey,
    pub amount: i32,
    pub cause: ShiftCause,
}

/// A holder change ([`rebuild_control`]): event `0x31b` (`FUN_00510d70`)
/// with the system, its holder before and its control after, and the troop
/// bits of the old and the new word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlRerun {
    pub system: SystemKey,
    pub previous: ControlKind,
    pub control: ControlKind,
    pub troops: (bool, bool),
}

/// Slot `+0x28` of events `0x301`, `0x320` and `0x31b` (`FUN_0048af30`).
const PRIORITY_HOLDER: u16 = 0x10;
/// Slot `+0x28` of event `0x373` (`FUN_0048bc60`).
const PRIORITY_SHIFT: u16 = 0x100;

/// One event waiting for phase 5, which serves the queue until it is empty
/// (`FUN_0051df30`): the lowest priority first, and within one priority in
/// the order raised ([`raise`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueuedEvent {
    /// Event `0x373`.
    Shift(PostedShift),
    /// Event `0x31b`.
    Control(ControlRerun),
    /// Event `0x301` or `0x320` for a regiment (families `0x10`-`0x13`)
    /// at the system: the system's slot `+0xc8` (`FUN_00508660`) reruns
    /// `FUN_0050b610` ([`regiments_changed`]).
    Regiments(SystemKey),
}

impl QueuedEvent {
    /// The event's slot `+0x28`, which orders the queue (`FUN_0054ee90`).
    const fn priority(self) -> u16 {
        match self {
            Self::Shift(_) => PRIORITY_SHIFT,
            Self::Control(_) | Self::Regiments(_) => PRIORITY_HOLDER,
        }
    }
}

/// Queue `event` on `GameWorld::support_events` (queue `+0xb0`): an insert
/// (`FUN_005f61c0`) goes ahead of a node only when its priority is strictly
/// lower (`FUN_0054ee90`), so it lands behind every queued event of its
/// priority or lower and ahead of every higher one, even while [`fire`]
/// drains the queue.
fn raise(world: &mut GameWorld, event: QueuedEvent) {
    let at = world
        .support_events
        .iter()
        .position(|queued| event.priority() < queued.priority())
        .unwrap_or(world.support_events.len());
    world.support_events.insert(at, event);
}

/// What [`fire`] did: each shift with the points it moved, and each holder
/// change, in the order they ran.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Fired {
    pub shifts: Vec<AppliedShift>,
    pub controls: Vec<ControlRerun>,
}

/// A fired shift: the posted amount and the Alliance points it moved after
/// the clamp (`delta`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedShift {
    pub system: SystemKey,
    pub cause: ShiftCause,
    pub amount: i32,
    pub delta: i32,
}

/// The game root's per-step control-change counters (`+0x78`-`+0x88`,
/// `FUN_00518040`): the step they count and the changes let through for
/// old holders 1 and 2.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ControlChangeCap {
    step: Option<u64>,
    changes: [u32; 2],
}

impl ControlChangeCap {
    /// `FUN_00518040`: a new step zeroes the counters; a change passes when
    /// its old holder has none yet in this step, which counts it. An old
    /// holder of 3 (Neutral) has no counter and always passes. The step is
    /// the scheduler's step counter (`FUN_004fd330` -> `FUN_004fcee0`,
    /// `+0xc`), raised on every frame step (`FUN_0051dea0.c:11`); a day
    /// runs the larger side's speed word (`+0xc4`, `FUN_0051dea0.c:23-28`:
    /// 600, 60, 12 or 4 from message `0x280`, `FUN_00487eb0`,
    /// `FUN_00628ff0` -> `FUN_005305e0`) plus one steps. Timers are due by
    /// day (`FUN_0051d8a0.c:6`) and run in the day's first step
    /// (`FUN_0051df30`), so the changes they cause share one step; an order
    /// runs in the step whose phase 5 executes it (`FUN_00436250`,
    /// `FUN_0051df30`). `step` is [`step`]'s key.
    fn pass(&mut self, step: u64, old_holder: u8) -> bool {
        if self.step != Some(step) {
            self.step = Some(step);
            self.changes = [0; 2];
        }
        let count = match old_holder {
            1 => &mut self.changes[0],
            2 => &mut self.changes[1],
            _ => return true,
        };
        if *count > 0 {
            return false;
        }
        *count += 1;
        true
    }
}

/// The key of scheduler step `in_day` of `day` (`FUN_004fd330`, `+0xc`):
/// one value per step, the day's first (`in_day` 0) keyed by the day
/// alone. The step is the in-day count `+0x20` (`FUN_0051dea0`), as
/// `ai_scheduler` counts its rounds.
#[must_use]
pub fn step(day: u64, in_day: u16) -> u64 {
    day + (u64::from(in_day) << 32)
}

/// A control word's holder (`+0x84` bits 0-1): 1 Alliance, 2 Empire, 3
/// Neutral (`FUN_0050b610`).
fn holder_code(control: ControlKind) -> u8 {
    match control.faction() {
        Some(Faction::Alliance) => 1,
        Some(Faction::Empire) => 2,
        _ => 3,
    }
}

/// The side across from side `code` (`2 - (code != 1)`, `FUN_0055de50`);
/// 3 maps to 1.
const fn opposite(code: u8) -> u8 {
    if code == 1 {
        2
    } else {
        1
    }
}

/// Side code 1 or 2 as a faction.
const fn side(code: u8) -> Faction {
    if code == 1 {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}

/// The side across from `faction`.
fn other(faction: Faction) -> Faction {
    if faction == Faction::Alliance {
        Faction::Empire
    } else {
        Faction::Alliance
    }
}

/// `FUN_0055de50`, `FUN_0055e010`: `from` and `to` for holders `old` and
/// `new` that differ; a holder of 3 counts as the opposite of the other end.
fn ends(old: u8, new: u8) -> (Faction, Faction) {
    let from = if old == 3 { opposite(new) } else { old };
    let to = if new == 3 { opposite(from) } else { new };
    (side(from), side(to))
}

/// `FUN_00518b20` -> `FUN_00518be0`: queue `record` as event `0x373` at each
/// system of `sector`, in the sector's order, whose family is in the
/// record's range, which is populated (`+0x88` bit 0) and not kept out
/// (bit 3), and where the gate side's support is below the limit.
pub fn post(world: &mut GameWorld, sector: SectorKey, record: &ShiftRecord, cause: ShiftCause) {
    if record.amount == 0 {
        return;
    }
    let Some(systems) = world.sectors.get(sector).map(|value| value.systems.clone()) else {
        return;
    };
    let amount = if record.direction == Faction::Alliance {
        record.amount
    } else {
        -record.amount
    };
    for system in systems {
        let Some(sys) = world.systems.get(system) else {
            continue;
        };
        if record.families.contains(&world.system_family(system))
            && sys.is_populated
            && !world.shift_excluded.contains(&system)
            && crate::uprising::support_points(sys, record.gate_side) < record.gate_limit
        {
            raise(
                world,
                QueuedEvent::Shift(PostedShift {
                    system,
                    amount,
                    cause,
                }),
            );
        }
    }
}

/// `FUN_00518ac0`: post `record` to the sector of `system`.
fn post_at(world: &mut GameWorld, system: SystemKey, record: &ShiftRecord, cause: ShiftCause) {
    if let Some(sector) = world.systems.get(system).map(|value| value.sector) {
        post(world, sector, record, cause);
    }
}

/// Phase 5: serve the queue's head until it is empty (`FUN_0051df30`,
/// `FUN_0054ec90`), the lowest priority first ([`raise`]): every regiment
/// rerun and holder change before any shift, and a holder change a shift
/// causes before the next shift. Event `0x311` (`0x400`, `FUN_0048e670`)
/// would follow them all; the port runs the blockade rerun at its callers,
/// after a `fire`. A shift (event `0x373`) fires through `FUN_00562fb0` ->
/// `FUN_0050c9f0(system, amount, 1)`, whose setter may rebuild the control
/// word ([`support_changed`]); a regiment's event rebuilds it
/// ([`regiments_changed`]); a holder change (event `0x31b`) posts its
/// shifts in `step` (`FUN_00517aa0`), which fire in turn.
pub fn fire(world: &mut GameWorld, step: u64) -> Fired {
    let points = |world: &GameWorld, system| {
        world.systems.get(system).map_or(0, |sys| {
            crate::uprising::support_points(sys, Faction::Alliance)
        })
    };
    let mut fired = Fired::default();
    while !world.support_events.is_empty() {
        match world.support_events.remove(0) {
            QueuedEvent::Shift(shift) => {
                let before = points(world, shift.system);
                crate::uprising::apply_support_change(
                    world,
                    shift.system,
                    Faction::Alliance,
                    shift.amount,
                );
                fired.shifts.push(AppliedShift {
                    system: shift.system,
                    cause: shift.cause,
                    amount: shift.amount,
                    delta: points(world, shift.system) - before,
                });
            }
            QueuedEvent::Control(change) => {
                event_0x31b(world, change, step);
                fired.controls.push(change);
            }
            QueuedEvent::Regiments(system) => rebuild_control(world, system),
        }
    }
    fired
}

/// The support setter's hook (`FUN_00509c30` -> slot `+0x1d0`,
/// `FUN_005109f0.c:14-29`), run after every change of `system`'s support:
/// it rebuilds the control word ([`rebuild_control`], `FUN_0050b610`), then
/// the garrison requirement (`FUN_0050b5a0`), drift (`FUN_0050b230`) and
/// collection rate (`FUN_0050b2c0`)
/// ([`crate::economy::refresh_support_values`]). port: the blockade
/// timer's delay (`FUN_0053fab0`, `:30-48`) is `blockade.rs`'s, checked
/// once a tick.
pub fn support_changed(world: &mut GameWorld, system: SystemKey) {
    rebuild_control(world, system);
    crate::economy::refresh_support_values(world, system);
}

/// A regiment joined or left `system`, or its usable bit changed there:
/// event `0x320` (`FUN_00514a60`, the old and the new container) or
/// `0x301` (`FUN_00514510`; `FUN_004f7410` -> hook `+0x110` ->
/// `FUN_004fb310` on every usable-bit change) reaches the system's slot
/// `+0xc8` (`FUN_00508660`), whose regiment branch (families `0x10`-`0x13`)
/// reruns `FUN_0050b610` when [`fire`] serves the event.
pub fn regiments_changed(world: &mut GameWorld, system: SystemKey) {
    raise(world, QueuedEvent::Regiments(system));
}

/// `FUN_0050b610` for `system`: rebuild its troop bits (`.c:34-63`) and its
/// holder (`economy::system_control`), and write a changed control
/// (`FUN_0050a300`), whose hook (slot `+0x208`, `FUN_00510d70`) hands the
/// facilities over, runs the owner change (`owner_changed`) and raises
/// event `0x31b` for [`fire`]. Run by the support hook, the regiment hook
/// ([`regiments_changed`]) and the populated bit's hook (`FUN_00510e30`).
pub fn rebuild_control(world: &mut GameWorld, system: SystemKey) {
    let Some(control) = crate::economy::system_control(world, system) else {
        return;
    };
    let troops = refresh_troop_bits(world, system);
    let previous = world.systems[system].control;
    if control == previous {
        return;
    }
    crate::trace::record(crate::trace::TraceMask::CONTROL, "control_resolved", || {
        serde_json::json!({
            "system": world.systems[system].name,
            "from": format!("{previous:?}"),
            "to": format!("{control:?}"),
            "populated": world.systems[system].is_populated,
            "alliance_support": world.systems[system].popularity_alliance,
        })
    });
    world.systems[system].control = control;
    world.hand_over_facilities(system);
    if let Some(change) = owner_changed(world, system, previous, control, troops) {
        raise(world, QueuedEvent::Control(change));
    }
}

/// `FUN_0050b890`, run by the owner hook `FUN_00510820` (and the populated
/// hook `FUN_00510e30`) only while bit 3 is set: it stays set while the
/// system is neutral or unpopulated, else clears for good.
pub fn refresh_exclusion(world: &mut GameWorld, system: SystemKey) {
    if !world.shift_excluded.contains(&system) {
        return;
    }
    let Some(sys) = world.systems.get(system) else {
        return;
    };
    if holder_code(sys.control) != 3 && sys.is_populated {
        world.shift_excluded.remove(&system);
    }
}

/// Whether `system` holds the Alliance HQ (families `0x20`-`0x21`). port:
/// the port flags both headquarters systems; Coruscant's flag is the
/// Empire's capital, not an ALLFACSD object.
#[must_use]
pub fn alliance_headquarters(world: &GameWorld, system: SystemKey) -> bool {
    world
        .systems
        .get(system)
        .is_some_and(|sys| sys.is_headquarters && sys.dat_id.index() != CORUSCANT)
}

/// The Alliance HQ at `system` is removed, by a bombardment hit, Destroy
/// System's cascade or a capture (`+0xac(3)`): clear it and post
/// `FUN_00517cd0`'s shift (`FUN_0055e1f0`): gate side the Alliance, limit
/// GNPRTB `0x1408`, direction the Alliance, amount GNPRTB `0x1407`, over the
/// HQ's sector. Returns whether an HQ was there.
pub fn remove_headquarters(world: &mut GameWorld, system: SystemKey) -> bool {
    if !alliance_headquarters(world, system) {
        return false;
    }
    if let Some(sys) = world.systems.get_mut(system) {
        sys.is_headquarters = false;
    }
    let value = |id| world.gnprtb.value(id, world.difficulty_index);
    let record = ShiftRecord {
        families: ALL_SYSTEMS,
        gate_side: Faction::Alliance,
        gate_limit: value(GNPRTB_HQ_LIMIT),
        direction: Faction::Alliance,
        amount: value(GNPRTB_HQ_AMOUNT),
    };
    post_at(world, system, &record, ShiftCause::Headquarters);
    true
}

/// `FUN_0055de50`: the control change's record for holders `old` and `new`
/// (they differ), with the troop bits (`+0x84 & 0x30`) of the old and new
/// words.
fn control_record(world: &GameWorld, old: u8, new: u8, troops: (bool, bool)) -> ShiftRecord {
    let (from, to) = ends(old, new);
    let value = |id, faction| world.sdprtb.value(id, world.difficulty_index, faction);
    let (amount, gate_limit) = match troops {
        (false, false) => {
            let mut amount = 0;
            if old == 1 || new == 1 {
                amount += value(SDPRTB_CONTROL_AMOUNT, Faction::Alliance);
            }
            if old == 2 || new == 2 {
                amount += value(SDPRTB_CONTROL_AMOUNT, Faction::Empire);
            }
            (amount, value(SDPRTB_CONTROL_LIMIT, to))
        }
        (false, true) => (
            value(SDPRTB_TROOPS_ARRIVE.0, to),
            value(SDPRTB_TROOPS_ARRIVE.1, to),
        ),
        (true, false) => (
            value(SDPRTB_TROOPS_LEAVE.0, to),
            value(SDPRTB_TROOPS_LEAVE.1, to),
        ),
        (true, true) => (
            value(SDPRTB_TROOPS_STAY.0, to),
            value(SDPRTB_TROOPS_STAY.1, to),
        ),
    };
    ShiftRecord {
        families: ALL_SYSTEMS,
        gate_side: from,
        gate_limit,
        direction: to,
        amount,
    }
}

/// `FUN_0055e010`: Coruscant's own record for holders `old` and `new`.
fn coruscant_record(world: &GameWorld, old: u8, new: u8) -> ShiftRecord {
    let (from, to) = ends(old, new);
    let value = |id, faction| world.sdprtb.value(id, world.difficulty_index, faction);
    let mut amount = 0;
    if old == 1 || new == 1 {
        amount += value(SDPRTB_CORUSCANT_AMOUNT, Faction::Alliance);
    }
    if old == 2 || new == 2 {
        amount += value(SDPRTB_CORUSCANT_AMOUNT, Faction::Empire);
    }
    ShiftRecord {
        families: ALL_SYSTEMS,
        gate_side: from,
        gate_limit: value(SDPRTB_CORUSCANT_LIMIT, to),
        direction: to,
        amount,
    }
}

/// `FUN_0050b610`'s troop bits (`+0x84` bits 4-5) for `system`, rebuilt
/// from its regiments (`FUN_00504cc0`, families `0x10`-`0x13`): set while
/// one side's stand there alone, cleared with none, kept with both
/// (`.c:34-63`); the holder is the troop side while they are set, else the
/// support holder (`:64-70`, `economy::resolve_system_control`). Stores
/// the result in `GameWorld::troop_held` and returns the old and the new
/// value for event `0x31b` ([`rebuild_control`]).
pub fn refresh_troop_bits(world: &mut GameWorld, system: SystemKey) -> (bool, bool) {
    let old = world.troop_held.contains(&system);
    let Some(sys) = world.systems.get(system) else {
        return (old, old);
    };
    let (mut alliance, mut empire) = (false, false);
    for troop in sys
        .ground_units
        .iter()
        .filter_map(|&key| world.troops.get(key))
    {
        if troop.is_alliance {
            alliance = true;
        } else {
            empire = true;
        }
    }
    let new = match (alliance, empire) {
        (false, false) => false,
        (true, true) => old,
        _ => true,
    };
    if new {
        world.troop_held.insert(system);
    } else {
        world.troop_held.remove(&system);
    }
    (old, new)
}

/// [`refresh_troop_bits`] for every system, as setup runs `FUN_0050b610` for
/// each one (`FUN_00508250`); in play each regiment's change reruns its own
/// system's (`FUN_00508660`, [`regiments_changed`]).
pub fn refresh_all_troop_bits(world: &mut GameWorld) {
    let systems: Vec<SystemKey> = world.systems.keys().collect();
    for system in systems {
        refresh_troop_bits(world, system);
    }
}

/// `system`'s control was written from `old` to its current value: the
/// write's owner change ([`owner_changed`]) and event `0x31b`'s shifts at
/// once, in `step`, as a test stages a holder change. `troops` is whether
/// the old and the new word have troop bits.
#[cfg(test)]
pub(crate) fn control_changed(
    world: &mut GameWorld,
    system: SystemKey,
    old: ControlKind,
    troops: (bool, bool),
    step: u64,
) {
    let Some(control) = world.systems.get(system).map(|sys| sys.control) else {
        return;
    };
    if let Some(change) = owner_changed(world, system, old, control, troops) {
        event_0x31b(world, change, step);
    }
}

/// The control write's hook (slot `+0x208`, `FUN_00510d70`) when the
/// holder changes from `old` to `control`: the owner bits
/// (`FUN_004f6f40`), then its hook (slot `+0xf0`, `FUN_00510820`): bit 3
/// (`FUN_0050b890`) and the Alliance HQ, whose class serves only the
/// Alliance, removed when the system turns Empire or neutral
/// (`FUN_004f8680`, `+0xac(3)`), and the production managers, which take
/// the new side and so lose their products (`FUN_0052b790`; served by
/// `manufacturing::serve_manager_sides`). Returns event `0x31b`, or `None`
/// when the holder stays.
fn owner_changed(
    world: &mut GameWorld,
    system: SystemKey,
    old: ControlKind,
    control: ControlKind,
    troops: (bool, bool),
) -> Option<ControlRerun> {
    let new = holder_code(control);
    if holder_code(old) == new {
        return None;
    }
    refresh_exclusion(world, system);
    if new != 1 {
        remove_headquarters(world, system);
    }
    move_enemies(world, system, new);
    world.manager_side_changes.push(system);
    world.autoscrap_systems.push(system);
    Some(ControlRerun {
        system,
        previous: old,
        control,
        troops,
    })
}

/// The owner change's child walk (`FUN_00510820` -> `FUN_004fae40`, slot
/// `+0xb0` on every child of `system`) for the new holder `side` (1, 2 or
/// 3; `ghidra/notes/ground-assault.md`, section 4):
///
/// - a character off a mission (`+0x78` bit 7) whose side bits differ from
///   the system's (`FUN_004ef260`): a prisoner, whose bits are its
///   captor's, is released first (slot `+0x218`, `FUN_004ef5b0`); one still
///   of another side takes the autoscrap request (`FUN_004f78e0(1)`), whose
///   sweep sends it off (`death_star::sweep_requests`);
/// - a special force off a mission (`FUN_00503d90`) takes the new side when
///   its class serves it (slot `+0x50`, `FUN_004f27d0`: class `+0x40`,
///   `+0x44`, both for side 3), else is removed (`+0xac(3)`).
///
/// port: the system's children are the characters it holds directly, not
/// aboard a fleet, with a side; a special force en route is on its Move.
/// A special force kept at a neutral system keeps its side, which the port
/// stores as a flag.
fn move_enemies(world: &mut GameWorld, system: SystemKey, side: u8) {
    let code = |is_alliance: bool| if is_alliance { 1 } else { 2 };
    let characters: Vec<crate::ids::CharacterKey> = world
        .characters
        .iter()
        .filter(|(_, c)| {
            c.current_system == Some(system)
                && c.current_fleet.is_none()
                && !c.on_mission
                && !c.is_killed
                && (c.is_alliance || c.is_empire)
        })
        .map(|(key, _)| key)
        .collect();
    for key in characters {
        let character = &mut world.characters[key];
        if code(character.is_alliance != character.is_captive) == side {
            continue;
        }
        if character.is_captive {
            character.is_captive = false;
            character.captured_by = None;
            character.capture_tick = None;
        }
        if code(character.is_alliance) != side {
            character.adrift = true;
        }
    }
    let Some(units) = world.systems.get(system).map(|sys| sys.special_forces.clone()) else {
        return;
    };
    for unit in units {
        let Some(force) = world
            .special_forces
            .get(unit)
            .filter(|force| !force.on_mission && !force.en_route)
        else {
            continue;
        };
        let class = world.buildable_classes.get(&force.class_dat_id);
        let (alliance, empire) = class.map_or((false, false), |c| (c.is_alliance, c.is_empire));
        let serves = match side {
            1 => alliance,
            2 => empire,
            _ => alliance && empire,
        };
        if !serves {
            crate::missions::destroy_special_force(world, unit);
        } else if side != 3 {
            world.special_forces[unit].is_alliance = side == 1;
        }
    }
}

/// Event `0x31b`'s handler (`FUN_00517aa0`): the change's shift over the
/// sector, capped to the first per old holder in `step` when neither word
/// has troop bits and the amount is nonzero (`FUN_0055de50`,
/// `FUN_00518040`), and at Coruscant its own shift (`FUN_00517be0`). port:
/// the victory rerun at Coruscant (`FUN_005180e0`) is the per-tick
/// `VictorySystem::check`.
fn event_0x31b(world: &mut GameWorld, change: ControlRerun, step: u64) {
    let Some(sys) = world.systems.get(change.system) else {
        return;
    };
    let coruscant = sys.dat_id.index() == CORUSCANT;
    let (old, new) = (holder_code(change.previous), holder_code(change.control));
    let record = control_record(world, old, new, change.troops);
    let has_troops = change.troops.0 || change.troops.1;
    if record.amount != 0 && (has_troops || world.control_change_cap.pass(step, old)) {
        post_at(world, change.system, &record, ShiftCause::ControlChange);
    }
    if coruscant {
        let record = coruscant_record(world, old, new);
        post_at(world, change.system, &record, ShiftCause::Coruscant);
    }
}

/// A regiment class's slot `+0x1c0` value: class `+0x58`, TROOPSD
/// `uprising_defense`. hyp: the slot is `FUN_00520b60` on the regiment
/// vtable as on the detector interface (`decoy-roll.md`), and slot
/// `+0x1c8`, which `FUN_00508c60` subtracts, is 0 like the other minus slots
/// (`status-window.md`).
#[must_use]
pub fn regiment_value(world: &GameWorld, class: DatId) -> i32 {
    world
        .buildable_classes
        .get(&class)
        .map_or(0, |value| value.stats.uprising_defense.cast_signed())
}

/// A capital ship class's slot `+0x1c0` value: CAPSHPSD `uprising_defense`
/// (class `+0x58`; the same hyp as [`regiment_value`]).
#[must_use]
pub fn ship_value(world: &GameWorld, class: CapitalShipKey) -> i32 {
    world
        .capital_ship_classes
        .get(class)
        .map_or(0, |value| value.uprising_defense.cast_signed())
}

/// A squadron class's slot `+0x1c0` value: FIGHTSD `uprising_defense`
/// (the same hyp as [`regiment_value`]).
#[must_use]
pub fn squadron_value(world: &GameWorld, class: FighterKey) -> i32 {
    world
        .fighter_classes
        .get(class)
        .map_or(0, |value| value.uprising_defense.cast_signed())
}

/// The loss records of the regiments and squadrons `system` itself holds,
/// all destroyed with `reason` by Destroy System's cascade (`FUN_004f8710`
/// -> slot `+0xac`); each destroy's event `0x300` feeds `FUN_00508c60`.
/// port: one record per squadron entry, its value times its count.
#[must_use]
pub fn system_unit_losses(world: &GameWorld, system: SystemKey, reason: u8) -> Vec<UnitLoss> {
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let side = |is_alliance: bool| Faction::of_alliance(is_alliance);
    let regiments = sys
        .ground_units
        .iter()
        .filter_map(|&troop| world.troops.get(troop))
        .map(|unit| UnitLoss {
            side: side(unit.is_alliance),
            value: regiment_value(world, unit.class_dat_id),
            reason,
        });
    let squadrons = sys.fighters.iter().map(|entry| UnitLoss {
        side: side(entry.is_alliance),
        value: squadron_value(world, entry.class)
            .saturating_mul(i32::try_from(entry.count).unwrap_or(i32::MAX)),
        reason,
    });
    regiments.chain(squadrons).collect()
}

/// `FUN_00517ee0`'s sums: slot `+0x1c0` over every family `0x10`-`0x1f`
/// object (regiments, capital ships, squadrons) that is active and complete
/// (`+0x50` bits `0x40` and `0x4`), Alliance then Empire; one on its own
/// leg counts (bit `0x20` is not read). An ordered capital ship exists
/// unfinished from its order (`ShipInstance::ordered`), so only completed
/// ships count. port: a regiment or squadron exists only once complete (a
/// queued one is a production entry); squadrons en route count.
pub fn strength(world: &GameWorld) -> [i32; 2] {
    let mut sums = [0_i32; 2];
    let mut add = |is_alliance: bool, value: i32, count: u32| {
        let total = value.saturating_mul(i32::try_from(count).unwrap_or(i32::MAX));
        let slot = &mut sums[usize::from(!is_alliance)];
        *slot = slot.saturating_add(total);
    };
    for troop in world.troops.values() {
        add(
            troop.is_alliance,
            regiment_value(world, troop.class_dat_id),
            1,
        );
    }
    for fleet in world.fleets.values() {
        for ship in fleet
            .capital_ships
            .iter()
            .filter(|ship| ship.alive && ship.completed)
        {
            add(fleet.is_alliance, ship_value(world, ship.class), 1);
        }
        for entry in &fleet.fighters {
            add(
                fleet.is_alliance,
                squadron_value(world, entry.class),
                entry.count,
            );
        }
    }
    for sys in world.systems.values() {
        for entry in &sys.fighters {
            add(
                entry.is_alliance,
                squadron_value(world, entry.class),
                entry.count,
            );
        }
    }
    for transit in &world.squadron_transits {
        add(transit.is_alliance, squadron_value(world, transit.class), 1);
    }
    sums
}

/// `FUN_0055dbf0`: the blockade end's record from the strength sums
/// `strength` and the losses `losses` (Alliance, Empire) with the
/// direction side `x`. The stronger side S weighs its losses by
/// `0x1412[S] + 0x1413[S]·d/T_S` percent, the weaker W by
/// `0x1414[W] + 0x1415[W] − 0x1415[W]·d/T_S` (both 0 when `T_S` is 0); the
/// amount is `(2·w_O − w_X) / G0x1420` toward X, turned toward the other
/// side when negative, and clamped to `0..=0x1411[direction]`; the gate is
/// the other side of the direction, limit `0x1410`. Integer division
/// truncates, as `FUN_0053e150`. port: a zero divisor reads as 1.
fn blockade_record(
    world: &GameWorld,
    strength: [i32; 2],
    x: Faction,
    losses: [i32; 2],
) -> ShiftRecord {
    let difficulty = world.difficulty_index;
    let sd = |id, faction| world.sdprtb.value(id, difficulty, faction);
    let index = |faction| usize::from(faction != Faction::Alliance);
    let strong = if strength[0] > strength[1] {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let weak = other(strong);
    let (total_strong, total_weak) = (strength[index(strong)], strength[index(weak)]);
    let (weight_strong, weight_weak) = if total_strong == 0 {
        (0, 0)
    } else {
        let d = total_strong - total_weak;
        let slope_strong = sd(SDPRTB_STRONG_SLOPE, strong);
        let slope_weak = sd(SDPRTB_WEAK_SLOPE, weak);
        (
            d * slope_strong / total_strong + sd(SDPRTB_STRONG_BASE, strong),
            sd(SDPRTB_WEAK_BASE, weak) + slope_weak - d * slope_weak / total_strong,
        )
    };
    let weighted_strong = weight_strong * losses[index(strong)] / 100;
    let weighted_weak = weight_weak * losses[index(weak)] / 100;
    let (weighted_o, weighted_x) = if strong == x {
        (weighted_weak, weighted_strong)
    } else {
        (weighted_strong, weighted_weak)
    };
    let divisor = world
        .gnprtb
        .value(GNPRTB_BLOCKADE_DIVISOR, difficulty)
        .max(1);
    let mut amount = (weighted_o * 2 - weighted_x) / divisor;
    let mut direction = x;
    if amount < 0 {
        amount = -amount;
        direction = other(x);
    }
    let amount = amount.max(0).min(sd(SDPRTB_BLOCKADE_CAP, direction));
    let gate_side = other(direction);
    ShiftRecord {
        families: ALL_SYSTEMS,
        gate_side,
        gate_limit: sd(SDPRTB_BLOCKADE_LIMIT, gate_side),
        direction,
        amount,
    }
}

/// The record a blockade's end at `system` posts ([`blockade_ended`]): X is
/// the stored blockader, turned to the other side when the blockader does
/// not hold the system, unless the system is destroyed; the losses are
/// `FUN_00509a20`'s totals ([`crate::blockade::BlockadeLosses`]). `None`
/// with no loss record or no blockader.
#[must_use]
pub fn blockade_end_record(
    world: &GameWorld,
    blockade: &BlockadeState,
    system: SystemKey,
) -> Option<ShiftRecord> {
    let losses = blockade.losses(system)?;
    let blockader = losses.blockader?;
    let sys = world.systems.get(system)?;
    let holder = holder_code(sys.control);
    let x = if !sys.is_destroyed && holder != holder_code(ControlKind::Controlled(blockader)) {
        other(blockader)
    } else {
        blockader
    };
    Some(blockade_record(
        world,
        strength(world),
        x,
        [
            losses.total(Faction::Alliance),
            losses.total(Faction::Empire),
        ],
    ))
}

/// The blockade bit of `system` cleared (event `0x305` with 0,
/// `FUN_00515f40` -> `FUN_00517ee0`): post [`blockade_end_record`] over its
/// sector.
pub fn blockade_ended(world: &mut GameWorld, blockade: &BlockadeState, system: SystemKey) {
    if let Some(record) = blockade_end_record(world, blockade, system) {
        post_at(world, system, &record, ShiftCause::BlockadeEnd);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockade::{UnitLoss, REASON_BOMBARDMENT, REASON_GROUND_COMBAT};
    use crate::carriage::ProductRoom;
    use crate::world::{
        BuildableClass, ClassStats, GnprtbEntry, GnprtbParams, SdprtbEntry, SdprtbParams, Sector,
        System, TroopUnit,
    };

    #[test]
    fn a_saved_world_keeps_the_changes_its_step_let_through() {
        // FUN_00518040 counts the step's changes; a load mid-step keeps the
        // count, as the uninterrupted game does.
        for holder in [1, 2] {
            let mut world = GameWorld::default();
            assert!(world.control_change_cap.pass(5, holder));
            let saved = serde_json::to_string(&world).unwrap();
            let mut loaded: GameWorld = serde_json::from_str(&saved).unwrap();
            assert!(
                !loaded.control_change_cap.pass(5, holder),
                "holder {holder}"
            );
            assert!(loaded.control_change_cap.pass(6, holder), "holder {holder}");
        }
    }

    fn gn(id: u16, value: i32) -> GnprtbEntry {
        GnprtbEntry {
            parameter_id: u32::from(id),
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        }
    }

    fn sd(id: u16, alliance: i32, empire: i32) -> SdprtbEntry {
        SdprtbEntry {
            parameter_id: u32::from(id),
            dev_alliance: alliance,
            dev_empire: empire,
            alliance_sp_easy_alliance: alliance,
            alliance_sp_easy_empire: empire,
            alliance_sp_medium_alliance: alliance,
            alliance_sp_medium_empire: empire,
            alliance_sp_hard_alliance: alliance,
            alliance_sp_hard_empire: empire,
            empire_sp_easy_alliance: alliance,
            empire_sp_easy_empire: empire,
            empire_sp_medium_alliance: alliance,
            empire_sp_medium_empire: empire,
            empire_sp_hard_alliance: alliance,
            empire_sp_hard_empire: empire,
            multiplayer_alliance: alliance,
            multiplayer_empire: empire,
        }
    }

    /// One sector of `count` populated core systems held by `holder`, each
    /// at 50 Alliance support, with the shipped GNPRTB and SDPRTB values of
    /// `support-shifts.md` ("Parameters", the Medium column: SDPRTB `0x1413`
    /// 400 for the player, 150 for the computer).
    fn sector_world(count: usize, holder: Faction) -> (GameWorld, Vec<SystemKey>) {
        let mut world = GameWorld {
            gnprtb: GnprtbParams::new(vec![
                gn(0x1407, -10),
                gn(0x1408, 101),
                gn(0x1420, 10),
                gn(7681, 2),
                gn(7760, 101),
            ]),
            sdprtb: SdprtbParams::new(vec![
                sd(0x1410, 101, 101),
                sd(0x1411, 20, 20),
                sd(0x1412, 100, 100),
                sd(0x1413, 400, 150),
                sd(0x1414, 0, 0),
                sd(0x1415, 100, 100),
                sd(0x1420, 1, 1),
                sd(0x1421, 100, 100),
                sd(0x1422, 10, 5),
                sd(0x1423, 100, 90),
                sd(0x1440, 0, 0),
                sd(0x1441, 101, 101),
                sd(0x1442, 10, 10),
                sd(0x1443, 101, 101),
                sd(0x1444, 0, 0),
                sd(0x1445, 101, 101),
            ]),
            ..GameWorld::default()
        };
        let sector = world.sectors.insert(Sector::default());
        let systems = (0..count)
            .map(|index| {
                let key = world.systems.insert(System {
                    dat_id: DatId::new(0x9000_0001 + u32::try_from(index).unwrap()),
                    sector,
                    is_populated: true,
                    popularity_alliance: 0.5,
                    popularity_empire: 0.5,
                    control: ControlKind::Controlled(holder),
                    ..System::default()
                });
                world.sectors[sector].systems.push(key);
                key
            })
            .collect::<Vec<_>>();
        for &system in &systems {
            hold(&mut world, system, ControlKind::Controlled(holder));
        }
        (world, systems)
    }

    /// Remove [`hold`]'s regiments from `system`, keeping its control.
    fn disband(world: &mut GameWorld, system: SystemKey) {
        for unit in std::mem::take(&mut world.systems[system].ground_units) {
            world.troops.remove(unit);
        }
    }

    /// Set `system`'s control with the regiments that keep it so when the
    /// support hook reruns `FUN_0050b610` (troop bits `.c:34-70`): one of
    /// the side for a held system, none for a neutral one, which GNPRTB
    /// 7760 (101) leaves to no side (`FUN_0055a080`). The class has no value
    /// (`regiment_value`).
    fn hold(world: &mut GameWorld, system: SystemKey, control: ControlKind) {
        disband(world, system);
        let sides: &[bool] = match control {
            ControlKind::Controlled(Faction::Alliance) => &[true],
            ControlKind::Controlled(Faction::Empire) => &[false],
            _ => &[],
        };
        for &is_alliance in sides {
            let unit = world.troops.insert(TroopUnit {
                class_dat_id: DatId::new(0x1000_0f00),
                is_alliance,
            });
            world.systems[system].ground_units.push(unit);
        }
        world.systems[system].control = control;
    }

    fn alliance(world: &GameWorld, system: SystemKey) -> i32 {
        crate::uprising::support_points(&world.systems[system], Faction::Alliance)
    }

    /// Change `system`'s holder to `new` and run the hook, then fire.
    fn change(world: &mut GameWorld, system: SystemKey, new: ControlKind, step: u64) {
        let old = world.systems[system].control;
        hold(world, system, new);
        control_changed(world, system, old, (false, false), step);
        fire(world, step);
    }

    /// Land one regiment of `side` at `system`, valued 0.
    fn land(world: &mut GameWorld, system: SystemKey, side: Faction) {
        let unit = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0f00),
            is_alliance: side == Faction::Alliance,
        });
        world.systems[system].ground_units.push(unit);
    }

    #[test]
    fn regiments_of_both_sides_leave_the_earlier_troop_side_holding_the_system() {
        // FUN_0050b610.c:45-61: with both sides' regiments the troop bits keep
        // their old value, and the holder is the troop side while they are
        // set (:64-70). No holder change, so no event 0x31b and the holder
        // keeps its facilities (facility-ownership.md, "Contested systems").
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, systems) = sector_world(2, side);
            let yard = world
                .manufacturing_facilities
                .insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId::new(0x2800_0001),
                    side,
                    ..crate::world::ManufacturingFacilityInstance::default()
                });
            world.systems[systems[0]].manufacturing_facilities.push(yard);
            refresh_troop_bits(&mut world, systems[0]);
            let other = if side == Faction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };
            land(&mut world, systems[0], other);
            rebuild_control(&mut world, systems[0]);
            assert_eq!(
                world.systems[systems[0]].control,
                ControlKind::Controlled(side),
                "{side:?}"
            );
            assert_eq!(
                world.systems[systems[0]].control.faction(),
                Some(side),
                "{side:?}"
            );
            assert!(world.troop_held.contains(&systems[0]), "{side:?}");
            assert!(world.support_events.is_empty(), "{side:?}");
            assert_eq!(world.manufacturing_facilities[yard].side, side, "{side:?}");
        }
    }

    #[test]
    fn regiments_of_both_sides_with_no_earlier_troop_side_leave_the_support_holder() {
        // FUN_0050b610.c:45-61 keeps troop bits of 0 when both sides'
        // regiments stand at a system that had none, so the holder is the
        // support side (:64-70); GNPRTB 7760 (101) gives no side, so
        // neutral.
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        hold(&mut world, systems[0], ControlKind::Uncontrolled);
        refresh_troop_bits(&mut world, systems[0]);
        land(&mut world, systems[0], Faction::Alliance);
        land(&mut world, systems[0], Faction::Empire);
        rebuild_control(&mut world, systems[0]);
        assert_eq!(world.systems[systems[0]].control, ControlKind::Uncontrolled);
        assert!(!world.troop_held.contains(&systems[0]));
    }

    /// The other side's regiment lands at `system`, whose holder's
    /// regiments stand there (troop bits set), and the hook reruns
    /// `FUN_0050b610` (`FUN_00508660`).
    fn stalemate(world: &mut GameWorld, system: SystemKey, side: Faction) {
        refresh_troop_bits(world, system);
        let other = if side == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        land(world, system, other);
        regiments_changed(world, system);
        fire(world, 1);
    }

    #[test]
    fn an_enemy_landing_beside_the_hq_garrison_keeps_the_hq_and_shifts_nothing() {
        // FUN_0050b610.c:45-70 keeps the earlier troop side while both
        // sides' regiments stand there, so no FUN_00517aa0 runs and no
        // FUN_004f8680 removes the HQ.
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        world.systems[systems[0]].is_headquarters = true;
        stalemate(&mut world, systems[0], Faction::Alliance);
        assert!(world.systems[systems[0]].is_headquarters);
        assert_eq!(alliance(&world, systems[1]), 50);
        assert!(world.support_events.is_empty());
    }

    /// [`change`] with the troop bits of the old and the new word.
    fn change_with(
        world: &mut GameWorld,
        system: SystemKey,
        new: ControlKind,
        troops: (bool, bool),
    ) {
        let old = world.systems[system].control;
        hold(world, system, new);
        control_changed(world, system, old, troops, 1);
        fire(world, 1);
    }

    #[test]
    fn a_stalemate_that_ends_with_its_earlier_holder_shifts_nothing() {
        // FUN_0050b610.c:34-70: with both sides' regiments the troop bits and
        // the holder stay the earlier side's, so its return is no change of
        // holder and FUN_00517aa0 posts nothing.
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, systems) = sector_world(2, side);
            stalemate(&mut world, systems[0], side);
            let landed = world.systems[systems[0]].ground_units.pop().unwrap();
            world.troops.remove(landed);
            regiments_changed(&mut world, systems[0]);
            fire(&mut world, 1);
            assert_eq!(
                world.systems[systems[0]].control,
                ControlKind::Controlled(side),
                "{side:?}"
            );
            assert_eq!(alliance(&world, systems[1]), 50, "{side:?}");
            assert!(world.support_events.is_empty());
        }
    }

    #[test]
    fn a_stalemate_that_ends_neutral_shifts_as_the_earlier_holders_loss() {
        // FUN_0050b610.c:34-70 keeps the earlier side through the stalemate,
        // so its end at neutral is that side's loss: FUN_0055de50's
        // old-word-only record (SDPRTB 0x1442) and, for the Alliance, the
        // HQ's removal (FUN_004f8680).
        for side in [Faction::Alliance, Faction::Empire] {
            let hq = side == Faction::Alliance;
            let (mut direct, systems) = sector_world(2, side);
            direct.systems[systems[0]].is_headquarters = hq;
            change_with(
                &mut direct,
                systems[0],
                ControlKind::Uncontrolled,
                (true, false),
            );
            let (mut world, systems) = sector_world(2, side);
            world.systems[systems[0]].is_headquarters = hq;
            stalemate(&mut world, systems[0], side);
            disband(&mut world, systems[0]);
            regiments_changed(&mut world, systems[0]);
            fire(&mut world, 1);
            assert_eq!(
                world.systems[systems[0]].control,
                ControlKind::Uncontrolled,
                "{side:?}"
            );
            assert_ne!(alliance(&direct, systems[1]), 50, "{side:?}");
            assert_eq!(
                alliance(&world, systems[1]),
                alliance(&direct, systems[1]),
                "{side:?}"
            );
            assert!(!world.systems[systems[0]].is_headquarters, "{side:?}");
        }
    }

    #[test]
    fn a_capture_from_the_other_side_shifts_the_sector_two_points_toward_the_captor() {
        // FUN_0055de50: SDPRTB 0x1420[1] + 0x1420[2] when the sides trade
        // a system; direction the new holder.
        let (mut world, systems) = sector_world(3, Faction::Alliance);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        for &system in &systems {
            assert_eq!(alliance(&world, system), 48);
        }
        let (mut world, systems) = sector_world(3, Faction::Empire);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Alliance),
            1,
        );
        for &system in &systems {
            assert_eq!(alliance(&world, system), 52);
        }
    }

    #[test]
    fn a_change_to_or_from_neutral_shifts_one_point_away_from_the_side_that_lost() {
        // FUN_0055de50: a holder of 3 counts as the opposite of the other
        // end; one side involved gives one 0x1420 amount.
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        change(&mut world, systems[0], ControlKind::Uncontrolled, 1);
        assert_eq!(alliance(&world, systems[1]), 49);
        let (mut world, systems) = sector_world(2, Faction::Empire);
        change(&mut world, systems[0], ControlKind::Uncontrolled, 1);
        assert_eq!(alliance(&world, systems[1]), 51);
        let (mut world, systems) = sector_world(2, Faction::Empire);
        hold(&mut world, systems[0], ControlKind::Uncontrolled);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Alliance),
            1,
        );
        assert_eq!(alliance(&world, systems[1]), 51);
    }

    #[test]
    fn only_the_first_change_per_old_holder_in_a_step_shifts() {
        // FUN_00518040: the game root's counters for old holder 1 and 2,
        // zeroed when the step number changes; old holder 3 always passes.
        let (mut world, systems) = sector_world(4, Faction::Alliance);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            5,
        );
        change(
            &mut world,
            systems[1],
            ControlKind::Controlled(Faction::Empire),
            5,
        );
        assert_eq!(alliance(&world, systems[3]), 48);
        change(
            &mut world,
            systems[2],
            ControlKind::Controlled(Faction::Empire),
            6,
        );
        assert_eq!(alliance(&world, systems[3]), 46);
        // Old holder 2 has its own counter in step 6, and a second change
        // from it in that step is capped too.
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Alliance),
            6,
        );
        assert_eq!(alliance(&world, systems[3]), 48);
        change(
            &mut world,
            systems[2],
            ControlKind::Controlled(Faction::Alliance),
            6,
        );
        assert_eq!(alliance(&world, systems[3]), 48);
        // Neutral as old holder is never capped.
        hold(&mut world, systems[1], ControlKind::Uncontrolled);
        hold(&mut world, systems[2], ControlKind::Uncontrolled);
        change(
            &mut world,
            systems[1],
            ControlKind::Controlled(Faction::Alliance),
            6,
        );
        change(
            &mut world,
            systems[2],
            ControlKind::Controlled(Faction::Alliance),
            6,
        );
        assert_eq!(alliance(&world, systems[3]), 50);
    }

    #[test]
    fn the_next_days_first_step_and_a_later_step_of_it_each_count_anew() {
        // FUN_00518040 keys its counters to the scheduler step (FUN_004fd330,
        // +0xc, raised every frame step by FUN_0051dea0.c:11): a day's first
        // step, the next day's first step and a later step of that day are
        // three steps; a second change in the last is capped.
        for (holder, captor, toward) in [
            (Faction::Alliance, Faction::Empire, -2),
            (Faction::Empire, Faction::Alliance, 2),
        ] {
            let (mut world, systems) = sector_world(5, holder);
            let steps = [step(1, 0), step(2, 0), step(2, 3), step(2, 3)];
            let mut expected = 50;
            for (index, key) in steps.into_iter().enumerate() {
                change(&mut world, systems[index], ControlKind::Controlled(captor), key);
                if index < 3 {
                    expected += toward;
                }
                assert_eq!(alliance(&world, systems[4]), expected, "{holder:?} {index}");
            }
        }
    }

    #[test]
    fn a_change_between_troop_holders_takes_the_troop_amounts_and_skips_the_cap() {
        // FUN_0055de50: troop bits in both words give SDPRTB 0x1444 (0,
        // nothing posted); the cap runs only when neither word has them
        // (FUN_00517aa0).
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        let old = world.systems[systems[0]].control;
        hold(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
        );
        control_changed(&mut world, systems[0], old, (true, true), 1);
        fire(&mut world, 1);
        assert_eq!(alliance(&world, systems[1]), 50);
        // The cap was not spent: a plain change in the same step shifts.
        change(
            &mut world,
            systems[1],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        assert_eq!(alliance(&world, systems[1]), 48);
        // That change spent the Alliance's cap for the step; a change with
        // troop bits in one word skips it and still shifts.
        let old = ControlKind::Controlled(Faction::Alliance);
        control_changed(&mut world, systems[0], old, (true, false), 1);
        fire(&mut world, 1);
        assert_eq!(alliance(&world, systems[1]), 38);
        // Troop bits in the old word alone (0x1442, 10) and in the new word
        // alone (0x1440, 0), both toward the new holder.
        let ended = control_record(&world, 1, 2, (true, false));
        assert_eq!(
            (ended.amount, ended.direction, ended.gate_side),
            (10, Faction::Empire, Faction::Alliance)
        );
        assert_eq!(control_record(&world, 2, 1, (false, true)).amount, 0);
    }

    #[test]
    fn regiments_of_one_side_set_the_troop_bits_none_clear_them_and_both_keep_them() {
        // FUN_0050b610.c:34-63 over the system's regiments (FUN_00504cc0,
        // families 0x10-0x13): only side 1 or only side 2 sets bits 4-5,
        // neither clears them, both leave the old value.
        let (mut world, systems) = sector_world(1, Faction::Alliance);
        let system = systems[0];
        disband(&mut world, system);
        let land = |world: &mut GameWorld, is_alliance| {
            let key = world.troops.insert(TroopUnit {
                is_alliance,
                ..TroopUnit::default()
            });
            world.systems[system].ground_units.push(key);
            key
        };
        assert_eq!(refresh_troop_bits(&mut world, system), (false, false));
        let alliance = land(&mut world, true);
        assert_eq!(refresh_troop_bits(&mut world, system), (false, true));
        land(&mut world, false);
        assert_eq!(refresh_troop_bits(&mut world, system), (true, true));
        world.systems[system].ground_units.clear();
        assert_eq!(refresh_troop_bits(&mut world, system), (true, false));
        // Both sides landing on a word without bits leaves it without.
        world.systems[system].ground_units.push(alliance);
        land(&mut world, false);
        assert_eq!(refresh_troop_bits(&mut world, system), (false, false));
        assert!(!world.troop_held.contains(&system));
    }

    #[test]
    fn the_troop_bits_rebuild_for_every_system_at_once() {
        // FUN_0050b610's bits 4-5 per system; the port's whole-galaxy pass.
        let (mut world, systems) = sector_world(3, Faction::Alliance);
        for &system in &systems {
            disband(&mut world, system);
        }
        for (&system, is_alliance) in systems.iter().zip([true, false]) {
            let key = world.troops.insert(TroopUnit {
                is_alliance,
                ..TroopUnit::default()
            });
            world.systems[system].ground_units.push(key);
        }
        world.troop_held.insert(systems[2]);
        refresh_all_troop_bits(&mut world);
        let held = |world: &GameWorld| {
            systems
                .iter()
                .map(|system| world.troop_held.contains(system))
                .collect::<Vec<_>>()
        };
        assert_eq!(held(&world), [true, true, false]);
        world.systems[systems[0]].ground_units.clear();
        refresh_all_troop_bits(&mut world);
        assert_eq!(held(&world), [false, true, false]);
    }

    #[test]
    fn coruscant_changing_hands_adds_its_own_shift_for_both_sides() {
        // FUN_00517be0 -> FUN_0055e010: 0x1422[1] + 0x1422[2] = 15 when the
        // sides trade Coruscant, limit 0x1423[to], after the generic shift.
        let (mut world, systems) = sector_world(2, Faction::Empire);
        world.systems[systems[0]].dat_id = DatId::new(0x9000_0000 | CORUSCANT);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Alliance),
            1,
        );
        assert_eq!(alliance(&world, systems[1]), 67);
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        world.systems[systems[0]].dat_id = DatId::new(0x9000_0000 | CORUSCANT);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        assert_eq!(alliance(&world, systems[1]), 33);
        // Toward the Empire the gate passes only where the Alliance, the
        // side that lost it, holds under 0x1423[2] (90).
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        world.systems[systems[0]].dat_id = DatId::new(0x9000_0000 | CORUSCANT);
        world.systems[systems[1]].popularity_alliance = 0.95;
        world.systems[systems[1]].popularity_empire = 0.05;
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        assert_eq!(alliance(&world, systems[0]), 33);
        assert_eq!(alliance(&world, systems[1]), 93);
    }

    #[test]
    fn a_rim_system_with_bit_3_is_left_out_until_a_held_change_clears_it() {
        // FUN_00518be0 skips +0x88 bit 3; FUN_0050b890 clears it once the
        // system is held and populated, and keeps it while neutral.
        let (mut world, systems) = sector_world(3, Faction::Alliance);
        world.shift_excluded.insert(systems[1]);
        world.shift_excluded.insert(systems[2]);
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        assert_eq!(alliance(&world, systems[0]), 48);
        assert_eq!(alliance(&world, systems[1]), 50);
        // The rim system changes hands: its bit clears and it shifts.
        change(
            &mut world,
            systems[1],
            ControlKind::Controlled(Faction::Empire),
            2,
        );
        assert!(!world.shift_excluded.contains(&systems[1]));
        assert_eq!(alliance(&world, systems[1]), 48);
        // A change to neutral keeps the bit.
        change(&mut world, systems[2], ControlKind::Uncontrolled, 3);
        assert!(world.shift_excluded.contains(&systems[2]));
    }

    #[test]
    fn capturing_the_alliance_hq_removes_it_and_costs_the_alliance_ten_points() {
        // FUN_004f8680 -> +0xac(3), FUN_00517cd0 -> FUN_0055e1f0: GNPRTB
        // 0x1407 (-10) toward the Alliance, then the capture's 2.
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        world.systems[systems[0]].is_headquarters = true;
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        assert!(!world.systems[systems[0]].is_headquarters);
        assert_eq!(alliance(&world, systems[1]), 38);
        // To neutral removes it too; the Alliance taking a system keeps it.
        let (mut world, systems) = sector_world(2, Faction::Alliance);
        world.systems[systems[0]].is_headquarters = true;
        change(&mut world, systems[0], ControlKind::Uncontrolled, 1);
        assert!(!world.systems[systems[0]].is_headquarters);
        assert_eq!(alliance(&world, systems[1]), 39);
        // Coruscant's flag is the Empire's capital, not the HQ object.
        let (mut world, systems) = sector_world(2, Faction::Empire);
        world.systems[systems[0]].dat_id = DatId::new(0x9000_0000 | CORUSCANT);
        world.systems[systems[0]].is_headquarters = true;
        hold(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Alliance),
        );
        change(
            &mut world,
            systems[0],
            ControlKind::Controlled(Faction::Empire),
            1,
        );
        assert!(world.systems[systems[0]].is_headquarters);
    }

    #[test]
    fn shifts_fire_in_posting_order_each_clamped_on_its_own() {
        // FUN_00562fb0 per event: an HQ removal (-10) posted before a +15
        // lands as 95 -> 85 -> 100, never past 100 in between.
        let (mut world, systems) = sector_world(1, Faction::Alliance);
        world.systems[systems[0]].popularity_alliance = 0.95;
        world.systems[systems[0]].popularity_empire = 0.05;
        world.systems[systems[0]].is_headquarters = true;
        assert!(remove_headquarters(&mut world, systems[0]));
        let sector = world.systems[systems[0]].sector;
        post(
            &mut world,
            sector,
            &ShiftRecord {
                families: ALL_SYSTEMS,
                gate_side: Faction::Alliance,
                gate_limit: 101,
                direction: Faction::Alliance,
                amount: 15,
            },
            ShiftCause::Bombardment,
        );
        assert_eq!(alliance(&world, systems[0]), 95, "posting applies nothing");
        let applied = fire(&mut world, 1).shifts;
        assert_eq!(alliance(&world, systems[0]), 100);
        assert!(world.support_events.is_empty());
        // Each record carries its cause, its amount and the clamped move.
        let moves: Vec<_> = applied
            .iter()
            .map(|shift| (shift.cause, shift.amount, shift.delta))
            .collect();
        assert_eq!(
            moves,
            vec![
                (ShiftCause::Headquarters, -10, -10),
                (ShiftCause::Bombardment, 15, 15)
            ]
        );
    }

    #[test]
    fn a_clamped_shift_records_only_the_points_it_moved() {
        let (mut world, systems) = sector_world(1, Faction::Alliance);
        world.systems[systems[0]].popularity_alliance = 0.95;
        world.systems[systems[0]].popularity_empire = 0.05;
        let sector = world.systems[systems[0]].sector;
        let record = ShiftRecord {
            families: ALL_SYSTEMS,
            gate_side: Faction::Alliance,
            gate_limit: 101,
            direction: Faction::Alliance,
            amount: 15,
        };
        post(&mut world, sector, &record, ShiftCause::ControlChange);
        let applied = fire(&mut world, 1).shifts;
        assert_eq!(applied.len(), 1);
        assert_eq!((applied[0].amount, applied[0].delta), (15, 5));
    }

    #[test]
    fn a_destroyed_systems_regiments_and_squadrons_are_its_losses() {
        // FUN_004f8710 destroys the system's own units with the strike's
        // reason; each event 0x300 carries the unit's slot +0x1c0 value
        // (FUN_00508c60), a squadron's times its count.
        use crate::world::{FighterClass, SystemSquadrons};
        let (mut world, systems) = sector_world(1, Faction::Alliance);
        disband(&mut world, systems[0]);
        regiments(&mut world, systems[0], 2, 1, 3);
        let squadron = world.fighter_classes.insert(FighterClass {
            uprising_defense: 4,
            ..FighterClass::default()
        });
        world.systems[systems[0]].fighters.push(SystemSquadrons {
            class: squadron,
            count: 2,
            is_alliance: false,
        });
        let losses = system_unit_losses(&world, systems[0], REASON_BOMBARDMENT);
        let summary: Vec<(Faction, i32)> =
            losses.iter().map(|loss| (loss.side, loss.value)).collect();
        assert_eq!(
            summary,
            vec![
                (Faction::Alliance, 3),
                (Faction::Alliance, 3),
                (Faction::Empire, 3),
                (Faction::Empire, 8),
            ]
        );
        assert!(losses.iter().all(|loss| loss.reason == REASON_BOMBARDMENT));
    }

    #[test]
    fn a_gate_reads_support_when_posted_not_when_fired() {
        // FUN_00518be0 tests the limit at the post; the shifts queued
        // before it have not applied yet.
        let (mut world, systems) = sector_world(1, Faction::Empire);
        let sector = world.systems[systems[0]].sector;
        let record = |amount| ShiftRecord {
            families: ALL_SYSTEMS,
            gate_side: Faction::Alliance,
            gate_limit: 55,
            direction: Faction::Alliance,
            amount,
        };
        post(&mut world, sector, &record(10), ShiftCause::Bombardment);
        post(&mut world, sector, &record(10), ShiftCause::Bombardment);
        fire(&mut world, 1);
        assert_eq!(alliance(&world, systems[0]), 70);
        // Support equal to the limit is not below it.
        let (mut world, systems) = sector_world(1, Faction::Empire);
        let sector = world.systems[systems[0]].sector;
        post(
            &mut world,
            sector,
            &ShiftRecord {
                gate_limit: 50,
                ..record(10)
            },
            ShiftCause::Bombardment,
        );
        assert!(world.support_events.is_empty());
    }

    #[test]
    fn an_unpopulated_or_out_of_range_system_takes_no_shift() {
        // FUN_00518be0: populated bit and family range.
        let (mut world, systems) = sector_world(3, Faction::Alliance);
        world.systems[systems[1]].is_populated = false;
        world.system_families.insert(systems[2], 0x92);
        let sector = world.systems[systems[0]].sector;
        post(
            &mut world,
            sector,
            &ShiftRecord {
                families: 0x90..0x92,
                gate_side: Faction::Empire,
                gate_limit: 101,
                direction: Faction::Empire,
                amount: 4,
            },
            ShiftCause::Bombardment,
        );
        fire(&mut world, 1);
        assert_eq!(alliance(&world, systems[0]), 46);
        assert_eq!(alliance(&world, systems[1]), 50);
        assert_eq!(alliance(&world, systems[2]), 50);
    }

    /// `alliance` and `empire` regiments of slot `+0x1c0` value `value`,
    /// landing beside those already at `system`, whose troop bits the
    /// earlier regiments set (`FUN_0050b610.c:34-63`).
    fn regiments(world: &mut GameWorld, system: SystemKey, alliance: u32, empire: u32, value: u32) {
        refresh_troop_bits(world, system);
        let class = DatId::new(0x1000_0001);
        world.buildable_classes.insert(
            class,
            BuildableClass {
                stats: ClassStats {
                    uprising_defense: value,
                    ..ClassStats::default()
                },
                ..BuildableClass::default()
            },
        );
        for is_alliance in (0..alliance)
            .map(|_| true)
            .chain((0..empire).map(|_| false))
        {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: class,
                is_alliance,
            });
            world.systems[system].ground_units.push(key);
        }
    }

    #[test]
    fn the_blockade_formula_weighs_each_sides_losses_by_the_galaxys_strength() {
        // FUN_0055dbf0 with T1 = 30, T2 = 10 (S = the Alliance, d = 20):
        // k_S = 100 + 400*20/30 = 366, k_W = 0 + 100 - 100*20/30 = 34.
        // L1 = 10, L2 = 50; X = the Alliance: w_X = 36, w_O = 17;
        // (2*17 - 36) / 10 = 0 (truncated). X = the Empire: (2*36 - 17)/10
        // = 5 toward the Empire, gate the Alliance, limit 101.
        let (world, _) = sector_world(1, Faction::Alliance);
        let toward_alliance = blockade_record(&world, [30, 10], Faction::Alliance, [10, 50]);
        assert_eq!(
            (toward_alliance.amount, toward_alliance.direction),
            (0, Faction::Alliance)
        );
        let toward_empire = blockade_record(&world, [30, 10], Faction::Empire, [10, 50]);
        assert_eq!(
            toward_empire,
            ShiftRecord {
                families: ALL_SYSTEMS,
                gate_side: Faction::Alliance,
                gate_limit: 101,
                direction: Faction::Empire,
                amount: 5,
            }
        );
        // The Empire stronger (T2 >= T1 picks side 2): k_S = 100 +
        // 150*10/20 = 175, k_W = 100 - 100*10/20 = 50; L1 = 40, L2 = 0;
        // X = the Alliance: w_X = 20, w_O = 0, (0 - 20)/10 = -2 -> 2 toward
        // the Empire.
        let turned = blockade_record(&world, [10, 20], Faction::Alliance, [40, 0]);
        assert_eq!((turned.amount, turned.direction), (2, Faction::Empire));
        // Capped at SDPRTB 0x1411 (20).
        let capped = blockade_record(&world, [10, 20], Faction::Empire, [400, 0]);
        assert_eq!((capped.amount, capped.direction), (20, Faction::Empire));
        // No strength at all: both weights 0, nothing posted.
        assert_eq!(
            blockade_record(&world, [0, 0], Faction::Alliance, [40, 40]).amount,
            0
        );
    }

    #[test]
    fn a_lifted_blockade_shifts_the_sector_toward_the_holder_whose_enemy_lost_more() {
        // FUN_00515f40 -> FUN_00517ee0: X is the blockader turned to the
        // holder; losses from FUN_00508c60's counter B (reason 0xf).
        for holder in [Faction::Alliance, Faction::Empire] {
            let blockader = other(holder);
            let (mut world, systems) = sector_world(2, holder);
            regiments(&mut world, systems[1], 3, 3, 10);
            let mut blockade = BlockadeState::new();
            blockade.start_losses(systems[0], blockader);
            for _ in 0..4 {
                blockade.record_loss(
                    systems[0],
                    UnitLoss {
                        side: blockader,
                        value: 10,
                        reason: REASON_BOMBARDMENT,
                    },
                );
            }
            // T1 = T2 = 30 (S = the Empire, d = 0): k_S = 100, k_W = 100.
            // The blockader lost 40: w_O = 40, w_X = 0, 80 / 10 = 8.
            blockade_ended(&mut world, &blockade, systems[0]);
            fire(&mut world, 1);
            let expected = if holder == Faction::Alliance { 58 } else { 42 };
            assert_eq!(alliance(&world, systems[0]), expected, "{holder:?}");
            assert_eq!(alliance(&world, systems[1]), expected, "{holder:?}");
        }
    }

    #[test]
    fn a_strength_tie_makes_the_empire_the_stronger_side() {
        // FUN_0055dbf0: S = (T1 <= T2) + 1. With SDPRTB 0x1412 at 100 for
        // the Alliance and 200 for the Empire and T1 = T2, k_E = 200 and
        // k_A = 0 + 100; the Empire lost 10, X = the Alliance:
        // (2*20 - 0) / 10 = 4.
        let (mut world, _) = sector_world(1, Faction::Alliance);
        let mut entries = vec![sd(0x1412, 100, 200)];
        for (id, alliance, empire) in [
            (0x1410, 101, 101),
            (0x1411, 20, 20),
            (0x1413, 400, 150),
            (0x1414, 0, 0),
            (0x1415, 100, 100),
        ] {
            entries.push(sd(id, alliance, empire));
        }
        world.sdprtb = SdprtbParams::new(entries);
        let record = blockade_record(&world, [10, 10], Faction::Alliance, [0, 10]);
        assert_eq!((record.amount, record.direction), (4, Faction::Alliance));
    }

    #[test]
    fn the_strength_sums_every_regiment_ship_and_squadron_by_side() {
        // FUN_00517ee0: slot +0x1c0 over families 0x10-0x1f, owner 1 or 2:
        // regiments, live capital ships, squadrons aboard fleets, held by
        // systems and en route.
        use crate::world::{
            CapitalShipClass, FighterClass, FighterEntry, Fleet, ShipInstance, SystemSquadrons,
        };
        let (mut world, systems) = sector_world(1, Faction::Alliance);
        regiments(&mut world, systems[0], 1, 2, 3);
        let ship = world.capital_ship_classes.insert(CapitalShipClass {
            uprising_defense: 20,
            ..CapitalShipClass::default()
        });
        let squadron = world.fighter_classes.insert(FighterClass {
            uprising_defense: 100,
            ..FighterClass::default()
        });
        let mut dead = ShipInstance::new(ship, 1, true);
        dead.alive = false;
        world.fleets.insert(Fleet {
            location: systems[0],
            capital_ships: vec![ShipInstance::new(ship, 1, true), dead],
            fighters: vec![FighterEntry {
                class: squadron,
                count: 2,
                carrier: 0,
            }],
            is_alliance: true,
            ..Fleet::default()
        });
        world.fleets.insert(Fleet {
            location: systems[0],
            capital_ships: vec![ShipInstance::new(ship, 1, false)],
            is_alliance: false,
            ..Fleet::default()
        });
        world.systems[systems[0]].fighters.push(SystemSquadrons {
            class: squadron,
            count: 3,
            is_alliance: false,
        });
        world
            .squadron_transits
            .push(crate::troop_transport::SquadronTransit {
                class: squadron,
                is_alliance: true,
                origin: systems[0],
                leg: crate::troop_transport::RegimentLeg::Surface(systems[0]),
                carrier: 0,
                arrival_tick: 9,
                via: None,
            });
        assert_eq!(strength(&world), [3 + 20 + 200 + 100, 6 + 20 + 300]);
    }

    #[test]
    fn the_strength_leaves_out_an_unfinished_ship_for_either_side() {
        // FUN_00517ee0 sums only objects with +0x50 bit 0x4 (completed): an
        // ordered ship, created with completed clear (FUN_0052a140 ->
        // FUN_004f7d50(.., 1, 0)), adds nothing; one on its own leg adds
        // its value.
        use crate::world::{CapitalShipClass, Fleet, ShipInstance, ShipTransit};
        for is_alliance in [true, false] {
            let (mut world, systems) = sector_world(1, Faction::Alliance);
            let ship = world.capital_ship_classes.insert(CapitalShipClass {
                uprising_defense: 20,
                ..CapitalShipClass::default()
            });
            let mut travelling = ShipInstance::new(ship, 1, is_alliance);
            travelling.transit = Some(ShipTransit {
                from: systems[0],
                to: systems[0],
                arrival_tick: 9,
            });
            world.fleets.insert(Fleet {
                location: systems[0],
                capital_ships: vec![
                    ShipInstance::new(ship, 1, is_alliance),
                    ShipInstance::ordered(ship, 1, is_alliance),
                    travelling,
                ],
                is_alliance,
                ..Fleet::default()
            });
            let expected = if is_alliance { [40, 0] } else { [0, 40] };
            assert_eq!(strength(&world), expected, "{is_alliance}");
        }
    }

    #[test]
    fn a_destroyed_system_keeps_its_blockader_as_x() {
        // FUN_00515f40: X turns to the other side only when the system is
        // not destroyed (+0x50 bit 3). The Empire blockades Alliance-held
        // ground and loses 40 (S = the Empire at a tie, k = 100 both):
        // X = the Alliance gives (2*40 - 0) / 10 = 8; destroyed, X = the
        // Empire gives (0 - 40) / 10 = -4, so 4 toward the Alliance.
        for (destroyed, expected) in [(false, 58), (true, 54)] {
            let (mut world, systems) = sector_world(2, Faction::Alliance);
            regiments(&mut world, systems[1], 1, 1, 10);
            world.systems[systems[0]].is_destroyed = destroyed;
            let mut blockade = BlockadeState::new();
            blockade.start_losses(systems[0], Faction::Empire);
            blockade.record_loss(
                systems[0],
                UnitLoss {
                    side: Faction::Empire,
                    value: 40,
                    reason: REASON_BOMBARDMENT,
                },
            );
            blockade_ended(&mut world, &blockade, systems[0]);
            fire(&mut world, 1);
            assert_eq!(
                alliance(&world, systems[1]),
                expected,
                "destroyed {destroyed}"
            );
        }
    }

    #[test]
    fn a_blockade_never_started_posts_nothing_and_reason_0xe_feeds_counter_a() {
        // FUN_00509a20 needs a stored blockader; A and B both count.
        let (mut world, systems) = sector_world(1, Faction::Alliance);
        let mut blockade = BlockadeState::new();
        blockade.record_loss(
            systems[0],
            UnitLoss {
                side: Faction::Empire,
                value: 50,
                reason: REASON_GROUND_COMBAT,
            },
        );
        blockade_ended(&mut world, &blockade, systems[0]);
        assert!(world.support_events.is_empty());
        blockade.start_losses(systems[0], Faction::Empire);
        assert_eq!(
            blockade
                .losses(systems[0])
                .map(|l| l.total(Faction::Empire)),
            Some(0)
        );
        blockade.record_loss(
            systems[0],
            UnitLoss {
                side: Faction::Empire,
                value: 50,
                reason: REASON_GROUND_COMBAT,
            },
        );
        assert_eq!(
            blockade
                .losses(systems[0])
                .map(|l| l.total(Faction::Empire)),
            Some(50)
        );
    }

    /// [`sector_world`] of `count` Empire-held systems, its regiments
    /// disbanded, at
    /// 40 Alliance support, so each holds by support alone: GNPRTB 7760
    /// (60) is the support a side needs (`FUN_0055a080`).
    fn support_held_world(count: usize) -> (GameWorld, Vec<SystemKey>) {
        let (mut world, systems) = sector_world(count, Faction::Empire);
        world.gnprtb = GnprtbParams::new(vec![
            gn(0x1407, -10),
            gn(0x1408, 101),
            gn(0x1420, 10),
            gn(7681, 2),
            gn(7760, 60),
        ]);
        for &system in &systems {
            disband(&mut world, system);
            world.systems[system].popularity_alliance = 0.4;
            world.systems[system].popularity_empire = 0.6;
        }
        (world, systems)
    }

    #[test]
    fn a_support_change_that_moves_the_holder_rebuilds_control_at_once() {
        // FUN_00509c30 calls hook +0x1d0 (FUN_005109f0) on every change of
        // +0x58, and the hook reruns FUN_0050b610: a system with no
        // regiments takes its holder from support (FUN_0055a080) before
        // the setter returns.
        let (mut world, systems) = support_held_world(2);
        crate::uprising::apply_support_change(&mut world, systems[0], Faction::Alliance, 20);
        assert_eq!(
            world.systems[systems[0]].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert_eq!(
            world.systems[systems[1]].control,
            ControlKind::Controlled(Faction::Empire)
        );
    }

    #[test]
    fn shifts_that_turn_systems_post_the_first_turns_shift_in_the_same_fire() {
        // Event 0x373 fires through FUN_0050c9f0 -> FUN_00509c30, whose hook
        // (FUN_005109f0 -> FUN_0050b610) rewrites the control word and
        // raises event 0x31b; FUN_0051df30 serves the queue until it is
        // empty, so FUN_00517aa0's shift lands in the same step. The step
        // cap (FUN_00518040) lets one change per old holder through.
        let (mut world, systems) = support_held_world(3);
        let record = ShiftRecord {
            families: ALL_SYSTEMS,
            gate_side: Faction::Empire,
            gate_limit: 101,
            direction: Faction::Alliance,
            amount: 20,
        };
        let sector = world.systems[systems[0]].sector;
        post(&mut world, sector, &record, ShiftCause::Bombardment);
        fire(&mut world, 1);
        for &system in &systems {
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(Faction::Alliance)
            );
            // 40 + 20, then the first change's SDPRTB 0x1420 shift: 1 + 1.
            assert_eq!(alliance(&world, system), 62);
        }
        assert!(world.support_events.is_empty());
    }

    /// One populated core system held by `side`'s only regiment, its
    /// support 70 for the other side (GNPRTB 7760 (60), `FUN_0055a080`),
    /// with a fleet of `side` there that carries one regiment.
    fn garrisoned(
        side: Faction,
    ) -> (
        GameWorld,
        SystemKey,
        crate::ids::TroopKey,
        crate::ids::FleetKey,
    ) {
        let (mut world, systems) = sector_world(1, side);
        let system = systems[0];
        world.gnprtb = GnprtbParams::new(vec![gn(7760, 60)]);
        let other = if side == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        crate::uprising::apply_support_change(&mut world, system, other, 20);
        world.troop_held.insert(system);
        let troop = world.systems[system].ground_units[0];
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                troop_capacity: 1,
                hull: 100,
                ..Default::default()
            });
        let is_alliance = side == Faction::Alliance;
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: system,
            capital_ships: vec![crate::world::ShipInstance::new(class, 100, is_alliance)],
            is_alliance,
            ..Default::default()
        });
        world.systems[system].fleets.push(fleet);
        (world, system, troop, fleet)
    }

    fn opposite_side(side: Faction) -> Faction {
        if side == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        }
    }

    #[test]
    fn loading_the_last_regiment_aboard_turns_its_system_to_the_support_holder_that_step() {
        // The load moves the regiment into the fleet (event 0x320,
        // FUN_00514a60): the old container's slot +0xc8 (FUN_00508660)
        // reruns FUN_0050b610 when phase 5 serves the event (FUN_0051df30),
        // so the holder falls to support. The original: the last Imperial
        // regiment at Imperial-held Tralus taken aboard turned Tralus to the
        // Alliance the same day.
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, system, troop, fleet) = garrisoned(side);
            let mut transport = crate::troop_transport::TroopTransportState::new();
            transport
                .embark(&mut world, &ProductRoom::default(), fleet, &[troop])
                .unwrap();
            let fired = fire(&mut world, 1);
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(opposite_side(side)),
                "{side:?}"
            );
            assert_eq!(fired.controls.len(), 1, "{side:?}");
            assert!(!world.troop_held.contains(&system), "{side:?}");
        }
    }

    #[test]
    fn scrapping_destroying_or_losing_the_last_regiment_turns_its_system() {
        // Each removal clears the regiment's usable bit (FUN_004f7410 ->
        // hook +0x110 FUN_004ff5e0 -> FUN_004fb310), whose event 0x301
        // reaches its system's slot +0xc8 (FUN_00514510 -> FUN_00508660 ->
        // FUN_0050b610): scrap (+0xac), a sabotage kill (FUN_005746e0) and
        // an uprising incident's code 2 (FUN_0050d150).
        for side in [Faction::Alliance, Faction::Empire] {
            for path in 0..3 {
                let (mut world, system, troop, _) = garrisoned(side);
                match path {
                    0 => {
                        let class = world.troops[troop].class_dat_id;
                        world.buildable_classes.insert(class, BuildableClass::default());
                        let scrapped = crate::scrap::scrap(
                            &mut world,
                            &mut crate::stockpiles::StockpileState::default(),
                            &mut crate::troop_transport::TroopTransportState::new(),
                            crate::scrap::ScrapTarget::Troop(troop),
                        );
                        assert!(scrapped.is_some());
                    }
                    1 => crate::missions::destroy_target(
                        &mut world,
                        crate::missions::MissionTarget::Troop(troop),
                    ),
                    _ => crate::uprising::apply_uprising_event(
                        &mut world,
                        &crate::uprising::UprisingEvent::UprisingIncident {
                            system,
                            tick: 1,
                            side,
                            codes: [2, 0],
                            losses: vec![crate::uprising::IncidentLoss::Regiment(troop)],
                            support_delta: 0,
                        },
                    ),
                }
                fire(&mut world, 1);
                assert_eq!(
                    world.systems[system].control,
                    ControlKind::Controlled(opposite_side(side)),
                    "{side:?} path {path}"
                );
            }
        }
    }

    #[test]
    fn a_regiment_landing_on_an_unpopulated_neutral_planet_holds_it() {
        // FUN_0050b610.c:24-70: an unpopulated system's support side is 3,
        // but one side's usable regiments set bits 4-5 and the holder; the
        // landing's event 0x320 reaches the planet's slot +0xc8
        // (FUN_00514a60 -> FUN_00508660).
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, system, troop, fleet) = garrisoned(side);
            let mut transport = crate::troop_transport::TroopTransportState::new();
            for selected in [false, true] {
                transport
                    .embark(&mut world, &ProductRoom::default(), fleet, &[troop])
                    .unwrap();
                fire(&mut world, 1);
                world.systems[system].is_populated = false;
                hold(&mut world, system, ControlKind::Uncontrolled);
                if selected {
                    transport
                        .disembark_selected(&mut world, fleet, system, &[troop])
                        .unwrap();
                } else {
                    transport.disembark_all(&mut world, fleet, system).unwrap();
                }
                fire(&mut world, 1);
                assert_eq!(
                    world.systems[system].control,
                    ControlKind::Controlled(side),
                    "{side:?} {selected}"
                );
            }
        }
    }

    #[test]
    fn a_holder_rerun_raised_after_a_shift_runs_before_it() {
        // Queue +0xb0 serves the lowest slot +0x28 first (FUN_0054ee90,
        // FUN_005f61c0): a regiment's event (0x301/0x320, 0x10) runs before
        // a shift (0x373, 0x100) queued earlier, so the rerun reads the
        // other side's 70 (GNPRTB 7760 60, FUN_0055a080) and turns the
        // system to it with the old word's troop bits. The shift first
        // would leave both sides at 50 and the system neutral.
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, system, _, _) = garrisoned(side);
            disband(&mut world, system);
            let record = ShiftRecord {
                families: ALL_SYSTEMS,
                gate_side: side,
                gate_limit: 101,
                direction: side,
                amount: 20,
            };
            let sector = world.systems[system].sector;
            post(&mut world, sector, &record, ShiftCause::Bombardment);
            regiments_changed(&mut world, system);
            let fired = fire(&mut world, 1);
            assert_eq!(
                fired.controls.first(),
                Some(&ControlRerun {
                    system,
                    previous: ControlKind::Controlled(side),
                    control: ControlKind::Controlled(opposite_side(side)),
                    troops: (true, false),
                }),
                "{side:?}"
            );
            assert!(world.support_events.is_empty(), "{side:?}");
        }
    }
}
