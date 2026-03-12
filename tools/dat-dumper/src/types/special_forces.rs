use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// SPECFCSD.DAT — 9 special forces, 116 bytes per record
// File size: 16 (header) + 9 * 116 = 1060 bytes
// Header: field1=1, count=9, family_id=60, field4=64
// Layout: 28 u32 (112 bytes) + 2 u16 (4 bytes) = 116 bytes per record

#[derive(Debug, Clone, Serialize)]
pub struct SpecialForcesFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub units: Vec<SpecialForce>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpecialForce {
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
    pub diplomacy_base: u32,
    pub diplomacy_variance: u32,
    pub espionage_base: u32,
    pub espionage_variance: u32,
    pub ship_design_base: u32,
    pub ship_design_variance: u32,
    pub troop_training_base: u32,
    pub troop_training_variance: u32,
    pub facility_design_base: u32,
    pub facility_design_variance: u32,
    pub combat_base: u32,
    pub combat_variance: u32,
    pub leadership_base: u32,
    pub leadership_variance: u32,
    pub loyalty_base: u32,
    pub loyalty_variance: u32,
    pub mission_id: u32,
}

impl DatRecord for SpecialForcesFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut units = Vec::with_capacity(count as usize);
        for _ in 0..count {
            units.push(SpecialForce::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, units })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for unit in &self.units {
            unit.write_entry(w);
        }
    }
}

impl SpecialForce {
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
            diplomacy_base: r.read_u32()?,
            diplomacy_variance: r.read_u32()?,
            espionage_base: r.read_u32()?,
            espionage_variance: r.read_u32()?,
            ship_design_base: r.read_u32()?,
            ship_design_variance: r.read_u32()?,
            troop_training_base: r.read_u32()?,
            troop_training_variance: r.read_u32()?,
            facility_design_base: r.read_u32()?,
            facility_design_variance: r.read_u32()?,
            combat_base: r.read_u32()?,
            combat_variance: r.read_u32()?,
            leadership_base: r.read_u32()?,
            leadership_variance: r.read_u32()?,
            loyalty_base: r.read_u32()?,
            loyalty_variance: r.read_u32()?,
            mission_id: r.read_u32()?,
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
        w.write_u32(self.diplomacy_base);
        w.write_u32(self.diplomacy_variance);
        w.write_u32(self.espionage_base);
        w.write_u32(self.espionage_variance);
        w.write_u32(self.ship_design_base);
        w.write_u32(self.ship_design_variance);
        w.write_u32(self.troop_training_base);
        w.write_u32(self.troop_training_variance);
        w.write_u32(self.facility_design_base);
        w.write_u32(self.facility_design_variance);
        w.write_u32(self.combat_base);
        w.write_u32(self.combat_variance);
        w.write_u32(self.leadership_base);
        w.write_u32(self.leadership_variance);
        w.write_u32(self.loyalty_base);
        w.write_u32(self.loyalty_variance);
        w.write_u32(self.mission_id);
    }
}
