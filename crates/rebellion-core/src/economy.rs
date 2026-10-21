//! Per-system economy tick loop — the territorial control feedback system.
//!
//! Implements the 18 sub-functions from the original `FUN_00508250`
//! (the community dump calls it `FUN_005073d0_adjust_and_deploy_each_system`).
//! Runs every tick for every system. Governs popular support drift, collection rates,
//! garrison requirements, and resource allocation.
//!
//! Open Souls mapping: this is a "cognitive step" — a pure function that transforms
//! economy state + read-only `GameWorld` into a Vec of effects.
//!
//! GNPRTB parameters used:
//!   7686 = 10 (each usable capital ship's influence on support drift)
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

use crate::ids::SystemKey;
use crate::tick::TickEvent;
use crate::world::{ControlKind, GameWorld, GnprtbParams};

// ---------------------------------------------------------------------------
// GNPRTB parameter IDs
// ---------------------------------------------------------------------------

// FUN_00558bb0 loads GNPRTB 0x1e06 (7686) into DAT_006bb3f4; FUN_00559c40 scales
// the holder's usable capital ships by it (p4, FUN_00509090).
const GNPRTB_SHIP_INFLUENCE: u16 = 7686;
// FUN_00558bb0 loads GNPRTB 0x1e07 (7687) into DAT_006bb3c0; FUN_00559c40 scales it.
const GNPRTB_FIGHTER_INFLUENCE: u16 = 7687;
// FUN_00558bb0 loads GNPRTB 0x1e08 (7688) into DAT_006bb3bc; FUN_00559c40 scales it.
const GNPRTB_TROOP_INFLUENCE: u16 = 7688;
// FUN_00558bb0 loads GNPRTB 0x1e34 (7732) into DAT_006bb478, the outer drift gate in FUN_00559c40.
const GNPRTB_DRIFT_THRESHOLD: u16 = 7732;
// FUN_00558bb0 loads GNPRTB 0x1e35 (7733) into DAT_006bb3f8, the base above the upper inner cut, up to the outer gate (support 31-40).
const GNPRTB_DRIFT_BASE_MID: u16 = 7733;
// FUN_00558bb0 loads GNPRTB 0x1e36 (7734) into DAT_006bb434, the upper inner cut in FUN_00559c40.
const GNPRTB_DRIFT_THRESHOLD_2: u16 = 7734;
// FUN_00558bb0 loads GNPRTB 0x1e37 (7735) into DAT_006bb4b0, the base between the two inner cuts (support 21-30).
const GNPRTB_DRIFT_BASE_LOW_MID: u16 = 7735;
// FUN_00558bb0 loads GNPRTB 0x1e38 (7736) into DAT_006bb438, the lower inner cut in FUN_00559c40.
const GNPRTB_DRIFT_THRESHOLD_1: u16 = 7736;
// FUN_00558bb0 loads GNPRTB 0x1e39 (7737) into DAT_006bb424, the base at or below the lower cut.
const GNPRTB_DRIFT_BASE_LOW: u16 = 7737;
// FUN_0055a050 (uprising-incident.md): the support below which the score term applies, GNPRTB 7761 (60).
const GNPRTB_GARRISON_THRESHOLD: u16 = 7761;
// FUN_0055a050 (uprising-incident.md): the score term's divisor, GNPRTB 7762 (-10).
const GNPRTB_GARRISON_DIVISOR: u16 = 7762;
// FUN_00559c10 (top-bar-resource-counters.md): collection rate = GNPRTB 7763 * 100 / max(support, 1).
const GNPRTB_COLLECTION_RATE_BASE: u16 = 7763;

// Phase 3b: new constants from source tree cross-reference
// GNPRTB 7680 (0x1e00, DAT_006bb3dc): FUN_00559b60 multiplies the Empire's regiments by it and
// FUN_00559fe0 divides the garrison requirement by it (support-shifts.md, bit 11).
const GNPRTB_EMPIRE_TROOP_MULT: u16 = 7680; // =2: Empire troop doubling + garrison halving divisor

// GNPRTB 7682 (0x1e02, DAT_006bb3e4): FUN_00559fe0 multiplies the requirement in an uprising.
const GNPRTB_UPRISING_GARRISON_MULT: u16 = 7682; // =2: garrison doubles during uprising

// GNPRTB 7684 (0x1e04, DAT_006bb458), read by FUN_00558bb0 (blockade-troop-withdrawal.md).
const GNPRTB_KDY_CAPSHIP_PENALTY: u16 = 7684; // =5: per-capital-ship cut to a blockaded system's troop-withdrawal percentage

// GNPRTB 7685 (0x1e05, DAT_006bb3cc), read by FUN_00558bb0 (blockade-troop-withdrawal.md).
const GNPRTB_KDY_FIGHTER_PENALTY: u16 = 7685; // =2: per-fighter cut to a blockaded system's troop-withdrawal percentage
const GNPRTB_SUPPORT_CONTROL_THRESHOLD: u16 = 7760; // =60: support a side needs to hold a system without troops (DAT_006bb4c0, FUN_00558bb0)

// ---------------------------------------------------------------------------
// Economy state
// ---------------------------------------------------------------------------

/// Incident state flags (original system `+0x88` bits 16-19).
/// When a flag turns on between ticks, the corresponding incident message fires.
///
/// In the original each incident is a one-tick pulse raised by a timer event,
/// and the system vtable (`0x0065e640`) holds each incident's effect.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "These independent flags preserve the existing state and serialization model."
)]
pub struct IncidentFlags {
    /// Bit 16 (0x10000), `FUN_0050aa50`; views get
    /// `SystemUprisingIncidentNotif` (`FUN_00512580`). Raised by
    /// `UprisingSystem`; always clear here, kept for the save layout.
    pub uprising: bool,
    /// Bit 17 (0x20000), `FUN_0050aac0`; views get
    /// `SystemInformantIncidentNotif` (`FUN_005125d0`). Our trigger has no
    /// source (F-029).
    pub informant: bool,
    /// Bit 18 (0x40000), `FUN_0050ab30`; views get
    /// `SystemDisasterIncidentNotif` (`FUN_00512620`). Raised by
    /// `UprisingSystem`; always clear here, kept for the save layout.
    pub disaster: bool,
    /// Bit 19 (0x80000), `FUN_0050aba0`; views get
    /// `SystemResourceIncidentNotif` (`FUN_00512670`). Our overcap trigger
    /// has no source (F-029).
    pub resource: bool,
}

/// Fleet posture summary for a system (`FUN_0050bc60`/`FUN_0050be00`/`FUN_0050c350`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FleetPosture {
    /// Number of Alliance capital ship hulls at this system.
    pub alliance_capships: u32,
    /// Number of Empire capital ship hulls at this system.
    pub empire_capships: u32,
    /// True if both sides have two or more usable fleets present. port: a
    /// telemetry flag only (the economy record); no recovered function
    /// reads a fleet-contested state, and the threshold of two per side is
    /// the port's.
    pub is_contested: bool,
}

/// Fighter posture for a system (`FUN_0050b8e0`).
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
    /// `FUN_0050b500`.
    pub troop_surplus: i32,
    /// Total troops for the controlling faction.
    /// `FUN_0050ba90`.
    pub total_controlling_troops: u32,
    /// Whether an orbital shipyard is present.
    /// `FUN_0050bb70`.
    pub has_shipyard: bool,
    /// Fleet posture (3-pass result).
    pub fleet_posture: FleetPosture,
    /// Fighter posture.
    pub fighter_posture: FighterPosture,
}

/// Who holds a system by support alone (`FUN_0055a080`, threshold
/// `DAT_006bb4c0` = GNPRTB 7760): the Alliance when its support reaches the
/// threshold, the Empire when its own does, else neither.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportHolder {
    /// `FUN_0055a080` returns 1.
    Alliance,
    /// `FUN_0055a080` returns 2.
    Empire,
    /// `FUN_0055a080` returns 3.
    #[default]
    Neither,
}

/// `FUN_0055a080` on the system's Alliance support (`+0x58`, 0-100).
#[expect(
    clippy::cast_possible_truncation,
    reason = "Support is a fraction of 1 and rounds to a whole percent."
)]
fn support_holder(
    sys: &crate::world::System,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> SupportHolder {
    let threshold = gnprtb.value(GNPRTB_SUPPORT_CONTROL_THRESHOLD, difficulty);
    let support = (sys.popularity_alliance * 100.0).round() as i32;
    if support >= threshold {
        SupportHolder::Alliance
    } else if 100 - support >= threshold {
        SupportHolder::Empire
    } else {
        SupportHolder::Neither
    }
}

/// Per-system resource and economy tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "These independent flags preserve the existing state and serialization model."
)]
pub struct SystemEconomy {
    /// Production speed modifier from fleet/KDY presence (0-100).
    pub production_modifier: i8,
    /// Current energy output (sum of production facility outputs, capped at system capacity).
    /// `FUN_0050ad60`: if `energy_allocated` > `System.total_energy`, cap it.
    pub energy_allocated: u32,
    /// Current raw material output (sum of mine outputs, capped at system capacity).
    /// `FUN_0050b0b0`: if `raw_material_allocated` > `System.raw_materials`, cap it.
    pub raw_material_allocated: u32,
    /// True if facilities exceed energy capacity (facility pruning needed).
    pub energy_overcapped: bool,
    /// True if mines exceed raw material capacity (mine pruning needed).
    pub raw_material_overcapped: bool,
    /// Derived troop/fleet/shipyard summary (functions 9-15).
    pub summary: SystemSummary,
    /// Incident state flags (original system `+0x88` bits 16-19).
    /// When one turns on from the previous tick, its incident message fires.
    pub incident_flags: IncidentFlags,
    /// System `+0x88` bit 9 (`FUN_0050bb00`): held, populated, in revolt, and
    /// not short of troops.
    pub uprising_visible: bool,
    /// The previous tick's holder by support (`FUN_0055a080`), for the
    /// `SupportChanged` record.
    pub last_support_holder: SupportHolder,
    /// True once the controlling faction has established this system in
    /// a prior tick. Suppresses `EVT_RESOURCE_DISCOVERY` (0x155) on the
    /// very first eval — the galaxy seed itself is not a "discovery".
    pub resource_discovery_armed: bool,
}

impl Default for SystemEconomy {
    fn default() -> Self {
        Self {
            production_modifier: 0,
            energy_allocated: 0,
            raw_material_allocated: 0,
            energy_overcapped: false,
            raw_material_overcapped: false,
            summary: SystemSummary::default(),
            incident_flags: IncidentFlags::default(),
            uprising_visible: false,
            last_support_holder: SupportHolder::default(),
            resource_discovery_armed: false,
        }
    }
}

/// What the support hook's three writers store on a system (`FUN_005109f0`):
/// the garrison requirement (`+0x80`, `FUN_0050b5a0`), the support drift
/// (`+0x6c`, `FUN_0050b230`) and the collection rate (`+0x70`,
/// `FUN_0050b2c0`). Kept per system in `GameWorld::support_values`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SupportValues {
    /// Resource collection rate, inversely proportional to support.
    pub collection_rate: f32,
    /// Number of troops required to prevent uprising.
    pub garrison_requirement: u32,
    /// The support drift (`FUN_00509ec0` from `FUN_0050b230` ->
    /// `FUN_00559c40`), positive while the Alliance holds the system and
    /// negative for the Empire. Its magnitude is the percent chance that a
    /// finished mine's or refinery's unit goes to the other side
    /// (`FUN_005166a0`; `crate::stockpiles`). Nothing adds it to support.
    pub support_drift: i8,
}

