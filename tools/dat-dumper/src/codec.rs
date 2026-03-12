/// Low-level binary reader for little-endian .DAT files.
/// All Star Wars Rebellion data files are raw little-endian structs with no header magic.
pub struct ByteReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn read_u32(&mut self) -> anyhow::Result<u32> {
        let bytes = self
            .data
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| anyhow::anyhow!("EOF at offset {} reading u32", self.pos))?;
        self.pos += 4;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i32(&mut self) -> anyhow::Result<i32> {
        let bytes = self
            .data
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| anyhow::anyhow!("EOF at offset {} reading i32", self.pos))?;
        self.pos += 4;
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u16(&mut self) -> anyhow::Result<u16> {
        let bytes = self
            .data
            .get(self.pos..self.pos + 2)
            .ok_or_else(|| anyhow::anyhow!("EOF at offset {} reading u16", self.pos))?;
        self.pos += 2;
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_bytes(&mut self, n: usize) -> anyhow::Result<Vec<u8>> {
        let bytes = self
            .data
            .get(self.pos..self.pos + n)
            .ok_or_else(|| anyhow::anyhow!("EOF at offset {} reading {} bytes", self.pos, n))?;
        self.pos += n;
        Ok(bytes.to_vec())
    }

    /// Verify we consumed every byte. Returns error if trailing data remains.
    pub fn assert_exhausted(&self, filename: &str) -> anyhow::Result<()> {
        if self.pos != self.data.len() {
            anyhow::bail!(
                "{}: {} bytes unread (pos={}, len={})",
                filename,
                self.data.len() - self.pos,
                self.pos,
                self.data.len()
            );
        }
        Ok(())
    }
}

/// Low-level binary writer for round-trip validation.
pub struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_i32(&mut self, v: i32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}
