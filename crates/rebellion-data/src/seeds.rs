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
//! # Placement logic (3-system model)
//!
//! The original game routes fixed seed tables to exactly 3 special systems:
//!
//! - **Coruscant** (Empire HQ): CMUNEFTB (Empire fleet), CMUNCRTB (garrison),
//!   FACLCRTB (facilities). Palpatine + Vader placed here.
//! - **Yavin** (Alliance base): CMUNAFTB group 1 (fleet), CMUNYVTB (garrison).
//!   Luke, Leia, Han, Wedge, Chewbacca, Jan Dodonna placed here.
//! - **Rebel HQ** (random rim system, NOT Yavin): CMUNAFTB group 2 (fleet),
//!   CMUNHQTB (garrison), FACLHQTB (facilities). Mon Mothma placed here.
//!
//! Army tables (CMUNEMTB, CMUNALTB) are distributed across the faction's
//! special systems.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use dat_dumper::types::seed_table::SeedTableFile;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
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

/// Number of starting systems for Empire (HQ + 9 nearest).
/// Used by the legacy proximity model; kept for backward compatibility.
const EMPIRE_STARTING_SYSTEMS: usize = 10;
/// Number of starting systems for Alliance (HQ + 2 nearest).
/// Used by the legacy proximity model; kept for backward compatibility.
const ALLIANCE_STARTING_SYSTEMS: usize = 3;

/// The three special systems that receive fixed seed table assets.
#[derive(Debug, Clone, Copy)]
pub struct SpecialSystems {
    /// Coruscant — Empire HQ. Receives CMUNEFTB, CMUNCRTB, FACLCRTB.
    pub coruscant: SystemKey,
    /// Yavin — Alliance base. Receives CMUNAFTB group 1, CMUNYVTB.
    pub yavin: SystemKey,
    /// Rebel HQ — random rim system (not Yavin). Receives CMUNAFTB group 2,
    /// CMUNHQTB, FACLHQTB. Mon Mothma placed here.
    pub rebel_hq: SystemKey,
    /// The sequential DatId of the rebel HQ system (for lookup purposes).
    pub rebel_hq_seq_id: u32,
}

/// Select the 3 special systems for seeding: Coruscant, Yavin, and a random
/// rim system as the Rebel HQ.
///
/// The Rebel HQ is chosen by shuffling all rim systems (RimInner + RimOuter)
/// and picking the first one that is not Yavin. This matches the original
/// game's behavior of selecting the first eligible rim system from a shuffled
/// system list.
fn select_special_systems<R: Rng + ?Sized>(
    world: &GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
    rng: &mut R,
) -> Option<SpecialSystems> {
    let coruscant_key = system_key_map.get(&CORUSCANT_SEQ_ID).copied()?;
    let yavin_key = system_key_map.get(&YAVIN_SEQ_ID).copied()?;

    // Collect all rim systems (RimInner + RimOuter) that are not Yavin.
    // Sort by seq_id first to ensure deterministic ordering before the shuffle,
    // since HashMap iteration order is non-deterministic.
    let mut rim_candidates: Vec<(u32, SystemKey)> = system_key_map
        .iter()
        .filter_map(|(&seq_id, &sys_key)| {
            if seq_id == YAVIN_SEQ_ID {
                return None; // Yavin is the Alliance base, not eligible for HQ
            }
            let sys = world.systems.get(sys_key)?;
            let sector = world.sectors.get(sys.sector)?;
            match sector.group {
                SectorGroup::RimInner | SectorGroup::RimOuter => Some((seq_id, sys_key)),
                SectorGroup::Core => None,
            }
        })
        .collect();

    // Sort by seq_id for deterministic ordering before shuffle.
    rim_candidates.sort_by_key(|(seq_id, _)| *seq_id);
    rim_candidates.shuffle(rng);

    let (rebel_hq_seq_id, rebel_hq_key) = rim_candidates.first().copied()?;

    Some(SpecialSystems {
        coruscant: coruscant_key,
        yavin: yavin_key,
        rebel_hq: rebel_hq_key,
        rebel_hq_seq_id,
    })
}

