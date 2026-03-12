use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// SECTORSD.DAT — 20 sectors, 36 bytes per record
// File size: 736 bytes. Header = 16. Records = 720. 720 / 36 = 20 sectors.
// Layout: 7 u32 (28 bytes) + 4 u16 (8 bytes) = 36 bytes per record
//   id(u32) + field2(u32) + production_family(u32) + next_production_family(u32) + family_id(u32) = 20
//   text_stra_dll_id(u16) + field7(u16) = 4 → total 24
//   group(u32) + galaxy_size(u32) = 8 → total 32
//   x_position(u16) + y_position(u16) = 4 → total 36 ✓

#[derive(Debug, Clone, Serialize)]
pub struct SectorsFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub sectors: Vec<Sector>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Sector {
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
    pub group: u32,
    pub galaxy_size: u32,
    pub x_position: u16,
    pub y_position: u16,
}

impl DatRecord for SectorsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut sectors = Vec::with_capacity(count as usize);
        for _ in 0..count {
            sectors.push(Sector::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, sectors })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for sector in &self.sectors {
            sector.write_entry(w);
        }
    }
}

impl Sector {
    fn parse_entry(r: &mut ByteReader) -> anyhow::Result<Self> {
        Ok(Self {
            id: r.read_u32()?,
            field2: r.read_u32()?,
            production_family: r.read_u32()?,
            next_production_family: r.read_u32()?,
            family_id: r.read_u32()?,
            text_stra_dll_id: r.read_u16()?,
            field7: r.read_u16()?,
            group: r.read_u32()?,
            galaxy_size: r.read_u32()?,
            x_position: r.read_u16()?,
            y_position: r.read_u16()?,
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
        w.write_u32(self.group);
        w.write_u32(self.galaxy_size);
        w.write_u16(self.x_position);
        w.write_u16(self.y_position);
    }
}
