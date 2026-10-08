//! Manufacturing system: production queues and their completions.
//!
//! Each star system keeps one queue per production area. The front unit is
//! under construction: every yard cycle (`crate::stockpiles`) draws one
//! refined unit and adds one unit of work, and the unit completes once its
//! work reaches its refined-material cost. The next unit then becomes
//! active.
//!
//! # Architecture
//!
//! Production state is kept in a `ManufacturingState` map that lives alongside
//! `GameWorld` rather than inside it. `GameWorld` stores entity class templates
//! (`CapitalShipClass`, `FighterClass`, etc.); `ManufacturingState` stores the
//! per-system work-in-progress queues.
//!
//! Each tick, the caller runs `crate::stockpiles::StockpileSystem::advance`,
//! whose yard cycles add the work, then `ManufacturingSystem::advance_tracked`,
//! which returns the finished units as `CompletionEvent`s for the caller to
//! act on (spawn units, add to fleet, etc.).

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use crate::ids::{CapitalShipKey, DatId, FighterKey, SystemKey};
use crate::tick::TickEvent;

// ---------------------------------------------------------------------------
// BuildableKind
// ---------------------------------------------------------------------------

/// The class being produced. The order names a class, as Build Selection
/// lists them (`FUN_00537ff0` -> `FUN_0052e580`), and the instance is made
/// when the unit completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildableKind {
    /// A capital ship class (Star Destroyer, Mon Cal Cruiser, etc.)
    CapitalShip(CapitalShipKey),
    /// A fighter squadron class (X-Wing, TIE Fighter, etc.)
    Fighter(FighterKey),
    /// A regiment class (TROOPSD, family `0x10`).
    Troop(DatId),
    /// A special-force class (SPECFCSD, family `0x3c`).
    SpecialForce(DatId),
    /// A planetary defense class (DEFFACSD, families `0x22..0x25`).
    DefenseFacility(FacilityBuild),
    /// A shipyard, training facility or construction yard class (MANFACSD,
    /// families `0x28..0x2a`).
    ManufacturingFacility(FacilityBuild),
    /// A mine or refinery class (PROFACSD, families `0x2c..0x2d`).
    ProductionFacility(FacilityBuild),
}

/// A facility class and the side it is built for: the facility files serve
/// both sides, so the order carries its builder's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacilityBuild {
    pub class: DatId,
    pub is_alliance: bool,
}

impl BuildableKind {
    /// The `DatId` of a regiment, special-force or facility class; ships and
    /// fighters are keyed by their class slots instead.
    #[must_use]
    pub const fn class_dat_id(self) -> Option<DatId> {
        match self {
            Self::Troop(class) | Self::SpecialForce(class) => Some(class),
            Self::DefenseFacility(build)
            | Self::ManufacturingFacility(build)
            | Self::ProductionFacility(build) => Some(build.class),
            Self::CapitalShip(_) | Self::Fighter(_) => None,
        }
    }
}

/// The first day by which `work` units of progress are done when each yard
/// adds one unit every `period` days (`FUN_00528b30`: the sum over the
/// manager's yards of `days / period`; `FUN_00528d30` searches the least
/// such day). Yards with no period add nothing; `None` when none adds any.
#[must_use]
pub fn completion_day(periods: &[u32], work: u32) -> Option<u32> {
    let periods: Vec<u32> = periods.iter().copied().filter(|&p| p > 0).collect();
    if periods.is_empty() {
        return None;
    }
    if work == 0 {
        return Some(0);
    }
    let done = |days: u32| -> u64 { periods.iter().map(|&period| u64::from(days / period)).sum() };
    // The slowest yard alone finishes by `work * max`, so the answer lies
    // in (0, work * max].
    let (mut low, mut high) = (0_u32, work.saturating_mul(*periods.iter().max()?));
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if done(middle) >= u64::from(work) {
            high = middle;
        } else {
            low = middle;
        }
    }
    Some(high)
}

// ---------------------------------------------------------------------------
// QueueItem
// ---------------------------------------------------------------------------

/// One item under construction in a system's production queue.
///
/// The first item in the queue is actively being built; the rest are waiting
/// their turn. A unit's work is its class's refined-material cost: each yard
/// cycle draws one refined unit and adds one unit of progress
/// (`FUN_00530950` → `FUN_0052a430`), and the unit completes once its
/// progress reaches the cost (`FUN_0052b960`;
/// `ghidra/notes/top-bar-resource-counters.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueItem {
    /// What is being built.
    pub kind: BuildableKind,
    /// Units of progress made: the manager's `+0x5c`.
    pub work_done: u32,
    /// Units of progress the unit needs: its class's refined-material cost,
    /// the manager's `+0x68`.
    pub total_cost: u32,
    /// Where the finished object goes; `None` keeps it at the building
    /// system. A different system makes it travel there once completed
    /// (`FUN_0052bee0`, `crate::delivery`).
    pub destination: Option<SystemKey>,
}