/// Mark the 3 special systems as populated, charted, and fully controlled
/// by their respective factions with 100% support.
fn initialize_special_systems(world: &mut GameWorld, special: &SpecialSystems) {
    // Coruscant — Empire HQ
    if let Some(sys) = world.systems.get_mut(special.coruscant) {
        sys.is_populated = true;
        sys.exploration_status = ExplorationStatus::Explored;
        sys.control = ControlKind::Controlled(Faction::Empire);
        sys.popularity_empire = 1.0;
        sys.popularity_alliance = 0.0;
        sys.is_headquarters = true;
    }

    // Yavin — Alliance base
    if let Some(sys) = world.systems.get_mut(special.yavin) {
        sys.is_populated = true;
        sys.exploration_status = ExplorationStatus::Explored;
        sys.control = ControlKind::Controlled(Faction::Alliance);
        sys.popularity_alliance = 1.0;
        sys.popularity_empire = 0.0;
        sys.is_headquarters = true;
    }

    // Rebel HQ — Alliance headquarters (separate from Yavin)
    if let Some(sys) = world.systems.get_mut(special.rebel_hq) {
        sys.is_populated = true;
        sys.exploration_status = ExplorationStatus::Explored;
        sys.control = ControlKind::Controlled(Faction::Alliance);
        sys.popularity_alliance = 1.0;
        sys.popularity_empire = 0.0;
        sys.is_headquarters = true;
    }
}

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

/// Select starting systems for each faction based on proximity to their HQ.
///
/// Empire: Coruscant + N nearest systems (EMPIRE_STARTING_SYSTEMS total).
/// Alliance: Yavin + N nearest systems (ALLIANCE_STARTING_SYSTEMS total),
/// excluding systems already claimed by the Empire.
///
/// Returns `(empire_systems, alliance_systems)` as sequential DatId arrays
/// suitable for passing as `system_rotation` to the seed functions.
pub fn select_starting_systems(
    world: &GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
) -> (Vec<u32>, Vec<u32>) {
    // Get HQ coordinates
    let coruscant_key = system_key_map.get(&CORUSCANT_SEQ_ID).copied();
    let yavin_key = system_key_map.get(&YAVIN_SEQ_ID).copied();

    let coruscant_pos = coruscant_key
        .and_then(|k| world.systems.get(k))
        .map(|s| (s.x as f64, s.y as f64))
        .unwrap_or((500.0, 500.0));
    let yavin_pos = yavin_key
        .and_then(|k| world.systems.get(k))
        .map(|s| (s.x as f64, s.y as f64))
        .unwrap_or((100.0, 200.0));

    // Sort all systems by distance to Coruscant
    let mut empire_candidates: Vec<(u32, f64)> = system_key_map
        .iter()
        .filter_map(|(&seq, &key)| {
            let sys = world.systems.get(key)?;
            let dx = sys.x as f64 - coruscant_pos.0;
            let dy = sys.y as f64 - coruscant_pos.1;
            Some((seq, (dx * dx + dy * dy).sqrt()))
        })
        .collect();
    empire_candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let empire_systems: Vec<u32> = empire_candidates
        .iter()
        .take(EMPIRE_STARTING_SYSTEMS)
        .map(|(seq, _)| *seq)
        .collect();

    // Alliance: nearest to Yavin, excluding Empire starting systems
    let empire_set: std::collections::HashSet<u32> = empire_systems.iter().copied().collect();
    let mut alliance_candidates: Vec<(u32, f64)> = system_key_map
        .iter()
        .filter_map(|(&seq, &key)| {
            if empire_set.contains(&seq) {
                return None;
            }
            let sys = world.systems.get(key)?;
            let dx = sys.x as f64 - yavin_pos.0;
            let dy = sys.y as f64 - yavin_pos.1;
            Some((seq, (dx * dx + dy * dy).sqrt()))
        })
        .collect();
    alliance_candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let alliance_systems: Vec<u32> = alliance_candidates
        .iter()
        .take(ALLIANCE_STARTING_SYSTEMS)
        .map(|(seq, _)| *seq)
        .collect();

    (empire_systems, alliance_systems)
}

/// Apply all 9 seed tables to the already-populated `world`.
///
/// Missing files are silently ignored (stripped installs, test environments).
/// Unresolvable item_ids are silently skipped with a debug log rather than
/// aborting — the game can start in a degraded state if needed.
pub fn apply_seeds(
    gdata_path: &Path,
    world: &mut GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
    seed_options: &SeedOptions,
) -> anyhow::Result<()> {
    let mut rng = make_seed_rng(seed_options);
    apply_seeds_with_rng(gdata_path, world, system_key_map, seed_options, &mut rng)
}

