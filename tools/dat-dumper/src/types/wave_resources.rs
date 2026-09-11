//! Win32 `WAVE` resource extraction for original Rebellion DLLs.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{ensure, Context};
use pelite::resources::Name;
use pelite::{FileMap, PeFile};

/// Load the requested named `WAVE` resources from a Win32 PE image.
///
/// # Errors
/// Returns an error if the DLL cannot be read, its PE resources are invalid,
/// or a requested resource is missing or is not RIFF/WAVE audio.
pub fn load_waves(path: &Path, resource_ids: &[u32]) -> anyhow::Result<HashMap<u32, Vec<u8>>> {
    let map = FileMap::open(path).with_context(|| format!("opening {}", path.display()))?;
    let pe =
        PeFile::from_bytes(&map).with_context(|| format!("parsing {} as PE", path.display()))?;
    let resources = pe
        .resources()
        .with_context(|| format!("no resource section in {}", path.display()))?;

    let mut waves = HashMap::with_capacity(resource_ids.len());
    for &resource_id in resource_ids {
        let bytes = resources
            .find_resource(&[Name::Str("WAVE"), Name::Id(resource_id)])
            .with_context(|| {
                format!(
                    "loading WAVE resource {resource_id} from {}",
                    path.display()
                )
            })?;
        ensure!(
            bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WAVE"),
            "resource {resource_id} in {} is not RIFF/WAVE audio",
            path.display()
        );
        waves.insert(resource_id, bytes.to_vec());
    }
    Ok(waves)
}