impl QueueItem {
    /// A unit of `kind` that needs `total_cost` units of work.
    #[must_use]
    pub fn new(kind: BuildableKind, total_cost: u32) -> Self {
        QueueItem {
            kind,
            work_done: 0,
            total_cost,
            destination: None,
        }
    }

    /// The same item, delivered to `destination` once completed.
    #[must_use]
    pub fn delivered_to(self, destination: SystemKey) -> Self {
        QueueItem {
            destination: Some(destination),
            ..self
        }
    }

    /// Units of work still to do.
    #[must_use]
    pub fn work_left(&self) -> u32 {
        self.total_cost.saturating_sub(self.work_done)
    }

    /// Whether the unit is finished: `FUN_0052b960` completes it once its
    /// progress reaches the cost. port: a unit costing nothing completes at
    /// once; the original's test needs some progress first.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.work_done >= self.total_cost
    }

    /// Progress fraction in [0.0, 1.0].
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    pub fn progress_fraction(&self) -> f32 {
        if self.total_cost == 0 {
            return 1.0;
        }
        (self.work_done as f32 / self.total_cost as f32).min(1.0)
    }
}

// ---------------------------------------------------------------------------
// ProductionQueue
// ---------------------------------------------------------------------------

/// The ordered production queue for one star system.
///
/// Items are processed front-to-back. The front item is "active": the
/// yards' cycles add to its progress. Items at index > 0 are queued.
///
/// Capacity is uncapped; the original game had a soft limit of ~5 items
/// per system enforced by the UI, not the engine.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionQueue {
    items: VecDeque<QueueItem>,
    /// Progress made beyond the active unit's cost, credited to the next
    /// unit when it starts: the manager's `+0x60` (`FUN_0052a430`,
    /// `FUN_00529d70`).
    credit: u32,
}

impl ProductionQueue {
    #[must_use]
    pub fn new() -> Self {
        ProductionQueue {
            items: VecDeque::new(),
            credit: 0,
        }
    }

    /// Append an item to the back of the queue.
    pub fn enqueue(&mut self, item: QueueItem) {
        self.items.push_back(item);
    }

    /// Remove the item at position `index` (0 = active item).
    ///
    /// Cancelling the active item does not refund costs (consistent with the
    /// original game). Returns `None` if the index is out of range.
    pub fn cancel(&mut self, index: usize) -> Option<QueueItem> {
        self.items.remove(index)
    }

    /// Move an item earlier in the queue (swap with the item ahead of it).
    ///
    /// No-ops if `index` is 0 (already at front) or out of range.
    pub fn prioritize(&mut self, index: usize) {
        if index == 0 || index >= self.items.len() {
            return;
        }
        self.items.swap(index - 1, index);
    }

    /// The item currently under construction, if any.
    #[must_use]
    pub fn active(&self) -> Option<&QueueItem> {
        self.items.front()
    }

    /// All items in queue order (index 0 = active).
    #[must_use]
    pub fn items(&self) -> &VecDeque<QueueItem> {
        &self.items
    }

    /// Units of work the queue still needs: the manager's
    /// `+0x6c − +0x60 − +0x5c` (`FUN_00529dd0`), the number of yards it
    /// keeps working.
    #[must_use]
    pub fn work_left(&self) -> u32 {
        self.items
            .iter()
            .map(QueueItem::work_left)
            .sum::<u32>()
            .saturating_sub(self.credit)
    }

    /// One yard cycle's unit of progress (`FUN_0052a430`): to the active
    /// unit while it is short of its cost, else to the next unit's credit.
    pub fn add_progress(&mut self) {
        match self.items.front_mut() {
            Some(front) if front.work_done < front.total_cost => front.work_done += 1,
            _ => self.credit += 1,
        }
    }

    /// Remove and return the finished units from the front. Each next unit
    /// starts with as much of the credit as it needs (`FUN_00529d70`:
    /// progress `min(cost, +0x60)`, taken from `+0x60`).
    fn take_completed(&mut self) -> Vec<QueueItem> {
        let mut completed = Vec::new();
        while self.items.front().is_some_and(QueueItem::is_complete) {
            completed.extend(self.items.pop_front());
            if let Some(next) = self.items.front_mut() {
                let take = next.work_left().min(self.credit);
                next.work_done += take;
                self.credit -= take;
            }
        }
        if self.items.is_empty() {
            self.credit = 0;
        }
        completed
    }

    /// The game day each item completes when the yards with these
    /// `periods` keep building from day `today` with materials on hand: the
    /// manual's "best case" (p. 84; `FUN_00528d30`). `None` for every item
    /// when no yard can build.
    #[must_use]
    pub fn completion_days(&self, today: u64, periods: &[u32]) -> Vec<Option<u64>> {
        let mut work = 0_u32;
        let mut credit = self.credit;
        self.items
            .iter()
            .map(|item| {
                let used = item.work_left().min(credit);
                credit -= used;
                work = work.saturating_add(item.work_left() - used);
                completion_day(periods, work).map(|days| today + u64::from(days))
            })
            .collect()
    }

