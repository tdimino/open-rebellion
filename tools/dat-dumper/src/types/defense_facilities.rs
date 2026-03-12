use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DefenseFacilitiesFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub facilities: Vec<DefenseFacility>,
}

/// One record from DEFFACSD.DAT — 60 bytes per entry.
/// 14 u32 fields (56 bytes) + text_stra_dll_id: u16 + field7: u16 (4 bytes) = 60 bytes.
#[derive(Debug, Clone, Serialize)]
pub struct DefenseFacility {
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
    pub is_alliance: u32,
    pub is_empire: u32,
    pub refined_material_cost: u32,
    pub maintenance_cost: u32,
    pub research_order: u32,
    pub research_difficulty: u32,
    pub bombardment_defense: u32,
    pub attack_strength: u32,
    pub shield_strength: u32,
}

impl DatRecord for DefenseFacilitiesFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut facilities = Vec::with_capacity(count as usize);
        for _ in 0..count {
            facilities.push(DefenseFacility::parse_entry(r)?);
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

impl DefenseFacility {
    fn parse_entry(r: &mut ByteReader) -> anyhow::Result<Self> {
        Ok(Self {
            id: r.read_u32()?,
            field2: r.read_u32()?,
            production_family: r.read_u32()?,
            next_production_family: r.read_u32()?,
            family_id: r.read_u32()?,
            text_stra_dll_id: r.read_u16()?,
            field7: r.read_u16()?,
            is_alliance: r.read_u32()?,
            is_empire: r.read_u32()?,
            refined_material_cost: r.read_u32()?,
            maintenance_cost: r.read_u32()?,
            research_order: r.read_u32()?,
            research_difficulty: r.read_u32()?,
            bombardment_defense: r.read_u32()?,
            attack_strength: r.read_u32()?,
            shield_strength: r.read_u32()?,
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
        w.write_u32(self.is_alliance);
        w.write_u32(self.is_empire);
        w.write_u32(self.refined_material_cost);
        w.write_u32(self.maintenance_cost);
        w.write_u32(self.research_order);
        w.write_u32(self.research_difficulty);
        w.write_u32(self.bombardment_defense);
        w.write_u32(self.attack_strength);
        w.write_u32(self.shield_strength);
    }
}
