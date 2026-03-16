//! Manufacturing system: production queues and per-tick advancement.
//!
//! Each star system can have a queue of items under construction. Every game-day
//! (tick) the active item's remaining time decrements. When it reaches zero the
//! item completes and the next item in the queue becomes active.
//!
//! # Architecture
//!
//! Production state is kept in a `ManufacturingState` map that lives alongside
//! `GameWorld` rather than inside it. `GameWorld` stores entity class templates
//! (CapitalShipClass, FighterClass, etc.); `ManufacturingState` stores the
//! per-system work-in-progress queues.
//!
//! Each tick, the caller feeds the `Vec<TickEvent>` from `GameClock::advance`
//! directly to `ManufacturingSystem::advance`, which returns a list of
//! `CompletionEvent`s for the caller to act on (spawn units, add to fleet, etc.).
//!
//! # Usage
//!
//! ```
//! use rebellion_core::ids::SystemKey;
//! use rebellion_core::manufacturing::{
//!     BuildableKind, ManufacturingState, ManufacturingSystem, QueueItem,
//! };
//! use rebellion_core::tick::{GameClock, GameSpeed};
//!
//! let mut clock = GameClock::new();
//! clock.set_speed(GameSpeed::Normal);
//!
//! let mut state = ManufacturingState::new();
//! // ... populate queues ...
//!
//! let tick_events = clock.advance(1.0 / 60.0);
//! let completions = ManufacturingSystem::advance(&mut state, &tick_events);
//! // Handle completions: add ships to fleets, place facilities, etc.
//! ```

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use crate::ids::{
    CapitalShipKey, DefenseFacilityKey, FighterKey, ManufacturingFacilityKey, ProductionFacilityKey,
    SystemKey, TroopKey,
};
use crate::tick::TickEvent;

// ---------------------------------------------------------------------------
// BuildableKind
// ---------------------------------------------------------------------------

/// The class template being produced.
///
/// Each variant wraps the slotmap key of the class definition stored in
/// `GameWorld`. Instance creation happens when the queue item completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildableKind {
    /// A capital ship class (Star Destroyer, Mon Cal Cruiser, etc.)
    CapitalShip(CapitalShipKey),
    /// A fighter squadron class (X-Wing, TIE Fighter, etc.)
    Fighter(FighterKey),
    /// A ground troop unit.
    Troop(TroopKey),
    /// A planetary defense installation.
    DefenseFacility(DefenseFacilityKey),
    /// A shipyard, training center, or other manufacturing facility.
    ManufacturingFacility(ManufacturingFacilityKey),
    /// A mine, refinery, or resource production facility.
    ProductionFacility(ProductionFacilityKey),
}

// ---------------------------------------------------------------------------
// QueueItem
// ---------------------------------------------------------------------------

/// One item under construction in a system's production queue.
///
/// The first item in the queue is actively being built; the rest are waiting
/// their turn. Only the active item's `ticks_remaining` decrements each tick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueItem {
    /// What is being built.
    pub kind: BuildableKind,
    /// Game-days remaining until construction completes.
    ///
    /// Derived from `refined_material_cost` and the system's facility
    /// `processing_rate` at queue time. Stored here so the queue is
    /// self-contained and survives facility changes mid-build.
    pub ticks_remaining: u32,
    /// Original construction cost in refined materials (for UI display).
    pub total_cost: u32,
}

impl QueueItem {
    /// Create a new queue item with the given cost and build duration.
    pub fn new(kind: BuildableKind, ticks_remaining: u32, total_cost: u32) -> Self {
        QueueItem {
            kind,
            ticks_remaining,
            total_cost,
        }
    }

    /// How many ticks have been spent so far (for progress bar rendering).
    pub fn ticks_spent(&self) -> u32 {
        self.total_cost.saturating_sub(self.ticks_remaining)
    }

    /// Progress fraction in [0.0, 1.0].
    pub fn progress_fraction(&self) -> f32 {
        if self.total_cost == 0 {
            return 1.0;
        }
        1.0 - (self.ticks_remaining as f32 / self.total_cost as f32)
    }
}

// ---------------------------------------------------------------------------
// ProductionQueue
// ---------------------------------------------------------------------------

/// The ordered production queue for one star system.
///
/// Items are processed front-to-back. The front item is "active" — its
/// `ticks_remaining` decrements each tick. Items at index > 0 are queued.
///
/// Capacity is uncapped; the original game had a soft limit of ~5 items
/// per system enforced by the UI, not the engine.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionQueue {
    items: VecDeque<QueueItem>,
}

