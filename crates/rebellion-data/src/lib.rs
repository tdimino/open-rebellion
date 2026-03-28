//! Data loading: converts .DAT binary structs into rebellion-core world types.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use dat_dumper::codec::ByteReader;
use dat_dumper::dat_record::DatRecord;
use dat_dumper::types::capital_ships::CapitalShipsFile;
use dat_dumper::types::defense_facilities::DefenseFacilitiesFile;
use dat_dumper::types::fighters::FightersFile;
use dat_dumper::types::general_params::GeneralParamsFile;
use dat_dumper::types::int_table::IntTableFile;
use dat_dumper::types::major_characters::{CharacterEntry, MajorCharactersFile};
use dat_dumper::types::minor_characters::MinorCharactersFile;
use dat_dumper::types::sectors::SectorsFile;
use dat_dumper::types::side_params::SideParamsFile;
use dat_dumper::types::systems::SystemsFile;
use dat_dumper::types::troops::TroopsFile;
#[cfg(not(target_arch = "wasm32"))]
use dat_dumper::types::textstra;
use rebellion_core::dat::{ExplorationStatus, SectorGroup};
use rebellion_core::ids::*;
use rebellion_core::world::*;

pub mod integrator;
pub mod mods;
pub mod save;
pub mod seeds;
pub mod simulation;

/// Load all game data from a GData directory into a GameWorld.
///
/// Expected files under `gdata_path`:
/// - `TEXTSTRA.DLL` (optional: if absent, placeholder names are used)
/// - `SECTORSD.DAT`
/// - `SYSTEMSD.DAT`
/// - `CAPSHPSD.DAT`
/// - `FIGHTSD.DAT`
/// - `MJCHARSD.DAT`
/// - `MNCHARSD.DAT`
pub fn load_game_data(gdata_path: &Path) -> anyhow::Result<GameWorld> {
    load_game_data_with_options(gdata_path, &SeedOptions::default())
}

