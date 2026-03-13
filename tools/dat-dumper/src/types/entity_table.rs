use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// Generic entity table — Pattern 1 with 24-byte records (standard entity prefix only).
//
// File layout:
//   field1:    u32  (always 1)
//   count:     u32  (number of records)
//   family_id: u32  (entity class identifier)
//   field4:    u32  (preserved)
//   records:   [EntityEntry; count]
//
// Each record is exactly the standard 24-byte entity prefix:
//   id: u32, field2: u32, production_family: u32, next_production_family: u32,
//   family_id: u32, text_stra_dll_id: u16, field7: u16
//
// Used by:
//   UNIQUESD.DAT  (6 entries,  family_id=0xf0) — unique objects (Death Star, etc.)
//   ABODESD.DAT   (5 entries,  family_id=0x98) — abode/location types
//   BASICSD.DAT   (16 entries, family_id=0x98) — basic entity catalog
//   MANMGRSD.DAT  (3 entries,  family_id=0xa0) — manager entity types

#[derive(Debug, Serialize)]
pub struct EntityEntry {
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
}

#[derive(Debug, Serialize)]
pub struct EntityTableFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub entries: Vec<EntityEntry>,
}

impl DatRecord for EntityTableFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            entries.push(EntityEntry {
                id: r.read_u32()?,
                field2: r.read_u32()?,
                production_family: r.read_u32()?,
                next_production_family: r.read_u32()?,
                family_id: r.read_u32()?,
                text_stra_dll_id: r.read_u16()?,
                field7: r.read_u16()?,
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
        }
    }
}
