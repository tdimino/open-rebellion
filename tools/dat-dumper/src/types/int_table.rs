use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// Generic IntTableEntry file — Pattern 2, shared by ~20 mission and gameplay tables.
//
// File layout:
//   field1:        u32   (always 1)
//   entries_count: u32
//   info_length:   u32
//   info:          [u8; 13]  ("IntTableEntry")
//   entries:       [IntTableEntry; entries_count]
//
// Each IntTableEntry is 16 bytes: 4 u32 fields.
// The third field (threshold) is signed — it uses negative values for below-average skill,
// zero for average, and positive for above-average (i32 stored as u32 in binary).
//
// Used by:
//   DIPLMSTB.DAT  (10 entries) — diplomacy mission success table
//   ESPIMSTB.DAT  (12 entries) — espionage mission success table
//   ASSNMSTB.DAT  (12 entries) — assassination mission success table
//   INCTMSTB.DAT  (13 entries) — incite uprising mission success table
//   DSSBMSTB.DAT  (12 entries) — death star sabotage mission success table
//   ABDCMSTB.DAT  (12 entries) — abduction mission success table
//   RCRTMSTB.DAT  (11 entries) — recruitment mission success table
//   RESCMSTB.DAT  (12 entries) — rescue mission success table
//   SBTGMSTB.DAT  (12 entries) — sabotage mission success table
//   SUBDMSTB.DAT  (13 entries) — subdue uprising mission success table
//   ESCAPETB.DAT  ( 9 entries) — escape probability table
//   FDECOYTB.DAT  (14 entries) — fleet decoy success table
//   FOILTB.DAT    (14 entries) — mission foil probability table
//   INFORMTB.DAT  ( 8 entries) — informant probability table
//   CSCRHTTB.DAT  ( 5 entries) — covert search probability table
//   UPRIS1TB.DAT  ( 3 entries) — uprising start table
//   UPRIS2TB.DAT  ( 4 entries) — uprising end table
//   RLEVADTB.DAT  (14 entries) — rebel evasion probability table
//   RESRCTB.DAT   ( 4 entries) — resource table
//   TDECOYTB.DAT  (14 entries) — troop decoy success table

#[derive(Debug, Serialize)]
pub struct IntTableEntry {
    pub id: u32,
    pub field2: u32,
    // Skill/difficulty threshold — signed, negative = below average, 0 = average
    pub threshold: i32,
    // Probability or value at this threshold level
    pub value: u32,
}

#[derive(Debug, Serialize)]
pub struct IntTableFile {
    pub field1: u32,
    pub info: String,
    pub entries: Vec<IntTableEntry>,
}

impl DatRecord for IntTableFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let entries_count = r.read_u32()?;
        let info_length = r.read_u32()?;
        let info_bytes = r.read_bytes(info_length as usize)?;
        let info = String::from_utf8(info_bytes)
            .map_err(|e| anyhow::anyhow!("IntTable info string is not valid UTF-8: {}", e))?;

        let mut entries = Vec::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            entries.push(IntTableEntry {
                id:        r.read_u32()?,
                field2:    r.read_u32()?,
                threshold: r.read_i32()?,
                value:     r.read_u32()?,
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
            w.write_i32(e.threshold);
            w.write_u32(e.value);
        }
    }
}