/// Load a new game with explicit seeding options.
pub fn load_game_data_with_options(
    gdata_path: &Path,
    seed_options: &SeedOptions,
) -> anyhow::Result<GameWorld> {
    // ── 0. String table ──────────────────────────────────────────────────────
    // Load TEXTSTRA.DLL for real entity names. Fall back to placeholders if
    // the file is absent (WASM builds, test environments, stripped installs).
    #[cfg(not(target_arch = "wasm32"))]
    let string_table: HashMap<u16, String> = {
        let textstra_path = gdata_path.join("TEXTSTRA.DLL");
        if textstra_path.exists() {
            textstra::load_strings(&textstra_path)
                .with_context(|| format!("loading strings from {}", textstra_path.display()))?
        } else {
            HashMap::new()
        }
    };
    #[cfg(target_arch = "wasm32")]
    let string_table: HashMap<u16, String> = WASM_STRING_TABLE.lock().unwrap().clone();

    // Convenience closure: look up a string by its DLL id, or fall back to
    // "<kind> <id>" if the DLL was absent or the id is not present.
    let lookup = |id: u16, kind: &str| -> String {
        string_table
            .get(&id)
            .cloned()
            .unwrap_or_else(|| format!("{} {}", kind, id))
    };

    let mut world = GameWorld {
        systems: slotmap::SlotMap::with_key(),
        sectors: slotmap::SlotMap::with_key(),
        capital_ship_classes: slotmap::SlotMap::with_key(),
        fighter_classes: slotmap::SlotMap::with_key(),
        characters: slotmap::SlotMap::with_key(),
        fleets: slotmap::SlotMap::with_key(),
        troops: slotmap::SlotMap::with_key(),
        special_forces: slotmap::SlotMap::with_key(),
        defense_facilities: slotmap::SlotMap::with_key(),
        manufacturing_facilities: slotmap::SlotMap::with_key(),
        production_facilities: slotmap::SlotMap::with_key(),
        gnprtb: GnprtbParams::default(),
        sdprtb: SdprtbParams::default(),
        mission_tables: std::collections::HashMap::new(),
        troop_classes: std::collections::HashMap::new(),
        defense_facility_classes: std::collections::HashMap::new(),
        difficulty_index: seed_options.gnprtb_index(),
    };

    // ── 1. Sectors ───────────────────────────────────────────────────────────
    // Must come first: systems reference sectors by dat id.
    let sectors_file: SectorsFile = read_dat_file(&gdata_path.join("SECTORSD.DAT"))?;

    // sector dat id → SectorKey (populated after inserting into the arena)
    let mut sector_key_map: HashMap<u32, SectorKey> = HashMap::with_capacity(sectors_file.sectors.len());

    for dat in &sectors_file.sectors {
        let group = match dat.group {
            1 => SectorGroup::Core,
            2 => SectorGroup::RimInner,
            3 => SectorGroup::RimOuter,
            other => anyhow::bail!("sector {} has unknown group value {}", dat.id, other),
        };
        let sector = Sector {
            dat_id: DatId::new(dat.id),
            name: lookup(dat.text_stra_dll_id, "Sector"),
            group,
            x: dat.x_position,
            y: dat.y_position,
            systems: Vec::new(), // filled in during the systems pass below
        };
        let key = world.sectors.insert(sector);
        sector_key_map.insert(dat.id, key);
    }

    // ── 2. Systems ───────────────────────────────────────────────────────────
    let systems_file: SystemsFile = read_dat_file(&gdata_path.join("SYSTEMSD.DAT"))?;

    // system dat id → SystemKey (stored for callers that need cross-referencing later)
    let mut system_key_map: HashMap<u32, SystemKey> = HashMap::with_capacity(systems_file.systems.len());

    for dat in &systems_file.systems {
        let sector_key = *sector_key_map
            .get(&dat.sector_id)
            .with_context(|| format!(
                "system {} references unknown sector_id {}",
                dat.id, dat.sector_id
            ))?;

        // family_id 0x90 (144) = Explored, 0x92 (146) = Unexplored (per SYSTEMSD.DAT format).
        let exploration_status = if dat.family_id == 0x90 {
            ExplorationStatus::Explored
        } else {
            ExplorationStatus::Unexplored
        };

        // Coruscant (0x109) is the Empire HQ; Yavin (0x121) is the Alliance HQ.
        // The lower 24 bits of `dat.id` are the sequential index.
        let seq_id = dat.id & 0x00FF_FFFF;
        let is_headquarters = seq_id == 0x109 || seq_id == 0x121;

        let system = System {
            dat_id: DatId::new(dat.id),
            name: lookup(dat.text_stra_dll_id, "System"),
            sector: sector_key,
            x: dat.x_position,
            y: dat.y_position,
            exploration_status,
            popularity_alliance: 0.0,
            popularity_empire: 0.0,
            is_populated: is_headquarters,
            total_energy: 0,
            raw_materials: 0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        };
        let key = world.systems.insert(system);
        system_key_map.insert(dat.id, key);

        // Register the system in its parent sector.
        world.sectors[sector_key].systems.push(key);
    }

    // ── 3. Capital ship classes ──────────────────────────────────────────────
    let capships_file: CapitalShipsFile = read_dat_file(&gdata_path.join("CAPSHPSD.DAT"))?;

    for dat in &capships_file.ships {
        let class = CapitalShipClass {
            dat_id: DatId::new(dat.id),
            name: lookup(dat.text_stra_dll_id, "CapShip"),
            is_alliance: dat.is_alliance != 0,
            is_empire: dat.is_empire != 0,
            refined_material_cost: dat.refined_material_cost,
            maintenance_cost: dat.maintenance_cost,
            research_order: dat.research_order,
            research_difficulty: dat.research_difficulty,
            hull: dat.hull,
            shield_strength: dat.shield_strength,
            sub_light_engine: dat.sub_light_engine,
            maneuverability: dat.maneuverability,
            hyperdrive: dat.hyperdrive,
            fighter_capacity: dat.fighter_capacity,
            troop_capacity: dat.troop_capacity,
            detection: dat.detection,
            turbolaser_fore: dat.turbolaser_fore,
            turbolaser_aft: dat.turbolaser_aft,
            turbolaser_port: dat.turbolaser_port,
            turbolaser_starboard: dat.turbolaser_starboard,
            ion_cannon_fore: dat.ion_cannon_fore,
            ion_cannon_aft: dat.ion_cannon_aft,
            ion_cannon_port: dat.ion_cannon_port,
            ion_cannon_starboard: dat.ion_cannon_starboard,
            laser_cannon_fore: dat.laser_cannon_fore,
            laser_cannon_aft: dat.laser_cannon_aft,
            laser_cannon_port: dat.laser_cannon_port,
            laser_cannon_starboard: dat.laser_cannon_starboard,
            shield_recharge_rate: dat.shield_recharge_rate,
            damage_control: dat.damage_control,
            bombardment_modifier: dat.bombardment_modifier,
            // Extended combat stats — wired from CAPSHPSD.DAT
            overall_attack_strength: dat.overall_attack_strength,
            weapon_recharge_rate: dat.weapon_recharge_rate,
            turbolaser_attack_strength: dat.turbolaser_attack_strength,
            ion_cannon_attack_strength: dat.ion_cannon_attack_strength,
            laser_cannon_attack_strength: dat.laser_cannon_attack_strength,
            turbolaser_range: dat.turbolaser_range,
            ion_cannon_range: dat.ion_cannon_range,
            laser_cannon_range: dat.laser_cannon_range,
            tractor_beam_power: dat.tractor_beam_power,
            tractor_beam_range: dat.tractor_beam_range,
            gravity_well_projector: dat.gravity_well_projector,
            interdiction_strength: dat.interdiction_strength,
            uprising_defense: dat.uprising_defense,
            hyperdrive_if_damaged: dat.hyperdrive_if_damaged,
        };
        world.capital_ship_classes.insert(class);
    }

    // ── 4. Fighter classes ───────────────────────────────────────────────────
    let fighters_file: FightersFile = read_dat_file(&gdata_path.join("FIGHTSD.DAT"))?;

    for dat in &fighters_file.fighters {
        let class = FighterClass {
            dat_id: DatId::new(dat.id),
            name: lookup(dat.text_stra_dll_id, "Fighter"),
            is_alliance: dat.is_alliance != 0,
            is_empire: dat.is_empire != 0,
            refined_material_cost: dat.refined_material_cost,
            maintenance_cost: dat.maintenance_cost,
            research_order: dat.research_order,
            research_difficulty: dat.research_difficulty,
            squadron_size: dat.squadron_size,
            torpedoes: dat.torpedoes,
            torpedoes_range: dat.torpedoes_range,
            overall_attack_strength: dat.overall_attack_strength,
            bombardment_defense: dat.bombardment_defense,
            // Extended fighter stats — wired from FIGHTSD.DAT
            shield_strength: dat.shield_strength,
            sub_light_engine: dat.sub_light_engine,
            maneuverability: dat.maneuverability,
            detection: dat.detection,
            uprising_defense: dat.uprising_defense,
            turbolaser_fore: dat.turbolaser_fore,
            ion_cannon_fore: dat.ion_cannon_fore,
            laser_cannon_fore: dat.laser_cannon_fore,
            turbolaser_attack_strength: dat.turbolaser_attack_strength,
            ion_cannon_attack_strength: dat.ion_cannon_attack_strength,
            laser_cannon_attack_strength: dat.laser_cannon_attack_strength,
        };
        world.fighter_classes.insert(class);
    }

    // ── 5. Major characters ──────────────────────────────────────────────────
    let major_file: MajorCharactersFile = read_dat_file(&gdata_path.join("MJCHARSD.DAT"))?;

    for dat in &major_file.characters {
        let name = lookup(dat.text_stra_dll_id, "Character");
        world.characters.insert(convert_character(dat, true, name));
    }

    // ── 6. Minor characters ──────────────────────────────────────────────────
    let minor_file: MinorCharactersFile = read_dat_file(&gdata_path.join("MNCHARSD.DAT"))?;

    for dat in &minor_file.characters {
        let name = lookup(dat.text_stra_dll_id, "Character");
        world.characters.insert(convert_character(dat, false, name));
    }

    // ── 7. GNPRTB / SDPRTB seeding parameters ───────────────────────────────
    let gnprtb_path = gdata_path.join("GNPRTB.DAT");
    if file_available(&gnprtb_path) {
        let gnprtb_file: GeneralParamsFile = read_dat_file(&gnprtb_path)?;
        let entries = gnprtb_file.entries.into_iter().map(|e| GnprtbEntry {
            parameter_id: e.parameter_id,
            development: e.development,
            alliance_sp_easy: e.alliance_sp_easy,
            alliance_sp_medium: e.alliance_sp_medium,
            alliance_sp_hard: e.alliance_sp_hard,
            empire_sp_easy: e.empire_sp_easy,
            empire_sp_medium: e.empire_sp_medium,
            empire_sp_hard: e.empire_sp_hard,
            multiplayer: e.multiplayer,
        }).collect();
        world.gnprtb = GnprtbParams::new(entries);
    }

    let sdprtb_path = gdata_path.join("SDPRTB.DAT");
    if file_available(&sdprtb_path) {
        let sdprtb_file: SideParamsFile = read_dat_file(&sdprtb_path)?;
        let entries = sdprtb_file.entries.into_iter().map(|e| SdprtbEntry {
            parameter_id: e.parameter_id,
            dev_alliance: e.dev_alliance,
            dev_empire: e.dev_empire,
            alliance_sp_easy_alliance: e.alliance_sp_easy_alliance,
            alliance_sp_easy_empire: e.alliance_sp_easy_empire,
            alliance_sp_medium_alliance: e.alliance_sp_medium_alliance,
            alliance_sp_medium_empire: e.alliance_sp_medium_empire,
            alliance_sp_hard_alliance: e.alliance_sp_hard_alliance,
            alliance_sp_hard_empire: e.alliance_sp_hard_empire,
            empire_sp_easy_alliance: e.empire_sp_easy_alliance,
            empire_sp_easy_empire: e.empire_sp_easy_empire,
            empire_sp_medium_alliance: e.empire_sp_medium_alliance,
            empire_sp_medium_empire: e.empire_sp_medium_empire,
            empire_sp_hard_alliance: e.empire_sp_hard_alliance,
            empire_sp_hard_empire: e.empire_sp_hard_empire,
            multiplayer_alliance: e.multiplayer_alliance,
            multiplayer_empire: e.multiplayer_empire,
        }).collect();
        world.sdprtb = SdprtbParams::new(entries);
    }

    // ── 8. Seed tables ──────────────────────────────────────────────────────
    // Populate starting fleets, ground units, and facilities from the DAT
    // tables after seeding parameters are available.
    seeds::apply_seeds(gdata_path, &mut world, &system_key_map, seed_options)?;

    // ── 8b. Derive controlling_faction from seeded assets (fallback only) ──
    // For systems that still have no explicit control after the M5 procedural
    // bucket assignment, infer control from asset ownership. Systems already
    // assigned control by the seed pipeline (special systems, bucket assignment)
    // are left untouched.
    {
        let fleet_factions: Vec<(SystemKey, bool)> = world.fleets.values()
            .map(|f| (f.location, f.is_alliance))
            .collect();

        let mut asset_factions: Vec<(SystemKey, bool)> = Vec::new();
        for (sys_key, sys) in world.systems.iter() {
            for &k in &sys.defense_facilities {
                if let Some(f) = world.defense_facilities.get(k) {
                    asset_factions.push((sys_key, f.is_alliance));
                }
            }
            for &k in &sys.manufacturing_facilities {
                if let Some(f) = world.manufacturing_facilities.get(k) {
                    asset_factions.push((sys_key, f.is_alliance));
                }
            }
            for &k in &sys.production_facilities {
                if let Some(f) = world.production_facilities.get(k) {
                    asset_factions.push((sys_key, f.is_alliance));
                }
            }
            for &k in &sys.ground_units {
                if let Some(t) = world.troops.get(k) {
                    asset_factions.push((sys_key, t.is_alliance));
                }
            }
            for &k in &sys.special_forces {
                if let Some(sf) = world.special_forces.get(k) {
                    asset_factions.push((sys_key, sf.is_alliance));
                }
            }
        }

        for (sys_key, sys) in world.systems.iter_mut() {
            // Skip systems that already have explicit control from seeding.
            if sys.control != ControlKind::Uncontrolled {
                continue;
            }
            let mut has_alliance = false;
            let mut has_empire = false;
            for &(fk, is_a) in &fleet_factions {
                if fk == sys_key { if is_a { has_alliance = true; } else { has_empire = true; } }
            }
            for &(fk, is_a) in &asset_factions {
                if fk == sys_key { if is_a { has_alliance = true; } else { has_empire = true; } }
            }
            if has_alliance && !has_empire {
                sys.control = ControlKind::Controlled(rebellion_core::dat::Faction::Alliance);
            } else if has_empire && !has_alliance {
                sys.control = ControlKind::Controlled(rebellion_core::dat::Faction::Empire);
            }
        }
    }

    // ── 8c. Populate character location tracking ────────────────────────────
    // Scan all fleets to back-fill each character's current_system and current_fleet.
    let fleet_keys: Vec<_> = world.fleets.keys().collect();
    for fleet_key in fleet_keys {
        let fleet = &world.fleets[fleet_key];
        let location = fleet.location;
        let char_keys: Vec<_> = fleet.characters.clone();
        for char_key in char_keys {
            if let Some(c) = world.characters.get_mut(char_key) {
                c.current_fleet = Some(fleet_key);
                c.current_system = Some(location);
            }
        }
    }

    // ── 9. Troop classes ──────────────────────────────────────────────────
    let troops_path = gdata_path.join("TROOPSD.DAT");
    if file_available(&troops_path) {
        let troops_file: TroopsFile = read_dat_file(&troops_path)?;
        for dat in &troops_file.troops {
            world.troop_classes.insert(
                DatId::new(dat.id),
                TroopClassDef {
                    attack_strength: dat.attack_strength,
                    defense_strength: dat.defense_strength,
                },
            );
        }
    }

    // ── 10. Defense facility classes ─────────────────────────────────────────
    let deffac_path = gdata_path.join("DEFFACSD.DAT");
    if file_available(&deffac_path) {
        let deffac_file: DefenseFacilitiesFile = read_dat_file(&deffac_path)?;
        for dat in &deffac_file.facilities {
            world.defense_facility_classes.insert(
                DatId::new(dat.id),
                DefenseFacilityClassDef {
                    bombardment_defense: dat.bombardment_defense as i32,
                },
            );
        }
    }

    // ── 10. Mission probability tables (*MSTB.DAT and *TB.DAT) ──────────────
    // All 19 IntTableFile tables. Missing files are silently skipped.
    const MSTB_FILES: &[&str] = &[
        "DIPLMSTB.DAT", "ESPIMSTB.DAT", "ASSNMSTB.DAT", "INCTMSTB.DAT",
        "DSSBMSTB.DAT", "ABDCMSTB.DAT", "RCRTMSTB.DAT", "RESCMSTB.DAT",
        "SBTGMSTB.DAT", "SUBDMSTB.DAT", "ESCAPETB.DAT", "FDECOYTB.DAT",
        "FOILTB.DAT",   "INFORMTB.DAT", "CSCRHTTB.DAT", "UPRIS1TB.DAT",
        "UPRIS2TB.DAT", "RLEVADTB.DAT", "RESRCTB.DAT",  "TDECOYTB.DAT",
    ];
    for filename in MSTB_FILES {
        let path = gdata_path.join(filename);
        if file_available(&path) {
            let table_file: IntTableFile = read_dat_file(&path)?;
            let entries = table_file.entries.into_iter().map(|e| MstbEntry {
                threshold: e.threshold,
                value: e.value,
            }).collect();
            // Key = file stem without extension, uppercase (e.g. "DIPLMSTB")
            let stem = filename.trim_end_matches(".DAT").to_string();
            world.mission_tables.insert(stem, MstbTable::new(entries));
        }
    }

    // ── 11. Apply enabled mods ──────────────────────────────────────────────
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mods_dir = gdata_path.parent().and_then(|p| p.parent()).unwrap_or(Path::new(".")).join("mods");
        if mods_dir.exists() {
            let runtime = crate::mods::ModRuntime::discover(&mods_dir);
            let errors = runtime.apply_enabled(&mut world);
            for err in &errors {
                eprintln!("Mod error: {:?}", err);
            }
        }
    }

    Ok(world)
}

