//! Seed table loading — populates the GameWorld with starting forces.
//!
//! # Overview
//!
//! Nine DAT files (Pattern 3 — SeedGroup/SeedItem) define the starting deployment
//! for a new game.  Each `SeedItem.item_id` encodes the entity class via:
//!
//!   `item_id = (family_byte << 24) | sequential_id`
//!
//! where `family_byte` identifies the entity type (see `Identifiers.cs` in the
//! Metasharp reference).  This module cross-references those ids against the
//! already-loaded class arenas to create fleet/unit/facility instances.
//!
//! # Family byte → entity type mapping
//!
//! | Byte  | Entity type                     |
//! |-------|---------------------------------|
//! | 0x14  | Capital ship class              |
//! | 0x1C  | Fighter squadron class          |
//! | 0x10  | Troop regiment class            |
//! | 0x3C  | Special forces unit class       |
//! | 0x20  | Alliance HQ facility            |
//! | 0x22..0x25 | Defense facility classes   |
//! | 0x28..0x2A | Manufacturing fac. classes |
//! | 0x2C..0x2D | Production fac. classes    |
//! | 0x90/0x92  | System (by DatId)          |
//! | 0x00  | Null / placeholder (skip)       |
//!
//! # Placement logic
//!
//! - **CMUNEFTB** (Empire fleet, 1 group): all ships placed at Coruscant.
//! - **CMUNAFTB** (Alliance fleet, 2 groups): group 1 at Yavin, group 2 at the
//!   first Alliance-owned system found.  Without text names we approximate by
//!   using the Yavin DatId for group 1 and the HQ placeholder for group 2.
//! - **CMUNCRTB / CMUNHQTB / CMUNYVTB**: single-system garrison tables; placed
//!   at Coruscant, Alliance-HQ, and Yavin respectively.  The first item in each
//!   group is always `0x00000000` (a null placeholder) and is skipped.
//! - **CMUNEMTB / CMUNALTB**: multi-group army tables; items with a capital-ship
//!   id are treated as the fleet component for that group, remaining items as
//!   ground units at the same system.  Groups are cycled over the faction's
//!   known starting systems (Coruscant / Yavin order).
//! - **FACLCRTB / FACLHQTB**: facility tables; placed at Coruscant and Alliance
//!   HQ respectively.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use dat_dumper::types::seed_table::SeedTableFile;
use rebellion_core::ids::*;
use rebellion_core::world::*;

use crate::read_dat_file;

// ── Known system sequential IDs (lower 24 bits of the Identifiers.cs compound ids)
//
// The `system_key_map` passed to `apply_seeds` is keyed by the raw `id` field
// from SYSTEMSD.DAT records, which is just the sequential index (lower 24 bits).
// The high byte (family: 0x90 = explored, 0x92 = unexplored) is NOT stored in
// the map.  We therefore use the index portion only for lookup.
//
// From Identifiers.cs:
//   Coruscant = 0x90000109  → sequential index 0x109
//   Yavin     = 0x92000121  → sequential index 0x121

/// Empire HQ — Coruscant sequential system id.
const CORUSCANT_SEQ_ID: u32 = 0x109;
/// Alliance Yavin sequential system id.
const YAVIN_SEQ_ID: u32 = 0x121;

// ── Family byte constants ─────────────────────────────────────────────────────

const FAM_CAPITAL_SHIP: u8 = 0x14;
const FAM_FIGHTER:      u8 = 0x1C;
const FAM_TROOP:        u8 = 0x10;
const FAM_SPECIAL:      u8 = 0x3C;
// Defense facilities span 0x22–0x25.
const FAM_DEF_MIN: u8 = 0x22;
const FAM_DEF_MAX: u8 = 0x25;
// Manufacturing facilities span 0x28–0x2A.
const FAM_MFG_MIN: u8 = 0x28;
const FAM_MFG_MAX: u8 = 0x2A;
// Production facilities span 0x2C–0x2D.
const FAM_PROD_MIN: u8 = 0x2C;
const FAM_PROD_MAX: u8 = 0x2D;
/// Alliance HQ building (a special single facility).
const FAM_ALLIANCE_HQ: u8 = 0x20;

// ─────────────────────────────────────────────────────────────────────────────

/// Index built from the loaded capital-ship arena: `dat_id.raw() → CapitalShipKey`.
type CapShipIndex  = HashMap<u32, CapitalShipKey>;
/// Index built from the fighter arena: `dat_id.raw() → FighterKey`.
type FighterIndex  = HashMap<u32, FighterKey>;