    /// Total items, including the active one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ---------------------------------------------------------------------------
// ManufacturingState
// ---------------------------------------------------------------------------

/// A system's production area: the kind of facility a product comes from.
/// The original keeps one manager object per area at a system (families
/// `0xa0..0xaf`, `FUN_0052c170`): the agent's Destination (`0x214`) and
/// build orders act on construction managers (`0xa0..0xa1`) and training
/// managers (`0xa4..0xa5`; `ghidra/notes/manage-automation.md`), and the
/// manual gives each area its own Destination (p. 84, Fig. 3.75). hyp:
/// shipyards are `0xa2..0xa3`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ProductionArea {
    Shipyard,
    TrainingFacility,
    ConstructionYard,
}

impl ProductionArea {
    pub const ALL: [Self; 3] = [
        Self::Shipyard,
        Self::TrainingFacility,
        Self::ConstructionYard,
    ];

    /// The area that builds `kind`.
    #[must_use]
    pub const fn of(kind: BuildableKind) -> Self {
        match kind {
            BuildableKind::CapitalShip(_) | BuildableKind::Fighter(_) => Self::Shipyard,
            BuildableKind::Troop(_) | BuildableKind::SpecialForce(_) => Self::TrainingFacility,
            BuildableKind::DefenseFacility(_)
            | BuildableKind::ManufacturingFacility(_)
            | BuildableKind::ProductionFacility(_) => Self::ConstructionYard,
        }
    }
}

/// Every production area's queue in the galaxy.
///
/// The original keeps one manager per area at a system (`FUN_00509670`:
/// ships 0, facilities 1, troops 2), and each builds its own units at once:
/// the Manufacturing window's overview shows the three side by side
/// (`FUN_00455060`, `FUN_00457c90`). Areas with nothing queued are not
/// stored (lazy entry on first enqueue).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManufacturingState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    queues: HashMap<(SystemKey, ProductionArea), ProductionQueue>,
    /// Each production area's Destination (`0x214`) where it is not the
    /// area's own system.
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    destinations: HashMap<(SystemKey, ProductionArea), SystemKey>,
}

impl ManufacturingState {
    #[must_use]
    pub fn new() -> Self {
        ManufacturingState {
            queues: HashMap::new(),
            destinations: HashMap::new(),
        }
    }

    /// The queue of `system`'s `area`, created when absent.
    pub fn queue_mut(&mut self, system: SystemKey, area: ProductionArea) -> &mut ProductionQueue {
        self.queues.entry((system, area)).or_default()
    }

    /// Remove every area's queue at a system (used when system is destroyed).
    pub fn clear_queue(&mut self, system: SystemKey) {
        self.queues.retain(|(key, _), _| *key != system);
    }

    /// The queue of `system`'s `area` (read-only); `None` when never used.
    #[must_use]
    pub fn queue(&self, system: SystemKey, area: ProductionArea) -> Option<&ProductionQueue> {
        self.queues.get(&(system, area))
    }

    /// Units queued in every area at `system`.
    #[must_use]
    pub fn queued_at(&self, system: SystemKey) -> usize {
        ProductionArea::ALL
            .iter()
            .filter_map(|&area| self.queue(system, area))
            .map(ProductionQueue::len)
            .sum()
    }

    /// Enqueue an item on its area's queue at `system`. An item with no
    /// destination of its own takes its area's (hyp: a new product copies
    /// its manager's destination, `+0x74` → `+0x3c`; `build-delivery.md`).
    pub fn enqueue(&mut self, system: SystemKey, item: QueueItem) {
        let area = ProductionArea::of(item.kind);
        let item = match (item.destination, self.destination(system, area)) {
            (None, Some(destination)) => item.delivered_to(destination),
            _ => item,
        };
        self.queue_mut(system, area).enqueue(item);
    }

    /// Build: `count` units of `item` replace whatever `system`'s area of
    /// that kind was building. Manual p. 84: "Starting a new project cancels
    /// the current construction"; the units are built one after another
    /// ("Number to build", Fig. 3.25).
    pub fn build(&mut self, system: SystemKey, item: &QueueItem, count: u32) {
        self.stop(system, ProductionArea::of(item.kind));
        for _ in 0..count {
            self.enqueue(system, item.clone());
        }
    }

    /// Build with each unit's own days (`unit_build_days`): `items` replace
    /// what their area was building, in order.
    pub fn build_units(&mut self, system: SystemKey, items: Vec<QueueItem>) {
        let Some(first) = items.first() else {
            return;
        };
        self.stop(system, ProductionArea::of(first.kind));
        for item in items {
            self.enqueue(system, item);
        }
    }

    /// Stop: `system`'s `area` drops every unit it was building (manual
    /// p. 84, "Stopping Construction").
    pub fn stop(&mut self, system: SystemKey, area: ProductionArea) {
        self.queues.remove(&(system, area));
    }

