//! Source-faithful Galactic Encyclopedia catalog loading.
//!
//! The original window filters one shared game-object catalog by the family
//! byte stored in each compound object id. This module rebuilds that immutable
//! reference catalog from the shipped DAT tables and `TEXTSTRA.DLL` without
//! adding presentation-only data to [`rebellion_core::world::GameWorld`] or
//! changing the save format.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use dat_dumper::types::capital_ships::CapitalShipsFile;
use dat_dumper::types::defense_facilities::DefenseFacilitiesFile;
use dat_dumper::types::fighters::FightersFile;
use dat_dumper::types::major_characters::MajorCharactersFile;
use dat_dumper::types::manufacturing_facilities::ManufacturingFacilitiesFile;
use dat_dumper::types::minor_characters::MinorCharactersFile;
use dat_dumper::types::missions::MissionsFile;
use dat_dumper::types::production_facilities::ProductionFacilitiesFile;
use dat_dumper::types::special_forces::SpecialForcesFile;
use dat_dumper::types::systems::SystemsFile;
#[cfg(not(target_arch = "wasm32"))]
use dat_dumper::types::textstra;
use dat_dumper::types::troops::TroopsFile;

use crate::read_dat_file;

/// Original DAT table that supplied an Encyclopedia catalog entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncyclopediaSourceTable {
    /// `SYSTEMSD.DAT`
    Systems,
    /// `CAPSHPSD.DAT`
    CapitalShips,
    /// `FIGHTSD.DAT`
    Fighters,
    /// `DEFFACSD.DAT`
    DefenseFacilities,
    /// `MANFACSD.DAT`
    ManufacturingFacilities,
    /// `PROFACSD.DAT`
    ProductionFacilities,
    /// `MISSNSD.DAT`
    Missions,
    /// `TROOPSD.DAT`
    Troops,
    /// `MJCHARSD.DAT`
    MajorCharacters,
    /// `MNCHARSD.DAT`
    MinorCharacters,
    /// `SPECFCSD.DAT`
    SpecialForces,
}

/// One original game-object entry available to the Encyclopedia index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaCatalogEntry {
    /// Compound game-object identity. The high byte is the source family.
    pub object_id: u32,
    /// TEXTSTRA resource used for the visible name.
    pub text_resource_id: u16,
    /// Localized visible name.
    pub name: String,
    source_table: EncyclopediaSourceTable,
    raw_dat_id: u32,
}

impl EncyclopediaCatalogEntry {
    #[must_use]
    pub const fn family(&self) -> u8 {
        (self.object_id >> 24) as u8
    }

    /// Return the original DAT table that supplied this entry.
    #[must_use]
    pub const fn source_table(&self) -> EncyclopediaSourceTable {
        self.source_table
    }

    /// Return the source record id exactly as decoded, before compounding.
    #[must_use]
    pub const fn raw_dat_id(&self) -> u32 {
        self.raw_dat_id
    }
}

/// One of the seven original index controls (`0x6f..=0x75`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaCategory {
    pub command_id: u16,
    pub label_resource_id: u16,
    pub label: String,
    pub(crate) family_range: Option<std::ops::Range<u8>>,
}

impl EncyclopediaCategory {
    #[must_use]
    pub fn contains(&self, entry: &EncyclopediaCatalogEntry) -> bool {
        self.family_range
            .as_ref()
            .is_none_or(|range| range.contains(&entry.family()))
    }
}

/// Immutable localized Encyclopedia index data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaCatalog {
    pub title: String,
    pub topic_label: String,
    pub categories: [EncyclopediaCategory; 7],
    /// Global alphabetical order used by the native list control. Filtering a
    /// category preserves this order.
    pub entries: Vec<EncyclopediaCatalogEntry>,
}

impl EncyclopediaCatalog {
    #[must_use]
    pub fn category(&self, command_id: u16) -> Option<&EncyclopediaCategory> {
        self.categories
            .iter()
            .find(|category| category.command_id == command_id)
    }

    #[must_use]
    pub fn entries_for(&self, command_id: u16) -> Vec<&EncyclopediaCatalogEntry> {
        let Some(category) = self.category(command_id) else {
            return Vec::new();
        };
        self.entries
            .iter()
            .filter(|entry| category.contains(entry))
            .collect()
    }
}

