//! Per-system economy tick loop — the territorial control feedback system.
//!
//! Implements the 18 sub-functions from the original `FUN_005073d0_adjust_and_deploy_each_system`.
//! Runs every tick for every system. Governs popular support drift, collection rates,
//! garrison requirements, and resource allocation.
//!
//! Open Souls mapping: this is a "cognitive step" — a pure function that transforms
//! economy state + read-only GameWorld into a Vec of effects.
//!
//! GNPRTB parameters used:
//!   7686 = 10 (fleet influence on support drift)
//!   7687 = 5  (fighter influence on support drift)
//!   7688 = 2  (troop influence on support drift)
//!   7732 = 40 (support drift threshold)
//!   7733 = 25 (drift base, mid bracket)
//!   7734 = 30 (drift threshold 2)
//!   7735 = 50 (drift base, low-mid bracket)
//!   7736 = 20 (drift threshold 1)
//!   7737 = 75 (drift base, low bracket)
//!   7761 = 60 (garrison requirement threshold)
//!   7762 = -10 (garrison requirement divisor)
//!   7763 = 100 (collection rate base)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ids::*;
use crate::tick::TickEvent;
use crate::world::{ControlKind, GameWorld, GnprtbParams};

// ---------------------------------------------------------------------------
// GNPRTB parameter IDs
// ---------------------------------------------------------------------------

const GNPRTB_FLEET_INFLUENCE: u16 = 7686;
const GNPRTB_FIGHTER_INFLUENCE: u16 = 7687;
const GNPRTB_TROOP_INFLUENCE: u16 = 7688;
const GNPRTB_DRIFT_THRESHOLD: u16 = 7732;
const GNPRTB_DRIFT_BASE_MID: u16 = 7733;
const GNPRTB_DRIFT_THRESHOLD_2: u16 = 7734;
const GNPRTB_DRIFT_BASE_LOW_MID: u16 = 7735;
const GNPRTB_DRIFT_THRESHOLD_1: u16 = 7736;
const GNPRTB_DRIFT_BASE_LOW: u16 = 7737;
const GNPRTB_GARRISON_THRESHOLD: u16 = 7761;
const GNPRTB_GARRISON_DIVISOR: u16 = 7762;
const GNPRTB_COLLECTION_RATE_BASE: u16 = 7763;

// ---------------------------------------------------------------------------
// Economy state
// ---------------------------------------------------------------------------

/// Per-system resource and economy tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEconomy {
    /// Resource collection rate, inversely proportional to support.
    pub collection_rate: f32,
    /// Number of troops required to prevent uprising.
    pub garrison_requirement: u32,
    /// Production speed modifier from fleet/KDY presence (-100 to +100).
    pub production_modifier: i8,
}

impl Default for SystemEconomy {
    fn default() -> Self {
        Self {
            collection_rate: 1.0,
            garrison_requirement: 0,
            production_modifier: 0,
        }
    }
}

/// Economy state across all systems.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EconomyState {
    pub per_system: HashMap<SystemKey, SystemEconomy>,
}

// ---------------------------------------------------------------------------
// Economy events (system output)
// ---------------------------------------------------------------------------

/// Events produced by the economy tick.
#[derive(Debug, Clone)]
pub enum EconomyEvent {
    /// Popular support shifted at a system.
    SupportDrifted {
        system: SystemKey,
        alliance_delta: f32,
        empire_delta: f32,
    },
    /// Collection rate recalculated.
    CollectionRateChanged {
        system: SystemKey,
        new_rate: f32,
    },
    /// Garrison requirement recalculated.
    GarrisonRequirementChanged {
        system: SystemKey,
        new_requirement: u32,
    },
}

// ---------------------------------------------------------------------------
// Economy system
// ---------------------------------------------------------------------------

pub struct EconomySystem;

impl EconomySystem {
    /// Run the per-system economy tick. Pure function: reads GameWorld, mutates
    /// EconomyState, returns events for the integrator.
    ///
    /// Runs BEFORE manufacturing in the tick order (economy affects production).
    pub fn advance(
        state: &mut EconomyState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        difficulty: u8,
    ) -> Vec<EconomyEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let gnprtb = &world.gnprtb;
        let mut events = Vec::new();

        // Collect system keys first to avoid borrow issues.
        let system_keys: Vec<SystemKey> = world.systems.keys().collect();

