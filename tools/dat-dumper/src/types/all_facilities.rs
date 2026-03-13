use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// ALLFACSD.DAT — 1 record, 52 bytes
// Header: field1=1, count=1, family_id=0x20, field4=0x22
// File size: 16 + 1 * 52 = 68 bytes
//
// This file defines the "all facilities" aggregate entity.
// Uses the standard 24-byte entity prefix plus 7 additional u32 fields.

#[derive(Debug, Serialize)]
pub struct AllFacilitiesFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub entries: Vec<AllFacilitiesEntry>,
}

#[derive(Debug, Serialize)]
pub struct AllFacilitiesEntry {
    // Standard 24-byte entity prefix
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
    // Extra 28 bytes (7 u32)
    pub extra1: u32,
    pub extra2: u32,
    pub extra3: u32,
    pub extra4: u32,
    pub extra5: u32,
    pub extra6: u32,
    pub extra7: u32,
}

impl DatRecord for AllFacilitiesFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            entries.push(AllFacilitiesEntry {
                id: r.read_u32()?,
                field2: r.read_u32()?,
                production_family: r.read_u32()?,
                next_production_family: r.read_u32()?,
                family_id: r.read_u32()?,
                text_stra_dll_id: r.read_u16()?,
                field7: r.read_u16()?,
                extra1: r.read_u32()?,
                extra2: r.read_u32()?,
                extra3: r.read_u32()?,
                extra4: r.read_u32()?,
                extra5: r.read_u32()?,
                extra6: r.read_u32()?,
                extra7: r.read_u32()?,
            });
        }
        Ok(Self { field1, count, family_id, field4, entries })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for e in &self.entries {
            w.write_u32(e.id);
            w.write_u32(e.field2);
            w.write_u32(e.production_family);
            w.write_u32(e.next_production_family);
            w.write_u32(e.family_id);
            w.write_u16(e.text_stra_dll_id);
            w.write_u16(e.field7);
            w.write_u32(e.extra1);
            w.write_u32(e.extra2);
            w.write_u32(e.extra3);
            w.write_u32(e.extra4);
            w.write_u32(e.extra5);
            w.write_u32(e.extra6);
            w.write_u32(e.extra7);
        }
    }
}
