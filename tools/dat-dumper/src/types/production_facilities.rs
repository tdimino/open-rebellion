use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// Re-export the shared facility layout so callers can use either module.
pub use crate::types::manufacturing_facilities::ManufacturingFacility;

#[derive(Debug, Clone, Serialize)]
pub struct ProductionFacilitiesFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub facilities: Vec<ManufacturingFacility>,
}

impl DatRecord for ProductionFacilitiesFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut facilities = Vec::with_capacity(count as usize);
        for _ in 0..count {
            facilities.push(ManufacturingFacility::parse_entry(r)?);
        }
        Ok(Self {
            field1,
            count,
            family_id,
            field4,
            facilities,
        })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for facility in &self.facilities {
            facility.write_entry(w);
        }
    }
}
