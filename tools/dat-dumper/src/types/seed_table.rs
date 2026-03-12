use serde::Serialize;
use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;

/// One item within a seed group.
/// Layout: 3 u32 = 12 bytes per item.
///   field1:  u32  (always 1)
///   field2:  u32  (always 0)
///   item_id: u32  (entity ID reference)
#[derive(Debug, Serialize)]
pub struct SeedItem {
    pub field1: u32,
    pub field2: u32,
    pub item_id: u32,
}

/// One group within a seed table file.
/// Layout:
///   entry:       u32  (sequential 1, 2, 3, ...)
///   field2:      u32  (always 1)
///   entry_bis:   u32  (= entry)
///   field4:      u32  (always 1)
///   field5:      u32  (always 1)
///   items_count: u32
///   items:       [SeedItem; items_count]
#[derive(Debug, Serialize)]
pub struct SeedGroup {
    pub entry: u32,
    pub field2: u32,
    pub entry_bis: u32,
    pub field4: u32,
    pub field5: u32,
    pub items: Vec<SeedItem>,
}

/// Generic seed table file — shared by all 9 CMUN*/FACL* seed tables.
///
/// File layout (Pattern 3):
///   field1:       u32   (always 1)
///   groups_count: u32
///   info_length:  u32
///   info:         [u8; info_length]  ("SeedFamilyTableEntry", 20 bytes)
///   groups:       [SeedGroup; groups_count]
///
/// Used by:
///   CMUNAFTB.DAT  — Alliance fleet seed        (128 bytes)
///   CMUNALTB.DAT  — Alliance army seed         (512 bytes)
///   CMUNCRTB.DAT  — Empire Coruscant army seed (260 bytes)
///   CMUNEFTB.DAT  — Empire fleet seed          (176 bytes)
///   CMUNEMTB.DAT  — Empire army seed           (584 bytes)
///   CMUNHQTB.DAT  — Alliance HQ army seed      (176 bytes)
///   CMUNYVTB.DAT  — Alliance Yavin army seed   (116 bytes)
///   FACLCRTB.DAT  — Empire Coruscant facilities seed (80 bytes)
///   FACLHQTB.DAT  — Alliance HQ facilities seed     (164 bytes)
#[derive(Debug, Serialize)]
pub struct SeedTableFile {
    pub field1: u32,
    pub info: String,
    pub groups: Vec<SeedGroup>,
}

impl DatRecord for SeedTableFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let groups_count = r.read_u32()?;
        let info_length = r.read_u32()?;
        let info_bytes = r.read_bytes(info_length as usize)?;
        let info = String::from_utf8(info_bytes)
            .map_err(|e| anyhow::anyhow!("seed table info string is not valid UTF-8: {}", e))?;

        let mut groups = Vec::with_capacity(groups_count as usize);
        for _ in 0..groups_count {
            let entry = r.read_u32()?;
            let field2 = r.read_u32()?;
            let entry_bis = r.read_u32()?;
            let field4 = r.read_u32()?;
            let field5 = r.read_u32()?;
            let items_count = r.read_u32()?;

            let mut items = Vec::with_capacity(items_count as usize);
            for _ in 0..items_count {
                items.push(SeedItem {
                    field1:  r.read_u32()?,
                    field2:  r.read_u32()?,
                    item_id: r.read_u32()?,
                });
            }

            groups.push(SeedGroup {
                entry,
                field2,
                entry_bis,
                field4,
                field5,
                items,
            });
        }

        Ok(Self { field1, info, groups })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.groups.len() as u32);
        w.write_u32(self.info.len() as u32);
        w.write_bytes(self.info.as_bytes());

        for g in &self.groups {
            w.write_u32(g.entry);
            w.write_u32(g.field2);
            w.write_u32(g.entry_bis);
            w.write_u32(g.field4);
            w.write_u32(g.field5);
            w.write_u32(g.items.len() as u32);
            for item in &g.items {
                w.write_u32(item.field1);
                w.write_u32(item.field2);
                w.write_u32(item.item_id);
            }
        }
    }
}