/// Load the complete localized index directly from original game data.
///
/// This reference catalog is deliberately separate from campaign state. It is
/// safe to recreate after loading a save and cannot change bincode layouts.
///
/// # Errors
/// Returns an error if a required DAT table or required TEXTSTRA string is
/// absent or malformed.
pub fn load_encyclopedia_catalog(gdata_path: &Path) -> Result<EncyclopediaCatalog> {
    let strings = load_string_table(gdata_path)?;
    let required = |resource_id: u16| -> Result<String> {
        strings
            .get(&resource_id)
            .cloned()
            .with_context(|| format!("TEXTSTRA resource {resource_id:#06x} is missing"))
    };

    let categories = [
        category(0x6f, 0x1850, required(0x1850)?, None),
        category(0x70, 0x1855, required(0x1855)?, Some(0x90..0x98)),
        category(0x71, 0x1854, required(0x1854)?, Some(0x14..0x20)),
        category(0x72, 0x1852, required(0x1852)?, Some(0x20..0x30)),
        category(0x73, 0x1851, required(0x1851)?, Some(0x40..0x80)),
        category(0x74, 0x1856, required(0x1856)?, Some(0x10..0x14)),
        category(0x75, 0x1853, required(0x1853)?, Some(0x30..0x40)),
    ];

    let mut entries = Vec::new();
    let systems: SystemsFile = read_dat_file(&gdata_path.join("SYSTEMSD.DAT"))?;
    for record in systems.systems {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Systems,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }

    let capital_ships: CapitalShipsFile = read_dat_file(&gdata_path.join("CAPSHPSD.DAT"))?;
    for record in capital_ships.ships {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::CapitalShips,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }
    let fighters: FightersFile = read_dat_file(&gdata_path.join("FIGHTSD.DAT"))?;
    for record in fighters.fighters {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Fighters,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }

    let defense: DefenseFacilitiesFile = read_dat_file(&gdata_path.join("DEFFACSD.DAT"))?;
    for record in defense.facilities {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::DefenseFacilities,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }
    let manufacturing: ManufacturingFacilitiesFile =
        read_dat_file(&gdata_path.join("MANFACSD.DAT"))?;
    for record in manufacturing.facilities {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::ManufacturingFacilities,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }
    let production: ProductionFacilitiesFile = read_dat_file(&gdata_path.join("PROFACSD.DAT"))?;
    for record in production.facilities {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::ProductionFacilities,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }

    let missions: MissionsFile = read_dat_file(&gdata_path.join("MISSNSD.DAT"))?;
    for record in missions.missions {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Missions,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }

    let troops: TroopsFile = read_dat_file(&gdata_path.join("TROOPSD.DAT"))?;
    for record in troops.troops {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Troops,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }

    let major: MajorCharactersFile = read_dat_file(&gdata_path.join("MJCHARSD.DAT"))?;
    for record in major.characters {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::MajorCharacters,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }
    let minor: MinorCharactersFile = read_dat_file(&gdata_path.join("MNCHARSD.DAT"))?;
    for record in minor.characters {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::MinorCharacters,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }
    let special_forces: SpecialForcesFile = read_dat_file(&gdata_path.join("SPECFCSD.DAT"))?;
    for record in special_forces.units {
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::SpecialForces,
            record.id,
            record.family_id,
            record.text_stra_dll_id,
            &strings,
        )?;
    }

    entries.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.object_id.cmp(&right.object_id))
    });
    entries.dedup_by_key(|entry| entry.object_id);

    Ok(EncyclopediaCatalog {
        title: required(0x1842)?,
        topic_label: required(0x1843)?,
        categories,
        entries,
    })
}

fn category(
    command_id: u16,
    label_resource_id: u16,
    label: String,
    family_range: Option<std::ops::Range<u8>>,
) -> EncyclopediaCategory {
    EncyclopediaCategory {
        command_id,
        label_resource_id,
        label,
        family_range,
    }
}

fn push_entry(
    entries: &mut Vec<EncyclopediaCatalogEntry>,
    source_table: EncyclopediaSourceTable,
    record_id: u32,
    family_id: u32,
    text_resource_id: u16,
    strings: &HashMap<u16, String>,
) -> Result<()> {
    let family = u8::try_from(family_id)
        .with_context(|| format!("object family {family_id:#x} does not fit one byte"))?;
    let object_id = compound_object_id(record_id, family);
    let name = strings
        .get(&text_resource_id)
        .cloned()
        .with_context(|| format!("TEXTSTRA resource {text_resource_id:#06x} is missing"))?;
    entries.push(EncyclopediaCatalogEntry {
        object_id,
        text_resource_id,
        name,
        source_table,
        raw_dat_id: record_id,
    });
    Ok(())
}