/// Apply all seed tables with an injected RNG for deterministic tests.
///
/// Uses the 3-system model: Coruscant (Empire HQ), Yavin (Alliance base),
/// and a random rim system as the Rebel HQ. Fixed seed tables are routed
/// only to these 3 systems, matching the original 1998 game behavior.
pub fn apply_seeds_with_rng<R: Rng + ?Sized>(
    gdata_path: &Path,
    world: &mut GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
    _seed_options: &SeedOptions,
    rng: &mut R,
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

    // Select the 3 special systems. If Coruscant or Yavin are missing from the
    // data files, fall back to the legacy proximity model.
    let special = match select_special_systems(world, system_key_map, rng) {
        Some(s) => s,
        None => {
            // Fallback: use old proximity model if special systems can't be resolved.
            // This should only happen with incomplete data files.
            return apply_seeds_legacy(gdata_path, world, system_key_map, &capship_index, &fighter_index);
        }
    };

    // Mark special systems as populated, charted, and controlled.
    initialize_special_systems(world, &special);

    // ── Empire fleet (CMUNEFTB) → Coruscant only ─────────────────────────────
    apply_fleet_seed(
        &load_seed(gdata_path, "CMUNEFTB.DAT")?,
        system_key_map,
        &capship_index,
        &fighter_index,
        &[CORUSCANT_SEQ_ID],
        false,
        world,
    );

    // ── Alliance fleet (CMUNAFTB) → group 1 at Yavin, group 2 at Rebel HQ ──
    apply_fleet_seed(
        &load_seed(gdata_path, "CMUNAFTB.DAT")?,
        system_key_map,
        &capship_index,
        &fighter_index,
        &[YAVIN_SEQ_ID, special.rebel_hq_seq_id],
        true,
        world,
    );

    // ── Empire army (CMUNEMTB) → Coruscant only ─────────────────────────────
    apply_army_seed(
        &load_seed(gdata_path, "CMUNEMTB.DAT")?,
        system_key_map,
        &[CORUSCANT_SEQ_ID],
        false,
        world,
    );

    // ── Alliance army (CMUNALTB) → Yavin + Rebel HQ ────────────────────────
    apply_army_seed(
        &load_seed(gdata_path, "CMUNALTB.DAT")?,
        system_key_map,
        &[YAVIN_SEQ_ID, special.rebel_hq_seq_id],
        true,
        world,
    );

    // ── Empire Coruscant garrison (CMUNCRTB) → Coruscant ────────────────────
    apply_garrison_seed(
        &load_seed(gdata_path, "CMUNCRTB.DAT")?,
        system_key_map,
        CORUSCANT_SEQ_ID,
        false,
        world,
    );

    // ── Alliance HQ garrison (CMUNHQTB) → Rebel HQ (NOT Yavin) ─────────────
    apply_garrison_seed(
        &load_seed(gdata_path, "CMUNHQTB.DAT")?,
        system_key_map,
        special.rebel_hq_seq_id,
        true,
        world,
    );

    // ── Alliance Yavin garrison (CMUNYVTB) → Yavin ──────────────────────────
    apply_garrison_seed(
        &load_seed(gdata_path, "CMUNYVTB.DAT")?,
        system_key_map,
        YAVIN_SEQ_ID,
        true,
        world,
    );

    // ── Empire Coruscant facilities (FACLCRTB) → Coruscant ──────────────────
    apply_facility_seed(
        &load_seed(gdata_path, "FACLCRTB.DAT")?,
        system_key_map,
        CORUSCANT_SEQ_ID,
        false,
        world,
    );

    // ── Alliance HQ facilities (FACLHQTB) → Rebel HQ (NOT Yavin) ───────────
    apply_facility_seed(
        &load_seed(gdata_path, "FACLHQTB.DAT")?,
        system_key_map,
        special.rebel_hq_seq_id,
        true,
        world,
    );

    // ── Character stat rolling + placement ──────────────────────────────────
    roll_character_stats(world, rng);
    place_named_characters(world, &special);

    Ok(())
}

