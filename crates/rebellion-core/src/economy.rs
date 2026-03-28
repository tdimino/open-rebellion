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

/// Incident state flags (bits 16-19 of original field_0x88).
/// When these change between ticks, the corresponding incident notification fires.
/// FUN_0050a970 evaluates these each tick; FUN_0050d720 dispatches on transitions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentFlags {
    /// Bit 16 (0x10000): uprising incident active (event 0x152).
    pub uprising: bool,
    /// Bit 17 (0x20000): informant incident active (event 0x153).
    pub informant: bool,
    /// Bit 18 (0x40000): disaster incident active (event 0x154).
    pub disaster: bool,
    /// Bit 19 (0x80000): resource incident active (event 0x155).
    pub resource: bool,
}

/// Fleet posture summary for a system (FUN_0050add0/af70/b4c0).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FleetPosture {
    /// Number of Alliance capital ship hulls at this system.
    pub alliance_capships: u32,
    /// Number of Empire capital ship hulls at this system.
    pub empire_capships: u32,
    /// True if both sides have fleets (2+ each) present.
    pub is_contested: bool,
}

/// Fighter posture for a system (FUN_0050aa50).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FighterPosture {
    /// True if both sides have fighters present.
    pub is_contested: bool,
    /// Alliance fighter count.
    pub alliance_fighters: u32,
    /// Empire fighter count.
    pub empire_fighters: u32,
}

