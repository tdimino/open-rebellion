//! Uprising system: loyalty-driven control changes with probability tables.
//!
//! Systems with low loyalty can revolt against their controlling faction.
//! This module:
//!
//! 1. **Evaluates loyalty thresholds** each tick using UPRIS1TB (3 entries).
//! 2. **Fires UprisingIncident** (`0x152`) as a precursor warning when loyalty
//!    is trending low but hasn't triggered a full uprising yet.
//! 3. **Fires Uprising** (`0x14b`) + **ControlKindUprising** (`0x151`) when an
//!    uprising begins — flipping `controlling_faction` on the system.
//! 4. **Subdues uprisings** via the Subdue Uprising mission using UPRIS2TB
//!    (4 entries) for probability lookup.
//!
//! # Architecture
//!
//! ```text
//! UprisingSystem::advance(&mut UprisingState, &world, &[TickEvent], &[f64]) -> Vec<UprisingEvent>
//! ```
//! RNG rolls are caller-supplied (`&[f64]`) for determinism and testability.
//! The caller applies `UprisingEvent::UprisingBegan` by updating
//! `System::controlling_faction` in `GameWorld`.
//!
//! # Probability Tables
//!
//! - **UPRIS1TB** (3 entries): `uprising_start_prob(loyalty) -> u32 (0–100)`.
//!   Negative threshold = below-neutral loyalty. At very low loyalty the
//!   probability approaches 100.
//! - **UPRIS2TB** (4 entries): `uprising_subdue_prob(loyalty) -> u32 (0–100)`.
//!   Subduing is harder at low loyalty, easier if loyalty has recovered.
//!
//! Both tables use `MstbTable::lookup()` for linear interpolation.
//!
//! # Source
//!
//! Ghidra RE: `economy-systems.md §3`.
//! - `FUN_005121e0` (`SystemUprisingNotif`), event `0x14b` (331).
//! - `FUN_00511f40` (`SystemControlKindUprisingNotif`), event `0x151` (337).
//! - `FUN_00512580` (`SystemUprisingIncidentNotif`), event `0x152` (338).
//! - `FUN_00509c30`: loyalty write gate: `*(uint*)(this + 0x88) & 1` = is_populated.
//! - UPRIS1TB.DAT: 3 IntTableEntry records; UPRIS2TB.DAT: 4 records.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ids::SystemKey;
use crate::tick::TickEvent;
use crate::world::{GameWorld, MstbTable};

// ---------------------------------------------------------------------------
// Event IDs
// ---------------------------------------------------------------------------

/// `SystemUprisingIncidentNotif` — precursor warning.
pub const EV_UPRISING_INCIDENT: u32 = 0x152; // 338

/// `SystemUprisingNotif` — uprising begins.
pub const EV_UPRISING: u32 = 0x14b; // 331

/// `SystemControlKindUprisingNotif` — control kind flips.
pub const EV_CONTROL_KIND_UPRISING: u32 = 0x151; // 337

// ---------------------------------------------------------------------------
// UprisingEvent
// ---------------------------------------------------------------------------

/// Events emitted by `UprisingSystem::advance`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UprisingEvent {
    /// Loyalty is dangerously low — precursor warning before a full uprising.
    ///
    /// Corresponds to `UprisingIncident` (event `0x152`). The player has a
    /// window to respond before the uprising fires.
    UprisingIncident { system: SystemKey, tick: u64 },

    /// An uprising has begun. The system's controlling faction flips.
    ///
    /// Corresponds to `Uprising` (event `0x14b`) + `ControlKindUprising`
    /// (`0x151`). The caller must update `System::controlling_faction`.
    UprisingBegan { system: SystemKey, tick: u64 },

    /// An uprising has been subdued (Subdue Uprising mission succeeded).
    ///
    /// The system remains under the current faction's control.
    UprisingSubdued { system: SystemKey, tick: u64 },
}

// ---------------------------------------------------------------------------
// UprisingState
// ---------------------------------------------------------------------------

/// Per-system uprising tracking, maintained across ticks.
///
/// `active_uprisings` tracks which systems are currently in revolt.
/// `incident_fired` prevents duplicate incident warnings per-system per-tick.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UprisingState {
    /// Systems currently in active revolt (after `UprisingBegan` fired).
    pub active_uprisings: HashMap<SystemKey, ActiveUprising>,
    /// Cooldown: last tick an UprisingIncident was fired per system (prevents spam).
    pub incident_cooldowns: HashMap<SystemKey, u64>,
}

