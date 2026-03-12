use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// SYSTEMSD.DAT — 200 systems, 44 bytes per record
// File size: 16 (header) + 200 * 44 = 8816 bytes
// Header: field1=1, count=200, family_id=144, field4=152
// Layout: 8 u32 (32 bytes) + 4 u16 (8 bytes) = 40 bytes... wait:
//   id(u32) + field2(u32) + production_family(u32) + next_production_family(u32) + family_id(u32) = 20
//   text_stra_dll_id(u16) + field7(u16) = 4 → total 24
//   sector_id(u32) + picture_id(u32) + field10(u32) = 12 → total 36
//   x_position(u16) + y_position(u16) = 4 → total 40
//   field13(u32) = 4 → total 44 ✓

#[derive(Debug, Clone, Serialize)]
pub struct SystemsFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub systems: Vec<StarSystem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StarSystem {
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
    pub sector_id: u32,
    pub picture_id: u32,
    pub field10: u32,
    pub x_position: u16,
    pub y_position: u16,
    pub field13: u32,
}

impl DatRecord for SystemsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut systems = Vec::with_capacity(count as usize);
        for _ in 0..count {
            systems.push(StarSystem::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, systems })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for system in &self.systems {
            system.write_entry(w);
        }
    }
}

impl StarSystem {
    fn parse_entry(r: &mut ByteReader) -> anyhow::Result<Self> {
        Ok(Self {
            id: r.read_u32()?,
            field2: r.read_u32()?,
            production_family: r.read_u32()?,
            next_production_family: r.read_u32()?,
            family_id: r.read_u32()?,
            text_stra_dll_id: r.read_u16()?,
            field7: r.read_u16()?,
            sector_id: r.read_u32()?,
            picture_id: r.read_u32()?,
            field10: r.read_u32()?,
            x_position: r.read_u16()?,
            y_position: r.read_u16()?,
            field13: r.read_u32()?,
        })
    }

    fn write_entry(&self, w: &mut ByteWriter) {
        w.write_u32(self.id);
        w.write_u32(self.field2);
        w.write_u32(self.production_family);
        w.write_u32(self.next_production_family);
        w.write_u32(self.family_id);
        w.write_u16(self.text_stra_dll_id);
        w.write_u16(self.field7);
        w.write_u32(self.sector_id);
        w.write_u32(self.picture_id);
        w.write_u32(self.field10);
        w.write_u16(self.x_position);
        w.write_u16(self.y_position);
        w.write_u32(self.field13);
    }
}