pub(crate) const fn compound_object_id(record_id: u32, family: u8) -> u32 {
    if record_id >> 24 == 0 {
        ((family as u32) << 24) | record_id
    } else {
        record_id
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_string_table(gdata_path: &Path) -> Result<HashMap<u16, String>> {
    let path = gdata_path.join("TEXTSTRA.DLL");
    textstra::load_strings(&path)
        .with_context(|| format!("loading Encyclopedia strings from {}", path.display()))
}

#[cfg(target_arch = "wasm32")]
fn load_string_table(_gdata_path: &Path) -> Result<HashMap<u16, String>> {
    Ok(super::WASM_STRING_TABLE.lock().unwrap().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_catalog() -> EncyclopediaCatalog {
        let categories = [
            category(0x6f, 0x1850, "All Databases".into(), None),
            category(0x70, 0x1855, "System Database".into(), Some(0x90..0x98)),
            category(0x71, 0x1854, "Ship Database".into(), Some(0x14..0x20)),
            category(0x72, 0x1852, "Facilities Database".into(), Some(0x20..0x30)),
            category(0x73, 0x1851, "Missions Database".into(), Some(0x40..0x80)),
            category(0x74, 0x1856, "Troop Database".into(), Some(0x10..0x14)),
            category(0x75, 0x1853, "Personnel Database".into(), Some(0x30..0x40)),
        ];
        EncyclopediaCatalog {
            title: "Galactic Encyclopedia".into(),
            topic_label: "Topic".into(),
            categories,
            entries: vec![
                EncyclopediaCatalogEntry {
                    object_id: 0x1000_0001,
                    text_resource_id: 1,
                    name: "Alliance Army Regiment".into(),
                    source_table: EncyclopediaSourceTable::Troops,
                    raw_dat_id: 1,
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x1400_0001,
                    text_resource_id: 2,
                    name: "Alliance Dreadnaught".into(),
                    source_table: EncyclopediaSourceTable::CapitalShips,
                    raw_dat_id: 1,
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x9000_0001,
                    text_resource_id: 3,
                    name: "Allyuen".into(),
                    source_table: EncyclopediaSourceTable::Systems,
                    raw_dat_id: 1,
                },
            ],
        }
    }

    #[test]
    fn category_family_ranges_match_the_recovered_filter_table() {
        let catalog = fixture_catalog();
        assert_eq!(catalog.entries_for(0x6f).len(), 3);
        assert_eq!(catalog.entries_for(0x70)[0].name, "Allyuen");
        assert_eq!(catalog.entries_for(0x71)[0].name, "Alliance Dreadnaught");
        assert_eq!(catalog.entries_for(0x74)[0].name, "Alliance Army Regiment");
        assert!(catalog.entries_for(0x75).is_empty());
        assert!(catalog.entries_for(0xff).is_empty());
    }

    #[test]
    fn compound_ids_preserve_existing_families_and_attach_sequential_ones() {
        assert_eq!(compound_object_id(0x0000_0109, 0x90), 0x9000_0109);
        assert_eq!(compound_object_id(0x9200_0109, 0x90), 0x9200_0109);
    }

    #[test]
    fn push_entry_requires_and_preserves_the_source_name() {
        let strings = HashMap::from([(0x2001, "A-wing".to_owned()), (0x2003, String::new())]);
        let mut entries = Vec::new();

        push_entry(
            &mut entries,
            EncyclopediaSourceTable::CapitalShips,
            7,
            0x14,
            0x2001,
            &strings,
        )
        .unwrap();
        assert_eq!(
            entries,
            [EncyclopediaCatalogEntry {
                object_id: 0x1400_0007,
                text_resource_id: 0x2001,
                name: "A-wing".to_owned(),
                source_table: EncyclopediaSourceTable::CapitalShips,
                raw_dat_id: 7,
            }]
        );

        let error = push_entry(
            &mut entries,
            EncyclopediaSourceTable::CapitalShips,
            8,
            0x14,
            0x2002,
            &strings,
        )
        .unwrap_err();
        assert!(error.to_string().contains("0x2002"));
        assert_eq!(entries.len(), 1);

        push_entry(
            &mut entries,
            EncyclopediaSourceTable::CapitalShips,
            9,
            0x14,
            0x2003,
            &strings,
        )
        .unwrap();
        assert!(entries[1].name.is_empty());
    }

    #[test]
    fn push_entry_retains_typed_source_provenance_when_raw_dat_ids_collide() {
        let strings = HashMap::from([
            (0x2001, "First source name".to_owned()),
            (0x2002, "Second source name".to_owned()),
        ]);
        let mut entries = Vec::new();

        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Systems,
            7,
            0x40,
            0x2001,
            &strings,
        )
        .unwrap();
        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Missions,
            7,
            0x40,
            0x2002,
            &strings,
        )
        .unwrap();

        assert_eq!(entries[0].source_table(), EncyclopediaSourceTable::Systems);
        assert_eq!(entries[1].source_table(), EncyclopediaSourceTable::Missions);
        assert_eq!(entries[0].raw_dat_id(), 7);
        assert_eq!(entries[1].raw_dat_id(), 7);
    }

    #[test]
    fn push_entry_preserves_precombined_raw_dat_id_beside_compound_identity() {
        let strings = HashMap::from([(0x2001, "Source name".to_owned())]);
        let mut entries = Vec::new();

        push_entry(
            &mut entries,
            EncyclopediaSourceTable::Systems,
            0x9200_0109,
            0x90,
            0x2001,
            &strings,
        )
        .unwrap();

        assert_eq!(entries[0].object_id, 0x9200_0109);
        assert_eq!(entries[0].raw_dat_id(), 0x9200_0109);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires E56_OWNED_REBELLION_ROOT pointing to immutable owned inputs"]
    fn owned_loader_retains_all_source_roles_and_the_p65_projection() {
        use std::cmp::Ordering;
        use std::collections::{HashMap, HashSet};
        use std::fs;

        let owned_root = std::env::var_os("E56_OWNED_REBELLION_ROOT")
            .map(std::path::PathBuf::from)
            .expect("E56_OWNED_REBELLION_ROOT must point to the owned installation root");
        let staged = tempfile::tempdir().unwrap();
        fs::copy(
            owned_root.join("TEXTSTRA.DLL"),
            staged.path().join("TEXTSTRA.DLL"),
        )
        .unwrap();
        for filename in [
            "SYSTEMSD.DAT",
            "CAPSHPSD.DAT",
            "FIGHTSD.DAT",
            "DEFFACSD.DAT",
            "MANFACSD.DAT",
            "PROFACSD.DAT",
            "MISSNSD.DAT",
            "TROOPSD.DAT",
            "MJCHARSD.DAT",
            "MNCHARSD.DAT",
            "SPECFCSD.DAT",
        ] {
            fs::copy(
                owned_root.join("GData").join(filename),
                staged.path().join(filename),
            )
            .unwrap();
        }

        let catalog = load_encyclopedia_catalog(staged.path()).unwrap();
        let expected_source_counts = [
            (EncyclopediaSourceTable::Systems, 200),
            (EncyclopediaSourceTable::CapitalShips, 30),
            (EncyclopediaSourceTable::Fighters, 8),
            (EncyclopediaSourceTable::DefenseFacilities, 6),
            (EncyclopediaSourceTable::ManufacturingFacilities, 6),
            (EncyclopediaSourceTable::ProductionFacilities, 2),
            (EncyclopediaSourceTable::Missions, 25),
            (EncyclopediaSourceTable::Troops, 10),
            (EncyclopediaSourceTable::MajorCharacters, 6),
            (EncyclopediaSourceTable::MinorCharacters, 54),
            (EncyclopediaSourceTable::SpecialForces, 9),
        ];
        let actual_source_counts = catalog.entries.iter().fold(
            HashMap::<EncyclopediaSourceTable, usize>::new(),
            |mut counts, entry| {
                *counts.entry(entry.source_table()).or_default() += 1;
                counts
            },
        );
        for (source_table, expected) in expected_source_counts {
            assert_eq!(actual_source_counts.get(&source_table), Some(&expected));
        }

        assert_eq!(catalog.entries.len(), 356);
        assert_eq!(catalog.entries_for(0x6f).len(), 356);
        assert_eq!(catalog.entries_for(0x70).len(), 200);
        assert_eq!(catalog.entries_for(0x71).len(), 38);
        assert_eq!(catalog.entries_for(0x72).len(), 14);
        assert_eq!(catalog.entries_for(0x73).len(), 25);
        assert_eq!(catalog.entries_for(0x74).len(), 10);
        assert_eq!(catalog.entries_for(0x75).len(), 69);
        assert_eq!(
            catalog
                .entries
                .iter()
                .map(|entry| entry.object_id)
                .collect::<HashSet<_>>()
                .len(),
            356
        );
        assert!(catalog.entries.windows(2).all(|pair| {
            pair[0]
                .name
                .to_lowercase()
                .cmp(&pair[1].name.to_lowercase())
                .then_with(|| pair[0].object_id.cmp(&pair[1].object_id))
                != Ordering::Greater
        }));

        let mut roles_by_low_dat_id = HashMap::<u32, HashSet<EncyclopediaSourceTable>>::new();
        for entry in &catalog.entries {
            roles_by_low_dat_id
                .entry(entry.raw_dat_id() & 0x00ff_ffff)
                .or_default()
                .insert(entry.source_table());
        }
        assert!(roles_by_low_dat_id.values().any(|roles| roles.len() > 1));
    }
}
