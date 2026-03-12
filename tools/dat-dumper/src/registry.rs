use std::collections::HashMap;

use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use crate::validate::compare_bytes;
use crate::types::*;

pub type ParseFn = fn(&[u8], &str) -> anyhow::Result<String>;

/// Parse a .DAT file, validate round-trip, and return pretty-printed JSON.
fn parse_and_dump<T: DatRecord>(data: &[u8], filename: &str) -> anyhow::Result<String> {
    let mut reader = ByteReader::new(data);
    let record = T::parse(&mut reader)?;
    reader.assert_exhausted(filename)?;

    let mut writer = ByteWriter::with_capacity(data.len());
    record.write_bytes(&mut writer);
    let reserialized = writer.into_bytes();
    compare_bytes(data, &reserialized, filename)?;

    let json = serde_json::to_string_pretty(&record)?;
    Ok(json)
}

pub fn build_registry() -> HashMap<&'static str, ParseFn> {
    let mut m: HashMap<&'static str, ParseFn> = HashMap::new();

    // Pattern 1 — entity tables
    m.insert("CAPSHPSD.DAT", parse_and_dump::<capital_ships::CapitalShipsFile> as ParseFn);
    m.insert("FIGHTSD.DAT",  parse_and_dump::<fighters::FightersFile> as ParseFn);
    m.insert("TROOPSD.DAT",  parse_and_dump::<troops::TroopsFile> as ParseFn);
    m.insert("SPECFCSD.DAT", parse_and_dump::<special_forces::SpecialForcesFile> as ParseFn);
    m.insert("MJCHARSD.DAT", parse_and_dump::<major_characters::MajorCharactersFile> as ParseFn);
    m.insert("MNCHARSD.DAT", parse_and_dump::<minor_characters::MinorCharactersFile> as ParseFn);
    m.insert("SYSTEMSD.DAT", parse_and_dump::<systems::SystemsFile> as ParseFn);
    m.insert("SECTORSD.DAT", parse_and_dump::<sectors::SectorsFile> as ParseFn);
    m.insert("DEFFACSD.DAT", parse_and_dump::<defense_facilities::DefenseFacilitiesFile> as ParseFn);
    m.insert("MANFACSD.DAT", parse_and_dump::<manufacturing_facilities::ManufacturingFacilitiesFile> as ParseFn);
    m.insert("PROFACSD.DAT", parse_and_dump::<production_facilities::ProductionFacilitiesFile> as ParseFn);

    // Pattern 2 — parameter tables
    m.insert("GNPRTB.DAT", parse_and_dump::<general_params::GeneralParamsFile> as ParseFn);
    m.insert("SDPRTB.DAT", parse_and_dump::<side_params::SideParamsFile> as ParseFn);

    // Pattern 3 — seed tables (all share SeedTableFile)
    for name in &[
        "CMUNAFTB.DAT",
        "CMUNALTB.DAT",
        "CMUNCRTB.DAT",
        "CMUNEFTB.DAT",
        "CMUNEMTB.DAT",
        "CMUNHQTB.DAT",
        "CMUNYVTB.DAT",
        "FACLCRTB.DAT",
        "FACLHQTB.DAT",
    ] {
        m.insert(name, parse_and_dump::<seed_table::SeedTableFile> as ParseFn);
    }

    m
}