/// Data about an active uprising at one system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveUprising {
    /// Game tick when the uprising started.
    pub started_tick: u64,
    /// Loyalty level at uprising start (for diagnostic purposes).
    pub loyalty_at_start: i32,
}

impl UprisingState {
    pub fn new() -> Self {
        UprisingState {
            active_uprisings: HashMap::new(),
            incident_cooldowns: HashMap::new(),
        }
    }

    /// Returns `true` if the system is currently in revolt.
    pub fn is_uprising(&self, system: SystemKey) -> bool {
        self.active_uprisings.contains_key(&system)
    }

    /// Remove an active uprising at a system (e.g., after a successful SubdueUprising mission).
    pub fn clear_uprising(&mut self, system: SystemKey) {
        self.active_uprisings.remove(&system);
    }
}

// ---------------------------------------------------------------------------
// UprisingSystem
// ---------------------------------------------------------------------------

/// Stateless uprising evaluator.
pub struct UprisingSystem;

impl UprisingSystem {
    /// Advance uprising logic for all populated systems.
    ///
    /// For each populated system:
    /// - If not in active uprising: evaluate UPRIS1TB threshold. If loyalty
    ///   below lowest threshold, fire `UprisingIncident` (warning). If RNG roll
    ///   passes the probability, fire `UprisingBegan`.
    ///
    /// `rng_rolls` is a caller-provided slice of `[0, 1)` values consumed in
    /// system iteration order. If there are fewer rolls than evaluations, the
    /// remaining evaluations use `1.0` (never fires — safe conservative default).
    ///
    /// `upris1tb` and `upris2tb` are the loaded probability tables from
    /// `GameWorld::mission_tables` (keys `"UPRIS1TB"` / `"UPRIS2TB"`).
    pub fn advance(
        state: &mut UprisingState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        rng_rolls: &[f64],
        upris1tb: &MstbTable,
    ) -> Vec<UprisingEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let tick = tick_events.last().unwrap().tick;
        let mut events = Vec::new();
        let mut roll_idx = 0;

        for (sys_key, sys) in world.systems.iter() {
            // Gate: system must be populated (Ghidra: `*(uint*)(this + 0x88) & 1`)
            if !sys.is_populated() {
                continue;
            }

            // Skip systems already in active revolt (managed separately by missions)
            if state.is_uprising(sys_key) {
                continue;
            }

            let loyalty = sys.loyalty_value();

            // Incident threshold: fire warning if loyalty is below the minimum
            // UPRIS1TB threshold (i.e., even a small probability exists).
            // The lowest threshold in UPRIS1TB represents "danger zone."
            // Loyalty >= 0 means the system is at or above neutral — no uprising risk.
            // The UPRIS1TB thresholds are all negative (below neutral), so any positive
            // loyalty value should produce zero probability.
            let start_prob = if loyalty >= 0 {
                0u32
            } else {
                upris1tb.lookup(loyalty)
            } as f64;

            if start_prob > 0.0 {
                // Fire incident warning with 10-tick cooldown to prevent spam.
                // First incident always fires (no cooldown entry yet).
                let can_fire = match state.incident_cooldowns.get(&sys_key) {
                    Some(&last) => tick >= last + 10,
                    None => true,
                };
                if can_fire {
                    events.push(UprisingEvent::UprisingIncident { system: sys_key, tick });
                    state.incident_cooldowns.insert(sys_key, tick);
                }

                // Roll for actual uprising start
                let roll = rng_rolls.get(roll_idx).copied().unwrap_or(1.0);
                roll_idx += 1;

                if roll < start_prob / 100.0 {
                    events.push(UprisingEvent::UprisingBegan { system: sys_key, tick });
                    state.active_uprisings.insert(
                        sys_key,
                        ActiveUprising {
                            started_tick: tick,
                            loyalty_at_start: loyalty,
                        },
                    );
                }
            }
        }