/// Derived summary computed per-system per-tick (functions 9-15 of economy pipeline).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemSummary {
    /// Troops present minus garrison requirement (negative = deficit).
    /// FUN_0050a670.
    pub troop_surplus: i32,
    /// Total troops for the controlling faction.
    /// FUN_0050ac00.
    pub total_controlling_troops: u32,
    /// Whether an orbital shipyard is present.
    /// FUN_0050ace0.
    pub has_shipyard: bool,
    /// Fleet posture (3-pass result).
    pub fleet_posture: FleetPosture,
    /// Fighter posture.
    pub fighter_posture: FighterPosture,
    /// Bit 11 of original field_0x88: "strong support" flag.
    /// When true AND Empire-controlled, troop suppression is doubled (GNPRTB[7680]).
    /// Set when controlling faction's support > drift threshold (GNPRTB[7732]=40).
    pub strong_support: bool,
}

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
    /// Derived troop/fleet/shipyard summary (functions 9-15).
    pub summary: SystemSummary,
    /// Incident state flags (bits 16-19 of field_0x88). FUN_0050a970.
    /// When these change from the previous tick, corresponding incidents fire.
    pub incident_flags: IncidentFlags,
    /// System is visibly under uprising. FUN_0050ac70.
    pub uprising_visible: bool,
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
            summary: SystemSummary::default(),
            incident_flags: IncidentFlags::default(),
            uprising_visible: false,
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
    /// System control resolved from troop presence (FUN_0050a780_system_join_side).
    ControlResolved {
        system: SystemKey,
        new_control: ControlKind,
    },
    /// Incident state changed — fire the corresponding notification (FUN_0050a970 + FUN_0050d720).
    IncidentTriggered {
        system: SystemKey,
        incident_type: &'static str,
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
            // Pass previous tick's strong_support flag (bit 11 of field_0x88).
            let (alliance_delta, empire_delta) =
                calculate_support_drift(sys, &presence, gnprtb, difficulty, eco.summary.strong_support);

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

            // 4. KDY production modifier (FUN_0050a480_adjust_for_kdy).
            // Formula: clamp_nonneg(100 - capships * GNPRTB[7684] - fighters * GNPRTB[7685])
            // Original only applies at systems with KDY flag (field_0x88 bit 5).
            // We apply to all controlled systems — modifier is 100 when no ships, harmless.
            // Note: uses hull counts (ShipEntry.count per fleet), not fleet object counts.
            // The original iterates fleet ship lists and sums individual hull entries.
            let capship_penalty = gnprtb.value(GNPRTB_KDY_CAPSHIP_PENALTY, difficulty) as i32;
            let fighter_penalty = gnprtb.value(GNPRTB_KDY_FIGHTER_PENALTY, difficulty) as i32;
            let total_capships = presence.alliance_capships as i32 + presence.empire_capships as i32;
            let total_fighters = presence.alliance_fighters as i32 + presence.empire_fighters as i32;
            let new_prod_mod = (100 - total_capships * capship_penalty
                - total_fighters * fighter_penalty)
                .max(0)
                .min(100) as i8;

            // 5. Troop-based side resolution (FUN_0050a780_system_join_side).
            // Original resolves controlling faction every tick from troop presence.
            // If Alliance troops only: Alliance controls. Empire only: Empire controls.
            // Both: keep existing (contested). Neither: uncontrolled.
            let resolved_control = resolve_system_control(
                sys, &presence, gnprtb, difficulty,
            );
            if resolved_control != sys.control {
                events.push(EconomyEvent::ControlResolved {
                    system: sys_key,
                    new_control: resolved_control,
                });
            }

            // Update per-system economy state (eco already bound above in step 0a).
            let old_rate = eco.collection_rate;
            let old_garrison = eco.garrison_requirement;
            eco.collection_rate = new_rate;
            eco.garrison_requirement = new_garrison;
            eco.production_modifier = new_prod_mod;

            // 6. Troop/fleet summary propagation (FUN_0050a670 through FUN_0050aa50).
            eco.summary = compute_system_summary(world, sys, &presence, eco.garrison_requirement, gnprtb, difficulty);

            // 7. Incident state + uprising visibility (FUN_0050a970 + FUN_0050ac70).
            // Evaluate incident flags based on system state. Fire events on transitions.
            let new_flags = evaluate_incident_flags(sys, &eco.summary, &eco);
            let old_flags = &eco.incident_flags;
            if new_flags.uprising && !old_flags.uprising {
                events.push(EconomyEvent::IncidentTriggered { system: sys_key, incident_type: "uprising" });
            }
            if new_flags.informant && !old_flags.informant {
                events.push(EconomyEvent::IncidentTriggered { system: sys_key, incident_type: "informant" });
            }
            if new_flags.disaster && !old_flags.disaster {
                events.push(EconomyEvent::IncidentTriggered { system: sys_key, incident_type: "disaster" });
            }
            if new_flags.resource && !old_flags.resource {
                events.push(EconomyEvent::IncidentTriggered { system: sys_key, incident_type: "resource" });
            }
            eco.incident_flags = new_flags;

            // Uprising visibility flag
            eco.uprising_visible = matches!(sys.control, ControlKind::Uprising(_));

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

/// Military presence at a system, used by support drift and KDY calculations.
struct MilitaryPresence {
    alliance_fleets: u32,
    empire_fleets: u32,
    alliance_fighters: u32,
    empire_fighters: u32,
    alliance_troops: u32,
    empire_troops: u32,
    /// Total capital ship hulls (sum of ShipEntry.count across all fleets).
    /// Used by KDY production modifier (FUN_0050a480).
    alliance_capships: u32,
    empire_capships: u32,
}

fn count_military_presence(world: &GameWorld, sys: &crate::world::System) -> MilitaryPresence {
    let mut presence = MilitaryPresence {
        alliance_fleets: 0,
        empire_fleets: 0,
        alliance_fighters: 0,
        empire_fighters: 0,
        alliance_troops: 0,
        empire_troops: 0,
        alliance_capships: 0,
        empire_capships: 0,
    };

    for &fleet_key in &sys.fleets {
        if let Some(fleet) = world.fleets.get(fleet_key) {
            let capship_count: u32 = fleet.capital_ships.iter().map(|s| s.count).sum();
            let fighter_count: u32 = fleet.fighters.iter().map(|f| f.count).sum();
            if fleet.is_alliance {
                presence.alliance_fleets += 1;
                presence.alliance_capships += capship_count;
                presence.alliance_fighters += fighter_count;
            } else {
                presence.empire_fleets += 1;
                presence.empire_capships += capship_count;
                presence.empire_fighters += fighter_count;
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
    strong_support: bool,
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
    let support = (controlling_support_f32 * 100.0).round() as i32;

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
    // When side==Empire (side==2) AND strong_support bit (field_0x88 bit 11) is set,
    // troop count is multiplied by GNPRTB[7680] (=2).
    // This doubles troop suppression effectiveness for the Empire.
    let adjusted_troops = if is_empire_controlled && strong_support {
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

// ---------------------------------------------------------------------------
// Troop-based side resolution (FUN_0050a780_system_join_side)
// ---------------------------------------------------------------------------

/// Resolve which faction controls a system based on troop presence.
///
/// Original logic (FUN_0050a780):
/// - If system not populated: Uncontrolled
/// - If only Alliance troops: Alliance controls
/// - If only Empire troops: Empire controls
/// - If both: keep existing control (Contested)
/// - If neither: Uncontrolled
fn resolve_system_control(
    sys: &crate::world::System,
    presence: &MilitaryPresence,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> ControlKind {
    if !sys.is_populated {
        return ControlKind::Uncontrolled;
    }

    let has_alliance = presence.alliance_troops > 0;
    let has_empire = presence.empire_troops > 0;

    match (has_alliance, has_empire) {
        (true, false) => ControlKind::Controlled(crate::dat::Faction::Alliance),
        (false, true) => ControlKind::Controlled(crate::dat::Faction::Empire),
        (true, true) => ControlKind::Contested,
        (false, false) => {
            // No troops — original checks energy vs GNPRTB[7760] threshold.
            // If system energy meets threshold, preserve existing control; else uncontrolled.
            let threshold = gnprtb.value(GNPRTB_ENERGY_CONTROL_THRESHOLD, difficulty) as u8;
            if sys.total_energy >= threshold {
                sys.control
            } else {
                ControlKind::Uncontrolled
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Troop/fleet summary propagation (FUN_0050a670 through FUN_0050aa50)
// ---------------------------------------------------------------------------

/// Compute the per-system derived summary from troop/fleet/facility presence.
/// Implements functions 9-15 of the economy tick pipeline.
fn compute_system_summary(
    world: &GameWorld,
    sys: &crate::world::System,
    presence: &MilitaryPresence,
    garrison_requirement: u32,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> SystemSummary {
    // FUN_0050a670: troop surplus = controlling troops - garrison requirement
    let controlling_troops = match sys.control {
        ControlKind::Controlled(crate::dat::Faction::Alliance) => presence.alliance_troops,
        ControlKind::Controlled(crate::dat::Faction::Empire) => presence.empire_troops,
        _ => presence.alliance_troops.max(presence.empire_troops),
    };
    let troop_surplus = controlling_troops as i32 - garrison_requirement as i32;

    // FUN_0050ac00: total controlling troops
    let total_controlling_troops = controlling_troops;

    // FUN_0050ace0: shipyard presence (check defense facilities for shipyard type)
    // Original checks virtual method on facility class. We check for any defense facility.
    // TODO: distinguish shipyard class once facility type promotion is complete.
    let has_shipyard = !sys.defense_facilities.is_empty();

    // FUN_0050add0/af70/b4c0: fleet posture (3 passes)
    let fleet_posture = FleetPosture {
        alliance_capships: presence.alliance_capships,
        empire_capships: presence.empire_capships,
        is_contested: presence.alliance_fleets >= 2 && presence.empire_fleets >= 2,
    };

    // FUN_0050aa50: fighter posture
    let fighter_posture = FighterPosture {
        is_contested: presence.alliance_fighters > 0 && presence.empire_fighters > 0,
        alliance_fighters: presence.alliance_fighters,
        empire_fighters: presence.empire_fighters,
    };

    // Bit 11 of field_0x88: "strong support" — set when controlling faction's
    // support exceeds the drift threshold. Controls Empire troop doubling in
    // FUN_005582e0_adjust_value_for_strong_support.
    let drift_threshold = gnprtb.value(GNPRTB_DRIFT_THRESHOLD, difficulty) as i32;
    let controlling_support = match sys.control {
        ControlKind::Controlled(crate::dat::Faction::Alliance) =>
            (sys.popularity_alliance * 100.0).round() as i32,
        ControlKind::Controlled(crate::dat::Faction::Empire) =>
            (sys.popularity_empire * 100.0).round() as i32,
        _ => 0,
    };
    let strong_support = controlling_support > drift_threshold;

    SystemSummary {
        troop_surplus,
        total_controlling_troops,
        has_shipyard,
        fleet_posture,
        fighter_posture,
        strong_support,
    }
}

// ---------------------------------------------------------------------------
// Incident evaluation (FUN_0050a970 + FUN_0050ac70)
// ---------------------------------------------------------------------------

/// Evaluate incident flags for a system based on its current state.
///
/// In the original, FUN_0050a970 evaluates field_0x88 bits and the galaxy
/// notification hub (FUN_0050d720) fires incidents on state transitions.
/// We compute flags each tick and let the advance loop detect transitions.
///
/// Flags:
/// - uprising: system under uprising (ControlKind::Uprising only)
/// - informant: system has negative troop surplus (garrison shortfall)
/// - disaster: system has very low support (below 20%)
/// - resource: facilities exceed system capacity (energy or raw material overcap)
fn evaluate_incident_flags(
    sys: &crate::world::System,
    summary: &SystemSummary,
    eco: &SystemEconomy,
) -> IncidentFlags {
    IncidentFlags {
        // Uprising incident: system is under uprising (field_0x88 bit 2).
        // Original checks only the uprising control state, not troop deficit.
        uprising: matches!(sys.control, ControlKind::Uprising(_)),
        // Informant incident: garrison shortfall (mild deficit)
        informant: summary.troop_surplus < 0,
        // Disaster incident: very low popular support (below 20%)
        disaster: {
            let controlling_support = match sys.control {
                ControlKind::Controlled(crate::dat::Faction::Alliance) => sys.popularity_alliance,
                ControlKind::Controlled(crate::dat::Faction::Empire) => sys.popularity_empire,
                _ => 0.5,
            };
            controlling_support < 0.20
        },
        // Resource incident: facilities exceed system capacity (overcap).
        // Original checks field_0x88 bit 19 which maps to resource overcap state.
        resource: eco.energy_overcapped || eco.raw_material_overcapped,
    }
}

fn calculate_collection_rate(support: f32, gnprtb: &GnprtbParams, difficulty: u8) -> f32 {
    // Original: FUN_0053c8d0_calculate_percentage(GNPRTB[7763], 100, support)
    //         = (100 * GNPRTB[7763]) / max(support_int, 1)
    //         = 10000 / support_int  (with stock GNPRTB[7763]=100)
    // Result: integer percentage (100 at full support, 10000 at near-zero).
    // Higher values = higher taxation burden on the system.
    let base = gnprtb.value(GNPRTB_COLLECTION_RATE_BASE, difficulty).max(1) as i32;
    let support_int = (support * 100.0).round() as i32;
    let support_clamped = support_int.max(1);
    let rate = (100 * base) / support_clamped;
    rate.max(1) as f32
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
    let support_int = (support * 100.0).round() as i32;
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
            alliance_capships: 0, empire_capships: 0,
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
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
            alliance_capships: 0, empire_capships: 0,
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
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
            alliance_capships: 0, empire_capships: 0,
        };
        let with_troops = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 5, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let (a_no, _) = calculate_support_drift(&sys, &no_troops, &gnprtb, 2, true);
        let (a_with, _) = calculate_support_drift(&sys, &with_troops, &gnprtb, 2, true);
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
        // (100 * 100) / 80 = 125
        assert!((rate - 125.0).abs() < 0.01, "expected 125.0, got {rate}");
    }

    #[test]
    fn collection_rate_low_support() {
        let gnprtb = stock_gnprtb();
        let rate = calculate_collection_rate(0.1, &gnprtb, 2);
        // (100 * 100) / 10 = 1000
        assert!((rate - 1000.0).abs() < 0.1, "expected 1000.0, got {rate}");
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
    fn kdy_production_modifier_reduces_with_ships() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with fleet presence (capships reduce production)
        let fleet_key = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: vec![
                crate::world::ShipEntry { class: crate::ids::CapitalShipKey::default(), count: 3 },
            ],
            fighters: vec![
                crate::world::FighterEntry { class: crate::ids::FighterKey::default(), count: 4 },
            ],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "KDY".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.7, popularity_empire: 0.3,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![fleet_key],
            ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        let eco = state.per_system.get(&sys_key).unwrap();
        // 3 capships * 5 (capship penalty) + 4 fighters * 2 (fighter penalty) = 15 + 8 = 23
        // production_modifier = max(100 - 23, 0) = 77
        assert_eq!(eco.production_modifier, 77,
            "production_modifier should be 100 - 3*5 - 4*2 = 77, got {}",
            eco.production_modifier);
    }

    #[test]
    fn kdy_production_modifier_floors_at_zero() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with massive fleet presence
        let fleet_key = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: vec![
                crate::world::ShipEntry { class: crate::ids::CapitalShipKey::default(), count: 10 },
            ],
            fighters: vec![
                crate::world::FighterEntry { class: crate::ids::FighterKey::default(), count: 30 },
            ],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Overcrowded".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.7, popularity_empire: 0.3,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![fleet_key],
            ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        let eco = state.per_system.get(&sys_key).unwrap();
        // 10 capships * 5 + 30 fighters * 2 = 50 + 60 = 110 → max(100 - 110, 0) = 0
        assert_eq!(eco.production_modifier, 0,
            "production modifier should floor at 0, got {}",
            eco.production_modifier);
    }

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

    // -----------------------------------------------------------------------
    // Phase 3b: Troop-based side resolution tests
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Phase 3b: Incident state tests
    // -----------------------------------------------------------------------

    #[test]
    fn incident_fires_on_state_transition() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with very low alliance support (< 20%) — triggers disaster incident
        world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Crisis".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.10, popularity_empire: 0.90,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        // First tick: should fire disaster incident (transition from false to true)
        let events1 = EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);
        assert!(
            events1.iter().any(|e| matches!(e, EconomyEvent::IncidentTriggered { incident_type: "disaster", .. })),
            "Should fire disaster incident on first tick when support < 20%"
        );

        // Second tick: should NOT fire again (no transition — flags already set)
        let events2 = EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 2 }], 2);
        assert!(
            !events2.iter().any(|e| matches!(e, EconomyEvent::IncidentTriggered { incident_type: "disaster", .. })),
            "Should NOT re-fire disaster incident when flags unchanged"
        );
    }

    #[test]
    fn informant_incident_on_garrison_shortfall() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with low support (garrison > 0) but no troops (deficit)
        world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Undermanned".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.4, popularity_empire: 0.6,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        let events = EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);
        assert!(
            events.iter().any(|e| matches!(e, EconomyEvent::IncidentTriggered { incident_type: "informant", .. })),
            "Should fire informant incident on garrison shortfall"
        );
    }

    // -----------------------------------------------------------------------
    // Phase 3b: System summary tests
    // -----------------------------------------------------------------------

    #[test]
    fn system_summary_troop_surplus() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        // System with low support (triggers garrison requirement) and some troops
        let troop1 = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: DatId(0x14000100), is_alliance: true, regiment_strength: 100,
        });
        let troop2 = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: DatId(0x14000100), is_alliance: true, regiment_strength: 100,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Test".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.3, popularity_empire: 0.7,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![troop1, troop2], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        let eco = state.per_system.get(&sys_key).unwrap();
        // Garrison = ceil(30/10) = 3, troops = 2, surplus = 2 - 3 = -1
        assert_eq!(eco.summary.troop_surplus, -1,
            "troop surplus should be troops - garrison = 2 - 3 = -1, got {}",
            eco.summary.troop_surplus);
        assert_eq!(eco.summary.total_controlling_troops, 2);
    }

    #[test]
    fn system_summary_fleet_posture() {
        let mut world = GameWorld::default();
        world.gnprtb = stock_gnprtb();
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0), name: "S".into(),
            group: crate::dat::SectorGroup::Core, x: 0, y: 0, systems: vec![],
        });
        let fleet1 = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: vec![crate::world::ShipEntry { class: crate::ids::CapitalShipKey::default(), count: 2 }],
            fighters: vec![crate::world::FighterEntry { class: crate::ids::FighterKey::default(), count: 5 }],
            characters: vec![], is_alliance: true, has_death_star: false,
        });
        let fleet2 = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: vec![crate::world::ShipEntry { class: crate::ids::CapitalShipKey::default(), count: 3 }],
            fighters: vec![], characters: vec![], is_alliance: false, has_death_star: false,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Contested".into(), sector: sector_key,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![fleet1, fleet2], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        EconomySystem::advance(&mut state, &world, &[TickEvent { tick: 1 }], 2);

        let eco = state.per_system.get(&sys_key).unwrap();
        assert_eq!(eco.summary.fleet_posture.alliance_capships, 2);
        assert_eq!(eco.summary.fleet_posture.empire_capships, 3);
        assert_eq!(eco.summary.fighter_posture.alliance_fighters, 5);
        assert_eq!(eco.summary.fighter_posture.empire_fighters, 0);
    }

    // -----------------------------------------------------------------------
    // Phase 3b: Troop-based side resolution tests
    // -----------------------------------------------------------------------

    #[test]
    fn side_resolution_alliance_troops_only() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Uncontrolled,
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 3, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let result = resolve_system_control(&sys, &presence, &gnprtb, 2);
        assert_eq!(result, ControlKind::Controlled(crate::dat::Faction::Alliance));
    }

    #[test]
    fn side_resolution_both_troops_contested() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 2, empire_troops: 3,
            alliance_capships: 0, empire_capships: 0,
        };
        let result = resolve_system_control(&sys, &presence, &gnprtb, 2);
        assert_eq!(result, ControlKind::Contested);
    }

    #[test]
    fn side_resolution_no_troops_energy_above_threshold_preserves() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 60, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 0, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let result = resolve_system_control(&sys, &presence, &gnprtb, 2);
        // No troops but energy >= GNPRTB[7760](60) — preserves existing control
        assert_eq!(result, ControlKind::Controlled(crate::dat::Faction::Empire));
    }

    #[test]
    fn side_resolution_no_troops_low_energy_becomes_uncontrolled() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 0, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let result = resolve_system_control(&sys, &presence, &gnprtb, 2);
        // No troops and energy < GNPRTB[7760](60) — becomes uncontrolled
        assert_eq!(result, ControlKind::Uncontrolled);
    }

    // --- Task #42: Empire troop doubling in support drift ---

    #[test]
    fn support_drift_empire_troop_doubling() {
        let gnprtb = stock_gnprtb();
        // Alliance-controlled system with low support and 2 troops
        let sys_alliance = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1, popularity_empire: 0.8,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence_alliance = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 2, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let (a_delta_alliance, _) = calculate_support_drift(&sys_alliance, &presence_alliance, &gnprtb, 2, true);

        // Empire-controlled system with same setup and 2 troops
        let sys_empire = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8, popularity_empire: 0.1,
            is_populated: true, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence_empire = MilitaryPresence {
            alliance_fleets: 0, empire_fleets: 0,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 0, empire_troops: 2,
            alliance_capships: 0, empire_capships: 0,
        };
        let (_, e_delta_empire) = calculate_support_drift(&sys_empire, &presence_empire, &gnprtb, 2, true);

        // Empire troops get doubled via GNPRTB[7680]=2, so Empire drift should be
        // less (more suppression) than Alliance drift with same troop count.
        // Both deltas are negative (drift away from controller).
        assert!(
            e_delta_empire.abs() < a_delta_alliance.abs(),
            "Empire should have less drift (more suppression) due to troop doubling: empire={e_delta_empire}, alliance={a_delta_alliance}"
        );
    }

    // --- Task #43: Unpopulated system skipping ---

    #[test]
    fn unpopulated_system_skipped_in_advance() {
        let gnprtb = stock_gnprtb();
        let mut world = GameWorld::default();
        world.gnprtb = gnprtb;
        let _sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Unpopulated".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: false, total_energy: 10, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });

        let mut state = EconomyState::default();
        let tick = vec![TickEvent { tick: 1 }];
        let events = EconomySystem::advance(&mut state, &world, &tick, 2);

        // Unpopulated system should produce zero events and no economy entry
        assert!(events.is_empty(), "unpopulated system should produce no events: {events:?}");
        assert!(state.per_system.is_empty(), "no economy entry for unpopulated system");
    }

    // --- Task #44: Uprising garrison doubling ---

    #[test]
    fn garrison_uprising_doubling() {
        let gnprtb = stock_gnprtb();
        // Controlled system: garrison = ceil((60 - 30) / 10) = 3
        let garrison_controlled = calculate_garrison_requirement(
            0.30,
            &ControlKind::Controlled(crate::dat::Faction::Alliance),
            &gnprtb,
            2,
        );
        // Uprising system: garrison = 3 * GNPRTB[7682](2) = 6
        let garrison_uprising = calculate_garrison_requirement(
            0.30,
            &ControlKind::Uprising(crate::dat::Faction::Alliance),
            &gnprtb,
            2,
        );
        assert_eq!(garrison_controlled, 3, "controlled garrison");
        assert_eq!(garrison_uprising, garrison_controlled * 2, "uprising garrison should be 2x controlled");
    }

    // --- Task #45: Collection rate at zero support + drift at threshold boundary ---

    #[test]
    fn collection_rate_at_zero_support() {
        let gnprtb = stock_gnprtb();
        let rate = calculate_collection_rate(0.0, &gnprtb, 2);
        // At zero support: (100 * 100) / max(0, 1) = 10000
        assert!((rate - 10000.0).abs() < f32::EPSILON, "zero support should yield max rate: {rate}");
    }

    #[test]
    fn support_drift_at_exact_threshold() {
        let gnprtb = stock_gnprtb();
        // Support at exactly 0.40 (threshold = 40 in integer)
        let sys = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.40, popularity_empire: 0.5,
            is_populated: true, total_energy: 10, raw_materials: 10,
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
            alliance_capships: 0, empire_capships: 0,
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
        // At exactly threshold (40), integer comparison: 40 > 40 is false → drift occurs.
        // Original: `if (support <= GNPRTB[7732])` means 40 <= 40 passes → drift.
        assert!(a_delta != 0.0, "at threshold boundary drift should occur: {a_delta}");
        assert!(e_delta != 0.0, "at threshold boundary drift should occur: {e_delta}");
    }

    // --- Task #46: Fleet posture is_contested + strengthen telemetry ---

    #[test]
    fn fleet_posture_contested_requires_two_per_side() {
        let gnprtb = stock_gnprtb();
        let mut world = GameWorld::default();
        world.gnprtb = gnprtb;

        let sys = crate::world::System {
            dat_id: DatId(0), name: "Test".into(),
            sector: SectorKey::default(),
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true, total_energy: 60, raw_materials: 10,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence_1v1 = MilitaryPresence {
            alliance_fleets: 1, empire_fleets: 1,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 5, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let gnprtb = stock_gnprtb();
        let summary_1v1 = compute_system_summary(&world, &sys, &presence_1v1, 0, &gnprtb, 2);
        assert!(!summary_1v1.fleet_posture.is_contested, "1v1 fleets should NOT be contested");

        let presence_2v2 = MilitaryPresence {
            alliance_fleets: 2, empire_fleets: 2,
            alliance_fighters: 0, empire_fighters: 0,
            alliance_troops: 5, empire_troops: 0,
            alliance_capships: 0, empire_capships: 0,
        };
        let summary_2v2 = compute_system_summary(&world, &sys, &presence_2v2, 0, &gnprtb, 2);
        assert!(summary_2v2.fleet_posture.is_contested, "2v2 fleets should be contested");
    }
}