impl Default for SupportValues {
    fn default() -> Self {
        Self {
            collection_rate: 1.0,
            garrison_requirement: 0,
            support_drift: 0,
        }
    }
}

/// `FUN_0050b5a0`, `FUN_0050b230` and `FUN_0050b2c0` for `system` at
/// `difficulty`: its garrison requirement (with `+0x88` bit 11, the Death
/// Star, and bit 2, `GameWorld::revolting`), drift and collection rate.
fn support_values(world: &GameWorld, system: SystemKey, difficulty: u8) -> Option<SupportValues> {
    let sys = world.systems.get(system)?;
    let gnprtb = &world.gnprtb;
    let presence = count_military_presence(world, sys);
    let death_star = world.death_star_in_sector(sys);
    let (alliance_delta, empire_delta) =
        calculate_support_drift(sys, &presence, gnprtb, difficulty, death_star);
    let controlling_support = match sys.control {
        ControlKind::Controlled(crate::dat::Faction::Alliance) => sys.popularity_alliance,
        ControlKind::Controlled(crate::dat::Faction::Empire) => sys.popularity_empire,
        _ => 0.5, // Neutral: baseline
    };
    Some(SupportValues {
        collection_rate: calculate_collection_rate(controlling_support, gnprtb, difficulty),
        garrison_requirement: calculate_garrison_requirement(
            controlling_support,
            sys.control,
            death_star,
            world.revolting.contains(&system),
            gnprtb,
            difficulty,
        ),
        support_drift: support_drift_value(sys.control, alliance_delta, empire_delta),
    })
}

/// `FUN_0050b5a0`, `FUN_0050b230` and `FUN_0050b2c0` for `system`, as the
/// support hook runs them (`FUN_005109f0.c:14-29`): store its garrison
/// requirement, drift and collection rate in `GameWorld::support_values`.
pub fn refresh_support_values(world: &mut GameWorld, system: SystemKey) {
    if let Some(values) = support_values(world, system, world.difficulty_index) {
        world.support_values.insert(system, values);
    }
}

/// `system`'s stored [`SupportValues`], the defaults before any write.
#[must_use]
pub fn stored_support_values(world: &GameWorld, system: SystemKey) -> SupportValues {
    world
        .support_values
        .get(&system)
        .copied()
        .unwrap_or_default()
}

/// Economy state across all systems.
///
/// Simulation state (not world state). Saved via `SaveState::economy`
/// since Dabora 1 (#F2) to close the incident re-fire bug.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EconomyState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub per_system: HashMap<SystemKey, SystemEconomy>,
}

// ---------------------------------------------------------------------------
// Economy events (system output)
// ---------------------------------------------------------------------------

/// Events produced by the economy tick.
#[derive(Debug, Clone)]
pub enum EconomyEvent {
    /// Collection rate recalculated.
    CollectionRateChanged { system: SystemKey, new_rate: f32 },
    /// Garrison requirement recalculated.
    GarrisonRequirementChanged {
        system: SystemKey,
        new_requirement: u32,
    },
    /// Incident state changed — fire the corresponding notification (`FUN_0050b800` + `FUN_0050e5b0`).
    IncidentTriggered {
        system: SystemKey,
        incident_type: &'static str,
    },
    /// Energy allocated exceeds system capacity (`FUN_0050ad60`).
    EnergyOvercapped {
        system: SystemKey,
        allocated: u32,
        capacity: u32,
    },
    /// Raw material output exceeds system capacity (`FUN_0050b0b0`).
    RawMaterialOvercapped {
        system: SystemKey,
        allocated: u32,
        capacity: u32,
    },
    // ── Knesset Shamash-Bet notification events (Dabora 2) ──────────────
    /// `EVT_SUPPORT_CHANGE` (0x100): the holder by support changed
    /// (`FUN_0055a080`). port: the original raises no event here; its
    /// support-change hook reruns control (`FUN_005109f0`). Telemetry only.
    SupportChanged {
        system: SystemKey,
        from: SupportHolder,
        to: SupportHolder,
    },
    /// K3: `EVT_RESOURCE_DISCOVERY` (0x155) — a new mine came online at
    /// a previously-seeded system. Detected as a positive delta on
    /// `eco.raw_material_allocated` (after the `resource_discovery_armed`
    /// gate disarms the seed tick).
    ResourceDiscovered { system: SystemKey, new_output: u32 },
}

// ---------------------------------------------------------------------------
// Economy system
// ---------------------------------------------------------------------------

pub struct EconomySystem;

impl EconomySystem {
    /// Run the per-system economy tick. Mutates `EconomyState` in-place,
    /// writes each system's [`SupportValues`] (`GameWorld::support_values`)
    /// and returns events for the integrator. `GameWorld::revolting` is
    /// system `+0x88` bit 2, which doubles the garrison requirement
    /// (`FUN_0050b5a0`).
    ///
    /// port: the original recomputes the [`SupportValues`] only from its
    /// hooks (setup, support, holder, units, `FUN_00511740`); the pass also
    /// does, once a day.
    ///
    /// The difficulty is the world's (`GameWorld::difficulty_index`), the
    /// column the support hook reads too.
    ///
    /// Runs BEFORE manufacturing in the tick order (economy affects production).
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    pub fn advance(
        state: &mut EconomyState,
        world: &mut GameWorld,
        tick_events: &[TickEvent],
    ) -> Vec<EconomyEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }
        let difficulty = world.difficulty_index;

        let gnprtb = &world.gnprtb;
        let mut events = Vec::new();
        let mut written = Vec::new();

        // Collect system keys first to avoid borrow issues.
        let system_keys: Vec<SystemKey> = world.systems.keys().collect();

        for sys_key in system_keys {
            let Some(sys) = world.systems.get(sys_key) else {
                continue;
            };

            // Only process populated, non-destroyed systems.
            if !sys.is_populated || sys.is_destroyed {
                continue;
            }

            // Count military presence at this system.
            let presence = count_military_presence(world, sys);

            // 0a. Resource capacity enforcement (FUN_0050ad60 + FUN_0050ad80 + FUN_0050b0b0).
            // Sum facility/mine outputs and cap at system limits.
            let eco = state.per_system.entry(sys_key).or_default();
            let prev_raw_allocated = eco.raw_material_allocated;
            let (energy_alloc, raw_alloc) = calculate_resource_allocation(world, sys);
            let energy_cap = u32::from(sys.total_energy);
            let raw_cap = u32::from(sys.raw_materials);

            eco.energy_allocated = energy_alloc.min(energy_cap);
            eco.energy_overcapped = energy_alloc > energy_cap;
            eco.raw_material_allocated = raw_alloc.min(raw_cap);
            eco.raw_material_overcapped = raw_alloc > raw_cap;

            // K3: EVT_RESOURCE_DISCOVERY (0x155).
            // Fire when a new mine comes online (capped raw material output
            // strictly increases). Suppress on the first armed pass — the
            // galaxy seed itself is not a "discovery".
            let new_raw = eco.raw_material_allocated;
            if eco.resource_discovery_armed && new_raw > prev_raw_allocated {
                events.push(EconomyEvent::ResourceDiscovered {
                    system: sys_key,
                    new_output: new_raw,
                });
            }
            eco.resource_discovery_armed = true;

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

            // 1-3. The drift (FUN_0050b230 -> FUN_00559c40, system +0x6c,
            // read by the diversion roll alone: FUN_00509ec0, FUN_005166a0),
            // the collection rate and the garrison requirement; support
            // itself moves only through its setter (FUN_00509c30).
            let old = stored_support_values(world, sys_key);
            let values = support_values(world, sys_key, difficulty).unwrap_or_default();
            let (new_rate, new_garrison) = (values.collection_rate, values.garrison_requirement);

            // 4. Blockade withdraw percent (FUN_0050b310 + FUN_0055a020).
            // Original formula: 100 unless the system is blockaded (+0x88 bit5)
            // with no KDY-150, else max(0, 100 - ships*GNPRTB[7684] - fighters*GNPRTB[7685]).
            // Our production modifier use has no recovered source (audit finding F-023).
            let capship_penalty = gnprtb.value(GNPRTB_KDY_CAPSHIP_PENALTY, difficulty);
            let fighter_penalty = gnprtb.value(GNPRTB_KDY_FIGHTER_PENALTY, difficulty);
            let total_capships =
                presence.alliance_capships.cast_signed() + presence.empire_capships.cast_signed();
            let total_fighters =
                presence.alliance_fighters.cast_signed() + presence.empire_fighters.cast_signed();
            let new_prod_mod =
                (100 - total_capships * capship_penalty - total_fighters * fighter_penalty)
                    .clamp(0, 100) as i8;

            // 5. The holder (FUN_0050b610) is not the economy pass's: its
            // hooks rebuild it as support, regiments or the populated bit
            // change (`support_shift::rebuild_control`).

            // Update per-system economy state (eco already bound above in step 0a).
            let (old_rate, old_garrison) = (old.collection_rate, old.garrison_requirement);
            written.push((sys_key, values));
            eco.production_modifier = new_prod_mod;

            // 6. Troop/fleet summary propagation (FUN_0050b500 through FUN_0050b8e0).
            eco.summary = compute_system_summary(world, sys, &presence, new_garrison);

            // 7. Incident state + uprising visibility (FUN_0050bb00).
            // The uprising and disaster incidents are timer events that
            // `UprisingSystem` raises (F-026). The informant and resource
            // triggers below have no recovered source (F-029).
            let new_flags = evaluate_incident_flags(&eco.summary, eco);
            let old_flags = &eco.incident_flags;
            if new_flags.informant && !old_flags.informant {
                events.push(EconomyEvent::IncidentTriggered {
                    system: sys_key,
                    incident_type: "informant",
                });
            }
            if new_flags.resource && !old_flags.resource {
                // NOTE: this is the legacy "grid overcap" incident, not K3.
                // K3 (EVT_RESOURCE_DISCOVERY) fires above on positive
                // raw_material_allocated deltas.
                events.push(EconomyEvent::IncidentTriggered {
                    system: sys_key,
                    incident_type: "resource",
                });
            }
            eco.incident_flags = new_flags;

            // FUN_0050bb00 sets system +0x88 bit 9 for a populated, held
            // system in revolt whose troop surplus is not negative.
            eco.uprising_visible = sys.control.faction().is_some()
                && world.revolting.contains(&sys_key)
                && eco.summary.troop_surplus >= 0;

            // EVT_SUPPORT_CHANGE (0x100): the holder by support changed.
            if sys.is_populated {
                let holder = support_holder(sys, gnprtb, difficulty);
                if holder != eco.last_support_holder {
                    events.push(EconomyEvent::SupportChanged {
                        system: sys_key,
                        from: eco.last_support_holder,
                        to: holder,
                    });
                    eco.last_support_holder = holder;
                }
            }

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
        world.support_values.extend(written);

        events
    }
}