    /// Where `system`'s `area` delivers what it builds, when not at home.
    #[must_use]
    pub fn destination(&self, system: SystemKey, area: ProductionArea) -> Option<SystemKey> {
        self.destinations.get(&(system, area)).copied()
    }

    /// The Destination order (`0x214`) on `system`'s `area`: what it builds,
    /// queued now or later, goes to `destination`; the area's own system
    /// clears it. hyp: the order rewrites the queued products' destination
    /// (`+0x3c`) too; its handler is untraced (`production-destination.md`).
    pub fn set_destination(
        &mut self,
        system: SystemKey,
        area: ProductionArea,
        destination: SystemKey,
    ) {
        let target = (destination != system).then_some(destination);
        match target {
            Some(destination) => self.destinations.insert((system, area), destination),
            None => self.destinations.remove(&(system, area)),
        };
        if let Some(queue) = self.queues.get_mut(&(system, area)) {
            for item in &mut queue.items {
                item.destination = target;
            }
        }
    }

    /// One yard cycle's unit of progress for `system`'s `area`; nothing when
    /// it has no queue.
    pub fn add_progress(&mut self, system: SystemKey, area: ProductionArea) {
        if let Some(queue) = self.queues.get_mut(&(system, area)) {
            queue.add_progress();
        }
    }

    /// All system queues (including empty ones that were created lazily).
    #[must_use]
    pub fn queues(&self) -> &HashMap<(SystemKey, ProductionArea), ProductionQueue> {
        &self.queues
    }
}

// ---------------------------------------------------------------------------
// CompletionEvent + ManufacturingAdvance
// ---------------------------------------------------------------------------

/// Emitted when an item finishes construction.
///
/// The caller is responsible for acting on completions: spawning fleet units,
/// registering facilities in `GameWorld`, sending a message log entry, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionEvent {
    /// The system where construction completed.
    pub system: SystemKey,
    /// The game-day on which the item completed.
    pub tick: u64,
    /// What was built.
    pub kind: BuildableKind,
}

/// Wraps the two outputs of `ManufacturingSystem::advance_tracked` so the
/// integrator can emit both `EVT_BUILD_COMPLETE`/`EVT_UNITS_DEPLOYED` (K5)
/// and `EVT_MANUFACTURING_IDLE` (K6) without extra world-state plumbing.
///
/// `newly_idle` is the set of system keys whose production queue
/// transitioned from non-empty to empty during this advance call. Detection
/// is intra-tick (pre/post length compare — no persistent "`was_empty`" bit).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManufacturingAdvance {
    pub completions: Vec<CompletionEvent>,
    pub newly_idle: Vec<SystemKey>,
    /// Completed items bound for another system; `crate::delivery` times
    /// their travel and completes them there on arrival.
    pub departures: Vec<Departure>,
}

/// A completed item leaving its building system for its destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Departure {
    /// The building system.
    pub origin: SystemKey,
    /// Where the item goes.
    pub destination: SystemKey,
    /// The game-day on which the item completed.
    pub tick: u64,
    /// What was built.
    pub kind: BuildableKind,
}

// ---------------------------------------------------------------------------
// ManufacturingSystem
// ---------------------------------------------------------------------------

/// Stateless system that advances production queues per tick.
///
/// Call `advance` once per frame, passing the `Vec<TickEvent>` from
/// `GameClock::advance`. Returns all items that completed during those ticks.
pub struct ManufacturingSystem;

impl ManufacturingSystem {
    /// Complete every queued unit whose work is done (the yards' cycles add
    /// it: `crate::stockpiles`), stamped with the last tick in `tick_events`.
    ///
    /// Systems in `blocked_systems` (e.g., blockaded systems) are skipped —
    /// their queues do not advance while blocked.
    ///
    /// Returns a `Vec<CompletionEvent>` — one entry per completed item,
    /// across all systems. Empty if no items completed this frame.
    ///
    /// Back-compat wrapper around [`advance_tracked`] that discards the
    /// `newly_idle` slot. New callers should use `advance_tracked` directly
    /// so they can route `EVT_MANUFACTURING_IDLE` (K6) telemetry.
    pub fn advance(
        state: &mut ManufacturingState,
        tick_events: &[TickEvent],
    ) -> Vec<CompletionEvent> {
        Self::advance_tracked(state, tick_events, &HashSet::new()).completions
    }

    /// Like `advance`, but skips systems in `blocked_systems`.
    ///
    /// Called by legacy paths that only care about completions. New paths
    /// should prefer `advance_tracked` so idle-transition telemetry is
    /// routed through `EVT_MANUFACTURING_IDLE` (K6).
    pub fn advance_with_blockade(
        state: &mut ManufacturingState,
        tick_events: &[TickEvent],
        blocked_systems: &HashSet<SystemKey>,
    ) -> Vec<CompletionEvent> {
        Self::advance_tracked(state, tick_events, blocked_systems).completions
    }