        events
    }

    /// Attempt to subdue an uprising at `system` using UPRIS2TB probability.
    ///
    /// Returns `Some(UprisingSubdued)` if the subdue roll succeeds, `None` otherwise.
    /// Called by the mission system when a Subdue Uprising mission completes.
    pub fn try_subdue(
        state: &mut UprisingState,
        world: &GameWorld,
        system: SystemKey,
        tick: u64,
        rng_roll: f64,
        upris2tb: &MstbTable,
    ) -> Option<UprisingEvent> {
        if !state.is_uprising(system) {
            return None;
        }

        let loyalty = world
            .systems
            .get(system)
            .map(|s| s.loyalty_value())
            .unwrap_or(50);

        let subdue_prob = upris2tb.lookup(loyalty) as f64 / 100.0;
        if rng_roll < subdue_prob {
            state.active_uprisings.remove(&system);
            Some(UprisingEvent::UprisingSubdued { system, tick })
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// System loyalty helpers (extension methods added here to avoid modifying world/mod.rs)
// ---------------------------------------------------------------------------

/// Uprising-specific helpers for `System`.
///
/// Accessing these requires `use crate::uprising::SystemUprisingExt`.
pub trait SystemUprisingExt {
    /// Returns `true` if the system is populated.
    ///
    /// Mirrors the C++ gate `*(uint*)(this + 0x88) & 1` from `FUN_00509c30`.
    /// A system is considered populated if its `popularity_alliance + popularity_empire > 0`.
    fn is_populated(&self) -> bool;

    /// Returns the current loyalty value as a signed integer for threshold comparison.
    ///
    /// Uses the dominant faction's popularity fraction scaled to 0–100. The
    /// uprising tables use signed deltas from 50 (neutral). We convert the
    /// controlling faction's popularity into a 0–100 loyalty scale.
    fn loyalty_value(&self) -> i32;
}

impl SystemUprisingExt for crate::world::System {
    fn is_populated(&self) -> bool {
        self.popularity_alliance + self.popularity_empire > 0.0
    }

    fn loyalty_value(&self) -> i32 {
        use crate::dat::Faction;

        let raw = match self.controlling_faction {
            Some(Faction::Alliance) => self.popularity_alliance,
            Some(Faction::Empire) => self.popularity_empire,
            _ => 0.5, // neutral: treat as average loyalty
        };
        // Scale [0.0, 1.0] → [0, 100], then shift to signed delta from neutral (50)
        // Uprising tables use negative thresholds (below neutral = risk)
        ((raw * 100.0) as i32) - 50
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, Faction, SectorGroup};
    use crate::ids::DatId;
    use crate::tick::TickEvent;
    use crate::world::{GameWorld, MstbEntry, MstbTable, Sector, System};

    fn tick(n: u64) -> TickEvent {
        TickEvent { tick: n }
    }

    /// UPRIS1TB with 3 entries: very low (-40), low (-20), medium (-5) loyalty → 90, 50, 10%.
    fn make_upris1tb() -> MstbTable {
        MstbTable::new(vec![
            MstbEntry { threshold: -40, value: 90 },
            MstbEntry { threshold: -20, value: 50 },
            MstbEntry { threshold: -5,  value: 10 },
        ])
    }

    /// UPRIS2TB with 4 entries.
    fn make_upris2tb() -> MstbTable {
        MstbTable::new(vec![
            MstbEntry { threshold: -40, value: 20 },
            MstbEntry { threshold: -20, value: 40 },
            MstbEntry { threshold: -5,  value: 70 },
            MstbEntry { threshold: 10,  value: 90 },
        ])
    }

    fn make_world_with_system(loyalty_fraction: f32, faction: Faction) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });

        let (pop_alliance, pop_empire) = match faction {
            Faction::Alliance => (loyalty_fraction, 1.0 - loyalty_fraction),
            Faction::Empire => (1.0 - loyalty_fraction, loyalty_fraction),
            Faction::Neutral => (0.5, 0.5),
        };

        let sys = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Naboo".into(),
            sector,
            x: 10,
            y: 20,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: pop_alliance,
            popularity_empire: pop_empire,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: Some(faction),
        });

        (world, sys)
    }

    #[test]
    fn no_events_without_ticks() {
        let (world, _) = make_world_with_system(0.1, Faction::Alliance);
        let mut state = UprisingState::new();
        let events = UprisingSystem::advance(&mut state, &world, &[], &[0.0], &make_upris1tb());
        assert!(events.is_empty());
    }

    #[test]
    fn high_loyalty_no_uprising() {
        // Alliance loyalty = 80% → loyalty_value = 30 (above all thresholds in upris1tb)
        let (world, _) = make_world_with_system(0.8, Faction::Alliance);
        let mut state = UprisingState::new();
        let events =
            UprisingSystem::advance(&mut state, &world, &[tick(1)], &[0.0], &make_upris1tb());
        assert!(events.is_empty());
    }

    #[test]
    fn very_low_loyalty_fires_incident_and_uprising_on_low_roll() {
        // loyalty = 0.1 → loyalty_value = 10 - 50 = -40 → start_prob = 90%
        let (world, sys) = make_world_with_system(0.1, Faction::Alliance);
        let mut state = UprisingState::new();
        // Roll 0.05 → 5% < 90% → uprising fires
        let events =
            UprisingSystem::advance(&mut state, &world, &[tick(1)], &[0.05], &make_upris1tb());

        assert!(events.iter().any(|e| matches!(e, UprisingEvent::UprisingIncident { .. })));
        assert!(events.iter().any(|e| matches!(e, UprisingEvent::UprisingBegan { .. })));
        assert!(state.is_uprising(sys));
    }

    #[test]
    fn very_low_loyalty_fires_incident_but_not_uprising_on_high_roll() {
        // loyalty = 0.1 → start_prob = 90%
        let (world, _) = make_world_with_system(0.1, Faction::Alliance);
        let mut state = UprisingState::new();
        // Roll 0.95 → 95% > 90% → uprising does NOT fire, only incident
        let events =
            UprisingSystem::advance(&mut state, &world, &[tick(1)], &[0.95], &make_upris1tb());

        assert!(events.iter().any(|e| matches!(e, UprisingEvent::UprisingIncident { .. })));
        assert!(!events.iter().any(|e| matches!(e, UprisingEvent::UprisingBegan { .. })));
    }

    #[test]
    fn system_in_active_uprising_skipped_for_new_uprising() {
        let (world, sys) = make_world_with_system(0.1, Faction::Alliance);
        let mut state = UprisingState::new();
        // Seed the system as already uprising
        state.active_uprisings.insert(
            sys,
            ActiveUprising { started_tick: 1, loyalty_at_start: -40 },
        );

        let events =
            UprisingSystem::advance(&mut state, &world, &[tick(2)], &[0.0], &make_upris1tb());
        // Should fire neither incident nor began for an already-revolting system
        assert!(!events.iter().any(|e| matches!(e, UprisingEvent::UprisingBegan { .. })));
    }

    #[test]
    fn subdue_success() {
        let (world, sys) = make_world_with_system(0.1, Faction::Alliance);
        let mut state = UprisingState::new();
        state.active_uprisings.insert(
            sys,
            ActiveUprising { started_tick: 1, loyalty_at_start: -40 },
        );

        // UPRIS2TB at loyalty = -40 → subdue_prob = 20%. Roll 0.1 → success.
        let result = UprisingSystem::try_subdue(&mut state, &world, sys, 5, 0.1, &make_upris2tb());
        assert!(matches!(result, Some(UprisingEvent::UprisingSubdued { .. })));
        assert!(!state.is_uprising(sys));
    }

    #[test]
    fn subdue_failure() {
        let (world, sys) = make_world_with_system(0.1, Faction::Alliance);
        let mut state = UprisingState::new();
        state.active_uprisings.insert(
            sys,
            ActiveUprising { started_tick: 1, loyalty_at_start: -40 },
        );

        // UPRIS2TB at loyalty = -40 → subdue_prob = 20%. Roll 0.9 → failure.
        let result = UprisingSystem::try_subdue(&mut state, &world, sys, 5, 0.9, &make_upris2tb());
        assert!(result.is_none());
        assert!(state.is_uprising(sys)); // still in revolt
    }

    #[test]
    fn subdue_noop_if_no_uprising() {
        let (world, sys) = make_world_with_system(0.8, Faction::Alliance);
        let mut state = UprisingState::new();
        let result = UprisingSystem::try_subdue(&mut state, &world, sys, 5, 0.0, &make_upris2tb());
        assert!(result.is_none());
    }

    #[test]
    fn unpopulated_system_skipped() {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Empty".into(),
            group: SectorGroup::RimOuter,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // Both popularity fields = 0.0 → is_populated() returns false
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Void".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Unexplored,
            popularity_alliance: 0.0,
            popularity_empire: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
        });

        let mut state = UprisingState::new();
        let events =
            UprisingSystem::advance(&mut state, &world, &[tick(1)], &[0.0], &make_upris1tb());
        assert!(events.is_empty());
    }
}
