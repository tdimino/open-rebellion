use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// System facility seed tables — Pattern 2 with "SeedTableEntry" info string.
// Each entry defines a facility to place at a specific system at game start.
//
// File layout:
//   field1:        u32   (always 1)
//   entries_count: u32
//   info_length:   u32   (14)
//   info:          [u8; 14]  ("SeedTableEntry")
//   entries:       [SyfcEntry; entries_count]
//
// SyfcEntry layout (16 bytes):
//   id:            u32   (sequential 1, 2, ...)
//   field2:        u32   (always 1)
//   system_id:     u32   (index into SYSTEMSD)
//   facility:      u32   (packed: lo_byte=facility_type_index, hi_byte=facility_family_id)
//
// Used by:
//   SYFCCRTB.DAT  (8 entries) — Empire Coruscant system facilities seed
//   SYFCRMTB.DAT  (7 entries) — Empire Coruscant Rim system facilities seed
//
// The `facility` field packs two values:
//   bytes[0] = within-family index (1-based, e.g. 1st, 2nd, or 3rd facility of this type)
//   bytes[1..2] = 0
//   bytes[3] = family_id of the facility record in DEFFACSD/MANFACSD/PROFACSD
//
// Example: 0x2d000002 -> index=2, family=0x2d (PROFACSD second entry)

#[derive(Debug, Serialize)]
pub struct SyfcEntry {
    pub id: u32,
    pub field2: u32,
    pub system_id: u32,
    // Packed: lo_byte = facility type index within family, hi_byte = family_id
    pub facility: u32,
}

#[derive(Debug, Serialize)]
pub struct SyfcTableFile {
    pub field1: u32,
    pub info: String,
    pub entries: Vec<SyfcEntry>,
}

impl DatRecord for SyfcTableFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let entries_count = r.read_u32()?;
        let info_length = r.read_u32()?;
        let info_bytes = r.read_bytes(info_length as usize)?;
        let info = String::from_utf8(info_bytes)
            .map_err(|e| anyhow::anyhow!("SyfcTable info string is not valid UTF-8: {}", e))?;

        let mut entries = Vec::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            entries.push(SyfcEntry {
                id:        r.read_u32()?,
                field2:    r.read_u32()?,
                system_id: r.read_u32()?,
                facility:  r.read_u32()?,
            });
        }

        Ok(Self { field1, info, entries })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.entries.len() as u32);
        w.write_u32(self.info.len() as u32);
        w.write_bytes(self.info.as_bytes());
        for e in &self.entries {
            w.write_u32(e.id);
            w.write_u32(e.field2);
            w.write_u32(e.system_id);
            w.write_u32(e.facility);
        }
    }
}