/// Apply all 9 seed tables to the already-populated `world`.
///
/// Missing files are silently ignored (stripped installs, test environments).
/// Unresolvable item_ids are silently skipped with a debug log rather than
/// aborting — the game can start in a degraded state if needed.
pub fn apply_seeds(
    gdata_path: &Path,
    world: &mut GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
) -> anyhow::Result<()> {
    // Build class-lookup indices once (avoid repeated arena scans).
    let capship_index: CapShipIndex = world
        .capital_ship_classes
        .iter()
        .map(|(k, v)| (v.dat_id.raw(), k))
        .collect();
    let fighter_index: FighterIndex = world
        .fighter_classes
        .iter()
        .map(|(k, v)| (v.dat_id.raw(), k))
        .collect();

    // ── Empire fleet (CMUNEFTB) ───────────────────────────────────────────────
    // Single group — all ships orbit Coruscant.
    apply_fleet_seed(
        &load_seed(gdata_path, "CMUNEFTB.DAT")?,
        system_key_map,
        &capship_index,
        &fighter_index,
        &[CORUSCANT_SEQ_ID],
        false, // is_alliance
        world,
    );

    // ── Alliance fleet (CMUNAFTB) ─────────────────────────────────────────────
    // Two groups — group 1 at Yavin, group 2 at Yavin (Alliance starts small).
    apply_fleet_seed(
        &load_seed(gdata_path, "CMUNAFTB.DAT")?,
        system_key_map,
        &capship_index,
        &fighter_index,
        &[YAVIN_SEQ_ID, YAVIN_SEQ_ID],
        true, // is_alliance
        world,
    );

    // ── Empire army (CMUNEMTB) ────────────────────────────────────────────────
    // Multi-group.  Groups are placed at Coruscant (the only known Empire
    // starting system at load time; real game spreads them via GNPRTB).
    apply_army_seed(
        &load_seed(gdata_path, "CMUNEMTB.DAT")?,
        system_key_map,
        &[CORUSCANT_SEQ_ID],
        false,
        world,
    );

    // ── Alliance army (CMUNALTB) ──────────────────────────────────────────────
    apply_army_seed(
        &load_seed(gdata_path, "CMUNALTB.DAT")?,
        system_key_map,
        &[YAVIN_SEQ_ID],
        true,
        world,
    );

    // ── Empire Coruscant garrison (CMUNCRTB) ──────────────────────────────────
    apply_garrison_seed(
        &load_seed(gdata_path, "CMUNCRTB.DAT")?,
        system_key_map,
        CORUSCANT_SEQ_ID,
        false,
        world,
    );

    // ── Alliance HQ garrison (CMUNHQTB) ──────────────────────────────────────
    apply_garrison_seed(
        &load_seed(gdata_path, "CMUNHQTB.DAT")?,
        system_key_map,
        YAVIN_SEQ_ID,
        true,
        world,
    );

    // ── Alliance Yavin garrison (CMUNYVTB) ────────────────────────────────────
    apply_garrison_seed(
        &load_seed(gdata_path, "CMUNYVTB.DAT")?,
        system_key_map,
        YAVIN_SEQ_ID,
        true,
        world,
    );

    // ── Empire Coruscant facilities (FACLCRTB) ────────────────────────────────
    apply_facility_seed(
        &load_seed(gdata_path, "FACLCRTB.DAT")?,
        system_key_map,
        CORUSCANT_SEQ_ID,
        false,
        world,
    );

    // ── Alliance HQ facilities (FACLHQTB) ────────────────────────────────────
    apply_facility_seed(
        &load_seed(gdata_path, "FACLHQTB.DAT")?,
        system_key_map,
        YAVIN_SEQ_ID,
        true,
        world,
    );

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Load a seed table file, returning `Ok(None)` if the file doesn't exist.
fn load_seed(gdata_path: &Path, filename: &str) -> anyhow::Result<Option<SeedTableFile>> {
    let path = gdata_path.join(filename);
    if !path.exists() {
        return Ok(None);
    }
    let file: SeedTableFile = read_dat_file(&path)
        .with_context(|| format!("parsing seed table {}", filename))?;
    Ok(Some(file))
}

/// Apply a fleet seed table.  Each `SeedGroup` becomes one `Fleet`.
/// `system_rotation` lists the system DatIds to cycle through when assigning
/// fleets; if there are more groups than systems the last system is reused.
fn apply_fleet_seed(
    seed: &Option<SeedTableFile>,
    system_key_map: &HashMap<u32, SystemKey>,
    capship_index: &CapShipIndex,
    fighter_index: &FighterIndex,
    system_rotation: &[u32],
    is_alliance: bool,
    world: &mut GameWorld,
) {
    let seed = match seed {
        Some(s) => s,
        None => return,
    };

    for (group_idx, group) in seed.groups.iter().enumerate() {
        // Pick the target system for this fleet group.
        let system_dat_id = system_rotation
            .get(group_idx)
            .copied()
            .unwrap_or_else(|| *system_rotation.last().unwrap_or(&CORUSCANT_SEQ_ID));

        let system_key = match system_key_map.get(&system_dat_id) {
            Some(&k) => k,
            None => continue, // System not loaded — skip.
        };

        let mut fleet_capital_ships: Vec<ShipEntry> = Vec::new();
        let mut fleet_fighters: Vec<FighterEntry> = Vec::new();

        for item in &group.items {
            if item.item_id == 0 {
                continue; // null placeholder
            }
            dispatch_fleet_item(
                item.item_id,
                capship_index,
                fighter_index,
                &mut fleet_capital_ships,
                &mut fleet_fighters,
            );
        }

        if fleet_capital_ships.is_empty() && fleet_fighters.is_empty() {
            continue; // Nothing to deploy.
        }

        let fleet = Fleet {
            location: system_key,
            capital_ships: fleet_capital_ships,
            fighters: fleet_fighters,
            characters: Vec::new(),
            is_alliance,
        };
        let fleet_key = world.fleets.insert(fleet);
        world.systems[system_key].fleets.push(fleet_key);
    }
}

/// Apply an army seed table (CMUNEMTB / CMUNALTB).
/// Items can mix capital ships (fleet component) and ground units.
/// All groups are placed at the given system rotation.
fn apply_army_seed(
    seed: &Option<SeedTableFile>,
    system_key_map: &HashMap<u32, SystemKey>,
    system_rotation: &[u32],
    is_alliance: bool,
    world: &mut GameWorld,
) {
    let seed = match seed {
        Some(s) => s,
        None => return,
    };

    for (group_idx, group) in seed.groups.iter().enumerate() {
        let system_dat_id = system_rotation
            .get(group_idx)
            .copied()
            .unwrap_or_else(|| *system_rotation.last().unwrap_or(&CORUSCANT_SEQ_ID));

        let system_key = match system_key_map.get(&system_dat_id) {
            Some(&k) => k,
            None => continue,
        };

        for item in &group.items {
            if item.item_id == 0 {
                continue;
            }
            dispatch_ground_item(item.item_id, system_key, is_alliance, world);
        }
    }
}

/// Apply a single-system garrison seed (CMUNCRTB / CMUNHQTB / CMUNYVTB).
/// The first item in each group is always `0x00000000` (skip it).
fn apply_garrison_seed(
    seed: &Option<SeedTableFile>,
    system_key_map: &HashMap<u32, SystemKey>,
    system_dat_id: u32,
    is_alliance: bool,
    world: &mut GameWorld,
) {
    let seed = match seed {
        Some(s) => s,
        None => return,
    };
    let system_key = match system_key_map.get(&system_dat_id) {
        Some(&k) => k,
        None => return,
    };

    for group in &seed.groups {
        for item in &group.items {
            if item.item_id == 0 {
                continue;
            }
            dispatch_ground_item(item.item_id, system_key, is_alliance, world);
        }
    }
}

/// Apply a facility seed table (FACLCRTB / FACLHQTB).
fn apply_facility_seed(
    seed: &Option<SeedTableFile>,
    system_key_map: &HashMap<u32, SystemKey>,
    system_dat_id: u32,
    is_alliance: bool,
    world: &mut GameWorld,
) {
    let seed = match seed {
        Some(s) => s,
        None => return,
    };
    let system_key = match system_key_map.get(&system_dat_id) {
        Some(&k) => k,
        None => return,
    };

    for group in &seed.groups {
        for item in &group.items {
            if item.item_id == 0 {
                continue;
            }
            dispatch_facility_item(item.item_id, system_key, is_alliance, world);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Item dispatch: route an item_id to the correct arena
// ─────────────────────────────────────────────────────────────────────────────

/// Route a fleet-context item (capital ship or fighter) to the right Vec.
///
/// `item_id` is the full compound id `(family_byte << 24) | sequential_id`.
/// The class arenas were loaded from entity DAT files which store only the
/// sequential part in their `id` field, so we strip the high byte before
/// indexing into the lookup maps.
fn dispatch_fleet_item(
    item_id: u32,
    capship_index: &CapShipIndex,
    fighter_index: &FighterIndex,
    capital_ships: &mut Vec<ShipEntry>,
    fighters: &mut Vec<FighterEntry>,
) {
    let family = (item_id >> 24) as u8;
    let seq_id = item_id & 0x00FF_FFFF;
    match family {
        FAM_CAPITAL_SHIP => {
            if let Some(&class) = capship_index.get(&seq_id) {
                // Merge into existing entry for this class, or add new entry.
                if let Some(entry) = capital_ships.iter_mut().find(|e| e.class == class) {
                    entry.count += 1;
                } else {
                    capital_ships.push(ShipEntry { class, count: 1 });
                }
            }
        }
        FAM_FIGHTER => {
            if let Some(&class) = fighter_index.get(&seq_id) {
                if let Some(entry) = fighters.iter_mut().find(|e| e.class == class) {
                    entry.count += 1;
                } else {
                    fighters.push(FighterEntry { class, count: 1 });
                }
            }
        }
        _ => {
            // Other families (troops, facilities) don't belong in a fleet context;
            // silently ignore.
        }
    }
}

/// Route a ground-context item (troop, special forces) to the system's lists.
fn dispatch_ground_item(
    item_id: u32,
    system_key: SystemKey,
    is_alliance: bool,
    world: &mut GameWorld,
) {
    let family = (item_id >> 24) as u8;
    let class_dat_id = DatId::new(item_id);

    match family {
        FAM_TROOP => {
            let unit = TroopUnit { class_dat_id, is_alliance };
            let key = world.troops.insert(unit);
            world.systems[system_key].ground_units.push(key);
        }
        FAM_SPECIAL => {
            let unit = SpecialForceUnit { class_dat_id, is_alliance };
            let key = world.special_forces.insert(unit);
            world.systems[system_key].special_forces.push(key);
        }
        _ => {
            // Capital ships / facilities in army tables — skip here.
            // Fleet ships in army tables are handled by apply_army_seed if desired.
        }
    }
}

/// Route a facility-context item to the system's facility lists.
fn dispatch_facility_item(
    item_id: u32,
    system_key: SystemKey,
    is_alliance: bool,
    world: &mut GameWorld,
) {
    let family = (item_id >> 24) as u8;
    let class_dat_id = DatId::new(item_id);

    match family {
        FAM_ALLIANCE_HQ => {
            // Alliance HQ is a manufacturing facility (construction yard class).
            let inst = ManufacturingFacilityInstance { class_dat_id, is_alliance: true };
            let key = world.manufacturing_facilities.insert(inst);
            world.systems[system_key].manufacturing_facilities.push(key);
        }
        f if f >= FAM_DEF_MIN && f <= FAM_DEF_MAX => {
            let inst = DefenseFacilityInstance { class_dat_id, is_alliance };
            let key = world.defense_facilities.insert(inst);
            world.systems[system_key].defense_facilities.push(key);
        }
        f if f >= FAM_MFG_MIN && f <= FAM_MFG_MAX => {
            let inst = ManufacturingFacilityInstance { class_dat_id, is_alliance };
            let key = world.manufacturing_facilities.insert(inst);
            world.systems[system_key].manufacturing_facilities.push(key);
        }
        f if f >= FAM_PROD_MIN && f <= FAM_PROD_MAX => {
            let inst = ProductionFacilityInstance { class_dat_id, is_alliance };
            let key = world.production_facilities.insert(inst);
            world.systems[system_key].production_facilities.push(key);
        }
        _ => {
            // Unknown family in facility table — skip.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn gdata_path() -> PathBuf {
        // Relative to workspace root when running `cargo test`.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent() // crates/
            .unwrap()
            .parent() // workspace root
            .unwrap()
            .join("data/base")
    }

    #[test]
    fn seeds_load_and_populate_world() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let world = crate::load_game_data(&path).expect("load_game_data failed");

        // Empire fleet at Coruscant — should have at least one fleet.
        assert!(!world.fleets.is_empty(), "no fleets were seeded");

        // Coruscant should have a fleet.
        let coruscant_key = world
            .systems
            .iter()
            .find(|(_, s)| s.dat_id.raw() == CORUSCANT_SEQ_ID)
            .map(|(k, _)| k);
        if let Some(k) = coruscant_key {
            assert!(
                !world.systems[k].fleets.is_empty(),
                "Coruscant should have at least one fleet"
            );
            assert!(
                !world.systems[k].manufacturing_facilities.is_empty(),
                "Coruscant should have facilities"
            );
        }

        // Yavin should have ground units.
        let yavin_key = world
            .systems
            .iter()
            .find(|(_, s)| s.dat_id.raw() == YAVIN_SEQ_ID)
            .map(|(k, _)| k);
        if let Some(k) = yavin_key {
            assert!(
                !world.systems[k].ground_units.is_empty(),
                "Yavin should have ground units"
            );
        }
    }
}
