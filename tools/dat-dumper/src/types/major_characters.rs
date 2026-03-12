use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// MJCHARSD.DAT — 6 major characters, 148 bytes per record
// File size: 16 (header) + 6 * 148 = 904 bytes
// Header: field1=1, count=6, family_id=48, field4=56
// Layout: 36 u32 (144 bytes) + 2 u16 (4 bytes) = 148 bytes per record
//
// CharacterEntry is shared with minor_characters.rs (identical binary layout).

#[derive(Debug, Clone, Serialize)]
pub struct MajorCharactersFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub characters: Vec<CharacterEntry>,
}

/// Shared character record layout used by both MJCHARSD.DAT and MNCHARSD.DAT.
#[derive(Debug, Clone, Serialize)]
pub struct CharacterEntry {
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
    pub jedi_probability: u32,
    pub is_known_jedi: u32,
    pub jedi_level_base: u32,
    pub jedi_level_variance: u32,
    pub can_be_admiral: u32,
    pub can_be_commander: u32,
    pub can_be_general: u32,
    pub is_unable_to_betray: u32,
    pub is_jedi_trainer: u32,
}

impl DatRecord for MajorCharactersFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut characters = Vec::with_capacity(count as usize);
        for _ in 0..count {
            characters.push(CharacterEntry::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, characters })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for character in &self.characters {
            character.write_entry(w);
        }
    }
}

impl CharacterEntry {
    pub fn parse_entry(r: &mut ByteReader) -> anyhow::Result<Self> {
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
            jedi_probability: r.read_u32()?,
            is_known_jedi: r.read_u32()?,
            jedi_level_base: r.read_u32()?,
            jedi_level_variance: r.read_u32()?,
            can_be_admiral: r.read_u32()?,
            can_be_commander: r.read_u32()?,
            can_be_general: r.read_u32()?,
            is_unable_to_betray: r.read_u32()?,
            is_jedi_trainer: r.read_u32()?,
        })
    }

    pub fn write_entry(&self, w: &mut ByteWriter) {
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
        w.write_u32(self.jedi_probability);
        w.write_u32(self.is_known_jedi);
        w.write_u32(self.jedi_level_base);
        w.write_u32(self.jedi_level_variance);
        w.write_u32(self.can_be_admiral);
        w.write_u32(self.can_be_commander);
        w.write_u32(self.can_be_general);
        w.write_u32(self.is_unable_to_betray);
        w.write_u32(self.is_jedi_trainer);
    }
}
