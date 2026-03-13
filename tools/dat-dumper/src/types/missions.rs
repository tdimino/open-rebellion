use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// MISSNSD.DAT — 25 mission definitions, 112 bytes per record
// Header: field1=1, count=25, family_id=0x40, field4=0x80
// File size: 16 + 25 * 112 = 2816 bytes

#[derive(Debug, Clone, Serialize)]
pub struct MissionsFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub missions: Vec<Mission>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Mission {
    // Standard 24-byte entity prefix
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,

    // Mission-specific fields (88 bytes = 22 u32)
    pub is_alliance: u32,
    pub is_empire: u32,
    // Bitmask of eligible special forces types (matches SPECFCSD MissionId bitmask)
    pub special_force_eligibility: u32,
    // 0x10000 when mission can target enemy systems, 0 otherwise
    pub target_flags: u32,
    // Maximum number of officers that can be assigned
    pub max_officers: u32,
    // Base duration in game turns
    pub base_duration: u32,
    // Boolean flags for mission properties (16 flags)
    pub flag_col6: u32,
    pub flag_col7: u32,
    pub flag_col8: u32,
    pub flag_col9: u32,
    pub flag_col10: u32,
    pub flag_col11: u32,
    pub flag_col12: u32,
    pub flag_col13: u32,
    pub flag_col14: u32,
    pub flag_col15: u32,
    pub flag_col16: u32,
    pub flag_col17: u32,
    pub flag_col18: u32,
    pub flag_col19: u32,
    pub flag_col20: u32,
    pub flag_col21: u32,
}

impl DatRecord for MissionsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut missions = Vec::with_capacity(count as usize);
        for _ in 0..count {
            missions.push(Mission::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, missions })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for m in &self.missions {
            m.write_entry(w);
        }
    }
}

impl Mission {
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
            special_force_eligibility: r.read_u32()?,
            target_flags: r.read_u32()?,
            max_officers: r.read_u32()?,
            base_duration: r.read_u32()?,
            flag_col6: r.read_u32()?,
            flag_col7: r.read_u32()?,
            flag_col8: r.read_u32()?,
            flag_col9: r.read_u32()?,
            flag_col10: r.read_u32()?,
            flag_col11: r.read_u32()?,
            flag_col12: r.read_u32()?,
            flag_col13: r.read_u32()?,
            flag_col14: r.read_u32()?,
            flag_col15: r.read_u32()?,
            flag_col16: r.read_u32()?,
            flag_col17: r.read_u32()?,
            flag_col18: r.read_u32()?,
            flag_col19: r.read_u32()?,
            flag_col20: r.read_u32()?,
            flag_col21: r.read_u32()?,
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
        w.write_u32(self.special_force_eligibility);
        w.write_u32(self.target_flags);
        w.write_u32(self.max_officers);
        w.write_u32(self.base_duration);
        w.write_u32(self.flag_col6);
        w.write_u32(self.flag_col7);
        w.write_u32(self.flag_col8);
        w.write_u32(self.flag_col9);
        w.write_u32(self.flag_col10);
        w.write_u32(self.flag_col11);
        w.write_u32(self.flag_col12);
        w.write_u32(self.flag_col13);
        w.write_u32(self.flag_col14);
        w.write_u32(self.flag_col15);
        w.write_u32(self.flag_col16);
        w.write_u32(self.flag_col17);
        w.write_u32(self.flag_col18);
        w.write_u32(self.flag_col19);
        w.write_u32(self.flag_col20);
        w.write_u32(self.flag_col21);
    }
}
