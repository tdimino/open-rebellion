use crate::codec::{ByteReader, ByteWriter};
use serde::Serialize;

/// Every .DAT file type implements this trait.
/// `parse` reads from binary, `write_bytes` reproduces the binary for round-trip validation.
pub trait DatRecord: Serialize + Sized {
    ///
    /// # Errors
    /// Returns an error for truncated input or an invalid record layout.
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self>;
    fn write_bytes(&self, w: &mut ByteWriter);
}
