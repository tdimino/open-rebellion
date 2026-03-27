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

// Phase 3b: new constants from source tree cross-reference
const GNPRTB_EMPIRE_TROOP_MULT: u16 = 7680;    // =2: Empire troop doubling + garrison halving divisor
const GNPRTB_UPRISING_GARRISON_MULT: u16 = 7682; // =2: garrison doubles during uprising
const GNPRTB_KDY_CAPSHIP_PENALTY: u16 = 7684;   // =5: KDY production penalty per capital ship
const GNPRTB_KDY_FIGHTER_PENALTY: u16 = 7685;   // =2: KDY production penalty per fighter
const GNPRTB_ENERGY_CONTROL_THRESHOLD: u16 = 7760; // =60: energy needed for control eligibility
const GNPRTB_MAINTENANCE_RATE_CONTROLLED: u16 = 7694; // =30: ticks between maintenance checks (controlled)
const GNPRTB_MAINTENANCE_RATE_NEUTRAL: u16 = 7696;   // =30: ticks between maintenance checks (neutral)

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
    /// Production speed modifier from fleet/KDY presence (0-100).
    pub production_modifier: i8,
    /// Current energy output (sum of production facility outputs, capped at system capacity).
    /// FUN_00509ed0: if energy_allocated > System.total_energy, cap it.
    pub energy_allocated: u32,
    /// Current raw material output (sum of mine outputs, capped at system capacity).
    /// FUN_0050a220: if raw_material_allocated > System.raw_materials, cap it.
    pub raw_material_allocated: u32,
    /// True if facilities exceed energy capacity (facility pruning needed).
    pub energy_overcapped: bool,
    /// True if mines exceed raw material capacity (mine pruning needed).
    pub raw_material_overcapped: bool,
}