impl ProductionQueue {
    pub fn new() -> Self {
        ProductionQueue {
            items: VecDeque::new(),
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
    pub fn active(&self) -> Option<&QueueItem> {
        self.items.front()
    }

    /// All items in queue order (index 0 = active).
    pub fn items(&self) -> &VecDeque<QueueItem> {
        &self.items
    }

    /// Total items, including the active one.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Advance the queue by `ticks` game-days.
    ///
    /// Returns a list of `BuildableKind` items that completed during this
    /// advance. Multiple completions are possible if `ticks` is large and
    /// several items have small remaining costs.
    fn advance_ticks(&mut self, ticks: u32) -> Vec<BuildableKind> {
        let mut completed = Vec::new();
        let mut remaining_ticks = ticks;

        loop {
            let front = match self.items.front_mut() {
                Some(item) => item,
                None => break,
            };

            if remaining_ticks >= front.ticks_remaining {
                // This item completes; consume its cost and continue with leftover ticks.
                remaining_ticks -= front.ticks_remaining;
                let finished = self.items.pop_front().unwrap();
                completed.push(finished.kind);
            } else {
                // Partial progress — item survives.
                front.ticks_remaining -= remaining_ticks;
                break;
            }
        }

        completed
    }
}

// ---------------------------------------------------------------------------
// ManufacturingState
// ---------------------------------------------------------------------------

/// Per-system production queues for the entire galaxy.
///
/// Systems with no active queue are not stored (lazy entry on first enqueue).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManufacturingState {
    queues: HashMap<SystemKey, ProductionQueue>,
}

impl ManufacturingState {
    pub fn new() -> Self {
        ManufacturingState {
            queues: HashMap::new(),
        }
    }

    /// Get the queue for a system, creating it if it doesn't exist.
    pub fn queue_mut(&mut self, system: SystemKey) -> &mut ProductionQueue {
        self.queues.entry(system).or_default()
    }

    /// Get the queue for a system (read-only). Returns `None` if empty.
    pub fn queue(&self, system: SystemKey) -> Option<&ProductionQueue> {
        self.queues.get(&system)
    }

    /// Enqueue an item at a system's production queue.
    pub fn enqueue(&mut self, system: SystemKey, item: QueueItem) {
        self.queue_mut(system).enqueue(item);
    }

    /// All system queues (including empty ones that were created lazily).
    pub fn queues(&self) -> &HashMap<SystemKey, ProductionQueue> {
        &self.queues
    }
}

// ---------------------------------------------------------------------------
// CompletionEvent
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

// ---------------------------------------------------------------------------
// ManufacturingSystem
// ---------------------------------------------------------------------------

/// Stateless system that advances production queues per tick.
///
/// Call `advance` once per frame, passing the `Vec<TickEvent>` from
/// `GameClock::advance`. Returns all items that completed during those ticks.
pub struct ManufacturingSystem;

impl ManufacturingSystem {
    /// Advance all production queues by the number of ticks in `tick_events`.
    ///
    /// Each `TickEvent` represents one game-day. If multiple ticks fired in
    /// one frame (e.g., at Faster speed) they are batched into a single pass
    /// per queue.
    ///
    /// Systems in `blocked_systems` (e.g., blockaded systems) are skipped —
    /// their queues do not advance while blocked.
    ///
    /// Returns a `Vec<CompletionEvent>` — one entry per completed item,
    /// across all systems. Empty if no items completed this frame.
    pub fn advance(
        state: &mut ManufacturingState,
        tick_events: &[TickEvent],
    ) -> Vec<CompletionEvent> {
        Self::advance_with_blockade(state, tick_events, &HashSet::new())
    }

    /// Like `advance`, but skips systems in `blocked_systems`.
    ///
    /// Called by the main loop with `BlockadeState::blockaded_systems()` to
    /// halt manufacturing at blockaded systems.
    pub fn advance_with_blockade(
        state: &mut ManufacturingState,
        tick_events: &[TickEvent],
        blocked_systems: &HashSet<SystemKey>,
    ) -> Vec<CompletionEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        // Batch all ticks that fired this frame into a single advance.
        let tick_count = tick_events.len() as u32;
        // The last tick number in this batch (used as the completion timestamp).
        let final_tick = tick_events.last().unwrap().tick;

        let mut completions = Vec::new();

        for (system_key, queue) in state.queues.iter_mut() {
            // Skip blockaded systems — manufacturing halted
            if blocked_systems.contains(system_key) {
                continue;
            }
            let finished = queue.advance_ticks(tick_count);
            for kind in finished {
                completions.push(CompletionEvent {
                    system: *system_key,
                    tick: final_tick,
                    kind,
                });
            }
        }

        completions
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::SystemKey;
    use crate::tick::{GameClock, GameSpeed};

    // Helper: fabricate distinct SystemKeys from one shared slotmap.
    fn mock_system_keys(n: usize) -> Vec<SystemKey> {
        let mut sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        (0..n).map(|_| sm.insert(())).collect()
    }

    fn mock_system_key() -> SystemKey {
        mock_system_keys(1).into_iter().next().unwrap()
    }