// ---------------------------------------------------------------------------
// Military presence counting
// ---------------------------------------------------------------------------

/// Military presence at a system, used by support drift and KDY calculations.
#[derive(Default)]
struct MilitaryPresence {
    /// The usable fleets at the system, the same mode-1 walk as the ships
    /// (`FUN_0050b310.c:32-45`, `FUN_00509090.c:23`): a fleet holding only
    /// unfinished ships or ships on their own legs is not counted. port:
    /// only the fleet posture's telemetry flag reads it.
    alliance_fleets: u32,
    empire_fleets: u32,
    /// The squadrons aboard the usable ships of the usable fleets and the
    /// system's own (`FUN_0050b310.c:45,58`).
    alliance_fighters: u32,
    empire_fighters: u32,
    alliance_troops: u32,
    empire_troops: u32,
    /// The usable capital ships of the usable fleets (`FUN_0050b310.c:38`;
    /// `FUN_00509090`, the drift's ship term). Production modifier use has
    /// no recovered source.
    alliance_capships: u32,
    empire_capships: u32,
    /// The system's own squadrons, mode 1 (`FUN_00508fb0` ->
    /// `FUN_00503a50`, `[0x1c,0x20)`): the drift's squadron term.
    alliance_system_fighters: u32,
    empire_system_fighters: u32,
    /// The usable Death Stars of the usable fleets (`FUN_00509140` ->
    /// `FUN_00500450`, `[0x18,0x1c)`, mode 1): the drift's gate.
    alliance_death_stars: u32,
    empire_death_stars: u32,
}

fn count_military_presence(world: &GameWorld, sys: &crate::world::System) -> MilitaryPresence {
    let mut presence = MilitaryPresence::default();

    // FUN_0050b310.c:32-45 and FUN_00509090.c:23,30 walk the usable fleets
    // (mode 1), their usable ships and each usable ship's squadrons
    // (FUN_005039d0(ship, 1)); a ship on its own leg or unfinished is not
    // usable (0x004f7b80). port: a squadron with no carrier rides the
    // fleet itself.
    for &fleet_key in &sys.fleets {
        if let Some(fleet) = world.fleets.get(fleet_key).filter(|fleet| fleet.usable()) {
            let usable = |tag: u32| {
                fleet
                    .capital_ships
                    .iter()
                    .any(|ship| ship.usable() && ship.tag == tag)
            };
            let capship_count =
                u32::try_from(fleet.capital_ships.iter().filter(|ship| ship.usable()).count())
                    .unwrap_or(u32::MAX);
            let fighter_count: u32 = fleet
                .fighters
                .iter()
                .filter(|entry| entry.carrier == 0 || usable(entry.carrier))
                .map(|f| f.count)
                .sum();
            // FUN_00509140 -> FUN_00500450: the usable children of family
            // [0x18,0x1c).
            let death_stars = u32::try_from(
                fleet
                    .capital_ships
                    .iter()
                    .filter(|ship| {
                        ship.usable()
                            && world
                                .capital_ship_classes
                                .get(ship.class)
                                .is_some_and(crate::world::CapitalShipClass::is_death_star)
                    })
                    .count(),
            )
            .unwrap_or(u32::MAX);
            if fleet.is_alliance {
                presence.alliance_fleets += 1;
                presence.alliance_capships += capship_count;
                presence.alliance_fighters += fighter_count;
                presence.alliance_death_stars += death_stars;
            } else {
                presence.empire_fleets += 1;
                presence.empire_capships += capship_count;
                presence.empire_fighters += fighter_count;
                presence.empire_death_stars += death_stars;
            }
        }
    }

    // The squadrons the system holds count with those aboard its fleets
    // (FUN_0050b310.c:58) and alone for the drift (FUN_00508fb0).
    presence.alliance_system_fighters = sys.squadron_count(true);
    presence.empire_system_fighters = sys.squadron_count(false);
    presence.alliance_fighters += presence.alliance_system_fighters;
    presence.empire_fighters += presence.empire_system_fighters;

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
// Popular support drift (FUN_00559c40)
// ---------------------------------------------------------------------------

/// The drift as the original stores it at system `+0x6c`: the integer
/// percentage, negated for side 2 (`FUN_00559c40`). It is 0 at an unheld
/// system.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a drift is at most 100 percent"
)]
fn support_drift_value(control: ControlKind, alliance_delta: f32, empire_delta: f32) -> i8 {
    let percent = |delta: f32| (delta * 100.0).round() as i8;
    match control {
        ControlKind::Controlled(crate::dat::Faction::Alliance) => percent(-alliance_delta),
        ControlKind::Controlled(crate::dat::Faction::Empire) => percent(empire_delta),
        _ => 0,
    }
}