// NOTE: load_game_data_from_files() was removed — the WASM file cache approach
// (set_file_cache + load_game_data) is simpler and avoids duplicating 250 lines
// of conversion logic.

/// Initialize the mod runtime for UI access. Returns `None` if the mods
/// directory does not exist (common for first-time players).
#[cfg(not(target_arch = "wasm32"))]
pub fn init_mod_runtime(gdata_path: &Path) -> Option<crate::mods::ModRuntime> {
    let mods_dir = gdata_path.parent().and_then(|p| p.parent()).unwrap_or(Path::new(".")).join("mods");
    if mods_dir.exists() {
        Some(crate::mods::ModRuntime::discover(&mods_dir))
    } else {
        None
    }
}

/// Convert a DAT character entry into a world Character.
fn convert_character(dat: &CharacterEntry, is_major: bool, name: String) -> Character {
    Character {
        dat_id: DatId::new(dat.id),
        name,
        is_alliance: dat.is_alliance != 0,
        is_empire: dat.is_empire != 0,
        is_major,
        diplomacy: SkillPair { base: dat.diplomacy_base, variance: dat.diplomacy_variance },
        espionage: SkillPair { base: dat.espionage_base, variance: dat.espionage_variance },
        ship_design: SkillPair { base: dat.ship_design_base, variance: dat.ship_design_variance },
        troop_training: SkillPair { base: dat.troop_training_base, variance: dat.troop_training_variance },
        facility_design: SkillPair { base: dat.facility_design_base, variance: dat.facility_design_variance },
        combat: SkillPair { base: dat.combat_base, variance: dat.combat_variance },
        leadership: SkillPair { base: dat.leadership_base, variance: dat.leadership_variance },
        loyalty: SkillPair { base: dat.loyalty_base, variance: dat.loyalty_variance },
        jedi_probability: dat.jedi_probability,
        jedi_level: SkillPair { base: dat.jedi_level_base, variance: dat.jedi_level_variance },
        can_be_admiral: dat.can_be_admiral != 0,
        can_be_commander: dat.can_be_commander != 0,
        can_be_general: dat.can_be_general != 0,
        force_tier: if dat.is_known_jedi != 0 { rebellion_core::world::ForceTier::Aware } else { rebellion_core::world::ForceTier::None },
        force_experience: 0,
        is_discovered_jedi: false,
        is_unable_to_betray: dat.is_unable_to_betray != 0,
        is_jedi_trainer: dat.is_jedi_trainer != 0,
        is_known_jedi: dat.is_known_jedi != 0,
        hyperdrive_modifier: 0,
        enhanced_loyalty: 0,
        on_mission: false,
        on_hidden_mission: false,
        on_mandatory_mission: false,
        captured_by: None,
        capture_tick: None,
        is_captive: false,
        current_system: None,
        current_fleet: None,
    }
}