    fn cap_ship_item(ticks: u32) -> QueueItem {
        // We need a CapitalShipKey to build the kind. Use a slotmap.
        let mut sm: slotmap::SlotMap<CapitalShipKey, ()> = slotmap::SlotMap::with_key();
        let key = sm.insert(());
        QueueItem::new(BuildableKind::CapitalShip(key), ticks, ticks)
    }

    fn fighter_item(ticks: u32) -> QueueItem {
        let mut sm: slotmap::SlotMap<FighterKey, ()> = slotmap::SlotMap::with_key();
        let key = sm.insert(());
        QueueItem::new(BuildableKind::Fighter(key), ticks, ticks)
    }

    // --- ProductionQueue tests ---

    #[test]
    fn enqueue_and_active() {
        let mut q = ProductionQueue::new();
        assert!(q.active().is_none());

        q.enqueue(cap_ship_item(10));
        assert!(q.active().is_some());
        assert_eq!(q.active().unwrap().ticks_remaining, 10);
    }

    #[test]
    fn advance_partial_progress() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        let completed = q.advance_ticks(4);
        assert!(completed.is_empty());
        assert_eq!(q.active().unwrap().ticks_remaining, 6);
    }

    #[test]
    fn advance_exact_completes_item() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(5));
        let completed = q.advance_ticks(5);
        assert_eq!(completed.len(), 1);
        assert!(q.is_empty());
    }

    #[test]
    fn advance_overflow_completes_and_starts_next() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(3));
        q.enqueue(fighter_item(10));

        // 5 ticks: completes first item (3 ticks), 2 ticks into next
        let completed = q.advance_ticks(5);
        assert_eq!(completed.len(), 1);
        assert_eq!(q.active().unwrap().ticks_remaining, 8);
    }

    #[test]
    fn advance_completes_multiple_items() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(2));
        q.enqueue(cap_ship_item(3));
        q.enqueue(cap_ship_item(4));

        // 10 ticks — all three should complete (2+3+4 = 9 ticks, 1 leftover)
        let completed = q.advance_ticks(10);
        assert_eq!(completed.len(), 3);
        assert!(q.is_empty());
    }

    #[test]
    fn cancel_active_item() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        q.enqueue(fighter_item(5));

        q.cancel(0);
        assert_eq!(q.len(), 1);
        assert_eq!(q.active().unwrap().ticks_remaining, 5);
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
        assert_eq!(q.active().unwrap().ticks_remaining, 10);
    }

    #[test]
    fn progress_fraction() {
        let mut q = ProductionQueue::new();
        q.enqueue(cap_ship_item(10));
        q.advance_ticks(5);
        let frac = q.active().unwrap().progress_fraction();
        assert!((frac - 0.5).abs() < 0.001, "expected ~0.5, got {frac}");
    }

    // --- ManufacturingSystem integration tests ---

    #[test]
    fn system_advance_no_ticks_no_completions() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(5));

        let completions = ManufacturingSystem::advance(&mut state, &[]);
        assert!(completions.is_empty());
        assert_eq!(state.queue(system).unwrap().active().unwrap().ticks_remaining, 5);
    }

    #[test]
    fn system_advance_with_tick_events() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(3));

        // Simulate GameClock emitting 3 TickEvents
        let tick_events = vec![
            TickEvent { tick: 1 },
            TickEvent { tick: 2 },
            TickEvent { tick: 3 },
        ];

        let completions = ManufacturingSystem::advance(&mut state, &tick_events);
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].system, system);
        assert_eq!(completions[0].tick, 3); // last tick in the batch
        assert!(state.queue(system).unwrap().is_empty());
    }

    #[test]
    fn multiple_systems_advance_independently() {
        let keys = mock_system_keys(2);
        let (sys_a, sys_b) = (keys[0], keys[1]);
        let mut state = ManufacturingState::new();
        state.enqueue(sys_a, cap_ship_item(2));
        state.enqueue(sys_b, cap_ship_item(5));

        let tick_events = vec![TickEvent { tick: 1 }, TickEvent { tick: 2 }];
        let completions = ManufacturingSystem::advance(&mut state, &tick_events);

        // sys_a completes, sys_b still has 3 ticks left
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].system, sys_a);
        assert_eq!(
            state.queue(sys_b).unwrap().active().unwrap().ticks_remaining,
            3
        );
    }

    #[test]
    fn integration_clock_drives_manufacturing() {
        let system = mock_system_key();
        let mut state = ManufacturingState::new();
        state.enqueue(system, cap_ship_item(2));

        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast); // 2× speed

        // 1 real second at 2× = 2 ticks — should complete the item
        let tick_events = clock.advance(1.0);
        assert_eq!(tick_events.len(), 2);

        let completions = ManufacturingSystem::advance(&mut state, &tick_events);
        assert_eq!(completions.len(), 1);
    }
}
