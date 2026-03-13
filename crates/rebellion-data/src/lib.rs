//! Data loading: converts .DAT binary structs into rebellion-core world types.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use dat_dumper::codec::ByteReader;
use dat_dumper::dat_record::DatRecord;
use dat_dumper::types::capital_ships::CapitalShipsFile;
use dat_dumper::types::fighters::FightersFile;
use dat_dumper::types::major_characters::{CharacterEntry, MajorCharactersFile};
use dat_dumper::types::minor_characters::MinorCharactersFile;
use dat_dumper::types::sectors::SectorsFile;
use dat_dumper::types::systems::SystemsFile;
use dat_dumper::types::textstra;
use rebellion_core::dat::{ExplorationStatus, SectorGroup};
use rebellion_core::ids::*;
use rebellion_core::world::*;

pub mod seeds;
pub mod mods;

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
    // ── 0. String table ──────────────────────────────────────────────────────
    // Load TEXTSTRA.DLL for real entity names. Fall back to placeholders if
    // the file is absent (WASM builds, test environments, stripped installs).
    let textstra_path = gdata_path.join("TEXTSTRA.DLL");
    let string_table: HashMap<u16, String> = if textstra_path.exists() {
        textstra::load_strings(&textstra_path)
            .with_context(|| format!("loading strings from {}", textstra_path.display()))?
    } else {
        HashMap::new()
    };

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

        let system = System {
            dat_id: DatId::new(dat.id),
            name: lookup(dat.text_stra_dll_id, "System"),
            sector: sector_key,
            x: dat.x_position,
            y: dat.y_position,
            exploration_status,
            popularity_alliance: 0.0,
            popularity_empire: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
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
            squadron_size: dat.squadron_size,
            torpedoes: dat.torpedoes,
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

    // ── 7. Seed tables ───────────────────────────────────────────────────────
    // Populate starting fleets, ground units, and facilities from the 9 seed
    // DAT files.  Missing files are silently skipped (stripped installs).
    seeds::apply_seeds(gdata_path, &mut world, &system_key_map)?;

    Ok(world)
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
    }
}

/// Read and parse a single .DAT file into type `T`.
pub(crate) fn read_dat_file<T: DatRecord>(path: &Path) -> anyhow::Result<T> {
    let data = std::fs::read(path)
        .with_context(|| format!("reading {}", path.display()))?;
    let mut reader = ByteReader::new(&data);
    T::parse(&mut reader).with_context(|| format!("parsing {}", path.display()))
}