    /// Complete with full tracking: completions **and** K6 idle transitions.
    ///
    /// The `newly_idle` vec contains system keys whose queue transitioned
    /// from non-empty (1+ items before advance) to empty (0 items after
    /// advance). Detection is purely intra-tick — pre/post length compare
    /// against a local snapshot. No persistent `was_empty` bit on world
    /// state (SIMP-H4).
    ///
    /// # Panics
    /// Panics if a queue key collected for this batch is absent when its queue is advanced.
    pub fn advance_tracked(
        state: &mut ManufacturingState,
        tick_events: &[TickEvent],
        blocked_systems: &HashSet<SystemKey>,
    ) -> ManufacturingAdvance {
        let Some(last_tick_event) = tick_events.last() else {
            return ManufacturingAdvance::default();
        };

        // The last tick number in this batch (used as the completion timestamp).
        let final_tick = last_tick_event.tick;

        // K6 intra-tick detection: capture `pre_len` inside the iter loop
        // itself — no scratch HashMap, no per-tick allocation. A system that
        // started non-empty and ends empty is "newly idle" (SIMP-H4).
        let mut completions = Vec::new();
        let mut newly_idle = Vec::new();
        let mut departures = Vec::new();

        // HashMap iteration order is randomized per process. Completion order
        // mutates slotmaps downstream, so walk queues by stable key: each
        // system's areas in `ProductionArea` order.
        let mut queue_keys: Vec<_> = state.queues.keys().copied().collect();
        queue_keys.sort_unstable();

        let mut pre_len = 0;
        for (index, &(system_key, area)) in queue_keys.iter().enumerate() {
            // Skip blockaded systems — manufacturing halted and they can't
            // transition idle while blocked.
            if blocked_systems.contains(&system_key) {
                continue;
            }
            let queue = state
                .queues
                .get_mut(&(system_key, area))
                .expect("manufacturing queue key collected from the same map");
            pre_len += queue.len();
            for item in queue.take_completed() {
                match item
                    .destination
                    .filter(|&destination| destination != system_key)
                {
                    Some(destination) => departures.push(Departure {
                        origin: system_key,
                        destination,
                        tick: final_tick,
                        kind: item.kind,
                    }),
                    None => completions.push(CompletionEvent {
                        system: system_key,
                        tick: final_tick,
                        kind: item.kind,
                    }),
                }
            }
            // port: K6 stays per system: it fires once every area there has
            // gone idle.
            let last_of_system = queue_keys
                .get(index + 1)
                .is_none_or(|&(next, _)| next != system_key);
            if last_of_system {
                if pre_len > 0 && state.queued_at(system_key) == 0 {
                    newly_idle.push(system_key);
                }
                pre_len = 0;
            }
        }

        ManufacturingAdvance {
            completions,
            newly_idle,
            departures,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::SystemKey;

    // Helper: fabricate distinct SystemKeys from one shared slotmap.
    fn mock_system_keys(n: usize) -> Vec<SystemKey> {
        let mut sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        (0..n).map(|_| sm.insert(())).collect()
    }

    fn mock_system_key() -> SystemKey {
        mock_system_keys(1).into_iter().next().unwrap()
    }

    fn cap_ship_item(cost: u32) -> QueueItem {
        // We need a CapitalShipKey to build the kind. Use a slotmap.
        let mut sm: slotmap::SlotMap<CapitalShipKey, ()> = slotmap::SlotMap::with_key();
        let key = sm.insert(());
        QueueItem::new(BuildableKind::CapitalShip(key), cost)
    }

    fn fighter_item(cost: u32) -> QueueItem {
        let mut sm: slotmap::SlotMap<FighterKey, ()> = slotmap::SlotMap::with_key();
        let key = sm.insert(());
        QueueItem::new(BuildableKind::Fighter(key), cost)
    }

    fn troop_item(cost: u32) -> QueueItem {
        QueueItem::new(BuildableKind::Troop(DatId::new(0x1000_0001)), cost)
    }

    /// `units` yard cycles' progress on `system`'s `area`.
    fn work(state: &mut ManufacturingState, system: SystemKey, area: ProductionArea, units: u32) {
        for _ in 0..units {
            state.add_progress(system, area);
        }
    }

    fn ticks(range: std::ops::RangeInclusive<u64>) -> Vec<TickEvent> {
        range.map(|tick| TickEvent { tick }).collect()
    }

    // FUN_00528b30: each yard adds days / period units, and FUN_00528d30
    // finds the least day whose sum reaches the work. MANFACSD gives a
    // standard yard period 4 and an advanced one 2.
    #[test]
    fn a_build_ends_on_the_first_day_the_yards_progress_covers_its_work() {
        assert_eq!(completion_day(&[4], 8), Some(32));
        assert_eq!(completion_day(&[4, 2], 6), Some(8));
        assert_eq!(completion_day(&[4, 4], 3), Some(8));
        assert_eq!(completion_day(&[4, 0], 1), Some(4));
        assert_eq!(completion_day(&[], 5), None);
        assert_eq!(completion_day(&[0], 5), None);
        assert_eq!(completion_day(&[4], 0), Some(0));
    }

    #[test]
    fn build_units_replaces_the_areas_queue_with_the_units_in_order() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(9));
        state.enqueue(system, fighter_item(9));
        state.build_units(system, vec![cap_ship_item(8), cap_ship_item(4)]);
        let queue = state.queue(system, ProductionArea::Shipyard).unwrap();
        let costs: Vec<u32> = queue.items().iter().map(|item| item.total_cost).collect();
        assert_eq!(costs, [8, 4]);
        state.build_units(system, Vec::new());
        assert_eq!(
            state.queue(system, ProductionArea::Shipyard).unwrap().len(),
            2
        );
    }

