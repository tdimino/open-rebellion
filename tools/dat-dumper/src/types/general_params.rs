use serde::Serialize;
use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;

/// One entry in GNPRTB.DAT.
/// Layout: 3 u32 + 8 i32 = 44 bytes per entry.
#[derive(Debug, Serialize)]
pub struct GeneralParamEntry {
    pub id: u32,
    pub field2: u32,
    pub parameter_id: u32,
    pub development: i32,
    pub alliance_sp_easy: i32,
    pub alliance_sp_medium: i32,
    pub alliance_sp_hard: i32,
    pub empire_sp_easy: i32,
    pub empire_sp_medium: i32,
    pub empire_sp_hard: i32,
    pub multiplayer: i32,
}

/// GNPRTB.DAT — General parameter table.
/// File layout (Pattern 2):
///   field1:        u32   (always 1)
///   entries_count: u32
///   info_length:   u32
///   info:          [u8; info_length]  ("GeneralParamTableEntry", 22 bytes)
///   entries:       [GeneralParamEntry; entries_count]
///
/// Expected file size: 4 + 4 + 4 + 22 + 213*44 = 9406 bytes.
#[derive(Debug, Serialize)]
pub struct GeneralParamsFile {
    pub field1: u32,
    pub info: String,
    pub entries: Vec<GeneralParamEntry>,
}

impl DatRecord for GeneralParamsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let entries_count = r.read_u32()?;
        let info_length = r.read_u32()?;
        let info_bytes = r.read_bytes(info_length as usize)?;
        let info = String::from_utf8(info_bytes)
            .map_err(|e| anyhow::anyhow!("GNPRTB info string is not valid UTF-8: {}", e))?;

        let mut entries = Vec::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            entries.push(GeneralParamEntry {
                id:               r.read_u32()?,
                field2:           r.read_u32()?,
                parameter_id:     r.read_u32()?,
                development:      r.read_i32()?,
                alliance_sp_easy: r.read_i32()?,
                alliance_sp_medium: r.read_i32()?,
                alliance_sp_hard: r.read_i32()?,
                empire_sp_easy:   r.read_i32()?,
                empire_sp_medium: r.read_i32()?,
                empire_sp_hard:   r.read_i32()?,
                multiplayer:      r.read_i32()?,
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
            w.write_i32(e.development);
            w.write_i32(e.alliance_sp_easy);
            w.write_i32(e.alliance_sp_medium);
            w.write_i32(e.alliance_sp_hard);
            w.write_i32(e.empire_sp_easy);
            w.write_i32(e.empire_sp_medium);
            w.write_i32(e.empire_sp_hard);
            w.write_i32(e.multiplayer);
        }
    }
}