/// Calculate support drift for a system based on military presence.
///
/// `FUN_0050b230` passes `FUN_00559c40` the holder's counts at the system,
/// each in mode 1: p4 its capital ships in its fleets (`FUN_00509090`), p5
/// the system's own squadrons (`FUN_00508fb0` -> `FUN_00503a50`), p6 its
/// regiments (`FUN_00509020`) and p7 its Death Stars in its fleets
/// (`FUN_00509140` -> `FUN_00500450`, `[0x18,0x1c)`):
/// ```text
/// if support <= 7732 AND p7 == 0:
///     if support <= 7736: base = 7737
///     elif support <= 7734: base = 7735
///     else: base = 7733
///     drift = clamp(base - p4*7686 - p5*7687 - regiments(p6)*7688, 0, 100)
/// if empire_controlled: drift = -drift
/// ```
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
fn calculate_support_drift(
    sys: &crate::world::System,
    presence: &MilitaryPresence,
    gnprtb: &GnprtbParams,
    difficulty: u8,
    death_star_in_sector: bool,
) -> (f32, f32) {
    // All computation in integer 0-100 to match original (FUN_00559c40).
    // Our popularity fields are f32 0.0-1.0; convert at boundary.
    let ship_influence = gnprtb.value(GNPRTB_SHIP_INFLUENCE, difficulty).max(1);
    let fighter_influence = gnprtb.value(GNPRTB_FIGHTER_INFLUENCE, difficulty).max(1);
    let troop_influence = gnprtb.value(GNPRTB_TROOP_INFLUENCE, difficulty).max(1);
    let drift_threshold = gnprtb.value(GNPRTB_DRIFT_THRESHOLD, difficulty); // 40
    let threshold_1 = gnprtb.value(GNPRTB_DRIFT_THRESHOLD_1, difficulty); // 20
    let threshold_2 = gnprtb.value(GNPRTB_DRIFT_THRESHOLD_2, difficulty); // 30
    let base_low = gnprtb.value(GNPRTB_DRIFT_BASE_LOW, difficulty); // 75
    let base_low_mid = gnprtb.value(GNPRTB_DRIFT_BASE_LOW_MID, difficulty); // 50
    let base_mid = gnprtb.value(GNPRTB_DRIFT_BASE_MID, difficulty); // 25

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

    let (controlling_support_f32, friendly_ships, friendly_fighters, friendly_troops, death_stars) =
        if is_alliance_controlled {
            (
                sys.popularity_alliance,
                presence.alliance_capships,
                presence.alliance_system_fighters,
                presence.alliance_troops,
                presence.alliance_death_stars,
            )
        } else {
            (
                sys.popularity_empire,
                presence.empire_capships,
                presence.empire_system_fighters,
                presence.empire_troops,
                presence.empire_death_stars,
            )
        };

    // Convert f32 0.0-1.0 to integer 0-100 for computation
    let support = (controlling_support_f32 * 100.0).round() as i32;

    // Only drift at or below GNPRTB 7732 with no friendly usable Death
    // Star (FUN_00559c40: p2 <= DAT_006bb478 && p7 == 0).
    if support > drift_threshold || death_stars > 0 {
        return (0.0, 0.0);
    }

    // The base by support bracket (FUN_00559c40): at most GNPRTB 7736 takes
    // 7737, at most 7734 takes 7735, and the rest up to 7732 take 7733.
    let base = if support <= threshold_1 {
        base_low // ≤20: strong drift (75)
    } else if support <= threshold_2 {
        base_low_mid // 21-30: moderate drift (50)
    } else {
        base_mid // 31-40: mild drift (25)
    };

    // FUN_00559b60: for side 2 with system +0x88 bit 11 set (a usable Death
    // Star in the sector), the regiment count is multiplied by GNPRTB 7680 (2).
    let adjusted_troops = if is_empire_controlled && death_star_in_sector {
        let empire_mult = gnprtb.value(GNPRTB_EMPIRE_TROOP_MULT, difficulty).max(1);
        friendly_troops.cast_signed() * empire_mult
    } else {
        friendly_troops.cast_signed()
    };

    // Military suppression: integer subtraction matching original exactly.
    // Original: clamp(base - ships*GNPRTB[7686] - squadrons*GNPRTB[7687] - troops_adj*GNPRTB[7688], 0, 100)
    let suppression = friendly_fighters.cast_signed() * fighter_influence
        + adjusted_troops * troop_influence
        + friendly_ships.cast_signed() * ship_influence;

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
// Resource capacity (FUN_0050ad60 + FUN_0050ad80 + FUN_0050b0b0)
// ---------------------------------------------------------------------------

/// Calculate total energy and raw material allocation from facilities and mines.
///
/// Returns (`energy_allocated`, `raw_material_allocated`).
/// The original sums facility outputs (virtual method at +0x1c8) and mine outputs,
/// then caps at system limits. If overcapped, the original randomly prunes facilities/mines.
/// We report overcap via events and let the caller decide on pruning.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
pub fn calculate_resource_allocation(world: &GameWorld, sys: &crate::world::System) -> (u32, u32) {
    // Sum production facility outputs (energy generators).
    // Each production facility contributes 1 unit of energy output.
    let energy_output = sys.production_facilities.len() as u32;

    // Sum mine outputs (raw material generators).
    // Mines are production facilities with is_mine = true.
    let raw_material_output = sys
        .production_facilities
        .iter()
        .filter(|k| {
            world
                .production_facilities
                .get(**k)
                .is_some_and(|f| f.is_mine)
        })
        .count() as u32;

    (energy_output, raw_material_output)
}

// ---------------------------------------------------------------------------
// Troop-based side resolution (FUN_0050b610)
// ---------------------------------------------------------------------------

/// Resolve which faction controls a system (`FUN_0050b610`): the side
/// whose regiments alone stand there (`.c:34-63`), populated or not; with
/// both sides' regiments the troop bits keep their old value (`.c:45-61`),
/// so a system whose earlier word had them (`troop_held`) keeps its holder;
/// otherwise the side whose support reaches GNPRTB 7760 (`FUN_0055a080`),
/// else neutral, and neutral for an unpopulated system (`.c:24-33`,
/// `:64-70`).
fn resolve_system_control(
    sys: &crate::world::System,
    presence: &MilitaryPresence,
    troop_held: bool,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> ControlKind {
    let has_alliance = presence.alliance_troops > 0;
    let has_empire = presence.empire_troops > 0;

    match (has_alliance, has_empire) {
        (true, false) => ControlKind::Controlled(crate::dat::Faction::Alliance),
        (false, true) => ControlKind::Controlled(crate::dat::Faction::Empire),
        (true, true) if troop_held => sys.control,
        _ if !sys.is_populated => ControlKind::Uncontrolled,
        _ => match support_holder(sys, gnprtb, difficulty) {
            SupportHolder::Alliance => ControlKind::Controlled(crate::dat::Faction::Alliance),
            SupportHolder::Empire => ControlKind::Controlled(crate::dat::Faction::Empire),
            SupportHolder::Neither => ControlKind::Uncontrolled,
        },
    }
}

/// [`resolve_system_control`] for `system` as it stands: the holder
/// `FUN_0050b610` rebuilds when a regiment there dies or lands (system hook
/// `FUN_00508660` on events `0x301` and `0x320`), as after an assault. The
/// troop bits read are the system's current ones (`GameWorld::troop_held`),
/// the word before the rebuild.
#[must_use]
pub fn system_control(world: &GameWorld, system: SystemKey) -> Option<ControlKind> {
    let sys = world.systems.get(system)?;
    let presence = count_military_presence(world, sys);
    Some(resolve_system_control(
        sys,
        &presence,
        world.troop_held.contains(&system),
        &world.gnprtb,
        world.difficulty_index,
    ))
}

// ---------------------------------------------------------------------------
// Troop/fleet summary propagation (FUN_0050b500 through FUN_0050b8e0)
// ---------------------------------------------------------------------------

/// Compute the per-system derived summary from troop/fleet/facility presence.
/// Implements functions 9-15 of the economy tick pipeline.
fn compute_system_summary(
    world: &GameWorld,
    sys: &crate::world::System,
    presence: &MilitaryPresence,
    garrison_requirement: u32,
) -> SystemSummary {
    let controlling_troops = match sys.control {
        ControlKind::Controlled(crate::dat::Faction::Alliance) => presence.alliance_troops,
        ControlKind::Controlled(crate::dat::Faction::Empire) => presence.empire_troops,
        _ => presence.alliance_troops.max(presence.empire_troops),
    };
    // FUN_0050b500: a held system's surplus is every regiment at it
    // (FUN_00504c40, no side filter) minus the requirement; otherwise 0.
    let troop_surplus = match sys.control {
        ControlKind::Controlled(crate::dat::Faction::Alliance | crate::dat::Faction::Empire) => {
            (presence.alliance_troops + presence.empire_troops).cast_signed()
                - garrison_requirement.cast_signed()
        }
        _ => 0,
    };

    // FUN_0050ba90: total controlling troops
    let total_controlling_troops = controlling_troops;

    // FUN_0050bb70: shipyard presence (check manufacturing facilities for shipyard type).
    let has_shipyard = sys.manufacturing_facilities.iter().any(|k| {
        world
            .manufacturing_facilities
            .get(*k)
            .is_some_and(|f| f.is_shipyard)
    });

    // FUN_0050bc60/FUN_0050be00/FUN_0050c350: fleet posture (3 passes).
    // port: is_contested counts usable fleets (the presence walk above) and
    // its threshold of two per side has no source; telemetry alone reads
    // it.
    let fleet_posture = FleetPosture {
        alliance_capships: presence.alliance_capships,
        empire_capships: presence.empire_capships,
        is_contested: presence.alliance_fleets >= 2 && presence.empire_fleets >= 2,
    };

    // FUN_0050b8e0: fighter posture
    let fighter_posture = FighterPosture {
        is_contested: presence.alliance_fighters > 0 && presence.empire_fighters > 0,
        alliance_fighters: presence.alliance_fighters,
        empire_fighters: presence.empire_fighters,
    };

    SystemSummary {
        troop_surplus,
        total_controlling_troops,
        has_shipyard,
        fleet_posture,
        fighter_posture,
    }
}

// ---------------------------------------------------------------------------
// Incident evaluation
// ---------------------------------------------------------------------------

/// Evaluate the informant and resource incident flags for a system.
///
/// Neither trigger has a recovered source (audit finding F-029): the original
/// raises the informant incident from a per-system timer (event `0x38e`,
/// `FUN_0050cbe0`) and the resource incident from a galaxy timer (event
/// `0x390`, `FUN_00556be0`). The uprising and disaster incidents are raised by
/// `UprisingSystem`, so their flags stay clear.
fn evaluate_incident_flags(summary: &SystemSummary, eco: &SystemEconomy) -> IncidentFlags {
    IncidentFlags {
        uprising: false,
        informant: summary.troop_surplus < 0,
        disaster: false,
        resource: eco.energy_overcapped || eco.raw_material_overcapped,
    }
}

/// Calculate resource collection rate from popular support.
///
/// Formula: `(GNPRTB[7763] * 100) / max(support_pct, 1)`.
/// Higher support means a lower collection rate (less taxation needed).
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
fn calculate_collection_rate(support: f32, gnprtb: &GnprtbParams, difficulty: u8) -> f32 {
    // Original: FUN_0053e170(GNPRTB[7763], 100, support)
    //         = (100 * GNPRTB[7763]) / max(support_int, 1)
    //         = 10000 / support_int  (with stock GNPRTB[7763]=100)
    // Note: FUN_0053e190(a,b) = FUN_0053e170(a,b,100) = a*b/100.
    // Result: integer percentage (100 at full support, 10000 at near-zero).
    // Higher values = higher taxation burden on the system.
    let base = gnprtb.value(GNPRTB_COLLECTION_RATE_BASE, difficulty).max(1);
    let support_int = (support * 100.0).round() as i32;
    let support_clamped = support_int.max(1);
    let rate = (100 * base) / support_clamped;
    rate.max(1) as f32
}

// ---------------------------------------------------------------------------
// Garrison requirement (FUN_00559fe0 + FUN_0055a050)
// ---------------------------------------------------------------------------

/// Calculate troops needed to prevent uprising at this system.
///
/// Formula: `(threshold - support_pct) / abs(divisor)` when support < threshold.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
fn calculate_garrison_requirement(
    support: f32,
    control: ControlKind,
    death_star_in_sector: bool,
    in_uprising: bool,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> u32 {
    // FUN_0050b5a0 stores FUN_00559fe0(side, support, +0x88 bit 11,
    // +0x88 bit 2, 1) for an Alliance- or Empire-held system and 0 otherwise.
    let Some(side) = control.faction() else {
        return 0;
    };
    // Our support is f32 0.0-1.0; convert to integer 0-100.
    let support_int = (support * 100.0).round() as i32;
    garrison_requirement(
        side,
        support_int,
        death_star_in_sector,
        in_uprising,
        gnprtb,
        difficulty,
    )
    .max(0) as u32
}

/// Troops a side must hold at a system (`FUN_00559fe0`): the uprising
/// threshold, divided by GNPRTB 7680 for the Empire with a Death Star in the
/// sector (+0x88 bit 11), and multiplied by GNPRTB 7682 during an uprising.
pub(crate) fn garrison_requirement(
    side: crate::dat::Faction,
    support: i32,
    death_star_in_sector: bool,
    in_uprising: bool,
    gnprtb: &GnprtbParams,
    difficulty: u8,
) -> i32 {
    let mut requirement = uprising_threshold(support, gnprtb, difficulty);
    if death_star_in_sector && side == crate::dat::Faction::Empire {
        requirement /= gnprtb.value(GNPRTB_EMPIRE_TROOP_MULT, difficulty).max(1);
    }
    if in_uprising {
        requirement *= gnprtb.value(GNPRTB_UPRISING_GARRISON_MULT, difficulty);
    }
    requirement
}

/// `FUN_0055a050`: `ceil((GNPRTB 7761 - support) / -GNPRTB 7762)` below the
/// GNPRTB 7761 threshold (60 and -10 shipped), else 0. `FUN_0053e160(a, b)`
/// is `(b - 1 + a) / b`.
pub(crate) fn uprising_threshold(support: i32, gnprtb: &GnprtbParams, difficulty: u8) -> i32 {
    let threshold = gnprtb.value(GNPRTB_GARRISON_THRESHOLD, difficulty);
    if support >= threshold {
        return 0;
    }
    let divisor = (-gnprtb.value(GNPRTB_GARRISON_DIVISOR, difficulty)).max(1);
    (divisor - 1 + threshold - support) / divisor
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::DatId;
    use crate::ids::SectorKey;
    use crate::world::GameWorld;

    /// Create a `GnprtbParams` with the stock economy values.
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    fn stock_gnprtb() -> GnprtbParams {
        use crate::world::GnprtbEntry;
        let entries = vec![
            GnprtbEntry {
                parameter_id: 7686,
                development: 10,
                alliance_sp_easy: 10,
                alliance_sp_medium: 10,
                alliance_sp_hard: 10,
                empire_sp_easy: 10,
                empire_sp_medium: 10,
                empire_sp_hard: 10,
                multiplayer: 10,
            },
            GnprtbEntry {
                parameter_id: 7687,
                development: 5,
                alliance_sp_easy: 5,
                alliance_sp_medium: 5,
                alliance_sp_hard: 5,
                empire_sp_easy: 5,
                empire_sp_medium: 5,
                empire_sp_hard: 5,
                multiplayer: 5,
            },
            GnprtbEntry {
                parameter_id: 7688,
                development: 2,
                alliance_sp_easy: 2,
                alliance_sp_medium: 2,
                alliance_sp_hard: 2,
                empire_sp_easy: 2,
                empire_sp_medium: 2,
                empire_sp_hard: 2,
                multiplayer: 2,
            },
            GnprtbEntry {
                parameter_id: 7732,
                development: 40,
                alliance_sp_easy: 40,
                alliance_sp_medium: 40,
                alliance_sp_hard: 40,
                empire_sp_easy: 40,
                empire_sp_medium: 40,
                empire_sp_hard: 40,
                multiplayer: 40,
            },
            GnprtbEntry {
                parameter_id: 7733,
                development: 25,
                alliance_sp_easy: 25,
                alliance_sp_medium: 25,
                alliance_sp_hard: 25,
                empire_sp_easy: 25,
                empire_sp_medium: 25,
                empire_sp_hard: 25,
                multiplayer: 25,
            },
            GnprtbEntry {
                parameter_id: 7734,
                development: 30,
                alliance_sp_easy: 30,
                alliance_sp_medium: 30,
                alliance_sp_hard: 30,
                empire_sp_easy: 30,
                empire_sp_medium: 30,
                empire_sp_hard: 30,
                multiplayer: 30,
            },
            GnprtbEntry {
                parameter_id: 7735,
                development: 50,
                alliance_sp_easy: 50,
                alliance_sp_medium: 50,
                alliance_sp_hard: 50,
                empire_sp_easy: 50,
                empire_sp_medium: 50,
                empire_sp_hard: 50,
                multiplayer: 50,
            },
            GnprtbEntry {
                parameter_id: 7736,
                development: 20,
                alliance_sp_easy: 20,
                alliance_sp_medium: 20,
                alliance_sp_hard: 20,
                empire_sp_easy: 20,
                empire_sp_medium: 20,
                empire_sp_hard: 20,
                multiplayer: 20,
            },
            GnprtbEntry {
                parameter_id: 7737,
                development: 75,
                alliance_sp_easy: 75,
                alliance_sp_medium: 75,
                alliance_sp_hard: 75,
                empire_sp_easy: 75,
                empire_sp_medium: 75,
                empire_sp_hard: 75,
                multiplayer: 75,
            },
            GnprtbEntry {
                parameter_id: 7761,
                development: 60,
                alliance_sp_easy: 60,
                alliance_sp_medium: 60,
                alliance_sp_hard: 60,
                empire_sp_easy: 60,
                empire_sp_medium: 60,
                empire_sp_hard: 60,
                multiplayer: 60,
            },
            GnprtbEntry {
                parameter_id: 7762,
                development: -10,
                alliance_sp_easy: -10,
                alliance_sp_medium: -10,
                alliance_sp_hard: -10,
                empire_sp_easy: -10,
                empire_sp_medium: -10,
                empire_sp_hard: -10,
                multiplayer: -10,
            },
            GnprtbEntry {
                parameter_id: 7763,
                development: 100,
                alliance_sp_easy: 100,
                alliance_sp_medium: 100,
                alliance_sp_hard: 100,
                empire_sp_easy: 100,
                empire_sp_medium: 100,
                empire_sp_hard: 100,
                multiplayer: 100,
            },
            // Phase 3b additions: all economy tick GNPRTB indices (values from GNPRTB.DAT)
            GnprtbEntry {
                parameter_id: 7680,
                development: 2,
                alliance_sp_easy: 2,
                alliance_sp_medium: 2,
                alliance_sp_hard: 2,
                empire_sp_easy: 2,
                empire_sp_medium: 2,
                empire_sp_hard: 2,
                multiplayer: 2,
            }, // Empire troop doubling + garrison halving
            GnprtbEntry {
                parameter_id: 7682,
                development: 2,
                alliance_sp_easy: 2,
                alliance_sp_medium: 2,
                alliance_sp_hard: 2,
                empire_sp_easy: 2,
                empire_sp_medium: 2,
                empire_sp_hard: 2,
                multiplayer: 2,
            }, // Uprising garrison doubler
            GnprtbEntry {
                parameter_id: 7684,
                development: 5,
                alliance_sp_easy: 5,
                alliance_sp_medium: 5,
                alliance_sp_hard: 5,
                empire_sp_easy: 5,
                empire_sp_medium: 5,
                empire_sp_hard: 5,
                multiplayer: 5,
            }, // KDY capship penalty per unit
            GnprtbEntry {
                parameter_id: 7685,
                development: 2,
                alliance_sp_easy: 2,
                alliance_sp_medium: 2,
                alliance_sp_hard: 2,
                empire_sp_easy: 2,
                empire_sp_medium: 2,
                empire_sp_hard: 2,
                multiplayer: 2,
            }, // KDY fighter penalty per unit
            GnprtbEntry {
                parameter_id: 7691,
                development: 0,
                alliance_sp_easy: 0,
                alliance_sp_medium: 0,
                alliance_sp_hard: 0,
                empire_sp_easy: 0,
                empire_sp_medium: 0,
                empire_sp_hard: 0,
                multiplayer: 0,
            }, // Support delta Empire-controlled transition
            GnprtbEntry {
                parameter_id: 7693,
                development: 1,
                alliance_sp_easy: 1,
                alliance_sp_medium: 1,
                alliance_sp_hard: 1,
                empire_sp_easy: 1,
                empire_sp_medium: 1,
                empire_sp_hard: 1,
                multiplayer: 1,
            }, // Timer 0x386: a blockader with the majority gains
            GnprtbEntry {
                parameter_id: 7694,
                development: 30,
                alliance_sp_easy: 30,
                alliance_sp_medium: 30,
                alliance_sp_hard: 30,
                empire_sp_easy: 30,
                empire_sp_medium: 30,
                empire_sp_hard: 30,
                multiplayer: 30,
            }, // Timer 0x386 delay, the blockader with the majority
            GnprtbEntry {
                parameter_id: 7695,
                development: -1,
                alliance_sp_easy: -1,
                alliance_sp_medium: -1,
                alliance_sp_hard: -1,
                empire_sp_easy: -1,
                empire_sp_medium: -1,
                empire_sp_hard: -1,
                multiplayer: -1,
            }, // Timer 0x386: a blockader without the majority loses
            GnprtbEntry {
                parameter_id: 7696,
                development: 30,
                alliance_sp_easy: 30,
                alliance_sp_medium: 30,
                alliance_sp_hard: 30,
                empire_sp_easy: 30,
                empire_sp_medium: 30,
                empire_sp_hard: 30,
                multiplayer: 30,
            }, // Timer 0x386 delay, the blockader without the majority
            GnprtbEntry {
                parameter_id: 7697,
                development: -1,
                alliance_sp_easy: -1,
                alliance_sp_medium: -1,
                alliance_sp_hard: -1,
                empire_sp_easy: -1,
                empire_sp_medium: -1,
                empire_sp_hard: -1,
                multiplayer: -1,
            }, // Support delta controlled systems
            GnprtbEntry {
                parameter_id: 7760,
                development: 60,
                alliance_sp_easy: 60,
                alliance_sp_medium: 60,
                alliance_sp_hard: 60,
                empire_sp_easy: 60,
                empire_sp_medium: 60,
                empire_sp_hard: 60,
                multiplayer: 60,
            }, // Support a side needs to hold a system without troops
        ];
        GnprtbParams::new(entries)
    }

    // FUN_00559c40 negates the drift for side 2, and FUN_00509ec0 stores it
    // at system +0x6c; an unheld system stores 0.
    #[test]
    fn the_stored_drift_is_positive_for_the_alliance_and_negative_for_the_empire() {
        let alliance = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let empire = ControlKind::Controlled(crate::dat::Faction::Empire);
        assert_eq!(support_drift_value(alliance, -0.25, 0.25), 25);
        assert_eq!(support_drift_value(empire, 0.5, -0.5), -50);
        assert_eq!(support_drift_value(ControlKind::Uncontrolled, 0.0, 0.0), 0);
    }
    #[test]
    fn support_drift_no_garrison_low_support() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.8,
            is_populated: true,
            total_energy: 5,
            raw_materials: 5,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
        // Low alliance support (0.1 < 0.40 threshold), no fleet → should drift away
        assert!(a_delta < 0.0, "alliance support should decrease: {a_delta}");
        assert!(e_delta > 0.0, "empire support should increase: {e_delta}");
    }

    #[test]
    fn support_drift_stops_for_a_friendly_usable_death_star_not_a_fleet() {
        // FUN_0050b230 passes FUN_00559c40 its gate p7 from FUN_00509140,
        // which counts the usable fleets' children of family [0x18,0x1c)
        // (FUN_00500450, mode 1): a Death Star, not a fleet, stops the drift.
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.8,
            is_populated: true,
            total_energy: 5,
            raw_materials: 5,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 1,
            empire_fleets: 0, // Friendly fleet present
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
        // A friendly fleet with no ship counted: support 10 drifts by 7737.
        assert!((a_delta + 0.75).abs() < 1e-6, "a fleet does not stop it: {a_delta}");
        assert!((e_delta - 0.75).abs() < 1e-6, "a fleet does not stop it: {e_delta}");
        let presence = MilitaryPresence {
            alliance_death_stars: 1,
            ..presence
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
        assert!(
            a_delta.abs() < f32::EPSILON && e_delta.abs() < f32::EPSILON,
            "a friendly usable Death Star stops it: {a_delta}/{e_delta}"
        );
    }

    #[test]
    fn support_drift_with_troops_reduces() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.15,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 5,
            raw_materials: 5,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let no_troops = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let with_troops = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 5,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
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
    fn garrison_requirement_above_threshold_is_zero() {
        let gnprtb = stock_gnprtb();
        let control = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let req = calculate_garrison_requirement(0.7, control, false, false, &gnprtb, 2);
        assert_eq!(req, 0, "above threshold should need 0 garrison");
    }

    #[test]
    fn empire_garrison_is_halved_only_with_a_death_star_in_the_sector() {
        // FUN_00559fe0 divides by GNPRTB 7680 only when param_3 (system
        // +0x88 bit 11, a usable Death Star in the sector, FUN_005178b0) is
        // set and the side is 2.
        let gnprtb = stock_gnprtb();
        let alliance = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let empire = ControlKind::Controlled(crate::dat::Faction::Empire);
        let req = |control, death_star| {
            calculate_garrison_requirement(0.3, control, death_star, false, &gnprtb, 2)
        };
        assert_eq!(req(alliance, false), 3);
        assert_eq!(req(alliance, true), 3);
        assert_eq!(req(empire, false), 3);
        assert_eq!(req(empire, true), 1);
        // Support 0: ceil(60 / 10) = 6, halved to 3.
        assert_eq!(
            calculate_garrison_requirement(0.0, empire, true, false, &gnprtb, 2),
            3
        );
    }

    #[test]
    fn a_revolt_is_visible_only_while_a_populated_held_system_keeps_its_garrison() {
        // FUN_0050bb00 sets system +0x88 bit 9 when the system is populated,
        // held by side 1 or 2, in revolt (bit 2), and its troop surplus
        // (+0x7c) is not negative.
        let alliance = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let visible = |support: f32, control, populated: bool, in_revolt: bool| {
            let (mut world, sys) = make_singleton_world(support, 1.0 - support, control, populated);
            if in_revolt {
                world.revolting.insert(sys);
            }
            let mut state = EconomyState::default();
            world.difficulty_index = 2;
            EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);
            state
                .per_system
                .get(&sys)
                .is_some_and(|e| e.uprising_visible)
        };
        // Support 90 needs no garrison, so the surplus is 0.
        assert!(visible(0.9, alliance, true, true));
        assert!(!visible(0.9, alliance, true, false));
        assert!(!visible(0.9, alliance, false, true));
        assert!(!visible(0.9, ControlKind::Uncontrolled, true, true));
        // Support 30 in revolt needs 3 * 2 regiments and has none.
        assert!(!visible(0.3, alliance, true, true));
    }

    #[test]
    fn a_support_change_recomputes_the_garrison_drift_and_collection_rate_at_once() {
        // FUN_00509c30 -> hook +0x1d0 (FUN_005109f0.c:14-29): after the
        // control rebuild (FUN_0050b610) it reruns FUN_0050b5a0 (garrison,
        // +0x80), FUN_0050b230 (drift, +0x6c) and FUN_0050b2c0 (collection
        // rate, +0x70) before the setter returns, with no economy pass.
        for side in [crate::dat::Faction::Alliance, crate::dat::Faction::Empire] {
            let (mut world, sys) = make_singleton_world(0.5, 0.5, ControlKind::Controlled(side), true);
            let unit = world.troops.insert(crate::world::TroopUnit {
                class_dat_id: DatId(0x1000_0001),
                is_alliance: side == crate::dat::Faction::Alliance,
            });
            world.systems[sys].ground_units.push(unit);
            crate::support_shift::refresh_troop_bits(&mut world, sys);
            crate::uprising::apply_support_change(&mut world, sys, side, -20);
            // Support 30: ceil((60 - 30) / 10) = 3 (FUN_0055a050).
            assert_eq!(
                world.support_values.get(&sys).map(|v| v.garrison_requirement),
                Some(3),
                "{side:?}"
            );
            let mut pass = world.clone();
            EconomySystem::advance(
                &mut EconomyState::default(),
                &mut pass,
                &[TickEvent { tick: 1 }],
            );
            assert_eq!(
                world.support_values.get(&sys),
                pass.support_values.get(&sys),
                "{side:?}"
            );
        }
    }

    #[test]
    fn a_system_no_side_holds_needs_no_garrison() {
        // FUN_0050b5a0 stores 0 unless the side is 1 or 2.
        let gnprtb = stock_gnprtb();
        assert_eq!(
            calculate_garrison_requirement(0.1, ControlKind::Uncontrolled, false, true, &gnprtb, 2),
            0
        );
    }

    #[test]
    fn economy_advance_no_ticks_no_events() {
        // The same world emits a SupportChanged on its first tick.
        let (mut world, _sys) = make_singleton_world(
            0.10,
            0.90,
            ControlKind::Controlled(crate::dat::Faction::Alliance),
            true,
        );
        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &[]);
        assert!(events.is_empty());
    }

    #[test]
    fn economy_advance_skips_destroyed_systems() {
        let mut state = EconomyState::default();
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let _sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Destroyed".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 5,
            raw_materials: 5,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: true, // Destroyed!
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });
        let tick_events = vec![TickEvent { tick: 1 }];
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &tick_events);
        // Destroyed system should be skipped entirely.
        assert!(
            events.is_empty(),
            "destroyed system should produce no events"
        );
    }

    // -----------------------------------------------------------------------
    // Phase 3b: Resource capacity enforcement tests
    // -----------------------------------------------------------------------

    #[test]
    fn kdy_production_modifier_reduces_with_ships() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with fleet presence (capships reduce production)
        let fleet_key = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: crate::world::ShipInstance::make(
                crate::ids::CapitalShipKey::default(),
                100,
                true,
                3,
            ),
            fighters: vec![crate::world::FighterEntry {
                class: crate::ids::FighterKey::default(),
                count: 4,
                carrier: 0,
            }],
            characters: vec![],
            is_alliance: true,
            bombarding: false,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "KDY".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.7,
            popularity_empire: 0.3,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![fleet_key],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

        let eco = state.per_system.get(&sys_key).unwrap();
        // 3 capships * 5 (capship penalty) + 4 fighters * 2 (fighter penalty) = 15 + 8 = 23
        // production_modifier = max(100 - 23, 0) = 77
        assert_eq!(
            eco.production_modifier, 77,
            "production_modifier should be 100 - 3*5 - 4*2 = 77, got {}",
            eco.production_modifier
        );
    }

    // FUN_0050b310.c:32-45 (the withdraw percent) and FUN_0050b230 ->
    // FUN_00509090.c:23,30 (the drift) walk the system's fleets in mode 1
    // and their ships in mode 1, and FUN_005039d0(ship, 1) a usable ship's
    // squadrons: a ship on its own leg from its yard (bit 5, FUN_00556430)
    // is not usable (0x004f7b80), nor is a fleet holding only it
    // (FUN_004fe540.c:20-35).
    #[test]
    fn a_ship_unfinished_or_on_its_own_leg_adds_no_military_presence_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, sys) = make_singleton_world(
                0.5,
                0.5,
                ControlKind::Controlled(crate::dat::Faction::of_alliance(is_alliance)),
                true,
            );
            let mut ship =
                crate::world::ShipInstance::new(crate::ids::CapitalShipKey::default(), 100, is_alliance);
            ship.tag = 7;
            ship.transit = Some(crate::world::ShipTransit {
                from: sys,
                to: sys,
                arrival_tick: 9,
            });
            let fleet = world.fleets.insert(crate::world::Fleet {
                location: sys,
                capital_ships: vec![ship],
                fighters: vec![crate::world::FighterEntry {
                    class: crate::ids::FighterKey::default(),
                    count: 2,
                    carrier: 7,
                }],
                characters: vec![],
                is_alliance,
                bombarding: false,
            });
            world.systems[sys].fleets.push(fleet);

            let presence = count_military_presence(&world, &world.systems[sys]);
            assert_eq!(
                (
                    presence.alliance_fleets + presence.empire_fleets,
                    presence.alliance_capships + presence.empire_capships,
                    presence.alliance_fighters + presence.empire_fighters,
                ),
                (0, 0, 0),
                "{is_alliance}"
            );

            // Unfinished at its order (FUN_004f7d50(.., completed 0)): not
            // usable either.
            world.fleets[fleet].capital_ships[0].transit = None;
            world.fleets[fleet].capital_ships[0].completed = false;
            let presence = count_military_presence(&world, &world.systems[sys]);
            assert_eq!(
                presence.alliance_fleets + presence.empire_fleets + presence.alliance_capships
                    + presence.empire_capships,
                0,
                "{is_alliance}"
            );

            world.fleets[fleet].capital_ships[0].completed = true;
            let presence = count_military_presence(&world, &world.systems[sys]);
            let own = |presence: &MilitaryPresence| {
                if is_alliance {
                    (presence.alliance_fleets, presence.alliance_capships, presence.alliance_fighters)
                } else {
                    (presence.empire_fleets, presence.empire_capships, presence.empire_fighters)
                }
            };
            assert_eq!(own(&presence), (1, 1, 2), "{is_alliance}");

            // Beside the usable ship, a second on its own leg: its 5
            // squadrons stay out; 3 riding the fleet (port: no carrier) and
            // the system's own 4 count.
            let mut second =
                crate::world::ShipInstance::new(crate::ids::CapitalShipKey::default(), 100, is_alliance);
            second.tag = 8;
            second.transit = Some(crate::world::ShipTransit {
                from: sys,
                to: sys,
                arrival_tick: 9,
            });
            let value = &mut world.fleets[fleet];
            value.capital_ships.push(second);
            for (carrier, count) in [(8, 5), (0, 3)] {
                value.fighters.push(crate::world::FighterEntry {
                    class: crate::ids::FighterKey::default(),
                    count,
                    carrier,
                });
            }
            world.systems[sys].add_squadrons(crate::ids::FighterKey::default(), 4, is_alliance);
            let presence = count_military_presence(&world, &world.systems[sys]);
            assert_eq!(own(&presence), (1, 1, 2 + 3 + 4), "{is_alliance}");
        }
    }

    // FUN_0050b230's drift inputs: p7 FUN_00509140 counts the usable
    // fleets' usable children of family [0x18,0x1c) (FUN_00500450, mode 1),
    // the Death Stars; p5 FUN_00508fb0 counts the system's own squadrons
    // (FUN_00503a50, mode 1), none aboard a fleet.
    #[test]
    fn the_drift_counts_usable_death_stars_and_the_systems_own_squadrons_for_either_side() {
        for is_alliance in [true, false] {
            let (mut world, sys) = make_singleton_world(
                0.5,
                0.5,
                ControlKind::Controlled(crate::dat::Faction::of_alliance(is_alliance)),
                true,
            );
            let death_star = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    dat_id: DatId::new(crate::world::DEATH_STAR_CLASS_ID),
                    family: 0x18,
                    ..Default::default()
                });
            let fleet = world.fleets.insert(crate::world::Fleet {
                location: sys,
                capital_ships: vec![crate::world::ShipInstance::new(death_star, 100, is_alliance)],
                fighters: vec![crate::world::FighterEntry {
                    class: crate::ids::FighterKey::default(),
                    count: 2,
                    carrier: 0,
                }],
                is_alliance,
                ..crate::world::Fleet::default()
            });
            world.systems[sys].fleets.push(fleet);
            world.systems[sys].add_squadrons(crate::ids::FighterKey::default(), 4, is_alliance);
            let own = |presence: &MilitaryPresence| {
                if is_alliance {
                    (presence.alliance_death_stars, presence.alliance_system_fighters)
                } else {
                    (presence.empire_death_stars, presence.empire_system_fighters)
                }
            };

            let presence = count_military_presence(&world, &world.systems[sys]);
            assert_eq!(own(&presence), (1, 4), "{is_alliance}");
            world.fleets[fleet].capital_ships[0].transit = Some(crate::world::ShipTransit {
                from: sys,
                to: sys,
                arrival_tick: 9,
            });
            let presence = count_military_presence(&world, &world.systems[sys]);
            assert_eq!(own(&presence), (0, 4), "{is_alliance}");
            // A usable cruiser makes the fleet usable; neither it nor the
            // Death Star on its own leg is a usable Death Star.
            let cruiser = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass::default());
            world.fleets[fleet]
                .capital_ships
                .push(crate::world::ShipInstance::new(cruiser, 100, is_alliance));
            let presence = count_military_presence(&world, &world.systems[sys]);
            assert_eq!(own(&presence), (0, 4), "{is_alliance}");
        }
    }

    #[test]
    fn kdy_production_modifier_floors_at_zero() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with massive fleet presence
        let fleet_key = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: crate::world::ShipInstance::make(
                crate::ids::CapitalShipKey::default(),
                100,
                true,
                10,
            ),
            fighters: vec![crate::world::FighterEntry {
                class: crate::ids::FighterKey::default(),
                count: 30,
                carrier: 0,
            }],
            characters: vec![],
            is_alliance: true,
            bombarding: false,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Overcrowded".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.7,
            popularity_empire: 0.3,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![fleet_key],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

        let eco = state.per_system.get(&sys_key).unwrap();
        // 10 capships * 5 + 30 fighters * 2 = 50 + 60 = 110 → max(100 - 110, 0) = 0
        assert_eq!(
            eco.production_modifier, 0,
            "production modifier should floor at 0, got {}",
            eco.production_modifier
        );
    }

    #[test]
    fn energy_overcap_emits_event() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with total_energy=2 but 5 production facilities
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Overcap".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 2,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0001),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0002),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0003),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0004),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0005),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
            ],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

        // Should emit EnergyOvercapped (5 facilities > 2 capacity)
        assert!(
            events
                .iter()
                .any(|e| matches!(e, EconomyEvent::EnergyOvercapped { .. })),
            "Expected EnergyOvercapped event"
        );

        // energy_allocated should be capped at capacity
        let eco = state.per_system.get(&sys_key).unwrap();
        assert_eq!(
            eco.energy_allocated, 2,
            "energy should be capped at system capacity"
        );
        assert!(eco.energy_overcapped, "should flag overcap");
    }

    #[test]
    fn no_overcap_when_within_limits() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with total_energy=10 and only 3 production facilities
        world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Normal".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0001),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x1800_0002),
                        side: crate::dat::Faction::Alliance,
                        is_mine: false,
                    }),
            ],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

        // Should NOT emit overcap events
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, EconomyEvent::EnergyOvercapped { .. })),
            "Should not overcap with 2 facilities and capacity 10"
        );
    }

    #[test]
    fn raw_material_overcap_emits_event() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with raw_materials=1 but 3 manufacturing facilities
        world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "MineCap".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 1,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![
                world.manufacturing_facilities.insert(
                    crate::world::ManufacturingFacilityInstance {
                        class_dat_id: DatId(0x1600_0001),
                        side: crate::dat::Faction::Alliance,
                        is_shipyard: false,
                    },
                ),
                world.manufacturing_facilities.insert(
                    crate::world::ManufacturingFacilityInstance {
                        class_dat_id: DatId(0x1600_0002),
                        side: crate::dat::Faction::Alliance,
                        is_shipyard: false,
                    },
                ),
                world.manufacturing_facilities.insert(
                    crate::world::ManufacturingFacilityInstance {
                        class_dat_id: DatId(0x1600_0003),
                        side: crate::dat::Faction::Alliance,
                        is_shipyard: false,
                    },
                ),
            ],
            production_facilities: vec![
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x2D00_0001),
                        side: crate::dat::Faction::Alliance,
                        is_mine: true,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x2D00_0002),
                        side: crate::dat::Faction::Alliance,
                        is_mine: true,
                    }),
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId(0x2D00_0003),
                        side: crate::dat::Faction::Alliance,
                        is_mine: true,
                    }),
            ],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

        assert!(
            events
                .iter()
                .any(|e| matches!(e, EconomyEvent::RawMaterialOvercapped { .. })),
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
    fn informant_incident_on_garrison_shortfall() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with low support (garrison > 0) but no troops (deficit)
        world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Undermanned".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.4,
            popularity_empire: 0.6,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);
        assert!(
            events.iter().any(|e| matches!(
                e,
                EconomyEvent::IncidentTriggered {
                    incident_type: "informant",
                    ..
                }
            )),
            "Should fire informant incident on garrison shortfall"
        );
    }

    // -----------------------------------------------------------------------
    // Phase 3b: System summary tests
    // -----------------------------------------------------------------------

    #[test]
    fn system_summary_troop_surplus() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        // System with low support (triggers garrison requirement) and some troops
        let troop1 = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: DatId(0x1400_0100),
            is_alliance: true,
        });
        let troop2 = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: DatId(0x1400_0100),
            is_alliance: true,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.3,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![troop1, troop2],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

        let eco = state.per_system.get(&sys_key).unwrap();
        // Garrison = ceil(30/10) = 3, troops = 2, surplus = 2 - 3 = -1
        assert_eq!(
            eco.summary.troop_surplus, -1,
            "troop surplus should be troops - garrison = 2 - 3 = -1, got {}",
            eco.summary.troop_surplus
        );
        assert_eq!(eco.summary.total_controlling_troops, 2);
    }

    #[test]
    fn the_troop_surplus_counts_every_regiment_and_is_zero_for_an_unheld_system() {
        // FUN_0050b500: for a system held by side 1 or 2, FUN_00504c40 (no side
        // filter) counts all regiments; any other system keeps surplus 0.
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let ground_units = [true, true, false]
            .map(|is_alliance| {
                world.troops.insert(crate::world::TroopUnit {
                    class_dat_id: DatId(0x1400_0100),
                    is_alliance,
                })
            })
            .to_vec();
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.3,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units,
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });
        let surplus = |world: &GameWorld| {
            let mut state = EconomyState::default();
            let mut world = world.clone();
            world.difficulty_index = 2;
            EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);
            state.per_system[&sys_key].summary.troop_surplus
        };
        // Garrison ceil(30 / 10) = 3, three regiments of either side: 0.
        assert_eq!(surplus(&world), 0);
        world.systems[sys_key].control = ControlKind::Uncontrolled;
        assert_eq!(surplus(&world), 0);
        world.systems[sys_key].ground_units.clear();
        assert_eq!(surplus(&world), 0);
    }

    #[test]
    fn system_summary_fleet_posture() {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let fleet1 = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: crate::world::ShipInstance::make(
                crate::ids::CapitalShipKey::default(),
                100,
                true,
                2,
            ),
            fighters: vec![crate::world::FighterEntry {
                class: crate::ids::FighterKey::default(),
                count: 5,
                carrier: 0,
            }],
            characters: vec![],
            is_alliance: true,
            bombarding: false,
        });
        let fleet2 = world.fleets.insert(crate::world::Fleet {
            location: crate::ids::SystemKey::default(),
            capital_ships: crate::world::ShipInstance::make(
                crate::ids::CapitalShipKey::default(),
                100,
                false,
                3,
            ),
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            bombarding: false,
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Contested".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![fleet1, fleet2],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });

        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);

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
            dat_id: DatId(0),
            name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 3,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let result = resolve_system_control(&sys, &presence, false, &gnprtb, 2);
        assert_eq!(
            result,
            ControlKind::Controlled(crate::dat::Faction::Alliance)
        );
    }

    #[test]
    fn side_resolution_with_both_sides_troops_keeps_the_earlier_troop_holder() {
        // FUN_0050b610.c:45-61: both sides' regiments keep the old troop
        // bits; the holder is the troop side while they are set, else the
        // support side (:64-70), as with no regiments at all.
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 2,
            empire_troops: 3,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        assert_eq!(
            resolve_system_control(&sys, &presence, true, &gnprtb, 2),
            ControlKind::Controlled(crate::dat::Faction::Empire)
        );
        let empty = MilitaryPresence {
            alliance_troops: 0,
            empire_troops: 0,
            ..presence
        };
        assert_eq!(
            resolve_system_control(&sys, &presence, false, &gnprtb, 2),
            resolve_system_control(&sys, &empty, false, &gnprtb, 2)
        );
    }

    #[test]
    fn an_unpopulated_system_is_held_by_its_regiments_alone() {
        // FUN_0050b610.c:24-33: an unpopulated system's support side is 3,
        // whatever its support; one side's regiments still hold it
        // (.c:34-70).
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            popularity_alliance: 1.0,
            popularity_empire: 0.0,
            is_populated: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
            ..crate::world::System::default()
        };
        let presence = |alliance_troops, empire_troops| MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops,
            empire_troops,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        assert_eq!(
            resolve_system_control(&sys, &presence(0, 0), false, &gnprtb, 2),
            ControlKind::Uncontrolled
        );
        assert_eq!(
            resolve_system_control(&sys, &presence(0, 1), false, &gnprtb, 2),
            ControlKind::Controlled(crate::dat::Faction::Empire)
        );
    }

    #[test]
    fn side_resolution_no_troops_goes_to_the_side_whose_support_reaches_the_threshold() {
        let gnprtb = stock_gnprtb();
        let mut sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.6,
            popularity_empire: 0.4,
            is_populated: true,
            total_energy: 60,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        // FUN_0055a080: Alliance support 60 reaches GNPRTB 7760, whatever
        // the earlier holder; Imperial support 60 (Alliance 40) gives the
        // Empire.
        let result = resolve_system_control(&sys, &presence, false, &gnprtb, 2);
        assert_eq!(
            result,
            ControlKind::Controlled(crate::dat::Faction::Alliance)
        );
        sys.popularity_alliance = 0.4;
        sys.popularity_empire = 0.6;
        let result = resolve_system_control(&sys, &presence, false, &gnprtb, 2);
        assert_eq!(result, ControlKind::Controlled(crate::dat::Faction::Empire));
    }

    #[test]
    fn side_resolution_no_troops_and_split_support_is_neutral() {
        let gnprtb = stock_gnprtb();
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let result = resolve_system_control(&sys, &presence, false, &gnprtb, 2);
        // FUN_0055a080: support 50 for each side reaches neither 60.
        assert_eq!(result, ControlKind::Uncontrolled);
    }

    // --- Task #42: Empire troop doubling in support drift ---

    #[test]
    fn support_drift_empire_troop_doubling() {
        let gnprtb = stock_gnprtb();
        // Alliance-controlled system with low support and 2 troops
        let sys_alliance = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.8,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence_alliance = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 2,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let (a_delta_alliance, _) =
            calculate_support_drift(&sys_alliance, &presence_alliance, &gnprtb, 2, true);

        // Empire-controlled system with same setup and 2 troops
        let sys_empire = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.1,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Empire),
        };
        let presence_empire = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 2,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let (_, e_delta_empire) =
            calculate_support_drift(&sys_empire, &presence_empire, &gnprtb, 2, true);

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
        let mut world = GameWorld {
            gnprtb,
            ..Default::default()
        };
        let _sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Unpopulated".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: false,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });

        let mut state = EconomyState::default();
        let tick = vec![TickEvent { tick: 1 }];
        world.difficulty_index = 2;
        let events = EconomySystem::advance(&mut state, &mut world, &tick);

        // Unpopulated system should produce zero events and no economy entry
        assert!(
            events.is_empty(),
            "unpopulated system should produce no events: {events:?}"
        );
        assert!(
            state.per_system.is_empty(),
            "no economy entry for unpopulated system"
        );
    }

    // --- Task #44: Uprising garrison doubling ---

    #[test]
    fn garrison_uprising_doubling() {
        let gnprtb = stock_gnprtb();
        // FUN_0050b5a0 passes system +0x88 bit 2 to FUN_00559fe0, which
        // multiplies by GNPRTB 7682. Controlled: ceil((60 - 30) / 10) = 3.
        let alliance = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let garrison_controlled =
            calculate_garrison_requirement(0.30, alliance, false, false, &gnprtb, 2);
        // In revolt: 3 * GNPRTB[7682](2) = 6
        let garrison_uprising =
            calculate_garrison_requirement(0.30, alliance, false, true, &gnprtb, 2);
        assert_eq!(garrison_controlled, 3, "controlled garrison");
        assert_eq!(
            garrison_uprising,
            garrison_controlled * 2,
            "uprising garrison should be 2x controlled"
        );
    }

    // --- Task #45: Collection rate at zero support + drift at threshold boundary ---

    #[test]
    fn collection_rate_at_zero_support() {
        let gnprtb = stock_gnprtb();
        let rate = calculate_collection_rate(0.0, &gnprtb, 2);
        // At zero support: (100 * 100) / max(0, 1) = 10000
        assert!(
            (rate - 10000.0).abs() < f32::EPSILON,
            "zero support should yield max rate: {rate}"
        );
    }

    fn presence_of(
        fleets: (u32, u32),
        fighters: (u32, u32),
        troops: (u32, u32),
    ) -> MilitaryPresence {
        MilitaryPresence {
            alliance_fleets: fleets.0,
            empire_fleets: fleets.1,
            alliance_fighters: fighters.0,
            empire_fighters: fighters.1,
            alliance_troops: troops.0,
            empire_troops: troops.1,
            alliance_capships: 0,
            empire_capships: 0,
            alliance_system_fighters: fighters.0,
            empire_system_fighters: fighters.1,
            ..MilitaryPresence::default()
        }
    }

    /// `FUN_00559c40` with the shipped GNPRTB: below 7732 (40) and with no
    /// friendly usable Death Star, the holder's support drifts by 7737 (75)
    /// at or below 7736 (20), 7735 (50) at or below 7734 (30), else 7733
    /// (25), less 7686 (10) per friendly usable capital ship, 7687 (5) per
    /// friendly squadron of the system and 7688 (2) per regiment, the Empire's
    /// regiments doubled by 7680 with a Death Star in the sector
    /// (`FUN_00559b60`); the drift moves support away from the holder.
    #[test]
    fn the_drift_takes_its_bracket_less_each_friendly_unit() {
        use crate::dat::Faction::{Alliance, Empire};
        let gnprtb = stock_gnprtb();
        let none = || presence_of((0, 0), (0, 0), (0, 0));
        // (holder, holder support %, presence, Death Star, drift away from the holder %)
        let cases = [
            (Alliance, 15, none(), false, 75),
            (Alliance, 20, none(), false, 75),
            (Alliance, 25, none(), false, 50),
            (Alliance, 30, none(), false, 50),
            (Alliance, 35, none(), false, 25),
            (Alliance, 41, none(), false, 0),
            (Alliance, 15, presence_of((0, 0), (2, 0), (3, 0)), false, 59),
            (Empire, 15, presence_of((0, 0), (0, 2), (0, 3)), false, 59),
            (Empire, 15, presence_of((0, 0), (0, 0), (0, 3)), true, 63),
            (Alliance, 15, presence_of((0, 0), (0, 0), (3, 0)), true, 69),
            // FUN_0050b230 -> FUN_00559c40: a fleet suppresses by its usable
            // ships alone (p4, FUN_00509090 x 7686), the squadrons are the
            // system's own (p5, FUN_00508fb0 x 7687), and only a friendly
            // usable Death Star stops the drift (p7, FUN_00509140 ->
            // FUN_00500450 [0x18,0x1c)).
            (Alliance, 15, presence_of((1, 0), (0, 0), (0, 0)), false, 75),
            (
                Alliance,
                15,
                MilitaryPresence {
                    alliance_capships: 2,
                    empire_capships: 3,
                    ..none()
                },
                false,
                55,
            ),
            (
                Empire,
                15,
                MilitaryPresence {
                    alliance_capships: 3,
                    empire_capships: 2,
                    ..none()
                },
                false,
                55,
            ),
            (
                Alliance,
                15,
                MilitaryPresence {
                    alliance_fighters: 2,
                    ..none()
                },
                false,
                75,
            ),
            (
                Alliance,
                15,
                MilitaryPresence {
                    alliance_death_stars: 1,
                    ..none()
                },
                false,
                0,
            ),
            (
                Empire,
                15,
                MilitaryPresence {
                    empire_death_stars: 1,
                    ..none()
                },
                false,
                0,
            ),
            (
                Empire,
                15,
                MilitaryPresence {
                    alliance_death_stars: 1,
                    ..none()
                },
                false,
                75,
            ),
        ];
        for (holder, support, presence, death_star, drift) in cases {
            let held = f32::from(u8::try_from(support).unwrap()) / 100.0;
            let sys = crate::world::System {
                popularity_alliance: if holder == Alliance { held } else { 1.0 - held },
                popularity_empire: if holder == Empire { held } else { 1.0 - held },
                control: ControlKind::Controlled(holder),
                ..crate::world::System::default()
            };
            let away = f32::from(u8::try_from(drift).unwrap()) / 100.0;
            let expected = if holder == Alliance {
                (-away, away)
            } else {
                (away, -away)
            };
            let (alliance, empire) =
                calculate_support_drift(&sys, &presence, &gnprtb, 2, death_star);
            assert!(
                (alliance - expected.0).abs() < 1e-6 && (empire - expected.1).abs() < 1e-6,
                "{holder:?} at {support} with {death_star}: {alliance}/{empire}, want {expected:?}"
            );
        }
        let neutral = crate::world::System {
            popularity_alliance: 0.1,
            popularity_empire: 0.1,
            control: ControlKind::Uncontrolled,
            ..crate::world::System::default()
        };
        assert_eq!(
            calculate_support_drift(&neutral, &none(), &gnprtb, 2, false),
            (0.0, 0.0)
        );
    }

    /// `FUN_0050ba90` counts the holder's regiments, the larger side's when
    /// nobody holds the system; `FUN_0050b8e0` marks the fighters contested
    /// when both sides have squadrons, and the fleets when both have two.
    #[test]
    fn the_summary_counts_the_holders_regiments_and_contests_only_shared_skies() {
        use crate::dat::Faction::{Alliance, Empire};
        let world = GameWorld::default();
        let summary = |control, presence: &MilitaryPresence| {
            let sys = crate::world::System {
                control,
                ..crate::world::System::default()
            };
            compute_system_summary(&world, &sys, presence, 0)
        };
        let mixed = presence_of((0, 0), (0, 0), (1, 3));
        assert_eq!(
            summary(ControlKind::Controlled(Alliance), &mixed).total_controlling_troops,
            1
        );
        assert_eq!(
            summary(ControlKind::Controlled(Empire), &mixed).total_controlling_troops,
            3
        );
        let mixed = presence_of((0, 0), (0, 0), (3, 1));
        assert_eq!(
            summary(ControlKind::Controlled(Empire), &mixed).total_controlling_troops,
            1
        );
        assert_eq!(
            summary(ControlKind::Uncontrolled, &mixed).total_controlling_troops,
            3
        );
        for (fighters, contested) in [((1, 1), true), ((1, 0), false), ((0, 1), false)] {
            let presence = presence_of((0, 0), fighters, (0, 0));
            assert_eq!(
                summary(ControlKind::Uncontrolled, &presence)
                    .fighter_posture
                    .is_contested,
                contested,
                "{fighters:?}"
            );
        }
        for (fleets, contested) in [((2, 2), true), ((2, 0), false), ((0, 2), false)] {
            let presence = presence_of(fleets, (0, 0), (0, 0));
            assert_eq!(
                summary(ControlKind::Uncontrolled, &presence)
                    .fleet_posture
                    .is_contested,
                contested,
                "{fleets:?}"
            );
        }
    }

    #[test]
    fn support_drift_at_exact_threshold() {
        let gnprtb = stock_gnprtb();
        // Support at exactly 0.40 (threshold = 40 in integer)
        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.40,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence = MilitaryPresence {
            alliance_fleets: 0,
            empire_fleets: 0,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 0,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let (a_delta, e_delta) = calculate_support_drift(&sys, &presence, &gnprtb, 2, true);
        // At exactly threshold (40), integer comparison: 40 > 40 is false → drift occurs.
        // Original: `if (support <= GNPRTB[7732])` means 40 <= 40 passes → drift.
        assert!(
            a_delta != 0.0,
            "at threshold boundary drift should occur: {a_delta}"
        );
        assert!(
            e_delta != 0.0,
            "at threshold boundary drift should occur: {e_delta}"
        );
    }

    // --- Task #46: Fleet posture is_contested + strengthen telemetry ---

    #[test]
    fn fleet_posture_contested_requires_two_per_side() {
        let gnprtb = stock_gnprtb();
        let world = GameWorld {
            gnprtb,
            ..Default::default()
        };

        let sys = crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 60,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        };
        let presence_1v1 = MilitaryPresence {
            alliance_fleets: 1,
            empire_fleets: 1,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 5,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let summary_1v1 = compute_system_summary(&world, &sys, &presence_1v1, 0);
        assert!(
            !summary_1v1.fleet_posture.is_contested,
            "1v1 fleets should NOT be contested"
        );

        let presence_2v2 = MilitaryPresence {
            alliance_fleets: 2,
            empire_fleets: 2,
            alliance_fighters: 0,
            empire_fighters: 0,
            alliance_troops: 5,
            empire_troops: 0,
            alliance_capships: 0,
            empire_capships: 0,
            ..MilitaryPresence::default()
        };
        let summary_2v2 = compute_system_summary(&world, &sys, &presence_2v2, 0);
        assert!(
            summary_2v2.fleet_posture.is_contested,
            "2v2 fleets should be contested"
        );
    }

    // -----------------------------------------------------------------------
    // Knesset Shamash-Bet Dabora 2 #K7: parameterized notification
    // idempotency tests for the state-transition events (K1
    // SupportChanged, K3 ResourceDiscovered) —
    // each must fire exactly ONCE per transition, not once per tick. K2's
    // support-based disaster was removed: the disaster is a galaxy timer
    // event (FUN_00556b50) that UprisingSystem raises.
    //
    // Per SIMP-L1 the plan explicitly merges six would-be idempotency
    // tests into this one parameterized fixture.
    // -----------------------------------------------------------------------

    /// Helper: build a minimally-seeded world with one system controlled
    /// by the given faction at the given support level.
    fn make_singleton_world(
        support_alliance: f32,
        support_empire: f32,
        control: ControlKind,
        is_populated: bool,
    ) -> (GameWorld, SystemKey) {
        let mut world = GameWorld {
            gnprtb: stock_gnprtb(),
            ..Default::default()
        };
        let sector_key = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "S".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let sys_key = world.systems.insert(crate::world::System {
            dat_id: DatId(0),
            name: "Test".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: support_alliance,
            popularity_empire: support_empire,
            is_populated,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control,
        });
        (world, sys_key)
    }

    #[test]
    fn k1_support_change_fires_only_when_the_holder_changes() {
        // Alliance support 25: the Empire's 75 reaches GNPRTB 7760, so the
        // holder by support is the Empire (FUN_0055a080), a change from the
        // default Neither: emit once. Second eval (no change): no emission.
        let (mut world, _sys) = make_singleton_world(
            0.25,
            0.75,
            ControlKind::Controlled(crate::dat::Faction::Alliance),
            true,
        );
        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events1 = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);
        let count1 = events1
            .iter()
            .filter(|e| matches!(e, EconomyEvent::SupportChanged { .. }))
            .count();
        assert_eq!(
            count1, 1,
            "K1: first tick should emit exactly one SupportChanged"
        );

        let events2 = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 2 }]);
        let count2 = events2
            .iter()
            .filter(|e| matches!(e, EconomyEvent::SupportChanged { .. }))
            .count();
        assert_eq!(count2, 0, "K1: an unchanged holder must not re-fire");
    }

    #[test]
    fn k3_resource_discovery_fires_on_positive_delta_not_on_seed() {
        // First tick (armed=false at start): no emission because the
        // seeded mine count isn't a "discovery". Second tick (now
        // armed): still no emission because nothing changed.
        let (mut world, sys_key) = make_singleton_world(
            0.5,
            0.5,
            ControlKind::Controlled(crate::dat::Faction::Alliance),
            true,
        );
        let mut state = EconomyState::default();
        world.difficulty_index = 2;
        let events1 = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 1 }]);
        let count1 = events1
            .iter()
            .filter(|e| matches!(e, EconomyEvent::ResourceDiscovered { .. }))
            .count();
        assert_eq!(
            count1, 0,
            "K3: seed tick must NOT emit ResourceDiscovered (not armed)"
        );

        // Seed a new mine. Second tick should detect the positive delta.
        let mine = world
            .production_facilities
            .insert(crate::world::ProductionFacilityInstance {
                class_dat_id: DatId(0x2200_0001),
                is_mine: true,
                side: crate::dat::Faction::Alliance,
            });
        world.systems[sys_key].production_facilities.push(mine);

        let events2 = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 2 }]);
        let count2 = events2
            .iter()
            .filter(|e| matches!(e, EconomyEvent::ResourceDiscovered { .. }))
            .count();
        assert_eq!(
            count2, 1,
            "K3: new mine should trigger exactly one ResourceDiscovered"
        );

        // Third tick (no change): should not re-fire.
        let events3 = EconomySystem::advance(&mut state, &mut world, &[TickEvent { tick: 3 }]);
        let count3 = events3
            .iter()
            .filter(|e| matches!(e, EconomyEvent::ResourceDiscovered { .. }))
            .count();
        assert_eq!(count3, 0, "K3: stable resource count must not re-fire");
    }
}