impl Default for SystemEconomy {
    fn default() -> Self {
        Self {
            collection_rate: 1.0,
            garrison_requirement: 0,
            production_modifier: 0,
            energy_allocated: 0,
            raw_material_allocated: 0,
            energy_overcapped: false,
            raw_material_overcapped: false,
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
    /// Energy allocated exceeds system capacity (FUN_00509ed0).
    EnergyOvercapped {
        system: SystemKey,
        allocated: u32,
        capacity: u32,
    },
    /// Raw material output exceeds system capacity (FUN_0050a220).
    RawMaterialOvercapped {
        system: SystemKey,
        allocated: u32,
        capacity: u32,
    },
}

// ---------------------------------------------------------------------------
// Economy system
// ---------------------------------------------------------------------------

pub struct EconomySystem;

impl EconomySystem {
    /// Run the per-system economy tick. Mutates EconomyState in-place,
    /// returns events for the integrator.
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

            // 0a. Resource capacity enforcement (FUN_00509ed0 + FUN_00509ef0 + FUN_0050a220).
            // Sum facility/mine outputs and cap at system limits.
            let eco = state.per_system.entry(sys_key).or_default();
            let (energy_alloc, raw_alloc) = calculate_resource_allocation(world, sys);
            let energy_cap = sys.total_energy as u32;
            let raw_cap = sys.raw_materials as u32;

            eco.energy_allocated = energy_alloc.min(energy_cap);
            eco.energy_overcapped = energy_alloc > energy_cap;
            eco.raw_material_allocated = raw_alloc.min(raw_cap);
            eco.raw_material_overcapped = raw_alloc > raw_cap;

            if eco.energy_overcapped {
                events.push(EconomyEvent::EnergyOvercapped {
                    system: sys_key,
                    allocated: energy_alloc,
                    capacity: energy_cap,
                });
            }
            if eco.raw_material_overcapped {
                events.push(EconomyEvent::RawMaterialOvercapped {
                    system: sys_key,
                    allocated: raw_alloc,
                    capacity: raw_cap,
                });
            }

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

            // Update per-system economy state (eco already bound above in step 0a).
            let old_rate = eco.collection_rate;
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
    // All computation in integer 0-100 to match original (FUN_005583c0).
    // Our popularity fields are f32 0.0-1.0; convert at boundary.
    let fleet_influence = gnprtb.value(GNPRTB_FLEET_INFLUENCE, difficulty).max(1) as i32;
    let fighter_influence = gnprtb.value(GNPRTB_FIGHTER_INFLUENCE, difficulty).max(1) as i32;
    let troop_influence = gnprtb.value(GNPRTB_TROOP_INFLUENCE, difficulty).max(1) as i32;
    let drift_threshold = gnprtb.value(GNPRTB_DRIFT_THRESHOLD, difficulty) as i32; // 40
    let threshold_1 = gnprtb.value(GNPRTB_DRIFT_THRESHOLD_1, difficulty) as i32; // 20
    let threshold_2 = gnprtb.value(GNPRTB_DRIFT_THRESHOLD_2, difficulty) as i32; // 30
    let base_low = gnprtb.value(GNPRTB_DRIFT_BASE_LOW, difficulty) as i32; // 75
    let base_low_mid = gnprtb.value(GNPRTB_DRIFT_BASE_LOW_MID, difficulty) as i32; // 50
    let base_mid = gnprtb.value(GNPRTB_DRIFT_BASE_MID, difficulty) as i32; // 25

    let is_alliance_controlled = matches!(
        sys.control,
        ControlKind::Controlled(crate::dat::Faction::Alliance)
    );
    let is_empire_controlled = matches!(
        sys.control,
        ControlKind::Controlled(crate::dat::Faction::Empire)
    );

    if !is_alliance_controlled && !is_empire_controlled {
        return (0.0, 0.0);
    }

    let (controlling_support_f32, friendly_fleets, friendly_fighters, friendly_troops) =
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

    // Convert f32 0.0-1.0 to integer 0-100 for computation
    let support = (controlling_support_f32 * 100.0) as i32;

    // Only drift if support is below threshold AND no friendly fleet present.
    // Original: if (support <= GNPRTB[7732]) && (fleet1 == 0)
    if support > drift_threshold || friendly_fleets > 0 {
        return (0.0, 0.0);
    }

    // Determine base drift rate based on support bracket.
    // Original bracket logic from FUN_005583c0:
    //   if threshold_1 < support <= threshold_2: base = base_low_mid (50)
    //   elif support > threshold_1: base = base_mid (25)
    //   else: base = base_low (75)
    let base = if support <= threshold_1 {
        base_low // ≤20: strong drift (75)
    } else if support <= threshold_2 {
        base_low_mid // 21-30: moderate drift (50)
    } else {
        base_mid // 31-40: mild drift (25)
    };

    // Empire troop doubling via FUN_005582e0_adjust_value_for_strong_support:
    // When side==Empire (side==2), troop count is multiplied by GNPRTB[7680] (=2).
    // This doubles troop suppression effectiveness for the Empire.
    let adjusted_troops = if is_empire_controlled {
        let empire_mult = gnprtb.value(GNPRTB_EMPIRE_TROOP_MULT, difficulty).max(1) as i32;
        friendly_troops as i32 * empire_mult
    } else {
        friendly_troops as i32
    };

    // Military suppression: integer subtraction matching original exactly.
    // Original: clamp(base - fighters*GNPRTB[7687] - troops_adj*GNPRTB[7688] - fleet*GNPRTB[7686], 0, 100)
    let suppression = friendly_fighters as i32 * fighter_influence
        + adjusted_troops * troop_influence
        + friendly_fleets as i32 * fleet_influence;

    let drift = (base - suppression).clamp(0, 100);

    // Convert drift back to f32 0.0-1.0 delta.
    // Original applies full drift as integer per game-day; we store as f32 0.0-1.0.
    let delta = drift as f32 / 100.0;

    // Drift direction: support moves AWAY from controlling faction.
    // If Empire controlled, drift is negated (support moves toward Alliance).
    if is_alliance_controlled {
        (-delta, delta)
    } else {
        (delta, -delta)
    }
}

// ---------------------------------------------------------------------------
// Collection rate (FUN_00558390)
// ---------------------------------------------------------------------------

/// Calculate resource collection rate from popular support.
///
/// Formula: `(GNPRTB[7763] * 100) / max(support_pct, 1)`
/// Higher support = lower collection rate (less taxation needed).
/// Range: [1.0, 100.0]. At full support (1.0) rate = 1.0; at zero support rate = 100.0.
// ---------------------------------------------------------------------------
// Resource capacity (FUN_00509ed0 + FUN_00509ef0 + FUN_0050a220)
// ---------------------------------------------------------------------------

/// Calculate total energy and raw material allocation from facilities and mines.
///
/// Returns (energy_allocated, raw_material_allocated).
/// The original sums facility outputs (virtual method at +0x1c8) and mine outputs,
/// then caps at system limits. If overcapped, the original randomly prunes facilities/mines.
/// We report overcap via events and let the caller decide on pruning.
fn calculate_resource_allocation(
    world: &GameWorld,
    sys: &crate::world::System,
) -> (u32, u32) {
    // Sum production facility outputs (energy generators).
    // Each production facility contributes 1 unit of energy output.
    let energy_output = sys.production_facilities.len() as u32;

    // Sum mine outputs (raw material generators).
    // In the original, mines have a per-mine virtual output method.
    // We count manufacturing facilities that are mines (by class type).
    // For now, use a simple count — each mine contributes 1 raw material unit.
    // TODO: distinguish mine facilities from non-mine manufacturing facilities
    // once ManufacturingFacilityInstance has class-type information.
    let raw_material_output = sys.manufacturing_facilities.len() as u32;

    (energy_output, raw_material_output)
}

fn calculate_collection_rate(support: f32, gnprtb: &GnprtbParams, difficulty: u8) -> f32 {
    let base = gnprtb.value(GNPRTB_COLLECTION_RATE_BASE, difficulty).max(1) as f32;
    let support_pct = (support * 100.0).max(1.0);
    (base / support_pct).clamp(1.0, 100.0)
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
    // Integer arithmetic matching FUN_005587d0_uprising_threshold + FUN_00558760_garrison_requirement.
    // Our support is f32 0.0-1.0; convert to integer 0-100.
    let support_int = (support * 100.0) as i32;
    let threshold = gnprtb.value(GNPRTB_GARRISON_THRESHOLD, difficulty) as i32; // 60
    let divisor = gnprtb.value(GNPRTB_GARRISON_DIVISOR, difficulty).abs().max(1) as i32; // 10

    if support_int >= threshold {
        return 0;
    }

    // Integer ceil division: ceil(dividend / divisor) = (divisor - 1 + dividend) / divisor
    let dividend = threshold - support_int;
    let raw = (divisor - 1 + dividend) / divisor;

    // Empire halving via GNPRTB[7680] (=2, used as divisor).
    // Original: if Empire + param_3: garrison /= GNPRTB[7680]
    let after_faction = match control {
        ControlKind::Controlled(crate::dat::Faction::Empire) => {
            let empire_divisor = gnprtb.value(GNPRTB_EMPIRE_TROOP_MULT, difficulty).max(1) as i32;
            raw / empire_divisor
        }
        _ => raw,
    };

    // Uprising doubler via GNPRTB[7682] (=2).
    // Original: if system under uprising (field_0x88 bit 2): garrison *= GNPRTB[7682]
    // Our ControlKind::Uprising(faction) maps directly to this bit.
    let after_uprising = match control {
        ControlKind::Uprising(_) => {
            let uprising_mult = gnprtb.value(GNPRTB_UPRISING_GARRISON_MULT, difficulty).max(1) as i32;
            after_faction * uprising_mult
        }
        _ => after_faction,
    };

    after_uprising.max(0) as u32
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
            // Phase 3b additions: all economy tick GNPRTB indices (values from GNPRTB.DAT)
            GnprtbEntry { parameter_id: 7680, development: 2, alliance_sp_easy: 2, alliance_sp_medium: 2, alliance_sp_hard: 2, empire_sp_easy: 2, empire_sp_medium: 2, empire_sp_hard: 2, multiplayer: 2 },   // Empire troop doubling + garrison halving
            GnprtbEntry { parameter_id: 7682, development: 2, alliance_sp_easy: 2, alliance_sp_medium: 2, alliance_sp_hard: 2, empire_sp_easy: 2, empire_sp_medium: 2, empire_sp_hard: 2, multiplayer: 2 },   // Uprising garrison doubler
            GnprtbEntry { parameter_id: 7684, development: 5, alliance_sp_easy: 5, alliance_sp_medium: 5, alliance_sp_hard: 5, empire_sp_easy: 5, empire_sp_medium: 5, empire_sp_hard: 5, multiplayer: 5 },   // KDY capship penalty per unit
            GnprtbEntry { parameter_id: 7685, development: 2, alliance_sp_easy: 2, alliance_sp_medium: 2, alliance_sp_hard: 2, empire_sp_easy: 2, empire_sp_medium: 2, empire_sp_hard: 2, multiplayer: 2 },   // KDY fighter penalty per unit
            GnprtbEntry { parameter_id: 7691, development: 0, alliance_sp_easy: 0, alliance_sp_medium: 0, alliance_sp_hard: 0, empire_sp_easy: 0, empire_sp_medium: 0, empire_sp_hard: 0, multiplayer: 0 },   // Support delta Empire-controlled transition
            GnprtbEntry { parameter_id: 7693, development: 1, alliance_sp_easy: 1, alliance_sp_medium: 1, alliance_sp_hard: 1, empire_sp_easy: 1, empire_sp_medium: 1, empire_sp_hard: 1, multiplayer: 1 },   // Support delta favors controller
            GnprtbEntry { parameter_id: 7694, development: 30, alliance_sp_easy: 30, alliance_sp_medium: 30, alliance_sp_hard: 30, empire_sp_easy: 30, empire_sp_medium: 30, empire_sp_hard: 30, multiplayer: 30 },  // Maintenance check rate (controlled)
            GnprtbEntry { parameter_id: 7695, development: -1, alliance_sp_easy: -1, alliance_sp_medium: -1, alliance_sp_hard: -1, empire_sp_easy: -1, empire_sp_medium: -1, empire_sp_hard: -1, multiplayer: -1 },  // Support delta opposes controller
            GnprtbEntry { parameter_id: 7696, development: 30, alliance_sp_easy: 30, alliance_sp_medium: 30, alliance_sp_hard: 30, empire_sp_easy: 30, empire_sp_medium: 30, empire_sp_hard: 30, multiplayer: 30 },  // Maintenance check rate (neutral)
            GnprtbEntry { parameter_id: 7697, development: -1, alliance_sp_easy: -1, alliance_sp_medium: -1, alliance_sp_hard: -1, empire_sp_easy: -1, empire_sp_medium: -1, empire_sp_hard: -1, multiplayer: -1 },  // Support delta controlled systems
            GnprtbEntry { parameter_id: 7760, development: 60, alliance_sp_easy: 60, alliance_sp_medium: 60, alliance_sp_hard: 60, empire_sp_easy: 60, empire_sp_medium: 60, empire_sp_hard: 60, multiplayer: 60 },  // Energy-based control threshold
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

    // -----------------------------------------------------------------------
    // Phase 3b: Resource capacity enforcement tests
    // -----------------------------------------------------------------------

    #[test]
    fn energy_overcap_emits_event() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with total_energy=2 but 5 production facilities
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Overcap".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 2, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000001), is_alliance: true,
                }),
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000002), is_alliance: true,
                }),
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000003), is_alliance: true,
                }),
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000004), is_alliance: true,
                }),
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000005), is_alliance: true,
                }),
            ],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        let events = EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        // Should emit EnergyOvercapped (5 facilities > 2 capacity)
        assert!(
            events.iter().any(|e| matches!(e, EconomyEvent::EnergyOvercapped { .. })),
            "Expected EnergyOvercapped event"
        );

        // energy_allocated should be capped at capacity
        let eco = state.per_system.get(&sys_key).unwrap();
        assert_eq!(eco.energy_allocated, 2, "energy should be capped at system capacity");
        assert!(eco.energy_overcapped, "should flag overcap");
    }

    #[test]
    fn no_overcap_when_within_limits() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with total_energy=10 and only 3 production facilities
        world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Normal".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000001), is_alliance: true,
                }),
                world.production_facilities.insert(crate::world::ProductionFacilityInstance {
                    class_dat_id: DatId(0x18000002), is_alliance: true,
                }),
            ],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        let events = EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        // Should NOT emit overcap events
        assert!(
            !events.iter().any(|e| matches!(e, EconomyEvent::EnergyOvercapped { .. })),
            "Should not overcap with 2 facilities and capacity 10"
        );
    }

    #[test]
    fn raw_material_overcap_emits_event() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with raw_materials=1 but 3 manufacturing facilities
        world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "MineCap".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 1,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![
                world.manufacturing_facilities.insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(0x16000001), is_alliance: true,
                }),
                world.manufacturing_facilities.insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(0x16000002), is_alliance: true,
                }),
                world.manufacturing_facilities.insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId(0x16000003), is_alliance: true,
                }),
            ],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        let events = EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        assert!(
            events.iter().any(|e| matches!(e, EconomyEvent::RawMaterialOvercapped { .. })),
            "Expected RawMaterialOvercapped event"
        );
    }
}
