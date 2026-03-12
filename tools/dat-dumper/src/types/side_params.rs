use serde::Serialize;
use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;

/// One entry in SDPRTB.DAT.
/// Layout: 3 u32 + 16 i32 = 76 bytes per entry.
#[derive(Debug, Serialize)]
pub struct SideParamEntry {
    pub id: u32,
    pub field2: u32,
    pub parameter_id: u32,
    pub dev_alliance: i32,
    pub dev_empire: i32,
    pub alliance_sp_easy_alliance: i32,
    pub alliance_sp_easy_empire: i32,
    pub alliance_sp_medium_alliance: i32,
    pub alliance_sp_medium_empire: i32,
    pub alliance_sp_hard_alliance: i32,
    pub alliance_sp_hard_empire: i32,
    pub empire_sp_easy_alliance: i32,
    pub empire_sp_easy_empire: i32,
    pub empire_sp_medium_alliance: i32,
    pub empire_sp_medium_empire: i32,
    pub empire_sp_hard_alliance: i32,
    pub empire_sp_hard_empire: i32,
    pub multiplayer_alliance: i32,
    pub multiplayer_empire: i32,
}

/// SDPRTB.DAT — Side parameter table.
/// File layout (Pattern 2):
///   field1:        u32   (always 1)
///   entries_count: u32
///   info_length:   u32
///   info:          [u8; info_length]  ("SideParamTableEntry", 19 bytes)
///   entries:       [SideParamEntry; entries_count]
///
/// Expected file size: 4 + 4 + 4 + 19 + 35*76 = 2691 bytes.
#[derive(Debug, Serialize)]
pub struct SideParamsFile {
    pub field1: u32,
    pub info: String,
    pub entries: Vec<SideParamEntry>,
}

impl DatRecord for SideParamsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let entries_count = r.read_u32()?;
        let info_length = r.read_u32()?;
        let info_bytes = r.read_bytes(info_length as usize)?;
        let info = String::from_utf8(info_bytes)
            .map_err(|e| anyhow::anyhow!("SDPRTB info string is not valid UTF-8: {}", e))?;

        let mut entries = Vec::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            entries.push(SideParamEntry {
                id:                          r.read_u32()?,
                field2:                      r.read_u32()?,
                parameter_id:                r.read_u32()?,
                dev_alliance:                r.read_i32()?,
                dev_empire:                  r.read_i32()?,
                alliance_sp_easy_alliance:   r.read_i32()?,
                alliance_sp_easy_empire:     r.read_i32()?,
                alliance_sp_medium_alliance: r.read_i32()?,
                alliance_sp_medium_empire:   r.read_i32()?,
                alliance_sp_hard_alliance:   r.read_i32()?,
                alliance_sp_hard_empire:     r.read_i32()?,
                empire_sp_easy_alliance:     r.read_i32()?,
                empire_sp_easy_empire:       r.read_i32()?,
                empire_sp_medium_alliance:   r.read_i32()?,
                empire_sp_medium_empire:     r.read_i32()?,
                empire_sp_hard_alliance:     r.read_i32()?,
                empire_sp_hard_empire:       r.read_i32()?,
                multiplayer_alliance:        r.read_i32()?,
                multiplayer_empire:          r.read_i32()?,
            });
        }

        Ok(Self { field1, info, entries })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.entries.len() as u32);
        w.write_u32(self.info.len() as u32);
        w.write_bytes(self.info.as_bytes());

        for e in &self.entries {
            w.write_u32(e.id);
            w.write_u32(e.field2);
            w.write_u32(e.parameter_id);
            w.write_i32(e.dev_alliance);
            w.write_i32(e.dev_empire);
            w.write_i32(e.alliance_sp_easy_alliance);
            w.write_i32(e.alliance_sp_easy_empire);
            w.write_i32(e.alliance_sp_medium_alliance);
            w.write_i32(e.alliance_sp_medium_empire);
            w.write_i32(e.alliance_sp_hard_alliance);
            w.write_i32(e.alliance_sp_hard_empire);
            w.write_i32(e.empire_sp_easy_alliance);
            w.write_i32(e.empire_sp_easy_empire);
            w.write_i32(e.empire_sp_medium_alliance);
            w.write_i32(e.empire_sp_medium_empire);
            w.write_i32(e.empire_sp_hard_alliance);
            w.write_i32(e.empire_sp_hard_empire);
            w.write_i32(e.multiplayer_alliance);
            w.write_i32(e.multiplayer_empire);
        }
    }
}
