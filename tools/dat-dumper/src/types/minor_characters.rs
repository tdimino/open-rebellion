use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use crate::types::major_characters::CharacterEntry;
use serde::Serialize;

// MNCHARSD.DAT — 54 minor characters, 148 bytes per record
// File size: 16 (header) + 54 * 148 = 8008 bytes
// Header: field1=1, count=54, family_id=56, field4=60
// Identical binary layout to MJCHARSD.DAT — reuses CharacterEntry.

#[derive(Debug, Clone, Serialize)]
pub struct MinorCharactersFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub characters: Vec<CharacterEntry>,
}

impl DatRecord for MinorCharactersFile {
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
