use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// TROOPSD.DAT — 10 troops, 68 bytes per record
// File size: 16 (header) + 10 * 68 = 696 bytes
// Header: field1=1, count=10, family_id=16, field4=20
// Layout: 16 u32 (64 bytes) + 2 u16 (4 bytes) = 68 bytes per record

#[derive(Debug, Clone, Serialize)]
pub struct TroopsFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub troops: Vec<Troop>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Troop {
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
    pub uprising_defense: u32,
    pub detection: u32,
    pub bombardment_defense: u32,
    pub attack_strength: u32,
    pub defense_strength: u32,
}

impl DatRecord for TroopsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut troops = Vec::with_capacity(count as usize);
        for _ in 0..count {
            troops.push(Troop::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, troops })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for troop in &self.troops {
            troop.write_entry(w);
        }
    }
}

impl Troop {
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
            uprising_defense: r.read_u32()?,
            detection: r.read_u32()?,
            bombardment_defense: r.read_u32()?,
            attack_strength: r.read_u32()?,
            defense_strength: r.read_u32()?,
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
        w.write_u32(self.uprising_defense);
        w.write_u32(self.detection);
        w.write_u32(self.bombardment_defense);
        w.write_u32(self.attack_strength);
        w.write_u32(self.defense_strength);
    }
}
