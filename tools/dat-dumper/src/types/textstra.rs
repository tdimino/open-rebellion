//! TEXTSTRA.DLL — Win32 PE RT_STRING resource extraction.
//!
//! RT_STRING resources store strings in bundles of 16. Bundle with resource ID N
//! holds string IDs (N-1)*16 through (N-1)*16+15. Each string entry is a u16
//! length (in UTF-16 code units) followed by that many little-endian UTF-16LE
//! code units. A length of 0 means the slot is empty.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use pelite::resources::{Entry, Name};
use pelite::{FileMap, PeFile};

/// RT_STRING resource type ID.
const RT_STRING_ID: u32 = 6;

/// Load all RT_STRING entries from a Win32 PE DLL into a `HashMap<string_id, name>`.
///
/// Works on both 32-bit and 64-bit PE images. Empty string slots (length == 0)
/// are omitted from the result.
pub fn load_strings(path: &Path) -> anyhow::Result<HashMap<u16, String>> {
    let map = FileMap::open(path)
        .with_context(|| format!("opening {}", path.display()))?;

    let pe = PeFile::from_bytes(&map)
        .with_context(|| format!("parsing {} as PE", path.display()))?;

    let resources = pe
        .resources()
        .with_context(|| format!("no resource section in {}", path.display()))?;

    let mut strings: HashMap<u16, String> = HashMap::new();

    // Root directory: entries are resource types. Find RT_STRING (id=6).
    let root = resources.root()?;
    for type_entry in root.entries() {
        let type_name = type_entry.name()?;
        if type_name != Name::Id(RT_STRING_ID) {
            continue;
        }

        // Type directory: entries are bundle IDs (1-based resource IDs).
        let type_dir = match type_entry.entry()? {
            Entry::Directory(d) => d,
            Entry::DataEntry(_) => continue,
        };

        for bundle_entry in type_dir.entries() {
            let bundle_id: u32 = match bundle_entry.name()? {
                Name::Id(id) => id,
                _ => continue,
            };

            // String IDs for this bundle: (bundle_id - 1) * 16  through  (bundle_id - 1) * 16 + 15
            if bundle_id == 0 {
                continue; // malformed
            }
            let base_id = (bundle_id - 1) * 16;

            // Language directory: one or more language entries. Use the first.
            let lang_dir = match bundle_entry.entry()? {
                Entry::Directory(d) => d,
                Entry::DataEntry(_) => continue,
            };

            let lang_entry = match lang_dir.entries().next() {
                Some(e) => e,
                None => continue,
            };

            let data = match lang_entry.entry()? {
                Entry::DataEntry(d) => d,
                Entry::Directory(_) => continue,
            };

            let raw = data.bytes()?;
            parse_string_bundle(raw, base_id, &mut strings)?;
        }

        break; // found RT_STRING; no need to continue
    }

    Ok(strings)
}

/// Parse one RT_STRING bundle (raw bytes) and insert decoded strings into `out`.
fn parse_string_bundle(
    raw: &[u8],
    base_id: u32,
    out: &mut HashMap<u16, String>,
) -> anyhow::Result<()> {
    let mut pos = 0usize;

    for slot in 0u32..16 {
        if pos + 2 > raw.len() {
            break;
        }
        let len = u16::from_le_bytes([raw[pos], raw[pos + 1]]) as usize;
        pos += 2;

        if len == 0 {
            continue; // empty slot
        }

        let byte_len = len * 2;
        if pos + byte_len > raw.len() {
            anyhow::bail!(
                "RT_STRING bundle truncated at slot {} (need {} bytes, have {})",
                base_id + slot,
                byte_len,
                raw.len().saturating_sub(pos)
            );
        }

        let utf16_bytes = &raw[pos..pos + byte_len];
        pos += byte_len;

        let code_units: Vec<u16> = utf16_bytes
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();

        let s = String::from_utf16(&code_units)
            .with_context(|| format!("invalid UTF-16 in string slot {}", base_id + slot))?;

        out.insert((base_id + slot) as u16, s);
    }

    Ok(())
}
