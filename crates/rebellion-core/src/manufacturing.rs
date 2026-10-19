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

use crate::ids::{CapitalShipKey, DatId, FighterKey, FleetKey, SystemKey};
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
    /// system. A different system or a fleet makes it travel there once
    /// completed (`FUN_0052bee0`, `crate::delivery`).
    pub destination: Option<Destination>,
    /// The tag of the ship a squadron or regiment bound for a fleet rides,
    /// chosen at the order (`FUN_005385f0` -> `FUN_00551a20` ->
    /// `FUN_00552300`), so it takes that ship's room from then
    /// (`crate::carriage::ProductRoom`); 0 for any other product.
    pub carrier: u32,
    /// The tag of the unfinished capital ship this entry builds, the key the
    /// manager lists (`+0x54 -> +0x18`): the ship exists from the order
    /// ([`seat_ordered_ships`]); 0 until it is seated, and for any other
    /// product.
    pub ship: u32,
}

/// Where a production area sends what it builds (the manager's `+0x74`,
/// copied to the product's `+0x3c`): a system, or a fleet whose container
/// takes the product (`production-destination.md`: the Destination
/// targeting release takes a planet or a fleet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Destination {
    System(SystemKey),
    Fleet(FleetKey),
}

impl Destination {
    /// The system it stands for now: a fleet's location, or `None` once
    /// the fleet is gone.
    #[must_use]
    pub fn system(self, world: &crate::world::GameWorld) -> Option<SystemKey> {
        match self {
            Self::System(system) => Some(system),
            Self::Fleet(fleet) => world.fleets.get(fleet).map(|f| f.location),
        }
    }
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
            carrier: 0,
            ship: 0,
        }
    }

    /// The same item, delivered to `destination` once completed.
    #[must_use]
    pub fn delivered_to(self, destination: Destination) -> Self {
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
/// manual gives each area its own Destination (p. 84, Fig. 3.75).
/// Shipyards are `0xa2..0xa3`: the system initializer creates
/// `0xa2000006` with kind 0, ships and fighters (`FUN_0050e200.c:24-38`),
/// and the Empire game's saves show Drall's `0xa2000500` listing an
/// unfinished capital ship and Duros's `0xa0000523` a facility.
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
    destinations: HashMap<(SystemKey, ProductionArea), Destination>,
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

    /// Take every product queued at `system`, area by area in
    /// [`ProductionArea::ALL`] order, leaving its queues empty.
    pub fn take_queued(&mut self, system: SystemKey) -> Vec<QueueItem> {
        ProductionArea::ALL
            .iter()
            .filter_map(|&area| self.queues.remove(&(system, area)))
            .flat_map(|queue| queue.items)
            .collect()
    }

    /// Every queued product with its producer and area, producer by producer
    /// in key order and each queue front to back.
    pub fn items(&self) -> impl Iterator<Item = (SystemKey, ProductionArea, &QueueItem)> {
        let mut keys: Vec<&(SystemKey, ProductionArea)> = self.queues.keys().collect();
        keys.sort_unstable();
        keys.into_iter().flat_map(move |key| {
            self.queues[key]
                .items
                .iter()
                .map(move |item| (key.0, key.1, item))
        })
    }

    /// Every queued product, mutably, in [`Self::items`] order.
    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut QueueItem> {
        let mut queues: Vec<(&(SystemKey, ProductionArea), &mut ProductionQueue)> =
            self.queues.iter_mut().collect();
        queues.sort_unstable_by_key(|(key, _)| **key);
        queues
            .into_iter()
            .flat_map(|(_, queue)| queue.items.iter_mut())
    }

    /// Walk the products other systems' areas build for `system` (their
    /// Destination `+0x3c` names it), producer by producer in key order and
    /// each queue front to back. `keep` may rewrite a product's destination
    /// and returns whether it stays in its queue; a kept product keeps its
    /// slot.
    pub fn retain_bound_for(
        &mut self,
        system: SystemKey,
        mut keep: impl FnMut(SystemKey, &mut QueueItem) -> bool,
    ) {
        let mut keys: Vec<(SystemKey, ProductionArea)> = self
            .queues
            .keys()
            .copied()
            .filter(|&(at, _)| at != system)
            .collect();
        keys.sort_unstable();
        for (at, area) in keys {
            let Some(queue) = self.queues.get_mut(&(at, area)) else {
                continue;
            };
            queue.items.retain_mut(|item| {
                item.destination != Some(Destination::System(system)) || keep(at, item)
            });
        }
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
    pub fn destination(&self, system: SystemKey, area: ProductionArea) -> Option<Destination> {
        self.destinations.get(&(system, area)).copied()
    }

    /// The Destination order (`0x214`) on `system`'s `area`: what it builds,
    /// queued now or later, goes to `destination`; the area's own system
    /// clears it. hyp: the order rewrites the queued products' destination
    /// (`+0x3c`) too, and a rewritten product leaves the ship it was bound
    /// for; its handler is untraced (`production-destination.md`). The
    /// order as given is `crate::build_selection::order_destination`, which
    /// seats the products again.
    pub fn set_destination(
        &mut self,
        system: SystemKey,
        area: ProductionArea,
        destination: Destination,
    ) {
        let target = (destination != Destination::System(system)).then_some(destination);
        match target {
            Some(destination) => self.destinations.insert((system, area), destination),
            None => self.destinations.remove(&(system, area)),
        };
        if let Some(queue) = self.queues.get_mut(&(system, area)) {
            for item in &mut queue.items {
                if item.destination != target {
                    item.destination = target;
                    item.carrier = 0;
                }
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

/// Keep the ordered capital ships in line with the queues, in the order the
/// original's Build runs (`FUN_00529ab0`: `FUN_0052a2c0`, then
/// `FUN_0052a140`):
///
/// 1. An entry whose ship is gone, or no longer unfinished, leaves its queue
///    (`FUN_0052a710` drops a key `FUN_00528890` no longer accepts).
/// 2. An unfinished ship no entry names is destroyed (`FUN_0052a2c0`: a
///    Stop, a Build's replacement, the yard's system destroyed).
/// 3. Each queue's unseated capital-ship entries become unfinished ships
///    (`FUN_004f7d50(.., created 1, completed 0)`) under fresh tags. Those
///    bound for a system join the side's bit-1 spare fleet there
///    (`FUN_00552300.c:127-131` -> `FUN_005097d0`), which keeps its bit, and
///    so takes every later ship bound there, until it is destroyed
///    (`FUN_0050bc60.c:41-51`); those bound for a fleet join it
///    (`FUN_00552300.c:100-118`). Ship and fleet are numbered on the order
///    day (`FUN_005140c0.c:127` -> `FUN_00517760`).
/// 4. A fleet step 2 left with no ship disbands (`FUN_004fe630.c:21-28`);
///    one a new ship joined in step 3 stays (Wine, the ds-b save, day 151).
///
/// The order's callers run it at once; a step that completes ships runs it
/// after [`finish_ships`].
pub fn seat_ordered_ships(
    world: &mut crate::world::GameWorld,
    manufacturing: &mut ManufacturingState,
) {
    let unfinished: HashSet<u32> = world
        .fleets
        .values()
        .flat_map(|fleet| &fleet.capital_ships)
        .filter(|ship| ship.alive && !ship.completed && ship.tag != 0)
        .map(|ship| ship.tag)
        .collect();
    for queue in manufacturing.queues.values_mut() {
        queue
            .items
            .retain(|item| item.ship == 0 || unfinished.contains(&item.ship));
    }

    let named: HashSet<u32> = manufacturing
        .queues
        .values()
        .flat_map(|queue| &queue.items)
        .map(|item| item.ship)
        .filter(|&tag| tag != 0)
        .collect();
    let mut emptied = Vec::new();
    for (key, fleet) in &mut world.fleets {
        let before = fleet.capital_ships.len();
        fleet
            .capital_ships
            .retain(|ship| ship.completed || !ship.alive || named.contains(&ship.tag));
        if fleet.capital_ships.len() != before {
            emptied.push(key);
        }
    }

    let mut keys: Vec<(SystemKey, ProductionArea)> = manufacturing.queues.keys().copied().collect();
    keys.sort_unstable();
    for key in keys {
        let Some(queue) = manufacturing.queues.get_mut(&key) else {
            continue;
        };
        for item in queue.items.iter_mut().filter(|item| item.ship == 0) {
            let BuildableKind::CapitalShip(class) = item.kind else {
                continue;
            };
            let Some((hull, is_alliance)) = world
                .capital_ship_classes
                .get(class)
                .map(|value| (value.hull.cast_signed(), value.is_alliance))
            else {
                continue;
            };
            let destination = item.destination.unwrap_or(Destination::System(key.0));
            let Some(fleet) = seat_fleet(world, destination, is_alliance) else {
                continue;
            };
            world.last_ship_tag += 1;
            let mut ship = crate::world::ShipInstance::ordered(class, hull, is_alliance);
            ship.tag = world.last_ship_tag;
            item.ship = ship.tag;
            world.fleets[fleet].capital_ships.push(ship);
        }
    }
    for fleet in emptied {
        world.remove_fleet_if_empty(fleet);
    }
    world.number_capital_ships();
}

/// The fleet an ordered ship of a side joins at `destination`: the
/// destination fleet itself (it takes capital ships, `FUN_004fe160`), or the
/// side's bit-1 spare at the destination system
/// ([`crate::world::GameWorld::spare_fleet_or_new`], `FUN_005097d0`).
/// `None` when neither exists.
fn seat_fleet(
    world: &mut crate::world::GameWorld,
    destination: Destination,
    is_alliance: bool,
) -> Option<FleetKey> {
    let system = match destination {
        Destination::Fleet(fleet) => return world.fleets.contains_key(fleet).then_some(fleet),
        Destination::System(system) => system,
    };
    world.systems.get(system)?;
    Some(world.spare_fleet_or_new(system, is_alliance))
}

/// The production managers of each system whose holder changed
/// (`GameWorld::manager_side_changes`) take the new side. The holder
/// change walks the system's children (`FUN_00510820` → `FUN_004fae40`,
/// slot `+0xb0`). A manager is completed, and its class accepts any side
/// (slot `+0x50`, `FUN_0051ebb0`), so `FUN_004f8680.c:28-31` sets its side
/// (`FUN_004f6f40`). The manager's side hook (slot `+0xf0`,
/// `FUN_0052b790.c:7-14`) then runs `FUN_0052a2c0`, which destroys every
/// product the manager lists with reason `0x16` (`FUN_0052a2c0.c:20-36`):
/// any kind, wherever it sits, refunding nothing (`FUN_00530270`). The
/// ships those entries named leave their fleets, and a fleet left with no
/// ship disbands (`FUN_004fe630.c:21-28`); no other ship changes, names
/// included, since numbering belongs to the order day (`FUN_00517760`).
/// Another system's product bound here keeps its queue slot, since its
/// manager keeps its side. Returns the destroyed products.
pub fn serve_manager_sides(
    world: &mut crate::world::GameWorld,
    manufacturing: &mut ManufacturingState,
) -> Vec<BuildableKind> {
    let destroyed: Vec<QueueItem> = std::mem::take(&mut world.manager_side_changes)
        .into_iter()
        .flat_map(|system| manufacturing.take_queued(system))
        .collect();
    let ships: HashSet<u32> = destroyed
        .iter()
        .map(|item| item.ship)
        .filter(|&tag| tag != 0)
        .collect();
    if !ships.is_empty() {
        let mut emptied = Vec::new();
        for (key, fleet) in &mut world.fleets {
            let before = fleet.capital_ships.len();
            fleet.capital_ships.retain(|ship| !ships.contains(&ship.tag));
            if fleet.capital_ships.len() != before {
                emptied.push(key);
            }
        }
        for fleet in emptied {
            world.remove_fleet_if_empty(fleet);
        }
    }
    destroyed.into_iter().map(|item| item.kind).collect()
}

/// The Destination order on a production area with units queued
/// (`FUN_00537180.c:321-345`, executed by `FUN_00529cd0` ->
/// `FUN_0052a380`): every unfinished ship the area builds moves at once
/// into the container its new leg chose, the side's spare at the new
/// system or the destination fleet, and a fleet it leaves with no ship
/// disbands (`FUN_004fe630`). Its progress stays.
pub fn move_ordered_ships(
    world: &mut crate::world::GameWorld,
    manufacturing: &ManufacturingState,
    system: SystemKey,
    area: ProductionArea,
) {
    let Some(queue) = manufacturing.queue(system, area) else {
        return;
    };
    let tags: Vec<u32> = queue
        .items
        .iter()
        .map(|item| item.ship)
        .filter(|&tag| tag != 0)
        .collect();
    let destination = manufacturing
        .destination(system, area)
        .unwrap_or(Destination::System(system));
    let mut moving = Vec::new();
    let mut left = Vec::new();
    for (key, fleet) in &mut world.fleets {
        if matches!(destination, Destination::Fleet(target) if target == key) {
            continue;
        }
        let before = fleet.capital_ships.len();
        let (taken, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut fleet.capital_ships)
            .into_iter()
            .partition(|ship| ship.alive && !ship.completed && tags.contains(&ship.tag));
        fleet.capital_ships = kept;
        if fleet.capital_ships.len() != before {
            left.push(key);
        }
        moving.extend(taken);
    }
    moving.sort_by_key(|ship| tags.iter().position(|&tag| tag == ship.tag));
    for ship in moving {
        let is_alliance = world
            .capital_ship_classes
            .get(ship.class)
            .is_some_and(|class| class.is_alliance);
        let Some(fleet) = seat_fleet(world, destination, is_alliance) else {
            continue;
        };
        world.fleets[fleet].capital_ships.push(ship);
    }
    for fleet in left {
        world.remove_fleet_if_empty(fleet);
    }
}

/// A capital ship its yard finished (`FUN_0052b960` -> `FUN_0052bee0`):
/// the ordered ship's tag and the yard's system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishedShip {
    /// The ordered ship ([`QueueItem::ship`]).
    pub tag: u32,
    /// The system of the yard that built it (the manager's location).
    pub yard: SystemKey,
    /// The day it was finished.
    pub tick: u64,
}

/// Complete each finished ship in place (`FUN_0052bee0`: deployed, then
/// completed). When its fleet lies in another system than its yard, the
/// deploy event starts its own leg there (`FUN_005150c0.c:90-102` ->
/// `FUN_00556430`), the ship already a member of that fleet. A fleet in
/// hyperspace is already its destination's child (`FUN_00556390`), so the
/// leg ends there ([`crate::movement::listed_location`]), as
/// `delivery::advance_ship_legs` reads it.
pub fn finish_ships(
    world: &mut crate::world::GameWorld,
    movement: &crate::movement::MovementState,
    finished: &[FinishedShip],
) {
    for done in finished {
        let Some(fleet) = crate::carriage::fleet_of_ship(world, done.tag) else {
            continue;
        };
        let Some(location) = crate::movement::listed_location(movement, world, fleet) else {
            continue;
        };
        let Some(index) = world.fleets[fleet]
            .capital_ships
            .iter()
            .position(|ship| ship.alive && ship.tag == done.tag)
        else {
            continue;
        };
        let class = world.fleets[fleet].capital_ships[index].class;
        let ticks = crate::delivery::transit_days(
            world,
            BuildableKind::CapitalShip(class),
            done.yard,
            location,
        );
        let ship = &mut world.fleets[fleet].capital_ships[index];
        ship.completed = true;
        ship.transit = (ticks > 0).then_some(crate::world::ShipTransit {
            from: done.yard,
            to: location,
            arrival_tick: done.tick + u64::from(ticks),
        });
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
    /// The ordered capital ships among them, for [`finish_ships`].
    pub finished_ships: Vec<FinishedShip>,
}

/// A completed item leaving its building system for its destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Departure {
    /// The building system.
    pub origin: SystemKey,
    /// Where the item goes.
    pub destination: Destination,
    /// The game-day on which the item completed.
    pub tick: u64,
    /// What was built.
    pub kind: BuildableKind,
    /// The ship it rides ([`QueueItem::carrier`]).
    pub carrier: u32,
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
        let mut finished_ships = Vec::new();

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
                if item.ship != 0 {
                    finished_ships.push(FinishedShip {
                        tag: item.ship,
                        yard: system_key,
                        tick: final_tick,
                    });
                }
                match item
                    .destination
                    .filter(|&destination| destination != Destination::System(system_key))
                {
                    Some(destination) => departures.push(Departure {
                        origin: system_key,
                        destination,
                        tick: final_tick,
                        kind: item.kind,
                        carrier: item.carrier,
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
            finished_ships,
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

    /// A world of `count` systems and a capital ship class of a side, named
    /// "Carrack", with 100 hull. port: test fixture values, not rules.
    fn yard_world(
        count: usize,
        is_alliance: bool,
    ) -> (crate::world::GameWorld, Vec<SystemKey>, CapitalShipKey) {
        let mut world = crate::world::GameWorld::default();
        let systems = (0..count)
            .map(|_| world.systems.insert(crate::world::System::default()))
            .collect();
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                name: "Carrack".into(),
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..crate::world::CapitalShipClass::default()
            });
        (world, systems, class)
    }

    /// The ships of `fleet` as (tag, completed, usable).
    fn ships(world: &crate::world::GameWorld, fleet: FleetKey) -> Vec<(u32, bool, bool)> {
        world.fleets[fleet]
            .capital_ships
            .iter()
            .map(|ship| (ship.tag, ship.completed, ship.usable()))
            .collect()
    }

    // FUN_0052a140 -> FUN_004f7d50(.., created 1, completed 0) creates the
    // ship at the order (FUN_004f7d50.c:31-38) in its destination system's
    // bit-1 spare fleet (FUN_00552300.c:127-131 -> FUN_005097d0), never an
    // existing fleet. The route-1 Empire game's day-115 save, the order day:
    // the Death Star +0x50 0x42 in its new fleet +0x50 0x10046.
    #[test]
    fn an_ordered_capital_ship_is_an_unfinished_member_of_a_new_fleet_at_its_destination_from_the_order_day_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(2, is_alliance);
            let (yard, target) = (systems[0], systems[1]);
            let standing = world.insert_fleet(crate::world::Fleet {
                location: target,
                is_alliance,
                capital_ships: vec![crate::world::ShipInstance::new(class, 100, is_alliance)],
                ..crate::world::Fleet::default()
            });
            world.systems[target].fleets.push(standing);
            let mut state = ManufacturingState::new();
            state.build_units(
                yard,
                vec![QueueItem::new(BuildableKind::CapitalShip(class), 5)
                    .delivered_to(Destination::System(target))],
            );

            seat_ordered_ships(&mut world, &mut state);

            assert_eq!(
                world.fleets[standing].capital_ships.len(),
                1,
                "{is_alliance}"
            );
            let fleets = world.systems[target].fleets.clone();
            assert_eq!(fleets.len(), 2, "{is_alliance}");
            let fleet = fleets[1];
            assert_eq!(world.fleets[fleet].is_alliance, is_alliance);
            assert_eq!(world.fleets[fleet].location, target);
            let tag = state.queue(yard, ProductionArea::Shipyard).unwrap().items()[0].ship;
            assert_ne!(tag, 0, "{is_alliance}");
            assert_eq!(ships(&world, fleet), [(tag, false, false)], "{is_alliance}");
            assert!(world.systems[yard].fleets.is_empty(), "{is_alliance}");
        }
    }

    // FUN_005385f0.c:200-219 resolves every unit of one Build in one pass
    // through FUN_005097d0, a pure query, and FUN_0052a140 creates each in
    // its entry's container: one fleet. The route-1 Empire game played on:
    // one Build of two Carracks at Coruscant, both 0x42 in one new fleet.
    #[test]
    fn the_units_of_one_build_share_one_new_fleet_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            let mut state = ManufacturingState::new();
            let unit = QueueItem::new(BuildableKind::CapitalShip(class), 5);
            state.build(yard, &unit, 2);

            seat_ordered_ships(&mut world, &mut state);
            seat_ordered_ships(&mut world, &mut state);

            assert_eq!(world.systems[yard].fleets.len(), 1, "{is_alliance}");
            let fleet = world.systems[yard].fleets[0];
            let tags: Vec<u32> = state
                .queue(yard, ProductionArea::Shipyard)
                .unwrap()
                .items()
                .iter()
                .map(|item| item.ship)
                .collect();
            assert_eq!(
                ships(&world, fleet),
                [(tags[0], false, false), (tags[1], false, false)],
                "{is_alliance}"
            );
            assert_ne!(tags[0], tags[1], "{is_alliance}");
        }
    }

    // FUN_005140c0.c:127 -> FUN_00517760 numbers an existing object on its
    // event 0x300, and the ordered ship exists from its order. The route-1
    // Empire game's day-115 save, the order day: "Death Star 1" in
    // "Fleet 6".
    #[test]
    fn an_ordered_ship_and_its_fleet_are_numbered_on_the_order_day_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            let mut state = ManufacturingState::new();
            state.build(
                yard,
                &QueueItem::new(BuildableKind::CapitalShip(class), 5),
                1,
            );

            seat_ordered_ships(&mut world, &mut state);

            let fleet = world.systems[yard].fleets[0];
            assert_eq!(world.fleet_name(fleet), Some("Fleet 1"), "{is_alliance}");
            assert_eq!(
                world.fleets[fleet].capital_ships[0].name.as_deref(),
                Some("Carrack 1"),
                "{is_alliance}"
            );
        }
    }

    // FUN_0050bc60.c:41-51 drops bit 1 only from a destroyed fleet, so the
    // side's spare at a system goes on taking every ship ordered there
    // (FUN_005097d0); the other side has its own. The route-1 Empire game
    // played on: a ship moved to Averam joined the fleet an earlier one had
    // made there.
    #[test]
    fn a_later_build_bound_for_the_same_system_joins_its_sides_spare_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(3, is_alliance);
            let (target, first, second) = (systems[0], systems[1], systems[2]);
            let enemy = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    is_alliance: !is_alliance,
                    is_empire: is_alliance,
                    hull: 100,
                    ..crate::world::CapitalShipClass::default()
                });
            let mut state = ManufacturingState::new();
            for (yard, class) in [(first, class), (second, class), (second, enemy)] {
                state.enqueue(
                    yard,
                    QueueItem::new(BuildableKind::CapitalShip(class), 5)
                        .delivered_to(Destination::System(target)),
                );
                seat_ordered_ships(&mut world, &mut state);
            }

            let sides: Vec<(bool, usize)> = world.systems[target]
                .fleets
                .iter()
                .map(|&fleet| {
                    let value = &world.fleets[fleet];
                    (value.is_alliance, value.capital_ships.len())
                })
                .collect();
            assert_eq!(
                sides,
                [(is_alliance, 2), (!is_alliance, 1)],
                "{is_alliance}"
            );
        }
    }

    // FUN_00529ab0 runs FUN_0052a2c0 (+0xac(0x16) on every listed product)
    // before FUN_0052a140 creates the new units, and the new unit takes the
    // replaced one's fleet, which does not disband. Wine, the ds-b save
    // (route-1 Empire game, day 151): a Build at Coruscant replaced the
    // unfinished Death Star `0x18000df5`, the only ship of Fleet 6
    // `0x08000aea`, with `0x18000e4f` in that same fleet.
    #[test]
    fn a_new_build_destroys_what_its_area_was_building_and_seats_the_new_ship_in_the_same_fleet_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            let mut state = ManufacturingState::new();
            let unit = QueueItem::new(BuildableKind::CapitalShip(class), 5);
            state.build(yard, &unit, 1);
            seat_ordered_ships(&mut world, &mut state);
            let old_fleet = world.systems[yard].fleets[0];
            let old_tag = world.fleets[old_fleet].capital_ships[0].tag;

            state.build(yard, &unit, 1);
            seat_ordered_ships(&mut world, &mut state);

            assert_eq!(world.systems[yard].fleets, [old_fleet], "{is_alliance}");
            let fleet = world.systems[yard].fleets[0];
            let tag = world.fleets[fleet].capital_ships[0].tag;
            assert_ne!(tag, old_tag, "{is_alliance}");
            assert_eq!(ships(&world, fleet), [(tag, false, false)], "{is_alliance}");
        }
    }

    // Stop is manager slot +0x1f8, FUN_00529ba0 -> FUN_0052a2c0; the fleet
    // it empties disbands (FUN_004fe630). The route-1 Empire game played
    // on: after Stop the Death Star's state byte is 0x16, its fleet's 0x14.
    #[test]
    fn stop_destroys_the_areas_unfinished_ships_and_disbands_their_emptied_fleet_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            let mut state = ManufacturingState::new();
            state.build(
                yard,
                &QueueItem::new(BuildableKind::CapitalShip(class), 5),
                2,
            );
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[yard].fleets[0];

            state.stop(yard, ProductionArea::Shipyard);
            seat_ordered_ships(&mut world, &mut state);

            assert!(!world.fleets.contains_key(fleet), "{is_alliance}");
            assert!(world.systems[yard].fleets.is_empty(), "{is_alliance}");
        }
    }

    // FUN_0052a710 drops a listed key FUN_00528890 no longer accepts (a
    // product destroyed elsewhere), so its entry leaves the queue.
    #[test]
    fn a_queue_entry_whose_ship_is_gone_leaves_its_queue_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            let mut state = ManufacturingState::new();
            state.build(
                yard,
                &QueueItem::new(BuildableKind::CapitalShip(class), 5),
                2,
            );
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[yard].fleets[0];
            let kept = world.fleets[fleet].capital_ships[1].tag;
            world.fleets[fleet].capital_ships[0].alive = false;

            seat_ordered_ships(&mut world, &mut state);

            let tags: Vec<u32> = state
                .queue(yard, ProductionArea::Shipyard)
                .unwrap()
                .items()
                .iter()
                .map(|item| item.ship)
                .collect();
            assert_eq!(tags, [kept], "{is_alliance}");
        }
    }

    // FUN_0052b960 -> FUN_0052bee0 sets deployed, then completed, on the
    // object the order made: no second ship, the same fleet, tag and name.
    // The route-1 Empire game: at completion the ship and its fleet read
    // 0x30147 under the same keys and names ("Death Star 1", "Fleet 6").
    #[test]
    fn completion_marks_the_ordered_ship_completed_in_place_keeping_its_fleet_tag_and_name_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            let mut state = ManufacturingState::new();
            state.build(
                yard,
                &QueueItem::new(BuildableKind::CapitalShip(class), 1),
                1,
            );
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[yard].fleets[0];
            let tag = world.fleets[fleet].capital_ships[0].tag;
            state.add_progress(yard, ProductionArea::Shipyard);

            let advance = ManufacturingSystem::advance_tracked(
                &mut state,
                &[TickEvent { tick: 7 }],
                &HashSet::new(),
            );
            finish_ships(
                &mut world,
                &crate::movement::MovementState::new(),
                &advance.finished_ships,
            );
            seat_ordered_ships(&mut world, &mut state);

            assert_eq!(
                advance.finished_ships,
                [FinishedShip { tag, yard, tick: 7 }],
                "{is_alliance}"
            );
            assert_eq!(world.systems[yard].fleets, [fleet], "{is_alliance}");
            assert_eq!(ships(&world, fleet), [(tag, true, true)], "{is_alliance}");
            assert_eq!(
                world.fleets[fleet].capital_ships[0].name.as_deref(),
                Some("Carrack 1"),
                "{is_alliance}"
            );
        }
    }

    // A fleet in hyperspace is its destination's child from its order
    // (FUN_00556390), so the deploy event's leg (FUN_005150c0.c:90-102 ->
    // FUN_00556430) for a ship finished in it ends at that destination.
    #[test]
    fn a_ship_finished_for_a_fleet_in_hyperspace_flies_to_its_destination_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(3, is_alliance);
            let (yard, target, far) = (systems[0], systems[1], systems[2]);
            world.systems[target].x = 100;
            world.systems[far].x = 200;
            world.capital_ship_classes[class].hyperdrive = 100;
            let mut state = ManufacturingState::new();
            state.build_units(
                yard,
                vec![
                    QueueItem::new(BuildableKind::CapitalShip(class), 5)
                        .delivered_to(Destination::System(target));
                    2
                ],
            );
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[target].fleets[0];
            let tag = world.fleets[fleet].capital_ships[1].tag;
            let mut movement = crate::movement::MovementState::new();
            assert!(crate::movement::begin_fleet_transit(
                &mut movement,
                &mut world,
                fleet,
                far,
                5
            ));

            finish_ships(
                &mut world,
                &movement,
                &[FinishedShip { tag, yard, tick: 7 }],
            );

            let transit = world.fleets[fleet].capital_ships[1].transit;
            assert_eq!(
                transit.map(|leg| (leg.from, leg.to)),
                Some((yard, far)),
                "{is_alliance}"
            );
        }
    }

    // FUN_0052bee0 sets deployed and completed on the finished product
    // alone, and en route active when its place is not the yard's; deploy
    // event 0x308 then starts its own leg from the yard (+0x38) to the
    // fleet it sits in (FUN_005150c0.c:90-102 -> FUN_00556430), on the
    // transit time of FUN_0055d8c0.
    #[test]
    fn a_ship_finished_for_a_fleet_elsewhere_starts_its_own_leg_from_the_yard_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, systems, class) = yard_world(2, is_alliance);
            let (yard, target) = (systems[0], systems[1]);
            world.systems[target].x = 100;
            world.capital_ship_classes[class].hyperdrive = 100;
            let mut state = ManufacturingState::new();
            state.build_units(
                yard,
                vec![
                    QueueItem::new(BuildableKind::CapitalShip(class), 5)
                        .delivered_to(Destination::System(target));
                    2
                ],
            );
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[target].fleets[0];
            let second = world.fleets[fleet].capital_ships[1].tag;

            finish_ships(
                &mut world,
                &crate::movement::MovementState::new(),
                &[FinishedShip {
                    tag: second,
                    yard,
                    tick: 7,
                }],
            );

            let days = crate::delivery::transit_days(
                &world,
                BuildableKind::CapitalShip(class),
                yard,
                target,
            );
            assert!(days > 0, "{is_alliance}");
            let ships = &world.fleets[fleet].capital_ships;
            assert!(!ships[0].completed, "{is_alliance}");
            assert!(ships[1].completed, "{is_alliance}");
            assert_eq!(
                ships[1].transit,
                Some(crate::world::ShipTransit {
                    from: yard,
                    to: target,
                    arrival_tick: 7 + u64::from(days),
                }),
                "{is_alliance}"
            );
        }
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
        state.enqueue(
            systems[0],
            cap_ship_item(1).delivered_to(Destination::System(systems[1])),
        );
        state.enqueue(
            systems[2],
            fighter_item(1).delivered_to(Destination::System(systems[2])),
        );
        work(&mut state, systems[0], ProductionArea::Shipyard, 1);
        work(&mut state, systems[2], ProductionArea::Shipyard, 1);

        let advance =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(9..=9), &HashSet::new());

        assert_eq!(
            advance.departures,
            vec![Departure {
                origin: systems[0],
                destination: Destination::System(systems[1]),
                tick: 9,
                kind: cap_ship_item(1).kind,
                carrier: 0,
            }]
        );
        assert_eq!(advance.completions.len(), 1);
        assert_eq!(advance.completions[0].system, systems[2]);
    }

    // A fleet-bound squadron rides the ship its Build chose (FUN_005385f0
    // -> FUN_00552300) from its order to its arrival, so its departure
    // names that ship. hyp: a Destination order that rewrites it frees the
    // ship (`set_destination`; the order's handler is untraced).
    #[test]
    fn a_fleet_bound_squadron_keeps_its_ship_until_its_destination_changes() {
        let systems = mock_system_keys(2);
        let fleet = {
            let mut keys = slotmap::SlotMap::<FleetKey, ()>::with_key();
            keys.insert(())
        };
        let mut state = ManufacturingState::new();
        let mut item = fighter_item(1).delivered_to(Destination::Fleet(fleet));
        item.carrier = 7;
        state.enqueue(systems[0], item.clone());
        state.enqueue(systems[0], item);
        state.set_destination(
            systems[0],
            ProductionArea::Shipyard,
            Destination::Fleet(fleet),
        );
        let carriers: Vec<u32> = state.items().map(|(_, _, item)| item.carrier).collect();
        assert_eq!(carriers, [7, 7]);
        work(&mut state, systems[0], ProductionArea::Shipyard, 1);

        let advance =
            ManufacturingSystem::advance_tracked(&mut state, &ticks(9..=9), &HashSet::new());

        assert_eq!(advance.departures.len(), 1);
        assert_eq!(advance.departures[0].carrier, 7);
        state.set_destination(
            systems[0],
            ProductionArea::Shipyard,
            Destination::System(systems[1]),
        );
        let carriers: Vec<u32> = state.items().map(|(_, _, item)| item.carrier).collect();
        assert_eq!(carriers, [0]);
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
    fn taking_a_systems_products_empties_every_area_and_spares_other_systems() {
        let keys = mock_system_keys(2);
        let mut state = ManufacturingState::new();
        state.enqueue(keys[0], cap_ship_item(2));
        state.enqueue(keys[0], troop_item(2));
        state.enqueue(keys[1], troop_item(2));

        let taken = state.take_queued(keys[0]);

        assert_eq!(taken, vec![cap_ship_item(2), troop_item(2)]);
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
        let (to_home, to_away) = (Destination::System(home), Destination::System(away));
        let mut state = ManufacturingState::new();
        state.enqueue(home, cap_ship_item(10));
        state.enqueue(home, troop_item(5));

        state.set_destination(home, ProductionArea::Shipyard, to_away);
        let destinations = |state: &ManufacturingState| {
            ProductionArea::ALL
                .iter()
                .filter_map(|&area| state.queue(home, area))
                .flat_map(|queue| queue.items().iter().map(|item| item.destination))
                .collect::<Vec<_>>()
        };
        assert_eq!(destinations(&state), [Some(to_away), None]);
        assert_eq!(
            state.destination(home, ProductionArea::Shipyard),
            Some(to_away)
        );
        assert_eq!(
            state.destination(home, ProductionArea::TrainingFacility),
            None
        );

        // A later product of that area takes it; another area's does not.
        state.enqueue(home, cap_ship_item(3));
        state.enqueue(home, troop_item(5));
        assert_eq!(
            destinations(&state),
            [Some(to_away), Some(to_away), None, None]
        );

        // Back at home clears it.
        state.set_destination(home, ProductionArea::Shipyard, to_home);
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

    /// `system` passes to `side` (`FUN_0050a300` → `FUN_00510d70`): the new
    /// control, then the owner hook (`FUN_00510820`).
    fn change_holder(
        world: &mut crate::world::GameWorld,
        system: SystemKey,
        control: crate::world::ControlKind,
    ) {
        let old = world.systems[system].control;
        world.systems[system].control = control;
        crate::support_shift::control_changed(world, system, old, (false, false), 1);
    }

    /// Both sides' controls: own first.
    fn controls(is_alliance: bool) -> [crate::world::ControlKind; 2] {
        use crate::dat::Faction;
        use crate::world::ControlKind;
        let (own, enemy) = if is_alliance {
            (Faction::Alliance, Faction::Empire)
        } else {
            (Faction::Empire, Faction::Alliance)
        };
        [ControlKind::Controlled(own), ControlKind::Controlled(enemy)]
    }

    // The holder change hands each production manager to the captor
    // (FUN_004f8680.c:28-31 -> FUN_004f6f40), and the manager's side hook
    // FUN_0052b790.c:7-14 runs FUN_0052a2c0: +0xac(0x16) on every product it
    // lists, whatever its kind or container (FUN_0052a2c0.c:20-36). The
    // Empire game of the Duros line: on day 306 Duros's facility manager
    // 0xa0000523 lists an unfinished Orbital Shipyard (0x28000e75, +0x50
    // 0x42); on day 430 Duros and that manager are the Alliance's, the
    // manager lists only the Alliance's own new Build (0x29001059), and the
    // shipyard has no record in any graph, though Duros's construction yard
    // (0x2a00077e) stands, now the Alliance's.
    #[test]
    fn a_captured_systems_managers_lose_every_product_wherever_it_sits_for_either_side() {
        for is_alliance in [true, false] {
            let [own, enemy] = controls(is_alliance);
            let (mut world, systems, class) = yard_world(2, is_alliance);
            let (yard, elsewhere) = (systems[0], systems[1]);
            for &system in &systems {
                world.systems[system].control = own;
            }
            let facility = BuildableKind::ManufacturingFacility(FacilityBuild {
                class: DatId::new(0x2800_0001),
                is_alliance,
            });
            let mut state = ManufacturingState::new();
            state.build_units(
                yard,
                vec![QueueItem::new(BuildableKind::CapitalShip(class), 5)
                    .delivered_to(Destination::System(elsewhere))],
            );
            state.enqueue(yard, QueueItem::new(facility, 5));
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[elsewhere].fleets[0];

            change_holder(&mut world, yard, enemy);
            let destroyed = serve_manager_sides(&mut world, &mut state);

            assert_eq!(
                destroyed,
                [BuildableKind::CapitalShip(class), facility],
                "{is_alliance}"
            );
            assert_eq!(state.queued_at(yard), 0, "{is_alliance}");
            assert!(!world.fleets.contains_key(fleet), "{is_alliance}");
            assert!(world.systems[elsewhere].fleets.is_empty(), "{is_alliance}");
            assert!(world.manager_side_changes.is_empty(), "{is_alliance}");
        }
    }

    // A capital ship product sits in a fleet, whose slot +0xb0 is
    // FUN_0051ebb0 (returns 1, changes nothing), so the holder change's
    // child walk (FUN_004fae40) leaves it, and its own manager keeps its
    // side: it stays listed, unfinished, of the side that ordered it.
    #[test]
    fn a_ship_another_systems_yard_builds_for_a_captured_system_stays_unfinished_for_either_side()
    {
        for is_alliance in [true, false] {
            let [own, enemy] = controls(is_alliance);
            let (mut world, systems, class) = yard_world(2, is_alliance);
            let (yard, captured) = (systems[0], systems[1]);
            for &system in &systems {
                world.systems[system].control = own;
            }
            let mut state = ManufacturingState::new();
            state.build_units(
                yard,
                vec![QueueItem::new(BuildableKind::CapitalShip(class), 5)
                    .delivered_to(Destination::System(captured))],
            );
            seat_ordered_ships(&mut world, &mut state);
            let fleet = world.systems[captured].fleets[0];
            let tag = state.queue(yard, ProductionArea::Shipyard).unwrap().items()[0].ship;

            change_holder(&mut world, captured, enemy);
            let destroyed = serve_manager_sides(&mut world, &mut state);

            assert!(destroyed.is_empty(), "{is_alliance}");
            assert_eq!(state.queued_at(yard), 1, "{is_alliance}");
            assert_eq!(world.fleets[fleet].is_alliance, is_alliance);
            assert_eq!(ships(&world, fleet), [(tag, false, false)], "{is_alliance}");
        }
    }

    // FUN_0052a2c0 destroys only the listed products (+0xac(0x16)): a fleet
    // keeps its other ships as they are, names included, and disbands only
    // when emptied (FUN_004fe630.c:21-28). Names come on the order day
    // (FUN_00517760), never at a destruction.
    #[test]
    fn a_holder_change_kills_only_its_yards_ships_and_renames_none_for_either_side() {
        for is_alliance in [true, false] {
            let [own, enemy] = controls(is_alliance);
            let (mut world, systems, class) = yard_world(3, is_alliance);
            let (captured, other, at) = (systems[0], systems[1], systems[2]);
            for &system in &systems {
                world.systems[system].control = own;
            }
            let fleet = world.insert_fleet(crate::world::Fleet {
                location: at,
                is_alliance,
                ..crate::world::Fleet::default()
            });
            world.systems[at].fleets.push(fleet);
            let mut state = ManufacturingState::new();
            for yard in [captured, other] {
                state.build_units(
                    yard,
                    vec![QueueItem::new(BuildableKind::CapitalShip(class), 5)
                        .delivered_to(Destination::Fleet(fleet))],
                );
            }
            seat_ordered_ships(&mut world, &mut state);
            let kept = state.queue(other, ProductionArea::Shipyard).unwrap().items()[0].ship;
            // A ship still bearing its class's name, which a numbering pass
            // would rename.
            world.fleets[fleet]
                .capital_ships
                .push(crate::world::ShipInstance::new(class, 100, is_alliance));
            let names = |world: &crate::world::GameWorld| {
                world.fleets[fleet]
                    .capital_ships
                    .iter()
                    .map(|ship| (ship.tag, ship.completed, ship.name.clone()))
                    .collect::<Vec<_>>()
            };
            let mut expected = names(&world);
            expected.retain(|&(tag, _, _)| tag == kept || tag == 0);

            change_holder(&mut world, captured, enemy);
            serve_manager_sides(&mut world, &mut state);

            assert_eq!(expected.len(), 2, "{is_alliance}");
            assert_eq!(names(&world), expected, "{is_alliance}");
            assert_eq!(state.queued_at(other), 1, "{is_alliance}");
        }
    }

    // FUN_004f6f40.c:22 calls the side hook only when the side bits change:
    // an uprising keeps the holder (FUN_0050b610 writes the same side), so
    // the managers keep their products.
    #[test]
    fn a_control_change_that_keeps_the_holder_keeps_the_products_for_either_side() {
        for is_alliance in [true, false] {
            let [own, _] = controls(is_alliance);
            let side = own.faction().unwrap();
            let (mut world, systems, class) = yard_world(1, is_alliance);
            let yard = systems[0];
            world.systems[yard].control = own;
            let mut state = ManufacturingState::new();
            state.build_units(
                yard,
                vec![QueueItem::new(BuildableKind::CapitalShip(class), 5)],
            );
            seat_ordered_ships(&mut world, &mut state);

            change_holder(&mut world, yard, crate::world::ControlKind::Uprising(side));
            let destroyed = serve_manager_sides(&mut world, &mut state);

            assert!(destroyed.is_empty(), "{is_alliance}");
            assert_eq!(state.queued_at(yard), 1, "{is_alliance}");
            assert_eq!(world.systems[yard].fleets.len(), 1, "{is_alliance}");
        }
    }
}