/// Legacy seed application using proximity-based fleet spread.
/// Used as fallback when Coruscant or Yavin can't be found in the data.
fn apply_seeds_legacy(
    gdata_path: &Path,
    world: &mut GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
    capship_index: &CapShipIndex,
    fighter_index: &FighterIndex,
) -> anyhow::Result<()> {
    let (empire_systems, alliance_systems) = select_starting_systems(world, system_key_map);

    apply_fleet_seed(&load_seed(gdata_path, "CMUNEFTB.DAT")?, system_key_map,
        capship_index, fighter_index, &empire_systems, false, world);
    apply_fleet_seed(&load_seed(gdata_path, "CMUNAFTB.DAT")?, system_key_map,
        capship_index, fighter_index, &alliance_systems, true, world);
    apply_army_seed(&load_seed(gdata_path, "CMUNEMTB.DAT")?, system_key_map,
        &empire_systems, false, world);
    apply_army_seed(&load_seed(gdata_path, "CMUNALTB.DAT")?, system_key_map,
        &alliance_systems, true, world);
    apply_garrison_seed(&load_seed(gdata_path, "CMUNCRTB.DAT")?, system_key_map,
        CORUSCANT_SEQ_ID, false, world);
    apply_garrison_seed(&load_seed(gdata_path, "CMUNHQTB.DAT")?, system_key_map,
        YAVIN_SEQ_ID, true, world);
    apply_garrison_seed(&load_seed(gdata_path, "CMUNYVTB.DAT")?, system_key_map,
        YAVIN_SEQ_ID, true, world);
    apply_facility_seed(&load_seed(gdata_path, "FACLCRTB.DAT")?, system_key_map,
        CORUSCANT_SEQ_ID, false, world);
    apply_facility_seed(&load_seed(gdata_path, "FACLHQTB.DAT")?, system_key_map,
        YAVIN_SEQ_ID, true, world);

    Ok(())
}

/// Apply seed tables from pre-loaded file bytes. Same logic as `apply_seeds` but
/// reads from a HashMap instead of the filesystem.
pub fn apply_seeds_from_files(
    files: &std::collections::HashMap<String, Vec<u8>>,
    world: &mut GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
    seed_options: &SeedOptions,
) -> anyhow::Result<()> {
    let mut rng = make_seed_rng(seed_options);
    apply_seeds_from_files_with_rng(files, world, system_key_map, seed_options, &mut rng)
}

/// Apply seed tables from pre-loaded file bytes with an injected RNG.
pub fn apply_seeds_from_files_with_rng<R: Rng + ?Sized>(
    files: &std::collections::HashMap<String, Vec<u8>>,
    world: &mut GameWorld,
    system_key_map: &HashMap<u32, SystemKey>,
    _seed_options: &SeedOptions,
    rng: &mut R,
) -> anyhow::Result<()> {
    let capship_index: CapShipIndex = world.capital_ship_classes.iter()
        .map(|(k, v)| (v.dat_id.raw(), k)).collect();
    let fighter_index: FighterIndex = world.fighter_classes.iter()
        .map(|(k, v)| (v.dat_id.raw(), k)).collect();

    // Use 3-system model when possible.
    let special = match select_special_systems(world, system_key_map, rng) {
        Some(s) => s,
        None => {
            // Fallback to proximity model for incomplete data
            let (empire_systems, alliance_systems) = select_starting_systems(world, system_key_map);
            apply_fleet_seed(&load_seed_from_files(files, "CMUNEFTB.DAT")?, system_key_map,
                &capship_index, &fighter_index, &empire_systems, false, world);
            apply_fleet_seed(&load_seed_from_files(files, "CMUNAFTB.DAT")?, system_key_map,
                &capship_index, &fighter_index, &alliance_systems, true, world);
            apply_army_seed(&load_seed_from_files(files, "CMUNEMTB.DAT")?, system_key_map,
                &empire_systems, false, world);
            apply_army_seed(&load_seed_from_files(files, "CMUNALTB.DAT")?, system_key_map,
                &alliance_systems, true, world);
            apply_garrison_seed(&load_seed_from_files(files, "CMUNCRTB.DAT")?, system_key_map,
                CORUSCANT_SEQ_ID, false, world);
            apply_garrison_seed(&load_seed_from_files(files, "CMUNHQTB.DAT")?, system_key_map,
                YAVIN_SEQ_ID, true, world);
            apply_garrison_seed(&load_seed_from_files(files, "CMUNYVTB.DAT")?, system_key_map,
                YAVIN_SEQ_ID, true, world);
            apply_facility_seed(&load_seed_from_files(files, "FACLCRTB.DAT")?, system_key_map,
                CORUSCANT_SEQ_ID, false, world);
            apply_facility_seed(&load_seed_from_files(files, "FACLHQTB.DAT")?, system_key_map,
                YAVIN_SEQ_ID, true, world);
            return Ok(());
        }
    };

    initialize_special_systems(world, &special);

    apply_fleet_seed(&load_seed_from_files(files, "CMUNEFTB.DAT")?, system_key_map,
        &capship_index, &fighter_index, &[CORUSCANT_SEQ_ID], false, world);
    apply_fleet_seed(&load_seed_from_files(files, "CMUNAFTB.DAT")?, system_key_map,
        &capship_index, &fighter_index, &[YAVIN_SEQ_ID, special.rebel_hq_seq_id], true, world);
    apply_army_seed(&load_seed_from_files(files, "CMUNEMTB.DAT")?, system_key_map,
        &[CORUSCANT_SEQ_ID], false, world);
    apply_army_seed(&load_seed_from_files(files, "CMUNALTB.DAT")?, system_key_map,
        &[YAVIN_SEQ_ID, special.rebel_hq_seq_id], true, world);
    apply_garrison_seed(&load_seed_from_files(files, "CMUNCRTB.DAT")?, system_key_map,
        CORUSCANT_SEQ_ID, false, world);
    apply_garrison_seed(&load_seed_from_files(files, "CMUNHQTB.DAT")?, system_key_map,
        special.rebel_hq_seq_id, true, world);
    apply_garrison_seed(&load_seed_from_files(files, "CMUNYVTB.DAT")?, system_key_map,
        YAVIN_SEQ_ID, true, world);
    apply_facility_seed(&load_seed_from_files(files, "FACLCRTB.DAT")?, system_key_map,
        CORUSCANT_SEQ_ID, false, world);
    apply_facility_seed(&load_seed_from_files(files, "FACLHQTB.DAT")?, system_key_map,
        special.rebel_hq_seq_id, true, world);

    roll_character_stats(world, rng);
    place_named_characters(world, &special);

    Ok(())
}