        for sys_key in system_keys {
            let sys = match world.systems.get(sys_key) {
                Some(s) => s,
                None => continue,
            };

            // Only process populated, non-destroyed systems.
            if !sys.is_populated || sys.is_destroyed {
                continue;
            }

            // Count military presence at this system.
            let presence = count_military_presence(world, sys);

            // 1. Calculate and apply popular support drift.
            let (alliance_delta, empire_delta) =
                calculate_support_drift(sys, &presence, gnprtb, difficulty);

            if alliance_delta.abs() > f32::EPSILON || empire_delta.abs() > f32::EPSILON {
                events.push(EconomyEvent::SupportDrifted {
                    system: sys_key,
                    alliance_delta,
                    empire_delta,
                });
            }

            // 2. Calculate collection rate.
            let controlling_support = match sys.control {
                ControlKind::Controlled(crate::dat::Faction::Alliance) => sys.popularity_alliance,
                ControlKind::Controlled(crate::dat::Faction::Empire) => sys.popularity_empire,
                _ => 0.5, // Neutral/contested: baseline
            };
            let new_rate = calculate_collection_rate(controlling_support, gnprtb, difficulty);

            // 3. Calculate garrison requirement.
            let new_garrison = calculate_garrison_requirement(
                controlling_support,
                &sys.control,
                gnprtb,
                difficulty,
            );

            // Update per-system economy state.
            let eco = state.per_system.entry(sys_key).or_default();
            let old_rate = eco.collection_rate;
            eco.collection_rate = new_rate;
            let old_garrison = eco.garrison_requirement;
            eco.collection_rate = new_rate;
            eco.garrison_requirement = new_garrison;

            if (new_rate - old_rate).abs() > 0.01 {
                events.push(EconomyEvent::CollectionRateChanged {
                    system: sys_key,
                    new_rate,
                });
            }

            if new_garrison != old_garrison {
                events.push(EconomyEvent::GarrisonRequirementChanged {
                    system: sys_key,
                    new_requirement: new_garrison,
                });
            }
        }

        events
    }
}

// ---------------------------------------------------------------------------
// Military presence counting
// ---------------------------------------------------------------------------

/// Military presence at a system, used by support drift calculations.
struct MilitaryPresence {
    alliance_fleets: u32,
    empire_fleets: u32,
    alliance_fighters: u32,
    empire_fighters: u32,
    alliance_troops: u32,
    empire_troops: u32,
}

fn count_military_presence(world: &GameWorld, sys: &crate::world::System) -> MilitaryPresence {
    let mut presence = MilitaryPresence {
        alliance_fleets: 0,
        empire_fleets: 0,
        alliance_fighters: 0,
        empire_fighters: 0,
        alliance_troops: 0,
        empire_troops: 0,
    };

    for &fleet_key in &sys.fleets {
        if let Some(fleet) = world.fleets.get(fleet_key) {
            if fleet.is_alliance {
                presence.alliance_fleets += 1;
                presence.alliance_fighters +=
                    fleet.fighters.iter().map(|f| f.count).sum::<u32>();
            } else {
                presence.empire_fleets += 1;
                presence.empire_fighters +=
                    fleet.fighters.iter().map(|f| f.count).sum::<u32>();
            }
        }
    }

    for &troop_key in &sys.ground_units {
        if let Some(troop) = world.troops.get(troop_key) {
            if troop.is_alliance {
                presence.alliance_troops += 1;
            } else {
                presence.empire_troops += 1;
            }
        }
    }

    presence
}

// ---------------------------------------------------------------------------
// Popular support drift (FUN_005583c0)
// ---------------------------------------------------------------------------