/// Read and parse a single .DAT file into type `T` from the filesystem.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read_dat_file<T: DatRecord>(path: &Path) -> anyhow::Result<T> {
    let data = std::fs::read(path)
        .with_context(|| format!("reading {}", path.display()))?;
    parse_dat_bytes::<T>(&data, &path.display().to_string())
}

/// WASM variant: reads from the pre-loaded file cache set by `set_file_cache`.
#[cfg(target_arch = "wasm32")]
pub(crate) fn read_dat_file<T: DatRecord>(path: &Path) -> anyhow::Result<T> {
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let cache = WASM_FILE_CACHE.lock().unwrap();
    let data = cache.get(filename)
        .with_context(|| format!("file not in WASM cache: {} (have: {:?})", filename, cache.keys().collect::<Vec<_>>()))?;
    parse_dat_bytes::<T>(data, filename)
}

/// Pre-load DAT file bytes for WASM. Call before `load_game_data()`.
#[cfg(target_arch = "wasm32")]
pub fn set_file_cache(files: std::collections::HashMap<String, Vec<u8>>) {
    *WASM_FILE_CACHE.lock().unwrap() = files;
}

/// Pre-load TEXTSTRA string table for WASM. Call before `load_game_data()`.
/// The HashMap maps string resource IDs to their display names.
#[cfg(target_arch = "wasm32")]
pub fn set_string_table(strings: std::collections::HashMap<u16, String>) {
    *WASM_STRING_TABLE.lock().unwrap() = strings;
}

#[cfg(target_arch = "wasm32")]
static WASM_FILE_CACHE: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<String, Vec<u8>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

#[cfg(target_arch = "wasm32")]
static WASM_STRING_TABLE: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<u16, String>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

/// Check if a file exists. On native: filesystem. On WASM: checks the file cache.
pub(crate) fn file_available(path: &Path) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    { path.exists() }
    #[cfg(target_arch = "wasm32")]
    {
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        WASM_FILE_CACHE.lock().unwrap().contains_key(filename)
    }
}

/// Parse a DAT file from raw bytes. Platform-independent.
pub fn parse_dat_bytes<T: DatRecord>(data: &[u8], name: &str) -> anyhow::Result<T> {
    let mut reader = ByteReader::new(data);
    T::parse(&mut reader).with_context(|| format!("parsing {}", name))
}
