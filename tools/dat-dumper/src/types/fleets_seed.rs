use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// FLEETSD.DAT — 2 fleet seed entries, 24 bytes per record (standard entity prefix only)
// Header: field1=1, count=2, family_id=0x08, field4=0x10
// File size: 16 + 2 * 24 = 64 bytes
//
// This file defines the starting fleet composition templates.
// Records use the standard 24-byte entity prefix (6 u32 fields).

#[derive(Debug, Clone, Serialize)]
pub struct FleetsSeedFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub fleets: Vec<FleetSeedEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FleetSeedEntry {
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
}

impl DatRecord for FleetsSeedFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut fleets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            fleets.push(FleetSeedEntry {
                id: r.read_u32()?,
                field2: r.read_u32()?,
                production_family: r.read_u32()?,
                next_production_family: r.read_u32()?,
                family_id: r.read_u32()?,
                text_stra_dll_id: r.read_u16()?,
                field7: r.read_u16()?,
            });
        }
        Ok(Self { field1, count, family_id, field4, fleets })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for f in &self.fleets {
            w.write_u32(f.id);
            w.write_u32(f.field2);
            w.write_u32(f.production_family);
            w.write_u32(f.next_production_family);
            w.write_u32(f.family_id);
            w.write_u16(f.text_stra_dll_id);
            w.write_u16(f.field7);
        }
    }
}
