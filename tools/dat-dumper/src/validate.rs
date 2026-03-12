/// Compare original bytes against re-serialized bytes for round-trip validation.
pub fn compare_bytes(original: &[u8], reserialized: &[u8], filename: &str) -> anyhow::Result<()> {
    if original.len() != reserialized.len() {
        anyhow::bail!(
            "{}: size mismatch: original {} bytes, reserialized {} bytes",
            filename,
            original.len(),
            reserialized.len()
        );
    }
    if let Some(pos) = original.iter().zip(reserialized).position(|(a, b)| a != b) {
        anyhow::bail!(
            "{}: byte mismatch at offset {}: original=0x{:02x}, reserialized=0x{:02x}",
            filename,
            pos,
            original[pos],
            reserialized[pos]
        );
    }
    Ok(())
}