    // FUN_0052bee0: a product built for another system leaves its facility
    // en route on completion; one built for its own system completes there.
    #[test]
    fn a_build_for_another_system_departs_and_one_for_its_own_completes_there() {
        let systems = mock_system_keys(3);
        let mut state = ManufacturingState::new();
        state.enqueue(systems[0], cap_ship_item(1).delivered_to(systems[1]));
        state.enqueue(systems[2], fighter_item(1).delivered_to(systems[2]));
        work(&mut state, systems[0], ProductionArea::Shipyard, 1);
        work(&mut state, systems[2], ProductionArea::Shipyard, 1);

        let advance =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(9..=9), &HashSet::new());

        assert_eq!(
            advance.departures,
            vec![Departure {
                origin: systems[0],
                destination: systems[1],
                tick: 9,
                kind: cap_ship_item(1).kind,
            }]
        );
        assert_eq!(advance.completions.len(), 1);
        assert_eq!(advance.completions[0].system, systems[2]);
    }

    // FUN_00509670: a system has one manager per area (ships 0, facilities
    // 1, troops 2) and the overview shows each one's product at once
    // (FUN_00455060, FUN_00457c90).
    #[test]
    fn a_systems_shipyard_and_training_facility_build_at_the_same_time() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(2));
        state.enqueue(system, troop_item(2));
        assert_eq!(state.queued_at(system), 2);
        work(&mut state, system, ProductionArea::Shipyard, 2);
        work(&mut state, system, ProductionArea::TrainingFacility, 2);

        let advance =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(1..=2), &HashSet::new());

        assert_eq!(advance.completions.len(), 2);
        assert_eq!(advance.newly_idle, [system]);
        assert_eq!(state.queued_at(system), 0);
    }

    // K6 waits for the last busy area at a system.
    #[test]
    fn a_system_goes_idle_only_when_every_area_has() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(1));
        state.enqueue(system, troop_item(3));
        work(&mut state, system, ProductionArea::Shipyard, 1);
        work(&mut state, system, ProductionArea::TrainingFacility, 1);

        let first =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(1..=1), &HashSet::new());
        assert_eq!(first.completions.len(), 1);
        assert!(first.newly_idle.is_empty());

        work(&mut state, system, ProductionArea::TrainingFacility, 2);
        let second =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(2..=3), &HashSet::new());
        assert_eq!(second.completions.len(), 1);
        assert_eq!(second.newly_idle, [system]);
    }

    // Manual p. 84: a new project cancels the current construction, and its
    // Number to build units are built one after another (Fig. 3.25).
    #[test]
    fn building_replaces_only_that_areas_units() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(9));
        state.enqueue(system, troop_item(4));

        state.build(system, &cap_ship_item(2), 3);

        let ships = state.queue(system, ProductionArea::Shipyard).unwrap();
        assert_eq!(ships.len(), 3);
        assert!(ships.items().iter().all(|item| item.total_cost == 2));
        assert_eq!(
            state
                .queue(system, ProductionArea::TrainingFacility)
                .unwrap()
                .len(),
            1
        );
    }

    // Manual p. 84, "Stopping Construction".
    #[test]
    fn stopping_an_area_drops_its_units_and_keeps_the_others() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.build(system, &cap_ship_item(2), 2);
        state.enqueue(system, troop_item(4));

        state.stop(system, ProductionArea::Shipyard);

        assert!(state.queue(system, ProductionArea::Shipyard).is_none());
        assert_eq!(state.queued_at(system), 1);
    }

    #[test]
    fn clearing_a_system_drops_every_area() {
        let keys = mock_system_keys(2);
        let mut state = ManufacturingState::new();
        state.enqueue(keys[0], cap_ship_item(2));
        state.enqueue(keys[0], troop_item(2));
        state.enqueue(keys[1], troop_item(2));

        state.clear_queue(keys[0]);

        assert_eq!(state.queued_at(keys[0]), 0);
        assert_eq!(state.queued_at(keys[1]), 1);
    }

    // --- ProductionQueue tests ---

    #[test]
    fn enqueued_item_becomes_the_active_production() {
        let mut q = ProductionQueue::new();
        assert!(q.active().is_none());

        q.enqueue(cap_ship_item(10));
        let active = q.active().unwrap();
        assert_eq!((active.work_done, active.total_cost), (0, 10));
    }

    #[test]
    fn each_production_area_keeps_its_own_destination() {
        // Manual p. 84, Fig. 3.75: Destination per production area; the
        // agent's 0x214 acts on one area's manager (manage-automation.md).
        let mut systems: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let (home, away) = (systems.insert(()), systems.insert(()));
        let mut state = ManufacturingState::new();
        state.enqueue(home, cap_ship_item(10));
        state.enqueue(home, troop_item(5));

        state.set_destination(home, ProductionArea::Shipyard, away);
        let destinations = |state: &ManufacturingState| {
            ProductionArea::ALL
                .iter()
                .filter_map(|&area| state.queue(home, area))
                .flat_map(|queue| queue.items().iter().map(|item| item.destination))
                .collect::<Vec<_>>()
        };
        assert_eq!(destinations(&state), [Some(away), None]);
        assert_eq!(
            state.destination(home, ProductionArea::Shipyard),
            Some(away)
        );
        assert_eq!(
            state.destination(home, ProductionArea::TrainingFacility),
            None
        );

        // A later product of that area takes it; another area's does not.
        state.enqueue(home, cap_ship_item(3));
        state.enqueue(home, troop_item(5));
        assert_eq!(destinations(&state), [Some(away), Some(away), None, None]);

        // Back at home clears it.
        state.set_destination(home, ProductionArea::Shipyard, home);
        assert_eq!(destinations(&state), [None, None, None, None]);
        assert_eq!(state.destination(home, ProductionArea::Shipyard), None);
    }

    // Manual Fig. 3.58: completion is a game day. Manual p. 84: the time is
    // a best case, FUN_00528d30 over the yards' periods; each unit waits
    // for the work of those ahead of it.
    #[test]
    fn each_queued_item_completes_after_the_work_ahead_of_it() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        q.enqueue(cap_ship_item(4));
        for _ in 0..3 {
            q.add_progress();
        }
        assert_eq!(q.completion_days(100, &[1]), [Some(107), Some(111)]);
        assert_eq!(q.completion_days(100, &[2, 2]), [Some(108), Some(112)]);
        assert_eq!(q.completion_days(100, &[]), [None, None]);
        assert!(ProductionQueue::new().completion_days(5, &[1]).is_empty());
    }

    // FUN_00529dd0: the manager keeps working as many yards as there are
    // units of work left, +0x6c − +0x60 − +0x5c.
    #[test]
    fn the_work_left_is_the_queues_cost_less_progress_and_credit() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(3));
        q.enqueue(cap_ship_item(4));
        assert_eq!(q.work_left(), 7);
        for _ in 0..5 {
            q.add_progress();
        }
        assert_eq!(q.work_left(), 2);
    }

    // FUN_0052a430: a cycle adds to the active unit while it is short of its
    // cost.
    #[test]
    fn a_cycle_adds_one_unit_of_progress_to_the_active_unit() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        for _ in 0..4 {
            q.add_progress();
        }
        assert!(q.take_completed().is_empty());
        assert_eq!(q.active().unwrap().work_done, 4);
        assert_eq!(q.active().unwrap().work_left(), 6);
    }

    // FUN_0052b960: the unit completes once its progress reaches its cost.
    #[test]
    fn a_unit_completes_when_its_progress_reaches_its_cost() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(5));
        for _ in 0..5 {
            q.add_progress();
        }
        assert_eq!(q.take_completed().len(), 1);
        assert!(q.is_empty());
    }

    // FUN_0052a430: progress beyond the active unit's cost goes to the
    // credit +0x60, and FUN_00529d70 starts the next unit with
    // min(cost, credit).
    #[test]
    fn progress_beyond_a_units_cost_starts_the_next_unit() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(3));
        q.enqueue(fighter_item(10));
        for _ in 0..5 {
            q.add_progress();
        }
        assert_eq!(q.take_completed().len(), 1);
        assert_eq!(q.active().unwrap().work_done, 2);
    }

    #[test]
    fn enough_credit_completes_several_units_at_once() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(2));
        q.enqueue(cap_ship_item(3));
        q.enqueue(cap_ship_item(4));
        for _ in 0..9 {
            q.add_progress();
        }
        assert_eq!(q.take_completed().len(), 3);
        assert!(q.is_empty());
        assert_eq!(q.work_left(), 0);
    }

    #[test]
    fn canceling_the_active_item_promotes_the_next() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        q.enqueue(fighter_item(5));

        q.cancel(0);
        assert_eq!(q.len(), 1);
        assert_eq!(q.active().unwrap().total_cost, 5);
    }

    #[test]
    fn prioritize_moves_item_forward() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10)); // index 0
        q.enqueue(fighter_item(5)); // index 1

        q.prioritize(1);
        // Fighter should now be at index 0
        match q.active().unwrap().kind {
            BuildableKind::Fighter(_) => {}
            _ => panic!("expected fighter at front after prioritize"),
        }
    }

    #[test]
    fn prioritize_noop_on_front() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        q.prioritize(0); // no-op
        assert_eq!(q.active().unwrap().total_cost, 10);
    }

    #[test]
    fn progress_fraction_reports_half_after_half_the_work() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        for _ in 0..5 {
            q.add_progress();
        }
        let frac = q.active().unwrap().progress_fraction();
        assert!((frac - 0.5).abs() < 0.001, "expected ~0.5, got {frac}");
    }

    // --- ManufacturingSystem integration tests ---

    // The days alone build nothing: the work comes from yard cycles
    // (FUN_00530950), which draw refined material.
    #[test]
    fn days_without_yard_work_complete_nothing() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(2));

        let completions = ManufacturingSystem::advance(&mut state, &ticks(1..=30));
        assert!(completions.is_empty());
        assert_eq!(
            state
                .queue(system, ProductionArea::Shipyard)
                .unwrap()
                .active()
                .unwrap()
                .work_done,
            0
        );
    }

    #[test]
    fn a_completion_is_stamped_with_the_batchs_last_tick() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(3));
        work(&mut state, system, ProductionArea::Shipyard, 3);

        let completions = ManufacturingSystem::advance(&mut state, &ticks(1..=3));
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].system, system);
        assert_eq!(completions[0].tick, 3);
        assert!(state
            .queue(system, ProductionArea::Shipyard)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn multiple_systems_complete_independently() {
        let keys = mock_system_keys(2);
        let (sys_a, sys_b) = (keys[0], keys[1]);
        let mut state = ManufacturingState::new();
        state.enqueue(sys_a, cap_ship_item(2));
        state.enqueue(sys_b, cap_ship_item(5));
        work(&mut state, sys_a, ProductionArea::Shipyard, 2);
        work(&mut state, sys_b, ProductionArea::Shipyard, 2);

        let completions = ManufacturingSystem::advance(&mut state, &ticks(1..=2));

        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].system, sys_a);
        assert_eq!(
            state
                .queue(sys_b, ProductionArea::Shipyard)
                .unwrap()
                .work_left(),
            3
        );
    }

    // -----------------------------------------------------------------------
    // Knesset Shamash-Bet Dabora 2 #K6 — EVT_MANUFACTURING_IDLE (0x160)
    // intra-tick transition idempotency. Part of the #K7 parameterized
    // idempotency suite (the economy K1–K4 tests live in economy.rs).
    // -----------------------------------------------------------------------

    #[test]
    fn k6_manufacturing_idle_fires_on_empty_transition_only() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(2));
        work(&mut state, system, ProductionArea::Shipyard, 2);

        // First advance completes the single item, and the queue
        // transitions from non-empty → empty → fires K6.
        let advance =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(1..=2), &HashSet::new());
        assert_eq!(advance.completions.len(), 1);
        assert_eq!(
            advance.newly_idle,
            vec![system],
            "K6: queue empty transition fires once"
        );

        // Second advance: queue is already empty — pre/post length match,
        // transition detection must NOT emit again.
        let advance2 =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(3..=3), &HashSet::new());
        assert!(advance2.completions.is_empty());
        assert!(
            advance2.newly_idle.is_empty(),
            "K6: already-empty queue must not re-fire"
        );
    }

    #[test]
    fn k6_manufacturing_idle_skips_blockaded_systems() {
        let keys = mock_system_keys(2);
        let (sys_a, sys_b) = (keys[0], keys[1]);
        let mut state = ManufacturingState::new();
        state.enqueue(sys_a, cap_ship_item(2));
        state.enqueue(sys_b, cap_ship_item(2));
        work(&mut state, sys_a, ProductionArea::Shipyard, 2);
        work(&mut state, sys_b, ProductionArea::Shipyard, 2);

        // Blockade sys_a — it must NOT complete, so no idle transition.
        let mut blocked = HashSet::new();
        blocked.insert(sys_a);

        let advance = ManufacturingSystem::advance_tracked(&mut state, &ticks(1..=2), &blocked);
        assert_eq!(advance.completions.len(), 1, "only sys_b should complete");
        assert_eq!(advance.completions[0].system, sys_b);
        assert_eq!(
            advance.newly_idle,
            vec![sys_b],
            "only sys_b transitions to idle"
        );
        assert!(
            !advance.newly_idle.contains(&sys_a),
            "K6: blockaded systems must never appear in newly_idle"
        );
    }

    #[test]
    fn completions_use_stable_system_key_order() {
        let keys = mock_system_keys(3);
        let mut state = ManufacturingState::new();
        for &system in keys.iter().rev() {
            state.enqueue(system, cap_ship_item(1));
            work(&mut state, system, ProductionArea::Shipyard, 1);
        }

        let advance =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(1..=1), &HashSet::new());
        let completion_systems: Vec<_> = advance
            .completions
            .iter()
            .map(|completion| completion.system)
            .collect();

        assert_eq!(completion_systems, keys);
        assert_eq!(advance.newly_idle, keys);
    }
}
