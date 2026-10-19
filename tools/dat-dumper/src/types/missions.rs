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
    // Mission timer minimum in days (record +0x50 in memory, FUN_005236e0);
    // the timer fires after min + rand(0..=spread) (ghidra/notes/mission-lifecycle.md)
    pub timer_min_days: u32,
    // Mission timer spread in days (record +0x54, FUN_005236e0)
    pub timer_spread_days: u32,
    // Phase 10 loops back to 8 (record +0x58, FUN_00520b60, read by FUN_005227d0)
    pub repeats: u32,
    // Members go on a hidden mission (record +0x5c, FUN_00520b70)
    pub hidden: u32,
    // Decoy and detection phases run (record +0x60, FUN_00520b80)
    pub detection_phases: u32,
    // Members may resign (record +0x64, FUN_00520b90)
    pub can_resign: u32,
    // Where members go at the mission's end (record +0x68, FUN_00520ba0,
    // read by FUN_00521220): 0 stay, 1 home from another side's container,
    // 2 home always
    pub return_policy: u32,
    // A running mission ends (code 7) when its container is destroyed
    // (record +0x6c, FUN_00523450)
    pub container_loss_ends: u32,
    // Not yet mapped (record +0x70, a creation-only check in FUN_00592600)
    pub flag_col12: u32,
    // The container must be a populated system, else end 0xd (+0x74, FUN_00592600)
    pub needs_populated_container: u32,
    // The mission ends (code 6) when its target is destroyed (+0x78, FUN_00592600)
    pub target_loss_ends: u32,
    // A target on the mission's side is allowed, else end 8 (+0x7c, FUN_00523450)
    pub own_side_target: u32,
    // A target of a third side is allowed (+0x80, FUN_00523450)
    pub other_side_target: u32,
    // A target of the opponent is allowed (+0x84, FUN_00523450)
    pub opponent_target: u32,
    // A system target in an uprising is allowed, else end 6 (+0x88, FUN_005868c0)
    pub revolting_target: u32,
    // A system target not in an uprising is allowed (+0x8c, FUN_005868c0)
    pub calm_target: u32,
    // A prisoner character target is allowed, else end 6 (+0x90, FUN_00586e20)
    pub prisoner_target: u32,
    // A free character target is allowed (+0x94, FUN_00586e20)
    pub free_target: u32,
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
        Ok(Self {
            field1,
            count,
            family_id,
            field4,
            missions,
        })
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
            timer_min_days: r.read_u32()?,
            timer_spread_days: r.read_u32()?,
            repeats: r.read_u32()?,
            hidden: r.read_u32()?,
            detection_phases: r.read_u32()?,
            can_resign: r.read_u32()?,
            return_policy: r.read_u32()?,
            container_loss_ends: r.read_u32()?,
            flag_col12: r.read_u32()?,
            needs_populated_container: r.read_u32()?,
            target_loss_ends: r.read_u32()?,
            own_side_target: r.read_u32()?,
            other_side_target: r.read_u32()?,
            opponent_target: r.read_u32()?,
            revolting_target: r.read_u32()?,
            calm_target: r.read_u32()?,
            prisoner_target: r.read_u32()?,
            free_target: r.read_u32()?,
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
        w.write_u32(self.timer_min_days);
        w.write_u32(self.timer_spread_days);
        w.write_u32(self.repeats);
        w.write_u32(self.hidden);
        w.write_u32(self.detection_phases);
        w.write_u32(self.can_resign);
        w.write_u32(self.return_policy);
        w.write_u32(self.container_loss_ends);
        w.write_u32(self.flag_col12);
        w.write_u32(self.needs_populated_container);
        w.write_u32(self.target_loss_ends);
        w.write_u32(self.own_side_target);
        w.write_u32(self.other_side_target);
        w.write_u32(self.opponent_target);
        w.write_u32(self.revolting_target);
        w.write_u32(self.calm_target);
        w.write_u32(self.prisoner_target);
        w.write_u32(self.free_target);
    }
}