fn make_seed_rng(seed_options: &SeedOptions) -> Xoshiro256PlusPlus {
    Xoshiro256PlusPlus::seed_from_u64(seed_options.rng_seed.unwrap_or_else(default_seed))
}

fn default_seed() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0)
    }
    #[cfg(target_arch = "wasm32")]
    {
        0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Load a seed table file, returning `Ok(None)` if the file doesn't exist.
fn load_seed(gdata_path: &Path, filename: &str) -> anyhow::Result<Option<SeedTableFile>> {
    let path = gdata_path.join(filename);
    if !crate::file_available(&path) {
        return Ok(None);
    }
    let file: SeedTableFile = read_dat_file(&path)
        .with_context(|| format!("parsing seed table {}", filename))?;
    Ok(Some(file))
}

/// Load a seed table file from pre-loaded bytes, returning `Ok(None)` if not in the map.
fn load_seed_from_files(files: &std::collections::HashMap<String, Vec<u8>>, filename: &str) -> anyhow::Result<Option<SeedTableFile>> {
    match files.get(filename) {
        Some(data) => {
            let file: SeedTableFile = crate::parse_dat_bytes(data, filename)?;
            Ok(Some(file))
        }
        None => Ok(None),
    }
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
            has_death_star: false,
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
            let unit = TroopUnit { class_dat_id, is_alliance, regiment_strength: 100 };
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

// ─────────────────────────────────────────────────────────────────────────────
// M4: Character stat rolling + named character placement
// ─────────────────────────────────────────────────────────────────────────────

/// Roll concrete stats from SkillPair{base, variance} for all characters.
///
/// After rolling, each SkillPair is normalized: `base = rolled_value`, `variance = 0`.
/// This matches the original game's `setup_character()` which generates concrete
/// stats at game start from the DAT base+variance templates.
///
/// Jedi probability is also rolled here: characters with `jedi_probability > 0`
/// have a chance to become Force-sensitive based on their probability value.
fn roll_character_stats<R: Rng + ?Sized>(world: &mut GameWorld, rng: &mut R) {
    for (_, character) in world.characters.iter_mut() {
        roll_skill_pair(&mut character.diplomacy, rng);
        roll_skill_pair(&mut character.espionage, rng);
        roll_skill_pair(&mut character.ship_design, rng);
        roll_skill_pair(&mut character.troop_training, rng);
        roll_skill_pair(&mut character.facility_design, rng);
        roll_skill_pair(&mut character.combat, rng);
        roll_skill_pair(&mut character.leadership, rng);
        roll_skill_pair(&mut character.loyalty, rng);

        // Roll Jedi level
        roll_skill_pair(&mut character.jedi_level, rng);

        // Roll Jedi probability: if the character isn't already a known Jedi,
        // check if they become Force-sensitive.
        if !character.is_known_jedi && character.jedi_probability > 0 {
            let roll: u32 = rng.gen_range(0..100);
            if roll < character.jedi_probability {
                character.force_tier = ForceTier::Aware;
            }
        }
    }
}

/// Roll a single SkillPair into a concrete value: base + random(0..=variance).
/// After rolling, variance is set to 0 so the value is fixed.
fn roll_skill_pair<R: Rng + ?Sized>(pair: &mut SkillPair, rng: &mut R) {
    if pair.variance > 0 {
        let bonus: u32 = rng.gen_range(0..=pair.variance);
        pair.base += bonus;
        pair.variance = 0;
    }
}

/// Place named characters at their starting systems.
///
/// Per the original game:
/// - Luke Skywalker, Princess Leia, Han Solo, Wedge Antilles, Chewbacca,
///   and Jan Dodonna are placed at Yavin.
/// - Mon Mothma is placed at the Rebel HQ (random rim system).
/// - Emperor Palpatine and Darth Vader are placed at Coruscant.
fn place_named_characters(world: &mut GameWorld, special: &SpecialSystems) {
    // Alliance characters at Yavin
    const YAVIN_CHARACTERS: &[&str] = &[
        "Luke Skywalker",
        "Princess Leia",
        "Han Solo",
        "Wedge Antilles",
        "Chewbacca",
        "Jan Dodonna",
    ];

    // Alliance character at Rebel HQ
    const REBEL_HQ_CHARACTERS: &[&str] = &["Mon Mothma"];

    // Empire characters at Coruscant
    const CORUSCANT_CHARACTERS: &[&str] = &[
        "Emperor Palpatine",
        "Darth Vader",
    ];

    for (_, character) in world.characters.iter_mut() {
        let name = character.name.as_str();

        if YAVIN_CHARACTERS.iter().any(|&n| name == n) {
            character.current_system = Some(special.yavin);
        } else if REBEL_HQ_CHARACTERS.iter().any(|&n| name == n) {
            character.current_system = Some(special.rebel_hq);
        } else if CORUSCANT_CHARACTERS.iter().any(|&n| name == n) {
            character.current_system = Some(special.coruscant);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

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

    // ── M3: Special system tests ────────────────────────────────────────────

    #[test]
    fn special_systems_match_original_roles() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let seed_options = SeedOptions {
            rng_seed: Some(42),
            ..SeedOptions::default()
        };
        let world = crate::load_game_data_with_options(&path, &seed_options)
            .expect("load_game_data_with_options failed");

        // Coruscant: Empire-controlled, populated, charted, has fleet + facilities
        let coruscant = world.systems.iter()
            .find(|(_, s)| s.dat_id.raw() == CORUSCANT_SEQ_ID)
            .map(|(k, s)| (k, s));
        assert!(coruscant.is_some(), "Coruscant must exist");
        let (_ck, cs) = coruscant.unwrap();
        assert!(cs.is_populated, "Coruscant must be populated");
        assert_eq!(cs.exploration_status, ExplorationStatus::Explored, "Coruscant must be charted");
        assert!(cs.control.is_controlled_by(Faction::Empire), "Coruscant must be Empire-controlled");
        assert!(!cs.fleets.is_empty(), "Coruscant must have fleets (CMUNEFTB)");
        assert!(!cs.manufacturing_facilities.is_empty(), "Coruscant must have facilities (FACLCRTB)");

        // Yavin: Alliance-controlled, populated, charted, has ground units
        let yavin = world.systems.iter()
            .find(|(_, s)| s.dat_id.raw() == YAVIN_SEQ_ID)
            .map(|(k, s)| (k, s));
        assert!(yavin.is_some(), "Yavin must exist");
        let (yk, ys) = yavin.unwrap();
        assert!(ys.is_populated, "Yavin must be populated");
        assert_eq!(ys.exploration_status, ExplorationStatus::Explored, "Yavin must be charted");
        assert!(ys.control.is_controlled_by(Faction::Alliance), "Yavin must be Alliance-controlled");
        assert!(!ys.ground_units.is_empty(), "Yavin must have ground units (CMUNYVTB)");

        // Rebel HQ: must exist, be Alliance-controlled, populated, charted,
        // and be a DIFFERENT system from Yavin.
        // Find it: it's the Alliance HQ system that is NOT Yavin.
        let rebel_hq = world.systems.iter()
            .find(|(k, s)| {
                s.is_headquarters
                    && s.control.is_controlled_by(Faction::Alliance)
                    && s.dat_id.raw() != YAVIN_SEQ_ID
                    && *k != yk
            });
        assert!(rebel_hq.is_some(), "Rebel HQ must be a separate system from Yavin");
        let (_rhk, rhs) = rebel_hq.unwrap();
        assert!(rhs.is_populated, "Rebel HQ must be populated");
        assert_eq!(rhs.exploration_status, ExplorationStatus::Explored, "Rebel HQ must be charted");

        // Rebel HQ must be a rim system (not core)
        let hq_sector = world.sectors.get(rhs.sector);
        assert!(hq_sector.is_some(), "Rebel HQ sector must exist");
        let group = hq_sector.unwrap().group;
        assert!(
            group == SectorGroup::RimInner || group == SectorGroup::RimOuter,
            "Rebel HQ must be a rim system, got {:?}", group
        );

        // FACLHQTB should be at Rebel HQ, NOT at Yavin
        assert!(
            !rhs.manufacturing_facilities.is_empty() || !rhs.defense_facilities.is_empty()
                || !rhs.production_facilities.is_empty(),
            "Rebel HQ should have facilities from FACLHQTB"
        );
    }

    #[test]
    fn fixed_fleet_tables_only_target_special_systems() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let seed_options = SeedOptions {
            rng_seed: Some(42),
            ..SeedOptions::default()
        };
        let world = crate::load_game_data_with_options(&path, &seed_options)
            .expect("load_game_data_with_options failed");

        // Collect all systems that have fleets
        let systems_with_fleets: Vec<_> = world.systems.iter()
            .filter(|(_, s)| !s.fleets.is_empty())
            .map(|(_, s)| s.dat_id.raw())
            .collect();

        // With fixed seed tables only, fleets should appear at special systems only
        // (Coruscant, Yavin, or Rebel HQ). The Rebel HQ seq_id varies by RNG seed,
        // but Coruscant and Yavin should always be present.
        assert!(
            systems_with_fleets.contains(&CORUSCANT_SEQ_ID),
            "Coruscant should have fleet(s)"
        );

        // Empire fleets should NOT be at more than ~3 systems (was 10 in old proximity model)
        let empire_fleet_system_count = world.systems.iter()
            .filter(|(_, s)| {
                s.fleets.iter().any(|&fk| {
                    world.fleets.get(fk).map_or(false, |f| !f.is_alliance)
                })
            })
            .count();
        assert!(
            empire_fleet_system_count <= 3,
            "Empire fleets should be at <= 3 systems (3-system model), found {}",
            empire_fleet_system_count
        );
    }

    #[test]
    fn deterministic_rebel_hq_with_same_seed() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let seed_options = SeedOptions {
            rng_seed: Some(12345),
            ..SeedOptions::default()
        };

        let world1 = crate::load_game_data_with_options(&path, &seed_options)
            .expect("first load failed");
        let world2 = crate::load_game_data_with_options(&path, &seed_options)
            .expect("second load failed");

        // Find Rebel HQ in both worlds
        let find_rebel_hq = |world: &GameWorld| -> Option<u32> {
            world.systems.iter()
                .find(|(_, s)| {
                    s.is_headquarters
                        && s.control.is_controlled_by(Faction::Alliance)
                        && s.dat_id.raw() != YAVIN_SEQ_ID
                })
                .map(|(_, s)| s.dat_id.raw())
        };

        let hq1 = find_rebel_hq(&world1);
        let hq2 = find_rebel_hq(&world2);
        assert!(hq1.is_some(), "Rebel HQ should exist in world 1");
        assert_eq!(hq1, hq2, "Same RNG seed must produce same Rebel HQ system");
    }

    // ── M4: Character stat rolling + placement tests ────────────────────────

    #[test]
    fn fixed_named_characters_spawn_at_expected_systems() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let seed_options = SeedOptions {
            rng_seed: Some(42),
            ..SeedOptions::default()
        };
        let world = crate::load_game_data_with_options(&path, &seed_options)
            .expect("load_game_data_with_options failed");

        // Find system keys
        let coruscant_key = world.systems.iter()
            .find(|(_, s)| s.dat_id.raw() == CORUSCANT_SEQ_ID)
            .map(|(k, _)| k);
        let yavin_key = world.systems.iter()
            .find(|(_, s)| s.dat_id.raw() == YAVIN_SEQ_ID)
            .map(|(k, _)| k);
        let rebel_hq_key = world.systems.iter()
            .find(|(_, s)| {
                s.is_headquarters
                    && s.control.is_controlled_by(Faction::Alliance)
                    && s.dat_id.raw() != YAVIN_SEQ_ID
            })
            .map(|(k, _)| k);

        assert!(coruscant_key.is_some(), "Coruscant must exist");
        assert!(yavin_key.is_some(), "Yavin must exist");
        assert!(rebel_hq_key.is_some(), "Rebel HQ must exist");

        // Check Alliance characters at Yavin
        for name in &["Luke Skywalker", "Princess Leia", "Han Solo", "Wedge Antilles",
                       "Chewbacca", "Jan Dodonna"] {
            let found = world.characters.iter()
                .find(|(_, c)| c.name == *name);
            if let Some((_, ch)) = found {
                assert_eq!(
                    ch.current_system, yavin_key,
                    "{} should be at Yavin, but is at {:?}", name, ch.current_system
                );
            }
            // If the name isn't found, that's OK (TEXTSTRA.DLL may use different names)
        }

        // Check Mon Mothma at Rebel HQ
        if let Some((_, mm)) = world.characters.iter().find(|(_, c)| c.name == "Mon Mothma") {
            assert_eq!(
                mm.current_system, rebel_hq_key,
                "Mon Mothma should be at Rebel HQ"
            );
        }

        // Check Empire characters at Coruscant
        for name in &["Emperor Palpatine", "Darth Vader"] {
            if let Some((_, ch)) = world.characters.iter().find(|(_, c)| c.name == *name) {
                assert_eq!(
                    ch.current_system, coruscant_key,
                    "{} should be at Coruscant, but is at {:?}", name, ch.current_system
                );
            }
        }
    }

    #[test]
    fn character_stats_are_rolled_not_raw() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let seed_options = SeedOptions {
            rng_seed: Some(42),
            ..SeedOptions::default()
        };
        let world = crate::load_game_data_with_options(&path, &seed_options)
            .expect("load_game_data_with_options failed");

        // After rolling, all character skill variances should be 0
        for (_, character) in world.characters.iter() {
            assert_eq!(character.diplomacy.variance, 0,
                "Character {} diplomacy variance should be 0 after rolling", character.name);
            assert_eq!(character.espionage.variance, 0,
                "Character {} espionage variance should be 0 after rolling", character.name);
            assert_eq!(character.combat.variance, 0,
                "Character {} combat variance should be 0 after rolling", character.name);
            assert_eq!(character.leadership.variance, 0,
                "Character {} leadership variance should be 0 after rolling", character.name);
            assert_eq!(character.loyalty.variance, 0,
                "Character {} loyalty variance should be 0 after rolling", character.name);
        }
    }

    #[test]
    fn deterministic_stat_rolls_with_same_seed() {
        let path = gdata_path();
        if !path.exists() {
            eprintln!("skipping: data/base not found at {:?}", path);
            return;
        }
        let seed_options = SeedOptions {
            rng_seed: Some(99999),
            ..SeedOptions::default()
        };

        let world1 = crate::load_game_data_with_options(&path, &seed_options)
            .expect("first load failed");
        let world2 = crate::load_game_data_with_options(&path, &seed_options)
            .expect("second load failed");

        // Same seed must produce identical character stats
        for ((_, c1), (_, c2)) in world1.characters.iter().zip(world2.characters.iter()) {
            assert_eq!(c1.name, c2.name, "Character order must be deterministic");
            assert_eq!(c1.diplomacy.base, c2.diplomacy.base,
                "Character {} diplomacy should be deterministic", c1.name);
            assert_eq!(c1.combat.base, c2.combat.base,
                "Character {} combat should be deterministic", c1.name);
            assert_eq!(c1.leadership.base, c2.leadership.base,
                "Character {} leadership should be deterministic", c1.name);
        }
    }

    #[test]
    fn roll_skill_pair_unit() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let mut pair = SkillPair { base: 50, variance: 20 };
        roll_skill_pair(&mut pair, &mut rng);
        assert!(pair.base >= 50 && pair.base <= 70, "rolled base should be in [50, 70], got {}", pair.base);
        assert_eq!(pair.variance, 0, "variance should be zeroed after rolling");
    }

    #[test]
    fn roll_skill_pair_zero_variance() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let mut pair = SkillPair { base: 100, variance: 0 };
        roll_skill_pair(&mut pair, &mut rng);
        assert_eq!(pair.base, 100, "zero-variance pair should keep original base");
        assert_eq!(pair.variance, 0);
    }
}
