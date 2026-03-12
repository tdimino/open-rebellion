use crate::codec::{ByteReader, ByteWriter};
use serde::Serialize;

/// Every .DAT file type implements this trait.
/// `parse` reads from binary, `write_bytes` reproduces the binary for round-trip validation.
pub trait DatRecord: Serialize + Sized {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self>;
    fn write_bytes(&self, w: &mut ByteWriter);
}
