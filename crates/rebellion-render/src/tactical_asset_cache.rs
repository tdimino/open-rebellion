//! Platform-neutral storage for decoded tactical runtime objects.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
pub(crate) struct TacticalObjectCache {
    pub(crate) meshes: HashMap<String, Vec<u8>>,
    pub(crate) textures: HashMap<String, Vec<u8>>,
}

pub(crate) static TACTICAL_OBJECT_CACHE: LazyLock<Mutex<TacticalObjectCache>> =
    LazyLock::new(|| Mutex::new(TacticalObjectCache::default()));

/// Install typed tactical runtime objects unpacked from the browser pack.
pub fn set_tactical_asset_cache(
    meshes: HashMap<String, Vec<u8>>,
    textures: HashMap<String, Vec<u8>>,
) {
    *TACTICAL_OBJECT_CACHE.lock().unwrap() = TacticalObjectCache { meshes, textures };
}

/// Return the installed mesh and texture counts for runtime diagnostics.
#[must_use]
pub fn tactical_asset_cache_counts() -> (usize, usize) {
    let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
    (cache.meshes.len(), cache.textures.len())
}