/// Calculate support drift for a system based on military presence.
///
/// From the decompiled `FUN_005583c0_calculate_adjusted_support_value`:
/// ```text
/// if support <= 40 AND no_friendly_fleet:
///     if support > 20 AND support <= 30: base = 50
///     elif support > 20: base = 25
///     else: base = 75
///     drift = clamp(base - fighters*5 - troops*2 - fleet_presence*10, 0, 100)
/// if empire_controlled: drift = -drift
/// ```
fn calculate_support_drift(
    sys: &crate::world::System,
    presence: &MilitaryPresence,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> (f32, f32) {
    let fleet_influence = gnprtb.value(GNPRTB_FLEET_INFLUENCE, difficulty).max(1) as f32;
    let fighter_influence = gnprtb.value(GNPRTB_FIGHTER_INFLUENCE, difficulty).max(1) as f32;
    let troop_influence = gnprtb.value(GNPRTB_TROOP_INFLUENCE, difficulty).max(1) as f32;
    let drift_threshold = gnprtb.value(GNPRTB_DRIFT_THRESHOLD, difficulty) as f32 / 100.0;
    let threshold_1 = gnprtb.value(GNPRTB_DRIFT_THRESHOLD_1, difficulty) as f32 / 100.0;
    let threshold_2 = gnprtb.value(GNPRTB_DRIFT_THRESHOLD_2, difficulty) as f32 / 100.0;
    let base_low = gnprtb.value(GNPRTB_DRIFT_BASE_LOW, difficulty) as f32 / 100.0;
    let base_low_mid = gnprtb.value(GNPRTB_DRIFT_BASE_LOW_MID, difficulty) as f32 / 100.0;
    let base_mid = gnprtb.value(GNPRTB_DRIFT_BASE_MID, difficulty) as f32 / 100.0;

    let is_alliance_controlled = matches!(
        sys.control,
        ControlKind::Controlled(crate::dat::Faction::Alliance)
    );
    let is_empire_controlled = matches!(
        sys.control,
        ControlKind::Controlled(crate::dat::Faction::Empire)
    );

    if !is_alliance_controlled && !is_empire_controlled {
        // Neutral/uncontrolled systems don't drift.
        return (0.0, 0.0);
    }

    let (controlling_support, friendly_fleets, friendly_fighters, friendly_troops) =
        if is_alliance_controlled {
            (
                sys.popularity_alliance,
                presence.alliance_fleets,
                presence.alliance_fighters,
                presence.alliance_troops,
            )
        } else {
            (
                sys.popularity_empire,
                presence.empire_fleets,
                presence.empire_fighters,
                presence.empire_troops,
            )
        };

    // Only drift if support is below threshold and no friendly fleet present.
    if controlling_support > drift_threshold || friendly_fleets > 0 {
        return (0.0, 0.0);
    }

    // Determine base drift rate based on support bracket.
    let base = if controlling_support <= threshold_1 {
        base_low // Very low support: strong drift
    } else if controlling_support <= threshold_2 {
        base_low_mid // Low-mid: moderate drift
    } else {
        base_mid // Mid: mild drift
    };

    // Military presence reduces drift (troops/fighters suppress dissent).
    let suppression = (friendly_fighters as f32 * fighter_influence
        + friendly_troops as f32 * troop_influence * 2.0
        + friendly_fleets as f32 * fleet_influence)
        / 100.0;

    let drift = (base - suppression).clamp(0.0, 1.0);

    // Drift direction: support moves AWAY from controlling faction.
    // Scale to a small per-tick delta (original applies per game-day).
    let per_tick_drift = drift * 0.01; // 1% of computed drift per tick

    if is_alliance_controlled {
        (-per_tick_drift, per_tick_drift)
    } else {
        (per_tick_drift, -per_tick_drift)
    }
}

// ---------------------------------------------------------------------------
// Collection rate (FUN_00558390)
// ---------------------------------------------------------------------------

/// Calculate resource collection rate from popular support.
///
/// Formula: `(GNPRTB[7763] * 100) / max(support_pct, 1)`
/// Higher support = lower collection rate (less taxation needed).
fn calculate_collection_rate(support: f32, gnprtb: &GnprtbParams, difficulty: u8) -> f32 {
    let base = gnprtb.value(GNPRTB_COLLECTION_RATE_BASE, difficulty).max(1) as f32;
    let support_pct = (support * 100.0).max(1.0);
    base / support_pct
}

// ---------------------------------------------------------------------------
// Garrison requirement (FUN_00558760 + FUN_005587d0)
// ---------------------------------------------------------------------------

/// Calculate troops needed to prevent uprising at this system.
///
/// Formula: `(threshold - support_pct) / abs(divisor)` when support < threshold.
fn calculate_garrison_requirement(
    support: f32,
    control: &ControlKind,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> u32 {
    let threshold = gnprtb.value(GNPRTB_GARRISON_THRESHOLD, difficulty) as f32 / 100.0;
    let divisor = gnprtb.value(GNPRTB_GARRISON_DIVISOR, difficulty).abs().max(1) as f32 / 100.0;

    if support >= threshold {
        return 0;
    }

    let raw = (threshold - support) / divisor;

    // Empire core systems need half the garrison (GNPRTB 7680 = 2).
    let modifier = match control {
        ControlKind::Controlled(crate::dat::Faction::Empire) => 0.5,
        _ => 1.0,
    };

    (raw * modifier).ceil() as u32
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::GameWorld;

    /// Create a GnprtbParams with the stock economy values.
    fn stock_gnprtb() -> GnprtbParams {
        use crate::world::GnprtbEntry;
        let entries = vec![
            GnprtbEntry { parameter_id: 7686, development: 10, alliance_sp_easy: 10, alliance_sp_medium: 10, alliance_sp_hard: 10, empire_sp_easy: 10, empire_sp_medium: 10, empire_sp_hard: 10, multiplayer: 10 },
            GnprtbEntry { parameter_id: 7687, development: 5, alliance_sp_easy: 5, alliance_sp_medium: 5, alliance_sp_hard: 5, empire_sp_easy: 5, empire_sp_medium: 5, empire_sp_hard: 5, multiplayer: 5 },
            GnprtbEntry { parameter_id: 7688, development: 2, alliance_sp_easy: 2, alliance_sp_medium: 2, alliance_sp_hard: 2, empire_sp_easy: 2, empire_sp_medium: 2, empire_sp_hard: 2, multiplayer: 2 },
            GnprtbEntry { parameter_id: 7732, development: 40, alliance_sp_easy: 40, alliance_sp_medium: 40, alliance_sp_hard: 40, empire_sp_easy: 40, empire_sp_medium: 40, empire_sp_hard: 40, multiplayer: 40 },
            GnprtbEntry { parameter_id: 7733, development: 25, alliance_sp_easy: 25, alliance_sp_medium: 25, alliance_sp_hard: 25, empire_sp_easy: 25, empire_sp_medium: 25, empire_sp_hard: 25, multiplayer: 25 },
            GnprtbEntry { parameter_id: 7734, development: 30, alliance_sp_easy: 30, alliance_sp_medium: 30, alliance_sp_hard: 30, empire_sp_easy: 30, empire_sp_medium: 30, empire_sp_hard: 30, multiplayer: 30 },
            GnprtbEntry { parameter_id: 7735, development: 50, alliance_sp_easy: 50, alliance_sp_medium: 50, alliance_sp_hard: 50, empire_sp_easy: 50, empire_sp_medium: 50, empire_sp_hard: 50, multiplayer: 50 },
            GnprtbEntry { parameter_id: 7736, development: 20, alliance_sp_easy: 20, alliance_sp_medium: 20, alliance_sp_hard: 20, empire_sp_easy: 20, empire_sp_medium: 20, empire_sp_hard: 20, multiplayer: 20 },
            GnprtbEntry { parameter_id: 7737, development: 75, alliance_sp_easy: 75, alliance_sp_medium: 75, alliance_sp_hard: 75, empire_sp_easy: 75, empire_sp_medium: 75, empire_sp_hard: 75, multiplayer: 75 },
            GnprtbEntry { parameter_id: 7761, development: 60, alliance_sp_easy: 60, alliance_sp_medium: 60, alliance_sp_hard: 60, empire_sp_easy: 60, empire_sp_medium: 60, empire_sp_hard: 60, multiplayer: 60 },
            GnprtbEntry { parameter_id: 7762, development: -10, alliance_sp_easy: -10, alliance_sp_medium: -10, alliance_sp_hard: -10, empire_sp_easy: -10, empire_sp_medium: -10, empire_sp_hard: -10, multiplayer: -10 },
            GnprtbEntry { parameter_id: 7763, development: 100, alliance_sp_easy: 100, alliance_sp_medium: 100, alliance_sp_hard: 100, empire_sp_easy: 100, empire_sp_medium: 100, empire_sp_hard: 100, multiplayer: 100 },
        ];
        GnprtbParams::new(entries)
    }

    #[test]
    fn support_drift_no_garrison_low_support() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.8,
            is_populated: true,
            total_energy: 5,
            raw_materials: 5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 0, empire_troops: 0,
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2);
        // Low alliance support (0.1 < 0.40 threshold), no fleet → should drift away
        assert!(a_delta < 0.0, "alliance support should decrease: {a_delta}");
        assert!(e_delta > 0.0, "empire support should increase: {e_delta}");
    }

    #[test]
    fn support_drift_with_fleet_suppresses() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.8,
            is_populated: true,
            total_energy: 5, raw_materials: 5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 1, empire_fleets: 0, // Friendly fleet present
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 0, empire_troops: 0,
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2);
        // Fleet present → no drift
        assert!(a_delta.abs() < f32::EPSILON, "should not drift with fleet: {a_delta}");
        assert!(e_delta.abs() < f32::EPSILON, "should not drift with fleet: {e_delta}");
    }

    #[test]
    fn support_drift_with_troops_reduces() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.15,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 5, raw_materials: 5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let no_troops = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 0, empire_troops: 0,
        };
        let with_troops = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 5, empire_troops: 0,
        };
        let (a_no, _) = calculate_support_drift(&sys, &no_troops, &gnprtb, 2);
        let (a_with, _) = calculate_support_drift(&sys, &with_troops, &gnprtb, 2);
        // Troops should reduce drift magnitude
        assert!(
            a_with.abs() < a_no.abs(),
            "troops should reduce drift: with={a_with} vs without={a_no}"
        );
    }

    #[test]
    fn collection_rate_high_support() {
        let gnprtb = stock_gnprtb();
        let rate = calculate_collection_rate(0.8, &gnprtb, 2);
        // 100 / 80 = 1.25
        assert!((rate - 1.25).abs() < 0.01, "expected ~1.25, got {rate}");
    }

    #[test]
    fn collection_rate_low_support() {
        let gnprtb = stock_gnprtb();
        let rate = calculate_collection_rate(0.1, &gnprtb, 2);
        // 100 / 10 = 10.0
        assert!((rate - 10.0).abs() < 0.1, "expected ~10.0, got {rate}");
    }

    #[test]
    fn garrison_requirement_below_threshold() {
        let gnprtb = stock_gnprtb();
        let control = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let req = calculate_garrison_requirement(0.3, &control, &gnprtb, 2);
        // (0.60 - 0.30) / 0.10 = 3.0 → 3 troops
        assert_eq!(req, 3, "expected 3, got {req}");
    }

    #[test]
    fn garrison_requirement_above_threshold_is_zero() {
        let gnprtb = stock_gnprtb();
        let control = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let req = calculate_garrison_requirement(0.7, &control, &gnprtb, 2);
        assert_eq!(req, 0, "above threshold should need 0 garrison");
    }

    #[test]
    fn garrison_requirement_empire_halved() {
        let gnprtb = stock_gnprtb();
        let alliance = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let empire = ControlKind::Controlled(crate::dat::Faction::Empire);
        let req_alliance = calculate_garrison_requirement(0.3, &alliance, &gnprtb, 2);
        let req_empire = calculate_garrison_requirement(0.3, &empire, &gnprtb, 2);
        assert!(
            req_empire < req_alliance,
            "empire garrison should be less: empire={req_empire} vs alliance={req_alliance}"
        );
    }

    #[test]
    fn economy_advance_no_ticks_no_events() {
        let mut state = EconomyState::default();
        let world = GameWorld::default();
        let events = EconomySystem::advance(&mut state, &world, &[], 2);
        assert!(events.is_empty());
    }

    #[test]
    fn economy_advance_skips_destroyed_systems() {
        let mut state = EconomyState::default();
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Destroyed".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 5, raw_materials: 5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: true, // Destroyed!
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });
        let tick_events = vec![TickEvent { tick: 1 }];
        let events = EconomySystem::advance(&mut state, &world, &tick_events, 2);
        // Destroyed system should be skipped entirely.
        assert!(events.is_empty(), "destroyed system should produce no events");
    }

    #[test]
    fn economy_advance_produces_telemetry() {
        let mut state = EconomyState::default();
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let _sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Coruscant".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.8,
            is_populated: true,
            total_energy: 10, raw_materials: 8,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });
        let tick_events = vec![TickEvent { tick: 1 }];
        let events = EconomySystem::advance(&mut state, &world, &tick_events, 2);
        // Should produce at least garrison requirement events.
        assert!(!events.is_empty(), "should produce economy events");
    }
}
